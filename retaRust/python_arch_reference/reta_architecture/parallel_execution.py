from __future__ import annotations

"""Process-based parallel row preparation for Reta.

The Reta table builders are intentionally mutable while columns are generated.
That phase is kept serial.  This module parallelises the later, deterministic
row/cell preparation phase by sending already-selected rows to worker processes
and gluing the chunks back in source order.
"""

import multiprocessing
import os
import platform
import html
import json
from collections import OrderedDict
from copy import deepcopy
from dataclasses import dataclass, replace
from typing import Iterable, Mapping, Sequence

_OFF_VALUES = {"", "0", "off", "false", "no", "none", "serial", "single"}
_AUTO_VALUES = {"auto", "pypy", "pypy3"}
_PROCESS_VALUES = {"1", "on", "true", "yes", "process", "processes", "multiprocess", "multiprocessing", "mp"}


@dataclass(frozen=True)
class ProcessorCoreCounts:
    """Physical/logical processor counts used by Reta process pools.

    ``virtual`` is the logical CPU count visible to the current process.  On
    Linux this respects CPU affinity when available.  ``physical`` is best
    effort; if the OS does not expose it cheaply, it falls back to ``virtual``.
    Reta uses ``default_workers`` for process pools, because PyPy3 multiprocessing
    benefits from one worker per schedulable logical core for CPU-heavy chunks.
    """

    physical: int
    virtual: int
    available: int

    @property
    def default_workers(self) -> int:
        return max(1, self.available or self.virtual or self.physical or 1)

    def snapshot(self) -> dict:
        return {
            "physical": self.physical,
            "virtual": self.virtual,
            "available": self.available,
            "default_workers": self.default_workers,
        }


def _available_virtual_cpu_count() -> int:
    process_cpu_count = getattr(os, "process_cpu_count", None)
    if callable(process_cpu_count):
        try:
            value = process_cpu_count()
            if value:
                return max(1, int(value))
        except Exception:
            pass
    sched_getaffinity = getattr(os, "sched_getaffinity", None)
    if callable(sched_getaffinity):
        try:
            return max(1, len(sched_getaffinity(0)))
        except Exception:
            pass
    return max(1, os.cpu_count() or 1)


def _linux_physical_cpu_count() -> int | None:
    cpuinfo = "/proc/cpuinfo"
    if not os.path.exists(cpuinfo):
        return None
    physical_core_pairs: set[tuple[str, str]] = set()
    physical_id: str | None = None
    core_id: str | None = None
    processor_count = 0
    try:
        with open(cpuinfo, "r", encoding="utf-8", errors="ignore") as handle:
            for raw_line in handle:
                line = raw_line.strip()
                if not line:
                    if physical_id is not None and core_id is not None:
                        physical_core_pairs.add((physical_id, core_id))
                    physical_id = None
                    core_id = None
                    continue
                if line.startswith("processor") and ":" in line:
                    processor_count += 1
                elif line.startswith("physical id") and ":" in line:
                    physical_id = line.split(":", 1)[1].strip()
                elif line.startswith("core id") and ":" in line:
                    core_id = line.split(":", 1)[1].strip()
        if physical_id is not None and core_id is not None:
            physical_core_pairs.add((physical_id, core_id))
    except Exception:
        return None
    if physical_core_pairs:
        return max(1, len(physical_core_pairs))
    return max(1, processor_count) if processor_count else None


def detect_processor_core_counts() -> ProcessorCoreCounts:
    virtual = max(1, os.cpu_count() or 1)
    available = _available_virtual_cpu_count()
    physical = _linux_physical_cpu_count() or virtual
    physical = max(1, min(int(physical), virtual))
    available = max(1, min(int(available), virtual))
    return ProcessorCoreCounts(physical=physical, virtual=virtual, available=available)


# Globale Prozessor-Kernzahlen: von allen Parallelisierungsstellen gemeinsam benutzt.
RETA_PROCESSOR_CORES = detect_processor_core_counts()
RETA_PHYSICAL_PROCESSOR_CORES = RETA_PROCESSOR_CORES.physical
RETA_VIRTUAL_PROCESSOR_CORES = RETA_PROCESSOR_CORES.virtual
RETA_AVAILABLE_PROCESSOR_CORES = RETA_PROCESSOR_CORES.available
RETA_PARALLEL_PROCESSOR_CORES = RETA_PROCESSOR_CORES.default_workers


def _normalise_mode(value: object | None) -> str:
    if value is None:
        return "auto"
    mode = str(value).strip().lower()
    if mode in _OFF_VALUES:
        return "off"
    if mode in _PROCESS_VALUES:
        return "processes"
    if mode in _AUTO_VALUES:
        return "auto"
    return mode or "auto"


def _positive_int(value: object | None, default: int | None = None) -> int | None:
    if value is None or value == "":
        return default
    try:
        parsed = int(str(value).strip())
    except (TypeError, ValueError):
        return default
    return parsed if parsed > 0 else default


def is_pypy_runtime() -> bool:
    """Return whether the current interpreter is PyPy/PyPy3."""
    return platform.python_implementation().lower() == "pypy" or "__pypy__" in getattr(__import__("sys"), "builtin_module_names", ())


@dataclass(frozen=True)
class ParallelExecutionConfig:
    """Runtime switch for process-parallel row generation.

    ``mode='auto'`` is intentionally conservative on CPython and active on
    PyPy.  CPython users can still force process parallelism with
    ``--parallel=processes`` or ``RETA_PARALLEL=processes``.
    """

    mode: str = "auto"
    workers: int | None = None
    chunk_size: int = 64
    threshold: int = 128
    start_method: str | None = None
    source: str = "defaults"

    def __post_init__(self) -> None:
        object.__setattr__(self, "mode", _normalise_mode(self.mode))
        object.__setattr__(self, "workers", _positive_int(self.workers, None))
        object.__setattr__(self, "chunk_size", _positive_int(self.chunk_size, 64) or 64)
        object.__setattr__(self, "threshold", _positive_int(self.threshold, 128) or 128)
        if self.start_method in {"", "default", "none", None}:
            object.__setattr__(self, "start_method", None)

    @classmethod
    def from_environment(cls) -> "ParallelExecutionConfig":
        mode = os.environ.get("RETA_PARALLEL_MODE", os.environ.get("RETA_PARALLEL", "auto"))
        return cls(
            mode=mode,
            workers=_positive_int(os.environ.get("RETA_PARALLEL_WORKERS"), None),
            chunk_size=_positive_int(os.environ.get("RETA_PARALLEL_CHUNK_SIZE"), 64) or 64,
            threshold=_positive_int(os.environ.get("RETA_PARALLEL_THRESHOLD"), 128) or 128,
            start_method=os.environ.get("RETA_PARALLEL_START_METHOD") or None,
            source="environment" if "RETA_PARALLEL" in os.environ or "RETA_PARALLEL_MODE" in os.environ else "defaults",
        )

    def with_overrides(self, **kwargs) -> "ParallelExecutionConfig":
        return replace(self, **{key: value for key, value in kwargs.items() if value is not None})

    @property
    def resolved_workers(self) -> int:
        if self.workers is not None and self.workers > 0:
            return self.workers
        return RETA_PARALLEL_PROCESSOR_CORES

    @property
    def enabled_by_mode(self) -> bool:
        if self.mode == "off":
            return False
        if self.mode == "auto":
            return is_pypy_runtime()
        return self.mode in _PROCESS_VALUES or self.mode == "processes"

    def should_use_processes(self, row_count: int) -> bool:
        return bool(
            self.enabled_by_mode
            and self.resolved_workers > 1
            and row_count >= self.threshold
            and self.chunk_size > 0
        )

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "mode": self.mode,
            "enabled_by_mode": self.enabled_by_mode,
            "workers": self.workers,
            "resolved_workers": self.resolved_workers,
            "chunk_size": self.chunk_size,
            "threshold": self.threshold,
            "start_method": self.start_method,
            "runtime": platform.python_implementation(),
            "source": self.source,
            "processor_cores": RETA_PROCESSOR_CORES.snapshot(),
        }


@dataclass(frozen=True)
class ParallelRowsResult:
    rows: list
    religion_numbers: list[int]
    workers: int
    chunks: int
    row_count: int
    config: ParallelExecutionConfig

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "rows": len(self.rows),
            "religion_numbers": len(self.religion_numbers),
            "workers": self.workers,
            "chunks": self.chunks,
            "row_count": self.row_count,
            "config": self.config.snapshot(),
        }


@dataclass(frozen=True)
class ParallelExecutionBundle:
    config: ParallelExecutionConfig

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "strategy": "process_chunked_table_work",
            "execution_network": "reta_architecture.execution_network.ExecutionNetworkBundle",
            "config": self.config.snapshot(),
            "processor_cores": RETA_PROCESSOR_CORES.snapshot(),
            "morphisms": [
                "extract_parallel_config_from_argv",
                "prepare_rows_in_processes",
                "decode_religion_rows_in_processes",
                "decode_kombi_rows_in_processes",
                "select_columns_in_processes",
                "max_cell_text_len_in_processes",
                "prepare_kombi_join_tables_in_processes",
                "moon_numbers_in_processes",
                "prime_factors_in_processes",
                "filter_numbers_in_processes",
                "factor_pairs_in_processes",
                "normalize_column_buckets_in_processes",
                "glue_parallel_row_chunks",
            ],
            "default_policy": "auto_on_pypy_off_on_cpython",
            "default_workers": RETA_PARALLEL_PROCESSOR_CORES,
        }


def bootstrap_parallel_execution(config: ParallelExecutionConfig | None = None) -> ParallelExecutionBundle:
    return ParallelExecutionBundle(config=config or ParallelExecutionConfig.from_environment())


def _consume_value(argv: Sequence[str], index: int) -> tuple[str | None, int]:
    next_index = index + 1
    if next_index < len(argv) and not str(argv[next_index]).startswith("-"):
        return str(argv[next_index]), 1
    return None, 0


def extract_parallel_config_from_argv(
    argv: Sequence[str],
    inherited: ParallelExecutionConfig | None = None,
) -> tuple[list[str], ParallelExecutionConfig]:
    """Strip Reta-internal parallel flags from argv and return the runtime config."""
    base = inherited or ParallelExecutionConfig.from_environment()
    overrides: dict[str, object] = {}
    clean: list[str] = []
    skip = 0
    recognised = False
    argv_list = [str(arg).strip() for arg in argv]
    for index, arg in enumerate(argv_list):
        if skip:
            skip -= 1
            continue
        if arg == "--no-parallel":
            overrides["mode"] = "off"
            recognised = True
            continue
        if arg == "--parallel":
            overrides["mode"] = "processes"
            recognised = True
            continue
        if arg.startswith("--parallel="):
            overrides["mode"] = arg.split("=", 1)[1]
            recognised = True
            continue
        if arg in {"--parallel-workers", "--parallel-worker", "--parallel-prozesse"}:
            value, consumed = _consume_value(argv_list, index)
            overrides["workers"] = _positive_int(value, base.workers)
            skip = consumed
            recognised = True
            continue
        if arg.startswith("--parallel-workers=") or arg.startswith("--parallel-worker=") or arg.startswith("--parallel-prozesse="):
            overrides["workers"] = _positive_int(arg.split("=", 1)[1], base.workers)
            recognised = True
            continue
        if arg in {"--parallel-chunk-size", "--parallel-chunksize", "--parallel-chunk"}:
            value, consumed = _consume_value(argv_list, index)
            overrides["chunk_size"] = _positive_int(value, base.chunk_size)
            skip = consumed
            recognised = True
            continue
        if arg.startswith("--parallel-chunk-size=") or arg.startswith("--parallel-chunksize=") or arg.startswith("--parallel-chunk="):
            overrides["chunk_size"] = _positive_int(arg.split("=", 1)[1], base.chunk_size)
            recognised = True
            continue
        if arg in {"--parallel-threshold", "--parallel-min-rows"}:
            value, consumed = _consume_value(argv_list, index)
            overrides["threshold"] = _positive_int(value, base.threshold)
            skip = consumed
            recognised = True
            continue
        if arg.startswith("--parallel-threshold=") or arg.startswith("--parallel-min-rows="):
            overrides["threshold"] = _positive_int(arg.split("=", 1)[1], base.threshold)
            recognised = True
            continue
        if arg in {"--parallel-start-method", "--parallel-start"}:
            value, consumed = _consume_value(argv_list, index)
            overrides["start_method"] = value
            skip = consumed
            recognised = True
            continue
        if arg.startswith("--parallel-start-method=") or arg.startswith("--parallel-start="):
            overrides["start_method"] = arg.split("=", 1)[1]
            recognised = True
            continue
        clean.append(arg)
    config = base.with_overrides(**overrides)
    if recognised:
        config = config.with_overrides(source="argv")
    return clean, config


def apply_parallel_environment(config: ParallelExecutionConfig) -> None:
    """Expose an argv-derived config to nested prompt-triggered Reta calls."""
    os.environ["RETA_PARALLEL_MODE"] = config.mode
    os.environ["RETA_PARALLEL_CHUNK_SIZE"] = str(config.chunk_size)
    os.environ["RETA_PARALLEL_THRESHOLD"] = str(config.threshold)
    if config.workers is not None:
        os.environ["RETA_PARALLEL_WORKERS"] = str(config.workers)
    if config.start_method is not None:
        os.environ["RETA_PARALLEL_START_METHOD"] = str(config.start_method)


def _chunks(items: Sequence, chunk_size: int) -> Iterable[list]:
    for index in range(0, len(items), chunk_size):
        yield list(items[index : index + chunk_size])


def _default_start_method(config: ParallelExecutionConfig) -> str | None:
    if config.start_method:
        return config.start_method
    if os.name != "nt" and "fork" in multiprocessing.get_all_start_methods():
        return "fork"
    return None


def _prepare_row_chunk_worker(payload):
    rows, context = payload
    from .table_preparation import prepare_row_cells
    from .table_wrapping import Wraptype, refresh_textwrap_runtime, width_for_row, wrap_cell_text

    wrapping_type_name = context.get("wrapping_type_name") or "pyhyphen"
    try:
        wrapping_type = Wraptype[wrapping_type_name]
    except Exception:
        wrapping_type = Wraptype.pyhyphen
    refresh_textwrap_runtime(wrapping_type=wrapping_type)

    class WorkerPrepare:
        def __init__(self, ctx):
            self.rowsAsNumbers = set(ctx["rows_as_numbers"])
            self.shellRowsAmount = ctx["shell_rows_amount"]
            self.breiten = list(ctx["breiten"])
            self.textwidth = ctx["textwidth"]
            self.religionNumbers = []

        def setWidth(self, rowToDisplay: int, combiRows1: int = 0) -> int:
            return width_for_row(self, rowToDisplay, combiRows1)

        def wrapping(self, text: str, length: int):
            return wrap_cell_text(text, length, wrapping_type=wrapping_type)

    worker_prepare = WorkerPrepare(context)
    old2_rows = ({}, {})
    prepared_rows = []
    rows_as_numbers = set(context["rows_as_numbers"])
    for u, line in rows:
        new_row = prepare_row_cells(
            worker_prepare,
            context["combi_rows"],
            {},
            context["headings_amount"],
            line,
            old2_rows,
            None,
            context["religion_numbers_bool"],
            context["reli_table_len_until_now"],
            rows_as_numbers,
            u,
            kombiCSVNumber=context["kombi_csv_number"],
        )
        if new_row != []:
            prepared_rows.append((u, new_row))
    return prepared_rows, list(worker_prepare.religionNumbers)


def _parallel_context_from_prepare(
    prepare,
    rows_as_numbers: set,
    combi_rows: int,
    headings_amount: int,
    religion_numbers_bool: bool,
    reli_table_len_until_now,
    kombi_csv_number: int,
) -> dict:
    try:
        from .table_wrapping import get_wrapping_type

        wrapping_type_name = get_wrapping_type().name
    except Exception:
        wrapping_type_name = "pyhyphen"
    return {
        "rows_as_numbers": tuple(rows_as_numbers),
        "combi_rows": combi_rows,
        "headings_amount": headings_amount,
        "shell_rows_amount": getattr(prepare, "shellRowsAmount", None),
        "breiten": tuple(getattr(prepare, "breiten", [])),
        "textwidth": getattr(prepare, "textwidth", getattr(prepare, "textWidth", 21)),
        "wrapping_type_name": wrapping_type_name,
        "religion_numbers_bool": bool(religion_numbers_bool),
        "reli_table_len_until_now": reli_table_len_until_now,
        "kombi_csv_number": kombi_csv_number,
    }



@dataclass(frozen=True)
class ParallelOperationResult:
    operation: str
    values: object
    workers: int
    chunks: int
    item_count: int
    config: ParallelExecutionConfig

    def snapshot(self) -> dict:
        try:
            value_len = len(self.values)
        except Exception:
            value_len = None
        return {
            "class": type(self).__name__,
            "operation": self.operation,
            "values": value_len,
            "workers": self.workers,
            "chunks": self.chunks,
            "item_count": self.item_count,
            "config": self.config.snapshot(),
        }


def _pool_map_ordered(worker, payloads: list, config: ParallelExecutionConfig):
    """Map chunk payloads through the execution-network layer.

    The mathematical modules stay pure.  This helper converts process-parallel
    chunk work into importable ExecutionTask objects, runs them through
    ExecutionNetworkCategory, and deterministically reduces by original chunk
    index.
    """
    from .execution_network import ExecutionNetworkConfig, ExecutionTask, execute_tasks_deterministically

    workers = min(config.resolved_workers, max(1, len(payloads)))
    callable_path = f"{worker.__module__}:{worker.__name__}"
    if worker.__module__ == "__main__" or "<locals>" in getattr(worker, "__qualname__", ""):
        start_method = _default_start_method(config)
        ctx = multiprocessing.get_context(start_method) if start_method else multiprocessing.get_context()
        with ctx.Pool(processes=workers) as pool:
            return pool.map(worker, payloads), workers
    tasks = [
        ExecutionTask(index=index, payload=payload, operation=getattr(worker, "__name__", "chunk_worker"), callable_path=callable_path)
        for index, payload in enumerate(payloads)
    ]
    network_config = ExecutionNetworkConfig(
        max_workers=workers,
        queue_discipline="fifo",
        use_processes=True,
        start_method=_default_start_method(config),
        preserve_input_order=True,
    )
    result = execute_tasks_deterministically(tasks, config=network_config)
    return result.values, result.workers


def _decode_religion_cell_static(cell: str, output_kind: str) -> str:
    if not (cell[:2] == "|{" and cell[-2:] == "}|"):
        return html.escape(cell, quote=True) if output_kind == "html" else cell
    payload = json.loads(cell[1:-1])
    if output_kind == "bbcode":
        return payload["bbcode"]
    if output_kind == "html":
        return payload["html"]
    return payload[""]


def _decode_religion_rows_worker(payload):
    rows, output_kind = payload
    return [
        (row_index, [_decode_religion_cell_static(cell, output_kind) for cell in row])
        for row_index, row in rows
    ]


def decode_religion_rows_in_processes(
    rows: Sequence[tuple[int, list]],
    output_kind: str,
    *,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Decode religion CSV rows in process chunks, or return ``None``."""
    config = config or ParallelExecutionConfig.from_environment()
    row_count = len(rows)
    if not config.should_use_processes(row_count):
        return None
    chunks = list(_chunks(list(rows), config.chunk_size))
    if len(chunks) <= 1:
        return None
    payloads = [(chunk, output_kind) for chunk in chunks]
    chunk_results, workers = _pool_map_ordered(_decode_religion_rows_worker, payloads, config)
    decoded_pairs: list[tuple[int, list]] = []
    for chunk in chunk_results:
        decoded_pairs.extend(chunk)
    decoded_pairs.sort(key=lambda item: item[0])
    return ParallelOperationResult(
        operation="decode_religion_rows",
        values=[row for _index, row in decoded_pairs],
        workers=workers,
        chunks=len(chunks),
        item_count=row_count,
        config=config,
    )


def _parse_kombi_number_static(num: str) -> list[int]:
    num = str(num).strip()
    if len(num) > 2 and num[0] == "(" and num[-1] == ")":
        return _parse_kombi_number_static(num[1:-1])
    if num.isdecimal() or (len(num) > 0 and num[0] in ["+", "-"] and num[1:].isdecimal()):
        return [abs(int(num))]
    if len(num) > 2 and "/" in num:
        left, right = num.split("/", 1)
        return _parse_kombi_number_static(left) + _parse_kombi_number_static(right)
    raise ValueError(
        "Die kombi.csv ist in der ersten Spalte nicht so wie sie sein soll mit den Zahlen. "
        + str(num)
        + " "
        + str(type(num))
        + " "
        + str(len(num))
    )


def _decode_kombi_rows_worker(payload):
    rows = payload
    result = []
    for row_index, raw_row in rows:
        col = list(raw_row)
        for i, _row in enumerate(col):
            if i > 0 and col[i].strip() != "" and len(col[0].strip()) != 0:
                col[i] = "(" + col[0] + ") " + col[i] + " (" + col[0] + ")"
        kombi_numbers: list[int] = []
        if len(col) > 0 and row_index > 0:
            for num in col[0].split("|"):
                kombi_numbers.extend(_parse_kombi_number_static(num))
        result.append((row_index, col, kombi_numbers))
    return result


def decode_kombi_rows_in_processes(
    rows: Sequence[tuple[int, list]],
    *,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Parse/decorate Kombi CSV rows in process chunks, or return ``None``."""
    config = config or ParallelExecutionConfig.from_environment()
    row_count = len(rows)
    if not config.should_use_processes(row_count):
        return None
    chunks = list(_chunks(list(rows), config.chunk_size))
    if len(chunks) <= 1:
        return None
    chunk_results, workers = _pool_map_ordered(_decode_kombi_rows_worker, chunks, config)
    decoded: list[tuple[int, list, list[int]]] = []
    for chunk in chunk_results:
        decoded.extend(chunk)
    decoded.sort(key=lambda item: item[0])
    return ParallelOperationResult(
        operation="decode_kombi_rows",
        values=decoded,
        workers=workers,
        chunks=len(chunks),
        item_count=row_count,
        config=config,
    )


def _select_columns_worker(payload):
    rows, columns = payload
    out = []
    for row in rows:
        new_col = []
        for i in columns:
            try:
                new_col.append(deepcopy(row[i - 1]))
            except IndexError:
                pass
        out.append(new_col)
    return out


def select_columns_in_processes(
    table: Sequence[list],
    only_that_columns: Sequence[int],
    *,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Project table columns in process chunks, preserving row order."""
    if len(only_that_columns) == 0:
        return None
    config = config or ParallelExecutionConfig.from_environment()
    row_count = len(table)
    if not config.should_use_processes(row_count):
        return None
    chunks = list(_chunks(list(table), config.chunk_size))
    if len(chunks) <= 1:
        return None
    payloads = [(chunk, tuple(only_that_columns)) for chunk in chunks]
    chunk_results, workers = _pool_map_ordered(_select_columns_worker, payloads, config)
    projected: list = []
    for chunk in chunk_results:
        projected.extend(chunk)
    return ParallelOperationResult(
        operation="select_columns",
        values=projected,
        workers=workers,
        chunks=len(chunks),
        item_count=row_count,
        config=config,
    )


def _max_cell_text_len_worker(payload):
    rows, rows_range = payload
    local: dict[int, int] = {}
    for row in rows:
        for m in rows_range:
            for i, _cell in enumerate(row):
                try:
                    text_len = len(row[i][m])
                except Exception:
                    continue
                previous = local.get(i)
                if previous is None or text_len > previous:
                    local[i] = text_len
    return local


def max_cell_text_len_in_processes(
    new_table: Sequence[list],
    rows_range: Sequence[int] | range,
    *,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Compute maximum cell text lengths by output column in process chunks."""
    config = config or ParallelExecutionConfig.from_environment()
    row_count = len(new_table)
    if not config.should_use_processes(row_count):
        return None
    chunks = list(_chunks(list(new_table), config.chunk_size))
    if len(chunks) <= 1:
        return None
    payloads = [(chunk, tuple(rows_range)) for chunk in chunks]
    chunk_results, workers = _pool_map_ordered(_max_cell_text_len_worker, payloads, config)
    merged: dict[int, int] = {}
    for local in chunk_results:
        for key, value in local.items():
            previous = merged.get(key)
            if previous is None or value > previous:
                merged[key] = value
    ordered = OrderedDict((key, merged[key]) for key in sorted(merged))
    return ParallelOperationResult(
        operation="max_cell_text_len",
        values=ordered,
        workers=workers,
        chunks=len(chunks),
        item_count=row_count,
        config=config,
    )


def _prepare_kombi_join_tables_worker(payload):
    items, new_table_kombi_1 = payload
    result = []
    for key, value in items:
        rows = []
        for kombi_line_number in value:
            try:
                rows.append(deepcopy(new_table_kombi_1[int(kombi_line_number)]))
            except (IndexError, TypeError, ValueError):
                pass
        if rows:
            result.append((key, rows))
    return result


def _moon_numbers_worker(payload):
    numbers = payload
    from .number_theory import moonNumber

    return [(int(number), moonNumber(int(number))) for number in numbers]


def moon_numbers_in_processes(
    numbers: Sequence[int],
    *,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Compute ``moonNumber`` for many row numbers in process chunks.

    The sequential state transition in ``set_zaehlungen`` is preserved by
    returning ordered ``(number, moon_type)`` pairs; the caller still performs the
    mutable update serially.  Only the expensive per-number moon predicate is
    parallelised.
    """
    config = config or ParallelExecutionConfig.from_environment()
    ordered_numbers = [int(number) for number in numbers]
    item_count = len(ordered_numbers)
    if not config.should_use_processes(item_count):
        return None
    chunks = list(_chunks(ordered_numbers, config.chunk_size))
    if len(chunks) <= 1:
        return None
    chunk_results, workers = _pool_map_ordered(_moon_numbers_worker, chunks, config)
    values: list[tuple[int, object]] = []
    for chunk in chunk_results:
        values.extend(chunk)
    values.sort(key=lambda item: item[0])
    return ParallelOperationResult(
        operation="moon_numbers",
        values=values,
        workers=workers,
        chunks=len(chunks),
        item_count=item_count,
        config=config,
    )


def _prime_factors_worker(payload):
    numbers = payload
    from .number_theory import primFak

    return [(int(number), primFak(int(number))) for number in numbers]


def prime_factors_in_processes(
    numbers: Sequence[int],
    *,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Compute prime factors for many numbers in process chunks."""
    config = config or ParallelExecutionConfig.from_environment()
    ordered_numbers = [int(number) for number in numbers]
    item_count = len(ordered_numbers)
    if not config.should_use_processes(item_count):
        return None
    chunks = list(_chunks(ordered_numbers, config.chunk_size))
    if len(chunks) <= 1:
        return None
    chunk_results, workers = _pool_map_ordered(_prime_factors_worker, chunks, config)
    values: list[tuple[int, list]] = []
    for chunk in chunk_results:
        values.extend(chunk)
    values.sort(key=lambda item: item[0])
    return ParallelOperationResult(
        operation="prime_factors",
        values=values,
        workers=workers,
        chunks=len(chunks),
        item_count=item_count,
        config=config,
    )


def _number_filter_worker(payload):
    numbers, mode, criteria = payload
    out: set[int] = set()
    mode = str(mode)
    if mode == "sonne_mit_mondanteil":
        from .number_theory import primFak, primRepeat

        for number in numbers:
            number = int(number)
            booleans = {amount == 1 for _prime, amount in primRepeat(tuple(primFak(number)))}
            if len({True, False} & booleans) > 1:
                out.add(number)
    elif mode == "prime_multiples":
        from .number_theory import isPrimMultiple

        multiples = list(criteria or [])
        for number in numbers:
            number = int(number)
            if isPrimMultiple(number, multiples):
                out.add(number)
    elif mode == "ordinary_multiples":
        divisors = [int(value) for value in (criteria or []) if int(value) != 0]
        for number in numbers:
            number = int(number)
            for divisor in divisors:
                if number % divisor == 0:
                    out.add(number)
                    break
    elif mode == "modulo":
        divisor, remainder = criteria
        divisor = int(divisor)
        remainder = int(remainder)
        for number in numbers:
            number = int(number)
            if divisor and number % divisor == remainder:
                out.add(number)
    elif mode == "moon":
        from .number_theory import moonNumber

        want_moon = bool(criteria)
        for number in numbers:
            number = int(number)
            if (moonNumber(number)[0] != []) == want_moon:
                out.add(number)
    return out


def filter_numbers_in_processes(
    numbers: Sequence[int] | set[int],
    mode: str,
    criteria=None,
    *,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Filter a finite number set in process chunks for pure row predicates."""
    config = config or ParallelExecutionConfig.from_environment()
    ordered_numbers = sorted(int(number) for number in numbers)
    item_count = len(ordered_numbers)
    if not config.should_use_processes(item_count):
        return None
    chunks = list(_chunks(ordered_numbers, config.chunk_size))
    if len(chunks) <= 1:
        return None
    payloads = [(chunk, mode, criteria) for chunk in chunks]
    chunk_results, workers = _pool_map_ordered(_number_filter_worker, payloads, config)
    values: set[int] = set()
    for chunk in chunk_results:
        values |= set(chunk)
    return ParallelOperationResult(
        operation=f"filter_numbers:{mode}",
        values=values,
        workers=workers,
        chunks=len(chunks),
        item_count=item_count,
        config=config,
    )


def _factor_pairs_worker(payload):
    numbers, include_one = payload
    from .arithmetic import factor_pairs

    return [(int(number), factor_pairs(int(number), bool(include_one))) for number in numbers]


def factor_pairs_in_processes(
    numbers: Sequence[int] | set[int],
    *,
    include_one: bool = True,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Compute factor pairs for many numbers in process chunks."""
    config = config or ParallelExecutionConfig.from_environment()
    ordered_numbers = sorted(int(number) for number in numbers)
    item_count = len(ordered_numbers)
    if not config.should_use_processes(item_count):
        return None
    chunks = list(_chunks(ordered_numbers, config.chunk_size))
    if len(chunks) <= 1:
        return None
    payloads = [(chunk, bool(include_one)) for chunk in chunks]
    chunk_results, workers = _pool_map_ordered(_factor_pairs_worker, payloads, config)
    values: list[tuple[int, list]] = []
    for chunk in chunk_results:
        values.extend(chunk)
    values.sort(key=lambda item: item[0])
    return ParallelOperationResult(
        operation="factor_pairs",
        values=values,
        workers=workers,
        chunks=len(chunks),
        item_count=item_count,
        config=config,
    )


def _normalize_column_bucket_worker(payload):
    bucket_pairs = payload
    result = []
    for bucket_type, positive, negative in bucket_pairs:
        result.append((bucket_type, set(positive) - (set(positive) & set(negative)) - set(negative)))
    return result


def normalize_column_buckets_in_processes(
    spalten_arten: Mapping[tuple[int, int], set],
    *,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Normalise positive/negative column buckets in process chunks.

    This mirrors ``universal.normalize_column_buckets`` but keeps the final dict
    assembly serial and ordered.  It is used only when bucket payloads are large
    enough that process overhead can pay off.
    """
    config = config or ParallelExecutionConfig.from_environment()
    buckets = {key: set(value) for key, value in spalten_arten.items()}
    max_type = int(len(buckets) / 2)
    pairs = [
        (bucket_type, buckets.get((0, bucket_type), set()), buckets.get((1, bucket_type), set()))
        for bucket_type in range(max_type)
        if (0, bucket_type) in buckets and (1, bucket_type) in buckets
    ]
    item_count = sum(len(pos) + len(neg) for _bucket, pos, neg in pairs)
    if not config.should_use_processes(item_count):
        return None
    # There are usually only a handful of positive/negative bucket pairs, but
    # each pair can contain a large set.  Use one bucket-pair per chunk once the
    # total payload crosses the threshold; otherwise a default chunk size of 64
    # would collapse all pairs back into one serial worker payload.
    pair_chunk_size = 1 if item_count >= config.threshold and len(pairs) > 1 else max(1, min(config.chunk_size, len(pairs) or 1))
    chunks = list(_chunks(pairs, pair_chunk_size))
    if len(chunks) <= 1:
        return None
    chunk_results, workers = _pool_map_ordered(_normalize_column_bucket_worker, chunks, config)
    normalised_positive = {}
    for chunk in chunk_results:
        for bucket_type, value in chunk:
            normalised_positive[bucket_type] = value
    result = {key: set(value) for key, value in buckets.items()}
    for bucket_type, value in normalised_positive.items():
        result[(0, bucket_type)] = value
        result.pop((1, bucket_type), None)
    return ParallelOperationResult(
        operation="normalize_column_buckets",
        values=result,
        workers=workers,
        chunks=len(chunks),
        item_count=item_count,
        config=config,
    )


def prepare_kombi_join_tables_in_processes(
    chosen_kombi_lines,
    new_table_kombi_1: Sequence[list],
    *,
    config: ParallelExecutionConfig | None = None,
) -> ParallelOperationResult | None:
    """Prepare Kombi sub-table selections in process chunks, preserving key order."""
    items = [(key, list(value)) for key, value in chosen_kombi_lines.items()]
    config = config or ParallelExecutionConfig.from_environment()
    item_count = len(items)
    if not config.should_use_processes(item_count):
        return None
    chunks = list(_chunks(items, config.chunk_size))
    if len(chunks) <= 1:
        return None
    payloads = [(chunk, list(new_table_kombi_1)) for chunk in chunks]
    chunk_results, workers = _pool_map_ordered(_prepare_kombi_join_tables_worker, payloads, config)
    values = []
    for chunk in chunk_results:
        for key, rows in chunk:
            values.append(OrderedDict(((key, rows),)))
    return ParallelOperationResult(
        operation="prepare_kombi_join_tables",
        values=values,
        workers=workers,
        chunks=len(chunks),
        item_count=item_count,
        config=config,
    )


def prepare_rows_in_processes(
    prepare,
    rows: Sequence[tuple[int, list]],
    *,
    rows_as_numbers: set,
    combi_rows: int,
    headings_amount: int,
    religion_numbers_bool: bool,
    reli_table_len_until_now=None,
    kombi_csv_number: int = 0,
    config: ParallelExecutionConfig | None = None,
) -> ParallelRowsResult | None:
    """Prepare selected non-header rows in process chunks, or return ``None``."""
    config = config or getattr(prepare, "parallel_config", None) or ParallelExecutionConfig.from_environment()
    row_count = len(rows)
    if not config.should_use_processes(row_count):
        return None
    chunks = list(_chunks(list(rows), config.chunk_size))
    if len(chunks) <= 1:
        return None
    workers = min(config.resolved_workers, max(1, len(chunks)))
    context = _parallel_context_from_prepare(
        prepare,
        rows_as_numbers=rows_as_numbers,
        combi_rows=combi_rows,
        headings_amount=headings_amount,
        religion_numbers_bool=religion_numbers_bool,
        reli_table_len_until_now=reli_table_len_until_now,
        kombi_csv_number=kombi_csv_number,
    )
    payloads = [(chunk, context) for chunk in chunks]
    start_method = _default_start_method(config)
    ctx = multiprocessing.get_context(start_method) if start_method else multiprocessing.get_context()
    with ctx.Pool(processes=workers) as pool:
        chunk_results = pool.map(_prepare_row_chunk_worker, payloads)
    prepared_pairs: list[tuple[int, list]] = []
    religion_numbers: list[int] = []
    for rows_result, religion_numbers_result in chunk_results:
        prepared_pairs.extend(rows_result)
        religion_numbers.extend(religion_numbers_result)
    prepared_pairs.sort(key=lambda item: item[0])
    return ParallelRowsResult(
        rows=[row for _u, row in prepared_pairs],
        religion_numbers=religion_numbers,
        workers=workers,
        chunks=len(chunks),
        row_count=row_count,
        config=config,
    )
