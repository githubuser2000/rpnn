//! Replay and rollback guard for table-view activation journals.
//!
//! Stage 42 made activation decisions journaled.  This module adds the next
//! safety layer: a journal may be replayed only when the latest record is safe,
//! still embeds the selected lines, still matches its checksum, and still
//! matches the current transaction/legacy checksum when the policy asks for it.
//! In all other cases the replay morphism returns the current legacy lines.

use serde::{Deserialize, Serialize};

use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::table_view_activation_journal::{
    TableViewActivationJournal, TableViewActivationJournalPolicy,
    TableViewActivationJournalReplayReport, activation_journal_for_cli_args,
    replay_activation_journal,
};
use crate::table_view_activation_transaction::{
    TableViewActivationTransactionPolicy, stable_line_checksum,
    table_view_activation_transaction_for_cli_args,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationReplayPolicy {
    pub require_journal_replayable: bool,
    pub require_latest_transaction_match: bool,
    pub require_current_legacy_checksum_match: bool,
    pub include_selected_lines: bool,
    pub include_journal_replay_report: bool,
    pub preview_limit: usize,
}

impl Default for TableViewActivationReplayPolicy {
    fn default() -> Self {
        Self {
            require_journal_replayable: true,
            require_latest_transaction_match: true,
            require_current_legacy_checksum_match: true,
            include_selected_lines: true,
            include_journal_replay_report: true,
            preview_limit: 8,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationReplayReport {
    pub class: String,
    pub morphism: String,
    pub replay_visible_output: bool,
    pub replay_safe: bool,
    pub selected_source: String,
    pub reason: String,
    pub latest_transaction_id: Option<String>,
    pub current_transaction_id: Option<String>,
    pub latest_transaction_matches_current: bool,
    pub latest_legacy_checksum: Option<u64>,
    pub current_legacy_checksum: u64,
    pub latest_legacy_checksum_matches_current: bool,
    pub latest_selected_checksum: Option<u64>,
    pub selected_line_count: usize,
    pub selected_lines_checksum: u64,
    pub fallback_line_count: usize,
    pub fallback_lines_checksum: u64,
    pub selected_lines_preview: Vec<String>,
    pub selected_lines: Vec<String>,
    pub journal_replay: Option<TableViewActivationJournalReplayReport>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationReplaySnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub replay_guards: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationReplayBundle;

impl TableViewActivationReplayBundle {
    pub fn snapshot(&self) -> TableViewActivationReplaySnapshot {
        TableViewActivationReplaySnapshot {
            class: "TableViewActivationReplaySnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_replay.guard_journal_replay".to_string(),
                "table_view_activation_replay.match_transaction_id".to_string(),
                "table_view_activation_replay.match_legacy_checksum".to_string(),
                "table_view_activation_replay.rollback_to_legacy_lines".to_string(),
            ],
            replay_guards: vec![
                "journal_is_replayable".to_string(),
                "latest_transaction_id_matches_current_transaction".to_string(),
                "latest_legacy_checksum_matches_current_legacy".to_string(),
                "selected_lines_checksum_matches_embedded_lines".to_string(),
            ],
            universal_property:
                "journal_replay_is_identity_on_safe_activation_records_and_rollback_on_drift"
                    .to_string(),
        }
    }

    pub fn replay_from_journal(
        &self,
        journal: &TableViewActivationJournal,
        fallback_legacy_lines: &[String],
        current_transaction_id: Option<&str>,
        policy: &TableViewActivationReplayPolicy,
    ) -> TableViewActivationReplayReport {
        activation_replay_from_journal(
            journal,
            fallback_legacy_lines,
            current_transaction_id,
            policy,
        )
    }

    pub fn replay_for_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        config: &ArchitectureSwitchConfig,
        policy: &TableViewActivationReplayPolicy,
    ) -> TableViewActivationReplayReport {
        activation_replay_for_cli_args(args, legacy_lines, config, policy)
    }
}

pub fn bootstrap_table_view_activation_replay() -> TableViewActivationReplayBundle {
    TableViewActivationReplayBundle
}

pub fn activation_replay_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationReplayPolicy,
) -> TableViewActivationReplayReport {
    let transaction = table_view_activation_transaction_for_cli_args(
        args,
        legacy_lines,
        config,
        &TableViewActivationTransactionPolicy::default(),
    );
    let journal = activation_journal_for_cli_args(
        args,
        legacy_lines,
        config,
        &TableViewActivationJournalPolicy::default(),
    );
    activation_replay_from_journal(
        &journal,
        legacy_lines,
        Some(transaction.transaction_id.as_str()),
        policy,
    )
}

pub fn activation_replay_from_journal(
    journal: &TableViewActivationJournal,
    fallback_legacy_lines: &[String],
    current_transaction_id: Option<&str>,
    policy: &TableViewActivationReplayPolicy,
) -> TableViewActivationReplayReport {
    let journal_policy = TableViewActivationJournalPolicy::default();
    let replay = replay_activation_journal(journal, fallback_legacy_lines, &journal_policy);
    let latest = journal.records.last();
    let current_legacy_checksum = stable_line_checksum(fallback_legacy_lines);
    let fallback_lines_checksum = current_legacy_checksum;
    let latest_transaction_id = latest.map(|record| record.transaction_id.clone());
    let latest_legacy_checksum = latest.map(|record| record.legacy_lines_checksum);
    let latest_selected_checksum = latest.map(|record| record.selected_lines_checksum);
    let latest_transaction_matches_current = match (latest_transaction_id.as_deref(), current_transaction_id) {
        (Some(latest), Some(current)) => latest == current,
        (Some(_), None) => !policy.require_latest_transaction_match,
        (None, _) => false,
    };
    let latest_legacy_checksum_matches_current = latest_legacy_checksum
        .map(|checksum| checksum == current_legacy_checksum)
        .unwrap_or(false);

    let mut failed = Vec::new();
    if policy.require_journal_replayable && !journal.replayable {
        failed.push("journal_not_replayable");
    }
    if policy.require_latest_transaction_match && !latest_transaction_matches_current {
        failed.push("latest_transaction_id_does_not_match_current_transaction");
    }
    if policy.require_current_legacy_checksum_match && !latest_legacy_checksum_matches_current {
        failed.push("latest_legacy_checksum_does_not_match_current_legacy");
    }
    if !replay.replayed || !replay.replay_safe {
        failed.push("journal_replay_report_not_safe_to_replay");
    }

    let replay_visible_output = failed.is_empty();
    let (selected_source, reason, selected_lines) = if replay_visible_output {
        (
            replay.selected_source.clone(),
            "latest_journal_record_replayed_under_current_guards".to_string(),
            replay.selected_lines.clone(),
        )
    } else {
        (
            "legacy_output".to_string(),
            format!("rollback_to_legacy:{:?}", failed),
            fallback_legacy_lines.to_vec(),
        )
    };
    let selected_line_count = selected_lines.len();
    let selected_lines_checksum = stable_line_checksum(&selected_lines);
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

    TableViewActivationReplayReport {
        class: "TableViewActivationReplayReport".to_string(),
        morphism: "table_view_activation_replay.guard_journal_replay".to_string(),
        replay_visible_output,
        replay_safe: failed.is_empty(),
        selected_source,
        reason,
        latest_transaction_id,
        current_transaction_id: current_transaction_id.map(ToString::to_string),
        latest_transaction_matches_current,
        latest_legacy_checksum,
        current_legacy_checksum,
        latest_legacy_checksum_matches_current,
        latest_selected_checksum,
        selected_line_count,
        selected_lines_checksum,
        fallback_line_count: fallback_legacy_lines.len(),
        fallback_lines_checksum,
        selected_lines_preview,
        selected_lines,
        journal_replay: if policy.include_journal_replay_report {
            Some(replay)
        } else {
            None
        },
        universal_property:
            "journal_replay_commutes_with_current_transaction_only_when_checksums_and_transaction_ids_match".to_string(),
    }
}

pub fn continuum_m_activation_replay_smoke() -> TableViewActivationReplayReport {
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "--breite=0".to_string(),
    ];
    let config = ArchitectureSwitchConfig::default().with_mode(
        crate::runtime_switch::ArchitectureSwitchMode::Commit,
        "stage43-smoke",
    );
    let transaction = crate::table_view_activation_transaction::continuum_m_activation_transaction_smoke();
    activation_replay_for_cli_args(
        &args,
        &transaction.selected_lines,
        &config,
        &TableViewActivationReplayPolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_replay_replays_safe_journal() {
        let replay = continuum_m_activation_replay_smoke();
        assert!(replay.replay_visible_output);
        assert!(replay.replay_safe);
        assert_eq!(replay.selected_source, "table_view_output");
        assert!(replay.latest_transaction_matches_current);
        assert!(replay.latest_legacy_checksum_matches_current);
    }

    #[test]
    fn activation_replay_rolls_back_on_legacy_checksum_drift() {
        let tx = crate::table_view_activation_transaction::continuum_m_activation_transaction_smoke();
        let journal = crate::table_view_activation_journal::activation_journal_from_transactions(
            &[tx.clone()],
            &TableViewActivationJournalPolicy::default(),
        );
        let fallback = vec!["legacy changed".to_string()];
        let replay = activation_replay_from_journal(
            &journal,
            &fallback,
            Some(tx.transaction_id.as_str()),
            &TableViewActivationReplayPolicy::default(),
        );
        assert!(!replay.replay_visible_output);
        assert_eq!(replay.selected_source, "legacy_output");
        assert_eq!(replay.selected_lines, fallback);
    }

    #[test]
    fn activation_replay_rolls_back_on_transaction_mismatch() {
        let tx = crate::table_view_activation_transaction::continuum_m_activation_transaction_smoke();
        let journal = crate::table_view_activation_journal::activation_journal_from_transactions(
            &[tx],
            &TableViewActivationJournalPolicy::default(),
        );
        let fallback = journal.records[0].selected_lines.clone();
        let replay = activation_replay_from_journal(
            &journal,
            &fallback,
            Some("different-transaction"),
            &TableViewActivationReplayPolicy::default(),
        );
        assert!(!replay.replay_visible_output);
        assert_eq!(replay.selected_source, "legacy_output");
    }
}
