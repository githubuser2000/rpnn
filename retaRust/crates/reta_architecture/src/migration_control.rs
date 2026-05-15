//! Migration, rehearsal and activation control layer for the architecture port.
//!
//! This is the Rust-side operational form of the Python architecture audit
//! modules (`architecture_migration`, `architecture_rehearsal`,
//! `architecture_activation`, `architecture_progress`).  It does not claim that
//! a migration was executed; it gives each future activation unit a typed wave,
//! gate, rollback anchor and parity invariant.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::runtime_switch::{ArchitectureSwitchConfig, RuntimeSwitchBundle, SwitchGateDecision};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MigrationWaveSpec {
    pub wave_id: String,
    pub order: usize,
    pub title: String,
    pub capsule: String,
    pub owner_modules: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MigrationStepSpec {
    pub step_id: String,
    pub wave_id: String,
    pub source_owner: String,
    pub target_owner: String,
    pub morphism: String,
    pub required_gate: String,
    pub parity_oracle: String,
    pub rollback_anchor: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActivationUnitSpec {
    pub activation_id: String,
    pub step_id: String,
    pub morphism: String,
    pub gate_suite: String,
    pub open_set: String,
    pub rollback_anchor: String,
    pub can_commit: bool,
    pub shadow_execution: bool,
    pub decision_reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActivationTransactionSpec {
    pub transaction_id: String,
    pub wave_id: String,
    pub activation_units: Vec<String>,
    pub commit_order: Vec<String>,
    pub rollback_order: Vec<String>,
    pub universal_property: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MigrationControlValidation {
    pub status: String,
    pub missing_waves: Vec<String>,
    pub missing_gates: Vec<String>,
    pub duplicate_morphisms: Vec<String>,
    pub rollbackless_steps: Vec<String>,
    pub checked_steps: usize,
}

impl MigrationControlValidation {
    pub fn is_ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MigrationControlBundle {
    pub waves: Vec<MigrationWaveSpec>,
    pub steps: Vec<MigrationStepSpec>,
    pub transactions: Vec<ActivationTransactionSpec>,
    pub validation: MigrationControlValidation,
}

impl MigrationControlBundle {
    pub fn activation_units_for_switch(
        &self,
        switch_bundle: &RuntimeSwitchBundle,
        config: &ArchitectureSwitchConfig,
    ) -> Vec<ActivationUnitSpec> {
        let known = switch_bundle
            .known_morphisms
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        self.steps
            .iter()
            .map(|step| {
                let decision = if known.contains(&step.morphism) {
                    config.gate_for_morphism(&step.morphism)
                } else {
                    SwitchGateDecision {
                        morphism: step.morphism.clone(),
                        allowed_to_commit: false,
                        shadow_execution: false,
                        reason: "unknown_morphism".to_string(),
                        mode: config.mode.canonical().to_string(),
                    }
                };
                ActivationUnitSpec {
                    activation_id: format!("activate:{}", step.step_id),
                    step_id: step.step_id.clone(),
                    morphism: step.morphism.clone(),
                    gate_suite: step.required_gate.clone(),
                    open_set: step.wave_id.clone(),
                    rollback_anchor: step.rollback_anchor.clone(),
                    can_commit: decision.allowed_to_commit,
                    shadow_execution: decision.shadow_execution,
                    decision_reason: decision.reason,
                }
            })
            .collect()
    }

    pub fn transaction_for_wave(&self, wave_id: &str) -> Option<&ActivationTransactionSpec> {
        self.transactions
            .iter()
            .find(|item| item.wave_id == wave_id)
    }

    pub fn snapshot(&self) -> MigrationControlSnapshot {
        MigrationControlSnapshot {
            class: "MigrationControlBundle".to_string(),
            waves: self.waves.len(),
            steps: self.steps.len(),
            transactions: self.transactions.len(),
            validation: self.validation.clone(),
            step_morphisms: self
                .steps
                .iter()
                .map(|step| step.morphism.clone())
                .collect(),
            universal_property:
                "local_rehearsal_sections_glue_to_one_guarded_activation_transaction".to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MigrationControlSnapshot {
    pub class: String,
    pub waves: usize,
    pub steps: usize,
    pub transactions: usize,
    pub validation: MigrationControlValidation,
    pub step_morphisms: Vec<String>,
    pub universal_property: String,
}

pub fn bootstrap_migration_control() -> MigrationControlBundle {
    let waves = default_waves();
    let steps = default_steps();
    let transactions = default_transactions(&waves, &steps);
    let validation = validate_migration_control(&waves, &steps, &transactions);
    MigrationControlBundle {
        waves,
        steps,
        transactions,
        validation,
    }
}

fn default_waves() -> Vec<MigrationWaveSpec> {
    vec![
        MigrationWaveSpec {
            wave_id: "wave-01-runtime-switch".to_string(),
            order: 1,
            title: "Guarded activation envelope".to_string(),
            capsule: "RuntimeSwitchCapsule".to_string(),
            owner_modules: vec![
                "runtime_switch.rs".to_string(),
                "migration_control.rs".to_string(),
            ],
            universal_property: "switch_flags_strip_to_same_legacy_cli_args".to_string(),
        },
        MigrationWaveSpec {
            wave_id: "wave-02-table-adapters".to_string(),
            order: 2,
            title: "Table adapter shadow path".to_string(),
            capsule: "TableAdapterCapsule".to_string(),
            owner_modules: vec![
                "table_adapters.rs".to_string(),
                "table_preparation.rs".to_string(),
                "table_output.rs".to_string(),
                "shadow_pipeline.rs".to_string(),
            ],
            universal_property: "prepared_table_rendering_glues_to_same_visible_lines".to_string(),
        },
        MigrationWaveSpec {
            wave_id: "wave-03-prompt-interaction".to_string(),
            order: 3,
            title: "Prompt interaction planning".to_string(),
            capsule: "PromptCapsule".to_string(),
            owner_modules: vec![
                "prompt_interaction.rs".to_string(),
                "prompt_execution.rs".to_string(),
                "completion_nested.rs".to_string(),
                "shadow_pipeline.rs".to_string(),
            ],
            universal_property: "prompt_state_compiles_to_same_reta_argv".to_string(),
        },
        MigrationWaveSpec {
            wave_id: "wave-04-dataflow".to_string(),
            order: 4,
            title: "Threaded dataflow with deterministic gluing".to_string(),
            capsule: "ExecutionNetworkCapsule".to_string(),
            owner_modules: vec![
                "dataflow.rs".to_string(),
                "parallel_execution.rs".to_string(),
            ],
            universal_property: "fifo_lifo_priority_schedules_reduce_to_ordered_result".to_string(),
        },
    ]
}

fn default_steps() -> Vec<MigrationStepSpec> {
    vec![
        step(
            "step-runtime-switch-cli",
            "wave-01-runtime-switch",
            "argv",
            "runtime_switch",
            "runtime_switch.extract_architecture_switch_from_argv",
            "switch-flags-stripped",
            "legacy_cli_args_equal_after_strip",
        ),
        step(
            "step-table-prepare-adapter",
            "wave-02-table-adapters",
            "Program.__resultingTable",
            "table_adapters.prepare",
            "table_adapters.prepare",
            "table-prepare-shadow-diff",
            "py_reta_vs_py_arch_vs_rust_table_prepare",
        ),
        step(
            "step-table-render-adapter",
            "wave-02-table-adapters",
            "Program.finallyDisplayLines",
            "table_adapters.render",
            "table_adapters.render",
            "table-render-shadow-diff",
            "py_reta_vs_py_arch_vs_rust_rendered_lines",
        ),
        step(
            "step-table-materialization",
            "wave-02-table-adapters",
            "parameter_matrix + csv_catalog",
            "table_materialization.generation_plan",
            "table_materialization.generation_plan",
            "csv-materialization-shadow",
            "parameter_projection_to_same_csv_cells",
        ),
        step(
            "step-column-order-override",
            "wave-02-table-adapters",
            "spaltenreihenfolgeundnurdiese + materialized_csv_projection",
            "table_materialization.column_order_override",
            "table_materialization.column_order_override",
            "column-order-shadow-diff",
            "explicit_output_column_order_is_preserved_before_rendering",
        ),
        step(
            "step-row-order-override",
            "wave-02-table-adapters",
            "vorhervonausschnitt/zaehlung + materialized_csv_projection",
            "table_materialization.row_order_override",
            "table_materialization.row_order_override",
            "row-order-shadow-diff",
            "explicit_output_row_order_is_preserved_before_rendering",
        ),
        step(
            "step-table-view",
            "wave-02-table-adapters",
            "materialized_csv_sections + virtual_column_witnesses",
            "table_view.materialized_view",
            "table_view.render_lines",
            "table-view-shadow-diff",
            "materialized_sections_glue_to_same_ordered_renderable_rows",
        ),
        step(
            "step-table-view-virtual-columns",
            "wave-02-table-adapters",
            "virtual_column_witnesses + output CLI flags",
            "table_view_virtual_columns.policy",
            "table_view_virtual_columns.policy",
            "table-view-virtual-column-shadow-diff",
            "virtual_columns_are_suppressed_or_rendered_only_by_explicit_policy",
        ),
        step(
            "step-table-view-virtual-parity",
            "wave-02-table-adapters",
            "virtual_column_policy + direct_csv_cell_signatures",
            "table_view_virtual_parity.direct_cell_identity",
            "table_view_virtual_parity.added_virtual_only",
            "table-view-virtual-parity-shadow-diff",
            "rendered_virtual_columns_may_add_witnesses_but_preserve_direct_cells",
        ),
        step(
            "step-table-view-virtual-commit-guard",
            "wave-02-table-adapters",
            "CLI virtual-column flags + commit guard",
            "table_view_virtual_parity.cli_policy_lift",
            "table_view_output.commit_virtual_guard",
            "table-view-virtual-policy-commit-guard",
            "CLI-selected_virtual_policy_may_commit_only_if_direct_cells_are_identity",
        ),
        step(
            "step-table-view-commit-audit",
            "wave-02-table-adapters",
            "raw diff + semantic diff + virtual direct identity",
            "table_view_commit_audit.audit_report",
            "table_view_commit_audit.required_guards",
            "table-view-commit-audit-shadow-witness",
            "visible_table_view_output_commits_only_when_all_required_audit_guards_commute",
        ),
        step(
            "step-table-view-activation-transaction",
            "wave-02-table-adapters",
            "commit audit + selected visible line source + rollback witness",
            "table_view_activation_transaction.select_visible_source",
            "table_view_activation_transaction.commit_audit_gate",
            "table-view-activation-transaction-witness",
            "visible_output_source_is_selected_by_the_unique_safe_commit_transaction",
        ),
        step(
            "step-table-view-html-attributes",
            "wave-02-table-adapters",
            "MaterializedTableView + htmlclassesPy.jsonl",
            "table_view_html_attributes.class_projection",
            "table_view_html_attributes.class_projection",
            "table-view-html-attribute-shadow-diff",
            "html_class_witnesses_are_policy_controlled_before_visible_html_commit",
        ),
        step(
            "step-table-view-layout",
            "wave-02-table-adapters",
            "MaterializedTableView + shell width policy",
            "table_view_layout.horizontal_pages",
            "table_view_layout.horizontal_pages",
            "table-view-layout-shadow-diff",
            "column_widths_and_horizontal_pages_preserve_ordered_row_cell_matrix",
        ),
        step(
            "step-table-view-numbering",
            "wave-02-table-adapters",
            "MaterializedTableViewRow.source_row_zero_based",
            "table_view_numbering.legacy_prefix",
            "table_view_numbering.legacy_prefix",
            "table-view-numbering-shadow-diff",
            "legacy_zaehlung_and_nummerierung_prefixes_are_policy_controlled",
        ),
        step(
            "step-table-view-row-styles",
            "wave-02-table-adapters",
            "MaterializedTableViewRow.source_row_zero_based + output_syntax.coloredBeginCol",
            "table_view_row_styles.legacy_colored_begin_col",
            "table_view_row_styles.legacy_colored_begin_col",
            "table-view-row-style-shadow-diff",
            "legacy_row_colours_are_policy_controlled_and_do_not_change_cells",
        ),
        step(
            "step-table-view-cell-styles",
            "wave-02-table-adapters",
            "MaterializedTableViewCell + output_syntax.generateCell",
            "table_view_cell_styles.legacy_generate_cell",
            "table_view_cell_styles.legacy_generate_cell",
            "table-view-cell-style-shadow-diff",
            "legacy_cell_wrappers_are_policy_controlled_and_do_not_change_cell_values",
        ),
        step(
            "step-table-view-style-composition",
            "wave-02-table-adapters",
            "htmlclassesPy witnesses + output_syntax.generateCell",
            "table_view_style_composition.html_cell_merge",
            "table_view_style_composition.html_cell_merge",
            "table-view-style-composition-shadow-diff",
            "html_attribute_and_cell_style_sections_compose_without_losing_either_witness",
        ),
        step(
            "step-table-view-style-parity",
            "wave-02-table-adapters",
            "styled HTML/BBCode output + TableViewOutputParity",
            "table_view_style_parity.markup_document_normalize",
            "table_view_style_parity.raw_commit_guard",
            "table-view-style-parity-shadow-diff",
            "style_wrappers_are_semantically_ignored_but_raw_diff_still_guards_commit",
        ),
        step(
            "step-table-view-shell-styles",
            "wave-02-table-adapters",
            "MaterializedTableView + table_output.colorize",
            "table_view_shell_styles.legacy_colorize",
            "table_view_shell_styles.legacy_colorize",
            "table-view-shell-style-shadow-diff",
            "ANSI_shell_colours_are_policy_controlled_and_semantically_stripped_for_parity",
        ),
        step(
            "step-table-view-output",
            "wave-02-table-adapters",
            "MaterializedTableView",
            "table_view_output.render",
            "table_view_output.render",
            "table-view-output-shadow-diff",
            "output_mode_projection_preserves_materialized_row_order",
        ),
        step(
            "step-table-view-output-flags",
            "wave-02-table-adapters",
            "Ausgabe CLI flags + MaterializedTableView",
            "table_view_output.output_flags",
            "table_view_output.output_flags",
            "table-view-output-flags-shadow-diff",
            "output_flags_filter_headers_empty_rows_and_width_without_changing_materialized_sections",
        ),
        step(
            "step-table-view-output-commit",
            "wave-02-table-adapters",
            "TableViewOutputReport",
            "table_view_output.commit",
            "table_view_output.commit",
            "table-view-output-commit-gate",
            "commit_only_when_materialized_view_output_matches_legacy_lines",
        ),
        step(
            "step-table-view-output-parity",
            "wave-02-table-adapters",
            "Legacy lines + TableViewOutputReport",
            "table_view_output.semantic_diff",
            "table_view_output.semantic_diff",
            "table-view-output-semantic-parity",
            "raw_line_diff_and_normalized_cell_diff_are_reported_separately",
        ),
        step(
            "step-shadow-table-view-output",
            "wave-02-table-adapters",
            "Program.finallyDisplayLines + MaterializedTableView",
            "shadow_pipeline.table_view_output_adapter",
            "shadow_pipeline.table_view_output_adapter",
            "shadow-table-view-output-diff",
            "legacy_visible_lines_vs_materialized_table_view_output",
        ),
        step(
            "step-shadow-table-view-output-commit",
            "wave-02-table-adapters",
            "shadow_pipeline.table_view_output_adapter",
            "shadow_pipeline.table_view_output_commit",
            "shadow_pipeline.table_view_output_commit",
            "shadow-table-view-output-commit-gate",
            "commit_only_when_view_output_gate_and_diff_policy_commute",
        ),
        step(
            "step-shadow-table-adapter",
            "wave-02-table-adapters",
            "Program.__resultingTable + Program.finallyDisplayLines",
            "shadow_pipeline.table_adapter",
            "shadow_pipeline.table_adapter",
            "shadow-table-diff",
            "legacy_visible_lines_vs_shadow_rendered_lines",
        ),
        step(
            "step-shadow-table-commit",
            "wave-02-table-adapters",
            "shadow_pipeline.table_adapter",
            "shadow_pipeline.table_commit",
            "shadow_pipeline.table_commit",
            "shadow-table-commit-gate",
            "commit_only_when_gate_and_diff_policy_commute",
        ),
        step(
            "step-parallel-rows",
            "wave-04-dataflow",
            "multiprocessing_rows",
            "parallel_execution.rows",
            "parallel_execution.rows",
            "ordered-gluing-check",
            "serial_vs_threaded_ordered_rows",
        ),
        step(
            "step-prompt-interaction-plan",
            "wave-03-prompt-interaction",
            "PromptGrosseAusgabe",
            "prompt_interaction.plan",
            "prompt_interaction.plan",
            "prompt-argv-shadow-diff",
            "prompt_input_to_same_reta_argv",
        ),
        step(
            "step-shadow-prompt-adapter",
            "wave-03-prompt-interaction",
            "PromptGrosseAusgabe + NestedCompleter",
            "shadow_pipeline.prompt_adapter",
            "shadow_pipeline.prompt_adapter",
            "shadow-prompt-plan",
            "prompt_shadow_plan_to_same_reta_argv_and_completion_context",
        ),
        step(
            "step-shadow-prompt-commit",
            "wave-03-prompt-interaction",
            "shadow_pipeline.prompt_adapter",
            "shadow_pipeline.prompt_commit",
            "shadow_pipeline.prompt_commit",
            "shadow-prompt-commit-gate",
            "commit_only_when_legacy_compile_and_rust_prompt_argv_commute",
        ),
        step(
            "step-prompt-execution-argv",
            "wave-03-prompt-interaction",
            "PromptGrosseAusgabe.argv",
            "prompt_execution.argv",
            "prompt_execution.argv",
            "prompt-exec-shadow-diff",
            "prepared_prompt_to_same_reta_argv",
        ),
        step(
            "step-nested-completion",
            "wave-03-prompt-interaction",
            "NestedCompleter",
            "completion_nested.candidates",
            "completion_nested.candidates",
            "completion-parity-sample",
            "python_nested_vs_rust_nested_candidates",
        ),
    ]
}

fn step(
    step_id: &str,
    wave_id: &str,
    source_owner: &str,
    target_owner: &str,
    morphism: &str,
    gate: &str,
    parity_oracle: &str,
) -> MigrationStepSpec {
    MigrationStepSpec {
        step_id: step_id.to_string(),
        wave_id: wave_id.to_string(),
        source_owner: source_owner.to_string(),
        target_owner: target_owner.to_string(),
        morphism: morphism.to_string(),
        required_gate: gate.to_string(),
        parity_oracle: parity_oracle.to_string(),
        rollback_anchor: format!("rollback:{step_id}"),
        status: "planned_not_committed".to_string(),
    }
}

fn default_transactions(
    waves: &[MigrationWaveSpec],
    steps: &[MigrationStepSpec],
) -> Vec<ActivationTransactionSpec> {
    waves
        .iter()
        .map(|wave| {
            let mut unit_ids = steps
                .iter()
                .filter(|step| step.wave_id == wave.wave_id)
                .map(|step| format!("activate:{}", step.step_id))
                .collect::<Vec<_>>();
            unit_ids.sort();
            let mut rollback_order = unit_ids.clone();
            rollback_order.reverse();
            ActivationTransactionSpec {
                transaction_id: format!("transaction:{}", wave.wave_id),
                wave_id: wave.wave_id.clone(),
                activation_units: unit_ids.clone(),
                commit_order: unit_ids,
                rollback_order,
                universal_property: wave.universal_property.clone(),
                status: "rehearsal_ready".to_string(),
            }
        })
        .collect()
}

fn validate_migration_control(
    waves: &[MigrationWaveSpec],
    steps: &[MigrationStepSpec],
    transactions: &[ActivationTransactionSpec],
) -> MigrationControlValidation {
    let wave_ids = waves
        .iter()
        .map(|wave| wave.wave_id.clone())
        .collect::<BTreeSet<_>>();
    let transaction_wave_ids = transactions
        .iter()
        .map(|tx| tx.wave_id.clone())
        .collect::<BTreeSet<_>>();
    let mut missing_waves = Vec::new();
    let mut missing_gates = Vec::new();
    let mut rollbackless_steps = Vec::new();
    let mut morphism_counts: BTreeMap<String, usize> = BTreeMap::new();

    for step in steps {
        if !wave_ids.contains(&step.wave_id) || !transaction_wave_ids.contains(&step.wave_id) {
            missing_waves.push(step.step_id.clone());
        }
        if step.required_gate.trim().is_empty() {
            missing_gates.push(step.step_id.clone());
        }
        if step.rollback_anchor.trim().is_empty() {
            rollbackless_steps.push(step.step_id.clone());
        }
        *morphism_counts.entry(step.morphism.clone()).or_insert(0) += 1;
    }
    let duplicate_morphisms = morphism_counts
        .into_iter()
        .filter_map(|(morphism, count)| (count > 1).then_some(morphism))
        .collect::<Vec<_>>();

    let ready = missing_waves.is_empty()
        && missing_gates.is_empty()
        && duplicate_morphisms.is_empty()
        && rollbackless_steps.is_empty();

    MigrationControlValidation {
        status: if ready { "ready" } else { "blocked" }.to_string(),
        missing_waves,
        missing_gates,
        duplicate_morphisms,
        rollbackless_steps,
        checked_steps: steps.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_switch::{ArchitectureSwitchMode, bootstrap_runtime_switch};

    #[test]
    fn migration_control_is_validation_ready() {
        let bundle = bootstrap_migration_control();
        assert!(bundle.validation.is_ready());
        assert!(
            bundle
                .steps
                .iter()
                .any(|step| step.morphism == "table_adapters.prepare")
        );
    }

    #[test]
    fn activation_units_obey_switch_gates() {
        let bundle = bootstrap_migration_control();
        let switch = bootstrap_runtime_switch(None);
        let config =
            ArchitectureSwitchConfig::default().with_mode(ArchitectureSwitchMode::Adapter, "test");
        let units = bundle.activation_units_for_switch(&switch, &config);
        let table = units
            .iter()
            .find(|unit| unit.morphism == "table_adapters.prepare")
            .expect("table adapter unit exists");
        let prompt = units
            .iter()
            .find(|unit| unit.morphism == "prompt_execution.argv")
            .expect("prompt execution unit exists");
        assert!(table.can_commit);
        assert!(!prompt.can_commit);
    }
}
