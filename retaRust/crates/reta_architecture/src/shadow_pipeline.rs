//! Shadow execution pipeline for guarded Rust-architecture activation.
//!
//! Stages 1-9 built the typed Rust architecture and its activation metadata.
//! This module is the next operational step: it can run selected Rust
//! architecture morphisms *beside* the legacy path, produce deterministic diff
//! summaries, and decide whether an adapter result is still shadow-only or can
//! be committed by the runtime switch gate.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::dataflow::DataflowDiscipline;
use crate::execution_network::{ExecutionNetworkPlan, execution_network_plan_for_indices};
use crate::migration_control::{ActivationUnitSpec, bootstrap_migration_control};
use crate::output_syntax::OutputMode;
use crate::parity_harness::{ParityProbePlan, bootstrap_parity_harness};
use crate::prompt_interaction::{PromptInteractionPlan, bootstrap_prompt_interaction};
use crate::prompt_language::PromptModus;
use crate::runtime_switch::{
    ArchitectureSwitchConfig, ArchitectureSwitchMode, SwitchGateDecision, bootstrap_runtime_switch,
};
use crate::table_materialization::{
    TableMaterializationConfig, TableMaterializationReport, bootstrap_table_materialization,
};
use crate::table_output::{TableOutputConfig, TableRenderResult, render_prepared_table};
use crate::table_preparation::{PreparedTable, prepare_row_cells};
use crate::table_view::{MaterializedTableView, MaterializedTableViewConfig, bootstrap_table_view};
use crate::table_view_output::{
    TableViewOutputConfig, TableViewOutputReport, bootstrap_table_view_output,
};
use crate::table_view_language_parity::{
    TableViewLanguageParityReport, TableViewLanguageParityPolicy, language_parity_for_cli_args,
};
use crate::table_view_language_coverage::{
    TableViewLanguageCoverageReport, TableViewLanguageCoveragePolicy, language_coverage_for_cli_args,
};
use crate::table_view_output_parity::{
    TableViewOutputParityConfig, TableViewOutputParityReport, bootstrap_table_view_output_parity,
};
use crate::table_view_virtual_parity::{
    TableViewVirtualParityConfig, TableViewVirtualParityReport,
    compare_virtual_column_policies_for_cli_args,
};
use crate::table_wrapping::TableWidthContext;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowDiffSummary {
    pub compared_left: String,
    pub compared_right: String,
    pub equal: bool,
    pub left_lines: usize,
    pub right_lines: usize,
    pub first_mismatch_index: Option<usize>,
    pub left_at_mismatch: Option<String>,
    pub right_at_mismatch: Option<String>,
    pub missing_left_count: usize,
    pub missing_right_count: usize,
}

impl ShadowDiffSummary {
    pub fn from_lines(
        compared_left: impl Into<String>,
        compared_right: impl Into<String>,
        left: &[String],
        right: &[String],
    ) -> Self {
        let max_len = left.len().max(right.len());
        let mut first_mismatch_index = None;
        for index in 0..max_len {
            if left.get(index) != right.get(index) {
                first_mismatch_index = Some(index);
                break;
            }
        }
        let equal = first_mismatch_index.is_none();
        let left_at_mismatch = first_mismatch_index.and_then(|index| left.get(index).cloned());
        let right_at_mismatch = first_mismatch_index.and_then(|index| right.get(index).cloned());
        Self {
            compared_left: compared_left.into(),
            compared_right: compared_right.into(),
            equal,
            left_lines: left.len(),
            right_lines: right.len(),
            first_mismatch_index,
            left_at_mismatch,
            right_at_mismatch,
            missing_left_count: right.len().saturating_sub(left.len()),
            missing_right_count: left.len().saturating_sub(right.len()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowTableInput {
    pub content_table: Vec<Vec<String>>,
    pub legacy_display_lines: Vec<String>,
    pub selected_columns_zero_based: Vec<usize>,
    pub mode: OutputMode,
    pub textwidth: usize,
    pub textheight: usize,
    pub breiten: Vec<usize>,
    pub shell_rows_amount: usize,
    pub numbering: bool,
    pub color: bool,
    pub keine_ueberschriften: bool,
    pub keine_leeren_inhalte: bool,
}

impl ShadowTableInput {
    pub fn empty_for_mode(mode: OutputMode) -> Self {
        Self {
            content_table: Vec::new(),
            legacy_display_lines: Vec::new(),
            selected_columns_zero_based: Vec::new(),
            mode,
            textwidth: 21,
            textheight: 0,
            breiten: Vec::new(),
            shell_rows_amount: 0,
            numbering: true,
            color: false,
            keine_ueberschriften: false,
            keine_leeren_inhalte: false,
        }
    }

    pub fn visible_row_count(&self) -> usize {
        self.content_table.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowTableReport {
    pub morphism: String,
    pub gate: SwitchGateDecision,
    pub switch_mode: String,
    pub legacy_rows: usize,
    pub prepared_rows: usize,
    pub rendered_rows: usize,
    pub selected_columns: usize,
    pub output_mode: String,
    pub diff: ShadowDiffSummary,
    /// Full Rust-shadow output lines.  Stage 10 only exposed a preview; Stage 11
    /// keeps the whole value so a guarded commit can actually reuse it without
    /// re-rendering and without touching the legacy path.
    pub rendered_lines: Vec<String>,
    pub rendered_preview: Vec<String>,
    pub commit_candidate: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowCommitPolicy {
    pub require_gate_commit: bool,
    pub require_equal_diff: bool,
    pub allow_force_mismatch_commit: bool,
    pub max_shadow_lines: Option<usize>,
}

impl Default for ShadowCommitPolicy {
    fn default() -> Self {
        Self {
            require_gate_commit: true,
            require_equal_diff: true,
            allow_force_mismatch_commit: true,
            max_shadow_lines: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowCommitDecision {
    pub morphism: String,
    pub use_shadow_output: bool,
    pub reason: String,
    pub switch_mode: String,
    pub gate_reason: String,
    pub gate_allowed_to_commit: bool,
    pub diff_equal: bool,
    pub force_override: bool,
    pub rendered_line_count: usize,
    pub rollback_anchor: Option<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowTableViewOutputReport {
    pub morphism: String,
    pub gate: SwitchGateDecision,
    pub switch_mode: String,
    pub legacy_rows: usize,
    pub rendered_rows: usize,
    pub output_mode: String,
    pub diff: ShadowDiffSummary,
    pub semantic_diff: TableViewOutputParityReport,
    pub virtual_column_parity: TableViewVirtualParityReport,
    pub language_parity: TableViewLanguageParityReport,
    pub language_coverage: TableViewLanguageCoverageReport,
    pub output_report: TableViewOutputReport,
    pub rendered_preview: Vec<String>,
    pub commit_candidate: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowTableViewOutputCommitPolicy {
    pub require_gate_commit: bool,
    pub require_equal_diff: bool,
    pub allow_force_mismatch_commit: bool,
    pub max_shadow_lines: Option<usize>,
    pub require_virtual_direct_identity: bool,
    pub require_language_parity_ready: bool,
}

impl Default for ShadowTableViewOutputCommitPolicy {
    fn default() -> Self {
        Self {
            require_gate_commit: true,
            require_equal_diff: true,
            allow_force_mismatch_commit: true,
            max_shadow_lines: None,
            require_virtual_direct_identity: true,
            require_language_parity_ready: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowTableViewOutputCommitDecision {
    pub morphism: String,
    pub use_view_output: bool,
    pub reason: String,
    pub switch_mode: String,
    pub gate_reason: String,
    pub gate_allowed_to_commit: bool,
    pub diff_equal: bool,
    pub semantic_equal: bool,
    pub virtual_direct_cells_equal: bool,
    pub virtual_rendered_policy: String,
    pub virtual_added_column_count: usize,
    pub language_parity_ready: bool,
    pub language_requested_language: String,
    pub language_effective_asset_name: String,
    pub language_fallback_applied: bool,
    pub language_failed_guards: Vec<String>,
    pub force_override: bool,
    pub rendered_line_count: usize,
    pub rollback_anchor: Option<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowPromptLegacyCommand {
    pub kind: String,
    pub argv: Vec<String>,
    pub argv_batches: Vec<Vec<String>>,
    pub description: String,
}

impl ShadowPromptLegacyCommand {
    pub fn reta(argv: Vec<String>) -> Self {
        Self {
            kind: "reta".to_string(),
            argv,
            argv_batches: Vec::new(),
            description: "legacy_prompt_command_reta".to_string(),
        }
    }

    pub fn reta_batch(argv_batches: Vec<Vec<String>>) -> Self {
        Self {
            kind: "reta_batch".to_string(),
            argv: Vec::new(),
            argv_batches,
            description: "legacy_prompt_command_reta_batch".to_string(),
        }
    }

    pub fn other(kind: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            argv: Vec::new(),
            argv_batches: Vec::new(),
            description: description.into(),
        }
    }

    pub fn visible_argv(&self) -> &[String] {
        &self.argv
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowPromptCommitPolicy {
    pub require_gate_commit: bool,
    pub require_same_argv: bool,
    pub allow_force_mismatch_commit: bool,
}

impl Default for ShadowPromptCommitPolicy {
    fn default() -> Self {
        Self {
            require_gate_commit: true,
            require_same_argv: true,
            allow_force_mismatch_commit: false,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowPromptCommitDecision {
    pub morphism: String,
    pub use_shadow_prompt_plan: bool,
    pub reason: String,
    pub switch_mode: String,
    pub gate_reason: String,
    pub gate_allowed_to_commit: bool,
    pub legacy_kind: String,
    pub same_argv: bool,
    pub force_override: bool,
    pub planned_argv: Vec<String>,
    pub legacy_argv: Vec<String>,
    pub rollback_anchor: Option<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowPromptInput {
    pub program_name: String,
    pub prompt_text: String,
    pub placeholder: String,
    pub prompt_mode: PromptModus,
}

impl ShadowPromptInput {
    pub fn new(program_name: impl Into<String>, prompt_text: impl Into<String>) -> Self {
        Self {
            program_name: program_name.into(),
            prompt_text: prompt_text.into(),
            placeholder: String::new(),
            prompt_mode: PromptModus::Normal,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowPromptReport {
    pub morphism: String,
    pub gate: SwitchGateDecision,
    pub switch_mode: String,
    pub prepared_token_count: usize,
    pub execution_argv_count: usize,
    pub completion_preview_count: usize,
    pub planned_argv: Vec<String>,
    pub completion_preview: Vec<String>,
    pub commit_candidate: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowCliPlan {
    pub original_args: Vec<String>,
    pub cleaned_args: Vec<String>,
    pub switch_mode: String,
    pub gates: Vec<SwitchGateDecision>,
    pub activation_units: Vec<ActivationUnitSpec>,
    pub parity_plans: Vec<ParityProbePlan>,
    pub execution_network_plan: ExecutionNetworkPlan,
    pub materialization_report: TableMaterializationReport,
    pub table_view: MaterializedTableView,
    pub table_view_output: TableViewOutputReport,
    pub virtual_column_parity: TableViewVirtualParityReport,
    pub language_parity: TableViewLanguageParityReport,
    pub language_coverage: TableViewLanguageCoverageReport,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowPipelineSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub table_morphism: String,
    pub prompt_morphism: String,
    pub plan_morphism: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShadowPipelineBundle;

impl ShadowPipelineBundle {
    pub fn snapshot(&self) -> ShadowPipelineSnapshot {
        ShadowPipelineSnapshot {
            class: "ShadowPipelineBundle".to_string(),
            morphisms: vec![
                "shadow_pipeline.cli_plan".to_string(),
                "shadow_pipeline.table_adapter".to_string(),
                "shadow_pipeline.table_commit".to_string(),
                "shadow_pipeline.table_view_output_adapter".to_string(),
                "shadow_pipeline.table_view_output_commit".to_string(),
                "shadow_pipeline.table_view_output_semantic_diff".to_string(),
                "table_view_output_parity.normalize_output_lines".to_string(),
                "table_view_output_parity.compare_output_lines".to_string(),
                "shadow_pipeline.prompt_adapter".to_string(),
                "shadow_pipeline.prompt_commit".to_string(),
                "shadow_pipeline.diff_lines".to_string(),
                "table_materialization.generation_plan".to_string(),
                "table_view.materialized_view".to_string(),
                "table_view_output.render".to_string(),
                "table_view_virtual_parity.direct_cell_identity".to_string(),
                "table_view_virtual_parity.cli_policy_lift".to_string(),
                "table_view_language_parity.base_fallback_guard".to_string(),
                "table_view_language_parity.direct_744_guard".to_string(),
                "table_view_language_coverage.translation_gap_report".to_string(),
                "table_view_language_coverage.fallback_readiness_witness".to_string(),
                "table_view_output.commit_virtual_guard".to_string(),
                "table_view_commit_audit.audit_report".to_string(),
                "table_view_commit_audit.required_guards".to_string(),
                "table_view_activation_transaction.select_visible_source".to_string(),
                "table_view_activation_transaction.rollback_witness".to_string(),
                "table_view_activation_journal.record_transaction".to_string(),
                "table_view_activation_journal.replay_selected_lines".to_string(),
                "table_view_activation_replay.guard_journal_replay".to_string(),
                "table_view_activation_ledger.hash_chain".to_string(),
                "table_view_activation_ledger.validate_chain".to_string(),
            ],
            table_morphism: "shadow_pipeline.table_adapter".to_string(),
            prompt_morphism: "shadow_pipeline.prompt_adapter".to_string(),
            plan_morphism: "shadow_pipeline.cli_plan".to_string(),
            universal_property:
                "shadow_sections_compare_against_legacy_before_any_commit_gate_changes_output"
                    .to_string(),
        }
    }

    pub fn cli_plan(&self, args: &[String], config: &ArchitectureSwitchConfig) -> ShadowCliPlan {
        let (cleaned_args, switch_config) =
            crate::runtime_switch::extract_architecture_switch_from_argv(
                args,
                Some(config.clone()),
            );
        let switch_bundle = bootstrap_runtime_switch(Some(switch_config.clone()));
        let gates = switch_bundle.gate_matrix(&switch_config);
        let migration_control = bootstrap_migration_control();
        let activation_units =
            migration_control.activation_units_for_switch(&switch_bundle, &switch_config);
        let parity_harness = bootstrap_parity_harness();
        let parity_plans = parity_harness.plans_for_switch(&switch_config);
        let task_indices = (0..cleaned_args.len()).collect::<Vec<_>>();
        let execution_network_plan =
            execution_network_plan_for_indices(&task_indices, DataflowDiscipline::Fifo);
        let materialization_config = TableMaterializationConfig::from_cli_args(&cleaned_args);
        let materialization_report = bootstrap_table_materialization()
            .materialize_cli_args(&cleaned_args, &materialization_config);
        let table_view = bootstrap_table_view().view_from_report(
            &materialization_report,
            &MaterializedTableViewConfig::default(),
        );
        let parsed =
            crate::parameter_runtime::bootstrap_parameter_runtime().parse_cli_args(&cleaned_args);
        let output_mode = parsed.selected_output_mode.unwrap_or(OutputMode::Shell);
        let table_view_output = bootstrap_table_view_output().render_cli_args(
            &cleaned_args,
            &materialization_config,
            &TableViewOutputConfig::default().with_mode(output_mode),
        );
        let virtual_column_parity_config = TableViewVirtualParityConfig::from_cli_args(
            &cleaned_args,
            output_mode,
        );
        let virtual_column_parity = compare_virtual_column_policies_for_cli_args(
            &cleaned_args,
            &virtual_column_parity_config,
        );
        let language_parity = language_parity_for_cli_args(
            &cleaned_args,
            &TableViewLanguageParityPolicy::default(),
        );
        let language_coverage = language_coverage_for_cli_args(
            &cleaned_args,
            &TableViewLanguageCoveragePolicy::default(),
        );
        ShadowCliPlan {
            original_args: args.to_vec(),
            cleaned_args,
            switch_mode: switch_config.mode.canonical().to_string(),
            gates,
            activation_units,
            parity_plans,
            execution_network_plan,
            materialization_report,
            table_view,
            table_view_output,
            virtual_column_parity,
            language_parity,
            language_coverage,
            universal_property: "same_clean_cli_args_feed_legacy_and_shadow_sections".to_string(),
        }
    }

    pub fn shadow_table(
        &self,
        input: &ShadowTableInput,
        config: &ArchitectureSwitchConfig,
    ) -> ShadowTableReport {
        let gate = config.gate_for_morphism("table_adapters.render");
        let prepared = prepare_shadow_table(input);
        let display_lines = (0..prepared.len()).collect::<BTreeSet<_>>();
        let rows_range = if input.textheight == 0 {
            Vec::new()
        } else {
            (0..input.textheight).collect::<Vec<_>>()
        };
        let render_config = TableOutputConfig {
            mode: input.mode,
            one_table: input.mode.force_one_table(),
            color: input.color,
            numbering: input.numbering,
            textheight: input.textheight,
            textwidth: input.textwidth,
            breiten: input.breiten.clone(),
            shell_rows_amount: input.shell_rows_amount,
            keine_ueberschriften: input.keine_ueberschriften,
            keine_leeren_inhalte: input.keine_leeren_inhalte,
            nichts_output_yes: matches!(input.mode, OutputMode::Nichts),
        };
        let render_result = render_prepared_table(
            &display_lines,
            &prepared,
            input.visible_row_count().to_string().len().max(1),
            &rows_range,
            &render_config,
        );
        table_report_from_render(input, config, gate, prepared, render_result)
    }

    pub fn table_commit_decision(
        &self,
        report: &ShadowTableReport,
        config: &ArchitectureSwitchConfig,
    ) -> ShadowCommitDecision {
        evaluate_shadow_table_commit(report, config, &ShadowCommitPolicy::default())
    }

    pub fn shadow_table_view_output(
        &self,
        args: &[String],
        legacy_display_lines: &[String],
        config: &ArchitectureSwitchConfig,
    ) -> ShadowTableViewOutputReport {
        let (cleaned_args, switch_config) =
            crate::runtime_switch::extract_architecture_switch_from_argv(
                args,
                Some(config.clone()),
            );
        let gate = switch_config.gate_for_morphism("table_view_output.render");
        let parsed =
            crate::parameter_runtime::bootstrap_parameter_runtime().parse_cli_args(&cleaned_args);
        let output_mode = parsed.selected_output_mode.unwrap_or(OutputMode::Shell);
        let output_config = TableViewOutputConfig::default().with_mode(output_mode);
        let materialization_config = TableMaterializationConfig::from_cli_args(&cleaned_args);
        let output_report = bootstrap_table_view_output().render_cli_args(
            &cleaned_args,
            &materialization_config,
            &output_config,
        );
        let diff = ShadowDiffSummary::from_lines(
            "legacy",
            "rust_table_view_output",
            legacy_display_lines,
            &output_report.rendered_lines,
        );
        let semantic_diff = bootstrap_table_view_output_parity()
            .compare_table_view_output_to_legacy(
                &output_report,
                legacy_display_lines,
                &TableViewOutputParityConfig::default().with_mode(output_mode),
            );
        let virtual_column_parity_config = TableViewVirtualParityConfig::from_cli_args(
            &cleaned_args,
            output_mode,
        );
        let virtual_column_parity = compare_virtual_column_policies_for_cli_args(
            &cleaned_args,
            &virtual_column_parity_config,
        );
        let language_parity = language_parity_for_cli_args(
            &cleaned_args,
            &TableViewLanguageParityPolicy::default(),
        );
        let language_coverage = language_coverage_for_cli_args(
            &cleaned_args,
            &TableViewLanguageCoveragePolicy::default(),
        );
        let commit_gate = switch_config.gate_for_morphism("table_view_output.commit");
        let commit_candidate = commit_gate.allowed_to_commit
            && (diff.equal || switch_config.mode == ArchitectureSwitchMode::Force);
        ShadowTableViewOutputReport {
            morphism: "shadow_pipeline.table_view_output_adapter".to_string(),
            gate,
            switch_mode: switch_config.mode.canonical().to_string(),
            legacy_rows: legacy_display_lines.len(),
            rendered_rows: output_report.rendered_line_count,
            output_mode: output_report.mode.clone(),
            diff,
            semantic_diff,
            virtual_column_parity,
            language_parity,
            language_coverage,
            rendered_preview: output_report
                .rendered_lines
                .iter()
                .take(8)
                .cloned()
                .collect(),
            output_report,
            commit_candidate,
            universal_property:
                "materialized_table_view_output_must_match_legacy_lines_before_commit".to_string(),
        }
    }

    pub fn table_view_output_commit_decision(
        &self,
        report: &ShadowTableViewOutputReport,
        config: &ArchitectureSwitchConfig,
    ) -> ShadowTableViewOutputCommitDecision {
        evaluate_shadow_table_view_output_commit(
            report,
            config,
            &ShadowTableViewOutputCommitPolicy::default(),
        )
    }

    pub fn shadow_prompt(
        &self,
        input: &ShadowPromptInput,
        config: &ArchitectureSwitchConfig,
    ) -> ShadowPromptReport {
        let gate = config.gate_for_morphism("prompt_interaction.plan");
        let mut interaction = bootstrap_prompt_interaction();
        let plan: PromptInteractionPlan = interaction.prepare_and_plan_one_input(
            &input.placeholder,
            &input.prompt_text,
            input.prompt_mode,
        );
        let prepared_token_count = plan.prepared.tokens.len();
        let planned_argv = plan.execution_plan.reta_argv.clone();
        let completion_preview = plan.completion_preview.clone();
        ShadowPromptReport {
            morphism: "shadow_pipeline.prompt_adapter".to_string(),
            gate: gate.clone(),
            switch_mode: config.mode.canonical().to_string(),
            prepared_token_count,
            execution_argv_count: planned_argv.len(),
            completion_preview_count: completion_preview.len(),
            planned_argv,
            completion_preview,
            commit_candidate: gate.allowed_to_commit,
            universal_property: "prompt_shadow_plan_must_compile_to_same_reta_argv_before_commit"
                .to_string(),
        }
    }

    pub fn prompt_commit_decision(
        &self,
        report: &ShadowPromptReport,
        legacy: &ShadowPromptLegacyCommand,
        config: &ArchitectureSwitchConfig,
    ) -> ShadowPromptCommitDecision {
        evaluate_shadow_prompt_commit(report, legacy, config, &ShadowPromptCommitPolicy::default())
    }
}

pub fn bootstrap_shadow_pipeline() -> ShadowPipelineBundle {
    ShadowPipelineBundle
}

pub fn prepare_shadow_table(input: &ShadowTableInput) -> PreparedTable {
    let selected_columns = input
        .selected_columns_zero_based
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let width_context = TableWidthContext {
        shell_rows_amount: if input.shell_rows_amount == 0 {
            None
        } else {
            Some(input.shell_rows_amount as i64)
        },
        rows_as_numbers_len: input.content_table.len(),
        breiten: input.breiten.iter().map(|value| *value as i64).collect(),
        textwidth: input.textwidth as i64,
    };
    input
        .content_table
        .iter()
        .map(|row| prepare_row_cells(row, &selected_columns, &width_context).row)
        .collect()
}

pub fn diff_shadow_lines(left: &[String], right: &[String]) -> ShadowDiffSummary {
    ShadowDiffSummary::from_lines("legacy", "rust_shadow", left, right)
}

pub fn evaluate_shadow_table_commit(
    report: &ShadowTableReport,
    config: &ArchitectureSwitchConfig,
    policy: &ShadowCommitPolicy,
) -> ShadowCommitDecision {
    let commit_gate = config.gate_for_morphism("shadow_pipeline.table_commit");
    let gate_ok = !policy.require_gate_commit || commit_gate.allowed_to_commit;
    let force_override = config.mode == ArchitectureSwitchMode::Force
        && policy.allow_force_mismatch_commit
        && commit_gate.allowed_to_commit;
    let diff_ok = !policy.require_equal_diff || report.diff.equal || force_override;
    let size_ok = policy
        .max_shadow_lines
        .map(|limit| report.rendered_lines.len() <= limit)
        .unwrap_or(true);
    let use_shadow_output = gate_ok && diff_ok && size_ok;
    let reason = if use_shadow_output {
        if force_override && !report.diff.equal {
            "force_commit_mismatch".to_string()
        } else if report.diff.equal {
            "commit_equal_shadow".to_string()
        } else {
            "commit_policy_allows_shadow".to_string()
        }
    } else if !gate_ok {
        "gate_not_allowed_to_commit".to_string()
    } else if !diff_ok {
        "shadow_diff_not_equal".to_string()
    } else if !size_ok {
        "shadow_output_too_large_for_policy".to_string()
    } else {
        "commit_policy_rejected".to_string()
    };
    ShadowCommitDecision {
        morphism: "shadow_pipeline.table_commit".to_string(),
        use_shadow_output,
        reason,
        switch_mode: config.mode.canonical().to_string(),
        gate_reason: commit_gate.reason,
        gate_allowed_to_commit: commit_gate.allowed_to_commit,
        diff_equal: report.diff.equal,
        force_override,
        rendered_line_count: report.rendered_lines.len(),
        rollback_anchor: config.rollback_anchor.clone(),
        universal_property:
            "committed_shadow_output_is_allowed_only_when_gate_and_parity_policy_commute"
                .to_string(),
    }
}

pub fn evaluate_shadow_table_view_output_commit(
    report: &ShadowTableViewOutputReport,
    config: &ArchitectureSwitchConfig,
    policy: &ShadowTableViewOutputCommitPolicy,
) -> ShadowTableViewOutputCommitDecision {
    let commit_gate = config.gate_for_morphism("table_view_output.commit");
    let gate_ok = !policy.require_gate_commit || commit_gate.allowed_to_commit;
    let force_override = config.mode == ArchitectureSwitchMode::Force
        && policy.allow_force_mismatch_commit
        && commit_gate.allowed_to_commit;
    let diff_ok = !policy.require_equal_diff || report.diff.equal || force_override;
    let virtual_direct_ok = !policy.require_virtual_direct_identity
        || report.virtual_column_parity.direct_cells_equal;
    let language_ok = !policy.require_language_parity_ready || report.language_parity.ready();
    let size_ok = policy
        .max_shadow_lines
        .map(|limit| report.output_report.rendered_lines.len() <= limit)
        .unwrap_or(true);
    let use_view_output = gate_ok && diff_ok && virtual_direct_ok && language_ok && size_ok;
    let reason = if use_view_output {
        if force_override && !report.diff.equal {
            "force_commit_table_view_output_mismatch".to_string()
        } else if report.diff.equal {
            "commit_equal_table_view_output".to_string()
        } else {
            "commit_policy_allows_table_view_output".to_string()
        }
    } else if !gate_ok {
        "gate_not_allowed_to_commit".to_string()
    } else if !diff_ok {
        "table_view_output_diff_not_equal".to_string()
    } else if !virtual_direct_ok {
        "virtual_policy_changed_direct_csv_cells".to_string()
    } else if !language_ok {
        "language_parity_blocked".to_string()
    } else if !size_ok {
        "table_view_output_too_large_for_policy".to_string()
    } else {
        "table_view_output_commit_policy_rejected".to_string()
    };
    ShadowTableViewOutputCommitDecision {
        morphism: "shadow_pipeline.table_view_output_commit".to_string(),
        use_view_output,
        reason,
        switch_mode: config.mode.canonical().to_string(),
        gate_reason: commit_gate.reason,
        gate_allowed_to_commit: commit_gate.allowed_to_commit,
        diff_equal: report.diff.equal,
        semantic_equal: report.semantic_diff.semantic_equal,
        virtual_direct_cells_equal: report.virtual_column_parity.direct_cells_equal,
        virtual_rendered_policy: report.virtual_column_parity.rendered_policy.clone(),
        virtual_added_column_count: report.virtual_column_parity.added_virtual_column_count,
        language_parity_ready: report.language_parity.ready(),
        language_requested_language: report.language_parity.requested_language.clone(),
        language_effective_asset_name: report.language_parity.effective_asset_name.clone(),
        language_fallback_applied: report.language_parity.fallback_applied,
        language_failed_guards: report.language_parity.failed_guards.clone(),
        force_override,
        rendered_line_count: report.output_report.rendered_lines.len(),
        rollback_anchor: config.rollback_anchor.clone(),
        universal_property:
            "materialized_view_output_commits_only_when_gate_and_diff_policy_commute".to_string(),
    }
}

pub fn evaluate_shadow_prompt_commit(
    report: &ShadowPromptReport,
    legacy: &ShadowPromptLegacyCommand,
    config: &ArchitectureSwitchConfig,
    policy: &ShadowPromptCommitPolicy,
) -> ShadowPromptCommitDecision {
    let commit_gate = config.gate_for_morphism("shadow_pipeline.prompt_commit");
    let gate_ok = !policy.require_gate_commit || commit_gate.allowed_to_commit;
    let kind_ok = legacy.kind == "reta";
    let same_argv = kind_ok && legacy.visible_argv() == report.planned_argv.as_slice();
    let force_override = config.mode == ArchitectureSwitchMode::Force
        && policy.allow_force_mismatch_commit
        && commit_gate.allowed_to_commit;
    let argv_ok = !policy.require_same_argv || same_argv || force_override;
    let use_shadow_prompt_plan = gate_ok && kind_ok && argv_ok;
    let reason = if use_shadow_prompt_plan {
        if force_override && !same_argv {
            "force_commit_prompt_mismatch".to_string()
        } else {
            "commit_same_prompt_argv".to_string()
        }
    } else if !gate_ok {
        "gate_not_allowed_to_commit".to_string()
    } else if !kind_ok {
        "unsupported_legacy_prompt_kind".to_string()
    } else if !argv_ok {
        "prompt_argv_not_equal".to_string()
    } else {
        "prompt_commit_policy_rejected".to_string()
    };
    ShadowPromptCommitDecision {
        morphism: "shadow_pipeline.prompt_commit".to_string(),
        use_shadow_prompt_plan,
        reason,
        switch_mode: config.mode.canonical().to_string(),
        gate_reason: commit_gate.reason,
        gate_allowed_to_commit: commit_gate.allowed_to_commit,
        legacy_kind: legacy.kind.clone(),
        same_argv,
        force_override,
        planned_argv: report.planned_argv.clone(),
        legacy_argv: legacy.argv.clone(),
        rollback_anchor: config.rollback_anchor.clone(),
        universal_property:
            "prompt_shadow_plan_commits_only_when_legacy_compile_and_rust_prompt_execution_commute"
                .to_string(),
    }
}

fn table_report_from_render(
    input: &ShadowTableInput,
    config: &ArchitectureSwitchConfig,
    gate: SwitchGateDecision,
    prepared: PreparedTable,
    render_result: TableRenderResult,
) -> ShadowTableReport {
    let rendered_lines = render_result.resulting_table;
    let diff = diff_shadow_lines(&input.legacy_display_lines, &rendered_lines);
    let commit_gate = config.gate_for_morphism("shadow_pipeline.table_commit");
    let commit_candidate = commit_gate.allowed_to_commit
        && (diff.equal || config.mode == ArchitectureSwitchMode::Force);
    ShadowTableReport {
        morphism: "shadow_pipeline.table_adapter".to_string(),
        gate: gate.clone(),
        switch_mode: config.mode.canonical().to_string(),
        legacy_rows: input.legacy_display_lines.len(),
        prepared_rows: prepared.len(),
        rendered_rows: rendered_lines.len(),
        selected_columns: input.selected_columns_zero_based.len(),
        output_mode: input.mode.canonical_name().to_string(),
        diff,
        rendered_preview: rendered_lines.iter().take(8).cloned().collect(),
        rendered_lines,
        commit_candidate,
        universal_property: "legacy_visible_lines_and_rust_shadow_lines_are_compared_before_commit"
            .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_summary_finds_first_mismatch() {
        let diff = ShadowDiffSummary::from_lines(
            "a",
            "b",
            &["same".to_string(), "left".to_string()],
            &["same".to_string(), "right".to_string()],
        );
        assert!(!diff.equal);
        assert_eq!(diff.first_mismatch_index, Some(1));
        assert_eq!(diff.left_at_mismatch.as_deref(), Some("left"));
    }

    #[test]
    fn shadow_table_renders_preview_without_commit_in_dry_run() {
        let input = ShadowTableInput {
            content_table: vec![vec!["head".to_string()], vec!["value".to_string()]],
            legacy_display_lines: vec!["legacy".to_string()],
            mode: OutputMode::Shell,
            color: false,
            ..ShadowTableInput::empty_for_mode(OutputMode::Shell)
        };
        let config = ArchitectureSwitchConfig::default().with_mode(
            crate::runtime_switch::ArchitectureSwitchMode::DryRun,
            "test",
        );
        let report = bootstrap_shadow_pipeline().shadow_table(&input, &config);
        assert_eq!(report.switch_mode, "dry-run");
        assert!(report.rendered_rows > 0);
        assert!(!report.commit_candidate);
    }

    #[test]
    fn commit_policy_requires_equal_shadow_in_commit_mode() {
        let mut report = ShadowTableReport {
            morphism: "shadow_pipeline.table_adapter".to_string(),
            gate: SwitchGateDecision::allowed(
                "shadow_pipeline.table_adapter",
                "commit_gate",
                ArchitectureSwitchMode::Commit,
            ),
            switch_mode: "commit".to_string(),
            legacy_rows: 1,
            prepared_rows: 1,
            rendered_rows: 1,
            selected_columns: 1,
            output_mode: "shell".to_string(),
            diff: ShadowDiffSummary::from_lines(
                "legacy",
                "rust_shadow",
                &["a".to_string()],
                &["b".to_string()],
            ),
            rendered_lines: vec!["b".to_string()],
            rendered_preview: vec!["b".to_string()],
            commit_candidate: false,
            universal_property: "test".to_string(),
        };
        let config =
            ArchitectureSwitchConfig::default().with_mode(ArchitectureSwitchMode::Commit, "test");
        let decision = evaluate_shadow_table_commit(&report, &config, &Default::default());
        assert!(!decision.use_shadow_output);
        assert_eq!(decision.reason, "shadow_diff_not_equal");

        report.diff = ShadowDiffSummary::from_lines(
            "legacy",
            "rust_shadow",
            &["b".to_string()],
            &["b".to_string()],
        );
        let decision = evaluate_shadow_table_commit(&report, &config, &Default::default());
        assert!(decision.use_shadow_output);
        assert_eq!(decision.reason, "commit_equal_shadow");
    }

    #[test]
    fn table_view_output_commit_requires_equal_diff() {
        let mut report = ShadowTableViewOutputReport {
            morphism: "shadow_pipeline.table_view_output_adapter".to_string(),
            gate: SwitchGateDecision::allowed(
                "table_view_output.render",
                "commit_gate",
                ArchitectureSwitchMode::Commit,
            ),
            switch_mode: "commit".to_string(),
            legacy_rows: 1,
            rendered_rows: 1,
            output_mode: "shell".to_string(),
            diff: ShadowDiffSummary::from_lines(
                "legacy",
                "rust_table_view_output",
                &["legacy".to_string()],
                &["view".to_string()],
            ),
            semantic_diff: crate::table_view_output_parity::compare_output_lines(
                &["legacy".to_string()],
                &["view".to_string()],
                &crate::table_view_output_parity::TableViewOutputParityConfig::default(),
            ),
            virtual_column_parity: crate::table_view_virtual_parity::compare_virtual_column_policies_for_cli_args(
                &["reta", "-spalten", "--kontinuum=m"],
                &crate::table_view_virtual_parity::TableViewVirtualParityConfig::default(),
            ),
            language_parity: crate::table_view_language_parity::language_parity_for_cli_args(
                &["reta", "-language=english", "-spalten", "--kontinuum=m"],
                &crate::table_view_language_parity::TableViewLanguageParityPolicy::default(),
            ),
            language_coverage: crate::table_view_language_coverage::language_coverage_for_cli_args(
                &["reta", "-language=english", "-spalten", "--kontinuum=m"],
                &crate::table_view_language_coverage::TableViewLanguageCoveragePolicy::default(),
            ),
            output_report: TableViewOutputReport {
                class: "TableViewOutputReport".to_string(),
                mode: "shell".to_string(),
                row_count: 1,
                rendered_row_count: 1,
                cell_count: 1,
                virtual_cell_count: 0,
                rendered_line_count: 1,
                rendered_lines: vec!["view".to_string()],
                rendered_text: "view".to_string(),
                table_view_policy: "suppress".to_string(),
                virtual_column_policy: "suppress".to_string(),
                suppress_question_mark_virtuals: true,
                virtual_column_option_count: 0,
                suppress_headers: false,
                include_empty_rows: true,
                include_row_numbers: false,
                numbering_mode: "disabled".to_string(),
                numbering_column_count: 0,
                layout_enabled: false,
                layout_page_count: 0,
                layout_column_count: 0,
                layout_column_widths: Vec::new(),
                html_attribute_enabled: false,
                html_attribute_policy: "plain".to_string(),
                html_attribute_cell_count: 0,
                html_attribute_class_cell_count: 0,
                html_attribute_report: None,
                row_style_enabled: false,
                row_style_policy: "plain".to_string(),
                row_style_row_count: 0,
                row_style_colored_row_count: 0,
                row_style_report: None,
                cell_style_enabled: false,
                cell_style_policy: "plain".to_string(),
                cell_style_cell_count: 0,
                cell_style_styled_cell_count: 0,
                cell_style_virtual_cell_count: 0,
                cell_style_report: None,
                shell_style_enabled: false,
                shell_style_policy: "plain".to_string(),
                shell_style_cell_count: 0,
                shell_style_ansi_cell_count: 0,
                shell_style_report: None,
                html_cell_style_composition_enabled: false,
                html_cell_style_composition_count: 0,
                html_attribute_only_cell_count: 0,
                wrap_cell_width: None,
                per_column_width_count: 0,
                dontwrap: false,
                nocolor: false,
                justtext: false,
                onetable: false,
                endlessscreen: false,
                endless: false,
                cli_options: crate::table_view_output::TableViewOutputCliOptions::default(),
                continuum_m_direct_header_present: false,
                continuum_m_virtual_744_kept_as_witness: false,
                visible_output_is_empty: false,
                universal_property: "test".to_string(),
            },
            rendered_preview: vec!["view".to_string()],
            commit_candidate: false,
            universal_property: "test".to_string(),
        };
        let config =
            ArchitectureSwitchConfig::default().with_mode(ArchitectureSwitchMode::Commit, "test");
        let decision =
            evaluate_shadow_table_view_output_commit(&report, &config, &Default::default());
        assert!(!decision.use_view_output);
        assert_eq!(decision.reason, "table_view_output_diff_not_equal");

        report.diff = ShadowDiffSummary::from_lines(
            "legacy",
            "rust_table_view_output",
            &["view".to_string()],
            &["view".to_string()],
        );
        let decision =
            evaluate_shadow_table_view_output_commit(&report, &config, &Default::default());
        assert!(decision.use_view_output);
        assert_eq!(decision.reason, "commit_equal_table_view_output");
        assert!(decision.virtual_direct_cells_equal);
    }

    #[test]
    fn prompt_commit_requires_same_reta_argv() {
        let report = ShadowPromptReport {
            morphism: "shadow_pipeline.prompt_adapter".to_string(),
            gate: SwitchGateDecision::allowed(
                "shadow_pipeline.prompt_adapter",
                "commit_gate",
                ArchitectureSwitchMode::Commit,
            ),
            switch_mode: "commit".to_string(),
            prepared_token_count: 3,
            execution_argv_count: 3,
            completion_preview_count: 0,
            planned_argv: vec![
                "reta".to_string(),
                "-zeilen".to_string(),
                "--alles".to_string(),
            ],
            completion_preview: Vec::new(),
            commit_candidate: true,
            universal_property: "test".to_string(),
        };
        let config =
            ArchitectureSwitchConfig::default().with_mode(ArchitectureSwitchMode::Commit, "test");
        let same = ShadowPromptLegacyCommand::reta(vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--alles".to_string(),
        ]);
        let decision = evaluate_shadow_prompt_commit(&report, &same, &config, &Default::default());
        assert!(decision.use_shadow_prompt_plan);
        assert_eq!(decision.reason, "commit_same_prompt_argv");

        let different = ShadowPromptLegacyCommand::reta(vec![
            "reta".to_string(),
            "-spalten".to_string(),
            "--alles".to_string(),
        ]);
        let decision =
            evaluate_shadow_prompt_commit(&report, &different, &config, &Default::default());
        assert!(!decision.use_shadow_prompt_plan);
        assert_eq!(decision.reason, "prompt_argv_not_equal");

        let dry_run =
            ArchitectureSwitchConfig::default().with_mode(ArchitectureSwitchMode::DryRun, "test");
        let decision = evaluate_shadow_prompt_commit(&report, &same, &dry_run, &Default::default());
        assert!(!decision.use_shadow_prompt_plan);
        assert_eq!(decision.reason, "gate_not_allowed_to_commit");
    }

    #[test]
    fn force_can_commit_mismatch_but_dry_run_cannot() {
        let report = ShadowTableReport {
            morphism: "shadow_pipeline.table_adapter".to_string(),
            gate: SwitchGateDecision::allowed(
                "shadow_pipeline.table_adapter",
                "force_gate",
                ArchitectureSwitchMode::Force,
            ),
            switch_mode: "force".to_string(),
            legacy_rows: 1,
            prepared_rows: 1,
            rendered_rows: 1,
            selected_columns: 1,
            output_mode: "shell".to_string(),
            diff: ShadowDiffSummary::from_lines(
                "legacy",
                "rust_shadow",
                &["legacy".to_string()],
                &["shadow".to_string()],
            ),
            rendered_lines: vec!["shadow".to_string()],
            rendered_preview: vec!["shadow".to_string()],
            commit_candidate: true,
            universal_property: "test".to_string(),
        };
        let force =
            ArchitectureSwitchConfig::default().with_mode(ArchitectureSwitchMode::Force, "test");
        assert!(
            evaluate_shadow_table_commit(&report, &force, &Default::default()).use_shadow_output
        );

        let dry_run =
            ArchitectureSwitchConfig::default().with_mode(ArchitectureSwitchMode::DryRun, "test");
        assert!(
            !evaluate_shadow_table_commit(&report, &dry_run, &Default::default()).use_shadow_output
        );
    }
}
