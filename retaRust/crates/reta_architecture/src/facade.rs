use serde::{Deserialize, Serialize};

use crate::category::{bootstrap_category_theory, CategoryTheoryBundle};
use crate::completion_runtime::{bootstrap_completion_runtime, CompletionRuntimeBundle};
use crate::column_selection::{bootstrap_column_selection, ColumnSelectionBundle};
use crate::dataflow::{bootstrap_execution_network, ExecutionNetworkBundle, ExecutionTask};
use crate::morphism::{MorphismEdge, MorphismGraph, MorphismKind};
use crate::output_syntax::{bootstrap_output_syntax, OutputSyntaxBundle};
use crate::output_semantics::{bootstrap_output_semantics, RetaOutputSemantics};
use crate::number_theory::{bootstrap_number_theory, NumberTheoryBundle};
use crate::presheaf::PresheafBundle;
use crate::program_workflow::{bootstrap_program_workflow, ProgramWorkflowBundle};
use crate::parameter_runtime::{bootstrap_parameter_runtime, ParameterRuntimeBundle};
use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};
use crate::prompt_language::{bootstrap_prompt_language, PromptLanguageBundle};
use crate::sheaf::SheafBundle;
use crate::table_runtime::{bootstrap_table_runtime, TableRuntimeBundle};
use crate::table_generation::{bootstrap_table_generation, TableGenerationBundle};
use crate::tag_schema::{bootstrap_tag_schema, TagSchemaBundle};
use crate::table_state::{bootstrap_table_state, TableStateBundle};
use crate::topology::{ContextSelection, RetaContextTopology};
use crate::universal::UniversalBundle;

pub const ARCHITECTURE_COUNTS_SNAPSHOT: &str = include_str!("../data/architecture_counts_snapshot.json");

#[derive(Clone, Debug)]
pub struct ArchitectureRuntime {
    pub topology: RetaContextTopology,
    pub category_theory: CategoryTheoryBundle,
    pub execution_network: ExecutionNetworkBundle,
    pub column_selection: ColumnSelectionBundle,
    pub completion_runtime: CompletionRuntimeBundle,
    pub number_theory: NumberTheoryBundle,
    pub output_semantics: RetaOutputSemantics,
    pub output_syntax: OutputSyntaxBundle,
    pub parameter_runtime: ParameterRuntimeBundle,
    pub program_workflow: ProgramWorkflowBundle,
    pub prompt_language: PromptLanguageBundle,
    pub presheaves: PresheafBundle,
    pub row_ranges: RowRangeMorphismBundle,
    pub sheaves: SheafBundle,
    pub tag_schema: TagSchemaBundle,
    pub table_generation: TableGenerationBundle,
    pub table_runtime: TableRuntimeBundle,
    pub table_state: TableStateBundle,
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
            column_selection: bootstrap_column_selection(),
            completion_runtime: bootstrap_completion_runtime(),
            number_theory: bootstrap_number_theory(),
            output_semantics: bootstrap_output_semantics(),
            output_syntax: bootstrap_output_syntax(),
            parameter_runtime: bootstrap_parameter_runtime(),
            program_workflow: bootstrap_program_workflow(),
            prompt_language: bootstrap_prompt_language(),
            presheaves: PresheafBundle::default(),
            row_ranges: bootstrap_row_range_morphisms(None),
            sheaves: SheafBundle::default(),
            tag_schema: bootstrap_tag_schema(),
            table_generation: bootstrap_table_generation(),
            table_runtime: bootstrap_table_runtime(),
            table_state: bootstrap_table_state(),
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
            "number_theory",
            "column_selection",
            "parameter_runtime",
            "output_syntax",
            "output_semantics",
            "table_state",
            "table_runtime",
            "table_generation",
            "program_workflow",
            "prompt_language",
            "completion_runtime",
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
            rust_column_bucket_count: self.column_selection.bucket_values().len(),
            rust_output_mode_count: self.output_syntax.modes().len(),
            rust_parameter_main_count: self.parameter_runtime.main_commands.len(),
            rust_prompt_start_command_count: self.completion_runtime.start_commands(true).len(),
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ArchitectureSnapshotRef {
    pub py_architecture_counts_json: &'static str,
    pub py_category_theory_json: &'static str,
    pub py_execution_network_json: &'static str,
    pub rust_category_count: usize,
    pub rust_functor_count: usize,
    pub rust_natural_transformation_count: usize,
    pub rust_column_bucket_count: usize,
    pub rust_output_mode_count: usize,
    pub rust_parameter_main_count: usize,
    pub rust_prompt_start_command_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetaRunArchitecture {
    pub context: ContextSelection,
    pub args_len: usize,
    pub scheduled_task_count: usize,
    pub parameter_main_count: usize,
    pub selected_output_mode: Option<String>,
    pub upper_limit: Option<i64>,
    pub topology_owner: String,
    pub universal_property: String,
}

impl RetaRunArchitecture {
    pub fn from_cli_args(args: &[String]) -> Self {
        let context = ContextSelection::from_cli_args(args);
        let task = ExecutionTask::new(0usize, args.to_vec()).with_operation("rreta_cli_run");
        let parameter_runtime = bootstrap_parameter_runtime();
        let parsed = parameter_runtime.parse_cli_args(args);
        Self {
            context,
            args_len: args.len(),
            scheduled_task_count: usize::from(!task.payload.is_empty()),
            parameter_main_count: parsed.main_context_history.len(),
            selected_output_mode: parsed
                .selected_output_mode
                .map(|mode| mode.canonical_name().to_string()),
            upper_limit: parsed.upper_limit,
            topology_owner: "OpenRetaContextCategory".to_string(),
            universal_property:
                "same_cli_context_maps_to_same_ordered_rreta_result".to_string(),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "args={} tasks={} mains={} output={:?} upper={:?} owner={} universal={}",
            self.args_len,
            self.scheduled_task_count,
            self.parameter_main_count,
            self.selected_output_mode,
            self.upper_limit,
            self.topology_owner,
            self.universal_property
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptArchitectureContext {
    pub program_name: String,
    pub input_len: usize,
    pub token_count: usize,
    pub start_command_count: usize,
    pub context: ContextSelection,
    pub data_stream_direction: String,
    pub universal_property: String,
}

impl PromptArchitectureContext {
    pub fn from_prompt_input(program_name: &str, input: &str) -> Self {
        let token_count = input.split_whitespace().count();
        let completion_runtime = bootstrap_completion_runtime();
        Self {
            program_name: program_name.to_string(),
            input_len: input.chars().count(),
            token_count,
            start_command_count: completion_runtime.start_commands(true).len(),
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
