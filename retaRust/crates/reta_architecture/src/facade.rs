use serde::{Deserialize, Serialize};

use crate::category::{bootstrap_category_theory, CategoryTheoryBundle};
use crate::column_selection::{bootstrap_column_selection, ColumnSelectionBundle};
use crate::combi_join::{bootstrap_combi_join, KombiJoinBundle};
use crate::completion_runtime::{bootstrap_completion_runtime, CompletionRuntimeBundle};
use crate::completion_nested::{bootstrap_nested_completion_morphisms, NestedCompletionMorphismBundle};
use crate::completion_word::{bootstrap_word_completion_morphisms, WordCompletionMorphismBundle};
use crate::concat_csv::{bootstrap_concat_csv, ConcatCsvBundle};
use crate::dataflow::{bootstrap_execution_network, ExecutionNetworkBundle, ExecutionTask};
use crate::generated_columns::{bootstrap_generated_columns, GeneratedColumnsBundle};
use crate::morphism::{MorphismEdge, MorphismGraph, MorphismKind};
use crate::meta_columns::{bootstrap_meta_columns, MetaColumnsBundle};
use crate::number_theory::{bootstrap_number_theory, NumberTheoryBundle};
use crate::output_semantics::{bootstrap_output_semantics, RetaOutputSemantics};
use crate::output_syntax::{bootstrap_output_syntax, OutputSyntaxBundle};
use crate::parameter_runtime::{bootstrap_parameter_runtime, ParameterRuntimeBundle};
use crate::presheaf::PresheafBundle;
use crate::program_workflow::{bootstrap_program_workflow, ProgramWorkflowBundle};
use crate::prompt_execution::{bootstrap_prompt_execution, PromptExecutionBundle};
use crate::prompt_interaction::{bootstrap_prompt_interaction, PromptInteractionBundle};
use crate::prompt_preparation::{bootstrap_prompt_preparation, PromptPreparationBundle};
use crate::prompt_runtime::{bootstrap_prompt_runtime, PromptRuntimeBundle};
use crate::prompt_session::{bootstrap_prompt_session, PromptSessionBundle};
use crate::prompt_language::{bootstrap_prompt_language, PromptLanguageBundle};
use crate::row_filtering::{bootstrap_row_filtering, RowFilteringBundle};
use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};
use crate::sheaf::SheafBundle;
use crate::table_generation::{bootstrap_table_generation, TableGenerationBundle};
use crate::table_output::{bootstrap_table_output, TableOutputBundle};
use crate::table_preparation::{bootstrap_table_preparation, TablePreparationBundle};
use crate::table_runtime::{bootstrap_table_runtime, TableRuntimeBundle};
use crate::table_state::{bootstrap_table_state, TableStateBundle};
use crate::table_wrapping::{bootstrap_table_wrapping, TableWrappingBundle};
use crate::tag_schema::{bootstrap_tag_schema, TagSchemaBundle};
use crate::topology::{ContextSelection, RetaContextTopology};
use crate::universal::UniversalBundle;

pub const ARCHITECTURE_COUNTS_SNAPSHOT: &str =
    include_str!("../data/architecture_counts_snapshot.json");

#[derive(Clone, Debug)]
pub struct ArchitectureRuntime {
    pub topology: RetaContextTopology,
    pub category_theory: CategoryTheoryBundle,
    pub execution_network: ExecutionNetworkBundle,
    pub column_selection: ColumnSelectionBundle,
    pub combi_join: KombiJoinBundle,
    pub completion_runtime: CompletionRuntimeBundle,
    pub completion_nested: NestedCompletionMorphismBundle,
    pub completion_word: WordCompletionMorphismBundle,
    pub concat_csv: ConcatCsvBundle,
    pub generated_columns: GeneratedColumnsBundle,
    pub meta_columns: MetaColumnsBundle,
    pub number_theory: NumberTheoryBundle,
    pub output_semantics: RetaOutputSemantics,
    pub output_syntax: OutputSyntaxBundle,
    pub parameter_runtime: ParameterRuntimeBundle,
    pub program_workflow: ProgramWorkflowBundle,
    pub prompt_runtime: PromptRuntimeBundle,
    pub prompt_session: PromptSessionBundle,
    pub prompt_preparation: PromptPreparationBundle,
    pub prompt_execution: PromptExecutionBundle,
    pub prompt_interaction: PromptInteractionBundle,
    pub prompt_language: PromptLanguageBundle,
    pub presheaves: PresheafBundle,
    pub row_filtering: RowFilteringBundle,
    pub row_ranges: RowRangeMorphismBundle,
    pub sheaves: SheafBundle,
    pub tag_schema: TagSchemaBundle,
    pub table_generation: TableGenerationBundle,
    pub table_output: TableOutputBundle,
    pub table_preparation: TablePreparationBundle,
    pub table_runtime: TableRuntimeBundle,
    pub table_state: TableStateBundle,
    pub table_wrapping: TableWrappingBundle,
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
            combi_join: bootstrap_combi_join(),
            completion_runtime: bootstrap_completion_runtime(),
            completion_nested: bootstrap_nested_completion_morphisms(),
            completion_word: bootstrap_word_completion_morphisms(),
            concat_csv: bootstrap_concat_csv(),
            generated_columns: bootstrap_generated_columns(),
            meta_columns: bootstrap_meta_columns(),
            number_theory: bootstrap_number_theory(),
            output_semantics: bootstrap_output_semantics(),
            output_syntax: bootstrap_output_syntax(),
            parameter_runtime: bootstrap_parameter_runtime(),
            program_workflow: bootstrap_program_workflow(),
            prompt_runtime: bootstrap_prompt_runtime(),
            prompt_session: bootstrap_prompt_session(),
            prompt_preparation: bootstrap_prompt_preparation(),
            prompt_execution: bootstrap_prompt_execution(),
            prompt_interaction: bootstrap_prompt_interaction(),
            prompt_language: bootstrap_prompt_language(),
            presheaves: PresheafBundle::default(),
            row_filtering: bootstrap_row_filtering(),
            row_ranges: bootstrap_row_range_morphisms(None),
            sheaves: SheafBundle::default(),
            tag_schema: bootstrap_tag_schema(),
            table_generation: bootstrap_table_generation(),
            table_output: bootstrap_table_output(),
            table_preparation: bootstrap_table_preparation(),
            table_runtime: bootstrap_table_runtime(),
            table_state: bootstrap_table_state(),
            table_wrapping: bootstrap_table_wrapping(),
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
            "row_filtering",
            "number_theory",
            "column_selection",
            "parameter_runtime",
            "output_syntax",
            "output_semantics",
            "table_state",
            "table_runtime",
            "table_generation",
            "table_preparation",
            "table_output",
            "table_wrapping",
            "generated_columns",
            "meta_columns",
            "concat_csv",
            "combi_join",
            "program_workflow",
            "prompt_language",
            "completion_runtime",
            "completion_word",
            "completion_nested",
            "prompt_runtime",
            "prompt_session",
            "prompt_preparation",
            "prompt_execution",
            "prompt_interaction",
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
            rust_row_filter_condition_count: self.row_filtering.snapshot().condition_families.len(),
            rust_table_preparation_morphism_count: self
                .table_preparation
                .snapshot()
                .universal_operations
                .len(),
            rust_table_output_morphism_count: self.table_output.snapshot().morphisms.len(),
            rust_word_completion_morphism_count: self.completion_word.snapshot().morphisms.len(),
            rust_nested_completion_morphism_count: self.completion_nested.snapshot().morphisms.len(),
            rust_prompt_runtime_main_count: self.prompt_runtime.snapshot().main_para_cmds.len(),
            rust_prompt_session_end_command_count: self.prompt_session.snapshot().befehle_beenden_len,
            rust_prompt_preparation_domain_count: self.prompt_preparation.snapshot().cached_parameter_value_domains.len(),
            rust_prompt_execution_command_count: self.prompt_execution.snapshot().known_commands_len,
            rust_prompt_interaction_command_count: self.prompt_interaction.snapshot().befehle_len,
            rust_generated_column_morphism_count: self.generated_columns.snapshot().count,
            rust_meta_column_morphism_count: self.meta_columns.snapshot().count,
            rust_concat_csv_morphism_count: self.concat_csv.snapshot().count,
            rust_combi_join_morphism_count: self.combi_join.snapshot().count,
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
    pub rust_row_filter_condition_count: usize,
    pub rust_table_preparation_morphism_count: usize,
    pub rust_table_output_morphism_count: usize,
    pub rust_word_completion_morphism_count: usize,
    pub rust_nested_completion_morphism_count: usize,
    pub rust_prompt_runtime_main_count: usize,
    pub rust_prompt_session_end_command_count: usize,
    pub rust_prompt_preparation_domain_count: usize,
    pub rust_prompt_execution_command_count: usize,
    pub rust_prompt_interaction_command_count: usize,
    pub rust_generated_column_morphism_count: usize,
    pub rust_meta_column_morphism_count: usize,
    pub rust_concat_csv_morphism_count: usize,
    pub rust_combi_join_morphism_count: usize,
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
            universal_property: "same_cli_context_maps_to_same_ordered_rreta_result".to_string(),
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
    pub word_completion_sample_count: usize,
    pub nested_completion_preview_count: usize,
    pub prepared_token_count: usize,
    pub execution_plan_argv_count: usize,
    pub context: ContextSelection,
    pub data_stream_direction: String,
    pub universal_property: String,
}

impl PromptArchitectureContext {
    pub fn from_prompt_input(program_name: &str, input: &str) -> Self {
        let token_count = input.split_whitespace().count();
        let completion_runtime = bootstrap_completion_runtime();
        let word_completion = bootstrap_word_completion_morphisms();
        let nested_completion = bootstrap_nested_completion_morphisms();
        let prompt_preparation = bootstrap_prompt_preparation();
        let prompt_execution = bootstrap_prompt_execution();
        let text_state = crate::prompt_session::PromptTextState::new(input);
        let prepared = prompt_preparation.prepare_large_output(
            "",
            crate::prompt_language::PromptModus::Normal,
            crate::prompt_language::PromptModus::Normal,
            crate::prompt_language::PromptModus::Normal,
            input,
            &[],
        );
        let execution_plan = prompt_execution.plan_prompt_execution(&prepared, &text_state);
        Self {
            program_name: program_name.to_string(),
            input_len: input.chars().count(),
            token_count,
            start_command_count: completion_runtime.start_commands(true).len(),
            word_completion_sample_count: word_completion.sample_completions("re").len(),
            nested_completion_preview_count: nested_completion.complete(input).len(),
            prepared_token_count: prepared.tokens.len(),
            execution_plan_argv_count: execution_plan.reta_argv.len(),
            context: ContextSelection::from_prompt_input(program_name, input),
            data_stream_direction: "bidirectional_prompt_reta_channel".to_string(),
            universal_property: "prompt_local_state_glues_to_same_compiled_reta_command"
                .to_string(),
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
        assert!(runtime
            .architecture_terms()
            .contains(&"natural_transformation"));
        assert!(runtime
            .snapshot_ref()
            .py_category_theory_json
            .contains("Functor"));
    }

    #[test]
    fn prompt_context_is_bidirectional() {
        let context = PromptArchitectureContext::from_prompt_input("rp", "reta -zeilen --alles");
        assert!(context.data_stream_direction.contains("bidirectional"));
        assert!(context.context.scopes.unwrap().contains("embedded_reta"));
    }
}
