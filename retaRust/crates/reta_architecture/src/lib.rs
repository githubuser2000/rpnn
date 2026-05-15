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
pub mod csv_catalog;
pub mod dataflow;
pub mod execution_network;
pub mod facade;
pub mod generated_columns;
pub mod html_class_catalog;
pub mod input_semantics;
pub mod meta_columns;
pub mod migration_control;
pub mod morphism;
pub mod number_theory;
pub mod output_semantics;
pub mod output_syntax;
pub mod package_integrity;
pub mod parallel_execution;
pub mod parameter_matrix;
pub mod parameter_runtime;
pub mod parity_harness;
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
pub mod shadow_pipeline;
pub mod sheaf;
pub mod split_i18n;
pub mod table_adapters;
pub mod table_generation;
pub mod table_materialization;
pub mod table_output;
pub mod table_preparation;
pub mod table_runtime;
pub mod table_state;
pub mod table_view;
pub mod table_view_html_attributes;
pub mod table_view_cell_styles;
pub mod table_view_layout;
pub mod table_view_numbering;
pub mod table_view_output;
pub mod table_view_output_parity;
pub mod table_view_row_styles;
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
    bootstrap_combi_join, prepare_table_join, read_kombi_csv_by_name, remove_number_from_cell,
    remove_one_number, rows_of_combi_from_relation, table_join, KombiJoinBundle, KombiJoinSnapshot,
    KombiJoinSpec, KombiSubTable,
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
    rational_mul, read_concat_csv_by_name, read_concat_csv_tabelle_dazu_colchange, ConcatCsvBundle,
    ConcatCsvSnapshot, ConcatCsvSpec, ConcatCsvSpecSnapshot, FractionPair, FractionPairMap,
};
pub use console_io::{
    bootstrap_console_io_morphisms, chunks as console_chunks, cli_output_text, debug_pair_text,
    debug_value_text, doc_path, get_text_wrap_things, reta_prompt_help_text_from_markdown,
    strip_markdown_anchors, unique_strings_everseen, ConsoleIOMorphismBundle, ConsoleIOSnapshot,
    DefaultOrderedDictSnapshot, TextWrapRuntimeSnapshot as ConsoleTextWrapRuntimeSnapshot,
};
pub use csv_catalog::{
    bootstrap_csv_catalog, csv_asset_by_name, csv_asset_count, csv_asset_records,
    csv_assets_by_kind, csv_assets_by_language, csv_catalog_owned, csv_cell_by_name,
    csv_language_variant_count, csv_rows_by_name, csv_text_by_name, csv_total_row_count,
    parse_csv_text, parse_csv_text_with_delimiter, select_csv_rows_one_based, CsvAsset,
    CsvAssetKind, CsvCatalogBundle, CsvCatalogSnapshot, CsvDelimiter, CsvLanguage, OwnedCsvAsset,
    OwnedCsvCatalogBundle, CSV_ASSETS,
};
pub use dataflow::{
    bootstrap_execution_network, deterministic_reduce, execute_tasks_deterministically,
    execute_tasks_threaded_ordered, DataflowDiscipline, ExecutionNetworkBundle,
    ExecutionNetworkConfig, ExecutionResult, ExecutionRunResult, ExecutionTask, FifoTaskQueue,
    FullDuplexChannel, HalfDuplexChannel, LifoTaskStack, PriorityTaskQueue, ResourceSemaphore,
    EXECUTION_NETWORK_SNAPSHOT,
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
pub use html_class_catalog::{
    bootstrap_html_class_catalog, html_class_catalog_snapshot, html_class_class_record_count,
    html_class_owned_records, html_class_record, html_class_record_count,
    html_class_records_for_column, html_class_text_for_column_row, html_class_text_record_count,
    html_class_unique_column_count, HtmlClassCatalogBundle, HtmlClassCatalogSnapshot,
    HtmlClassRecord, OwnedHtmlClassRecord, HTML_CLASS_RECORDS,
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
pub use morphism::{
    bootstrap_semantic_morphisms, morphism_snapshot_terms, AliasMorphisms, MorphismBundle,
    MorphismBundleSnapshot, MorphismEdge, MorphismGraph, MorphismKind, PromptMorphisms,
    RangeMorphisms, RendererMorphisms,
};
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
pub use parameter_matrix::{
    canonical_pair_for_aliases, columns_for_alias_pair, integer_column_projection_count,
    parameter_matrix_entries, parameter_matrix_seed_count, ParameterMatrixSeed,
    PARAMETER_MATRIX_SEEDS,
};
pub use parameter_runtime::{
    bootstrap_parameter_runtime, MainParameter, ParameterCommandSets, ParameterParseResult,
    ParameterRuntimeBundle, ParameterRuntimeSnapshot, ParameterToken, ParameterTokenKind,
};
pub use parity_harness::{
    bootstrap_parity_harness, ParityCommandCase, ParityHarnessBundle, ParityHarnessSnapshot,
    ParityOracle, ParityProbePlan,
};
pub use persistence::{
    bootstrap_persistence, stable_digest_text, AuditEventRecord, CacheEntryRecord,
    ExecutionRunRecord, PersistedRecord, PersistedSection, PersistenceBundle, PersistenceConfig,
    PersistenceConfigSnapshot, PersistenceSnapshot, PersistenceStore, SheafSnapshotRecord,
};
pub use presheaf::{
    bootstrap_presheaves, LocalSection, LocalSectionSnapshot, Presheaf, PresheafBundle,
    PresheafBundleSnapshot, PresheafSnapshot,
};
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
pub use shadow_pipeline::{
    bootstrap_shadow_pipeline, diff_shadow_lines, evaluate_shadow_prompt_commit,
    evaluate_shadow_table_commit, evaluate_shadow_table_view_output_commit, prepare_shadow_table,
    ShadowCliPlan, ShadowCommitDecision, ShadowCommitPolicy, ShadowDiffSummary,
    ShadowPipelineBundle, ShadowPipelineSnapshot, ShadowPromptCommitDecision,
    ShadowPromptCommitPolicy, ShadowPromptInput, ShadowPromptLegacyCommand, ShadowPromptReport,
    ShadowTableInput, ShadowTableReport, ShadowTableViewOutputCommitDecision,
    ShadowTableViewOutputCommitPolicy, ShadowTableViewOutputReport,
};
pub use sheaf::{
    bootstrap_sheaves, ColumnParameterMeta, GeneratedColumnsSheaf, GeneratedColumnsSheafSnapshot,
    GluedSection, HtmlReferenceSheaf, ParameterSemanticsSheaf, ParameterSemanticsSheafSnapshot,
    Sheaf, SheafBundle, SheafBundleSnapshot, TableOutputSection, TableOutputSheaf,
};
pub use split_i18n::{
    build_split_i18n_proxy, SplitI18nProxy, SplitI18nProxySnapshot, DEFAULT_MODULE_NAMES,
};
pub use table_adapters::{
    bootstrap_table_adapters, ConcatAdapter, PrepareAdapter, TableAdaptersBundle,
    TableAdaptersSnapshot,
};
pub use table_generation::{
    bootstrap_table_generation, csv_asset_names_for_bucket_state, TableGenerationBundle,
    TableGenerationBundleSnapshot, TableGenerationPlan, TableGenerationResult,
    TableGenerationResultSnapshot,
};
pub use table_materialization::{
    asset_name_for_language, asset_names_for_symbolic_bucket, bootstrap_table_materialization,
    csv_kind_for_asset, materialize_cli_args, materialize_csv_projection,
    materialize_generation_plan, materialize_kontinuum_m_smoke,
    materialize_symbolic_bucket_sections, numeric_selectors_from_symbols,
    ordered_columns_for_projection, plan_rows_to_source_indices, CsvProjectionRequest,
    MaterializedCsvCell, MaterializedCsvRow, MaterializedCsvSection, SymbolicBucketMaterialization,
    TableMaterializationBundle, TableMaterializationConfig, TableMaterializationReport,
    TableMaterializationSnapshot, VirtualColumnMaterialization,
};
pub use table_output::{
    bootstrap_table_output, colorize, determine_row_width, max_cell_text_len, only_that_columns_fn,
    render_prepared_table, BreakoutReason, TableOutputBundle, TableOutputBundleSnapshot,
    TableOutputConfig, TableRenderResult,
};
pub use table_view::{
    bootstrap_table_view, build_materialized_table_view, continuum_m_table_view_smoke,
    render_table_view_lines, view_for_cli_args as table_view_for_cli_args, virtual_column_value,
    MaterializedTableCellSource, MaterializedTableView, MaterializedTableViewCell,
    MaterializedTableViewConfig, MaterializedTableViewRow, TableViewBundle, TableViewSnapshot,
    VirtualColumnDisplayPolicy,
};
pub use table_view_html_attributes::{
    bootstrap_table_view_html_attributes, continuum_m_html_attribute_smoke,
    find_html_record_by_row_text, html_attribute_for_cell, html_attribute_report_for_rows,
    html_attribute_rows_for_view_rows, html_escape_attribute, render_html_cell_with_attribute,
    render_html_table_with_attributes, TableViewHtmlAttributeBundle, TableViewHtmlAttributeConfig,
    TableViewHtmlAttributePolicy, TableViewHtmlAttributeReport, TableViewHtmlAttributeRow,
    TableViewHtmlAttributeSnapshot, TableViewHtmlCellAttribute,
};

pub use table_view_cell_styles::{
    bootstrap_table_view_cell_styles, cell_style_for_output_value, cell_style_for_row_cell,
    cell_style_report_for_rows, continuum_m_cell_style_smoke, styled_begin_cell_for_output_value,
    styled_end_cell_for_mode, TableViewCellStyle, TableViewCellStyleBundle,
    TableViewCellStyleConfig, TableViewCellStylePolicy, TableViewCellStyleReport,
    TableViewCellStyleSnapshot,
};
pub use table_view_layout::{
    bootstrap_table_view_layout, column_pages_for_widths, display_width, effective_column_widths,
    layout_materialized_rows, layout_smoke_report, layout_value_rows, measure_column_widths,
    pad_to_width, TableViewColumnPage, TableViewLayoutBundle, TableViewLayoutConfig,
    TableViewLayoutReport, TableViewLayoutSnapshot,
};
pub use table_view_numbering::{
    bootstrap_table_view_numbering, legacy_zaehlung_for_row, legacy_zaehlung_map,
    numbering_projection_for_source_row, numbering_report_for_rows, numbering_smoke_report,
    numbering_values_for_source_row, TableViewNumberingBundle, TableViewNumberingCell,
    TableViewNumberingConfig, TableViewNumberingMode, TableViewNumberingProjection,
    TableViewNumberingReport, TableViewNumberingSnapshot,
};
pub use table_view_output::{
    bbcode_escape_cell, bootstrap_table_view_output, continuum_m_table_view_output_smoke,
    csv_escape_cell, expand_row_to_value_lines, filtered_output_rows, html_escape_cell,
    compose_html_td_open_tags, data_cell_for_output_value, html_begin_cell_for_output_value,
    html_cell_style_composition_counts, markdown_escape_cell, output_flags_smoke,
    output_layout_smoke, output_prefix_column_count, parse_table_view_output_cli_options,
    render_materialized_table_view, render_table_view_for_cli_args, render_table_view_rows_as_mode,
    rendered_row_value_lines, row_values, row_values_with_options, shell_layout_report_for_rows,
    wrap_output_cell,
    TableViewOutputBundle, TableViewOutputCliOptions, TableViewOutputConfig, TableViewOutputReport,
    TableViewOutputSnapshot,
};

pub use table_view_output_parity::{
    bootstrap_table_view_output_parity, canonicalize_cell,
    compare_output_lines as compare_table_view_output_lines, compare_table_view_output_to_legacy,
    normalize_output_lines, parse_line_as_cells, parse_markup_document_rows, semantic_rows_from_lines,
    strip_ansi_escape_sequences, NormalizedOutputLine, NormalizedOutputReport,
    TableViewOutputParityBundle, TableViewOutputParityConfig, TableViewOutputParityReport,
    TableViewOutputParitySnapshot,
};
pub use table_view_row_styles::{
    bootstrap_table_view_row_styles, continuum_m_row_style_smoke, row_style_for_row,
    row_style_for_source_row, row_style_report_for_rows, styled_begin_row_for_row,
    TableViewRowStyle, TableViewRowStyleBundle, TableViewRowStyleConfig, TableViewRowStylePolicy,
    TableViewRowStyleReport, TableViewRowStyleSnapshot,
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
