//! Deterministic parallel-execution planning transcompiled from
//! `python_arch_reference/reta_architecture/parallel_execution.py`.
//!
//! Python used multiprocessing for heavy row/cell chunks.  Rust already has a
//! safer native execution model, so this module keeps the same switches,
//! chunking, FIFO/LIFO dataflow invariants and gluing order as typed planning
//! primitives.  Actual callers may execute serially or in worker pools while
//! preserving the universal property: ordered chunks glue to the same result.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::arithmetic::factor_pairs;
use crate::number_theory::{is_prime_multiple, moon_number, prime_factors};

const OFF_VALUES: &[&str] = &["", "0", "off", "false", "no", "none", "serial", "single"];
const AUTO_VALUES: &[&str] = &["auto", "pypy", "pypy3"];
const PROCESS_VALUES: &[&str] = &[
    "1",
    "on",
    "true",
    "yes",
    "process",
    "processes",
    "multiprocess",
    "multiprocessing",
    "mp",
];

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProcessorCoreCounts {
    pub physical: usize,
    pub virtual_count: usize,
    pub available: usize,
}

impl ProcessorCoreCounts {
    pub fn default_workers(&self) -> usize {
        self.available
            .max(self.virtual_count)
            .max(self.physical)
            .max(1)
    }

    pub fn snapshot(&self) -> BTreeMap<String, usize> {
        BTreeMap::from([
            ("physical".to_string(), self.physical),
            ("virtual".to_string(), self.virtual_count),
            ("available".to_string(), self.available),
            ("default_workers".to_string(), self.default_workers()),
        ])
    }
}

pub fn detect_processor_core_counts() -> ProcessorCoreCounts {
    let available = std::thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(1)
        .max(1);
    ProcessorCoreCounts {
        physical: available,
        virtual_count: available,
        available,
    }
}

pub fn normalise_parallel_mode(value: Option<&str>) -> String {
    let mode = value.unwrap_or("auto").trim().to_ascii_lowercase();
    if OFF_VALUES.contains(&mode.as_str()) {
        "off".to_string()
    } else if PROCESS_VALUES.contains(&mode.as_str()) {
        "processes".to_string()
    } else if AUTO_VALUES.contains(&mode.as_str()) {
        "auto".to_string()
    } else if mode.is_empty() {
        "auto".to_string()
    } else {
        mode
    }
}

pub fn positive_int(value: Option<&str>, default: Option<usize>) -> Option<usize> {
    match value
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|v| *v > 0)
    {
        Some(parsed) => Some(parsed),
        None => default,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParallelExecutionConfig {
    pub mode: String,
    pub workers: Option<usize>,
    pub chunk_size: usize,
    pub threshold: usize,
    pub start_method: Option<String>,
    pub source: String,
}

impl Default for ParallelExecutionConfig {
    fn default() -> Self {
        Self::new("auto", None, 64, 128, None, "defaults")
    }
}

impl ParallelExecutionConfig {
    pub fn new(
        mode: impl AsRef<str>,
        workers: Option<usize>,
        chunk_size: usize,
        threshold: usize,
        start_method: Option<String>,
        source: impl Into<String>,
    ) -> Self {
        let start_method = start_method.and_then(|value| {
            let lowered = value.trim().to_ascii_lowercase();
            if lowered.is_empty() || lowered == "default" || lowered == "none" {
                None
            } else {
                Some(value)
            }
        });
        Self {
            mode: normalise_parallel_mode(Some(mode.as_ref())),
            workers: workers.filter(|value| *value > 0),
            chunk_size: chunk_size.max(1),
            threshold: threshold.max(1),
            start_method,
            source: source.into(),
        }
    }

    pub fn from_environment() -> Self {
        let mode = std::env::var("RETA_PARALLEL_MODE")
            .ok()
            .or_else(|| std::env::var("RETA_PARALLEL").ok())
            .unwrap_or_else(|| "auto".to_string());
        let workers = positive_int(std::env::var("RETA_PARALLEL_WORKERS").ok().as_deref(), None);
        let chunk_size = positive_int(
            std::env::var("RETA_PARALLEL_CHUNK_SIZE").ok().as_deref(),
            Some(64),
        )
        .unwrap_or(64);
        let threshold = positive_int(
            std::env::var("RETA_PARALLEL_THRESHOLD").ok().as_deref(),
            Some(128),
        )
        .unwrap_or(128);
        let start_method = std::env::var("RETA_PARALLEL_START_METHOD").ok();
        let source = if std::env::var("RETA_PARALLEL").is_ok()
            || std::env::var("RETA_PARALLEL_MODE").is_ok()
        {
            "environment"
        } else {
            "defaults"
        };
        Self::new(mode, workers, chunk_size, threshold, start_method, source)
    }

    pub fn with_overrides(
        &self,
        mode: Option<String>,
        workers: Option<Option<usize>>,
        chunk_size: Option<usize>,
        threshold: Option<usize>,
        start_method: Option<Option<String>>,
        source: Option<String>,
    ) -> Self {
        Self::new(
            mode.as_deref().unwrap_or(&self.mode),
            workers.unwrap_or(self.workers),
            chunk_size.unwrap_or(self.chunk_size),
            threshold.unwrap_or(self.threshold),
            start_method.unwrap_or_else(|| self.start_method.clone()),
            source.unwrap_or_else(|| self.source.clone()),
        )
    }

    pub fn resolved_workers(&self) -> usize {
        self.workers
            .unwrap_or_else(|| detect_processor_core_counts().default_workers())
            .max(1)
    }

    pub fn enabled_by_mode(&self) -> bool {
        match self.mode.as_str() {
            "off" => false,
            "auto" => cfg!(target_family = "wasm"), // conservative, like CPython-off/PyPy-on policy
            "processes" => true,
            value => PROCESS_VALUES.contains(&value),
        }
    }

    pub fn should_use_processes(&self, row_count: usize) -> bool {
        self.enabled_by_mode()
            && self.resolved_workers() > 1
            && row_count >= self.threshold
            && self.chunk_size > 0
    }

    pub fn snapshot(&self) -> ParallelExecutionConfigSnapshot {
        ParallelExecutionConfigSnapshot {
            class: "ParallelExecutionConfig".to_string(),
            mode: self.mode.clone(),
            enabled_by_mode: self.enabled_by_mode(),
            workers: self.workers,
            resolved_workers: self.resolved_workers(),
            chunk_size: self.chunk_size,
            threshold: self.threshold,
            start_method: self.start_method.clone(),
            runtime: "rust".to_string(),
            source: self.source.clone(),
            processor_cores: detect_processor_core_counts(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParallelExecutionConfigSnapshot {
    pub class: String,
    pub mode: String,
    pub enabled_by_mode: bool,
    pub workers: Option<usize>,
    pub resolved_workers: usize,
    pub chunk_size: usize,
    pub threshold: usize,
    pub start_method: Option<String>,
    pub runtime: String,
    pub source: String,
    pub processor_cores: ProcessorCoreCounts,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParallelRowsResult<T = String> {
    pub rows: Vec<T>,
    pub religion_numbers: Vec<i64>,
    pub workers: usize,
    pub chunks: usize,
    pub row_count: usize,
    pub config: ParallelExecutionConfig,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParallelOperationResult<T = String> {
    pub operation: String,
    pub values: Vec<T>,
    pub workers: usize,
    pub chunks: usize,
    pub item_count: usize,
    pub config: ParallelExecutionConfig,
}

impl<T> ParallelOperationResult<T> {
    pub fn snapshot_len(&self) -> ParallelOperationSnapshot {
        ParallelOperationSnapshot {
            class: "ParallelOperationResult".to_string(),
            operation: self.operation.clone(),
            values: self.values.len(),
            workers: self.workers,
            chunks: self.chunks,
            item_count: self.item_count,
            config: self.config.snapshot(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParallelOperationSnapshot {
    pub class: String,
    pub operation: String,
    pub values: usize,
    pub workers: usize,
    pub chunks: usize,
    pub item_count: usize,
    pub config: ParallelExecutionConfigSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParallelExecutionSnapshot {
    pub class: String,
    pub strategy: String,
    pub execution_network: String,
    pub config: ParallelExecutionConfigSnapshot,
    pub processor_cores: ProcessorCoreCounts,
    pub morphisms: Vec<String>,
    pub default_policy: String,
    pub default_workers: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParallelExecutionBundle {
    pub config: ParallelExecutionConfig,
}

impl ParallelExecutionBundle {
    pub fn snapshot(&self) -> ParallelExecutionSnapshot {
        let cores = detect_processor_core_counts();
        ParallelExecutionSnapshot {
            class: "ParallelExecutionBundle".to_string(),
            strategy: "process_chunked_table_work".to_string(),
            execution_network: "reta_architecture.dataflow.ExecutionNetworkBundle".to_string(),
            config: self.config.snapshot(),
            processor_cores: cores.clone(),
            morphisms: vec![
                "extract_parallel_config_from_argv".to_string(),
                "prepare_rows_in_processes".to_string(),
                "decode_religion_rows_in_processes".to_string(),
                "decode_kombi_rows_in_processes".to_string(),
                "select_columns_in_processes".to_string(),
                "max_cell_text_len_in_processes".to_string(),
                "prepare_kombi_join_tables_in_processes".to_string(),
                "moon_numbers_in_processes".to_string(),
                "prime_factors_in_processes".to_string(),
                "filter_numbers_in_processes".to_string(),
                "factor_pairs_in_processes".to_string(),
                "normalize_column_buckets_in_processes".to_string(),
                "glue_parallel_row_chunks".to_string(),
            ],
            default_policy: "auto_on_pypy_off_on_cpython_rust_explicit_only".to_string(),
            default_workers: cores.default_workers(),
        }
    }
}

pub fn bootstrap_parallel_execution(
    config: Option<ParallelExecutionConfig>,
) -> ParallelExecutionBundle {
    ParallelExecutionBundle {
        config: config.unwrap_or_else(ParallelExecutionConfig::from_environment),
    }
}

fn consume_value(argv: &[String], index: usize) -> (Option<String>, usize) {
    let next_index = index + 1;
    if next_index < argv.len() && !argv[next_index].starts_with('-') {
        (Some(argv[next_index].clone()), 1)
    } else {
        (None, 0)
    }
}

pub fn extract_parallel_config_from_argv(
    argv: &[String],
    inherited: Option<ParallelExecutionConfig>,
) -> (Vec<String>, ParallelExecutionConfig) {
    let base = inherited.unwrap_or_else(ParallelExecutionConfig::from_environment);
    let mut clean = Vec::new();
    let mut skip = 0usize;
    let mut recognised = false;
    let mut mode: Option<String> = None;
    let mut workers: Option<Option<usize>> = None;
    let mut chunk_size: Option<usize> = None;
    let mut threshold: Option<usize> = None;
    let mut start_method: Option<Option<String>> = None;

    for (index, raw) in argv.iter().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        let arg = raw.trim().to_string();
        match arg.as_str() {
            "--no-parallel" => {
                mode = Some("off".to_string());
                recognised = true;
            }
            "--parallel" => {
                mode = Some("processes".to_string());
                recognised = true;
            }
            "--parallel-workers" | "--parallel-worker" | "--parallel-prozesse" => {
                let (value, consumed) = consume_value(argv, index);
                workers = Some(positive_int(value.as_deref(), base.workers));
                skip = consumed;
                recognised = true;
            }
            "--parallel-chunk-size" | "--parallel-chunksize" | "--parallel-chunk" => {
                let (value, consumed) = consume_value(argv, index);
                chunk_size = positive_int(value.as_deref(), Some(base.chunk_size));
                skip = consumed;
                recognised = true;
            }
            "--parallel-threshold" | "--parallel-min-rows" => {
                let (value, consumed) = consume_value(argv, index);
                threshold = positive_int(value.as_deref(), Some(base.threshold));
                skip = consumed;
                recognised = true;
            }
            "--parallel-start-method" | "--parallel-start" => {
                let (value, consumed) = consume_value(argv, index);
                start_method = Some(value);
                skip = consumed;
                recognised = true;
            }
            _ if arg.starts_with("--parallel=") => {
                mode = Some(
                    arg.split_once('=')
                        .map(|(_, value)| value.to_string())
                        .unwrap_or_default(),
                );
                recognised = true;
            }
            _ if arg.starts_with("--parallel-workers=")
                || arg.starts_with("--parallel-worker=")
                || arg.starts_with("--parallel-prozesse=") =>
            {
                let value = arg.split_once('=').map(|(_, value)| value.to_string());
                workers = Some(positive_int(value.as_deref(), base.workers));
                recognised = true;
            }
            _ if arg.starts_with("--parallel-chunk-size=")
                || arg.starts_with("--parallel-chunksize=")
                || arg.starts_with("--parallel-chunk=") =>
            {
                let value = arg.split_once('=').map(|(_, value)| value.to_string());
                chunk_size = positive_int(value.as_deref(), Some(base.chunk_size));
                recognised = true;
            }
            _ if arg.starts_with("--parallel-threshold=")
                || arg.starts_with("--parallel-min-rows=") =>
            {
                let value = arg.split_once('=').map(|(_, value)| value.to_string());
                threshold = positive_int(value.as_deref(), Some(base.threshold));
                recognised = true;
            }
            _ if arg.starts_with("--parallel-start-method=")
                || arg.starts_with("--parallel-start=") =>
            {
                start_method = Some(arg.split_once('=').map(|(_, value)| value.to_string()));
                recognised = true;
            }
            _ => clean.push(arg),
        }
    }

    let source = recognised.then(|| "argv".to_string());
    let config = base.with_overrides(mode, workers, chunk_size, threshold, start_method, source);
    (clean, config)
}

pub fn apply_parallel_environment_pairs(config: &ParallelExecutionConfig) -> Vec<(String, String)> {
    let mut pairs = vec![
        ("RETA_PARALLEL_MODE".to_string(), config.mode.clone()),
        (
            "RETA_PARALLEL_CHUNK_SIZE".to_string(),
            config.chunk_size.to_string(),
        ),
        (
            "RETA_PARALLEL_THRESHOLD".to_string(),
            config.threshold.to_string(),
        ),
    ];
    if let Some(workers) = config.workers {
        pairs.push(("RETA_PARALLEL_WORKERS".to_string(), workers.to_string()));
    }
    if let Some(start_method) = &config.start_method {
        pairs.push((
            "RETA_PARALLEL_START_METHOD".to_string(),
            start_method.clone(),
        ));
    }
    pairs
}

pub fn chunk_items<T: Clone>(items: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    let chunk_size = chunk_size.max(1);
    items
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

pub fn glue_parallel_row_chunks<T: Clone>(chunks: &[Vec<(usize, T)>]) -> Vec<(usize, T)> {
    let mut rows = Vec::new();
    for chunk in chunks {
        rows.extend(chunk.iter().cloned());
    }
    rows.sort_by_key(|(index, _)| *index);
    rows
}

pub fn moon_numbers_in_processes(
    numbers: &[i64],
    config: Option<ParallelExecutionConfig>,
) -> Option<ParallelOperationResult<(i64, (Vec<i64>, Vec<i64>))>> {
    let config = config.unwrap_or_else(ParallelExecutionConfig::from_environment);
    let mut ordered = numbers.to_vec();
    ordered.sort_unstable();
    if !config.should_use_processes(ordered.len())
        || chunk_items(&ordered, config.chunk_size).len() <= 1
    {
        return None;
    }
    let values = ordered
        .into_iter()
        .map(|number| (number, moon_number(number)))
        .collect();
    Some(ParallelOperationResult {
        operation: "moon_numbers".to_string(),
        values,
        workers: config.resolved_workers(),
        chunks: numbers.len().div_ceil(config.chunk_size),
        item_count: numbers.len(),
        config,
    })
}

pub fn prime_factors_in_processes(
    numbers: &[i64],
    config: Option<ParallelExecutionConfig>,
) -> Option<ParallelOperationResult<(i64, Vec<i64>)>> {
    let config = config.unwrap_or_else(ParallelExecutionConfig::from_environment);
    let mut ordered = numbers.to_vec();
    ordered.sort_unstable();
    if !config.should_use_processes(ordered.len())
        || chunk_items(&ordered, config.chunk_size).len() <= 1
    {
        return None;
    }
    let values = ordered
        .into_iter()
        .map(|number| (number, prime_factors(number)))
        .collect();
    Some(ParallelOperationResult {
        operation: "prime_factors".to_string(),
        values,
        workers: config.resolved_workers(),
        chunks: numbers.len().div_ceil(config.chunk_size),
        item_count: numbers.len(),
        config,
    })
}

pub fn factor_pairs_in_processes(
    numbers: &[i64],
    include_one: bool,
    config: Option<ParallelExecutionConfig>,
) -> Option<ParallelOperationResult<(i64, Vec<(i64, i64)>)>> {
    let config = config.unwrap_or_else(ParallelExecutionConfig::from_environment);
    let mut ordered = numbers.to_vec();
    ordered.sort_unstable();
    if !config.should_use_processes(ordered.len())
        || chunk_items(&ordered, config.chunk_size).len() <= 1
    {
        return None;
    }
    let values = ordered
        .into_iter()
        .map(|number| (number, factor_pairs(number, include_one)))
        .collect();
    Some(ParallelOperationResult {
        operation: "factor_pairs".to_string(),
        values,
        workers: config.resolved_workers(),
        chunks: numbers.len().div_ceil(config.chunk_size),
        item_count: numbers.len(),
        config,
    })
}

pub fn filter_numbers_in_processes(
    numbers: &[i64],
    mode: &str,
    criteria: &[i64],
    config: Option<ParallelExecutionConfig>,
) -> Option<ParallelOperationResult<i64>> {
    let config = config.unwrap_or_else(ParallelExecutionConfig::from_environment);
    let mut ordered = numbers.to_vec();
    ordered.sort_unstable();
    if !config.should_use_processes(ordered.len())
        || chunk_items(&ordered, config.chunk_size).len() <= 1
    {
        return None;
    }
    let mode = mode.to_string();
    let values = ordered
        .into_iter()
        .filter(|number| match mode.as_str() {
            "prime_multiples" => is_prime_multiple(*number, criteria),
            "ordinary_multiples" => criteria
                .iter()
                .any(|divisor| *divisor != 0 && number % divisor == 0),
            "modulo" if criteria.len() >= 2 => {
                criteria[0] != 0 && number % criteria[0] == criteria[1]
            }
            "moon" => {
                (moon_number(*number).0.is_empty()) != (criteria.first().copied().unwrap_or(0) != 0)
            }
            "sonne_mit_mondanteil" => {
                let repeated = crate::number_theory::prime_repeat(&prime_factors(*number));
                let has_single = repeated.iter().any(|(_, amount)| *amount == 1);
                let has_repeated = repeated.iter().any(|(_, amount)| *amount != 1);
                has_single && has_repeated
            }
            _ => false,
        })
        .collect::<Vec<_>>();
    Some(ParallelOperationResult {
        operation: format!("filter_numbers:{mode}"),
        values,
        workers: config.resolved_workers(),
        chunks: numbers.len().div_ceil(config.chunk_size),
        item_count: numbers.len(),
        config,
    })
}

pub fn normalize_column_buckets_in_processes(
    spalten_arten: &BTreeMap<(i64, i64), BTreeSet<i64>>,
    config: Option<ParallelExecutionConfig>,
) -> Option<ParallelOperationResult<((i64, i64), BTreeSet<i64>)>> {
    let config = config.unwrap_or_else(ParallelExecutionConfig::from_environment);
    let item_count: usize = spalten_arten.values().map(BTreeSet::len).sum();
    if !config.should_use_processes(item_count) || spalten_arten.len() <= 1 {
        return None;
    }
    let max_type = (spalten_arten.len() / 2) as i64;
    let mut values = Vec::new();
    for bucket_type in 0..max_type {
        if let (Some(positive), Some(negative)) = (
            spalten_arten.get(&(0, bucket_type)),
            spalten_arten.get(&(1, bucket_type)),
        ) {
            let normalised = positive
                .difference(negative)
                .copied()
                .collect::<BTreeSet<_>>();
            values.push(((0, bucket_type), normalised));
        }
    }
    Some(ParallelOperationResult {
        operation: "normalize_column_buckets".to_string(),
        values,
        workers: config.resolved_workers(),
        chunks: spalten_arten.len(),
        item_count,
        config,
    })
}



#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkerPrepare {
    pub text_width: usize,
    pub shell_rows_amount: Option<String>,
    pub breiten: Vec<usize>,
}

impl Default for WorkerPrepare {
    fn default() -> Self {
        Self { text_width: 21, shell_rows_amount: None, breiten: Vec::new() }
    }
}

impl WorkerPrepare {
    pub fn snapshot(&self) -> BTreeMap<String, String> {
        BTreeMap::from([
            ("class".to_string(), "WorkerPrepare".to_string()),
            ("text_width".to_string(), self.text_width.to_string()),
            ("breiten".to_string(), self.breiten.len().to_string()),
        ])
    }

    pub fn set_width(&self, row_to_display: usize, row_count: usize) -> usize {
        self.breiten
            .get(row_to_display)
            .copied()
            .unwrap_or(self.text_width.max(row_count))
    }

    pub fn wrapping(&self, text: &str, width: usize) -> Vec<String> {
        crate::table_wrapping::wrap_cell_text(text, width, None).unwrap_or_else(|| vec![text.to_string()])
    }
}

pub fn __init__() -> WorkerPrepare {
    WorkerPrepare::default()
}

pub fn __post_init__(config: ParallelExecutionConfig) -> ParallelExecutionConfig {
    ParallelExecutionConfig::new(config.mode, config.workers, config.chunk_size, config.threshold, config.start_method, config.source)
}

pub fn _available_virtual_cpu_count() -> usize {
    detect_processor_core_counts().available
}

pub fn _linux_physical_cpu_count() -> usize {
    detect_processor_core_counts().physical
}

pub fn _normalise_mode(value: Option<&str>) -> String {
    normalise_parallel_mode(value)
}

pub fn _positive_int(value: Option<&str>, default: Option<usize>) -> Option<usize> {
    positive_int(value, default)
}

pub fn is_pypy_runtime() -> bool {
    false
}

pub fn _default_start_method() -> Option<String> {
    None
}

pub fn _consume_value(argv: &[String], index: usize) -> (Option<String>, usize) {
    consume_value(argv, index)
}

pub fn _chunks<T: Clone>(items: &[T], chunk_size: usize) -> Vec<Vec<T>> {
    chunk_items(items, chunk_size)
}

pub fn _parallel_context_from_prepare(prepare: &WorkerPrepare) -> ParallelExecutionConfig {
    ParallelExecutionConfig::default().with_overrides(None, None, Some(prepare.text_width.max(1)), None, None, Some("worker-prepare".to_string()))
}

pub fn _pool_map_ordered<T, U, F>(items: &[T], handler: F, config: Option<ParallelExecutionConfig>) -> Vec<U>
where
    T: Clone,
    U: Clone,
    F: Fn(&T) -> U,
{
    let _config = config.unwrap_or_default();
    items.iter().map(handler).collect()
}

pub fn apply_parallel_environment(config: &ParallelExecutionConfig) -> Vec<(String, String)> {
    apply_parallel_environment_pairs(config)
}

pub fn _decode_religion_cell_static(cell: &str) -> Vec<i64> {
    cell.split(|ch: char| !ch.is_ascii_digit() && ch != '-')
        .filter_map(|part| part.parse::<i64>().ok())
        .collect()
}

pub fn _decode_religion_rows_worker(rows: &[String]) -> Vec<Vec<i64>> {
    rows.iter().map(|row| _decode_religion_cell_static(row)).collect()
}

pub fn _parse_kombi_number_static(cell: &str) -> Option<i64> {
    cell.trim().parse::<i64>().ok()
}

pub fn _decode_kombi_rows_worker(rows: &[String]) -> Vec<Option<i64>> {
    rows.iter().map(|row| _parse_kombi_number_static(row)).collect()
}

pub fn _select_columns_worker(row: &[String], columns: &[usize]) -> Vec<String> {
    columns.iter().filter_map(|idx| row.get(*idx).cloned()).collect()
}

pub fn _max_cell_text_len_worker(row: &[String]) -> usize {
    row.iter().map(|cell| cell.chars().count()).max().unwrap_or(0)
}

pub fn _prepare_kombi_join_tables_worker(table: &[Vec<String>]) -> Vec<Vec<String>> {
    table.to_vec()
}

pub fn _moon_numbers_worker(number: i64) -> (Vec<i64>, Vec<i64>) {
    moon_number(number)
}

pub fn _prime_factors_worker(number: i64) -> Vec<i64> {
    prime_factors(number)
}

pub fn _number_filter_worker(number: i64, mode: &str, criteria: &[i64]) -> bool {
    match mode {
        "prime_multiples" => is_prime_multiple(number, criteria),
        "ordinary_multiples" => criteria.iter().any(|divisor| *divisor != 0 && number % divisor == 0),
        _ => true,
    }
}

pub fn _factor_pairs_worker(number: i64, include_one: bool) -> Vec<(i64, i64)> {
    factor_pairs(number, include_one)
}

pub fn _normalize_column_bucket_worker(positive: &BTreeSet<i64>, negative: &BTreeSet<i64>) -> BTreeSet<i64> {
    positive.difference(negative).copied().collect()
}

pub fn set_width(prepare: &WorkerPrepare, row_to_display: usize, row_count: usize) -> usize {
    prepare.set_width(row_to_display, row_count)
}

pub fn wrapping(prepare: &WorkerPrepare, text: &str, width: usize) -> Vec<String> {
    prepare.wrapping(text, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argv_parallel_flags_are_stripped() {
        let argv = vec![
            "reta".into(),
            "--parallel=processes".into(),
            "--parallel-workers".into(),
            "4".into(),
            "-zeilen".into(),
        ];
        let (clean, cfg) =
            extract_parallel_config_from_argv(&argv, Some(ParallelExecutionConfig::default()));
        assert_eq!(clean, vec!["reta".to_string(), "-zeilen".to_string()]);
        assert_eq!(cfg.mode, "processes");
        assert_eq!(cfg.workers, Some(4));
        assert_eq!(cfg.source, "argv");
    }

    #[test]
    fn chunks_glue_back_in_order() {
        let chunks = vec![vec![(2, "c"), (0, "a")], vec![(1, "b")]];
        assert_eq!(
            glue_parallel_row_chunks(&chunks),
            vec![(0, "a"), (1, "b"), (2, "c")]
        );
    }
}
