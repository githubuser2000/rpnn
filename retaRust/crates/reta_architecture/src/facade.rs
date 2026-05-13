use serde::{Deserialize, Serialize};

use crate::category::{bootstrap_category_theory, CategoryTheoryBundle};
use crate::dataflow::{bootstrap_execution_network, ExecutionNetworkBundle, ExecutionTask};
use crate::morphism::{MorphismEdge, MorphismGraph, MorphismKind};
use crate::presheaf::PresheafBundle;
use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};
use crate::sheaf::SheafBundle;
use crate::tag_schema::{bootstrap_tag_schema, TagSchemaBundle};
use crate::topology::{ContextSelection, RetaContextTopology};
use crate::universal::UniversalBundle;

pub const ARCHITECTURE_COUNTS_SNAPSHOT: &str = include_str!("../data/architecture_counts_snapshot.json");

#[derive(Clone, Debug)]
pub struct ArchitectureRuntime {
    pub topology: RetaContextTopology,
    pub category_theory: CategoryTheoryBundle,
    pub execution_network: ExecutionNetworkBundle,
    pub presheaves: PresheafBundle,
    pub row_ranges: RowRangeMorphismBundle,
    pub sheaves: SheafBundle,
    pub tag_schema: TagSchemaBundle,
    pub morphisms: MorphismGraph,
    pub universal: UniversalBundle,
}

impl ArchitectureRuntime {
    pub fn new() -> Self {
        let mut morphisms = MorphismGraph::new();
        morphisms.add(MorphismEdge::new(
            "parse_cli",
            "RawArgs",
            "ContextSelection",
            MorphismKind::Parse,
            "RetaRunArchitecture::from_cli_args",
        ));
        morphisms.add(MorphismEdge::new(
            "parse_prompt",
            "PromptInput",
            "PromptArchitectureContext",
            MorphismKind::Parse,
            "PromptArchitectureContext::from_prompt_input",
        ));
        morphisms.add(MorphismEdge::new(
            "enqueue_task",
            "ContextSelection",
            "ExecutionTask",
            MorphismKind::Enqueue,
            "ExecutionTask::new",
        ));
        morphisms.add(MorphismEdge::new(
            "glue_results",
            "ExecutionResult",
            "GluedSection",
            MorphismKind::Glue,
            "deterministic_reduce",
        ));

        Self {
            topology: RetaContextTopology::standard(),
            category_theory: bootstrap_category_theory(),
            execution_network: bootstrap_execution_network(None),
            presheaves: PresheafBundle::default(),
            row_ranges: bootstrap_row_range_morphisms(None),
            sheaves: SheafBundle::default(),
            tag_schema: bootstrap_tag_schema(),
            morphisms,
            universal: UniversalBundle::new(),
        }
    }

    pub fn architecture_terms(&self) -> Vec<&'static str> {
        vec![
            "network",
            "queue",
            "stack",
            "fifo",
            "lifo",
            "dataflow",
            "bidirectionality",
            "semaphore",
            "topology",
            "tag_schema",
            "row_ranges",
            "morphism",
            "universal_property",
            "presheaf",
            "sheaf",
            "category",
            "functor",
            "natural_transformation",
        ]
    }

    pub fn snapshot_ref(&self) -> ArchitectureSnapshotRef {
        ArchitectureSnapshotRef {
            py_architecture_counts_json: ARCHITECTURE_COUNTS_SNAPSHOT,
            py_category_theory_json: self.category_theory.python_snapshot(),
            py_execution_network_json: crate::dataflow::EXECUTION_NETWORK_SNAPSHOT,
            rust_category_count: self.category_theory.categories.len(),
            rust_functor_count: self.category_theory.functors.len(),
            rust_natural_transformation_count: self.category_theory.natural_transformations.len(),
        }
    }
}

impl Default for ArchitectureRuntime {
    fn default() -> Self {
        Self::new()
    }
}

pub fn bootstrap_architecture_runtime() -> ArchitectureRuntime {
    ArchitectureRuntime::new()
}

#[derive(Clone, Debug)]
pub struct ArchitectureSnapshotRef {
    pub py_architecture_counts_json: &'static str,
    pub py_category_theory_json: &'static str,
    pub py_execution_network_json: &'static str,
    pub rust_category_count: usize,
    pub rust_functor_count: usize,
    pub rust_natural_transformation_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetaRunArchitecture {
    pub context: ContextSelection,
    pub args_len: usize,
    pub scheduled_task_count: usize,
    pub topology_owner: String,
    pub universal_property: String,
}

impl RetaRunArchitecture {
    pub fn from_cli_args(args: &[String]) -> Self {
        let context = ContextSelection::from_cli_args(args);
        let task = ExecutionTask::new(0usize, args.to_vec()).with_operation("rreta_cli_run");
        Self {
            context,
            args_len: args.len(),
            scheduled_task_count: usize::from(!task.payload.is_empty()),
            topology_owner: "OpenRetaContextCategory".to_string(),
            universal_property:
                "same_cli_context_maps_to_same_ordered_rreta_result".to_string(),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "args={} tasks={} owner={} universal={}",
            self.args_len, self.scheduled_task_count, self.topology_owner, self.universal_property
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptArchitectureContext {
    pub program_name: String,
    pub input_len: usize,
    pub token_count: usize,
    pub context: ContextSelection,
    pub data_stream_direction: String,
    pub universal_property: String,
}

impl PromptArchitectureContext {
    pub fn from_prompt_input(program_name: &str, input: &str) -> Self {
        let token_count = input.split_whitespace().count();
        Self {
            program_name: program_name.to_string(),
            input_len: input.chars().count(),
            token_count,
            context: ContextSelection::from_prompt_input(program_name, input),
            data_stream_direction: "bidirectional_prompt_reta_channel".to_string(),
            universal_property:
                "prompt_local_state_glues_to_same_compiled_reta_command".to_string(),
        }
    }

    pub fn as_task(&self, index: usize) -> ExecutionTask<String> {
        ExecutionTask::new(index, self.program_name.clone())
            .with_operation("retaprompt_context")
            .with_metadata("tokens", self.token_count.to_string())
            .with_metadata("direction", self.data_stream_direction.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_exposes_architecture_terms() {
        let runtime = bootstrap_architecture_runtime();
        assert!(runtime.architecture_terms().contains(&"fifo"));
        assert!(runtime.architecture_terms().contains(&"natural_transformation"));
        assert!(runtime.snapshot_ref().py_category_theory_json.contains("Functor"));
    }

    #[test]
    fn prompt_context_is_bidirectional() {
        let context = PromptArchitectureContext::from_prompt_input("rp", "reta -zeilen --alles");
        assert!(context.data_stream_direction.contains("bidirectional"));
        assert!(context.context.scopes.unwrap().contains("embedded_reta"));
    }
}
