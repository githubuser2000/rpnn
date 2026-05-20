//! Bounded streaming handoff for Reta output.
//!
//! This module is intentionally part of the runtime/shared-library side.  The
//! launcher executable only supplies argv/stdin and receives byte chunks through
//! the C callback.  The heavy renderer can therefore stream rows while the `.so`
//! is still building them instead of first filling one giant `Vec<String>` or a
//! giant C string.
//!
//! The topology is:
//!
//! ```text
//! renderer workers -> bounded FIFO queues -> ordered aggregator -> chunk buffer -> callback
//! ```
//!
//! The bounded queues act as semaphores/back-pressure.  Render workers now send
//! bounded byte blocks instead of many tiny cell events.  A real LIFO buffer pool
//! recycles `Vec<u8>` blocks, while a byte semaphore limits queued bytes in
//! flight.  Visible output itself is never LIFO because CSV/HTML/Markdown/Shell
//! must stay byte-stable and ordered.

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex};

use crate::shared::parallel_runtime::{self, ParallelArea};

const DEFAULT_QUEUE_CAPACITY: usize = 64;
const DEFAULT_CHUNK_BYTES: usize = 64 * 1024;
const DEFAULT_PARALLEL_MIN_LINES_PER_WORKER: usize = 256;
const DEFAULT_IN_FLIGHT_BYTES: usize = DEFAULT_CHUNK_BYTES * 8;
const DEFAULT_BUFFER_POOL_CAPACITY: usize = 16;

type EmitThunk = unsafe fn(*mut (), OutputStreamKind, &[u8]) -> Result<(), String>;

unsafe fn call_emit_thunk<E>(
    ptr: *mut (),
    kind: OutputStreamKind,
    bytes: &[u8],
) -> Result<(), String>
where
    E: FnMut(OutputStreamKind, &[u8]) -> Result<(), String>,
{
    let emit = unsafe { &mut *(ptr as *mut E) };
    emit(kind, bytes)
}

fn call_erased_emit(
    ptr: *mut (),
    emit_fn: EmitThunk,
    kind: OutputStreamKind,
    bytes: &[u8],
) -> Result<(), String> {
    unsafe { emit_fn(ptr, kind, bytes) }
}

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
    pub in_flight_bytes: usize,
    pub buffer_pool_capacity: usize,
}

impl Default for OutputStreamNetworkConfig {
    fn default() -> Self {
        Self {
            queue_capacity: DEFAULT_QUEUE_CAPACITY,
            chunk_bytes: DEFAULT_CHUNK_BYTES,
            parallel_min_lines_per_worker: DEFAULT_PARALLEL_MIN_LINES_PER_WORKER,
            in_flight_bytes: DEFAULT_IN_FLIGHT_BYTES,
            buffer_pool_capacity: DEFAULT_BUFFER_POOL_CAPACITY,
        }
    }
}

impl OutputStreamNetworkConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();
        config.queue_capacity = env_usize("RETA_OUTPUT_QUEUE_CAPACITY")
            .or_else(|| env_usize("RETA_RENDER_QUEUE_CAPACITY"))
            .unwrap_or(config.queue_capacity)
            .max(1);
        config.chunk_bytes = env_usize("RETA_OUTPUT_CHUNK_BYTES")
            .or_else(|| env_usize("RETA_RENDER_CHUNK_BYTES"))
            .unwrap_or(config.chunk_bytes)
            .max(1);
        config.parallel_min_lines_per_worker = env_usize("RETA_OUTPUT_STREAM_MIN_LINES")
            .or_else(|| env_usize("RETA_OUTPUT_STREAM_MIN_ITEMS"))
            .or_else(|| env_usize("RETA_RENDER_MIN_ROWS"))
            .unwrap_or(config.parallel_min_lines_per_worker)
            .max(1);
        config.in_flight_bytes = env_usize("RETA_OUTPUT_IN_FLIGHT_BYTES")
            .or_else(|| env_usize("RETA_OUTPUT_MAX_BYTES_IN_FLIGHT"))
            .or_else(|| env_usize("RETA_RENDER_IN_FLIGHT_BYTES"))
            .or_else(|| env_usize("RETA_RENDER_MAX_BYTES_IN_FLIGHT"))
            .unwrap_or(config.in_flight_bytes)
            .max(1);
        config.buffer_pool_capacity = env_usize("RETA_OUTPUT_BUFFER_POOL_CAPACITY")
            .or_else(|| env_usize("RETA_RENDER_BUFFER_POOL_CAPACITY"))
            .unwrap_or(config.buffer_pool_capacity)
            .max(1);
        config
    }

    fn worker_block_bytes(&self) -> usize {
        self.chunk_bytes.min(self.in_flight_bytes).max(1)
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
        self.add_lines(kind, 1);
    }

    fn add_lines(&mut self, kind: OutputStreamKind, count: usize) {
        match kind {
            OutputStreamKind::Stdout => self.stdout_lines += count,
            OutputStreamKind::Stderr => self.stderr_lines += count,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct OutputFrame<'a> {
    line: &'a str,
}

#[derive(Debug)]
pub enum OrderedStreamItem {
    Bytes(Vec<u8>),
    Text(String),
    Static(&'static [u8]),
    Newline,
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

    fn push_line_erased(
        &mut self,
        line: &str,
        emit_ptr: *mut (),
        emit_fn: EmitThunk,
        stats: &mut OutputStreamStats,
    ) -> Result<(), String> {
        stats.add_line(self.kind);
        self.push_bytes_erased(line.as_bytes(), emit_ptr, emit_fn, stats)?;
        self.push_bytes_erased(b"\n", emit_ptr, emit_fn, stats)
    }

    fn push_newline_erased(
        &mut self,
        emit_ptr: *mut (),
        emit_fn: EmitThunk,
        stats: &mut OutputStreamStats,
    ) -> Result<(), String> {
        stats.add_line(self.kind);
        self.push_bytes_erased(b"\n", emit_ptr, emit_fn, stats)
    }

    fn push_bytes_erased(
        &mut self,
        mut incoming: &[u8],
        emit_ptr: *mut (),
        emit_fn: EmitThunk,
        stats: &mut OutputStreamStats,
    ) -> Result<(), String> {
        while !incoming.is_empty() {
            if self.bytes.len() >= self.max_bytes {
                self.flush_erased(emit_ptr, emit_fn, stats)?;
            }

            let remaining = self.max_bytes.saturating_sub(self.bytes.len()).max(1);
            let take = remaining.min(incoming.len());
            self.bytes.extend_from_slice(&incoming[..take]);
            incoming = &incoming[take..];

            if self.bytes.len() >= self.max_bytes {
                self.flush_erased(emit_ptr, emit_fn, stats)?;
            }
        }
        Ok(())
    }

    fn flush_erased(
        &mut self,
        emit_ptr: *mut (),
        emit_fn: EmitThunk,
        stats: &mut OutputStreamStats,
    ) -> Result<(), String> {
        if self.bytes.is_empty() {
            return Ok(());
        }
        call_erased_emit(emit_ptr, emit_fn, self.kind, &self.bytes)?;
        stats.add_chunk(self.kind, self.bytes.len());
        self.bytes.clear();
        Ok(())
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
        stats.add_line(self.kind);
        self.push_bytes(line.as_bytes(), emit, stats)?;
        self.push_bytes(b"\n", emit, stats)
    }

    fn push_bytes<E>(
        &mut self,
        mut incoming: &[u8],
        emit: &mut E,
        stats: &mut OutputStreamStats,
    ) -> Result<(), String>
    where
        E: FnMut(OutputStreamKind, &[u8]) -> Result<(), String>,
    {
        while !incoming.is_empty() {
            if self.bytes.len() >= self.max_bytes {
                self.flush(emit, stats)?;
            }

            let remaining = self.max_bytes.saturating_sub(self.bytes.len()).max(1);
            let take = remaining.min(incoming.len());
            self.bytes.extend_from_slice(&incoming[..take]);
            incoming = &incoming[take..];

            if self.bytes.len() >= self.max_bytes {
                self.flush(emit, stats)?;
            }
        }
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

struct ActiveOutputStream {
    emit_ptr: *mut (),
    emit_fn: EmitThunk,
    stdout_buffer: ChunkBuffer,
    stderr_buffer: ChunkBuffer,
    stats: OutputStreamStats,
    error: Option<String>,
    stdout_used: bool,
    stderr_used: bool,
}

impl ActiveOutputStream {
    fn new<E>(config: &OutputStreamNetworkConfig, emit: &mut E) -> Self
    where
        E: FnMut(OutputStreamKind, &[u8]) -> Result<(), String>,
    {
        Self {
            emit_ptr: emit as *mut E as *mut (),
            emit_fn: call_emit_thunk::<E>,
            stdout_buffer: ChunkBuffer::new(OutputStreamKind::Stdout, config.chunk_bytes),
            stderr_buffer: ChunkBuffer::new(OutputStreamKind::Stderr, config.chunk_bytes),
            stats: OutputStreamStats::default(),
            error: None,
            stdout_used: false,
            stderr_used: false,
        }
    }

    fn record_error(&mut self, error: String) {
        if self.error.is_none() {
            self.error = Some(error);
        }
    }

    fn push_kind_bytes(&mut self, kind: OutputStreamKind, bytes: &[u8]) -> Result<(), String> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        let result = match kind {
            OutputStreamKind::Stdout => {
                if !bytes.is_empty() {
                    self.stdout_used = true;
                }
                self.stdout_buffer.push_bytes_erased(
                    bytes,
                    self.emit_ptr,
                    self.emit_fn,
                    &mut self.stats,
                )
            }
            OutputStreamKind::Stderr => {
                if !bytes.is_empty() {
                    self.stderr_used = true;
                }
                self.stderr_buffer.push_bytes_erased(
                    bytes,
                    self.emit_ptr,
                    self.emit_fn,
                    &mut self.stats,
                )
            }
        };
        if let Err(error) = result.as_ref() {
            self.record_error(error.clone());
        }
        result
    }

    fn push_kind_block(
        &mut self,
        kind: OutputStreamKind,
        bytes: &[u8],
        line_count: usize,
    ) -> Result<(), String> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        let result = match kind {
            OutputStreamKind::Stdout => {
                if !bytes.is_empty() {
                    self.stdout_used = true;
                }
                let result = self.stdout_buffer.push_bytes_erased(
                    bytes,
                    self.emit_ptr,
                    self.emit_fn,
                    &mut self.stats,
                );
                if result.is_ok() && line_count > 0 {
                    self.stats.add_lines(kind, line_count);
                }
                result
            }
            OutputStreamKind::Stderr => {
                if !bytes.is_empty() {
                    self.stderr_used = true;
                }
                let result = self.stderr_buffer.push_bytes_erased(
                    bytes,
                    self.emit_ptr,
                    self.emit_fn,
                    &mut self.stats,
                );
                if result.is_ok() && line_count > 0 {
                    self.stats.add_lines(kind, line_count);
                }
                result
            }
        };
        if let Err(error) = result.as_ref() {
            self.record_error(error.clone());
        }
        result
    }

    fn push_kind_newline(&mut self, kind: OutputStreamKind) -> Result<(), String> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        let result = match kind {
            OutputStreamKind::Stdout => {
                self.stdout_used = true;
                self.stdout_buffer.push_newline_erased(
                    self.emit_ptr,
                    self.emit_fn,
                    &mut self.stats,
                )
            }
            OutputStreamKind::Stderr => {
                self.stderr_used = true;
                self.stderr_buffer.push_newline_erased(
                    self.emit_ptr,
                    self.emit_fn,
                    &mut self.stats,
                )
            }
        };
        if let Err(error) = result.as_ref() {
            self.record_error(error.clone());
        }
        result
    }

    fn push_kind_line(&mut self, kind: OutputStreamKind, line: &str) -> Result<(), String> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        let result = match kind {
            OutputStreamKind::Stdout => {
                self.stdout_used = true;
                self.stdout_buffer.push_line_erased(
                    line,
                    self.emit_ptr,
                    self.emit_fn,
                    &mut self.stats,
                )
            }
            OutputStreamKind::Stderr => {
                self.stderr_used = true;
                self.stderr_buffer.push_line_erased(
                    line,
                    self.emit_ptr,
                    self.emit_fn,
                    &mut self.stats,
                )
            }
        };
        if let Err(error) = result.as_ref() {
            self.record_error(error.clone());
        }
        result
    }

    fn flush_all(&mut self) -> Result<(), String> {
        if let Some(error) = self.error.clone() {
            return Err(error);
        }
        self.stderr_buffer.flush_erased(self.emit_ptr, self.emit_fn, &mut self.stats)?;
        self.stdout_buffer.flush_erased(self.emit_ptr, self.emit_fn, &mut self.stats)?;
        Ok(())
    }
}

std::thread_local! {
    static ACTIVE_OUTPUT_STREAM: RefCell<Option<*mut ()>> = RefCell::new(None);
}

struct ActiveOutputStreamScope {
    previous: Option<*mut ()>,
}

impl ActiveOutputStreamScope {
    fn install(ptr: *mut ()) -> Self {
        let previous = ACTIVE_OUTPUT_STREAM.with(|slot| slot.replace(Some(ptr)));
        Self { previous }
    }
}

impl Drop for ActiveOutputStreamScope {
    fn drop(&mut self) {
        let previous = self.previous.take();
        ACTIVE_OUTPUT_STREAM.with(|slot| {
            let _ = slot.replace(previous);
        });
    }
}

pub struct ActiveStreamOutcome<T> {
    pub result: T,
    pub stats: OutputStreamStats,
    pub stdout_used: bool,
    pub stderr_used: bool,
    pub error: Option<String>,
}

pub fn with_active_output_stream<E, T>(
    config: &OutputStreamNetworkConfig,
    emit: &mut E,
    f: impl FnOnce() -> T,
) -> ActiveStreamOutcome<T>
where
    E: FnMut(OutputStreamKind, &[u8]) -> Result<(), String>,
{
    let mut sink = ActiveOutputStream::new(config, emit);
    let sink_ptr = &mut sink as *mut ActiveOutputStream as *mut ();
    let scope_guard = ActiveOutputStreamScope::install(sink_ptr);
    let result = f();
    drop(scope_guard);

    if sink.error.is_none() {
        if let Err(error) = sink.flush_all() {
            sink.record_error(error);
        }
    }

    ActiveStreamOutcome {
        result,
        stats: sink.stats,
        stdout_used: sink.stdout_used,
        stderr_used: sink.stderr_used,
        error: sink.error,
    }
}

fn active_sink_ptr() -> Option<*mut ()> {
    ACTIVE_OUTPUT_STREAM.with(|slot| *slot.borrow())
}

fn with_active_sink_mut<R>(f: impl FnOnce(&mut ActiveOutputStream) -> R) -> Option<R> {
    let ptr = active_sink_ptr()?;
    let sink = unsafe { &mut *(ptr.cast::<ActiveOutputStream>()) };
    Some(f(sink))
}

pub fn active_streaming_enabled() -> bool {
    active_sink_ptr().is_some()
}

pub fn active_stream_has_error() -> bool {
    with_active_sink_mut(|sink| sink.error.is_some()).unwrap_or(false)
}

pub fn active_stream_error_message() -> Option<String> {
    with_active_sink_mut(|sink| sink.error.clone()).flatten()
}

pub fn active_record_stream_error(error: String) {
    let _ = with_active_sink_mut(|sink| sink.record_error(error));
}

pub fn active_stdout_line(line: &str) -> Result<(), String> {
    active_kind_line(OutputStreamKind::Stdout, line)
}

pub fn active_stdout_bytes(bytes: &[u8]) -> Result<(), String> {
    active_kind_bytes(OutputStreamKind::Stdout, bytes)
}

fn active_stdout_block(bytes: &[u8], line_count: usize) -> Result<(), String> {
    active_kind_block(OutputStreamKind::Stdout, bytes, line_count)
}

pub fn active_stdout_newline() -> Result<(), String> {
    active_kind_newline(OutputStreamKind::Stdout)
}

fn active_kind_line(kind: OutputStreamKind, line: &str) -> Result<(), String> {
    with_active_sink_mut(|sink| sink.push_kind_line(kind, line))
        .unwrap_or_else(|| Err("no active reta output stream".to_string()))
}

fn active_kind_bytes(kind: OutputStreamKind, bytes: &[u8]) -> Result<(), String> {
    with_active_sink_mut(|sink| sink.push_kind_bytes(kind, bytes))
        .unwrap_or_else(|| Err("no active reta output stream".to_string()))
}

fn active_kind_newline(kind: OutputStreamKind) -> Result<(), String> {
    with_active_sink_mut(|sink| sink.push_kind_newline(kind))
        .unwrap_or_else(|| Err("no active reta output stream".to_string()))
}

fn active_kind_block(kind: OutputStreamKind, bytes: &[u8], line_count: usize) -> Result<(), String> {
    with_active_sink_mut(|sink| sink.push_kind_block(kind, bytes, line_count))
        .unwrap_or_else(|| Err("no active reta output stream".to_string()))
}

struct RenderedOutputBlock {
    bytes: Vec<u8>,
    line_count: usize,
    reserved_bytes: usize,
}

enum WorkerBlockMessage {
    Block(RenderedOutputBlock),
    Error(String),
}

struct ByteBufferPool {
    stack: Mutex<Vec<Vec<u8>>>,
    max_buffers: usize,
    initial_capacity: usize,
}

impl ByteBufferPool {
    fn new(max_buffers: usize, initial_capacity: usize) -> Self {
        Self {
            stack: Mutex::new(Vec::with_capacity(max_buffers.min(32))),
            max_buffers: max_buffers.max(1),
            initial_capacity: initial_capacity.max(1),
        }
    }

    fn take(&self) -> Vec<u8> {
        if let Ok(mut stack) = self.stack.lock() {
            if let Some(mut bytes) = stack.pop() {
                bytes.clear();
                return bytes;
            }
        }
        Vec::with_capacity(self.initial_capacity)
    }

    fn put(&self, mut bytes: Vec<u8>) {
        if bytes.capacity() > self.initial_capacity.saturating_mul(4).max(self.initial_capacity) {
            bytes = Vec::with_capacity(self.initial_capacity);
        } else {
            bytes.clear();
        }
        if let Ok(mut stack) = self.stack.lock() {
            if stack.len() < self.max_buffers {
                stack.push(bytes);
            }
        }
    }
}

struct ByteSemaphore {
    limit: usize,
    used: Mutex<usize>,
    cvar: Condvar,
}

impl ByteSemaphore {
    fn new(limit: usize) -> Self {
        Self {
            limit: limit.max(1),
            used: Mutex::new(0),
            cvar: Condvar::new(),
        }
    }

    fn acquire(&self, amount: usize, cancelled: &AtomicBool) -> Result<usize, String> {
        let amount = amount.min(self.limit).max(1);
        let mut used = self
            .used
            .lock()
            .map_err(|_| "output byte semaphore poisoned".to_string())?;
        while used.saturating_add(amount) > self.limit {
            if cancelled.load(Ordering::Acquire) {
                return Err("ordered output cancelled".to_string());
            }
            used = self
                .cvar
                .wait(used)
                .map_err(|_| "output byte semaphore poisoned".to_string())?;
        }
        *used = used.saturating_add(amount);
        Ok(amount)
    }

    fn release(&self, amount: usize) {
        if amount == 0 {
            return;
        }
        if let Ok(mut used) = self.used.lock() {
            *used = used.saturating_sub(amount);
            self.cvar.notify_all();
        }
    }

    fn wake_all(&self) {
        self.cvar.notify_all();
    }
}

struct DirectBlockEmitter {
    buffer: Vec<u8>,
    line_count: usize,
    max_bytes: usize,
}

impl DirectBlockEmitter {
    fn new(max_bytes: usize) -> Self {
        let max_bytes = max_bytes.max(1);
        Self {
            buffer: Vec::with_capacity(max_bytes.min(DEFAULT_CHUNK_BYTES)),
            line_count: 0,
            max_bytes,
        }
    }

    fn push_item(&mut self, item: OrderedStreamItem) -> Result<(), String> {
        match item {
            OrderedStreamItem::Bytes(bytes) => self.push_bytes(&bytes),
            OrderedStreamItem::Text(text) => self.push_bytes(text.as_bytes()),
            OrderedStreamItem::Static(bytes) => self.push_bytes(bytes),
            OrderedStreamItem::Newline => {
                self.line_count = self.line_count.saturating_add(1);
                self.push_bytes(b"\n")
            }
        }
    }

    fn push_bytes(&mut self, mut incoming: &[u8]) -> Result<(), String> {
        while !incoming.is_empty() {
            if self.buffer.len() >= self.max_bytes {
                self.flush()?;
            }
            let remaining = self.max_bytes.saturating_sub(self.buffer.len()).max(1);
            let take = remaining.min(incoming.len());
            self.buffer.extend_from_slice(&incoming[..take]);
            incoming = &incoming[take..];
            if self.buffer.len() >= self.max_bytes {
                self.flush()?;
            }
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> {
        if self.buffer.is_empty() && self.line_count == 0 {
            return Ok(());
        }
        active_stdout_block(&self.buffer, self.line_count)?;
        self.buffer.clear();
        self.line_count = 0;
        Ok(())
    }

    fn finish(&mut self) -> Result<(), String> {
        self.flush()
    }
}

struct OrderedBlockEmitter<'a> {
    tx: &'a mpsc::SyncSender<WorkerBlockMessage>,
    pool: Arc<ByteBufferPool>,
    semaphore: Arc<ByteSemaphore>,
    cancelled: Arc<AtomicBool>,
    buffer: Vec<u8>,
    line_count: usize,
    max_bytes: usize,
}

impl<'a> OrderedBlockEmitter<'a> {
    fn new(
        tx: &'a mpsc::SyncSender<WorkerBlockMessage>,
        pool: Arc<ByteBufferPool>,
        semaphore: Arc<ByteSemaphore>,
        cancelled: Arc<AtomicBool>,
        max_bytes: usize,
    ) -> Self {
        let buffer = pool.take();
        Self {
            tx,
            pool,
            semaphore,
            cancelled,
            buffer,
            line_count: 0,
            max_bytes: max_bytes.max(1),
        }
    }

    fn push_item(&mut self, item: OrderedStreamItem) -> Result<(), String> {
        match item {
            OrderedStreamItem::Bytes(bytes) => self.push_bytes(&bytes),
            OrderedStreamItem::Text(text) => self.push_bytes(text.as_bytes()),
            OrderedStreamItem::Static(bytes) => self.push_bytes(bytes),
            OrderedStreamItem::Newline => {
                self.line_count = self.line_count.saturating_add(1);
                self.push_bytes(b"\n")
            }
        }
    }

    fn push_bytes(&mut self, mut incoming: &[u8]) -> Result<(), String> {
        while !incoming.is_empty() {
            if self.cancelled.load(Ordering::Acquire) {
                return Err("ordered output cancelled".to_string());
            }
            if self.buffer.len() >= self.max_bytes {
                self.flush()?;
            }
            let remaining = self.max_bytes.saturating_sub(self.buffer.len()).max(1);
            let take = remaining.min(incoming.len());
            self.buffer.extend_from_slice(&incoming[..take]);
            incoming = &incoming[take..];
            if self.buffer.len() >= self.max_bytes {
                self.flush()?;
            }
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), String> {
        if self.buffer.is_empty() && self.line_count == 0 {
            return Ok(());
        }
        let next_buffer = self.pool.take();
        let bytes = std::mem::replace(&mut self.buffer, next_buffer);
        let line_count = std::mem::take(&mut self.line_count);
        let reserved_bytes = match self.semaphore.acquire(bytes.len().max(1), &self.cancelled) {
            Ok(reserved_bytes) => reserved_bytes,
            Err(error) => {
                self.pool.put(bytes);
                return Err(error);
            }
        };
        let block = RenderedOutputBlock {
            bytes,
            line_count,
            reserved_bytes,
        };
        match self.tx.send(WorkerBlockMessage::Block(block)) {
            Ok(()) => Ok(()),
            Err(error) => match error.0 {
                WorkerBlockMessage::Block(mut block) => {
                    self.semaphore.release(block.reserved_bytes);
                    let bytes = std::mem::take(&mut block.bytes);
                    self.pool.put(bytes);
                    Err("ordered output consumer stopped".to_string())
                }
                WorkerBlockMessage::Error(error) => Err(error),
            },
        }
    }

    fn finish(&mut self) -> Result<(), String> {
        self.flush()
    }
}

impl Drop for OrderedBlockEmitter<'_> {
    fn drop(&mut self) {
        let bytes = std::mem::take(&mut self.buffer);
        self.pool.put(bytes);
    }
}

fn wake_all_semaphores(semaphores: &[Arc<ByteSemaphore>]) {
    for semaphore in semaphores {
        semaphore.wake_all();
    }
}

pub fn stream_active_stdout_ordered_items<F>(
    item_count: usize,
    min_items_per_worker: usize,
    config: &OutputStreamNetworkConfig,
    render_item: F,
) -> Result<(), String>
where
    F: Fn(usize, &mut dyn FnMut(OrderedStreamItem) -> Result<(), String>) -> Result<(), String>
        + Sync,
{
    if item_count == 0 {
        return Ok(());
    }
    if !active_streaming_enabled() {
        return Err("no active reta output stream".to_string());
    }

    let reservation = parallel_runtime::reserve_ranges(
        ParallelArea::Output,
        item_count,
        min_items_per_worker.max(1),
    );
    let (budget_guard, ranges) = match reservation {
        Some((guard, ranges)) => (Some(guard), ranges),
        None => (None, vec![(0, item_count)]),
    };

    if ranges.len() <= 1 {
        let mut emitter = DirectBlockEmitter::new(config.worker_block_bytes());
        for index in 0..item_count {
            if let Some(error) = active_stream_error_message() {
                return Err(error);
            }
            let mut emit_item = |item| emitter.push_item(item);
            render_item(index, &mut emit_item)?;
        }
        return emitter.finish();
    }

    let queue_capacity = config.queue_capacity.max(1);
    let producer_count = ranges.len().max(1);
    let per_queue_in_flight = (config.in_flight_bytes / producer_count).max(1);
    let block_bytes = config.chunk_bytes.min(per_queue_in_flight).max(1);
    let per_queue_in_flight = per_queue_in_flight.max(block_bytes);
    let pool_capacity = config
        .buffer_pool_capacity
        .saturating_add(producer_count.saturating_mul(queue_capacity))
        .max(1);
    let buffer_pool = Arc::new(ByteBufferPool::new(pool_capacity, block_bytes));
    let cancelled = Arc::new(AtomicBool::new(false));

    std::thread::scope(|scope| {
        let _budget_guard = budget_guard;
        let render_item = &render_item;
        let mut receivers = Vec::with_capacity(ranges.len());
        let mut semaphores: Vec<Arc<ByteSemaphore>> = Vec::with_capacity(ranges.len());
        let mut handles = Vec::with_capacity(ranges.len());

        for (start, end) in ranges {
            let (tx, rx) = mpsc::sync_channel::<WorkerBlockMessage>(queue_capacity);
            let semaphore = Arc::new(ByteSemaphore::new(per_queue_in_flight));
            receivers.push((rx, Arc::clone(&semaphore)));
            semaphores.push(Arc::clone(&semaphore));
            let cancelled_for_worker = Arc::clone(&cancelled);
            let pool_for_worker = Arc::clone(&buffer_pool);
            handles.push(scope.spawn(move || {
                let _depth_guard = parallel_runtime::enter_parallel_worker_scope();
                for index in start..end {
                    if cancelled_for_worker.load(Ordering::Acquire) {
                        break;
                    }
                    let mut emitter = OrderedBlockEmitter::new(
                        &tx,
                        Arc::clone(&pool_for_worker),
                        Arc::clone(&semaphore),
                        Arc::clone(&cancelled_for_worker),
                        block_bytes,
                    );
                    let render_result = {
                        let mut send_item = |item| emitter.push_item(item);
                        render_item(index, &mut send_item)
                    };
                    let result = render_result.and_then(|_| emitter.finish());
                    if let Err(error) = result {
                        cancelled_for_worker.store(true, Ordering::Release);
                        semaphore.wake_all();
                        let _ = tx.send(WorkerBlockMessage::Error(error));
                        break;
                    }
                }
            }));
        }

        let mut first_error: Option<String> = None;
        for (rx, semaphore) in receivers {
            while let Ok(message) = rx.recv() {
                match message {
                    WorkerBlockMessage::Block(mut block) => {
                        if first_error.is_none() {
                            if let Err(error) = active_stdout_block(&block.bytes, block.line_count) {
                                cancelled.store(true, Ordering::Release);
                                wake_all_semaphores(&semaphores);
                                first_error = Some(error);
                            }
                        }
                        semaphore.release(block.reserved_bytes);
                        let bytes = std::mem::take(&mut block.bytes);
                        buffer_pool.put(bytes);
                    }
                    WorkerBlockMessage::Error(error) => {
                        cancelled.store(true, Ordering::Release);
                        wake_all_semaphores(&semaphores);
                        if first_error.is_none() {
                            first_error = Some(error);
                        }
                    }
                }
                // Do not break immediately on the first error.  Later workers may
                // already be blocked inside bounded sync_channel::send or the byte
                // semaphore; draining every FIFO releases them, while cancelled
                // stops new work.
            }
        }

        for handle in handles {
            if handle.join().is_err() && first_error.is_none() {
                cancelled.store(true, Ordering::Release);
                wake_all_semaphores(&semaphores);
                first_error = Some("panic inside ordered output renderer".to_string());
            }
        }

        if let Some(error) = first_error {
            active_record_stream_error(error.clone());
            Err(error)
        } else {
            Ok(())
        }
    })
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
            ..OutputStreamNetworkConfig::default()
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
            ..OutputStreamNetworkConfig::default()
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
    fn oversized_line_is_split_into_bounded_chunks() {
        let lines = vec!["abcdef".to_string()];
        let config = OutputStreamNetworkConfig {
            queue_capacity: 1,
            chunk_bytes: 2,
            parallel_min_lines_per_worker: usize::MAX / 2,
            ..OutputStreamNetworkConfig::default()
        };
        let mut chunks: Vec<Vec<u8>> = Vec::new();
        let stats = stream_lines(&lines, OutputStreamKind::Stdout, &config, &mut |_kind, bytes| {
            chunks.push(bytes.to_vec());
            Ok(())
        })
        .unwrap();

        assert_eq!(
            chunks,
            vec![b"ab".to_vec(), b"cd".to_vec(), b"ef".to_vec(), b"\n".to_vec()]
        );
        assert_eq!(stats.stdout_lines, 1);
        assert_eq!(stats.stdout_bytes, 7);
    }

    #[test]
    fn active_ordered_items_stream_as_single_logical_stdout() {
        let config = OutputStreamNetworkConfig {
            queue_capacity: 1,
            chunk_bytes: 3,
            parallel_min_lines_per_worker: 1,
            in_flight_bytes: 6,
            buffer_pool_capacity: 2,
        };
        let mut out = Vec::new();
        let outcome = with_active_output_stream(&config, &mut |_kind, bytes| {
            out.extend_from_slice(bytes);
            Ok(())
        }, || {
            stream_active_stdout_ordered_items(3, 1, &config, |idx, emit| {
                emit(OrderedStreamItem::Text(format!("row{idx}")))?;
                emit(OrderedStreamItem::Newline)
            })
            .unwrap();
        });

        assert!(outcome.error.is_none());
        assert!(outcome.stdout_used);
        assert_eq!(outcome.stats.stdout_lines, 3);
        assert_eq!(String::from_utf8(out).unwrap(), "row0\nrow1\nrow2\n");
    }
}
