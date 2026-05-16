//! Guarded commit audit for materialized table-view output.
//!
//! The shadow pipeline already computes raw line diffs, semantic diffs and the
//! virtual-column identity guard.  This module glues those local witnesses into
//! one typed audit record so a future visible commit can be explained without
//! re-running or guessing which guard passed or failed.

use serde::{Deserialize, Serialize};

use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::shadow_pipeline::{
    ShadowTableViewOutputCommitDecision, ShadowTableViewOutputReport, bootstrap_shadow_pipeline,
};
use crate::table_view_language_coverage::TableViewLanguageCoverageReport;
use crate::table_view_language_parity::TableViewLanguageParityReport;
use crate::table_view_language_sync::TableViewLanguageSyncReport;
use crate::table_view_output_parity::TableViewOutputParityReport;
use crate::table_view_virtual_parity::TableViewVirtualParityReport;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewCommitAuditCheck {
    pub name: String,
    pub required: bool,
    pub passed: bool,
    pub value: String,
    pub reason: String,
}

impl TableViewCommitAuditCheck {
    pub fn new(
        name: impl Into<String>,
        required: bool,
        passed: bool,
        value: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            required,
            passed,
            value: value.into(),
            reason: reason.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewCommitAuditReport {
    pub class: String,
    pub morphism: String,
    pub switch_mode: String,
    pub commit_reason: String,
    pub use_view_output: bool,
    pub safe_to_commit: bool,
    pub force_override: bool,
    pub required_check_count: usize,
    pub passed_required_check_count: usize,
    pub failed_required_checks: Vec<String>,
    pub diagnostic_check_count: usize,
    pub raw_equal: bool,
    pub semantic_equal: bool,
    pub virtual_direct_cells_equal: bool,
    pub virtual_added_column_count: usize,
    pub language_parity_ready: bool,
    pub language_requested_language: String,
    pub language_effective_asset_name: String,
    pub language_fallback_applied: bool,
    pub language_failed_guards: Vec<String>,
    pub language_coverage_ready: bool,
    pub language_coverage_status: String,
    pub language_coverage_stale_language_count: usize,
    pub language_coverage_languages_missing_744: Vec<String>,
    pub language_coverage_failed_guards: Vec<String>,
    pub language_sync_ready: bool,
    pub language_sync_status: String,
    pub language_sync_pending_action_count: usize,
    pub language_sync_pending_languages: Vec<String>,
    pub language_sync_pending_columns: Vec<usize>,
    pub language_sync_failed_guards: Vec<String>,
    pub rendered_line_count: usize,
    pub legacy_line_count: usize,
    pub first_raw_mismatch_index: Option<usize>,
    pub first_semantic_mismatch_index: Option<usize>,
    pub virtual_rendered_policy: String,
    pub rollback_anchor: Option<String>,
    pub checks: Vec<TableViewCommitAuditCheck>,
    pub semantic_diff: TableViewOutputParityReport,
    pub virtual_column_parity: TableViewVirtualParityReport,
    pub language_parity: TableViewLanguageParityReport,
    pub language_coverage: TableViewLanguageCoverageReport,
    pub language_sync: TableViewLanguageSyncReport,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewCommitAuditSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub required_guards: Vec<String>,
    pub diagnostic_guards: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewCommitAuditBundle;

impl TableViewCommitAuditBundle {
    pub fn snapshot(&self) -> TableViewCommitAuditSnapshot {
        TableViewCommitAuditSnapshot {
            class: "TableViewCommitAuditSnapshot".to_string(),
            morphisms: vec![
                "table_view_commit_audit.audit_report".to_string(),
                "table_view_commit_audit.required_guards".to_string(),
                "table_view_commit_audit.semantic_witness".to_string(),
                "table_view_commit_audit.virtual_direct_identity".to_string(),
                "table_view_commit_audit.cli_shadow_audit".to_string(),
            ],
            required_guards: vec![
                "commit_gate_allowed".to_string(),
                "raw_line_diff_equal_or_force".to_string(),
                "virtual_direct_cells_equal".to_string(),
                "language_parity_ready".to_string(),
                "language_coverage_ready".to_string(),
                "language_sync_ready".to_string(),
                "decision_uses_view_output".to_string(),
            ],
            diagnostic_guards: vec![
                "semantic_rows_equal".to_string(),
                "virtual_added_columns".to_string(),
                "language_fallback_witness".to_string(),
                "language_coverage_gap_report".to_string(),
                "language_sync_backlog_report".to_string(),
                "rollback_anchor".to_string(),
            ],
            universal_property:
                "a_visible_table_view_commit_is_the_unique_shadow_projection_whose_required_guards_commute"
                    .to_string(),
        }
    }

    pub fn audit(
        &self,
        report: &ShadowTableViewOutputReport,
        decision: &ShadowTableViewOutputCommitDecision,
    ) -> TableViewCommitAuditReport {
        audit_table_view_output_commit(report, decision)
    }

    pub fn audit_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        config: &ArchitectureSwitchConfig,
    ) -> TableViewCommitAuditReport {
        audit_table_view_output_for_cli_args(args, legacy_lines, config)
    }
}

pub fn bootstrap_table_view_commit_audit() -> TableViewCommitAuditBundle {
    TableViewCommitAuditBundle
}

pub fn audit_table_view_output_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    config: &ArchitectureSwitchConfig,
) -> TableViewCommitAuditReport {
    let pipeline = bootstrap_shadow_pipeline();
    let report = pipeline.shadow_table_view_output(args, legacy_lines, config);
    let decision = pipeline.table_view_output_commit_decision(&report, config);
    audit_table_view_output_commit(&report, &decision)
}

pub fn audit_table_view_output_commit(
    report: &ShadowTableViewOutputReport,
    decision: &ShadowTableViewOutputCommitDecision,
) -> TableViewCommitAuditReport {
    let raw_guard = decision.diff_equal || decision.force_override;
    let virtual_direct_guard = decision.virtual_direct_cells_equal;
    let language_guard = decision.language_parity_ready;
    let language_coverage_guard = decision.language_coverage_ready;
    let language_sync_guard = decision.language_sync_ready;
    let gate_guard = decision.gate_allowed_to_commit;
    let decision_guard = decision.use_view_output;

    let checks = vec![
        TableViewCommitAuditCheck::new(
            "commit_gate_allowed",
            true,
            gate_guard,
            decision.gate_reason.clone(),
            "runtime switch must allow the table-view-output commit gate",
        ),
        TableViewCommitAuditCheck::new(
            "raw_line_diff_equal_or_force",
            true,
            raw_guard,
            format!(
                "raw_equal={} force_override={}",
                decision.diff_equal, decision.force_override
            ),
            "visible output bytes/lines must match unless explicit force mode is used",
        ),
        TableViewCommitAuditCheck::new(
            "virtual_direct_cells_equal",
            true,
            virtual_direct_guard,
            format!(
                "policy={} added_virtual_columns={}",
                decision.virtual_rendered_policy, decision.virtual_added_column_count
            ),
            "virtual-column policies may add witnesses but must preserve every direct CSV cell",
        ),
        TableViewCommitAuditCheck::new(
            "language_parity_ready",
            true,
            language_guard,
            format!(
                "requested_language={} effective_asset={} fallback_applied={} failed_guards={:?}",
                decision.language_requested_language,
                decision.language_effective_asset_name,
                decision.language_fallback_applied,
                decision.language_failed_guards
            ),
            "localized table sections may commit only when missing requested direct columns fallback to a safe base asset",
        ),
        TableViewCommitAuditCheck::new(
            "language_coverage_ready",
            true,
            language_coverage_guard,
            format!(
                "status={} stale_languages={} missing_744={:?} failed_guards={:?}",
                decision.language_coverage_status,
                decision.language_coverage_stale_language_count,
                decision.language_coverage_languages_missing_744,
                decision.language_coverage_failed_guards
            ),
            "language coverage must show that requested direct columns are covered by the effective asset or safe fallback",
        ),
        TableViewCommitAuditCheck::new(
            "language_sync_ready",
            true,
            language_sync_guard,
            format!(
                "status={} pending_actions={} pending_languages={:?} pending_columns={:?} failed_guards={:?}",
                decision.language_sync_status,
                decision.language_sync_pending_action_count,
                decision.language_sync_pending_languages,
                decision.language_sync_pending_columns,
                decision.language_sync_failed_guards
            ),
            "language synchronization backlog must be empty before a localized table-view output can be promoted",
        ),
        TableViewCommitAuditCheck::new(
            "decision_uses_view_output",
            true,
            decision_guard,
            decision.reason.clone(),
            "the shadow pipeline commit decision must itself select the materialized view output",
        ),
        TableViewCommitAuditCheck::new(
            "semantic_rows_equal",
            false,
            decision.semantic_equal,
            format!(
                "semantic_first_mismatch={:?}",
                report.semantic_diff.first_semantic_mismatch_index
            ),
            "semantic equality is diagnostic until raw commit policy is relaxed",
        ),
        TableViewCommitAuditCheck::new(
            "virtual_added_columns_are_witnesses",
            false,
            report.virtual_column_parity.direct_cells_equal,
            format!(
                "added_columns={:?}",
                report.virtual_column_parity.added_virtual_columns
            ),
            "added virtual columns are valid only as witnesses around unchanged direct cells",
        ),
        TableViewCommitAuditCheck::new(
            "language_fallback_witness",
            false,
            report.language_parity.fallback_applied,
            format!(
                "{} -> {}",
                report.language_parity.requested_asset_name,
                report.language_parity.effective_asset_name
            ),
            "fallback to the base CSV is diagnostic unless a localized asset lacks requested direct columns",
        ),
        TableViewCommitAuditCheck::new(
            "language_coverage_gap_report",
            false,
            report.language_coverage.ready(),
            format!(
                "stale_languages={} missing_744={:?}",
                report.language_coverage.stale_language_count,
                report.language_coverage.languages_missing_744
            ),
            "language coverage is diagnostic for synchronization gaps even when fallback keeps output safe",
        ),
        TableViewCommitAuditCheck::new(
            "language_sync_backlog_report",
            false,
            report.language_sync.ready(),
            format!(
                "pending_actions={} pending_languages={:?} pending_columns={:?}",
                report.language_sync.pending_action_count,
                report.language_sync.pending_languages,
                report.language_sync.pending_columns
            ),
            "language sync report is diagnostic after all translations cover requested direct columns",
        ),
        TableViewCommitAuditCheck::new(
            "rollback_anchor_recorded",
            false,
            decision.rollback_anchor.is_some(),
            decision
                .rollback_anchor
                .clone()
                .unwrap_or_else(|| "<none>".to_string()),
            "rollback anchors are optional but useful for activation transactions",
        ),
    ];

    let failed_required_checks = checks
        .iter()
        .filter(|check| check.required && !check.passed)
        .map(|check| check.name.clone())
        .collect::<Vec<_>>();
    let required_check_count = checks.iter().filter(|check| check.required).count();
    let passed_required_check_count = checks
        .iter()
        .filter(|check| check.required && check.passed)
        .count();
    let diagnostic_check_count = checks.iter().filter(|check| !check.required).count();
    let safe_to_commit = failed_required_checks.is_empty() && decision.use_view_output;

    TableViewCommitAuditReport {
        class: "TableViewCommitAuditReport".to_string(),
        morphism: "table_view_commit_audit.audit_report".to_string(),
        switch_mode: decision.switch_mode.clone(),
        commit_reason: decision.reason.clone(),
        use_view_output: decision.use_view_output,
        safe_to_commit,
        force_override: decision.force_override,
        required_check_count,
        passed_required_check_count,
        failed_required_checks,
        diagnostic_check_count,
        raw_equal: decision.diff_equal,
        semantic_equal: decision.semantic_equal,
        virtual_direct_cells_equal: decision.virtual_direct_cells_equal,
        virtual_added_column_count: decision.virtual_added_column_count,
        language_parity_ready: decision.language_parity_ready,
        language_requested_language: decision.language_requested_language.clone(),
        language_effective_asset_name: decision.language_effective_asset_name.clone(),
        language_fallback_applied: decision.language_fallback_applied,
        language_failed_guards: decision.language_failed_guards.clone(),
        language_coverage_ready: decision.language_coverage_ready,
        language_coverage_status: decision.language_coverage_status.clone(),
        language_coverage_stale_language_count: decision.language_coverage_stale_language_count,
        language_coverage_languages_missing_744: decision.language_coverage_languages_missing_744.clone(),
        language_coverage_failed_guards: decision.language_coverage_failed_guards.clone(),
        language_sync_ready: decision.language_sync_ready,
        language_sync_status: decision.language_sync_status.clone(),
        language_sync_pending_action_count: decision.language_sync_pending_action_count,
        language_sync_pending_languages: decision.language_sync_pending_languages.clone(),
        language_sync_pending_columns: decision.language_sync_pending_columns.clone(),
        language_sync_failed_guards: decision.language_sync_failed_guards.clone(),
        rendered_line_count: decision.rendered_line_count,
        legacy_line_count: report.legacy_rows,
        first_raw_mismatch_index: report.diff.first_mismatch_index,
        first_semantic_mismatch_index: report.semantic_diff.first_semantic_mismatch_index,
        virtual_rendered_policy: decision.virtual_rendered_policy.clone(),
        rollback_anchor: decision.rollback_anchor.clone(),
        checks,
        semantic_diff: report.semantic_diff.clone(),
        virtual_column_parity: report.virtual_column_parity.clone(),
        language_parity: report.language_parity.clone(),
        language_coverage: report.language_coverage.clone(),
        language_sync: report.language_sync.clone(),
        universal_property:
            "commit_audit_glues_raw_diff_semantic_diff_virtual_identity_language_parity_language_coverage_and_language_sync_into_one_guarded_witness"
                .to_string(),
    }
}

pub fn continuum_m_commit_audit_smoke() -> TableViewCommitAuditReport {
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "-ausgabe".to_string(),
        "--spaltenreihenfolgeundnurdiese=744,493".to_string(),
        "--virtualcolumns".to_string(),
        "--breite=0".to_string(),
    ];
    let config = ArchitectureSwitchConfig::default();
    audit_table_view_output_for_cli_args(&args, &[], &config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_audit_keeps_virtual_guard_visible() {
        let audit = continuum_m_commit_audit_smoke();
        assert_eq!(audit.class, "TableViewCommitAuditReport");
        assert!(audit
            .checks
            .iter()
            .any(|check| check.name == "virtual_direct_cells_equal"));
        assert!(audit
            .checks
            .iter()
            .any(|check| check.name == "language_parity_ready"));
        assert!(audit.checks
            .iter()
            .any(|check| check.name == "language_coverage_ready"));
        assert!(audit.checks
            .iter()
            .any(|check| check.name == "language_sync_ready"));
        assert_eq!(audit.virtual_rendered_policy, "tag-summary");
        assert!(!audit.safe_to_commit);
    }
}
