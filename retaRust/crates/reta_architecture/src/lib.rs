#![allow(non_snake_case)]

//! Shared typed architecture layer for `rreta` and `rretaPrompt`.
//!
//! This crate is the first Rust transcompilation layer for the modular
//! `py reta arch` source.  It deliberately keeps visible Reta behaviour stable:
//! the old output/parity code still renders.  The new structures make the
//! architecture explicit so later ports can target Rust modules instead of the
//! historical Python monolith.

pub mod architecture_activation;
pub mod architecture_boundaries;
pub mod architecture_coherence;
pub mod architecture_contracts;
pub mod architecture_impact;
pub mod architecture_map;
pub mod architecture_migration;
pub mod architecture_progress;
pub mod architecture_rehearsal;
pub mod architecture_traces;
pub mod architecture_validation;
pub mod architecture_witnesses;
pub mod arithmetic;
pub mod category;
pub mod column_selection;
pub mod combi_join;
pub mod completion_nested;
pub mod completion_runtime;
pub mod completion_word;
pub mod concat_csv;
pub mod console_io;
pub mod dataflow;
pub mod execution_network;
pub mod facade;
pub mod generated_columns;
pub mod input_semantics;
pub mod meta_columns;
pub mod migration_control;
pub mod morphism;
pub mod number_theory;
pub mod output_semantics;
pub mod output_syntax;
pub mod package_integrity;
pub mod parallel_execution;
pub mod parity_harness;
pub mod parameter_runtime;
pub mod persistence;
pub mod presheaf;
pub mod program_workflow;
pub mod prompt_execution;
pub mod prompt_interaction;
pub mod prompt_language;
pub mod prompt_preparation;
pub mod prompt_runtime;
pub mod prompt_session;
pub mod row_filtering;
pub mod row_ranges;
pub mod runtime_compat;
pub mod runtime_switch;
pub mod schema;
pub mod semantics_builder;
pub mod sheaf;
pub mod shadow_pipeline;
pub mod split_i18n;
pub mod table_adapters;
pub mod table_generation;
pub mod table_output;
pub mod table_preparation;
pub mod table_runtime;
pub mod table_state;
pub mod table_wrapping;
pub mod tag_schema;
pub mod topology;
pub mod universal;

pub use architecture_activation::{
    bootstrap_architecture_activation, ArchitectureActivationBundle, ArchitectureActivationSnapshot,
};
pub use architecture_boundaries::{
    bootstrap_architecture_boundaries, ArchitectureBoundariesBundle, ArchitectureBoundariesSnapshot,
};
pub use architecture_coherence::{
    bootstrap_architecture_coherence, ArchitectureCoherenceBundle, ArchitectureCoherenceSnapshot,
};
pub use architecture_contracts::{
    bootstrap_architecture_contracts, ArchitectureContractsBundle, ArchitectureContractsSnapshot,
};
pub use architecture_impact::{
    bootstrap_architecture_impact, ArchitectureImpactBundle, ArchitectureImpactSnapshot,
};
pub use architecture_map::{
    bootstrap_architecture_map, ArchitectureMapBundle, ArchitectureMapSnapshot,
};
pub use architecture_migration::{
    bootstrap_architecture_migration, ArchitectureMigrationBundle, ArchitectureMigrationSnapshot,
};
pub use architecture_progress::{
    bootstrap_architecture_progress, ArchitectureProgressBundle, ArchitectureProgressSnapshot,
};
pub use architecture_rehearsal::{
    bootstrap_architecture_rehearsal, ArchitectureRehearsalBundle, ArchitectureRehearsalSnapshot,
};
pub use architecture_traces::{
    bootstrap_architecture_traces, ArchitectureTraceBundle, ArchitectureTraceSnapshot,
};
pub use architecture_validation::{
    bootstrap_architecture_validation, ArchitectureValidationBundle, ArchitectureValidationSnapshot,
};
pub use architecture_witnesses::{
    bootstrap_architecture_witnesses, ArchitectureWitnessBundle, ArchitectureWitnessSnapshot,
};
pub use arithmetic::{
    bootstrap_arithmetic_morphisms, divisor_range, factor_pairs, has_digit, invert_int_value_dict,
    modulo_table_lines, prime_factors_legacy as arithmetic_prime_factors, prime_repeat_legacy,
    prime_repeat_pairs as arithmetic_prime_repeat_pairs, ArithmeticMorphismBundle,
    ArithmeticSnapshot,
};
pub use category::{
    bootstrap_category_theory, CategoryMorphismSpec, CategoryObjectSpec, CategorySpec,
    CategoryTheoryBundle, FunctorSpec, NaturalTransformationSpec, PYTHON_CATEGORY_THEORY_SNAPSHOT,
};
pub use column_selection::{
    bootstrap_column_selection, ColumnBucketKey, ColumnBucketSnapshot, ColumnSelectionBundle,
    ColumnSelectionSnapshot, COLUMN_BUCKET_NAMES, COLUMN_BUCKET_VALUES,
};
pub use combi_join::{
    bootstrap_combi_join, prepare_table_join, remove_number_from_cell, remove_one_number,
    rows_of_combi_from_relation, table_join, KombiJoinBundle, KombiJoinSnapshot, KombiJoinSpec,
    KombiSubTable,
};
pub use completion_nested::{
    bootstrap_nested_completion_morphisms, candidates_for_situation,
    classify_nested_completion_context, match_text_alx, nested_completion_candidates,
    prompt_document_for_nested_text, word_options_for_nested, ComplSitua,
    NestedCompletionCandidate, NestedCompletionContext, NestedCompletionMorphismBundle,
    NestedCompletionOptions, NestedCompletionRuntimeSnapshot, NestedCompletionRuntimeView,
    NestedCompletionSnapshot, HUNDERT,
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
pub use concat_csv::{
    bootstrap_concat_csv, combine_dicts,
    convert_fractions_to_dict_of_num_to_pairs_of_mul_of_int_and_fraction,
    convert_set_of_pairs_to_dict_of_num_to_pairs_div,
    convert_set_of_pairs_to_dict_of_num_to_pairs_mul, normalize_fraction, rational_div,
    rational_mul, read_concat_csv_tabelle_dazu_colchange, ConcatCsvBundle, ConcatCsvSnapshot,
    ConcatCsvSpec, ConcatCsvSpecSnapshot, FractionPair, FractionPairMap,
};
pub use console_io::{
    bootstrap_console_io_morphisms, chunks as console_chunks, cli_output_text, debug_pair_text,
    debug_value_text, doc_path, get_text_wrap_things, reta_prompt_help_text_from_markdown,
    strip_markdown_anchors, unique_strings_everseen, ConsoleIOMorphismBundle, ConsoleIOSnapshot,
    DefaultOrderedDictSnapshot, TextWrapRuntimeSnapshot as ConsoleTextWrapRuntimeSnapshot,
};
pub use dataflow::{
    bootstrap_execution_network, deterministic_reduce, execute_tasks_deterministically,
    execute_tasks_threaded_ordered,
    DataflowDiscipline, ExecutionNetworkBundle, ExecutionNetworkConfig, ExecutionResult,
    ExecutionRunResult, ExecutionTask, FifoTaskQueue, FullDuplexChannel, HalfDuplexChannel,
    LifoTaskStack, PriorityTaskQueue, ResourceSemaphore, EXECUTION_NETWORK_SNAPSHOT,
};
pub use execution_network::{
    bootstrap_execution_network_bridge, execution_network_plan_for_indices,
    ExecutionNetworkBridgeBundle, ExecutionNetworkBridgeSnapshot, ExecutionNetworkGate,
    ExecutionNetworkPlan,
};
pub use facade::{
    bootstrap_architecture_runtime, ArchitectureRuntime, ArchitectureSnapshotRef,
    PromptArchitectureContext, RetaRunArchitecture, ARCHITECTURE_COUNTS_SNAPSHOT,
};
pub use generated_columns::{
    bootstrap_generated_columns, concat_prim_creativity_type, create_spalte_gestirn,
    default_generated_column_registry, ensure_generated_parameter_slot_free,
    equality_freedom_domination_type, generated_parameter_index, love_polygon_cell,
    mind_emotion_energy_matter_topology_type, GeneratedColumnRegistry,
    GeneratedColumnRegistrySnapshot, GeneratedColumnSpec, GeneratedColumnSpecSnapshot,
    GeneratedColumnsBundle,
};
pub use input_semantics::{
    bootstrap_input_semantics, InputBundle, InputBundleSnapshot,
    PromptVocabulary as InputPromptVocabulary,
    PromptVocabularyBuilder as InputPromptVocabularyBuilder,
    PromptVocabularySnapshot as InputPromptVocabularySnapshot,
};
pub use meta_columns::{
    bootstrap_meta_columns, find_all_brueche_and_their_combinations, gcd_i64, make_vorwort,
    meta_number_signature, spalte_fuer_gegen_innen_aussen_seitlich_prim,
    spalte_meta_konkret_abstrakt_is_ganzzahlig, switching_meta_pair, MetaColumnSpec,
    MetaColumnSpecSnapshot, MetaColumnsBundle, MetaColumnsSnapshot, MetaVorwort,
    PrimeCrossColumnClass, Rational,
};
pub use migration_control::{
    bootstrap_migration_control, ActivationTransactionSpec, ActivationUnitSpec,
    MigrationControlBundle, MigrationControlSnapshot, MigrationControlValidation,
    MigrationStepSpec, MigrationWaveSpec,
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
pub use package_integrity::{
    bootstrap_package_integrity, is_runtime_artifact, iter_manifest_files, normalise_path,
    PackageIntegrityBundle, PackageIntegritySnapshot, RepoManifest, RepoManifestSnapshot,
    IGNORED_DIR_NAMES, IGNORED_SUFFIXES, REQUIRED_SOURCE_PATHS,
};
pub use parallel_execution::{
    apply_parallel_environment_pairs, bootstrap_parallel_execution, chunk_items,
    detect_processor_core_counts, extract_parallel_config_from_argv, factor_pairs_in_processes,
    filter_numbers_in_processes, glue_parallel_row_chunks, moon_numbers_in_processes,
    normalise_parallel_mode, normalize_column_buckets_in_processes, positive_int,
    prime_factors_in_processes, ParallelExecutionBundle, ParallelExecutionConfig,
    ParallelExecutionConfigSnapshot, ParallelExecutionSnapshot, ParallelOperationResult,
    ParallelOperationSnapshot, ParallelRowsResult, ProcessorCoreCounts,
};
pub use parity_harness::{
    bootstrap_parity_harness, ParityCommandCase, ParityHarnessBundle, ParityHarnessSnapshot,
    ParityOracle, ParityProbePlan,
};
pub use parameter_runtime::{
    bootstrap_parameter_runtime, MainParameter, ParameterCommandSets, ParameterParseResult,
    ParameterRuntimeBundle, ParameterRuntimeSnapshot, ParameterToken, ParameterTokenKind,
};
pub use persistence::{
    bootstrap_persistence, stable_digest_text, AuditEventRecord, CacheEntryRecord,
    ExecutionRunRecord, PersistedRecord, PersistedSection, PersistenceBundle, PersistenceConfig,
    PersistenceConfigSnapshot, PersistenceSnapshot, PersistenceStore, SheafSnapshotRecord,
};
pub use presheaf::{LocalSection, Presheaf, PresheafBundle};
pub use program_workflow::{
    bootstrap_program_workflow, ProgramWorkflowBundle, ProgramWorkflowSnapshot, WorkflowStep,
    WorkflowTrace,
};
pub use prompt_execution::{
    another_oberes_maximum, bootstrap_prompt_execution, bruch_spalt, create_ranges_for_bruch_parts,
    fraction_range_management, get_dict_limited_by_key_list, greater_and_less_than_anchor,
    if_print_cmd_again, plan_prompt_execution, return_only_paras_as_list,
    split_reta_argv_like_python, vorher_von_ausschnitt_oder_zaehlung, BruchPart,
    FractionRangeManagementResult, PromptExecutionBundle, PromptExecutionPlan,
    PromptExecutionSnapshot,
};
pub use prompt_interaction::{
    bootstrap_prompt_interaction, PromptInteractionBundle, PromptInteractionPlan,
    PromptInteractionSnapshot, PromptStorageDecision,
};
pub use prompt_language::{
    bootstrap_prompt_language, custom_split, custom_split2, is_15_or_16_command, is_reta_parameter,
    verkuerze_dict, FractionOrIntegerCheck, PromptLanguageBundle, PromptLanguageSnapshot,
    PromptModus,
};
pub use prompt_preparation::{
    bootstrap_prompt_preparation, prepare_large_output, regex_replace, rotate_where_reta_command,
    simple_pattern_match, vorher_von_ausschnitt_or_zaehlung, PreparedPromptOutput,
    PromptPreparationBundle, PromptPreparationSnapshot,
};
pub use prompt_runtime::{
    bootstrap_prompt_runtime, build_main_parameter_commands, prime_command_predicate,
    PromptProgramView, PromptRuntimeBuilder, PromptRuntimeBundle, PromptRuntimeSnapshot,
    PromptRuntimeValidation, PromptTablesView, PromptVocabulary, PromptVocabularySnapshot,
};
pub use prompt_session::{
    bootstrap_prompt_session, split_command_words, split_prompt_text, PromptLoopSetup,
    PromptLoopSetupSnapshot, PromptSessionBundle, PromptSessionSnapshot, PromptStoreResult,
    PromptTextState,
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
pub use runtime_compat::{
    bootstrap_runtime_compat, NPmEnum, RuntimeCompatBundle, RuntimeCompatSnapshot,
};
pub use runtime_switch::{
    bootstrap_runtime_switch, extract_architecture_switch_from_argv, ArchitectureSwitchConfig,
    ArchitectureSwitchMode, ArchitectureSwitchSnapshot, RuntimeSwitchBundle,
    RuntimeSwitchBundleSnapshot, SwitchGateDecision,
};
pub use schema::{
    bootstrap_schema, AliasGroup, ParameterMatrixEntry, RetaContextSchema,
    RetaContextSchemaSnapshot,
};
pub use semantics_builder::{
    bootstrap_semantics_builder, ParameterSemanticsBuildResult, ParameterSemanticsBuildSnapshot,
    ParameterSemanticsBuilder, SemanticsBuilderBundle,
};
pub use sheaf::{GluedSection, Sheaf, SheafBundle};
pub use shadow_pipeline::{
    bootstrap_shadow_pipeline, diff_shadow_lines, evaluate_shadow_prompt_commit,
    evaluate_shadow_table_commit, prepare_shadow_table, ShadowCliPlan, ShadowCommitDecision,
    ShadowCommitPolicy, ShadowDiffSummary, ShadowPipelineBundle, ShadowPipelineSnapshot,
    ShadowPromptCommitDecision, ShadowPromptCommitPolicy, ShadowPromptInput,
    ShadowPromptLegacyCommand, ShadowPromptReport, ShadowTableInput, ShadowTableReport,
};
pub use split_i18n::{
    build_split_i18n_proxy, SplitI18nProxy, SplitI18nProxySnapshot, DEFAULT_MODULE_NAMES,
};
pub use table_adapters::{
    bootstrap_table_adapters, ConcatAdapter, PrepareAdapter, TableAdaptersBundle,
    TableAdaptersSnapshot,
};
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
