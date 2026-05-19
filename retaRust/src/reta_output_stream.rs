//! Bounded streaming handoff for final Reta output.
//!
//! The legacy path still keeps the byte-exact renderer intact, but this module
//! removes the final "join everything into one huge String, then copy it again
//! over FFI" step.  Lines are handed through a bounded FIFO queue and written
//! in fixed-size chunks.  The bounded queue acts as the semaphore/back-pressure
//! point: if the consumer is slower than the producer, producers block instead
//! of growing an unbounded output buffer.  The C-ABI layer uses the same
//! callback shape for stdout and stderr, so the final handoff is duplex from
//! the engine to the launcher while stdin still travels in the request.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};

use crate::shared::parallel_runtime::{self, ParallelArea};

const DEFAULT_QUEUE_CAPACITY: usize = 64;
const DEFAULT_CHUNK_BYTES: usize = 64 * 1024;
const DEFAULT_PARALLEL_MIN_LINES_PER_WORKER: usize = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputStreamKind {
    Stdout,
    Stderr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutputStreamNetworkConfig {
    pub queue_capacity: usize,
    pub chunk_bytes: usize,
    pub parallel_min_lines_per_worker: usize,
}

impl Default for OutputStreamNetworkConfig {
    fn default() -> Self {
        Self {
            queue_capacity: DEFAULT_QUEUE_CAPACITY,
            chunk_bytes: DEFAULT_CHUNK_BYTES,
            parallel_min_lines_per_worker: DEFAULT_PARALLEL_MIN_LINES_PER_WORKER,
        }
    }
}

impl OutputStreamNetworkConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();
        config.queue_capacity = env_usize("RETA_OUTPUT_QUEUE_CAPACITY")
            .unwrap_or(config.queue_capacity)
            .max(1);
        config.chunk_bytes = env_usize("RETA_OUTPUT_CHUNK_BYTES")
            .unwrap_or(config.chunk_bytes)
            .max(1);
        config.parallel_min_lines_per_worker = env_usize("RETA_OUTPUT_STREAM_MIN_LINES")
            .or_else(|| env_usize("RETA_OUTPUT_STREAM_MIN_ITEMS"))
            .unwrap_or(config.parallel_min_lines_per_worker)
            .max(1);
        config
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OutputStreamStats {
    pub stdout_chunks: usize,
    pub stderr_chunks: usize,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub stdout_lines: usize,
    pub stderr_lines: usize,
}

impl OutputStreamStats {
    pub fn merge(&mut self, other: OutputStreamStats) {
        self.stdout_chunks += other.stdout_chunks;
        self.stderr_chunks += other.stderr_chunks;
        self.stdout_bytes += other.stdout_bytes;
        self.stderr_bytes += other.stderr_bytes;
        self.stdout_lines += other.stdout_lines;
        self.stderr_lines += other.stderr_lines;
    }

    fn add_chunk(&mut self, kind: OutputStreamKind, len: usize) {
        match kind {
            OutputStreamKind::Stdout => {
                self.stdout_chunks += 1;
                self.stdout_bytes += len;
            }
            OutputStreamKind::Stderr => {
                self.stderr_chunks += 1;
                self.stderr_bytes += len;
            }
        }
    }

    fn add_line(&mut self, kind: OutputStreamKind) {
        match kind {
            OutputStreamKind::Stdout => self.stdout_lines += 1,
            OutputStreamKind::Stderr => self.stderr_lines += 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct OutputFrame<'a> {
    line: &'a str,
}

struct ChunkBuffer {
    kind: OutputStreamKind,
    max_bytes: usize,
    bytes: Vec<u8>,
}

impl ChunkBuffer {
    fn new(kind: OutputStreamKind, max_bytes: usize) -> Self {
        let max_bytes = max_bytes.max(1);
        Self {
            kind,
            max_bytes,
            bytes: Vec::with_capacity(max_bytes.min(DEFAULT_CHUNK_BYTES)),
        }
    }

    fn push_line<E>(
        &mut self,
        line: &str,
        emit: &mut E,
        stats: &mut OutputStreamStats,
    ) -> Result<(), String>
    where
        E: FnMut(OutputStreamKind, &[u8]) -> Result<(), String>,
    {
        let line_bytes = line.as_bytes();
        let needed = line_bytes.len().saturating_add(1);
        stats.add_line(self.kind);

        if needed > self.max_bytes {
            self.flush(emit, stats)?;
            if !line_bytes.is_empty() {
                emit(self.kind, line_bytes)?;
                stats.add_chunk(self.kind, line_bytes.len());
            }
            emit(self.kind, b"\n")?;
            stats.add_chunk(self.kind, 1);
            return Ok(());
        }

        if self.bytes.len().saturating_add(needed) > self.max_bytes {
            self.flush(emit, stats)?;
        }
        self.bytes.extend_from_slice(line_bytes);
        self.bytes.push(b'\n');
        Ok(())
    }

    fn flush<E>(&mut self, emit: &mut E, stats: &mut OutputStreamStats) -> Result<(), String>
    where
        E: FnMut(OutputStreamKind, &[u8]) -> Result<(), String>,
    {
        if self.bytes.is_empty() {
            return Ok(());
        }
        emit(self.kind, &self.bytes)?;
        stats.add_chunk(self.kind, self.bytes.len());
        self.bytes.clear();
        Ok(())
    }
}

pub fn stream_lines<E>(
    lines: &[String],
    kind: OutputStreamKind,
    config: &OutputStreamNetworkConfig,
    emit: &mut E,
) -> Result<OutputStreamStats, String>
where
    E: FnMut(OutputStreamKind, &[u8]) -> Result<(), String>,
{
    if lines.is_empty() {
        return Ok(OutputStreamStats::default());
    }

    let reservation = parallel_runtime::reserve_ranges(
        ParallelArea::Output,
        lines.len(),
        config.parallel_min_lines_per_worker,
    );
    let (budget_guard, ranges) = match reservation {
        Some((guard, ranges)) => (Some(guard), ranges),
        None => (None, vec![(0, lines.len())]),
    };

    stream_lines_with_ranges(lines, kind, config, emit, ranges, budget_guard)
}

fn stream_lines_with_ranges<E>(
    lines: &[String],
    kind: OutputStreamKind,
    config: &OutputStreamNetworkConfig,
    emit: &mut E,
    ranges: Vec<(usize, usize)>,
    budget_guard: Option<parallel_runtime::ParallelWorkGuard>,
) -> Result<OutputStreamStats, String>
where
    E: FnMut(OutputStreamKind, &[u8]) -> Result<(), String>,
{
    let queue_capacity = config.queue_capacity.max(1);
    let chunk_bytes = config.chunk_bytes.max(1);

    let cancelled = Arc::new(AtomicBool::new(false));

    std::thread::scope(|scope| {
        let _budget_guard = budget_guard;
        let mut receivers = Vec::with_capacity(ranges.len());
        let mut handles = Vec::with_capacity(ranges.len());

        for (start, end) in ranges {
            let (tx, rx) = mpsc::sync_channel::<OutputFrame<'_>>(queue_capacity);
            receivers.push(rx);
            let cancelled_for_worker = Arc::clone(&cancelled);
            handles.push(scope.spawn(move || {
                for line in lines[start..end].iter() {
                    if cancelled_for_worker.load(Ordering::Acquire) {
                        break;
                    }
                    if tx
                        .send(OutputFrame {
                            line: line.as_str(),
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            }));
        }

        let mut stats = OutputStreamStats::default();
        let mut buffer = ChunkBuffer::new(kind, chunk_bytes);
        let mut first_error: Option<String> = None;

        // Drain each FIFO in range order.  Producers may run in parallel, but
        // visible output stays deterministic and the total queued data stays
        // bounded by queue_capacity * producer_count.  Later producers simply
        // block on their own queue when the ordered consumer has not reached
        // them yet.
        for rx in receivers {
            while let Ok(frame) = rx.recv() {
                if first_error.is_some() {
                    continue;
                }
                if let Err(error) = buffer.push_line(frame.line, emit, &mut stats) {
                    cancelled.store(true, Ordering::Release);
                    first_error = Some(error);
                }
            }
        }

        if first_error.is_none() {
            if let Err(error) = buffer.flush(emit, &mut stats) {
                first_error = Some(error);
            }
        }

        for handle in handles {
            if handle.join().is_err() && first_error.is_none() {
                first_error = Some("panic inside output stream producer".to_string());
            }
        }

        match first_error {
            Some(error) => Err(error),
            None => Ok(stats),
        }
    })
}

fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streams_lines_without_final_join_string() {
        let lines = vec!["eins".to_string(), "zwei".to_string(), "drei".to_string()];
        let config = OutputStreamNetworkConfig {
            queue_capacity: 2,
            chunk_bytes: 64,
            parallel_min_lines_per_worker: usize::MAX / 2,
        };
        let mut out = Vec::new();
        let stats = stream_lines(&lines, OutputStreamKind::Stdout, &config, &mut |kind, bytes| {
            assert_eq!(kind, OutputStreamKind::Stdout);
            out.extend_from_slice(bytes);
            Ok(())
        })
        .unwrap();
        assert_eq!(String::from_utf8(out).unwrap(), "eins\nzwei\ndrei\n");
        assert_eq!(stats.stdout_lines, 3);
        assert_eq!(stats.stdout_bytes, "eins\nzwei\ndrei\n".len());
    }

    #[test]
    fn multiple_fifo_queues_preserve_visible_order() {
        let lines = vec![
            "eins".to_string(),
            "zwei".to_string(),
            "drei".to_string(),
            "vier".to_string(),
        ];
        let config = OutputStreamNetworkConfig {
            queue_capacity: 1,
            chunk_bytes: 128,
            parallel_min_lines_per_worker: 1,
        };
        let mut out = Vec::new();
        let stats = stream_lines_with_ranges(
            &lines,
            OutputStreamKind::Stdout,
            &config,
            &mut |_kind, bytes| {
                out.extend_from_slice(bytes);
                Ok(())
            },
            vec![(0, 2), (2, 4)],
            None,
        )
        .unwrap();

        assert_eq!(String::from_utf8(out).unwrap(), "eins\nzwei\ndrei\nvier\n");
        assert_eq!(stats.stdout_lines, 4);
    }

    #[test]
    fn chunk_buffer_stays_bounded_for_small_lines() {
        let lines = vec!["aa".to_string(), "bb".to_string(), "cc".to_string()];
        let config = OutputStreamNetworkConfig {
            queue_capacity: 2,
            chunk_bytes: 3,
            parallel_min_lines_per_worker: usize::MAX / 2,
        };
        let mut chunks = Vec::new();
        let stats = stream_lines(&lines, OutputStreamKind::Stdout, &config, &mut |_kind, bytes| {
            chunks.push(bytes.to_vec());
            Ok(())
        })
        .unwrap();
        assert_eq!(stats.stdout_chunks, 3);
        assert_eq!(chunks, vec![b"aa\n".to_vec(), b"bb\n".to_vec(), b"cc\n".to_vec()]);
    }
}
