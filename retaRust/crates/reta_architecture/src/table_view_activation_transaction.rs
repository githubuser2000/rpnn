//! Activation transactions for guarded materialized table-view output.
//!
//! Stage 40 made the commit decision auditable.  This module turns that audit
//! into a typed activation transaction: a deterministic record that says which
//! visible line source would be used, why it is safe or blocked, and which
//! rollback witness belongs to the decision.  The default transaction is still
//! conservative: it selects the Rust materialized `TableViewOutput` only when
//! the commit decision and the audit both agree.

use serde::{Deserialize, Serialize};

use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::shadow_pipeline::{
    ShadowTableViewOutputCommitDecision, ShadowTableViewOutputReport, bootstrap_shadow_pipeline,
};
use crate::table_view_commit_audit::{
    TableViewCommitAuditReport, audit_table_view_output_commit,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum TableViewActivationLineSource {
    LegacyOutput,
    TableViewOutput,
}

impl TableViewActivationLineSource {
    pub fn canonical(self) -> &'static str {
        match self {
            Self::LegacyOutput => "legacy_output",
            Self::TableViewOutput => "table_view_output",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationTransactionPolicy {
    pub prefer_table_view_output: bool,
    pub require_audit_safe: bool,
    pub require_commit_decision: bool,
    pub keep_legacy_lines_on_reject: bool,
    pub preview_limit: usize,
}

impl Default for TableViewActivationTransactionPolicy {
    fn default() -> Self {
        Self {
            prefer_table_view_output: true,
            require_audit_safe: true,
            require_commit_decision: true,
            keep_legacy_lines_on_reject: true,
            preview_limit: 8,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationTransactionReport {
    pub class: String,
    pub morphism: String,
    pub transaction_id: String,
    pub switch_mode: String,
    pub selected_source: String,
    pub should_replace_visible_output: bool,
    pub safe_to_apply: bool,
    pub reason: String,
    pub commit_reason: String,
    pub audit_safe: bool,
    pub commit_decision_allows_view_output: bool,
    pub required_failed_count: usize,
    pub failed_required_checks: Vec<String>,
    pub selected_line_count: usize,
    pub legacy_line_count: usize,
    pub view_output_line_count: usize,
    pub selected_lines_checksum: u64,
    pub legacy_lines_checksum: u64,
    pub view_output_lines_checksum: u64,
    pub selected_lines_preview: Vec<String>,
    pub selected_lines: Vec<String>,
    pub rollback_anchor: Option<String>,
    pub audit: TableViewCommitAuditReport,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationTransactionSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub required_inputs: Vec<String>,
    pub output_sources: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationTransactionBundle;

impl TableViewActivationTransactionBundle {
    pub fn snapshot(&self) -> TableViewActivationTransactionSnapshot {
        TableViewActivationTransactionSnapshot {
            class: "TableViewActivationTransactionSnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_transaction.select_visible_source".to_string(),
                "table_view_activation_transaction.rollback_witness".to_string(),
                "table_view_activation_transaction.commit_audit_gate".to_string(),
                "table_view_activation_transaction.selected_lines_checksum".to_string(),
            ],
            required_inputs: vec![
                "ShadowTableViewOutputReport".to_string(),
                "ShadowTableViewOutputCommitDecision".to_string(),
                "TableViewCommitAuditReport".to_string(),
                "legacy_visible_lines".to_string(),
            ],
            output_sources: vec![
                TableViewActivationLineSource::LegacyOutput.canonical().to_string(),
                TableViewActivationLineSource::TableViewOutput.canonical().to_string(),
            ],
            universal_property:
                "visible_output_source_is_selected_by_the_unique_safe_commit_transaction".to_string(),
        }
    }

    pub fn transaction(
        &self,
        report: &ShadowTableViewOutputReport,
        decision: &ShadowTableViewOutputCommitDecision,
        legacy_lines: &[String],
        policy: &TableViewActivationTransactionPolicy,
    ) -> TableViewActivationTransactionReport {
        table_view_activation_transaction(report, decision, legacy_lines, policy)
    }

    pub fn transaction_for_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        config: &ArchitectureSwitchConfig,
        policy: &TableViewActivationTransactionPolicy,
    ) -> TableViewActivationTransactionReport {
        table_view_activation_transaction_for_cli_args(args, legacy_lines, config, policy)
    }
}

pub fn bootstrap_table_view_activation_transaction() -> TableViewActivationTransactionBundle {
    TableViewActivationTransactionBundle
}

pub fn table_view_activation_transaction_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationTransactionPolicy,
) -> TableViewActivationTransactionReport {
    let pipeline = bootstrap_shadow_pipeline();
    let report = pipeline.shadow_table_view_output(args, legacy_lines, config);
    let decision = pipeline.table_view_output_commit_decision(&report, config);
    table_view_activation_transaction(&report, &decision, legacy_lines, policy)
}

pub fn table_view_activation_transaction(
    report: &ShadowTableViewOutputReport,
    decision: &ShadowTableViewOutputCommitDecision,
    legacy_lines: &[String],
    policy: &TableViewActivationTransactionPolicy,
) -> TableViewActivationTransactionReport {
    let audit = audit_table_view_output_commit(report, decision);
    let audit_ok = !policy.require_audit_safe || audit.safe_to_commit;
    let decision_ok = !policy.require_commit_decision || decision.use_view_output;
    let should_replace_visible_output = policy.prefer_table_view_output && audit_ok && decision_ok;
    let selected_source = if should_replace_visible_output {
        TableViewActivationLineSource::TableViewOutput
    } else {
        TableViewActivationLineSource::LegacyOutput
    };
    let selected_lines = if should_replace_visible_output {
        report.output_report.rendered_lines.clone()
    } else if policy.keep_legacy_lines_on_reject {
        legacy_lines.to_vec()
    } else {
        Vec::new()
    };
    let reason = if should_replace_visible_output {
        "activate_table_view_output".to_string()
    } else if !policy.prefer_table_view_output {
        "policy_prefers_legacy_output".to_string()
    } else if !decision_ok {
        format!("commit_decision_rejected:{}", decision.reason)
    } else if !audit_ok {
        format!("audit_not_safe:{:?}", audit.failed_required_checks)
    } else {
        "activation_policy_rejected".to_string()
    };
    let selected_lines_preview = selected_lines
        .iter()
        .take(policy.preview_limit)
        .cloned()
        .collect::<Vec<_>>();
    let selected_lines_checksum = stable_line_checksum(&selected_lines);
    let legacy_lines_checksum = stable_line_checksum(legacy_lines);
    let view_output_lines_checksum = stable_line_checksum(&report.output_report.rendered_lines);
    let transaction_id = format!(
        "{}:{}:{}:{}:{}",
        report.switch_mode,
        selected_source.canonical(),
        legacy_lines.len(),
        report.output_report.rendered_lines.len(),
        selected_lines_checksum,
    );
    TableViewActivationTransactionReport {
        class: "TableViewActivationTransactionReport".to_string(),
        morphism: "table_view_activation_transaction.select_visible_source".to_string(),
        transaction_id,
        switch_mode: report.switch_mode.clone(),
        selected_source: selected_source.canonical().to_string(),
        should_replace_visible_output,
        safe_to_apply: should_replace_visible_output && audit.safe_to_commit,
        reason,
        commit_reason: decision.reason.clone(),
        audit_safe: audit.safe_to_commit,
        commit_decision_allows_view_output: decision.use_view_output,
        required_failed_count: audit.failed_required_checks.len(),
        failed_required_checks: audit.failed_required_checks.clone(),
        selected_line_count: selected_lines.len(),
        legacy_line_count: legacy_lines.len(),
        view_output_line_count: report.output_report.rendered_lines.len(),
        selected_lines_checksum,
        legacy_lines_checksum,
        view_output_lines_checksum,
        selected_lines_preview,
        selected_lines,
        rollback_anchor: decision.rollback_anchor.clone(),
        audit,
        universal_property:
            "a_visible_activation_transaction_factors_through_a_safe_commit_audit".to_string(),
    }
}

pub fn stable_line_checksum(lines: &[String]) -> u64 {
    // 64-bit FNV-1a over UTF-8 bytes plus line separators.  This is a stable
    // witness, not a cryptographic checksum.
    let mut hash = 0xcbf29ce484222325u64;
    for line in lines {
        for byte in line.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn continuum_m_activation_transaction_smoke() -> TableViewActivationTransactionReport {
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "--breite=0".to_string(),
    ];
    let config = ArchitectureSwitchConfig::default()
        .with_mode(crate::runtime_switch::ArchitectureSwitchMode::Commit, "stage41-smoke");
    let pipeline = bootstrap_shadow_pipeline();
    let first_report = pipeline.shadow_table_view_output(&args, &[], &config);
    let legacy_lines = first_report.output_report.rendered_lines.clone();
    let report = pipeline.shadow_table_view_output(&args, &legacy_lines, &config);
    let decision = pipeline.table_view_output_commit_decision(&report, &config);
    table_view_activation_transaction(
        &report,
        &decision,
        &legacy_lines,
        &TableViewActivationTransactionPolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_changes_when_lines_change() {
        assert_ne!(
            stable_line_checksum(&["a".to_string()]),
            stable_line_checksum(&["b".to_string()]),
        );
    }

    #[test]
    fn activation_transaction_uses_view_output_only_when_audit_is_safe() {
        let tx = continuum_m_activation_transaction_smoke();
        assert!(tx.audit_safe);
        assert!(tx.should_replace_visible_output);
        assert_eq!(tx.selected_source, "table_view_output");
        assert_eq!(tx.required_failed_count, 0);
    }

    #[test]
    fn activation_transaction_keeps_legacy_lines_when_diff_fails() {
        let args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
            "--breite=0".to_string(),
        ];
        let config = ArchitectureSwitchConfig::default()
            .with_mode(crate::runtime_switch::ArchitectureSwitchMode::Commit, "test");
        let legacy_lines = vec!["legacy line that does not match".to_string()];
        let tx = table_view_activation_transaction_for_cli_args(
            &args,
            &legacy_lines,
            &config,
            &TableViewActivationTransactionPolicy::default(),
        );
        assert!(!tx.should_replace_visible_output);
        assert_eq!(tx.selected_source, "legacy_output");
        assert_eq!(tx.selected_lines, legacy_lines);
        assert!(tx.required_failed_count > 0);
    }
}
