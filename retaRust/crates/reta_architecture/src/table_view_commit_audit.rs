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
    pub rendered_line_count: usize,
    pub legacy_line_count: usize,
    pub first_raw_mismatch_index: Option<usize>,
    pub first_semantic_mismatch_index: Option<usize>,
    pub virtual_rendered_policy: String,
    pub rollback_anchor: Option<String>,
    pub checks: Vec<TableViewCommitAuditCheck>,
    pub semantic_diff: TableViewOutputParityReport,
    pub virtual_column_parity: TableViewVirtualParityReport,
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
                "decision_uses_view_output".to_string(),
            ],
            diagnostic_guards: vec![
                "semantic_rows_equal".to_string(),
                "virtual_added_columns".to_string(),
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
        rendered_line_count: decision.rendered_line_count,
        legacy_line_count: report.legacy_rows,
        first_raw_mismatch_index: report.diff.first_mismatch_index,
        first_semantic_mismatch_index: report.semantic_diff.first_semantic_mismatch_index,
        virtual_rendered_policy: decision.virtual_rendered_policy.clone(),
        rollback_anchor: decision.rollback_anchor.clone(),
        checks,
        semantic_diff: report.semantic_diff.clone(),
        virtual_column_parity: report.virtual_column_parity.clone(),
        universal_property:
            "commit_audit_glues_raw_diff_semantic_diff_and_virtual_identity_into_one_guarded_witness"
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
        assert_eq!(audit.virtual_rendered_policy, "tag-summary");
        assert!(!audit.safe_to_commit);
    }
}
