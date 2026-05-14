use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Condvar, Mutex};

use serde::{Deserialize, Serialize};

pub const EXECUTION_NETWORK_SNAPSHOT: &str = include_str!("../data/execution_network_snapshot.json");

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum DataflowDiscipline {
    #[default]
    Fifo,
    Lifo,
    Priority,
}

impl DataflowDiscipline {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fifo => "fifo",
            Self::Lifo => "lifo",
            Self::Priority => "priority",
        }
    }

    pub fn from_name(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "lifo" | "stack" => Self::Lifo,
            "priority" | "prio" => Self::Priority,
            _ => Self::Fifo,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionNetworkConfig {
    pub max_workers: usize,
    pub queue_discipline: DataflowDiscipline,
    pub use_processes: bool,
    pub start_method: Option<String>,
    pub preserve_input_order: bool,
    pub bounded_queue_size: Option<usize>,
}

impl Default for ExecutionNetworkConfig {
    fn default() -> Self {
        Self {
            max_workers: available_parallelism(),
            queue_discipline: DataflowDiscipline::Fifo,
            use_processes: false,
            start_method: None,
            preserve_input_order: true,
            bounded_queue_size: None,
        }
    }
}

impl ExecutionNetworkConfig {
    pub fn new(max_workers: usize, queue_discipline: DataflowDiscipline) -> Self {
        Self {
            max_workers: max_workers.max(1),
            queue_discipline,
            ..Self::default()
        }
    }

    pub fn workers_for(&self, task_count: usize) -> usize {
        self.max_workers.min(task_count.max(1))
    }

    pub fn snapshot(&self) -> Vec<(&'static str, String)> {
        vec![
            ("class", "ExecutionNetworkConfig".to_string()),
            ("max_workers", self.max_workers.to_string()),
            ("queue_discipline", self.queue_discipline.as_str().to_string()),
            ("use_processes", self.use_processes.to_string()),
            (
                "start_method",
                self.start_method.clone().unwrap_or_else(|| "None".to_string()),
            ),
            ("preserve_input_order", self.preserve_input_order.to_string()),
            (
                "bounded_queue_size",
                self.bounded_queue_size
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "None".to_string()),
            ),
        ]
    }
}

fn available_parallelism() -> usize {
    std::thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(1)
        .max(1)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionTask<T = String> {
    pub index: usize,
    pub payload: T,
    pub operation: String,
    pub priority: i64,
    pub callable_path: Option<String>,
    pub metadata: Vec<(String, String)>,
}

impl<T> ExecutionTask<T> {
    pub fn new(index: usize, payload: T) -> Self {
        Self {
            index,
            payload,
            operation: "identity".to_string(),
            priority: 0,
            callable_path: None,
            metadata: Vec::new(),
        }
    }

    pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = operation.into();
        self
    }

    pub fn with_priority(mut self, priority: i64) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionResult<T = String> {
    pub task_index: usize,
    pub value: T,
    pub operation: String,
    pub metadata: Vec<(String, String)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionRunResult<T = String> {
    pub values: Vec<T>,
    pub results: Vec<ExecutionResult<T>>,
    pub config: ExecutionNetworkConfig,
    pub workers: usize,
    pub task_count: usize,
    pub queue_discipline: DataflowDiscipline,
    pub mode: String,
}

impl<T> ExecutionRunResult<T> {
    pub fn universal_property(&self) -> &'static str {
        "parallel_or_serial_task_cover_glues_to_the_same_ordered_result"
    }
}

pub struct FifoTaskQueue<T> {
    items: VecDeque<ExecutionTask<T>>,
}

impl<T> FifoTaskQueue<T> {
    pub fn new(tasks: impl IntoIterator<Item = ExecutionTask<T>>) -> Self {
        Self {
            items: tasks.into_iter().collect(),
        }
    }

    pub fn push(&mut self, task: ExecutionTask<T>) {
        self.items.push_back(task);
    }

    pub fn pop(&mut self) -> Option<ExecutionTask<T>> {
        self.items.pop_front()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

pub struct LifoTaskStack<T> {
    items: Vec<ExecutionTask<T>>,
}

impl<T> LifoTaskStack<T> {
    pub fn new(tasks: impl IntoIterator<Item = ExecutionTask<T>>) -> Self {
        Self {
            items: tasks.into_iter().collect(),
        }
    }

    pub fn push(&mut self, task: ExecutionTask<T>) {
        self.items.push(task);
    }

    pub fn pop(&mut self) -> Option<ExecutionTask<T>> {
        self.items.pop()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

pub struct PriorityTaskQueue<T> {
    items: Vec<ExecutionTask<T>>,
}

impl<T> PriorityTaskQueue<T> {
    pub fn new(tasks: impl IntoIterator<Item = ExecutionTask<T>>) -> Self {
        Self {
            items: tasks.into_iter().collect(),
        }
    }

    pub fn push(&mut self, task: ExecutionTask<T>) {
        self.items.push(task);
    }

    pub fn pop(&mut self) -> Option<ExecutionTask<T>> {
        if self.items.is_empty() {
            return None;
        }
        let mut best_index = 0usize;
        for index in 1..self.items.len() {
            let current = &self.items[index];
            let best = &self.items[best_index];
            if (current.priority, current.index) < (best.priority, best.index) {
                best_index = index;
            }
        }
        Some(self.items.remove(best_index))
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

pub fn order_tasks<T: Clone>(
    tasks: &[ExecutionTask<T>],
    config: &ExecutionNetworkConfig,
) -> Vec<ExecutionTask<T>> {
    match config.queue_discipline {
        DataflowDiscipline::Fifo => {
            let mut queue = FifoTaskQueue::new(tasks.iter().cloned());
            let mut out = Vec::with_capacity(tasks.len());
            while let Some(task) = queue.pop() {
                out.push(task);
            }
            out
        }
        DataflowDiscipline::Lifo => {
            let mut stack = LifoTaskStack::new(tasks.iter().cloned());
            let mut out = Vec::with_capacity(tasks.len());
            while let Some(task) = stack.pop() {
                out.push(task);
            }
            out
        }
        DataflowDiscipline::Priority => {
            let mut queue = PriorityTaskQueue::new(tasks.iter().cloned());
            let mut out = Vec::with_capacity(tasks.len());
            while let Some(task) = queue.pop() {
                out.push(task);
            }
            out
        }
    }
}

pub fn deterministic_reduce<T: Clone>(
    results: &[ExecutionResult<T>],
    preserve_input_order: bool,
) -> Vec<T> {
    let mut ordered = results.to_vec();
    if preserve_input_order {
        ordered.sort_by_key(|item| item.task_index);
    }
    ordered.into_iter().map(|item| item.value).collect()
}

pub fn execute_tasks_deterministically<T, U, F>(
    tasks: &[ExecutionTask<T>],
    handler: F,
    config: Option<ExecutionNetworkConfig>,
) -> ExecutionRunResult<U>
where
    T: Clone,
    U: Clone,
    F: Fn(&T) -> U,
{
    let config = config.unwrap_or_default();
    if tasks.is_empty() {
        return ExecutionRunResult {
            values: Vec::new(),
            results: Vec::new(),
            workers: 0,
            task_count: 0,
            queue_discipline: config.queue_discipline,
            mode: "empty".to_string(),
            config,
        };
    }

    let scheduled = order_tasks(tasks, &config);
    let results = scheduled
        .iter()
        .map(|task| ExecutionResult {
            task_index: task.index,
            value: handler(&task.payload),
            operation: task.operation.clone(),
            metadata: task.metadata.clone(),
        })
        .collect::<Vec<_>>();
    let values = deterministic_reduce(&results, config.preserve_input_order);

    ExecutionRunResult {
        values,
        results,
        workers: 1,
        task_count: scheduled.len(),
        queue_discipline: config.queue_discipline,
        mode: "serial".to_string(),
        config,
    }
}


/// Execute tasks in a small Rust worker pool while preserving the same gluing
/// law as the serial architecture path.  Scheduling may be FIFO/LIFO/priority,
/// but `deterministic_reduce` can still restore input order, which is the
/// important Reta output invariant.
pub fn execute_tasks_threaded_ordered<T, U, F>(
    tasks: &[ExecutionTask<T>],
    handler: F,
    config: Option<ExecutionNetworkConfig>,
) -> ExecutionRunResult<U>
where
    T: Clone + Send + 'static,
    U: Clone + Send + 'static,
    F: Fn(&T) -> U + Send + Sync + 'static,
{
    let config = config.unwrap_or_default();
    if tasks.is_empty() {
        return ExecutionRunResult {
            values: Vec::new(),
            results: Vec::new(),
            workers: 0,
            task_count: 0,
            queue_discipline: config.queue_discipline,
            mode: "empty".to_string(),
            config,
        };
    }

    let scheduled = order_tasks(tasks, &config);
    let worker_count = config.workers_for(scheduled.len());
    if worker_count <= 1 || scheduled.len() <= 1 {
        return execute_tasks_deterministically(tasks, handler, Some(config));
    }

    let queue = Arc::new(Mutex::new(VecDeque::from(scheduled.clone())));
    let handler = Arc::new(handler);
    let (tx, rx) = mpsc::channel::<ExecutionResult<U>>();
    let mut handles = Vec::with_capacity(worker_count);

    for _ in 0..worker_count {
        let queue = Arc::clone(&queue);
        let handler = Arc::clone(&handler);
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || loop {
            let next_task = {
                let mut guard = queue
                    .lock()
                    .expect("execution network task queue mutex poisoned");
                guard.pop_front()
            };
            let Some(task) = next_task else {
                break;
            };
            let value = handler(&task.payload);
            if tx
                .send(ExecutionResult {
                    task_index: task.index,
                    value,
                    operation: task.operation,
                    metadata: task.metadata,
                })
                .is_err()
            {
                break;
            }
        }));
    }
    drop(tx);

    let mut results = rx.into_iter().collect::<Vec<_>>();
    for handle in handles {
        let _ = handle.join();
    }
    if config.preserve_input_order {
        results.sort_by_key(|item| item.task_index);
    }
    let values = deterministic_reduce(&results, config.preserve_input_order);

    ExecutionRunResult {
        values,
        results,
        workers: worker_count,
        task_count: scheduled.len(),
        queue_discipline: config.queue_discipline,
        mode: "threaded".to_string(),
        config,
    }
}

#[derive(Clone, Debug)]
pub struct ResourceSemaphore {
    capacity: usize,
    state: Arc<(Mutex<usize>, Condvar)>,
}

impl ResourceSemaphore {
    pub fn new(value: usize) -> Self {
        let capacity = value.max(1);
        Self {
            capacity,
            state: Arc::new((Mutex::new(capacity), Condvar::new())),
        }
    }

    pub fn acquire(&self) {
        let (lock, cvar) = &*self.state;
        let mut available = lock.lock().expect("resource semaphore mutex poisoned");
        while *available == 0 {
            available = cvar
                .wait(available)
                .expect("resource semaphore mutex poisoned while waiting");
        }
        *available -= 1;
    }

    pub fn try_acquire(&self) -> bool {
        let (lock, _cvar) = &*self.state;
        let mut available = lock.lock().expect("resource semaphore mutex poisoned");
        if *available == 0 {
            return false;
        }
        *available -= 1;
        true
    }

    pub fn release(&self) {
        let (lock, cvar) = &*self.state;
        let mut available = lock.lock().expect("resource semaphore mutex poisoned");
        if *available < self.capacity {
            *available += 1;
            cvar.notify_one();
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn available(&self) -> usize {
        let (lock, _cvar) = &*self.state;
        *lock.lock().expect("resource semaphore mutex poisoned")
    }
}

#[derive(Clone, Debug)]
struct MessageQueue<T> {
    maxsize: Option<usize>,
    state: Arc<(Mutex<VecDeque<T>>, Condvar)>,
}

impl<T> MessageQueue<T> {
    fn new(maxsize: usize) -> Self {
        Self {
            maxsize: if maxsize == 0 { None } else { Some(maxsize) },
            state: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    fn push(&self, value: T) {
        let (lock, cvar) = &*self.state;
        let mut queue = lock.lock().expect("message queue mutex poisoned");
        while self.maxsize.is_some_and(|maxsize| queue.len() >= maxsize) {
            queue = cvar
                .wait(queue)
                .expect("message queue mutex poisoned while waiting");
        }
        queue.push_back(value);
        cvar.notify_all();
    }

    fn pop(&self) -> T {
        let (lock, cvar) = &*self.state;
        let mut queue = lock.lock().expect("message queue mutex poisoned");
        while queue.is_empty() {
            queue = cvar
                .wait(queue)
                .expect("message queue mutex poisoned while waiting");
        }
        let value = queue.pop_front().expect("queue checked non-empty");
        cvar.notify_all();
        value
    }

    fn try_pop(&self) -> Option<T> {
        let (lock, cvar) = &*self.state;
        let mut queue = lock.lock().expect("message queue mutex poisoned");
        let value = queue.pop_front();
        if value.is_some() {
            cvar.notify_all();
        }
        value
    }

    fn len(&self) -> usize {
        let (lock, _cvar) = &*self.state;
        lock.lock().expect("message queue mutex poisoned").len()
    }
}

#[derive(Clone, Debug)]
pub struct HalfDuplexChannel<T> {
    requests: MessageQueue<T>,
    responses: MessageQueue<T>,
}

impl<T> HalfDuplexChannel<T> {
    pub fn new(maxsize: usize) -> Self {
        Self {
            requests: MessageQueue::new(maxsize),
            responses: MessageQueue::new(maxsize),
        }
    }

    pub fn send_request(&self, message: T) {
        self.requests.push(message);
    }

    pub fn receive_request(&self) -> T {
        self.requests.pop()
    }

    pub fn try_receive_request(&self) -> Option<T> {
        self.requests.try_pop()
    }

    pub fn send_response(&self, message: T) {
        self.responses.push(message);
    }

    pub fn receive_response(&self) -> T {
        self.responses.pop()
    }

    pub fn try_receive_response(&self) -> Option<T> {
        self.responses.try_pop()
    }

    pub fn lengths(&self) -> (usize, usize) {
        (self.requests.len(), self.responses.len())
    }
}

#[derive(Clone, Debug)]
pub struct FullDuplexChannel<T> {
    a_to_b: MessageQueue<T>,
    b_to_a: MessageQueue<T>,
}

impl<T> FullDuplexChannel<T> {
    pub fn new(maxsize: usize) -> Self {
        Self {
            a_to_b: MessageQueue::new(maxsize),
            b_to_a: MessageQueue::new(maxsize),
        }
    }

    pub fn send_a_to_b(&self, message: T) {
        self.a_to_b.push(message);
    }

    pub fn receive_a_to_b(&self) -> T {
        self.a_to_b.pop()
    }

    pub fn try_receive_a_to_b(&self) -> Option<T> {
        self.a_to_b.try_pop()
    }

    pub fn send_b_to_a(&self, message: T) {
        self.b_to_a.push(message);
    }

    pub fn receive_b_to_a(&self) -> T {
        self.b_to_a.pop()
    }

    pub fn try_receive_b_to_a(&self) -> Option<T> {
        self.b_to_a.try_pop()
    }

    pub fn lengths(&self) -> (usize, usize) {
        (self.a_to_b.len(), self.b_to_a.len())
    }
}

#[derive(Clone, Debug)]
pub struct ExecutionNetworkBundle {
    pub config: ExecutionNetworkConfig,
    pub cpu_semaphore: ResourceSemaphore,
    pub file_io_semaphore: ResourceSemaphore,
    pub output_semaphore: ResourceSemaphore,
}

impl ExecutionNetworkBundle {
    pub fn snapshot_terms(&self) -> Vec<&'static str> {
        vec![
            "ExecutionNetworkCategory",
            "SchedulerCategory",
            "ChannelCategory",
            "FifoTaskQueue",
            "LifoTaskStack",
            "PriorityTaskQueue",
            "HalfDuplexChannel",
            "FullDuplexChannel",
            "ResourceSemaphore",
            "parallel_chunks_glue_deterministically_to_serial_result",
        ]
    }
}

pub fn bootstrap_execution_network(config: Option<ExecutionNetworkConfig>) -> ExecutionNetworkBundle {
    let config = config.unwrap_or_default();
    ExecutionNetworkBundle {
        cpu_semaphore: ResourceSemaphore::new(config.max_workers),
        file_io_semaphore: ResourceSemaphore::new(4usize.min(config.max_workers).max(1)),
        output_semaphore: ResourceSemaphore::new(1),
        config,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_lifo_priority_are_different_schedules_but_reduce_can_preserve_input_order() {
        let tasks = vec![
            ExecutionTask::new(0, "a").with_priority(3),
            ExecutionTask::new(1, "b").with_priority(1),
            ExecutionTask::new(2, "c").with_priority(2),
        ];
        let priority = ExecutionNetworkConfig::new(1, DataflowDiscipline::Priority);
        let run = execute_tasks_deterministically(&tasks, |value| value.to_string(), Some(priority));
        assert_eq!(run.values, vec!["a", "b", "c"]);
        assert_eq!(run.results[0].task_index, 1);
    }

    #[test]
    fn threaded_execution_glues_back_to_input_order() {
        let tasks = vec![
            ExecutionTask::new(0, 10),
            ExecutionTask::new(1, 20),
            ExecutionTask::new(2, 30),
            ExecutionTask::new(3, 40),
        ];
        let cfg = ExecutionNetworkConfig::new(4, DataflowDiscipline::Lifo);
        let run = execute_tasks_threaded_ordered(&tasks, |value| *value + 1, Some(cfg));
        assert_eq!(run.values, vec![11, 21, 31, 41]);
        assert_eq!(run.mode, "threaded");
        assert!(run.workers > 1);
    }
}
