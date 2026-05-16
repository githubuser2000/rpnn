//! Promotion-readiness report for guarded table-view activation.
//!
//! Stages 40-48 created many local witnesses: commit decisions, audits,
//! transactions, journals, ledgers, stores, persistence and file recovery.  This
//! module folds those local witnesses into one readable readiness report.  It
//! deliberately does not relax the visible-output commit rule.  Its job is to
//! explain whether a CLI case is ready for a future default promotion, which
//! guard still blocks it, and which visible line source would be selected.

use serde::{Deserialize, Serialize};

use crate::persistence::PersistenceStore;
use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::shadow_pipeline::{
    ShadowTableViewOutputCommitDecision, bootstrap_shadow_pipeline,
};
use crate::table_view_activation_journal::{
    TableViewActivationJournal, TableViewActivationJournalPolicy,
    activation_journal_from_transactions,
};
use crate::table_view_activation_ledger::{
    TableViewActivationLedger, TableViewActivationLedgerPolicy, activation_ledger_from_journal,
};
use crate::table_view_activation_persistence::{
    TableViewActivationPersistenceReport, TableViewActivationPersistencePolicy,
    persist_activation_store_to_persistence,
};
use crate::table_view_activation_recovery::{
    TableViewActivationRecoveryPolicy, TableViewActivationRecoveryReport,
    activation_recovery_for_cli_args, activation_recovery_policy_from_cli_args,
};
use crate::table_view_activation_replay::{
    TableViewActivationReplayPolicy, TableViewActivationReplayReport, activation_replay_from_journal,
};
use crate::table_view_activation_store::{
    TableViewActivationStore, TableViewActivationStorePolicy,
    activation_store_from_journal_and_ledger,
};
use crate::table_view_activation_transaction::{
    TableViewActivationTransactionPolicy, TableViewActivationTransactionReport,
    stable_line_checksum, table_view_activation_transaction,
};
use crate::table_view_commit_audit::{TableViewCommitAuditReport, audit_table_view_output_commit};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationReadinessPolicy {
    pub require_commit_decision: bool,
    pub require_commit_audit_safe: bool,
    pub require_transaction_safe: bool,
    pub require_transaction_replaces_visible_output: bool,
    pub require_journal_replayable: bool,
    pub require_replay_safe: bool,
    pub require_ledger_ready: bool,
    pub require_store_ready: bool,
    pub require_persistence_ready: bool,
    pub require_recovery_ready_when_enabled: bool,
    pub require_language_parity_ready: bool,
    pub require_language_coverage_ready: bool,
    pub include_selected_lines: bool,
    pub preview_limit: usize,
}

impl Default for TableViewActivationReadinessPolicy {
    fn default() -> Self {
        Self {
            require_commit_decision: true,
            require_commit_audit_safe: true,
            require_transaction_safe: true,
            require_transaction_replaces_visible_output: true,
            require_journal_replayable: true,
            require_replay_safe: true,
            require_ledger_ready: true,
            require_store_ready: true,
            require_persistence_ready: true,
            require_recovery_ready_when_enabled: false,
            require_language_parity_ready: true,
            require_language_coverage_ready: true,
            include_selected_lines: true,
            preview_limit: 8,
        }
    }
}


impl TableViewActivationReadinessPolicy {
    pub fn strict() -> Self {
        Self::default()
    }

    pub fn diagnostic() -> Self {
        Self {
            require_commit_decision: false,
            require_commit_audit_safe: false,
            require_transaction_safe: false,
            require_transaction_replaces_visible_output: false,
            require_journal_replayable: false,
            require_replay_safe: false,
            require_ledger_ready: false,
            require_store_ready: false,
            require_persistence_ready: false,
            require_recovery_ready_when_enabled: false,
            require_language_parity_ready: false,
            require_language_coverage_ready: false,
            include_selected_lines: true,
            preview_limit: 8,
        }
    }

    pub fn without_selected_lines(mut self) -> Self {
        self.include_selected_lines = false;
        self
    }

    pub fn with_preview_limit(mut self, limit: usize) -> Self {
        self.preview_limit = limit;
        self
    }

    pub fn from_cli_args(args: &[String], base: &Self) -> (Self, bool) {
        let mut policy = base.clone();
        let mut recognized = false;
        for arg in args {
            match arg.as_str() {
                "--activation-readiness-strict" | "--readiness-strict" => {
                    policy = Self::strict();
                    recognized = true;
                }
                "--activation-readiness-diagnostic" | "--readiness-diagnostic" => {
                    policy = Self::diagnostic();
                    recognized = true;
                }
                "--activation-readiness-no-selected-lines" | "--readiness-no-selected-lines" => {
                    policy.include_selected_lines = false;
                    recognized = true;
                }
                "--activation-readiness-include-selected-lines"
                | "--readiness-include-selected-lines" => {
                    policy.include_selected_lines = true;
                    recognized = true;
                }
                "--activation-readiness-require-recovery" | "--readiness-require-recovery" => {
                    policy.require_recovery_ready_when_enabled = true;
                    recognized = true;
                }
                "--activation-readiness-ignore-recovery" | "--readiness-ignore-recovery" => {
                    policy.require_recovery_ready_when_enabled = false;
                    recognized = true;
                }
                "--activation-readiness-require-persistence" | "--readiness-require-persistence" => {
                    policy.require_persistence_ready = true;
                    recognized = true;
                }
                "--activation-readiness-ignore-persistence" | "--readiness-ignore-persistence" => {
                    policy.require_persistence_ready = false;
                    recognized = true;
                }
                "--activation-readiness-require-language-parity"
                | "--readiness-require-language-parity" => {
                    policy.require_language_parity_ready = true;
                    recognized = true;
                }
                "--activation-readiness-ignore-language-parity"
                | "--readiness-ignore-language-parity" => {
                    policy.require_language_parity_ready = false;
                    recognized = true;
                }
                "--activation-readiness-require-language-coverage"
                | "--readiness-require-language-coverage" => {
                    policy.require_language_coverage_ready = true;
                    recognized = true;
                }
                "--activation-readiness-ignore-language-coverage"
                | "--readiness-ignore-language-coverage" => {
                    policy.require_language_coverage_ready = false;
                    recognized = true;
                }
                _ => {
                    if let Some(value) = arg
                        .strip_prefix("--activation-readiness-preview=")
                        .or_else(|| arg.strip_prefix("--readiness-preview="))
                    {
                        if let Ok(limit) = value.parse::<usize>() {
                            policy.preview_limit = limit;
                            recognized = true;
                        }
                    }
                }
            }
        }
        (policy, recognized)
    }

    pub fn required_guard_names(&self) -> Vec<&'static str> {
        let mut guards = Vec::new();
        if self.require_commit_decision {
            guards.push("shadow_commit_decision_uses_view_output");
        }
        if self.require_commit_audit_safe {
            guards.push("commit_audit_is_safe");
        }
        if self.require_transaction_safe {
            guards.push("activation_transaction_is_safe");
        }
        if self.require_transaction_replaces_visible_output {
            guards.push("activation_transaction_replaces_visible_output");
        }
        if self.require_journal_replayable {
            guards.push("journal_is_replayable");
        }
        if self.require_replay_safe {
            guards.push("replay_is_safe");
        }
        if self.require_ledger_ready {
            guards.push("ledger_validation_ready");
        }
        if self.require_store_ready {
            guards.push("store_validation_ready");
        }
        if self.require_persistence_ready {
            guards.push("persistence_roundtrip_ready");
        }
        if self.require_recovery_ready_when_enabled {
            guards.push("recovery_ready_when_enabled");
        }
        if self.require_language_parity_ready {
            guards.push("language_parity_ready");
        }
        if self.require_language_coverage_ready {
            guards.push("language_coverage_ready");
        }
        guards
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationReadinessCheck {
    pub name: String,
    pub required: bool,
    pub passed: bool,
    pub value: String,
    pub reason: String,
}

impl TableViewActivationReadinessCheck {
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
pub struct TableViewActivationReadinessReport {
    pub class: String,
    pub morphism: String,
    pub status: String,
    pub ready_for_visible_activation: bool,
    pub promotion_level: String,
    pub selected_source: String,
    pub selected_line_count: usize,
    pub selected_lines_checksum: u64,
    pub selected_lines_preview: Vec<String>,
    pub selected_lines: Vec<String>,
    pub switch_mode: String,
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
    pub commit_decision: bool,
    pub audit_safe: bool,
    pub transaction_safe: bool,
    pub transaction_replaces_visible_output: bool,
    pub journal_replayable: bool,
    pub replay_safe: bool,
    pub ledger_ready: bool,
    pub store_ready: bool,
    pub persistence_ready: bool,
    pub recovery_enabled: bool,
    pub recovery_ready: bool,
    pub recovery_replays_visible_output: bool,
    pub required_check_count: usize,
    pub passed_required_check_count: usize,
    pub failed_required_checks: Vec<String>,
    pub diagnostic_check_count: usize,
    pub checks: Vec<TableViewActivationReadinessCheck>,
    pub rollback_anchors: Vec<String>,
    pub universal_property: String,
}

impl TableViewActivationReadinessReport {
    pub fn is_ready(&self) -> bool {
        self.ready_for_visible_activation
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationReadinessSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub required_guards: Vec<String>,
    pub diagnostic_guards: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationReadinessBundle;

impl TableViewActivationReadinessBundle {
    pub fn snapshot(&self) -> TableViewActivationReadinessSnapshot {
        TableViewActivationReadinessSnapshot {
            class: "TableViewActivationReadinessSnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_readiness.fold_local_witnesses".to_string(),
                "table_view_activation_readiness.required_guard_summary".to_string(),
                "table_view_activation_readiness.promotion_level".to_string(),
                "table_view_activation_readiness.policy_from_cli".to_string(),
                "table_view_activation_readiness.rollback_sources".to_string(),
            ],
            required_guards: vec![
                "shadow_commit_decision_uses_view_output".to_string(),
                "commit_audit_is_safe".to_string(),
                "activation_transaction_is_safe".to_string(),
                "activation_transaction_replaces_visible_output".to_string(),
                "journal_is_replayable".to_string(),
                "replay_is_safe".to_string(),
                "ledger_validation_ready".to_string(),
                "store_validation_ready".to_string(),
                "persistence_roundtrip_ready".to_string(),
            ],
            diagnostic_guards: vec![
                "semantic_rows_equal".to_string(),
                "virtual_columns_are_witnesses".to_string(),
                "file_recovery_candidate".to_string(),
                "language_coverage_gap_report".to_string(),
                "rollback_anchor_available".to_string(),
            ],
            universal_property:
                "local_activation_witnesses_glue_to_one_readiness_report_before_default_promotion"
                    .to_string(),
        }
    }

    pub fn readiness_for_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        config: &ArchitectureSwitchConfig,
        policy: &TableViewActivationReadinessPolicy,
    ) -> TableViewActivationReadinessReport {
        activation_readiness_for_cli_args(args, legacy_lines, config, policy)
    }
}

pub fn bootstrap_table_view_activation_readiness() -> TableViewActivationReadinessBundle {
    TableViewActivationReadinessBundle
}

#[allow(clippy::too_many_arguments)]
pub fn activation_readiness_from_reports(
    commit: Option<&ShadowTableViewOutputCommitDecision>,
    audit: Option<&TableViewCommitAuditReport>,
    transaction: Option<&TableViewActivationTransactionReport>,
    journal: Option<&TableViewActivationJournal>,
    replay: Option<&TableViewActivationReplayReport>,
    ledger: Option<&TableViewActivationLedger>,
    store: Option<&TableViewActivationStore>,
    persistence: Option<&TableViewActivationPersistenceReport>,
    recovery: Option<&TableViewActivationRecoveryReport>,
    policy: &TableViewActivationReadinessPolicy,
) -> TableViewActivationReadinessReport {
    let commit_decision = commit.map(|value| value.use_view_output).unwrap_or(false);
    let raw_equal = commit.map(|value| value.diff_equal).unwrap_or(false);
    let semantic_equal = commit.map(|value| value.semantic_equal).unwrap_or(false);
    let virtual_direct_cells_equal = commit
        .map(|value| value.virtual_direct_cells_equal)
        .unwrap_or(false);
    let virtual_added_column_count = commit
        .map(|value| value.virtual_added_column_count)
        .unwrap_or(0);
    let language_parity_ready = audit
        .map(|value| value.language_parity_ready)
        .or_else(|| commit.map(|value| value.language_parity_ready))
        .unwrap_or(false);
    let language_requested_language = audit
        .map(|value| value.language_requested_language.clone())
        .or_else(|| commit.map(|value| value.language_requested_language.clone()))
        .unwrap_or_else(|| "unknown".to_string());
    let language_effective_asset_name = audit
        .map(|value| value.language_effective_asset_name.clone())
        .or_else(|| commit.map(|value| value.language_effective_asset_name.clone()))
        .unwrap_or_else(|| "unknown".to_string());
    let language_fallback_applied = audit
        .map(|value| value.language_fallback_applied)
        .or_else(|| commit.map(|value| value.language_fallback_applied))
        .unwrap_or(false);
    let language_failed_guards = audit
        .map(|value| value.language_failed_guards.clone())
        .or_else(|| commit.map(|value| value.language_failed_guards.clone()))
        .unwrap_or_default();
    let language_coverage_ready = audit
        .map(|value| value.language_coverage_ready)
        .or_else(|| commit.map(|value| value.language_coverage_ready))
        .unwrap_or(false);
    let language_coverage_status = audit
        .map(|value| value.language_coverage_status.clone())
        .or_else(|| commit.map(|value| value.language_coverage_status.clone()))
        .unwrap_or_else(|| "unknown".to_string());
    let language_coverage_stale_language_count = audit
        .map(|value| value.language_coverage_stale_language_count)
        .or_else(|| commit.map(|value| value.language_coverage_stale_language_count))
        .unwrap_or(0);
    let language_coverage_languages_missing_744 = audit
        .map(|value| value.language_coverage_languages_missing_744.clone())
        .or_else(|| commit.map(|value| value.language_coverage_languages_missing_744.clone()))
        .unwrap_or_default();
    let language_coverage_failed_guards = audit
        .map(|value| value.language_coverage_failed_guards.clone())
        .or_else(|| commit.map(|value| value.language_coverage_failed_guards.clone()))
        .unwrap_or_default();
    let switch_mode = commit
        .map(|value| value.switch_mode.clone())
        .or_else(|| transaction.map(|value| value.switch_mode.clone()))
        .unwrap_or_else(|| "unknown".to_string());

    let audit_safe = audit.map(|value| value.safe_to_commit).unwrap_or(false);
    let transaction_safe = transaction
        .map(|value| value.safe_to_apply)
        .unwrap_or(false);
    let transaction_replaces_visible_output = transaction
        .map(|value| value.should_replace_visible_output)
        .unwrap_or(false);
    let journal_replayable = journal.map(|value| value.replayable).unwrap_or(false);
    let replay_safe = replay.map(|value| value.replay_safe).unwrap_or(false);
    let ledger_ready = ledger
        .map(|value| value.validation.is_ready())
        .unwrap_or(false);
    let store_ready = store
        .map(|value| value.validation.is_ready())
        .unwrap_or(false);
    let persistence_ready = persistence
        .map(|value| value.is_ready())
        .unwrap_or(false);
    let recovery_enabled = recovery.is_some();
    let recovery_ready = recovery.map(|value| value.is_ready()).unwrap_or(false);
    let recovery_replays_visible_output = recovery
        .map(|value| value.recover_visible_output)
        .unwrap_or(false);

    let mut checks = Vec::new();
    checks.push(TableViewActivationReadinessCheck::new(
        "shadow_commit_decision_uses_view_output",
        policy.require_commit_decision,
        commit_decision,
        commit
            .map(|value| value.reason.clone())
            .unwrap_or_else(|| "missing_shadow_commit_decision".to_string()),
        "shadow pipeline must select the materialized view output before promotion",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "commit_audit_is_safe",
        policy.require_commit_audit_safe,
        audit_safe,
        audit
            .map(|value| format!("failed_required={:?}", value.failed_required_checks))
            .unwrap_or_else(|| "missing_commit_audit".to_string()),
        "all required commit audit guards must pass",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "activation_transaction_is_safe",
        policy.require_transaction_safe,
        transaction_safe,
        transaction
            .map(|value| value.reason.clone())
            .unwrap_or_else(|| "missing_activation_transaction".to_string()),
        "activation transaction must be safe to apply",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "activation_transaction_replaces_visible_output",
        policy.require_transaction_replaces_visible_output,
        transaction_replaces_visible_output,
        transaction
            .map(|value| value.selected_source.clone())
            .unwrap_or_else(|| "missing_activation_transaction".to_string()),
        "transaction must choose table_view_output rather than legacy output",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "journal_is_replayable",
        policy.require_journal_replayable,
        journal_replayable,
        journal
            .map(|value| format!("records={} safe={}", value.record_count, value.safe_record_count))
            .unwrap_or_else(|| "missing_activation_journal".to_string()),
        "activation journal must contain a replayable safe record",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "replay_is_safe",
        policy.require_replay_safe,
        replay_safe,
        replay
            .map(|value| value.reason.clone())
            .unwrap_or_else(|| "missing_activation_replay".to_string()),
        "journal replay must match the current transaction and legacy checksum",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "ledger_validation_ready",
        policy.require_ledger_ready,
        ledger_ready,
        ledger
            .map(|value| value.validation.status.clone())
            .unwrap_or_else(|| "missing_activation_ledger".to_string()),
        "hash-chain ledger must validate before promotion",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "store_validation_ready",
        policy.require_store_ready,
        store_ready,
        store
            .map(|value| value.validation.status.clone())
            .unwrap_or_else(|| "missing_activation_store".to_string()),
        "line-oriented activation store must validate before promotion",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "persistence_roundtrip_ready",
        policy.require_persistence_ready,
        persistence_ready,
        persistence
            .map(|value| value.status.clone())
            .unwrap_or_else(|| "missing_activation_persistence".to_string()),
        "persistence bridge must load, parse and hash-match the activation store",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "recovery_ready_when_enabled",
        policy.require_recovery_ready_when_enabled && recovery_enabled,
        !recovery_enabled || recovery_ready,
        recovery
            .map(|value| value.status.clone())
            .unwrap_or_else(|| "recovery_not_requested".to_string()),
        "file recovery is optional, but if required it must parse and replay safely",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "language_parity_ready",
        policy.require_language_parity_ready,
        language_parity_ready,
        format!(
            "requested_language={language_requested_language} effective_asset={language_effective_asset_name} fallback_applied={language_fallback_applied} failed_guards={language_failed_guards:?}"
        ),
        "localized materialization may be promoted only when missing direct columns fall back to a safe base asset",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "language_coverage_ready",
        policy.require_language_coverage_ready,
        language_coverage_ready,
        format!(
            "status={language_coverage_status} stale_languages={language_coverage_stale_language_count} missing_744={language_coverage_languages_missing_744:?} failed_guards={language_coverage_failed_guards:?}"
        ),
        "language coverage must confirm every requested direct column is covered by the effective asset or safe fallback",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "semantic_rows_equal",
        false,
        semantic_equal,
        format!("raw_equal={raw_equal}"),
        "semantic equality is diagnostic; raw equality remains the commit guard",
    ));
    checks.push(TableViewActivationReadinessCheck::new(
        "virtual_direct_cells_equal",
        false,
        virtual_direct_cells_equal,
        format!("added_virtual_columns={virtual_added_column_count}"),
        "virtual columns may add witnesses but must not change direct CSV cells",
    ));

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
    let ready_for_visible_activation = failed_required_checks.is_empty()
        && (recovery_replays_visible_output
            || ledger.map(|value| value.replay_visible_output).unwrap_or(false)
            || replay.map(|value| value.replay_visible_output).unwrap_or(false)
            || transaction_replaces_visible_output
            || commit_decision);

    let (promotion_level, selected_source, selected_lines) = if recovery_replays_visible_output {
        let recovery = recovery.expect("checked Some above");
        (
            "recovery_replay".to_string(),
            recovery.selected_source.clone(),
            recovery.selected_lines.clone(),
        )
    } else if let Some(ledger) = ledger.filter(|value| value.replay_visible_output) {
        let lines = ledger
            .replay
            .as_ref()
            .map(|value| value.selected_lines.clone())
            .unwrap_or_default();
        (
            "ledger_replay".to_string(),
            ledger.replay_selected_source.clone(),
            lines,
        )
    } else if let Some(replay) = replay.filter(|value| value.replay_visible_output) {
        (
            "journal_replay".to_string(),
            replay.selected_source.clone(),
            replay.selected_lines.clone(),
        )
    } else if let Some(transaction) = transaction.filter(|value| value.should_replace_visible_output) {
        (
            "activation_transaction".to_string(),
            transaction.selected_source.clone(),
            transaction.selected_lines.clone(),
        )
    } else if commit_decision {
        (
            "shadow_commit".to_string(),
            "table_view_output".to_string(),
            Vec::new(),
        )
    } else {
        ("legacy".to_string(), "legacy_output".to_string(), Vec::new())
    };

    let selected_lines_checksum = stable_line_checksum(&selected_lines);
    let selected_line_count = selected_lines.len();
    let selected_lines_preview = selected_lines
        .iter()
        .take(policy.preview_limit)
        .cloned()
        .collect::<Vec<_>>();
    let selected_lines = if policy.include_selected_lines {
        selected_lines
    } else {
        Vec::new()
    };

    let mut rollback_anchors = Vec::new();
    if let Some(anchor) = commit.and_then(|value| value.rollback_anchor.clone()) {
        rollback_anchors.push(anchor);
    }
    if let Some(anchor) = transaction.and_then(|value| value.rollback_anchor.clone()) {
        rollback_anchors.push(anchor);
    }
    if let Some(anchor) = journal.and_then(|value| value.latest_rollback_anchor.clone()) {
        rollback_anchors.push(anchor);
    }
    if let Some(anchor) = ledger.and_then(|value| value.latest_rollback_anchor.clone()) {
        rollback_anchors.push(anchor);
    }
    rollback_anchors.sort();
    rollback_anchors.dedup();

    TableViewActivationReadinessReport {
        class: "TableViewActivationReadinessReport".to_string(),
        morphism: "table_view_activation_readiness.fold_local_witnesses".to_string(),
        status: if ready_for_visible_activation { "ready" } else { "blocked" }.to_string(),
        ready_for_visible_activation,
        promotion_level,
        selected_source,
        selected_line_count,
        selected_lines_checksum,
        selected_lines_preview,
        selected_lines,
        switch_mode,
        raw_equal,
        semantic_equal,
        virtual_direct_cells_equal,
        virtual_added_column_count,
        language_parity_ready,
        language_requested_language,
        language_effective_asset_name,
        language_fallback_applied,
        language_failed_guards,
        language_coverage_ready,
        language_coverage_status,
        language_coverage_stale_language_count,
        language_coverage_languages_missing_744,
        language_coverage_failed_guards,
        commit_decision,
        audit_safe,
        transaction_safe,
        transaction_replaces_visible_output,
        journal_replayable,
        replay_safe,
        ledger_ready,
        store_ready,
        persistence_ready,
        recovery_enabled,
        recovery_ready,
        recovery_replays_visible_output,
        required_check_count,
        passed_required_check_count,
        failed_required_checks,
        diagnostic_check_count,
        checks,
        rollback_anchors,
        universal_property:
            "all_local_activation_guards_must_glue_before_the_table_view_output_can_be_promoted"
                .to_string(),
    }
}

pub fn activation_readiness_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationReadinessPolicy,
) -> TableViewActivationReadinessReport {
    let pipeline = bootstrap_shadow_pipeline();
    let report = pipeline.shadow_table_view_output(args, legacy_lines, config);
    let commit = pipeline.table_view_output_commit_decision(&report, config);
    let audit = audit_table_view_output_commit(&report, &commit);
    let transaction = table_view_activation_transaction(
        &report,
        &commit,
        legacy_lines,
        &TableViewActivationTransactionPolicy::default(),
    );
    let journal = activation_journal_from_transactions(
        std::slice::from_ref(&transaction),
        &TableViewActivationJournalPolicy::default(),
    );
    let replay = activation_replay_from_journal(
        &journal,
        legacy_lines,
        Some(transaction.transaction_id.as_str()),
        &TableViewActivationReplayPolicy::default(),
    );
    let ledger = activation_ledger_from_journal(
        &journal,
        legacy_lines,
        Some(transaction.transaction_id.as_str()),
        &TableViewActivationLedgerPolicy::default(),
    );
    let store = activation_store_from_journal_and_ledger(
        &journal,
        &ledger,
        &TableViewActivationStorePolicy::default(),
    );
    let mut persistence = PersistenceStore::default();
    let persistence_report = persist_activation_store_to_persistence(
        &store,
        legacy_lines,
        store.latest_transaction_id.as_deref(),
        &mut persistence,
        &TableViewActivationPersistencePolicy::default(),
    );
    let (recovery_policy, recovery_enabled) = activation_recovery_policy_from_cli_args(
        args,
        &TableViewActivationRecoveryPolicy::default(),
    );
    let recovery = recovery_enabled.then(|| {
        activation_recovery_for_cli_args(args, legacy_lines, config, &recovery_policy)
    });
    activation_readiness_from_reports(
        Some(&commit),
        Some(&audit),
        Some(&transaction),
        Some(&journal),
        Some(&replay),
        Some(&ledger),
        Some(&store),
        Some(&persistence_report),
        recovery.as_ref(),
        policy,
    )
}

pub fn continuum_m_activation_readiness_smoke() -> TableViewActivationReadinessReport {
    let args = vec![
        "reta".to_string(),
        "--reta-arch=commit".to_string(),
        "--reta-arch-allow=table_view_output.commit,shadow_pipeline.table_view_output_commit,table_view_commit_audit.required_guards".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "--breite=0".to_string(),
    ];
    let legacy_lines = Vec::<String>::new();
    let (_, config) = crate::runtime_switch::extract_architecture_switch_from_argv(&args, None);
    activation_readiness_for_cli_args(
        &args,
        &legacy_lines,
        &config,
        &TableViewActivationReadinessPolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_switch::ArchitectureSwitchConfig;

    #[test]
    fn readiness_reports_blocked_without_legacy_parity() {
        let args = vec![
            "reta".to_string(),
            "--reta-arch=commit".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
        ];
        let report = activation_readiness_for_cli_args(
            &args,
            &[],
            &ArchitectureSwitchConfig::default().with_mode(crate::runtime_switch::ArchitectureSwitchMode::Commit, "test"),
            &TableViewActivationReadinessPolicy::default(),
        );
        assert_eq!(report.status, "blocked");
        assert!(report.required_check_count >= report.passed_required_check_count);
    }

    #[test]
    fn readiness_from_matching_reports_can_be_ready() {
        let fake_commit = ShadowTableViewOutputCommitDecision {
            morphism: "shadow_pipeline.table_view_output_commit".to_string(),
            use_view_output: true,
            reason: "ok".to_string(),
            switch_mode: "commit".to_string(),
            gate_reason: "commit_gate".to_string(),
            gate_allowed_to_commit: true,
            diff_equal: true,
            semantic_equal: true,
            virtual_direct_cells_equal: true,
            virtual_rendered_policy: "Suppress".to_string(),
            virtual_added_column_count: 0,
            language_parity_ready: true,
            language_requested_language: "base".to_string(),
            language_effective_asset_name: "religion.csv".to_string(),
            language_fallback_applied: false,
            language_failed_guards: Vec::new(),
            language_coverage_ready: true,
            language_coverage_status: "ready".to_string(),
            language_coverage_stale_language_count: 0,
            language_coverage_languages_missing_744: Vec::new(),
            language_coverage_failed_guards: Vec::new(),
            force_override: false,
            rendered_line_count: 1,
            rollback_anchor: Some("rollback:test".to_string()),
            universal_property: "test".to_string(),
        };
        let policy = TableViewActivationReadinessPolicy {
            require_commit_audit_safe: false,
            require_transaction_safe: false,
            require_transaction_replaces_visible_output: false,
            require_journal_replayable: false,
            require_replay_safe: false,
            require_ledger_ready: false,
            require_store_ready: false,
            require_persistence_ready: false,
            ..TableViewActivationReadinessPolicy::default()
        };
        let report = activation_readiness_from_reports(
            Some(&fake_commit),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            &policy,
        );
        assert!(report.ready_for_visible_activation);
        assert_eq!(report.promotion_level, "shadow_commit");
    }

    #[test]
    fn readiness_policy_cli_can_switch_to_diagnostic_mode() {
        let args = vec![
            "reta".to_string(),
            "--activation-readiness-diagnostic".to_string(),
            "--activation-readiness-no-selected-lines".to_string(),
            "--activation-readiness-preview=2".to_string(),
        ];
        let (policy, recognized) = TableViewActivationReadinessPolicy::from_cli_args(
            &args,
            &TableViewActivationReadinessPolicy::default(),
        );
        assert!(recognized);
        assert!(!policy.require_commit_decision);
        assert!(!policy.require_persistence_ready);
        assert!(!policy.include_selected_lines);
        assert_eq!(policy.preview_limit, 2);
        assert!(policy.required_guard_names().is_empty());
    }

    #[test]
    fn readiness_policy_cli_can_require_recovery() {
        let args = vec![
            "reta".to_string(),
            "--activation-readiness-require-recovery".to_string(),
            "--activation-readiness-ignore-persistence".to_string(),
        ];
        let (policy, recognized) = TableViewActivationReadinessPolicy::from_cli_args(
            &args,
            &TableViewActivationReadinessPolicy::default(),
        );
        assert!(recognized);
        assert!(policy.require_recovery_ready_when_enabled);
        assert!(!policy.require_persistence_ready);
        assert!(policy
            .required_guard_names()
            .contains(&"recovery_ready_when_enabled"));
    }

}
