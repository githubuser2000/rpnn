from __future__ import annotations

"""Deterministic execution-network primitives for Reta runtime work.

This module deliberately sits *above* the mathematical layers.  Topologies,
presheaves, sheaves, morphisms, functors and natural transformations stay pure;
this layer only schedules and glues executable tasks that already have a clear
semantic owner.
"""

import heapq
import importlib
import multiprocessing
import os
from collections import deque
from dataclasses import dataclass, field
from queue import Queue
from threading import BoundedSemaphore
from typing import Any, Callable, Iterable, Mapping, MutableSequence, Sequence


def _available_worker_count() -> int:
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


EXECUTION_NETWORK_PROCESS_CORES = _available_worker_count()


@dataclass(frozen=True)
class ExecutionNetworkConfig:
    """Configuration for deterministic task execution.

    ``use_processes`` selects multiprocessing rather than threads.  Worker counts
    are capped by both the configured maximum and the number of available tasks.
    """

    max_workers: int = EXECUTION_NETWORK_PROCESS_CORES
    queue_discipline: str = "fifo"
    use_processes: bool = False
    start_method: str | None = None
    preserve_input_order: bool = True
    bounded_queue_size: int | None = None

    def __post_init__(self) -> None:
        object.__setattr__(self, "max_workers", max(1, int(self.max_workers or 1)))
        discipline = str(self.queue_discipline or "fifo").lower()
        if discipline not in {"fifo", "lifo", "priority"}:
            discipline = "fifo"
        object.__setattr__(self, "queue_discipline", discipline)
        if self.start_method in {"", "default", "none"}:
            object.__setattr__(self, "start_method", None)
        if self.bounded_queue_size is not None:
            object.__setattr__(self, "bounded_queue_size", max(1, int(self.bounded_queue_size)))

    def workers_for(self, task_count: int) -> int:
        return min(self.max_workers, max(1, int(task_count or 1)))

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "max_workers": self.max_workers,
            "resolved_available_workers": EXECUTION_NETWORK_PROCESS_CORES,
            "queue_discipline": self.queue_discipline,
            "use_processes": self.use_processes,
            "start_method": self.start_method,
            "preserve_input_order": self.preserve_input_order,
            "bounded_queue_size": self.bounded_queue_size,
        }


@dataclass(frozen=True)
class ExecutionTask:
    """One executable local task.

    ``callable_path`` is preferred for process execution because it is importable
    under both ``fork`` and ``spawn``.  The callable receives ``payload`` and
    returns a serialisable value.
    """

    index: int
    payload: Any
    operation: str = "identity"
    priority: int = 0
    callable_path: str | None = None
    metadata: Mapping[str, Any] = field(default_factory=dict)

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "index": self.index,
            "operation": self.operation,
            "priority": self.priority,
            "callable_path": self.callable_path,
            "metadata": dict(self.metadata),
        }


@dataclass(frozen=True)
class ExecutionResult:
    task_index: int
    value: Any
    operation: str
    metadata: Mapping[str, Any] = field(default_factory=dict)

    def snapshot(self) -> dict:
        try:
            value_len = len(self.value)
        except Exception:
            value_len = None
        return {
            "class": type(self).__name__,
            "task_index": self.task_index,
            "operation": self.operation,
            "value_type": type(self.value).__name__,
            "value_len": value_len,
            "metadata": dict(self.metadata),
        }


@dataclass(frozen=True)
class ExecutionRunResult:
    values: list[Any]
    results: tuple[ExecutionResult, ...]
    config: ExecutionNetworkConfig
    workers: int
    task_count: int
    queue_discipline: str
    mode: str

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "values": len(self.values),
            "results": [item.snapshot() for item in self.results],
            "config": self.config.snapshot(),
            "workers": self.workers,
            "task_count": self.task_count,
            "queue_discipline": self.queue_discipline,
            "mode": self.mode,
            "universal_property": "parallel_or_serial_task_cover_glues_to_the_same_ordered_result",
        }


class FifoTaskQueue:
    def __init__(self, tasks: Iterable[ExecutionTask] = ()) -> None:
        self._items = deque(tasks)

    def push(self, task: ExecutionTask) -> None:
        self._items.append(task)

    def pop(self) -> ExecutionTask:
        return self._items.popleft()

    def __len__(self) -> int:
        return len(self._items)


class LifoTaskStack:
    def __init__(self, tasks: Iterable[ExecutionTask] = ()) -> None:
        self._items = list(tasks)

    def push(self, task: ExecutionTask) -> None:
        self._items.append(task)

    def pop(self) -> ExecutionTask:
        return self._items.pop()

    def __len__(self) -> int:
        return len(self._items)


class PriorityTaskQueue:
    def __init__(self, tasks: Iterable[ExecutionTask] = ()) -> None:
        self._heap: list[tuple[int, int, ExecutionTask]] = []
        for task in tasks:
            self.push(task)

    def push(self, task: ExecutionTask) -> None:
        heapq.heappush(self._heap, (task.priority, task.index, task))

    def pop(self) -> ExecutionTask:
        return heapq.heappop(self._heap)[2]

    def __len__(self) -> int:
        return len(self._heap)


class ResourceSemaphore:
    """Small process-safe semaphore wrapper used only at resource boundaries."""

    def __init__(self, value: int) -> None:
        self.value = max(1, int(value or 1))
        self._semaphore = BoundedSemaphore(self.value)

    def acquire(self, timeout: float | None = None) -> bool:
        if timeout is None:
            return bool(self._semaphore.acquire())
        return bool(self._semaphore.acquire(timeout=timeout))

    def release(self) -> None:
        self._semaphore.release()

    def snapshot(self) -> dict:
        return {"class": type(self).__name__, "value": self.value}


class HalfDuplexChannel:
    """Request/response channel: one direction is active at a time."""

    def __init__(self, maxsize: int = 0) -> None:
        self.requests: Queue = Queue(maxsize=max(0, int(maxsize or 0)))
        self.responses: Queue = Queue(maxsize=max(0, int(maxsize or 0)))

    def send_request(self, message: Any) -> None:
        self.requests.put(message)

    def receive_request(self, timeout: float | None = None) -> Any:
        return self.requests.get(timeout=timeout)

    def send_response(self, message: Any) -> None:
        self.responses.put(message)

    def receive_response(self, timeout: float | None = None) -> Any:
        return self.responses.get(timeout=timeout)

    def snapshot(self) -> dict:
        return {"class": type(self).__name__, "requests": self.requests.qsize(), "responses": self.responses.qsize()}


class FullDuplexChannel:
    """Two independent queues for bidirectional supervisor/worker traffic."""

    def __init__(self, maxsize: int = 0) -> None:
        self.a_to_b: Queue = Queue(maxsize=max(0, int(maxsize or 0)))
        self.b_to_a: Queue = Queue(maxsize=max(0, int(maxsize or 0)))

    def send_a_to_b(self, message: Any) -> None:
        self.a_to_b.put(message)

    def receive_a_to_b(self, timeout: float | None = None) -> Any:
        return self.a_to_b.get(timeout=timeout)

    def send_b_to_a(self, message: Any) -> None:
        self.b_to_a.put(message)

    def receive_b_to_a(self, timeout: float | None = None) -> Any:
        return self.b_to_a.get(timeout=timeout)

    def snapshot(self) -> dict:
        return {"class": type(self).__name__, "a_to_b": self.a_to_b.qsize(), "b_to_a": self.b_to_a.qsize()}


@dataclass(frozen=True)
class ExecutionNetworkBundle:
    config: ExecutionNetworkConfig
    cpu_semaphore: ResourceSemaphore
    file_io_semaphore: ResourceSemaphore
    output_semaphore: ResourceSemaphore

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "category": "ExecutionNetworkCategory",
            "scheduler_category": "SchedulerCategory",
            "channel_category": "ChannelCategory",
            "config": self.config.snapshot(),
            "processor_cores": EXECUTION_NETWORK_PROCESS_CORES,
            "queues": ["FifoTaskQueue", "LifoTaskStack", "PriorityTaskQueue"],
            "channels": ["HalfDuplexChannel", "FullDuplexChannel"],
            "semaphores": {
                "cpu": self.cpu_semaphore.snapshot(),
                "file_io": self.file_io_semaphore.snapshot(),
                "output": self.output_semaphore.snapshot(),
            },
            "morphisms": [
                "enqueue_task",
                "dequeue_task",
                "dispatch_task",
                "collect_result",
                "deterministic_reduce",
                "acquire_resource",
                "release_resource",
                "send_message",
                "receive_message",
            ],
            "universal_property": "parallel_chunks_glue_deterministically_to_serial_result",
        }


def bootstrap_execution_network(config: ExecutionNetworkConfig | None = None) -> ExecutionNetworkBundle:
    config = config or ExecutionNetworkConfig()
    return ExecutionNetworkBundle(
        config=config,
        cpu_semaphore=ResourceSemaphore(config.max_workers),
        file_io_semaphore=ResourceSemaphore(max(1, min(4, config.max_workers))),
        output_semaphore=ResourceSemaphore(1),
    )


def _queue_for(config: ExecutionNetworkConfig, tasks: Sequence[ExecutionTask]):
    if config.queue_discipline == "lifo":
        return LifoTaskStack(tasks)
    if config.queue_discipline == "priority":
        return PriorityTaskQueue(tasks)
    return FifoTaskQueue(tasks)


def order_tasks(tasks: Sequence[ExecutionTask], config: ExecutionNetworkConfig) -> list[ExecutionTask]:
    queue = _queue_for(config, tasks)
    ordered: list[ExecutionTask] = []
    while len(queue) > 0:
        ordered.append(queue.pop())
    return ordered


def _resolve_callable(callable_path: str | None, handler: Callable[[Any], Any] | None = None) -> Callable[[Any], Any]:
    if handler is not None:
        return handler
    if callable_path:
        module_name, func_name = callable_path.split(":", 1)
        module = importlib.import_module(module_name)
        return getattr(module, func_name)
    return _builtin_operation


def _builtin_operation(payload: Any) -> Any:
    return payload


def _task_worker(task: ExecutionTask) -> ExecutionResult:
    handler = _resolve_callable(task.callable_path)
    return ExecutionResult(
        task_index=task.index,
        value=handler(task.payload),
        operation=task.operation,
        metadata=task.metadata,
    )


def _run_serial(tasks: Sequence[ExecutionTask], handler: Callable[[Any], Any] | None = None) -> list[ExecutionResult]:
    results: list[ExecutionResult] = []
    for task in tasks:
        resolved_handler = _resolve_callable(task.callable_path, handler)
        results.append(
            ExecutionResult(
                task_index=task.index,
                value=resolved_handler(task.payload),
                operation=task.operation,
                metadata=task.metadata,
            )
        )
    return results


def deterministic_reduce(results: Sequence[ExecutionResult], *, preserve_input_order: bool = True) -> list[Any]:
    ordered = sorted(results, key=lambda item: item.task_index) if preserve_input_order else list(results)
    return [item.value for item in ordered]


def execute_tasks_deterministically(
    tasks: Sequence[ExecutionTask],
    *,
    handler: Callable[[Any], Any] | None = None,
    config: ExecutionNetworkConfig | None = None,
) -> ExecutionRunResult:
    """Execute tasks via FIFO/LIFO/priority scheduling and deterministic reduce.

    Process mode requires importable ``task.callable_path`` entries.  Passing a
    local ``handler`` intentionally forces serial mode to avoid hidden thread-like
    behaviour or non-portable process pickling.
    """

    config = config or ExecutionNetworkConfig()
    tasks = tuple(tasks)
    if not tasks:
        return ExecutionRunResult([], (), config, workers=0, task_count=0, queue_discipline=config.queue_discipline, mode="empty")
    scheduled = order_tasks(tasks, config)
    use_processes = bool(config.use_processes and handler is None and any(task.callable_path for task in scheduled))
    workers = config.workers_for(len(scheduled)) if use_processes else 1
    if use_processes:
        start_method = config.start_method
        ctx = multiprocessing.get_context(start_method) if start_method else multiprocessing.get_context()
        with ctx.Pool(processes=workers) as pool:
            results = pool.map(_task_worker, scheduled)
        mode = "processes"
    else:
        results = _run_serial(scheduled, handler=handler)
        mode = "serial"
    values = deterministic_reduce(results, preserve_input_order=config.preserve_input_order)
    return ExecutionRunResult(
        values=values,
        results=tuple(results),
        config=config,
        workers=workers,
        task_count=len(scheduled),
        queue_discipline=config.queue_discipline,
        mode=mode,
    )
