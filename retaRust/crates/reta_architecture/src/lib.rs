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
pub mod table_view_activation_transaction;
pub mod table_view_activation_journal;
pub mod table_view_activation_replay;
pub mod table_view_activation_ledger;
pub mod table_view_activation_store;
pub mod table_view_activation_persistence;
pub mod table_view_activation_file;
pub mod table_view_activation_recovery;
pub mod table_view_activation_readiness;
// Stable root-level recovery/readiness exports are intentionally kept for reta/rreta binaries.
pub mod table_view_cell_styles;
pub mod table_view_commit_audit;
pub mod table_view_html_attributes;
pub mod table_view_layout;
pub mod table_view_numbering;
pub mod table_view_output;
pub mod table_view_output_parity;
pub mod table_view_row_styles;
pub mod table_view_shell_styles;
pub mod table_view_virtual_columns;
pub mod table_view_virtual_parity;
pub mod table_wrapping;
pub mod tag_schema;
pub mod topology;
pub mod universal;

pub use architecture_activation::{
    ArchitectureActivationBundle, ArchitectureActivationSnapshot, bootstrap_architecture_activation,
};
pub use architecture_boundaries::{
    ArchitectureBoundariesBundle, ArchitectureBoundariesSnapshot, bootstrap_architecture_boundaries,
};
pub use architecture_coherence::{
    ArchitectureCoherenceBundle, ArchitectureCoherenceSnapshot, bootstrap_architecture_coherence,
};
pub use architecture_contracts::{
    ArchitectureContractsBundle, ArchitectureContractsSnapshot, bootstrap_architecture_contracts,
};
pub use architecture_impact::{
    ArchitectureImpactBundle, ArchitectureImpactSnapshot, bootstrap_architecture_impact,
};
pub use architecture_map::{
    ArchitectureMapBundle, ArchitectureMapSnapshot, bootstrap_architecture_map,
};
pub use architecture_migration::{
    ArchitectureMigrationBundle, ArchitectureMigrationSnapshot, bootstrap_architecture_migration,
};
pub use architecture_progress::{
    ArchitectureProgressBundle, ArchitectureProgressSnapshot, bootstrap_architecture_progress,
};
pub use architecture_rehearsal::{
    ArchitectureRehearsalBundle, ArchitectureRehearsalSnapshot, bootstrap_architecture_rehearsal,
};
pub use architecture_traces::{
    ArchitectureTraceBundle, ArchitectureTraceSnapshot, bootstrap_architecture_traces,
};
pub use architecture_validation::{
    ArchitectureValidationBundle, ArchitectureValidationSnapshot, bootstrap_architecture_validation,
};
pub use architecture_witnesses::{
    ArchitectureWitnessBundle, ArchitectureWitnessSnapshot, bootstrap_architecture_witnesses,
};
pub use arithmetic::{
    ArithmeticMorphismBundle, ArithmeticSnapshot, bootstrap_arithmetic_morphisms, divisor_range,
    factor_pairs, has_digit, invert_int_value_dict, modulo_table_lines,
    prime_factors_legacy as arithmetic_prime_factors, prime_repeat_legacy,
    prime_repeat_pairs as arithmetic_prime_repeat_pairs,
};
pub use category::{
    CategoryMorphismSpec, CategoryObjectSpec, CategorySpec, CategoryTheoryBundle, FunctorSpec,
    NaturalTransformationSpec, PYTHON_CATEGORY_THEORY_SNAPSHOT, bootstrap_category_theory,
};
pub use column_selection::{
    COLUMN_BUCKET_NAMES, COLUMN_BUCKET_VALUES, ColumnBucketKey, ColumnBucketSnapshot,
    ColumnSelectionBundle, ColumnSelectionSnapshot, bootstrap_column_selection,
};
pub use combi_join::{
    KombiJoinBundle, KombiJoinSnapshot, KombiJoinSpec, KombiSubTable, bootstrap_combi_join,
    prepare_table_join, read_kombi_csv_by_name, remove_number_from_cell, remove_one_number,
    rows_of_combi_from_relation, table_join,
};
pub use completion_nested::{
    ComplSitua, HUNDERT, NestedCompletionCandidate, NestedCompletionContext,
    NestedCompletionMorphismBundle, NestedCompletionOptions, NestedCompletionRuntimeSnapshot,
    NestedCompletionRuntimeView, NestedCompletionSnapshot, bootstrap_nested_completion_morphisms,
    candidates_for_situation, classify_nested_completion_context, match_text_alx,
    nested_completion_candidates, prompt_document_for_nested_text, word_options_for_nested,
};
pub use completion_runtime::{
    CompletionRuntimeBundle, CompletionRuntimeSnapshot, CompletionSortKey,
    bootstrap_completion_runtime, sort_completion_key,
};
pub use completion_word::{
    CompletionCandidate, PromptDocument, WordCompletionMorphismBundle, WordCompletionOptions,
    WordCompletionSnapshot, bootstrap_word_completion_morphisms, iter_word_completions,
    resolve_words, word_before_cursor, word_completion_matches,
};
pub use concat_csv::{
    ConcatCsvBundle, ConcatCsvSnapshot, ConcatCsvSpec, ConcatCsvSpecSnapshot, FractionPair,
    FractionPairMap, bootstrap_concat_csv, combine_dicts,
    convert_fractions_to_dict_of_num_to_pairs_of_mul_of_int_and_fraction,
    convert_set_of_pairs_to_dict_of_num_to_pairs_div,
    convert_set_of_pairs_to_dict_of_num_to_pairs_mul, normalize_fraction, rational_div,
    rational_mul, read_concat_csv_by_name, read_concat_csv_tabelle_dazu_colchange,
};
pub use console_io::{
    ConsoleIOMorphismBundle, ConsoleIOSnapshot, DefaultOrderedDictSnapshot,
    TextWrapRuntimeSnapshot as ConsoleTextWrapRuntimeSnapshot, bootstrap_console_io_morphisms,
    chunks as console_chunks, cli_output_text, debug_pair_text, debug_value_text, doc_path,
    get_text_wrap_things, reta_prompt_help_text_from_markdown, strip_markdown_anchors,
    unique_strings_everseen,
};
pub use csv_catalog::{
    CSV_ASSETS, CsvAsset, CsvAssetKind, CsvCatalogBundle, CsvCatalogSnapshot, CsvDelimiter,
    CsvLanguage, OwnedCsvAsset, OwnedCsvCatalogBundle, bootstrap_csv_catalog, csv_asset_by_name,
    csv_asset_count, csv_asset_records, csv_assets_by_kind, csv_assets_by_language,
    csv_catalog_owned, csv_cell_by_name, csv_language_variant_count, csv_rows_by_name,
    csv_text_by_name, csv_total_row_count, parse_csv_text, parse_csv_text_with_delimiter,
    select_csv_rows_one_based,
};
pub use dataflow::{
    DataflowDiscipline, EXECUTION_NETWORK_SNAPSHOT, ExecutionNetworkBundle, ExecutionNetworkConfig,
    ExecutionResult, ExecutionRunResult, ExecutionTask, FifoTaskQueue, FullDuplexChannel,
    HalfDuplexChannel, LifoTaskStack, PriorityTaskQueue, ResourceSemaphore,
    bootstrap_execution_network, deterministic_reduce, execute_tasks_deterministically,
    execute_tasks_threaded_ordered,
};
pub use execution_network::{
    ExecutionNetworkBridgeBundle, ExecutionNetworkBridgeSnapshot, ExecutionNetworkGate,
    ExecutionNetworkPlan, bootstrap_execution_network_bridge, execution_network_plan_for_indices,
};
pub use facade::{
    ARCHITECTURE_COUNTS_SNAPSHOT, ArchitectureRuntime, ArchitectureSnapshotRef,
    PromptArchitectureContext, RetaRunArchitecture, bootstrap_architecture_runtime,
};
pub use generated_columns::{
    GeneratedColumnRegistry, GeneratedColumnRegistrySnapshot, GeneratedColumnSpec,
    GeneratedColumnSpecSnapshot, GeneratedColumnsBundle, bootstrap_generated_columns,
    concat_prim_creativity_type, create_spalte_gestirn, default_generated_column_registry,
    ensure_generated_parameter_slot_free, equality_freedom_domination_type,
    generated_parameter_index, love_polygon_cell, mind_emotion_energy_matter_topology_type,
};
pub use html_class_catalog::{
    HTML_CLASS_RECORDS, HtmlClassCatalogBundle, HtmlClassCatalogSnapshot, HtmlClassRecord,
    OwnedHtmlClassRecord, bootstrap_html_class_catalog, html_class_catalog_snapshot,
    html_class_class_record_count, html_class_owned_records, html_class_record,
    html_class_record_count, html_class_records_for_column, html_class_text_for_column_row,
    html_class_text_record_count, html_class_unique_column_count,
};
pub use input_semantics::{
    InputBundle, InputBundleSnapshot, PromptVocabulary as InputPromptVocabulary,
    PromptVocabularyBuilder as InputPromptVocabularyBuilder,
    PromptVocabularySnapshot as InputPromptVocabularySnapshot, bootstrap_input_semantics,
};
pub use meta_columns::{
    MetaColumnSpec, MetaColumnSpecSnapshot, MetaColumnsBundle, MetaColumnsSnapshot, MetaVorwort,
    PrimeCrossColumnClass, Rational, bootstrap_meta_columns,
    find_all_brueche_and_their_combinations, gcd_i64, make_vorwort, meta_number_signature,
    spalte_fuer_gegen_innen_aussen_seitlich_prim, spalte_meta_konkret_abstrakt_is_ganzzahlig,
    switching_meta_pair,
};
pub use migration_control::{
    ActivationTransactionSpec, ActivationUnitSpec, MigrationControlBundle,
    MigrationControlSnapshot, MigrationControlValidation, MigrationStepSpec, MigrationWaveSpec,
    bootstrap_migration_control,
};
pub use morphism::{
    AliasMorphisms, MorphismBundle, MorphismBundleSnapshot, MorphismEdge, MorphismGraph,
    MorphismKind, PromptMorphisms, RangeMorphisms, RendererMorphisms, bootstrap_semantic_morphisms,
    morphism_snapshot_terms,
};
pub use number_theory::{
    NumberTheoryBundle, NumberTheorySnapshot, bootstrap_number_theory,
    could_be_prime_number_primzahlkreuz, could_be_prime_number_primzahlkreuz_fuer_aussen,
    could_be_prime_number_primzahlkreuz_fuer_innen, divisor_generator, is_prime_multiple,
    moon_number, prime_creativity, prime_factors, prime_multiple, prime_repeat,
};
pub use output_semantics::{
    OutputConfig, OutputModeApplication, OutputSemanticsSnapshot, RetaOutputSemantics,
    bootstrap_output_semantics,
};
pub use output_syntax::{
    OutputMode, OutputModeSpec, OutputSyntaxBundle, OutputSyntaxSnapshot, SyntaxMarkup,
    bootstrap_output_syntax, colored_begin_col, generate_cell_begin, output_syntax_snapshot,
};
pub use package_integrity::{
    IGNORED_DIR_NAMES, IGNORED_SUFFIXES, PackageIntegrityBundle, PackageIntegritySnapshot,
    REQUIRED_SOURCE_PATHS, RepoManifest, RepoManifestSnapshot, bootstrap_package_integrity,
    is_runtime_artifact, iter_manifest_files, normalise_path,
};
pub use parallel_execution::{
    ParallelExecutionBundle, ParallelExecutionConfig, ParallelExecutionConfigSnapshot,
    ParallelExecutionSnapshot, ParallelOperationResult, ParallelOperationSnapshot,
    ParallelRowsResult, ProcessorCoreCounts, apply_parallel_environment_pairs,
    bootstrap_parallel_execution, chunk_items, detect_processor_core_counts,
    extract_parallel_config_from_argv, factor_pairs_in_processes, filter_numbers_in_processes,
    glue_parallel_row_chunks, moon_numbers_in_processes, normalise_parallel_mode,
    normalize_column_buckets_in_processes, positive_int, prime_factors_in_processes,
};
pub use parameter_matrix::{
    PARAMETER_MATRIX_SEEDS, ParameterMatrixSeed, canonical_pair_for_aliases,
    columns_for_alias_pair, integer_column_projection_count, parameter_matrix_entries,
    parameter_matrix_seed_count,
};
pub use parameter_runtime::{
    MainParameter, ParameterCommandSets, ParameterParseResult, ParameterRuntimeBundle,
    ParameterRuntimeSnapshot, ParameterToken, ParameterTokenKind, bootstrap_parameter_runtime,
};
pub use parity_harness::{
    ParityCommandCase, ParityHarnessBundle, ParityHarnessSnapshot, ParityOracle, ParityProbePlan,
    bootstrap_parity_harness,
};
pub use persistence::{
    AuditEventRecord, CacheEntryRecord, ExecutionRunRecord, PersistedRecord, PersistedSection,
    PersistenceBundle, PersistenceConfig, PersistenceConfigSnapshot, PersistenceSnapshot,
    PersistenceStore, SheafSnapshotRecord, bootstrap_persistence, stable_digest_text,
};
pub use presheaf::{
    LocalSection, LocalSectionSnapshot, Presheaf, PresheafBundle, PresheafBundleSnapshot,
    PresheafSnapshot, bootstrap_presheaves,
};
pub use program_workflow::{
    ProgramWorkflowBundle, ProgramWorkflowSnapshot, WorkflowStep, WorkflowTrace,
    bootstrap_program_workflow,
};
pub use prompt_execution::{
    BruchPart, FractionRangeManagementResult, PromptExecutionBundle, PromptExecutionPlan,
    PromptExecutionSnapshot, another_oberes_maximum, bootstrap_prompt_execution, bruch_spalt,
    create_ranges_for_bruch_parts, fraction_range_management, get_dict_limited_by_key_list,
    greater_and_less_than_anchor, if_print_cmd_again, plan_prompt_execution,
    return_only_paras_as_list, split_reta_argv_like_python, vorher_von_ausschnitt_oder_zaehlung,
};
pub use prompt_interaction::{
    PromptInteractionBundle, PromptInteractionPlan, PromptInteractionSnapshot,
    PromptStorageDecision, bootstrap_prompt_interaction,
};
pub use prompt_language::{
    FractionOrIntegerCheck, PromptLanguageBundle, PromptLanguageSnapshot, PromptModus,
    bootstrap_prompt_language, custom_split, custom_split2, is_15_or_16_command, is_reta_parameter,
    verkuerze_dict,
};
pub use prompt_preparation::{
    PreparedPromptOutput, PromptPreparationBundle, PromptPreparationSnapshot,
    bootstrap_prompt_preparation, prepare_large_output, regex_replace, rotate_where_reta_command,
    simple_pattern_match, vorher_von_ausschnitt_or_zaehlung,
};
pub use prompt_runtime::{
    PromptProgramView, PromptRuntimeBuilder, PromptRuntimeBundle, PromptRuntimeSnapshot,
    PromptRuntimeValidation, PromptTablesView, PromptVocabulary, PromptVocabularySnapshot,
    bootstrap_prompt_runtime, build_main_parameter_commands, prime_command_predicate,
};
pub use prompt_session::{
    PromptLoopSetup, PromptLoopSetupSnapshot, PromptSessionBundle, PromptSessionSnapshot,
    PromptStoreResult, PromptTextState, bootstrap_prompt_session, split_command_words,
    split_prompt_text,
};
pub use row_filtering::{
    RowFilterContext, RowFilteringBundle, RowFilteringSnapshot, bootstrap_row_filtering,
    delete_doubles_in_sets, filter_original_lines, from_until, moon_sun_filter,
    parameters_cmd_with_some_bereich,
};
pub use row_ranges::{
    RowRangeMorphismBundle, RowRangeSyntax, bootstrap_row_range_morphisms,
    is_fraction_or_integer_range, is_fraction_range, is_fraction_range_token,
    is_integer_range_token, is_row_range, is_row_range_token, range_to_numbers,
    str_as_generator_to_set,
};
pub use runtime_compat::{
    NPmEnum, RuntimeCompatBundle, RuntimeCompatSnapshot, bootstrap_runtime_compat,
};
pub use runtime_switch::{
    ArchitectureSwitchConfig, ArchitectureSwitchMode, ArchitectureSwitchSnapshot,
    RuntimeSwitchBundle, RuntimeSwitchBundleSnapshot, SwitchGateDecision, bootstrap_runtime_switch,
    extract_architecture_switch_from_argv,
};
pub use schema::{
    AliasGroup, ParameterMatrixEntry, RetaContextSchema, RetaContextSchemaSnapshot,
    bootstrap_schema,
};
pub use semantics_builder::{
    ParameterSemanticsBuildResult, ParameterSemanticsBuildSnapshot, ParameterSemanticsBuilder,
    SemanticsBuilderBundle, bootstrap_semantics_builder,
};
pub use shadow_pipeline::{
    ShadowCliPlan, ShadowCommitDecision, ShadowCommitPolicy, ShadowDiffSummary,
    ShadowPipelineBundle, ShadowPipelineSnapshot, ShadowPromptCommitDecision,
    ShadowPromptCommitPolicy, ShadowPromptInput, ShadowPromptLegacyCommand, ShadowPromptReport,
    ShadowTableInput, ShadowTableReport, ShadowTableViewOutputCommitDecision,
    ShadowTableViewOutputCommitPolicy, ShadowTableViewOutputReport, bootstrap_shadow_pipeline,
    diff_shadow_lines, evaluate_shadow_prompt_commit, evaluate_shadow_table_commit,
    evaluate_shadow_table_view_output_commit, prepare_shadow_table,
};
pub use sheaf::{
    ColumnParameterMeta, GeneratedColumnsSheaf, GeneratedColumnsSheafSnapshot, GluedSection,
    HtmlReferenceSheaf, ParameterSemanticsSheaf, ParameterSemanticsSheafSnapshot, Sheaf,
    SheafBundle, SheafBundleSnapshot, TableOutputSection, TableOutputSheaf, bootstrap_sheaves,
};
pub use split_i18n::{
    DEFAULT_MODULE_NAMES, SplitI18nProxy, SplitI18nProxySnapshot, build_split_i18n_proxy,
};
pub use table_adapters::{
    ConcatAdapter, PrepareAdapter, TableAdaptersBundle, TableAdaptersSnapshot,
    bootstrap_table_adapters,
};
pub use table_generation::{
    TableGenerationBundle, TableGenerationBundleSnapshot, TableGenerationPlan,
    TableGenerationResult, TableGenerationResultSnapshot, bootstrap_table_generation,
    csv_asset_names_for_bucket_state,
};
pub use table_materialization::{
    CsvProjectionRequest, MaterializedCsvCell, MaterializedCsvRow, MaterializedCsvSection,
    SymbolicBucketMaterialization, TableMaterializationBundle, TableMaterializationConfig,
    TableMaterializationReport, TableMaterializationSnapshot, VirtualColumnMaterialization,
    asset_name_for_language, asset_names_for_symbolic_bucket, bootstrap_table_materialization,
    csv_kind_for_asset, materialize_cli_args, materialize_csv_projection,
    materialize_generation_plan, materialize_kontinuum_m_smoke,
    materialize_symbolic_bucket_sections, numeric_selectors_from_symbols,
    ordered_columns_for_projection, plan_rows_to_source_indices,
};
pub use table_output::{
    BreakoutReason, TableOutputBundle, TableOutputBundleSnapshot, TableOutputConfig,
    TableRenderResult, bootstrap_table_output, colorize, determine_row_width, max_cell_text_len,
    only_that_columns_fn, render_prepared_table,
};
pub use table_view_activation_transaction::{
    TableViewActivationLineSource, TableViewActivationTransactionBundle,
    TableViewActivationTransactionPolicy, TableViewActivationTransactionReport,
    TableViewActivationTransactionSnapshot, bootstrap_table_view_activation_transaction,
    continuum_m_activation_transaction_smoke, stable_line_checksum,
    table_view_activation_transaction, table_view_activation_transaction_for_cli_args,
};
pub use table_view_activation_journal::{
    TableViewActivationJournal, TableViewActivationJournalBundle, TableViewActivationJournalPolicy,
    TableViewActivationJournalRecord, TableViewActivationJournalReplayReport,
    TableViewActivationJournalSnapshot, activation_journal_for_cli_args,
    activation_journal_from_records, activation_journal_from_transactions,
    activation_journal_record_from_transaction, bootstrap_table_view_activation_journal,
    continuum_m_activation_journal_smoke, replay_activation_journal,
};
pub use table_view_activation_replay::{
    TableViewActivationReplayBundle, TableViewActivationReplayPolicy,
    TableViewActivationReplayReport, TableViewActivationReplaySnapshot,
    activation_replay_for_cli_args, activation_replay_from_journal,
    bootstrap_table_view_activation_replay, continuum_m_activation_replay_smoke,
};
pub use table_view_activation_ledger::{
    TableViewActivationLedger, TableViewActivationLedgerBundle,
    TableViewActivationLedgerEntry, TableViewActivationLedgerPolicy,
    TableViewActivationLedgerSnapshot, TableViewActivationLedgerValidation,
    activation_ledger_entry_chain_hash, activation_ledger_entry_from_record,
    activation_ledger_entry_record_hash, activation_ledger_entries_from_records,
    activation_ledger_for_cli_args, activation_ledger_from_journal,
    bootstrap_table_view_activation_ledger, continuum_m_activation_ledger_smoke,
    validate_activation_ledger_entries,
};
pub use table_view_activation_store::{
    TableViewActivationStore, TableViewActivationStoreBundle,
    TableViewActivationStoreParseReport, TableViewActivationStorePolicy,
    TableViewActivationStoreSnapshot, TableViewActivationStoreValidation,
    activation_store_for_cli_args, activation_store_from_journal_and_ledger,
    activation_store_text_from_journal_and_ledger, bootstrap_table_view_activation_store,
    continuum_m_activation_store_smoke, parse_activation_store_text,
};
pub use table_view_activation_persistence::{
    TableViewActivationPersistenceBundle, TableViewActivationPersistencePolicy,
    TableViewActivationPersistenceReport, TableViewActivationPersistenceSnapshot,
    activation_persistence_for_cli_args, bootstrap_table_view_activation_persistence,
    continuum_m_activation_persistence_smoke, persist_activation_store_to_persistence,
};
pub use table_view_activation_file::{
    TableViewActivationFileBundle, TableViewActivationFilePolicy,
    TableViewActivationFileReport, TableViewActivationFileSnapshot,
    activation_file_for_cli_args, activation_file_policy_from_cli_args,
    bootstrap_table_view_activation_file,
    continuum_m_activation_file_smoke, default_activation_store_file_path,
    read_activation_store_file, write_activation_store_file,
};
pub use self::table_view_activation_recovery::{
    TableViewActivationRecoveryBundle, TableViewActivationRecoveryPolicy,
    TableViewActivationRecoveryReport, TableViewActivationRecoverySnapshot,
    activation_recovery_for_cli_args, activation_recovery_policy_from_cli_args,
    bootstrap_table_view_activation_recovery, continuum_m_activation_recovery_smoke,
    read_activation_store_file_for_recovery,
};
pub use self::table_view_activation_readiness::{
    TableViewActivationReadinessBundle, TableViewActivationReadinessCheck,
    TableViewActivationReadinessPolicy, TableViewActivationReadinessReport,
    TableViewActivationReadinessSnapshot, activation_readiness_for_cli_args,
    activation_readiness_from_reports, bootstrap_table_view_activation_readiness,
    continuum_m_activation_readiness_smoke,
};
pub use table_view::{
    MaterializedTableCellSource, MaterializedTableView, MaterializedTableViewCell,
    MaterializedTableViewConfig, MaterializedTableViewRow, TableViewBundle, TableViewSnapshot,
    VirtualColumnDisplayPolicy, bootstrap_table_view, build_materialized_table_view,
    continuum_m_table_view_smoke, render_table_view_lines,
    view_for_cli_args as table_view_for_cli_args, virtual_column_value,
};
pub use table_view_commit_audit::{
    TableViewCommitAuditBundle, TableViewCommitAuditCheck, TableViewCommitAuditReport,
    TableViewCommitAuditSnapshot, audit_table_view_output_commit,
    audit_table_view_output_for_cli_args, bootstrap_table_view_commit_audit,
    continuum_m_commit_audit_smoke,
};
pub use table_view_html_attributes::{
    TableViewHtmlAttributeBundle, TableViewHtmlAttributeConfig, TableViewHtmlAttributePolicy,
    TableViewHtmlAttributeReport, TableViewHtmlAttributeRow, TableViewHtmlAttributeSnapshot,
    TableViewHtmlCellAttribute, bootstrap_table_view_html_attributes,
    continuum_m_html_attribute_smoke, find_html_record_by_row_text, html_attribute_for_cell,
    html_attribute_report_for_rows, html_attribute_rows_for_view_rows, html_escape_attribute,
    render_html_cell_with_attribute, render_html_table_with_attributes,
};

pub use table_view_cell_styles::{
    TableViewCellStyle, TableViewCellStyleBundle, TableViewCellStyleConfig,
    TableViewCellStylePolicy, TableViewCellStyleReport, TableViewCellStyleSnapshot,
    bootstrap_table_view_cell_styles, cell_style_for_output_value, cell_style_for_row_cell,
    cell_style_report_for_rows, continuum_m_cell_style_smoke, styled_begin_cell_for_output_value,
    styled_end_cell_for_mode,
};
pub use table_view_layout::{
    TableViewColumnPage, TableViewLayoutBundle, TableViewLayoutConfig, TableViewLayoutReport,
    TableViewLayoutSnapshot, bootstrap_table_view_layout, column_pages_for_widths, display_width,
    effective_column_widths, layout_materialized_rows, layout_smoke_report, layout_value_rows,
    measure_column_widths, pad_to_width,
};
pub use table_view_numbering::{
    TableViewNumberingBundle, TableViewNumberingCell, TableViewNumberingConfig,
    TableViewNumberingMode, TableViewNumberingProjection, TableViewNumberingReport,
    TableViewNumberingSnapshot, bootstrap_table_view_numbering, legacy_zaehlung_for_row,
    legacy_zaehlung_map, numbering_projection_for_source_row, numbering_report_for_rows,
    numbering_smoke_report, numbering_values_for_source_row,
};
pub use table_view_output::{
    TableViewOutputBundle, TableViewOutputCliOptions, TableViewOutputConfig, TableViewOutputReport,
    TableViewOutputSnapshot, bbcode_escape_cell, bootstrap_table_view_output,
    compose_html_td_open_tags, continuum_m_table_view_output_smoke, csv_escape_cell,
    data_cell_for_output_value, expand_row_to_value_lines, filtered_output_rows,
    html_begin_cell_for_output_value, html_cell_style_composition_counts, html_escape_cell,
    markdown_escape_cell, output_flags_smoke, output_layout_smoke, output_prefix_column_count,
    parse_table_view_output_cli_options, render_materialized_table_view,
    render_table_view_for_cli_args, render_table_view_rows_as_mode, rendered_row_value_lines,
    row_values, row_values_with_options, shell_layout_report_for_rows, wrap_output_cell,
};

pub use table_view_output_parity::{
    NormalizedOutputLine, NormalizedOutputReport, TableViewOutputParityBundle,
    TableViewOutputParityConfig, TableViewOutputParityReport, TableViewOutputParitySnapshot,
    bootstrap_table_view_output_parity, canonicalize_cell,
    compare_output_lines as compare_table_view_output_lines, compare_table_view_output_to_legacy,
    normalize_output_lines, parse_line_as_cells, parse_markup_document_rows,
    semantic_rows_from_lines, strip_ansi_escape_sequences,
};
pub use table_view_row_styles::{
    TableViewRowStyle, TableViewRowStyleBundle, TableViewRowStyleConfig, TableViewRowStylePolicy,
    TableViewRowStyleReport, TableViewRowStyleSnapshot, bootstrap_table_view_row_styles,
    continuum_m_row_style_smoke, row_style_for_row, row_style_for_source_row,
    row_style_report_for_rows, styled_begin_row_for_row,
};
pub use table_view_virtual_columns::{
    TableViewVirtualColumnBundle, TableViewVirtualColumnCliOptions, TableViewVirtualColumnConfig,
    TableViewVirtualColumnReport, TableViewVirtualColumnSnapshot,
    bootstrap_table_view_virtual_columns, continuum_m_virtual_column_policy_smoke,
    parse_table_view_virtual_column_cli_options, virtual_column_report_for_cli_args,
    virtual_column_report_from_view,
};
pub use table_view_virtual_parity::{
    DirectCellSignature, TableViewVirtualParityBundle, TableViewVirtualParityConfig,
    TableViewVirtualParityReport, TableViewVirtualParitySnapshot, VirtualCellSignature,
    bootstrap_table_view_virtual_parity, compare_virtual_column_policies_for_cli_args,
    continuum_m_virtual_parity_smoke, direct_cell_signatures, first_line_mismatch,
    first_signature_mismatch, virtual_cell_signature, virtual_cell_signatures,
};
pub use table_view_shell_styles::{
    TableViewShellStyle, TableViewShellStyleBundle, TableViewShellStyleConfig,
    TableViewShellStylePolicy, TableViewShellStyleReport, TableViewShellStyleSnapshot,
    bootstrap_table_view_shell_styles, colorize_shell_output_value, continuum_m_shell_style_smoke,
    shell_color_signature, shell_style_for_output_value, shell_style_report_for_rows,
};
pub use tag_schema::{
    KOMBI_TABLE_TAG_GROUPS, KOMBI_TABLE2_TAG_GROUPS, ORDINARY_TAG_GROUPS, TableTag, TagGroup,
    TagSchemaBundle, TagSchemaSnapshot, TagTableSelector, bootstrap_tag_schema,
    columns_for_tags_in_selector, groups_for_selector, kombi_table_tags_for_column,
    kombi_table2_tags_for_column, ordinary_columns_for_tags, ordinary_tags_for_column,
    reverse_map_for_selector,
};

pub use table_preparation::{
    DisplayLineSelection, GebrSpalten, KombiTablePreparationResult,
    KombiTablePreparationResultSnapshot, MainTablePreparationResult,
    MainTablePreparationResultSnapshot, OldNewTableMap, PreparedCell, PreparedRow,
    PreparedRowWithMap, PreparedTable, TablePreparationBundle, TablePreparationBundleSnapshot,
    TablePreparationContext, bootstrap_table_preparation, cell_work, prepare_output_table,
    prepare_row_cells, select_display_lines, tag_output_column,
};
pub use table_runtime::{
    TableRuntimeBundle, TableRuntimeBundleSnapshot, TableRuntimeState, TableRuntimeStateSnapshot,
    bootstrap_table_runtime,
};
pub use table_state::{
    GeneratedColumnSection, GeneratedColumnSectionSnapshot, TableDisplayState,
    TableDisplayStateSnapshot, TableStateBundle, TableStateBundleSnapshot, TableStateSections,
    TableStateSectionsSnapshot, bootstrap_table_state, highest_rows,
};
pub use table_wrapping::{
    TableWidthContext, TableWrappingBundle, TableWrappingBundleSnapshot, TextWrapRuntime,
    TextWrapRuntimeSnapshot, WrapType, alxwrap, bootstrap_table_wrapping, chunks,
    split_more_if_not_small, width_for_row_context, wrap_cell_text,
};
pub use topology::{ContextDimension, ContextSelection, RetaContextTopology};
pub use universal::{UniversalBundle, merge_parameter_dicts, normalize_column_buckets};
