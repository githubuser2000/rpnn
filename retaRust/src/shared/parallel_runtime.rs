//! Central parallelism policy for reta.
//!
//! All Rust-side parallel work should pass through this module.  The goal is
//! not to force every workload to be parallel; the goal is to prevent many
//! independent modules from each starting their own full set of threads.  This
//! is especially important on Termux/Android and other small/big.LITTLE
//! systems where memory bandwidth and scheduler overhead can dominate.

#![allow(dead_code)]

use std::cell::Cell;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParallelArea {
    Generators,
    Output,
    Widths,
    PromptBatch,
}

impl ParallelArea {
    fn env_prefix(self) -> &'static str {
        match self {
            Self::Generators => "RETA_GENERATORS",
            Self::Output => "RETA_OUTPUT",
            Self::Widths => "RETA_WIDTH",
            Self::PromptBatch => "RETA_PROMPT",
        }
    }

    fn extra_env_prefixes(self) -> &'static [&'static str] {
        match self {
            Self::Widths => &["RETA_WIDTHS"],
            Self::PromptBatch => &["RETA_PROMPT_BATCH"],
            _ => &[],
        }
    }

    fn default_enabled_in_auto(self) -> bool {
        match self {
            // Generator cells are very string/allocation heavy.  They became
            // slower for common `-spalten --alles` cases when enabled by
            // default, so they stay opt-in unless RETA_PARALLEL=1 or
            // RETA_GENERATORS_PARALLEL=1 is explicitly set.
            Self::Generators => false,
            // These were already parallel in retaRust before this central
            // policy existed.  Keep their automatic behavior, but now bounded
            // by the shared worker budget.
            Self::Output | Self::Widths | Self::PromptBatch => true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParallelMode {
    Off,
    On,
    Auto,
}

#[derive(Clone, Debug)]
pub struct ParallelConfig {
    global_mode: ParallelMode,
    max_jobs: usize,
    global_min_items: Option<usize>,
    allow_nested: bool,
}

#[derive(Debug)]
pub struct ParallelWorkGuard {
    workers: usize,
    extra_workers: usize,
}

impl ParallelWorkGuard {
    pub fn workers(&self) -> usize {
        self.workers
    }

    pub fn ranges(&self, total_items: usize) -> Vec<(usize, usize)> {
        split_ranges(total_items, self.workers)
    }
}

impl Drop for ParallelWorkGuard {
    fn drop(&mut self) {
        if self.extra_workers > 0 {
            ACTIVE_EXTRA_WORKERS.fetch_sub(self.extra_workers, Ordering::AcqRel);
        }
    }
}

struct ParallelDepthGuard;

impl ParallelDepthGuard {
    fn enter() -> Self {
        PARALLEL_DEPTH.with(|depth| depth.set(depth.get().saturating_add(1)));
        Self
    }
}

impl Drop for ParallelDepthGuard {
    fn drop(&mut self) {
        PARALLEL_DEPTH.with(|depth| depth.set(depth.get().saturating_sub(1)));
    }
}

static PARALLEL_CONFIG: OnceLock<ParallelConfig> = OnceLock::new();
static ACTIVE_EXTRA_WORKERS: AtomicUsize = AtomicUsize::new(0);

thread_local! {
    static PARALLEL_DEPTH: Cell<usize> = const { Cell::new(0) };
}

fn parse_boolish_mode(value: &str) -> Option<ParallelMode> {
    match value.trim().to_ascii_lowercase().as_str() {
        "0" | "false" | "off" | "no" | "nein" | "serial" | "seriell" => {
            Some(ParallelMode::Off)
        }
        "1" | "true" | "on" | "yes" | "ja" | "parallel" => Some(ParallelMode::On),
        "auto" | "" => Some(ParallelMode::Auto),
        _ => None,
    }
}

fn env_mode(name: &str) -> Option<ParallelMode> {
    std::env::var(name)
        .ok()
        .and_then(|value| parse_boolish_mode(&value))
}

fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
}

fn env_flag_is_set(name: &str) -> bool {
    std::env::var_os(name).is_some()
}

pub fn parallel_config() -> &'static ParallelConfig {
    PARALLEL_CONFIG.get_or_init(|| {
        let available = std::thread::available_parallelism()
            .map(NonZeroUsize::get)
            .unwrap_or(1);
        let max_jobs = env_usize("RETA_JOBS")
            .or_else(|| env_usize("RETA_THREADS"))
            .or_else(|| env_usize("RETA_NUM_THREADS"))
            .unwrap_or(available)
            .max(1);
        let global_mode = env_mode("RETA_PARALLEL").unwrap_or(ParallelMode::Auto);
        let global_min_items = env_usize("RETA_PARALLEL_MIN_ITEMS")
            .or_else(|| env_usize("RETA_PARALLEL_MIN"));
        let allow_nested = matches!(
            env_mode("RETA_PARALLEL_ALLOW_NESTED"),
            Some(ParallelMode::On)
        );

        ParallelConfig {
            global_mode,
            max_jobs,
            global_min_items,
            allow_nested,
        }
    })
}

fn area_env_names(area: ParallelArea, suffix: &str) -> Vec<String> {
    let mut names = Vec::with_capacity(1 + area.extra_env_prefixes().len());
    names.push(format!("{}{}", area.env_prefix(), suffix));
    for prefix in area.extra_env_prefixes() {
        names.push(format!("{}{}", prefix, suffix));
    }
    names
}

fn area_mode(area: ParallelArea) -> Option<ParallelMode> {
    area_env_names(area, "_PARALLEL")
        .into_iter()
        .chain(area_env_names(area, "_PARALLEL_ENABLED"))
        .find_map(|name| env_mode(&name))
}

fn area_serial_flag(area: ParallelArea) -> bool {
    area_env_names(area, "_SERIAL")
        .into_iter()
        .any(|name| env_flag_is_set(&name))
}

fn area_min_items(area: ParallelArea) -> Option<usize> {
    area_env_names(area, "_PARALLEL_MIN_ITEMS")
        .into_iter()
        .chain(area_env_names(area, "_PARALLEL_MIN"))
        .find_map(|name| env_usize(&name))
}

fn area_enabled(area: ParallelArea, cfg: &ParallelConfig) -> bool {
    if cfg.max_jobs <= 1 {
        return false;
    }
    if matches!(cfg.global_mode, ParallelMode::Off) {
        return false;
    }
    if area_serial_flag(area) {
        return false;
    }

    match area_mode(area) {
        Some(ParallelMode::Off) => false,
        Some(ParallelMode::On) => true,
        Some(ParallelMode::Auto) | None => match cfg.global_mode {
            ParallelMode::Off => false,
            ParallelMode::On => true,
            ParallelMode::Auto => area.default_enabled_in_auto(),
        },
    }
}

fn in_parallel_scope() -> bool {
    PARALLEL_DEPTH.with(|depth| depth.get() > 0)
}

fn effective_min_items(area: ParallelArea, fallback_min_items: usize, cfg: &ParallelConfig) -> usize {
    area_min_items(area)
        .or(cfg.global_min_items)
        .unwrap_or(fallback_min_items)
        .max(1)
}

pub fn reserve_worker_count(
    area: ParallelArea,
    item_count: usize,
    fallback_min_items: usize,
    wanted_workers: usize,
) -> Option<ParallelWorkGuard> {
    if item_count <= 1 || wanted_workers <= 1 {
        return None;
    }

    let cfg = parallel_config();
    if !area_enabled(area, cfg) {
        return None;
    }
    if in_parallel_scope() && !cfg.allow_nested {
        return None;
    }

    let min_items = effective_min_items(area, fallback_min_items, cfg);
    if item_count < min_items {
        return None;
    }

    let wanted = wanted_workers.min(item_count).min(cfg.max_jobs).max(1);
    if wanted <= 1 {
        return None;
    }

    let max_extra = cfg.max_jobs.saturating_sub(1);
    let wanted_extra = wanted.saturating_sub(1).min(max_extra);
    if wanted_extra == 0 {
        return None;
    }

    loop {
        let current = ACTIVE_EXTRA_WORKERS.load(Ordering::Acquire);
        let remaining_extra = max_extra.saturating_sub(current);
        if remaining_extra == 0 {
            return None;
        }
        let extra = wanted_extra.min(remaining_extra);
        let new_value = current.saturating_add(extra);
        if ACTIVE_EXTRA_WORKERS
            .compare_exchange(current, new_value, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return Some(ParallelWorkGuard {
                workers: extra + 1,
                extra_workers: extra,
            });
        }
    }
}

pub fn split_ranges(total_items: usize, worker_count: usize) -> Vec<(usize, usize)> {
    if total_items == 0 {
        return Vec::new();
    }
    let workers = worker_count.max(1).min(total_items);
    if workers <= 1 {
        return vec![(0, total_items)];
    }
    let chunk_size = (total_items + workers - 1) / workers;
    let mut ranges = Vec::with_capacity(workers);
    let mut start = 0usize;
    while start < total_items {
        let end = start.saturating_add(chunk_size).min(total_items);
        ranges.push((start, end));
        start = end;
    }
    ranges
}

pub fn reserve_ranges(
    area: ParallelArea,
    total_items: usize,
    min_items_per_worker: usize,
) -> Option<(ParallelWorkGuard, Vec<(usize, usize)>)> {
    if total_items == 0 {
        return None;
    }
    let grain = min_items_per_worker.max(1);
    let max_workers_by_grain = (total_items + grain - 1) / grain;
    let fallback_min_items = grain.saturating_mul(2).max(2);
    let guard = reserve_worker_count(area, total_items, fallback_min_items, max_workers_by_grain)?;
    let ranges = guard.ranges(total_items);
    if ranges.len() <= 1 {
        None
    } else {
        Some((guard, ranges))
    }
}

pub fn parallel_map_indexed<T, F>(
    area: ParallelArea,
    len: usize,
    fallback_min_items: usize,
    f: F,
) -> Vec<T>
where
    T: Send,
    F: Fn(usize) -> T + Sync,
{
    if len == 0 {
        return Vec::new();
    }

    let Some(guard) = reserve_worker_count(area, len, fallback_min_items, len) else {
        return (0..len).map(f).collect();
    };
    let ranges = guard.ranges(len);
    if ranges.len() <= 1 {
        return (0..len).map(f).collect();
    }

    std::thread::scope(|scope| {
        let _budget_guard = guard;
        let f_ref = &f;
        let mut handles = Vec::with_capacity(ranges.len());
        for (start, end) in ranges {
            handles.push(scope.spawn(move || {
                let _depth_guard = ParallelDepthGuard::enter();
                let mut chunk = Vec::with_capacity(end - start);
                for index in start..end {
                    chunk.push(f_ref(index));
                }
                chunk
            }));
        }

        let mut out = Vec::with_capacity(len);
        for handle in handles {
            match handle.join() {
                Ok(mut chunk) => out.append(&mut chunk),
                Err(payload) => std::panic::resume_unwind(payload),
            }
        }
        out
    })
}

pub fn enter_parallel_worker_scope() -> impl Drop {
    ParallelDepthGuard::enter()
}

#[cfg(test)]
mod tests {
    use super::split_ranges;

    #[test]
    fn split_ranges_preserves_order_and_coverage() {
        let ranges = split_ranges(10, 3);
        assert_eq!(ranges, vec![(0, 4), (4, 8), (8, 10)]);
    }

    #[test]
    fn split_ranges_handles_empty_input() {
        assert!(split_ranges(0, 8).is_empty());
    }

    #[test]
    fn split_ranges_caps_workers_to_items() {
        let ranges = split_ranges(3, 99);
        assert_eq!(ranges, vec![(0, 1), (1, 2), (2, 3)]);
    }
}
