use serde::{Deserialize, Serialize};

use crate::architecture_activation::{
    ArchitectureActivationBundle,
    bootstrap_architecture_activation as bootstrap_architecture_activation_impl,
};
use crate::architecture_boundaries::{
    ArchitectureBoundariesBundle,
    bootstrap_architecture_boundaries as bootstrap_architecture_boundaries_impl,
};
use crate::architecture_coherence::{
    ArchitectureCoherenceBundle,
    bootstrap_architecture_coherence as bootstrap_architecture_coherence_impl,
};
use crate::architecture_contracts::{
    ArchitectureContractsBundle,
    bootstrap_architecture_contracts as bootstrap_architecture_contracts_impl,
};
use crate::architecture_impact::{
    ArchitectureImpactBundle, bootstrap_architecture_impact as bootstrap_architecture_impact_impl,
};
use crate::architecture_map::{
    ArchitectureMapBundle, bootstrap_architecture_map as bootstrap_architecture_map_impl,
};
use crate::architecture_migration::{
    ArchitectureMigrationBundle,
    bootstrap_architecture_migration as bootstrap_architecture_migration_impl,
};
use crate::architecture_progress::{
    ArchitectureProgressBundle,
    bootstrap_architecture_progress as bootstrap_architecture_progress_impl,
};
use crate::architecture_rehearsal::{
    ArchitectureRehearsalBundle,
    bootstrap_architecture_rehearsal as bootstrap_architecture_rehearsal_impl,
};
use crate::architecture_traces::{
    ArchitectureTraceBundle, bootstrap_architecture_traces as bootstrap_architecture_traces_impl,
};
use crate::architecture_validation::{
    ArchitectureValidationBundle,
    bootstrap_architecture_validation as bootstrap_architecture_validation_impl,
};
use crate::architecture_witnesses::{
    ArchitectureWitnessBundle,
    bootstrap_architecture_witnesses as bootstrap_architecture_witnesses_impl,
};
use crate::arithmetic::{ArithmeticMorphismBundle, bootstrap_arithmetic_morphisms};
use crate::category::{
    CategoryTheoryBundle, bootstrap_category_theory as bootstrap_category_theory_impl,
};
use crate::column_selection::{
    ColumnSelectionBundle, bootstrap_column_selection as bootstrap_column_selection_impl,
};
use crate::combi_join::{KombiJoinBundle, bootstrap_combi_join as bootstrap_combi_join_impl};
use crate::completion_nested::{
    NestedCompletionMorphismBundle, bootstrap_nested_completion_morphisms,
};
use crate::completion_runtime::{
    CompletionRuntimeBundle, bootstrap_completion_runtime as bootstrap_completion_runtime_impl,
};
use crate::completion_word::{WordCompletionMorphismBundle, bootstrap_word_completion_morphisms};
use crate::concat_csv::{ConcatCsvBundle, bootstrap_concat_csv as bootstrap_concat_csv_impl};
use crate::console_io::{ConsoleIOMorphismBundle, bootstrap_console_io_morphisms};
use crate::csv_catalog::{
    CsvCatalogBundle, bootstrap_csv_catalog, csv_asset_count, csv_language_variant_count,
    csv_total_row_count,
};
use crate::dataflow::{
    ExecutionNetworkBundle, ExecutionTask,
    bootstrap_execution_network as bootstrap_execution_network_impl,
};
use crate::execution_network::{ExecutionNetworkBridgeBundle, bootstrap_execution_network_bridge};
use crate::generated_columns::{
    GeneratedColumnsBundle, bootstrap_generated_columns as bootstrap_generated_columns_impl,
};
use crate::html_class_catalog::{
    HtmlClassCatalogBundle, bootstrap_html_class_catalog, html_class_record_count,
    html_class_unique_column_count,
};
use crate::input_semantics::{InputBundle, bootstrap_input_semantics};
use crate::meta_columns::{
    MetaColumnsBundle, bootstrap_meta_columns as bootstrap_meta_columns_impl,
};
use crate::migration_control::{MigrationControlBundle, bootstrap_migration_control};
use crate::morphism::{
    MorphismBundle, MorphismEdge, MorphismGraph, MorphismKind, bootstrap_semantic_morphisms,
};
use crate::number_theory::{
    NumberTheoryBundle, bootstrap_number_theory as bootstrap_number_theory_impl,
};
use crate::output_semantics::{RetaOutputSemantics, bootstrap_output_semantics};
use crate::output_syntax::{
    OutputSyntaxBundle, bootstrap_output_syntax as bootstrap_output_syntax_impl,
};
use crate::package_integrity::{PackageIntegrityBundle, bootstrap_package_integrity};
use crate::parallel_execution::{
    ParallelExecutionBundle, bootstrap_parallel_execution as bootstrap_parallel_execution_impl,
};
use crate::parameter_matrix::{
    integer_column_projection_count, nonempty_bucket_projection_count, parameter_matrix_seed_count,
    symbolic_bucket_projection_count,
};
use crate::parameter_runtime::{
    ParameterRuntimeBundle, bootstrap_parameter_runtime as bootstrap_parameter_runtime_impl,
};
use crate::parity_harness::{ParityHarnessBundle, bootstrap_parity_harness};
use crate::persistence::{PersistenceBundle, bootstrap_persistence as bootstrap_persistence_impl};
use crate::presheaf::{PresheafBundle, bootstrap_presheaves};
use crate::program_workflow::{
    ProgramWorkflowBundle, bootstrap_program_workflow as bootstrap_program_workflow_impl,
};
use crate::prompt_execution::{
    PromptExecutionBundle, bootstrap_prompt_execution as bootstrap_prompt_execution_impl,
};
use crate::prompt_interaction::{
    PromptInteractionBundle, bootstrap_prompt_interaction as bootstrap_prompt_interaction_impl,
};
use crate::prompt_language::{
    PromptLanguageBundle, bootstrap_prompt_language as bootstrap_prompt_language_impl,
};
use crate::prompt_preparation::{
    PromptPreparationBundle, bootstrap_prompt_preparation as bootstrap_prompt_preparation_impl,
};
use crate::prompt_runtime::{
    PromptRuntimeBundle, bootstrap_prompt_runtime as bootstrap_prompt_runtime_impl,
};
use crate::prompt_session::{
    PromptSessionBundle, bootstrap_prompt_session as bootstrap_prompt_session_impl,
};
use crate::row_filtering::{
    RowFilteringBundle, bootstrap_row_filtering as bootstrap_row_filtering_impl,
};
use crate::row_ranges::{RowRangeMorphismBundle, bootstrap_row_range_morphisms};
use crate::runtime_compat::{RuntimeCompatBundle, bootstrap_runtime_compat};
use crate::runtime_switch::{
    RuntimeSwitchBundle, bootstrap_runtime_switch, extract_architecture_switch_from_argv,
};
use crate::schema::{RetaContextSchema, bootstrap_schema};
use crate::semantics_builder::{SemanticsBuilderBundle, bootstrap_semantics_builder};
use crate::shadow_pipeline::{ShadowPipelineBundle, bootstrap_shadow_pipeline};
use crate::sheaf::{SheafBundle, bootstrap_sheaves};
use crate::split_i18n::{SplitI18nProxy, build_split_i18n_proxy};
use crate::table_adapters::{TableAdaptersBundle, bootstrap_table_adapters};
use crate::table_generation::{
    TableGenerationBundle, bootstrap_table_generation as bootstrap_table_generation_impl,
};
use crate::table_materialization::{
    TableMaterializationBundle, TableMaterializationConfig,
    bootstrap_table_materialization as bootstrap_table_materialization_impl,
};
use crate::table_output::{
    TableOutputBundle, bootstrap_table_output as bootstrap_table_output_impl,
};
use crate::table_preparation::{
    TablePreparationBundle, bootstrap_table_preparation as bootstrap_table_preparation_impl,
};
use crate::table_runtime::{
    TableRuntimeBundle, bootstrap_table_runtime as bootstrap_table_runtime_impl,
};
use crate::table_state::{TableStateBundle, bootstrap_table_state as bootstrap_table_state_impl};
use crate::table_view_activation_transaction::{
    TableViewActivationTransactionBundle,
    bootstrap_table_view_activation_transaction as bootstrap_table_view_activation_transaction_impl,
};
use crate::table_view_activation_journal::{
    TableViewActivationJournalBundle,
    bootstrap_table_view_activation_journal as bootstrap_table_view_activation_journal_impl,
};
use crate::table_view_activation_replay::{
    TableViewActivationReplayBundle,
    bootstrap_table_view_activation_replay as bootstrap_table_view_activation_replay_impl,
};
use crate::table_view_activation_ledger::{
    TableViewActivationLedgerBundle,
    bootstrap_table_view_activation_ledger as bootstrap_table_view_activation_ledger_impl,
};
use crate::table_view::{
    MaterializedTableViewConfig, TableViewBundle, bootstrap_table_view as bootstrap_table_view_impl,
};
use crate::table_view_cell_styles::{
    TableViewCellStyleBundle,
    bootstrap_table_view_cell_styles as bootstrap_table_view_cell_styles_impl,
};
use crate::table_view_commit_audit::{
    TableViewCommitAuditBundle,
    bootstrap_table_view_commit_audit as bootstrap_table_view_commit_audit_impl,
};
use crate::table_view_html_attributes::{
    TableViewHtmlAttributeBundle,
    bootstrap_table_view_html_attributes as bootstrap_table_view_html_attributes_impl,
};
use crate::table_view_layout::{
    TableViewLayoutBundle, bootstrap_table_view_layout as bootstrap_table_view_layout_impl,
};
use crate::table_view_numbering::{
    TableViewNumberingBundle, bootstrap_table_view_numbering as bootstrap_table_view_numbering_impl,
};
use crate::table_view_output::{
    TableViewOutputBundle, TableViewOutputConfig,
    bootstrap_table_view_output as bootstrap_table_view_output_impl,
};
use crate::table_view_output_parity::{
    TableViewOutputParityBundle, TableViewOutputParityConfig,
    bootstrap_table_view_output_parity as bootstrap_table_view_output_parity_impl,
};
use crate::table_view_row_styles::{
    TableViewRowStyleBundle,
    bootstrap_table_view_row_styles as bootstrap_table_view_row_styles_impl,
};
use crate::table_view_shell_styles::{
    TableViewShellStyleBundle,
    bootstrap_table_view_shell_styles as bootstrap_table_view_shell_styles_impl,
};
use crate::table_view_virtual_columns::{
    TableViewVirtualColumnBundle,
    bootstrap_table_view_virtual_columns as bootstrap_table_view_virtual_columns_impl,
};
use crate::table_view_virtual_parity::{
    TableViewVirtualParityBundle,
    bootstrap_table_view_virtual_parity as bootstrap_table_view_virtual_parity_impl,
};
use crate::table_wrapping::{
    TableWrappingBundle, bootstrap_table_wrapping as bootstrap_table_wrapping_impl,
};
use crate::tag_schema::{TagSchemaBundle, bootstrap_tag_schema};
use crate::topology::{ContextSelection, RetaContextTopology};
use crate::universal::UniversalBundle;

pub const ARCHITECTURE_COUNTS_SNAPSHOT: &str =
    include_str!("../data/architecture_counts_snapshot.json");

#[derive(Clone, Debug)]
pub struct ArchitectureRuntime {
    pub topology: RetaContextTopology,
    pub architecture_activation: ArchitectureActivationBundle,
    pub architecture_boundaries: ArchitectureBoundariesBundle,
    pub architecture_coherence: ArchitectureCoherenceBundle,
    pub architecture_contracts: ArchitectureContractsBundle,
    pub architecture_impact: ArchitectureImpactBundle,
    pub architecture_map: ArchitectureMapBundle,
    pub architecture_migration: ArchitectureMigrationBundle,
    pub architecture_progress: ArchitectureProgressBundle,
    pub architecture_rehearsal: ArchitectureRehearsalBundle,
    pub architecture_traces: ArchitectureTraceBundle,
    pub architecture_validation: ArchitectureValidationBundle,
    pub architecture_witnesses: ArchitectureWitnessBundle,
    pub arithmetic: ArithmeticMorphismBundle,
    pub category_theory: CategoryTheoryBundle,
    pub console_io: ConsoleIOMorphismBundle,
    pub csv_catalog: CsvCatalogBundle,
    pub execution_network: ExecutionNetworkBundle,
    pub execution_network_bridge: ExecutionNetworkBridgeBundle,
    pub column_selection: ColumnSelectionBundle,
    pub combi_join: KombiJoinBundle,
    pub completion_runtime: CompletionRuntimeBundle,
    pub completion_nested: NestedCompletionMorphismBundle,
    pub completion_word: WordCompletionMorphismBundle,
    pub concat_csv: ConcatCsvBundle,
    pub generated_columns: GeneratedColumnsBundle,
    pub html_class_catalog: HtmlClassCatalogBundle,
    pub input_semantics: InputBundle,
    pub meta_columns: MetaColumnsBundle,
    pub migration_control: MigrationControlBundle,
    pub number_theory: NumberTheoryBundle,
    pub output_semantics: RetaOutputSemantics,
    pub output_syntax: OutputSyntaxBundle,
    pub package_integrity: PackageIntegrityBundle,
    pub parallel_execution: ParallelExecutionBundle,
    pub parity_harness: ParityHarnessBundle,
    pub persistence: PersistenceBundle,
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
    pub runtime_compat: RuntimeCompatBundle,
    pub runtime_switch: RuntimeSwitchBundle,
    pub schema: RetaContextSchema,
    pub semantics_builder: SemanticsBuilderBundle,
    pub sheaves: SheafBundle,
    pub semantic_morphisms: MorphismBundle,
    pub shadow_pipeline: ShadowPipelineBundle,
    pub split_i18n: SplitI18nProxy,
    pub tag_schema: TagSchemaBundle,
    pub table_adapters: TableAdaptersBundle,
    pub table_generation: TableGenerationBundle,
    pub table_materialization: TableMaterializationBundle,
    pub table_view: TableViewBundle,
    pub table_view_activation_transaction: TableViewActivationTransactionBundle,
    pub table_view_activation_journal: TableViewActivationJournalBundle,
    pub table_view_activation_replay: TableViewActivationReplayBundle,
    pub table_view_activation_ledger: TableViewActivationLedgerBundle,
    pub table_view_html_attributes: TableViewHtmlAttributeBundle,
    pub table_view_cell_styles: TableViewCellStyleBundle,
    pub table_view_commit_audit: TableViewCommitAuditBundle,
    pub table_view_layout: TableViewLayoutBundle,
    pub table_view_numbering: TableViewNumberingBundle,
    pub table_view_output: TableViewOutputBundle,
    pub table_view_output_parity: TableViewOutputParityBundle,
    pub table_view_row_styles: TableViewRowStyleBundle,
    pub table_view_shell_styles: TableViewShellStyleBundle,
    pub table_view_virtual_columns: TableViewVirtualColumnBundle,
    pub table_view_virtual_parity: TableViewVirtualParityBundle,
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

        let schema = bootstrap_schema();
        let topology = RetaContextTopology::standard();
        let output_semantics = bootstrap_output_semantics();
        let output_syntax = bootstrap_output_syntax_impl();
        let sheaves = bootstrap_sheaves(Some(&schema));
        let semantic_morphisms =
            bootstrap_semantic_morphisms(&topology, &sheaves, Some(output_semantics.clone()));
        let architecture_map = bootstrap_architecture_map_impl();
        let architecture_contracts = bootstrap_architecture_contracts_impl(Some(&architecture_map));
        let architecture_witnesses =
            bootstrap_architecture_witnesses_impl(&architecture_map, &architecture_contracts);
        let architecture_coherence =
            bootstrap_architecture_coherence_impl(&architecture_map, &architecture_contracts);
        let architecture_boundaries =
            bootstrap_architecture_boundaries_impl(&architecture_map, &architecture_coherence);
        let architecture_traces = bootstrap_architecture_traces_impl(
            &architecture_map,
            &architecture_contracts,
            &architecture_witnesses,
            &architecture_coherence,
        );
        let architecture_impact = bootstrap_architecture_impact_impl(
            &architecture_map,
            &architecture_contracts,
            &architecture_witnesses,
            &architecture_boundaries,
            &architecture_traces,
        );
        let architecture_migration = bootstrap_architecture_migration_impl(&architecture_impact);
        let architecture_rehearsal =
            bootstrap_architecture_rehearsal_impl(&architecture_migration, &architecture_contracts);
        let architecture_activation =
            bootstrap_architecture_activation_impl(&architecture_rehearsal);
        let architecture_progress = bootstrap_architecture_progress_impl(
            &architecture_map,
            &architecture_migration,
            &architecture_activation,
        );
        let architecture_validation = bootstrap_architecture_validation_impl(
            &architecture_map,
            &architecture_contracts,
            &architecture_witnesses,
            &architecture_coherence,
            &architecture_boundaries,
            &architecture_traces,
            &architecture_impact,
            &architecture_migration,
            &architecture_rehearsal,
            &architecture_activation,
            &architecture_progress,
        );
        Self {
            topology,
            architecture_activation,
            architecture_boundaries,
            architecture_coherence,
            architecture_contracts,
            architecture_impact,
            architecture_map,
            architecture_migration,
            architecture_progress,
            architecture_rehearsal,
            architecture_traces,
            architecture_validation,
            architecture_witnesses,
            arithmetic: bootstrap_arithmetic_morphisms(None, None),
            category_theory: bootstrap_category_theory_impl(),
            console_io: bootstrap_console_io_morphisms(None),
            csv_catalog: bootstrap_csv_catalog(),
            execution_network: bootstrap_execution_network_impl(None),
            execution_network_bridge: bootstrap_execution_network_bridge(None),
            column_selection: bootstrap_column_selection_impl(),
            combi_join: bootstrap_combi_join_impl(),
            completion_runtime: bootstrap_completion_runtime_impl(),
            completion_nested: bootstrap_nested_completion_morphisms(),
            completion_word: bootstrap_word_completion_morphisms(),
            concat_csv: bootstrap_concat_csv_impl(),
            generated_columns: bootstrap_generated_columns_impl(),
            html_class_catalog: bootstrap_html_class_catalog(),
            input_semantics: bootstrap_input_semantics(Some(schema.clone())),
            meta_columns: bootstrap_meta_columns_impl(),
            migration_control: bootstrap_migration_control(),
            number_theory: bootstrap_number_theory_impl(),
            output_semantics,
            output_syntax,
            package_integrity: bootstrap_package_integrity(),
            parallel_execution: bootstrap_parallel_execution_impl(None),
            parity_harness: bootstrap_parity_harness(),
            persistence: bootstrap_persistence_impl(None, None),
            parameter_runtime: bootstrap_parameter_runtime_impl(),
            program_workflow: bootstrap_program_workflow_impl(),
            prompt_runtime: bootstrap_prompt_runtime_impl(),
            prompt_session: bootstrap_prompt_session_impl(),
            prompt_preparation: bootstrap_prompt_preparation_impl(),
            prompt_execution: bootstrap_prompt_execution_impl(),
            prompt_interaction: bootstrap_prompt_interaction_impl(),
            prompt_language: bootstrap_prompt_language_impl(),
            presheaves: bootstrap_presheaves(None),
            row_filtering: bootstrap_row_filtering_impl(),
            row_ranges: bootstrap_row_range_morphisms(None),
            runtime_compat: bootstrap_runtime_compat(None, &[]),
            runtime_switch: bootstrap_runtime_switch(None),
            schema: schema.clone(),
            semantics_builder: bootstrap_semantics_builder(Some(schema)),
            sheaves,
            semantic_morphisms,
            shadow_pipeline: bootstrap_shadow_pipeline(),
            split_i18n: build_split_i18n_proxy(None),
            tag_schema: bootstrap_tag_schema(),
            table_adapters: bootstrap_table_adapters(),
            table_generation: bootstrap_table_generation_impl(),
            table_materialization: bootstrap_table_materialization_impl(),
            table_view: bootstrap_table_view_impl(),
            table_view_activation_transaction: bootstrap_table_view_activation_transaction_impl(),
            table_view_activation_journal: bootstrap_table_view_activation_journal_impl(),
            table_view_activation_replay: bootstrap_table_view_activation_replay_impl(),
            table_view_activation_ledger: bootstrap_table_view_activation_ledger_impl(),
            table_view_html_attributes: bootstrap_table_view_html_attributes_impl(),
            table_view_cell_styles: bootstrap_table_view_cell_styles_impl(),
            table_view_commit_audit: bootstrap_table_view_commit_audit_impl(),
            table_view_layout: bootstrap_table_view_layout_impl(),
            table_view_numbering: bootstrap_table_view_numbering_impl(),
            table_view_output: bootstrap_table_view_output_impl(),
            table_view_output_parity: bootstrap_table_view_output_parity_impl(),
            table_view_row_styles: bootstrap_table_view_row_styles_impl(),
            table_view_shell_styles: bootstrap_table_view_shell_styles_impl(),
            table_view_virtual_columns: bootstrap_table_view_virtual_columns_impl(),
            table_view_virtual_parity: bootstrap_table_view_virtual_parity_impl(),
            table_output: bootstrap_table_output_impl(),
            table_preparation: bootstrap_table_preparation_impl(),
            table_runtime: bootstrap_table_runtime_impl(),
            table_state: bootstrap_table_state_impl(),
            table_wrapping: bootstrap_table_wrapping_impl(),
            morphisms,
            universal: UniversalBundle::new(),
        }
    }

    pub fn architecture_terms(&self) -> Vec<&'static str> {
        vec![
            "architecture_map",
            "architecture_contracts",
            "architecture_witnesses",
            "architecture_coherence",
            "architecture_boundaries",
            "architecture_traces",
            "architecture_impact",
            "architecture_migration",
            "architecture_rehearsal",
            "architecture_activation",
            "architecture_progress",
            "architecture_validation",
            "execution_network_bridge",
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
            "arithmetic",
            "console_io",
            "parallel_execution",
            "persistence",
            "schema",
            "input_semantics",
            "semantics_builder",
            "runtime_compat",
            "runtime_switch",
            "csv_catalog",
            "html_class_catalog",
            "shadow_pipeline",
            "migration_control",
            "parity_harness",
            "split_i18n",
            "package_integrity",
            "row_filtering",
            "number_theory",
            "column_selection",
            "parameter_runtime",
            "parameter_matrix",
            "output_syntax",
            "output_semantics",
            "table_state",
            "table_runtime",
            "table_generation",
            "table_materialization",
            "table_view",
            "table_view_activation_transaction",
            "table_view_activation_journal",
            "table_view_activation_replay",
            "table_view_activation_ledger",
            "table_view_html_attributes",
            "table_view_cell_styles",
            "table_view_commit_audit",
            "table_view_layout",
            "table_view_numbering",
            "table_view_output",
            "table_view_output_parity",
            "table_view_row_styles",
            "table_view_shell_styles",
            "table_view_virtual_columns",
            "table_view_virtual_parity",
            "table_preparation",
            "table_output",
            "table_wrapping",
            "table_adapters",
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
            "semantic_morphisms",
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
            rust_architecture_map_capsule_count: self.architecture_map.capsules.len(),
            rust_architecture_contract_diagram_count: self.architecture_contracts.diagrams.len(),
            rust_architecture_witness_anchor_count: self
                .architecture_witnesses
                .anchor_witnesses
                .len(),
            rust_architecture_coherence_route_count: self
                .architecture_coherence
                .functorial_routes
                .len(),
            rust_architecture_boundary_edge_count: self.architecture_boundaries.import_edges.len(),
            rust_architecture_trace_component_count: self
                .architecture_traces
                .component_traces
                .len(),
            rust_architecture_impact_candidate_count: self
                .architecture_impact
                .migration_candidates
                .len(),
            rust_architecture_migration_step_count: self.architecture_migration.steps.len(),
            rust_architecture_rehearsal_move_count: self.architecture_rehearsal.moves.len(),
            rust_architecture_activation_unit_count: self.architecture_activation.units.len(),
            rust_architecture_progress_outstanding_count: self
                .architecture_progress
                .outstanding_work
                .len(),
            rust_architecture_validation_status: self
                .architecture_validation
                .summary
                .status
                .clone(),
            rust_execution_network_gate_count: self.execution_network_bridge.gates.len(),
            rust_category_count: self.category_theory.categories.len(),
            rust_functor_count: self.category_theory.functors.len(),
            rust_natural_transformation_count: self.category_theory.natural_transformations.len(),
            rust_column_bucket_count: self.column_selection.bucket_values().len(),
            rust_output_mode_count: self.output_syntax.modes().len(),
            rust_parameter_main_count: self.parameter_runtime.main_commands.len(),
            rust_parameter_matrix_seed_count: parameter_matrix_seed_count(),
            rust_parameter_matrix_integer_column_count: integer_column_projection_count(),
            rust_parameter_matrix_symbol_count: symbolic_bucket_projection_count(),
            rust_parameter_matrix_bucket_projection_count: nonempty_bucket_projection_count(),
            rust_schema_parameter_pair_count: self.schema.para_n_data_matrix.len(),
            rust_prompt_start_command_count: self.completion_runtime.start_commands(true).len(),
            rust_row_filter_condition_count: self.row_filtering.snapshot().condition_families.len(),
            rust_table_preparation_morphism_count: self
                .table_preparation
                .snapshot()
                .universal_operations
                .len(),
            rust_table_output_morphism_count: self.table_output.snapshot().morphisms.len(),
            rust_word_completion_morphism_count: self.completion_word.snapshot().morphisms.len(),
            rust_nested_completion_morphism_count: self
                .completion_nested
                .snapshot()
                .morphisms
                .len(),
            rust_prompt_runtime_main_count: self.prompt_runtime.snapshot().main_para_cmds.len(),
            rust_prompt_session_end_command_count: self
                .prompt_session
                .snapshot()
                .befehle_beenden_len,
            rust_prompt_preparation_domain_count: self
                .prompt_preparation
                .snapshot()
                .cached_parameter_value_domains
                .len(),
            rust_prompt_execution_command_count: self
                .prompt_execution
                .snapshot()
                .known_commands_len,
            rust_prompt_interaction_command_count: self.prompt_interaction.snapshot().befehle_len,
            rust_generated_column_morphism_count: self.generated_columns.snapshot().count,
            rust_meta_column_morphism_count: self.meta_columns.snapshot().count,
            rust_concat_csv_morphism_count: self.concat_csv.snapshot().count,
            rust_combi_join_morphism_count: self.combi_join.snapshot().count,
            rust_arithmetic_morphism_count: self.arithmetic.snapshot().morphisms.len(),
            rust_console_io_morphism_count: self.console_io.snapshot().morphisms.len(),
            rust_csv_catalog_asset_count: csv_asset_count(),
            rust_csv_catalog_language_variant_count: csv_language_variant_count(),
            rust_csv_catalog_total_row_count: csv_total_row_count(),
            rust_csv_catalog_nonempty_cell_count: self
                .csv_catalog
                .snapshot()
                .total_nonempty_cell_count,
            rust_html_class_record_count: html_class_record_count(),
            rust_html_class_unique_column_count: html_class_unique_column_count(),
            rust_table_materialization_morphism_count: self
                .table_materialization
                .snapshot()
                .morphisms
                .len(),
            rust_table_materialization_smoke_cell_count: self
                .table_materialization
                .materialize_cli_args(
                    &[
                        "reta",
                        "-zeilen",
                        "--vorhervonausschnitt=1-1",
                        "-spalten",
                        "--kontinuum=m",
                    ],
                    &TableMaterializationConfig::default(),
                )
                .materialized_cell_count,
            rust_table_view_morphism_count: self.table_view.snapshot().morphisms.len(),
            rust_table_view_activation_transaction_morphism_count: self
                .table_view_activation_transaction
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_activation_transaction_smoke_selected_lines:
                crate::table_view_activation_transaction::continuum_m_activation_transaction_smoke()
                    .selected_line_count,
            rust_table_view_activation_journal_morphism_count: self
                .table_view_activation_journal
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_activation_journal_smoke_record_count:
                crate::table_view_activation_journal::continuum_m_activation_journal_smoke()
                    .record_count,
            rust_table_view_activation_replay_morphism_count: self
                .table_view_activation_replay
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_activation_replay_smoke_selected_lines:
                crate::table_view_activation_replay::continuum_m_activation_replay_smoke()
                    .selected_line_count,
            rust_table_view_activation_ledger_morphism_count: self
                .table_view_activation_ledger
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_activation_ledger_smoke_entry_count:
                crate::table_view_activation_ledger::continuum_m_activation_ledger_smoke()
                    .entry_count,
            rust_table_view_activation_ledger_validation_status:
                crate::table_view_activation_ledger::continuum_m_activation_ledger_smoke()
                    .validation
                    .status,
            rust_table_view_html_attribute_morphism_count: self
                .table_view_html_attributes
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_html_attribute_smoke_cell_count:
                crate::table_view_html_attributes::continuum_m_html_attribute_smoke()
                    .attributed_cell_count,
            rust_table_view_cell_style_morphism_count: self
                .table_view_cell_styles
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_cell_style_smoke_styled_count:
                crate::table_view_cell_styles::continuum_m_cell_style_smoke(
                    crate::output_syntax::OutputMode::Html,
                )
                .styled_cell_count,
            rust_table_view_layout_morphism_count: self
                .table_view_layout
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_layout_smoke_page_count: crate::table_view_layout::layout_smoke_report(
            )
            .page_count,
            rust_table_view_numbering_morphism_count: self
                .table_view_numbering
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_numbering_smoke_column_count:
                crate::table_view_numbering::numbering_smoke_report().numbering_column_count,
            rust_table_view_output_morphism_count: self
                .table_view_output
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_output_parity_morphism_count: self
                .table_view_output_parity
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_output_parity_smoke_row_count: self
                .table_view_output_parity
                .compare_lines(
                    &["| A |".to_string(), "| x |".to_string()],
                    &[
                        "| A |".to_string(),
                        "| --- |".to_string(),
                        "| x |".to_string(),
                    ],
                    &TableViewOutputParityConfig::default()
                        .with_mode(crate::output_syntax::OutputMode::Markdown),
                )
                .left
                .semantic_row_count,
            rust_table_view_row_style_morphism_count: self
                .table_view_row_styles
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_row_style_smoke_colored_count:
                crate::table_view_row_styles::continuum_m_row_style_smoke(
                    crate::output_syntax::OutputMode::Html,
                )
                .colored_row_count,
            rust_table_view_shell_style_morphism_count: self
                .table_view_shell_styles
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_shell_style_smoke_ansi_count:
                crate::table_view_shell_styles::continuum_m_shell_style_smoke().ansi_cell_count,
            rust_table_view_virtual_column_morphism_count: self
                .table_view_virtual_columns
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_virtual_column_smoke_rendered_count:
                crate::table_view_virtual_columns::continuum_m_virtual_column_policy_smoke(
                    crate::table_view::VirtualColumnDisplayPolicy::TagSummary,
                )
                .rendered_virtual_cell_count,
            rust_table_view_virtual_parity_morphism_count: self
                .table_view_virtual_parity
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_virtual_parity_smoke_added_count:
                crate::table_view_virtual_parity::continuum_m_virtual_parity_smoke()
                    .added_virtual_cell_count,
            rust_table_view_commit_audit_morphism_count: self
                .table_view_commit_audit
                .snapshot()
                .morphisms
                .len(),
            rust_table_view_commit_audit_required_guard_count: self
                .table_view_commit_audit
                .snapshot()
                .required_guards
                .len(),
            rust_table_view_output_smoke_line_count: self
                .table_view_output
                .render_cli_args(
                    &[
                        "reta",
                        "-zeilen",
                        "--vorhervonausschnitt=1-1",
                        "-spalten",
                        "--kontinuum=m",
                        "-ausgabe",
                        "--art=markdown",
                    ],
                    &TableMaterializationConfig::default(),
                    &TableViewOutputConfig::default(),
                )
                .rendered_line_count,
            rust_table_view_smoke_row_count: self
                .table_view
                .view_for_cli_args(
                    &[
                        "reta",
                        "-zeilen",
                        "--vorhervonausschnitt=1-1",
                        "-spalten",
                        "--kontinuum=m",
                    ],
                    &TableMaterializationConfig::default(),
                    &MaterializedTableViewConfig::default(),
                )
                .row_count,
            rust_parallel_execution_morphism_count: self
                .parallel_execution
                .snapshot()
                .morphisms
                .len(),
            rust_persistence_table_count: self.persistence.snapshot().tables.len(),
            rust_schema_main_alias_count: self.schema.main_alias_groups().len(),
            rust_input_semantics_main_alias_count: self
                .input_semantics
                .build_prompt_vocabulary()
                .main_parameters
                .len(),
            rust_semantics_builder_data_dict_count: self.semantics_builder.snapshot().data_dict_len,
            rust_package_required_path_count: self.package_integrity.required_source_paths.len(),
            rust_runtime_compat_morphism_count: self.runtime_compat.snapshot().morphisms.len(),
            rust_split_i18n_module_count: self.split_i18n.source_modules.len(),
            rust_table_adapter_morphism_count: self.table_adapters.snapshot().morphisms.len(),
            rust_runtime_switch_morphism_count: self.runtime_switch.known_morphisms.len(),
            rust_shadow_pipeline_morphism_count: self.shadow_pipeline.snapshot().morphisms.len(),
            rust_migration_control_step_count: self.migration_control.steps.len(),
            rust_parity_harness_case_count: self.parity_harness.cases.len(),
            rust_semantic_morphism_available_count: self
                .semantic_morphisms
                .snapshot()
                .available
                .len(),
            rust_parameter_sheaf_alias_count: self.sheaves.parameter_semantics.main_alias_map.len(),
            rust_parameter_sheaf_pair_count: self.sheaves.parameter_semantics.pair_to_columns.len(),
            rust_presheaf_section_count: self.presheaves.snapshot().csv.section_count
                + self.presheaves.snapshot().translations.section_count
                + self.presheaves.snapshot().assets.section_count
                + self.presheaves.snapshot().prompt_state.section_count,
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
    pub rust_architecture_map_capsule_count: usize,
    pub rust_architecture_contract_diagram_count: usize,
    pub rust_architecture_witness_anchor_count: usize,
    pub rust_architecture_coherence_route_count: usize,
    pub rust_architecture_boundary_edge_count: usize,
    pub rust_architecture_trace_component_count: usize,
    pub rust_architecture_impact_candidate_count: usize,
    pub rust_architecture_migration_step_count: usize,
    pub rust_architecture_rehearsal_move_count: usize,
    pub rust_architecture_activation_unit_count: usize,
    pub rust_architecture_progress_outstanding_count: usize,
    pub rust_architecture_validation_status: String,
    pub rust_execution_network_gate_count: usize,
    pub rust_category_count: usize,
    pub rust_functor_count: usize,
    pub rust_natural_transformation_count: usize,
    pub rust_column_bucket_count: usize,
    pub rust_output_mode_count: usize,
    pub rust_parameter_main_count: usize,
    pub rust_parameter_matrix_seed_count: usize,
    pub rust_parameter_matrix_integer_column_count: usize,
    pub rust_parameter_matrix_symbol_count: usize,
    pub rust_parameter_matrix_bucket_projection_count: usize,
    pub rust_schema_parameter_pair_count: usize,
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
    pub rust_arithmetic_morphism_count: usize,
    pub rust_console_io_morphism_count: usize,
    pub rust_csv_catalog_asset_count: usize,
    pub rust_csv_catalog_language_variant_count: usize,
    pub rust_csv_catalog_total_row_count: usize,
    pub rust_csv_catalog_nonempty_cell_count: usize,
    pub rust_html_class_record_count: usize,
    pub rust_html_class_unique_column_count: usize,
    pub rust_table_materialization_morphism_count: usize,
    pub rust_table_materialization_smoke_cell_count: usize,
    pub rust_table_view_morphism_count: usize,
    pub rust_table_view_activation_transaction_morphism_count: usize,
    pub rust_table_view_activation_transaction_smoke_selected_lines: usize,
    pub rust_table_view_activation_journal_morphism_count: usize,
    pub rust_table_view_activation_journal_smoke_record_count: usize,
    pub rust_table_view_activation_replay_morphism_count: usize,
    pub rust_table_view_activation_replay_smoke_selected_lines: usize,
    pub rust_table_view_activation_ledger_morphism_count: usize,
    pub rust_table_view_activation_ledger_smoke_entry_count: usize,
    pub rust_table_view_activation_ledger_validation_status: String,
    pub rust_table_view_html_attribute_morphism_count: usize,
    pub rust_table_view_html_attribute_smoke_cell_count: usize,
    pub rust_table_view_cell_style_morphism_count: usize,
    pub rust_table_view_cell_style_smoke_styled_count: usize,
    pub rust_table_view_layout_morphism_count: usize,
    pub rust_table_view_layout_smoke_page_count: usize,
    pub rust_table_view_numbering_morphism_count: usize,
    pub rust_table_view_numbering_smoke_column_count: usize,
    pub rust_table_view_output_morphism_count: usize,
    pub rust_table_view_output_parity_morphism_count: usize,
    pub rust_table_view_output_parity_smoke_row_count: usize,
    pub rust_table_view_row_style_morphism_count: usize,
    pub rust_table_view_row_style_smoke_colored_count: usize,
    pub rust_table_view_shell_style_morphism_count: usize,
    pub rust_table_view_shell_style_smoke_ansi_count: usize,
    pub rust_table_view_virtual_column_morphism_count: usize,
    pub rust_table_view_virtual_column_smoke_rendered_count: usize,
    pub rust_table_view_virtual_parity_morphism_count: usize,
    pub rust_table_view_virtual_parity_smoke_added_count: usize,
    pub rust_table_view_commit_audit_morphism_count: usize,
    pub rust_table_view_commit_audit_required_guard_count: usize,
    pub rust_table_view_output_smoke_line_count: usize,
    pub rust_table_view_smoke_row_count: usize,
    pub rust_parallel_execution_morphism_count: usize,
    pub rust_persistence_table_count: usize,
    pub rust_schema_main_alias_count: usize,
    pub rust_input_semantics_main_alias_count: usize,
    pub rust_semantics_builder_data_dict_count: usize,
    pub rust_package_required_path_count: usize,
    pub rust_runtime_compat_morphism_count: usize,
    pub rust_split_i18n_module_count: usize,
    pub rust_table_adapter_morphism_count: usize,
    pub rust_runtime_switch_morphism_count: usize,
    pub rust_shadow_pipeline_morphism_count: usize,
    pub rust_migration_control_step_count: usize,
    pub rust_parity_harness_case_count: usize,
    pub rust_semantic_morphism_available_count: usize,
    pub rust_parameter_sheaf_alias_count: usize,
    pub rust_parameter_sheaf_pair_count: usize,
    pub rust_presheaf_section_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetaRunArchitecture {
    pub context: ContextSelection,
    pub args_len: usize,
    pub clean_args_len: usize,
    pub scheduled_task_count: usize,
    pub execution_network_plan_mode: String,
    pub parameter_main_count: usize,
    pub selected_output_mode: Option<String>,
    pub upper_limit: Option<i64>,
    pub selected_column_count: usize,
    pub excluded_column_count: usize,
    pub resolved_column_pair_count: usize,
    pub column_bucket_count: usize,
    pub symbolic_column_bucket_count: usize,
    pub required_csv_asset_count: usize,
    pub materialized_csv_section_count: usize,
    pub materialized_csv_cell_count: usize,
    pub materialized_continuum_m: bool,
    pub materialized_column_order_override_applied: bool,
    pub materialized_column_order_preview: Vec<usize>,
    pub materialized_row_order_override_applied: bool,
    pub materialized_row_order_preview: Vec<usize>,
    pub materialized_table_view_row_count: usize,
    pub materialized_table_view_virtual_cell_count: usize,
    pub materialized_table_virtual_parity_direct_cells_equal: bool,
    pub materialized_table_virtual_parity_added_count: usize,
    pub materialized_table_output_mode: String,
    pub materialized_table_output_line_count: usize,
    pub materialized_table_output_semantic_row_count: usize,
    pub materialized_table_output_numbering_mode: String,
    pub materialized_table_output_numbering_column_count: usize,
    pub parallel_mode: String,
    pub parallel_workers: usize,
    pub architecture_mode: String,
    pub architecture_switch_source: String,
    pub architecture_allowed_gate_count: usize,
    pub architecture_shadow_gate_count: usize,
    pub architecture_rollback_anchor: Option<String>,
    pub architecture_visible_behaviour_may_change: bool,
    pub topology_owner: String,
    pub universal_property: String,
}

impl RetaRunArchitecture {
    pub fn from_cli_args(args: &[String]) -> Self {
        let context = ContextSelection::from_cli_args(args);
        let (arch_clean_args, arch_switch_config) =
            extract_architecture_switch_from_argv(args, None);
        let (clean_args, parallel_config) =
            crate::parallel_execution::extract_parallel_config_from_argv(&arch_clean_args, None);
        let task = ExecutionTask::new(0usize, clean_args.clone()).with_operation("rreta_cli_run");
        let execution_network_bridge = bootstrap_execution_network_bridge(None);
        let execution_network_plan = execution_network_bridge.plan_for_tasks(&[task.clone()]);
        let parameter_runtime = bootstrap_parameter_runtime_impl();
        let parsed = parameter_runtime.parse_cli_args(&clean_args);
        let table_generation_plan =
            crate::table_generation::TableGenerationPlan::from_parameter_command_sets(
                &parsed.command_sets,
            );
        let materialization_report =
            crate::table_materialization::bootstrap_table_materialization().materialize_plan(
                &table_generation_plan,
                &TableMaterializationConfig::default(),
            );
        let materialized_table_view = crate::table_view::bootstrap_table_view().view_from_report(
            &materialization_report,
            &MaterializedTableViewConfig::default(),
        );
        let table_output_mode = parsed
            .selected_output_mode
            .unwrap_or(crate::output_syntax::OutputMode::Shell);
        let materialized_table_output = crate::table_view_output::bootstrap_table_view_output()
            .render_cli_args(
                &clean_args,
                &TableMaterializationConfig::default(),
                &TableViewOutputConfig::default().with_mode(table_output_mode),
            );
        let materialized_table_output_semantic =
            crate::table_view_output_parity::bootstrap_table_view_output_parity().normalize_lines(
                &materialized_table_output.rendered_lines,
                &TableViewOutputParityConfig::default().with_mode(table_output_mode),
            );
        let materialized_table_virtual_parity =
            crate::table_view_virtual_parity::compare_virtual_column_policies_for_cli_args(
                &clean_args,
                &crate::table_view_virtual_parity::TableViewVirtualParityConfig::default()
                    .with_mode(table_output_mode),
            );
        let switch_bundle = bootstrap_runtime_switch(Some(arch_switch_config.clone()));
        let migration_control = bootstrap_migration_control();
        let activation_units =
            migration_control.activation_units_for_switch(&switch_bundle, &arch_switch_config);
        let architecture_allowed_gate_count = activation_units
            .iter()
            .filter(|unit| unit.can_commit)
            .count();
        let architecture_shadow_gate_count = activation_units
            .iter()
            .filter(|unit| unit.shadow_execution && !unit.can_commit)
            .count();
        Self {
            context,
            args_len: args.len(),
            clean_args_len: clean_args.len(),
            scheduled_task_count: usize::from(!task.payload.is_empty()),
            execution_network_plan_mode: execution_network_plan.mode,
            parameter_main_count: parsed.main_context_history.len(),
            selected_output_mode: parsed
                .selected_output_mode
                .map(|mode| mode.canonical_name().to_string()),
            upper_limit: parsed.upper_limit,
            selected_column_count: parsed.command_sets.selected_columns.len(),
            excluded_column_count: parsed.command_sets.excluded_columns.len(),
            resolved_column_pair_count: parsed.command_sets.resolved_alias_pairs.len(),
            column_bucket_count: parsed.command_sets.column_buckets.len(),
            symbolic_column_bucket_count: parsed.command_sets.symbolic_column_buckets.len(),
            required_csv_asset_count: table_generation_plan.csv_asset_names.len(),
            materialized_csv_section_count: materialization_report.section_count(),
            materialized_csv_cell_count: materialization_report.materialized_cell_count,
            materialized_continuum_m: materialization_report.continuum_m_columns_present,
            materialized_column_order_override_applied: materialization_report
                .column_order_override_applied,
            materialized_column_order_preview: materialization_report
                .materialized_column_order_legacy
                .iter()
                .copied()
                .take(12)
                .collect(),
            materialized_row_order_override_applied: materialization_report
                .row_order_override_applied,
            materialized_row_order_preview: materialization_report
                .materialized_row_order_zero_based
                .iter()
                .copied()
                .take(12)
                .collect(),
            materialized_table_view_row_count: materialized_table_view.row_count,
            materialized_table_view_virtual_cell_count: materialized_table_view
                .rendered_virtual_cell_count,
            materialized_table_virtual_parity_direct_cells_equal: materialized_table_virtual_parity
                .direct_cells_equal,
            materialized_table_virtual_parity_added_count: materialized_table_virtual_parity
                .added_virtual_cell_count,
            materialized_table_output_mode: materialized_table_output.mode.clone(),
            materialized_table_output_line_count: materialized_table_output.rendered_line_count,
            materialized_table_output_semantic_row_count: materialized_table_output_semantic
                .semantic_row_count,
            materialized_table_output_numbering_mode: materialized_table_output
                .numbering_mode
                .clone(),
            materialized_table_output_numbering_column_count: materialized_table_output
                .numbering_column_count,
            parallel_mode: parallel_config.mode.clone(),
            parallel_workers: parallel_config.resolved_workers(),
            architecture_mode: arch_switch_config.mode.canonical().to_string(),
            architecture_switch_source: arch_switch_config.source.clone(),
            architecture_allowed_gate_count,
            architecture_shadow_gate_count,
            architecture_rollback_anchor: arch_switch_config.rollback_anchor.clone(),
            architecture_visible_behaviour_may_change: arch_switch_config
                .visible_behaviour_may_change(),
            topology_owner: "OpenRetaContextCategory".to_string(),
            universal_property: "same_cli_context_maps_to_same_ordered_rreta_result".to_string(),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "args={} clean_args={} tasks={} exec_net={} mains={} output={:?} upper={:?} cols={}/-{} pairs={} buckets={} symbolic_buckets={} csv_assets={} materialized_sections={} materialized_cells={} continuum_m={} column_order_override={} column_order={:?} row_order_override={} row_order={:?} view_rows={} view_virtual_cells={} virtual_parity_direct={} virtual_parity_added={} view_output={} view_output_lines={} view_output_semantic_rows={} numbering={} numbering_cols={} parallel={} workers={} arch={} source={} gates={}/{} owner={} universal={}",
            self.args_len,
            self.clean_args_len,
            self.scheduled_task_count,
            self.execution_network_plan_mode,
            self.parameter_main_count,
            self.selected_output_mode,
            self.upper_limit,
            self.selected_column_count,
            self.excluded_column_count,
            self.resolved_column_pair_count,
            self.column_bucket_count,
            self.symbolic_column_bucket_count,
            self.required_csv_asset_count,
            self.materialized_csv_section_count,
            self.materialized_csv_cell_count,
            self.materialized_continuum_m,
            self.materialized_column_order_override_applied,
            self.materialized_column_order_preview,
            self.materialized_row_order_override_applied,
            self.materialized_row_order_preview,
            self.materialized_table_view_row_count,
            self.materialized_table_view_virtual_cell_count,
            self.materialized_table_virtual_parity_direct_cells_equal,
            self.materialized_table_virtual_parity_added_count,
            self.materialized_table_output_mode,
            self.materialized_table_output_line_count,
            self.materialized_table_output_semantic_row_count,
            self.materialized_table_output_numbering_mode,
            self.materialized_table_output_numbering_column_count,
            self.parallel_mode,
            self.parallel_workers,
            self.architecture_mode,
            self.architecture_switch_source,
            self.architecture_allowed_gate_count,
            self.architecture_shadow_gate_count,
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
    pub architecture_mode: String,
    pub activation_preview_count: usize,
    pub architecture_validation_status: String,
    pub context: ContextSelection,
    pub data_stream_direction: String,
    pub universal_property: String,
}

impl PromptArchitectureContext {
    pub fn from_prompt_input(program_name: &str, input: &str) -> Self {
        let token_count = input.split_whitespace().count();
        let completion_runtime = bootstrap_completion_runtime_impl();
        let word_completion = bootstrap_word_completion_morphisms();
        let nested_completion = bootstrap_nested_completion_morphisms();
        let prompt_preparation = bootstrap_prompt_preparation_impl();
        let prompt_execution = bootstrap_prompt_execution_impl();
        let switch_bundle = bootstrap_runtime_switch(None);
        let migration_control = bootstrap_migration_control();
        let activation_units = migration_control
            .activation_units_for_switch(&switch_bundle, &switch_bundle.default_config);
        let prompt_architecture_map = bootstrap_architecture_map_impl();
        let prompt_architecture_contracts =
            bootstrap_architecture_contracts_impl(Some(&prompt_architecture_map));
        let prompt_architecture_witnesses = bootstrap_architecture_witnesses_impl(
            &prompt_architecture_map,
            &prompt_architecture_contracts,
        );
        let prompt_architecture_coherence = bootstrap_architecture_coherence_impl(
            &prompt_architecture_map,
            &prompt_architecture_contracts,
        );
        let prompt_architecture_boundaries = bootstrap_architecture_boundaries_impl(
            &prompt_architecture_map,
            &prompt_architecture_coherence,
        );
        let prompt_architecture_traces = bootstrap_architecture_traces_impl(
            &prompt_architecture_map,
            &prompt_architecture_contracts,
            &prompt_architecture_witnesses,
            &prompt_architecture_coherence,
        );
        let prompt_architecture_impact = bootstrap_architecture_impact_impl(
            &prompt_architecture_map,
            &prompt_architecture_contracts,
            &prompt_architecture_witnesses,
            &prompt_architecture_boundaries,
            &prompt_architecture_traces,
        );
        let prompt_architecture_migration =
            bootstrap_architecture_migration_impl(&prompt_architecture_impact);
        let prompt_architecture_rehearsal = bootstrap_architecture_rehearsal_impl(
            &prompt_architecture_migration,
            &prompt_architecture_contracts,
        );
        let prompt_architecture_activation =
            bootstrap_architecture_activation_impl(&prompt_architecture_rehearsal);
        let prompt_architecture_progress = bootstrap_architecture_progress_impl(
            &prompt_architecture_map,
            &prompt_architecture_migration,
            &prompt_architecture_activation,
        );
        let prompt_architecture_validation = bootstrap_architecture_validation_impl(
            &prompt_architecture_map,
            &prompt_architecture_contracts,
            &prompt_architecture_witnesses,
            &prompt_architecture_coherence,
            &prompt_architecture_boundaries,
            &prompt_architecture_traces,
            &prompt_architecture_impact,
            &prompt_architecture_migration,
            &prompt_architecture_rehearsal,
            &prompt_architecture_activation,
            &prompt_architecture_progress,
        );
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
            architecture_mode: switch_bundle.default_config.mode.canonical().to_string(),
            activation_preview_count: activation_units.len(),
            architecture_validation_status: prompt_architecture_validation.summary.status,
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
        assert!(
            runtime
                .architecture_terms()
                .contains(&"natural_transformation")
        );
        assert!(
            runtime
                .snapshot_ref()
                .py_category_theory_json
                .contains("Functor")
        );
    }

    #[test]
    fn prompt_context_is_bidirectional() {
        let context = PromptArchitectureContext::from_prompt_input("rp", "reta -zeilen --alles");
        assert!(context.data_stream_direction.contains("bidirectional"));
        assert!(context.context.scopes.unwrap().contains("embedded_reta"));
    }
}

// Stage 16: concrete facade.py compatibility wrappers.
pub type RetaArchitecture = ArchitectureRuntime;

pub fn bootstrap() -> ArchitectureRuntime {
    bootstrap_architecture_runtime()
}
pub fn snapshot(runtime: Option<&ArchitectureRuntime>) -> ArchitectureSnapshotRef {
    runtime
        .map(|r| r.snapshot_ref())
        .unwrap_or_else(|| bootstrap_architecture_runtime().snapshot_ref())
}

pub fn sync_program_semantics(args: &[String]) -> RetaRunArchitecture {
    RetaRunArchitecture::from_cli_args(args)
}
pub fn sync_tables() -> TableAdaptersBundle {
    bootstrap_table_adapters()
}
pub fn update_prompt_state(program_name: &str, input: &str) -> PromptArchitectureContext {
    PromptArchitectureContext::from_prompt_input(program_name, input)
}

pub fn bootstrap_arithmetic() -> ArithmeticMorphismBundle {
    bootstrap_arithmetic_morphisms(None, None)
}
pub fn bootstrap_console_io() -> ConsoleIOMorphismBundle {
    bootstrap_console_io_morphisms(None)
}
pub fn bootstrap_nested_completion() -> NestedCompletionMorphismBundle {
    bootstrap_nested_completion_morphisms()
}
pub fn bootstrap_row_ranges() -> RowRangeMorphismBundle {
    bootstrap_row_range_morphisms(None)
}
pub fn bootstrap_word_completion() -> WordCompletionMorphismBundle {
    bootstrap_word_completion_morphisms()
}

pub fn bootstrap_architecture_activation() -> ArchitectureActivationBundle {
    bootstrap_architecture_runtime().architecture_activation
}
pub fn bootstrap_architecture_boundaries() -> ArchitectureBoundariesBundle {
    bootstrap_architecture_runtime().architecture_boundaries
}
pub fn bootstrap_architecture_coherence() -> ArchitectureCoherenceBundle {
    bootstrap_architecture_runtime().architecture_coherence
}
pub fn bootstrap_architecture_contracts() -> ArchitectureContractsBundle {
    bootstrap_architecture_runtime().architecture_contracts
}
pub fn bootstrap_architecture_impact() -> ArchitectureImpactBundle {
    bootstrap_architecture_runtime().architecture_impact
}
pub fn bootstrap_architecture_map() -> ArchitectureMapBundle {
    bootstrap_architecture_map_impl()
}
pub fn bootstrap_architecture_migration() -> ArchitectureMigrationBundle {
    bootstrap_architecture_runtime().architecture_migration
}
pub fn bootstrap_architecture_progress() -> ArchitectureProgressBundle {
    bootstrap_architecture_runtime().architecture_progress
}
pub fn bootstrap_architecture_rehearsal() -> ArchitectureRehearsalBundle {
    bootstrap_architecture_runtime().architecture_rehearsal
}
pub fn bootstrap_architecture_traces() -> ArchitectureTraceBundle {
    bootstrap_architecture_runtime().architecture_traces
}
pub fn bootstrap_architecture_validation() -> ArchitectureValidationBundle {
    bootstrap_architecture_runtime().architecture_validation
}
pub fn bootstrap_architecture_witnesses() -> ArchitectureWitnessBundle {
    bootstrap_architecture_runtime().architecture_witnesses
}

pub fn bootstrap_category_theory() -> CategoryTheoryBundle {
    bootstrap_category_theory_impl()
}
pub fn bootstrap_column_selection() -> ColumnSelectionBundle {
    bootstrap_column_selection_impl()
}
pub fn bootstrap_combi_join() -> KombiJoinBundle {
    bootstrap_combi_join_impl()
}
pub fn bootstrap_completion_runtime() -> CompletionRuntimeBundle {
    bootstrap_completion_runtime_impl()
}
pub fn bootstrap_concat_csv() -> ConcatCsvBundle {
    bootstrap_concat_csv_impl()
}
pub fn bootstrap_execution_network() -> ExecutionNetworkBundle {
    bootstrap_execution_network_impl(None)
}
pub fn bootstrap_generated_columns() -> GeneratedColumnsBundle {
    bootstrap_generated_columns_impl()
}
pub fn bootstrap_meta_columns() -> MetaColumnsBundle {
    bootstrap_meta_columns_impl()
}
pub fn bootstrap_number_theory() -> NumberTheoryBundle {
    bootstrap_number_theory_impl()
}
pub fn bootstrap_output_syntax() -> OutputSyntaxBundle {
    bootstrap_output_syntax_impl()
}
pub fn bootstrap_parallel_execution() -> ParallelExecutionBundle {
    bootstrap_parallel_execution_impl(None)
}
pub fn bootstrap_parameter_runtime() -> ParameterRuntimeBundle {
    bootstrap_parameter_runtime_impl()
}
pub fn bootstrap_persistence() -> PersistenceBundle {
    bootstrap_persistence_impl(None, None)
}
pub fn bootstrap_program_workflow() -> ProgramWorkflowBundle {
    bootstrap_program_workflow_impl()
}
pub fn bootstrap_prompt_execution() -> PromptExecutionBundle {
    bootstrap_prompt_execution_impl()
}
pub fn bootstrap_prompt_interaction() -> PromptInteractionBundle {
    bootstrap_prompt_interaction_impl()
}
pub fn bootstrap_prompt_language() -> PromptLanguageBundle {
    bootstrap_prompt_language_impl()
}
pub fn bootstrap_prompt_preparation() -> PromptPreparationBundle {
    bootstrap_prompt_preparation_impl()
}
pub fn bootstrap_prompt_runtime() -> PromptRuntimeBundle {
    bootstrap_prompt_runtime_impl()
}
pub fn bootstrap_prompt_session() -> PromptSessionBundle {
    bootstrap_prompt_session_impl()
}
pub fn bootstrap_row_filtering() -> RowFilteringBundle {
    bootstrap_row_filtering_impl()
}
pub fn bootstrap_table_generation() -> TableGenerationBundle {
    bootstrap_table_generation_impl()
}
pub fn bootstrap_table_materialization() -> TableMaterializationBundle {
    bootstrap_table_materialization_impl()
}
pub fn bootstrap_table_view_html_attributes() -> TableViewHtmlAttributeBundle {
    bootstrap_table_view_html_attributes_impl()
}

pub fn bootstrap_table_view_cell_styles() -> TableViewCellStyleBundle {
    bootstrap_table_view_cell_styles_impl()
}

pub fn bootstrap_table_view_row_styles() -> TableViewRowStyleBundle {
    bootstrap_table_view_row_styles_impl()
}

pub fn bootstrap_table_view_output() -> TableViewOutputBundle {
    bootstrap_table_view_output_impl()
}
pub fn bootstrap_table_output() -> TableOutputBundle {
    bootstrap_table_output_impl()
}
pub fn bootstrap_table_preparation() -> TablePreparationBundle {
    bootstrap_table_preparation_impl()
}
pub fn bootstrap_table_runtime() -> TableRuntimeBundle {
    bootstrap_table_runtime_impl()
}
pub fn bootstrap_table_state() -> TableStateBundle {
    bootstrap_table_state_impl()
}
pub fn bootstrap_table_wrapping() -> TableWrappingBundle {
    bootstrap_table_wrapping_impl()
}

// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "RetaArchitecture",
    "bootstrap",
    "bootstrap_arithmetic",
    "bootstrap_console_io",
    "bootstrap_nested_completion",
    "bootstrap_row_ranges",
    "bootstrap_word_completion",
    "sync_program_semantics",
    "sync_tables",
    "update_prompt_state",
    "bootstrap_architecture_activation",
    "bootstrap_architecture_boundaries",
    "bootstrap_architecture_coherence",
    "bootstrap_architecture_contracts",
    "bootstrap_architecture_impact",
    "bootstrap_architecture_map",
    "bootstrap_architecture_migration",
    "bootstrap_architecture_progress",
    "bootstrap_architecture_rehearsal",
    "bootstrap_architecture_traces",
    "bootstrap_architecture_validation",
    "bootstrap_architecture_witnesses",
    "bootstrap_category_theory",
    "bootstrap_column_selection",
    "bootstrap_combi_join",
    "bootstrap_completion_runtime",
    "bootstrap_concat_csv",
    "bootstrap_execution_network",
    "bootstrap_generated_columns",
    "bootstrap_meta_columns",
    "bootstrap_number_theory",
    "bootstrap_output_syntax",
    "bootstrap_parallel_execution",
    "bootstrap_parameter_runtime",
    "bootstrap_persistence",
    "bootstrap_program_workflow",
    "bootstrap_prompt_execution",
    "bootstrap_prompt_interaction",
    "bootstrap_prompt_language",
    "bootstrap_prompt_preparation",
    "bootstrap_prompt_runtime",
    "bootstrap_prompt_session",
    "bootstrap_row_filtering",
    "bootstrap_table_generation",
    "bootstrap_table_output",
    "bootstrap_table_preparation",
    "bootstrap_table_runtime",
    "bootstrap_table_state",
    "bootstrap_table_wrapping",
    "snapshot",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
