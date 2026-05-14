#![allow(non_snake_case)]

//! Shared typed architecture layer for `rreta` and `rretaPrompt`.
//!
//! This crate is the first Rust transcompilation layer for the modular
//! `py reta arch` source.  It deliberately keeps visible Reta behaviour stable:
//! the old output/parity code still renders.  The new structures make the
//! architecture explicit so later ports can target Rust modules instead of the
//! historical Python monolith.

pub mod category;
pub mod column_selection;
pub mod completion_runtime;
pub mod completion_word;
pub mod dataflow;
pub mod facade;
pub mod morphism;
pub mod number_theory;
pub mod output_semantics;
pub mod output_syntax;
pub mod parameter_runtime;
pub mod presheaf;
pub mod program_workflow;
pub mod prompt_language;
pub mod row_filtering;
pub mod row_ranges;
pub mod sheaf;
pub mod table_generation;
pub mod table_output;
pub mod table_preparation;
pub mod table_runtime;
pub mod table_state;
pub mod table_wrapping;
pub mod tag_schema;
pub mod topology;
pub mod universal;

pub use category::{
    bootstrap_category_theory, CategoryMorphismSpec, CategoryObjectSpec, CategorySpec,
    CategoryTheoryBundle, FunctorSpec, NaturalTransformationSpec, PYTHON_CATEGORY_THEORY_SNAPSHOT,
};
pub use column_selection::{
    bootstrap_column_selection, ColumnBucketKey, ColumnBucketSnapshot, ColumnSelectionBundle,
    ColumnSelectionSnapshot, COLUMN_BUCKET_NAMES, COLUMN_BUCKET_VALUES,
};
pub use completion_runtime::{
    bootstrap_completion_runtime, sort_completion_key, CompletionRuntimeBundle,
    CompletionRuntimeSnapshot, CompletionSortKey,
};
pub use completion_word::{
    bootstrap_word_completion_morphisms, iter_word_completions, resolve_words, word_before_cursor,
    word_completion_matches, CompletionCandidate, PromptDocument, WordCompletionMorphismBundle,
    WordCompletionOptions, WordCompletionSnapshot,
};
pub use dataflow::{
    bootstrap_execution_network, deterministic_reduce, execute_tasks_deterministically,
    DataflowDiscipline, ExecutionNetworkBundle, ExecutionNetworkConfig, ExecutionResult,
    ExecutionRunResult, ExecutionTask, FifoTaskQueue, FullDuplexChannel, HalfDuplexChannel,
    LifoTaskStack, PriorityTaskQueue, ResourceSemaphore, EXECUTION_NETWORK_SNAPSHOT,
};
pub use facade::{
    bootstrap_architecture_runtime, ArchitectureRuntime, ArchitectureSnapshotRef,
    PromptArchitectureContext, RetaRunArchitecture, ARCHITECTURE_COUNTS_SNAPSHOT,
};
pub use morphism::{MorphismEdge, MorphismGraph, MorphismKind};
pub use number_theory::{
    bootstrap_number_theory, could_be_prime_number_primzahlkreuz,
    could_be_prime_number_primzahlkreuz_fuer_aussen,
    could_be_prime_number_primzahlkreuz_fuer_innen, divisor_generator, is_prime_multiple,
    moon_number, prime_creativity, prime_factors, prime_multiple, prime_repeat, NumberTheoryBundle,
    NumberTheorySnapshot,
};
pub use output_semantics::{
    bootstrap_output_semantics, OutputConfig, OutputModeApplication, OutputSemanticsSnapshot,
    RetaOutputSemantics,
};
pub use output_syntax::{
    bootstrap_output_syntax, colored_begin_col, generate_cell_begin, output_syntax_snapshot,
    OutputMode, OutputModeSpec, OutputSyntaxBundle, OutputSyntaxSnapshot, SyntaxMarkup,
};
pub use parameter_runtime::{
    bootstrap_parameter_runtime, MainParameter, ParameterCommandSets, ParameterParseResult,
    ParameterRuntimeBundle, ParameterRuntimeSnapshot, ParameterToken, ParameterTokenKind,
};
pub use presheaf::{LocalSection, Presheaf, PresheafBundle};
pub use program_workflow::{
    bootstrap_program_workflow, ProgramWorkflowBundle, ProgramWorkflowSnapshot, WorkflowStep,
    WorkflowTrace,
};
pub use prompt_language::{
    bootstrap_prompt_language, custom_split, custom_split2, is_15_or_16_command, is_reta_parameter,
    verkuerze_dict, FractionOrIntegerCheck, PromptLanguageBundle, PromptLanguageSnapshot,
    PromptModus,
};
pub use row_filtering::{
    bootstrap_row_filtering, delete_doubles_in_sets, filter_original_lines, from_until,
    moon_sun_filter, parameters_cmd_with_some_bereich, RowFilterContext, RowFilteringBundle,
    RowFilteringSnapshot,
};
pub use row_ranges::{
    bootstrap_row_range_morphisms, is_fraction_or_integer_range, is_fraction_range,
    is_fraction_range_token, is_integer_range_token, is_row_range, is_row_range_token,
    range_to_numbers, str_as_generator_to_set, RowRangeMorphismBundle, RowRangeSyntax,
};
pub use sheaf::{GluedSection, Sheaf, SheafBundle};
pub use table_generation::{
    bootstrap_table_generation, TableGenerationBundle, TableGenerationBundleSnapshot,
    TableGenerationPlan, TableGenerationResult, TableGenerationResultSnapshot,
};
pub use table_output::{
    bootstrap_table_output, colorize, determine_row_width, max_cell_text_len, only_that_columns_fn,
    render_prepared_table, BreakoutReason, TableOutputBundle, TableOutputBundleSnapshot,
    TableOutputConfig, TableRenderResult,
};
pub use tag_schema::{
    bootstrap_tag_schema, columns_for_tags_in_selector, groups_for_selector,
    kombi_table2_tags_for_column, kombi_table_tags_for_column, ordinary_columns_for_tags,
    ordinary_tags_for_column, reverse_map_for_selector, TableTag, TagGroup, TagSchemaBundle,
    TagSchemaSnapshot, TagTableSelector, KOMBI_TABLE2_TAG_GROUPS, KOMBI_TABLE_TAG_GROUPS,
    ORDINARY_TAG_GROUPS,
};

pub use table_preparation::{
    bootstrap_table_preparation, cell_work, prepare_output_table, prepare_row_cells,
    select_display_lines, tag_output_column, DisplayLineSelection, GebrSpalten,
    KombiTablePreparationResult, KombiTablePreparationResultSnapshot, MainTablePreparationResult,
    MainTablePreparationResultSnapshot, OldNewTableMap, PreparedCell, PreparedRow,
    PreparedRowWithMap, PreparedTable, TablePreparationBundle, TablePreparationBundleSnapshot,
    TablePreparationContext,
};
pub use table_runtime::{
    bootstrap_table_runtime, TableRuntimeBundle, TableRuntimeBundleSnapshot, TableRuntimeState,
    TableRuntimeStateSnapshot,
};
pub use table_state::{
    bootstrap_table_state, highest_rows, GeneratedColumnSection, GeneratedColumnSectionSnapshot,
    TableDisplayState, TableDisplayStateSnapshot, TableStateBundle, TableStateBundleSnapshot,
    TableStateSections, TableStateSectionsSnapshot,
};
pub use table_wrapping::{
    alxwrap, bootstrap_table_wrapping, chunks, split_more_if_not_small, width_for_row_context,
    wrap_cell_text, TableWidthContext, TableWrappingBundle, TableWrappingBundleSnapshot,
    TextWrapRuntime, TextWrapRuntimeSnapshot, WrapType,
};
pub use topology::{ContextDimension, ContextSelection, RetaContextTopology};
pub use universal::{merge_parameter_dicts, normalize_column_buckets, UniversalBundle};
