//! Deterministic activation journals for guarded table-view output switches.
//!
//! Stage 41 made a single activation decision explicit.  This module turns such
//! decisions into replayable journal records.  The journal is intentionally not
//! a persistence backend; it is a typed witness that can later be written to a
//! file, FFI boundary, or diagnostic stream.  The important invariant is that a
//! visible Rust table-view activation can be replayed only through the same
//! transaction checksum and only when the selected line checksum still matches.

use serde::{Deserialize, Serialize};

use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::table_view_activation_transaction::{
    stable_line_checksum, table_view_activation_transaction_for_cli_args,
    TableViewActivationTransactionPolicy, TableViewActivationTransactionReport,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationJournalPolicy {
    pub include_selected_lines: bool,
    pub include_selected_preview: bool,
    pub preview_limit: usize,
    pub max_records: usize,
}

impl Default for TableViewActivationJournalPolicy {
    fn default() -> Self {
        Self {
            include_selected_lines: true,
            include_selected_preview: true,
            preview_limit: 8,
            max_records: 32,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationJournalRecord {
    pub class: String,
    pub morphism: String,
    pub sequence: usize,
    pub transaction_id: String,
    pub switch_mode: String,
    pub selected_source: String,
    pub should_replace_visible_output: bool,
    pub safe_to_apply: bool,
    pub reason: String,
    pub commit_reason: String,
    pub rollback_anchor: Option<String>,
    pub selected_line_count: usize,
    pub legacy_line_count: usize,
    pub view_output_line_count: usize,
    pub selected_lines_checksum: u64,
    pub legacy_lines_checksum: u64,
    pub view_output_lines_checksum: u64,
    pub raw_equal: bool,
    pub semantic_equal: bool,
    pub virtual_direct_cells_equal: bool,
    pub virtual_added_column_count: usize,
    pub required_failed_count: usize,
    pub failed_required_checks: Vec<String>,
    pub selected_lines_preview: Vec<String>,
    pub selected_lines: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationJournal {
    pub class: String,
    pub morphism: String,
    pub records: Vec<TableViewActivationJournalRecord>,
    pub record_count: usize,
    pub safe_record_count: usize,
    pub rejected_record_count: usize,
    pub latest_transaction_id: Option<String>,
    pub latest_selected_source: Option<String>,
    pub latest_selected_checksum: Option<u64>,
    pub latest_rollback_anchor: Option<String>,
    pub replayable: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationJournalReplayReport {
    pub class: String,
    pub morphism: String,
    pub record_count: usize,
    pub replayed: bool,
    pub replay_safe: bool,
    pub selected_source: String,
    pub reason: String,
    pub selected_line_count: usize,
    pub selected_lines_checksum: u64,
    pub fallback_line_count: usize,
    pub fallback_lines_checksum: u64,
    pub selected_lines_preview: Vec<String>,
    pub selected_lines: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationJournalSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub required_inputs: Vec<String>,
    pub replay_guards: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationJournalBundle;

impl TableViewActivationJournalBundle {
    pub fn snapshot(&self) -> TableViewActivationJournalSnapshot {
        TableViewActivationJournalSnapshot {
            class: "TableViewActivationJournalSnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_journal.record_transaction".to_string(),
                "table_view_activation_journal.fold_records".to_string(),
                "table_view_activation_journal.replay_selected_lines".to_string(),
                "table_view_activation_journal.rollback_checksum_witness".to_string(),
            ],
            required_inputs: vec![
                "TableViewActivationTransactionReport".to_string(),
                "legacy_visible_lines".to_string(),
                "selected_lines_checksum".to_string(),
            ],
            replay_guards: vec![
                "latest_record_is_safe".to_string(),
                "selected_lines_are_present".to_string(),
                "selected_lines_checksum_matches".to_string(),
                "fallback_legacy_lines_are_available_on_reject".to_string(),
            ],
            universal_property:
                "activation_records_replay_visible_output_only_through_a_matching_transaction_checksum".to_string(),
        }
    }

    pub fn record(
        &self,
        transaction: &TableViewActivationTransactionReport,
        sequence: usize,
        policy: &TableViewActivationJournalPolicy,
    ) -> TableViewActivationJournalRecord {
        activation_journal_record_from_transaction(transaction, sequence, policy)
    }

    pub fn journal_from_transactions(
        &self,
        transactions: &[TableViewActivationTransactionReport],
        policy: &TableViewActivationJournalPolicy,
    ) -> TableViewActivationJournal {
        activation_journal_from_transactions(transactions, policy)
    }

    pub fn journal_for_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        config: &ArchitectureSwitchConfig,
        policy: &TableViewActivationJournalPolicy,
    ) -> TableViewActivationJournal {
        activation_journal_for_cli_args(args, legacy_lines, config, policy)
    }

    pub fn replay(
        &self,
        journal: &TableViewActivationJournal,
        fallback_legacy_lines: &[String],
        policy: &TableViewActivationJournalPolicy,
    ) -> TableViewActivationJournalReplayReport {
        replay_activation_journal(journal, fallback_legacy_lines, policy)
    }
}

pub fn bootstrap_table_view_activation_journal() -> TableViewActivationJournalBundle {
    TableViewActivationJournalBundle
}

pub fn activation_journal_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationJournalPolicy,
) -> TableViewActivationJournal {
    let transaction = table_view_activation_transaction_for_cli_args(
        args,
        legacy_lines,
        config,
        &TableViewActivationTransactionPolicy::default(),
    );
    activation_journal_from_transactions(&[transaction], policy)
}

pub fn activation_journal_from_transactions(
    transactions: &[TableViewActivationTransactionReport],
    policy: &TableViewActivationJournalPolicy,
) -> TableViewActivationJournal {
    let records = transactions
        .iter()
        .take(policy.max_records)
        .enumerate()
        .map(|(sequence, transaction)| {
            activation_journal_record_from_transaction(transaction, sequence, policy)
        })
        .collect::<Vec<_>>();
    activation_journal_from_records(records)
}

pub fn activation_journal_from_records(
    records: Vec<TableViewActivationJournalRecord>,
) -> TableViewActivationJournal {
    let record_count = records.len();
    let safe_record_count = records.iter().filter(|record| record.safe_to_apply).count();
    let rejected_record_count = record_count.saturating_sub(safe_record_count);
    let latest = records.last();
    let replayable = latest
        .map(|record| {
            record.safe_to_apply
                && !record.selected_lines.is_empty()
                && stable_line_checksum(&record.selected_lines) == record.selected_lines_checksum
        })
        .unwrap_or(false);
    TableViewActivationJournal {
        class: "TableViewActivationJournal".to_string(),
        morphism: "table_view_activation_journal.fold_records".to_string(),
        record_count,
        safe_record_count,
        rejected_record_count,
        latest_transaction_id: latest.map(|record| record.transaction_id.clone()),
        latest_selected_source: latest.map(|record| record.selected_source.clone()),
        latest_selected_checksum: latest.map(|record| record.selected_lines_checksum),
        latest_rollback_anchor: latest.and_then(|record| record.rollback_anchor.clone()),
        replayable,
        records,
        universal_property:
            "a_journal_is_replayable_exactly_when_the_latest_safe_record_carries_matching_selected_lines".to_string(),
    }
}

pub fn activation_journal_record_from_transaction(
    transaction: &TableViewActivationTransactionReport,
    sequence: usize,
    policy: &TableViewActivationJournalPolicy,
) -> TableViewActivationJournalRecord {
    let selected_lines = if policy.include_selected_lines {
        transaction.selected_lines.clone()
    } else {
        Vec::new()
    };
    let selected_lines_preview = if policy.include_selected_preview {
        transaction
            .selected_lines
            .iter()
            .take(policy.preview_limit)
            .cloned()
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    TableViewActivationJournalRecord {
        class: "TableViewActivationJournalRecord".to_string(),
        morphism: "table_view_activation_journal.record_transaction".to_string(),
        sequence,
        transaction_id: transaction.transaction_id.clone(),
        switch_mode: transaction.switch_mode.clone(),
        selected_source: transaction.selected_source.clone(),
        should_replace_visible_output: transaction.should_replace_visible_output,
        safe_to_apply: transaction.safe_to_apply,
        reason: transaction.reason.clone(),
        commit_reason: transaction.commit_reason.clone(),
        rollback_anchor: transaction.rollback_anchor.clone(),
        selected_line_count: transaction.selected_line_count,
        legacy_line_count: transaction.legacy_line_count,
        view_output_line_count: transaction.view_output_line_count,
        selected_lines_checksum: transaction.selected_lines_checksum,
        legacy_lines_checksum: transaction.legacy_lines_checksum,
        view_output_lines_checksum: transaction.view_output_lines_checksum,
        raw_equal: transaction.audit.raw_equal,
        semantic_equal: transaction.audit.semantic_equal,
        virtual_direct_cells_equal: transaction.audit.virtual_direct_cells_equal,
        virtual_added_column_count: transaction.audit.virtual_added_column_count,
        required_failed_count: transaction.required_failed_count,
        failed_required_checks: transaction.failed_required_checks.clone(),
        selected_lines_preview,
        selected_lines,
        universal_property:
            "each_record_preserves_the_activation_transaction_and_rollback_checksum_witness"
                .to_string(),
    }
}

pub fn replay_activation_journal(
    journal: &TableViewActivationJournal,
    fallback_legacy_lines: &[String],
    policy: &TableViewActivationJournalPolicy,
) -> TableViewActivationJournalReplayReport {
    let fallback_lines_checksum = stable_line_checksum(fallback_legacy_lines);
    let latest = journal.records.last();
    let (replayed, replay_safe, selected_source, reason, selected_lines) = match latest {
        Some(record)
            if record.safe_to_apply
                && !record.selected_lines.is_empty()
                && stable_line_checksum(&record.selected_lines)
                    == record.selected_lines_checksum =>
        {
            (
                true,
                true,
                record.selected_source.clone(),
                "latest_safe_record_replayed".to_string(),
                record.selected_lines.clone(),
            )
        }
        Some(record) if record.safe_to_apply && record.selected_lines.is_empty() => (
            false,
            false,
            "legacy_output".to_string(),
            "latest_record_is_safe_but_selected_lines_are_not_embedded".to_string(),
            fallback_legacy_lines.to_vec(),
        ),
        Some(record) if !record.safe_to_apply => (
            false,
            true,
            "legacy_output".to_string(),
            format!("latest_record_rejected:{}", record.reason),
            fallback_legacy_lines.to_vec(),
        ),
        Some(_) => (
            false,
            false,
            "legacy_output".to_string(),
            "latest_record_checksum_mismatch".to_string(),
            fallback_legacy_lines.to_vec(),
        ),
        None => (
            false,
            true,
            "legacy_output".to_string(),
            "empty_journal_uses_fallback_legacy".to_string(),
            fallback_legacy_lines.to_vec(),
        ),
    };
    let selected_lines_preview = selected_lines
        .iter()
        .take(policy.preview_limit)
        .cloned()
        .collect::<Vec<_>>();
    let selected_lines_checksum = stable_line_checksum(&selected_lines);
    TableViewActivationJournalReplayReport {
        class: "TableViewActivationJournalReplayReport".to_string(),
        morphism: "table_view_activation_journal.replay_selected_lines".to_string(),
        record_count: journal.record_count,
        replayed,
        replay_safe,
        selected_source,
        reason,
        selected_line_count: selected_lines.len(),
        selected_lines_checksum,
        fallback_line_count: fallback_legacy_lines.len(),
        fallback_lines_checksum,
        selected_lines_preview,
        selected_lines,
        universal_property:
            "journal_replay_uses_rust_lines_only_when_the_embedded_checksum_matches_the_latest_safe_record".to_string(),
    }
}

pub fn continuum_m_activation_journal_smoke() -> TableViewActivationJournal {
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
        "stage42-smoke",
    );
    let transaction =
        crate::table_view_activation_transaction::continuum_m_activation_transaction_smoke();
    let legacy_lines = transaction.selected_lines.clone();
    activation_journal_for_cli_args(
        &args,
        &legacy_lines,
        &config,
        &TableViewActivationJournalPolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_journal_records_and_replays_safe_transaction() {
        let journal = continuum_m_activation_journal_smoke();
        assert_eq!(journal.record_count, 1);
        assert_eq!(journal.safe_record_count, 1);
        assert!(journal.replayable);
        let replay =
            replay_activation_journal(&journal, &[], &TableViewActivationJournalPolicy::default());
        assert!(replay.replayed);
        assert!(replay.replay_safe);
        assert_eq!(replay.selected_source, "table_view_output");
    }

    #[test]
    fn activation_journal_falls_back_when_lines_are_not_embedded() {
        let tx =
            crate::table_view_activation_transaction::continuum_m_activation_transaction_smoke();
        let policy = TableViewActivationJournalPolicy {
            include_selected_lines: false,
            ..TableViewActivationJournalPolicy::default()
        };
        let journal = activation_journal_from_transactions(&[tx], &policy);
        assert!(!journal.replayable);
        let fallback = vec!["legacy".to_string()];
        let replay = replay_activation_journal(&journal, &fallback, &policy);
        assert!(!replay.replayed);
        assert_eq!(replay.selected_lines, fallback);
    }

    #[test]
    fn activation_journal_falls_back_when_rejected() {
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
            "test",
        );
        let legacy = vec!["different legacy line".to_string()];
        let journal = activation_journal_for_cli_args(
            &args,
            &legacy,
            &config,
            &TableViewActivationJournalPolicy::default(),
        );
        assert_eq!(journal.rejected_record_count, 1);
        let replay = replay_activation_journal(
            &journal,
            &legacy,
            &TableViewActivationJournalPolicy::default(),
        );
        assert!(!replay.replayed);
        assert_eq!(replay.selected_source, "legacy_output");
        assert_eq!(replay.selected_lines, legacy);
    }
}
