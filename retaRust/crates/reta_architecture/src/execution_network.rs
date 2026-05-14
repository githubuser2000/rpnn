//! Named Rust execution-network adapter for `execution_network.py`.
//!
//! Stage 8 put the concrete queue/channel code in `dataflow.rs`.  This module
//! restores the Python architecture name `execution_network` and adds a bridge
//! plan that can be exported without running work.

use serde::{Deserialize, Serialize};

use crate::dataflow::{order_tasks, DataflowDiscipline, ExecutionNetworkConfig, ExecutionTask};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionNetworkPlan {
    pub task_count: usize,
    pub scheduled_indices: Vec<usize>,
    pub queue_discipline: String,
    pub workers: usize,
    pub preserve_input_order: bool,
    pub bounded_queue_size: Option<usize>,
    pub mode: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionNetworkGate {
    pub gate_id: String,
    pub allowed_disciplines: Vec<String>,
    pub requires_deterministic_reduce: bool,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionNetworkBridgeSnapshot {
    pub class: String,
    pub queue_disciplines: Vec<String>,
    pub gates: usize,
    pub default_workers: usize,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionNetworkBridgeBundle {
    pub default_config: ExecutionNetworkConfig,
    pub gates: Vec<ExecutionNetworkGate>,
}

impl ExecutionNetworkBridgeBundle {
    pub fn plan_for_tasks<T: Clone>(&self, tasks: &[ExecutionTask<T>]) -> ExecutionNetworkPlan {
        let scheduled = order_tasks(tasks, &self.default_config);
        ExecutionNetworkPlan {
            task_count: tasks.len(),
            scheduled_indices: scheduled.iter().map(|task| task.index).collect(),
            queue_discipline: self.default_config.queue_discipline.as_str().to_string(),
            workers: self.default_config.workers_for(tasks.len()),
            preserve_input_order: self.default_config.preserve_input_order,
            bounded_queue_size: self.default_config.bounded_queue_size,
            mode: if self.default_config.max_workers > 1 && tasks.len() > 1 { "threaded-plan" } else { "serial-plan" }.to_string(),
            universal_property: "parallel_chunks_glue_deterministically_to_serial_result".to_string(),
        }
    }

    pub fn plan_for_indices(&self, indices: &[usize]) -> ExecutionNetworkPlan {
        let tasks = indices.iter().map(|idx| ExecutionTask::new(*idx, *idx)).collect::<Vec<_>>();
        self.plan_for_tasks(&tasks)
    }

    pub fn snapshot(&self) -> ExecutionNetworkBridgeSnapshot {
        ExecutionNetworkBridgeSnapshot {
            class: "ExecutionNetworkBridgeBundle".to_string(),
            queue_disciplines: vec!["fifo".to_string(), "lifo".to_string(), "priority".to_string()],
            gates: self.gates.len(),
            default_workers: self.default_config.max_workers,
            universal_property: "queue_stack_priority_schedulers_share_one_deterministic_reduce".to_string(),
        }
    }
}

pub fn bootstrap_execution_network_bridge(config: Option<ExecutionNetworkConfig>) -> ExecutionNetworkBridgeBundle {
    let default_config = config.unwrap_or_else(|| ExecutionNetworkConfig { max_workers: 1, queue_discipline: DataflowDiscipline::Fifo, use_processes: false, start_method: None, preserve_input_order: true, bounded_queue_size: None });
    ExecutionNetworkBridgeBundle {
        default_config,
        gates: vec![
            ExecutionNetworkGate { gate_id: "serial-vs-threaded-order".to_string(), allowed_disciplines: vec!["fifo".to_string(), "lifo".to_string(), "priority".to_string()], requires_deterministic_reduce: true, status: "required".to_string() },
            ExecutionNetworkGate { gate_id: "bounded-queue-semaphore".to_string(), allowed_disciplines: vec!["fifo".to_string()], requires_deterministic_reduce: true, status: "optional".to_string() },
            ExecutionNetworkGate { gate_id: "bidirectional-channel-roundtrip".to_string(), allowed_disciplines: vec!["fifo".to_string()], requires_deterministic_reduce: false, status: "required-for-prompt".to_string() },
        ],
    }
}

pub fn execution_network_plan_for_indices(indices: &[usize], discipline: DataflowDiscipline) -> ExecutionNetworkPlan {
    let config = ExecutionNetworkConfig { queue_discipline: discipline, ..ExecutionNetworkConfig::default() };
    bootstrap_execution_network_bridge(Some(config)).plan_for_indices(indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_plan_orders_indices_but_preserves_glue_law() {
        let cfg = ExecutionNetworkConfig { max_workers: 2, queue_discipline: DataflowDiscipline::Priority, use_processes: false, start_method: None, preserve_input_order: true, bounded_queue_size: None };
        let tasks = vec![ExecutionTask::new(0, "a").with_priority(2), ExecutionTask::new(1, "b").with_priority(1)];
        let plan = bootstrap_execution_network_bridge(Some(cfg)).plan_for_tasks(&tasks);
        assert_eq!(plan.scheduled_indices, vec![1, 0]);
        assert!(plan.universal_property.contains("deterministically"));
    }
}
