//! Hash-chained activation ledger for guarded table-view output switches.
//!
//! Stage 42 made activations journaled and Stage 43 made replay rollback-aware.
//! This module adds a ledger witness over those records: each journal record is
//! folded into a stable hash chain, the chain can be validated, and the latest
//! safe replay can be explained with a deterministic ledger checksum.  This is
//! not cryptographic storage; it is a typed audit morphism for migration and
//! rollback decisions.

use serde::{Deserialize, Serialize};

use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::table_view_activation_journal::{
    TableViewActivationJournal, TableViewActivationJournalPolicy,
    TableViewActivationJournalRecord, activation_journal_for_cli_args,
};
use crate::table_view_activation_replay::{
    TableViewActivationReplayPolicy, TableViewActivationReplayReport,
    activation_replay_from_journal,
};
use crate::table_view_activation_transaction::stable_line_checksum;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationLedgerPolicy {
    pub include_record_previews: bool,
    pub include_replay_report: bool,
    pub max_records: usize,
    pub preview_limit: usize,
    pub require_contiguous_sequence: bool,
    pub require_hash_chain: bool,
    pub require_latest_replay_safe: bool,
}

impl Default for TableViewActivationLedgerPolicy {
    fn default() -> Self {
        Self {
            include_record_previews: true,
            include_replay_report: true,
            max_records: 64,
            preview_limit: 8,
            require_contiguous_sequence: true,
            require_hash_chain: true,
            require_latest_replay_safe: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationLedgerEntry {
    pub class: String,
    pub morphism: String,
    pub sequence: usize,
    pub transaction_id: String,
    pub previous_chain_hash: Option<u64>,
    pub record_hash: u64,
    pub chain_hash: u64,
    pub selected_source: String,
    pub should_replace_visible_output: bool,
    pub safe_to_apply: bool,
    pub reason: String,
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
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationLedgerValidation {
    pub class: String,
    pub status: String,
    pub entry_count: usize,
    pub contiguous_sequence: bool,
    pub hash_chain_valid: bool,
    pub latest_replay_safe: bool,
    pub latest_chain_hash: Option<u64>,
    pub failed_guards: Vec<String>,
    pub universal_property: String,
}

impl TableViewActivationLedgerValidation {
    pub fn is_ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationLedger {
    pub class: String,
    pub morphism: String,
    pub entry_count: usize,
    pub safe_entry_count: usize,
    pub rejected_entry_count: usize,
    pub latest_transaction_id: Option<String>,
    pub latest_selected_source: Option<String>,
    pub latest_chain_hash: Option<u64>,
    pub latest_selected_checksum: Option<u64>,
    pub latest_rollback_anchor: Option<String>,
    pub replay_visible_output: bool,
    pub replay_selected_source: String,
    pub replay_selected_line_count: usize,
    pub replay_selected_lines_checksum: u64,
    pub validation: TableViewActivationLedgerValidation,
    pub replay: Option<TableViewActivationReplayReport>,
    pub entries: Vec<TableViewActivationLedgerEntry>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationLedgerSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub ledger_guards: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationLedgerBundle;

impl TableViewActivationLedgerBundle {
    pub fn snapshot(&self) -> TableViewActivationLedgerSnapshot {
        TableViewActivationLedgerSnapshot {
            class: "TableViewActivationLedgerSnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_ledger.hash_chain".to_string(),
                "table_view_activation_ledger.validate_chain".to_string(),
                "table_view_activation_ledger.replay_latest_safe_record".to_string(),
                "table_view_activation_ledger.rollback_on_chain_drift".to_string(),
            ],
            ledger_guards: vec![
                "contiguous_sequence_numbers".to_string(),
                "previous_hash_points_to_prior_entry".to_string(),
                "entry_hash_matches_record_fields".to_string(),
                "latest_replay_report_is_safe".to_string(),
            ],
            universal_property:
                "activation_records_form_a_unique_replayable_chain_only_when_each_local_record_hash_glues_to_the_next".to_string(),
        }
    }

    pub fn ledger_from_journal(
        &self,
        journal: &TableViewActivationJournal,
        fallback_legacy_lines: &[String],
        current_transaction_id: Option<&str>,
        policy: &TableViewActivationLedgerPolicy,
    ) -> TableViewActivationLedger {
        activation_ledger_from_journal(
            journal,
            fallback_legacy_lines,
            current_transaction_id,
            policy,
        )
    }

    pub fn ledger_for_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        config: &ArchitectureSwitchConfig,
        policy: &TableViewActivationLedgerPolicy,
    ) -> TableViewActivationLedger {
        activation_ledger_for_cli_args(args, legacy_lines, config, policy)
    }

    pub fn validate(
        &self,
        ledger: &TableViewActivationLedger,
        policy: &TableViewActivationLedgerPolicy,
    ) -> TableViewActivationLedgerValidation {
        validate_activation_ledger_entries(
            &ledger.entries,
            ledger.replay.as_ref(),
            policy,
        )
    }
}

pub fn bootstrap_table_view_activation_ledger() -> TableViewActivationLedgerBundle {
    TableViewActivationLedgerBundle
}

pub fn activation_ledger_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationLedgerPolicy,
) -> TableViewActivationLedger {
    let journal_policy = TableViewActivationJournalPolicy {
        include_selected_lines: true,
        include_selected_preview: policy.include_record_previews,
        preview_limit: policy.preview_limit,
        max_records: policy.max_records,
    };
    let journal = activation_journal_for_cli_args(args, legacy_lines, config, &journal_policy);
    activation_ledger_from_journal(
        &journal,
        legacy_lines,
        journal.latest_transaction_id.as_deref(),
        policy,
    )
}

pub fn activation_ledger_from_journal(
    journal: &TableViewActivationJournal,
    fallback_legacy_lines: &[String],
    current_transaction_id: Option<&str>,
    policy: &TableViewActivationLedgerPolicy,
) -> TableViewActivationLedger {
    let replay_policy = TableViewActivationReplayPolicy {
        include_selected_lines: true,
        include_journal_replay_report: true,
        preview_limit: policy.preview_limit,
        ..TableViewActivationReplayPolicy::default()
    };
    let replay = activation_replay_from_journal(
        journal,
        fallback_legacy_lines,
        current_transaction_id,
        &replay_policy,
    );
    let replay_visible_output = replay.replay_visible_output;
    let replay_selected_source = replay.selected_source.clone();
    let replay_selected_line_count = replay.selected_line_count;
    let replay_selected_lines_checksum = replay.selected_lines_checksum;

    let entries = activation_ledger_entries_from_records(&journal.records, policy);
    let validation = validate_activation_ledger_entries(&entries, Some(&replay), policy);
    let entry_count = entries.len();
    let safe_entry_count = entries.iter().filter(|entry| entry.safe_to_apply).count();
    let rejected_entry_count = entry_count.saturating_sub(safe_entry_count);
    let latest = entries.last();

    TableViewActivationLedger {
        class: "TableViewActivationLedger".to_string(),
        morphism: "table_view_activation_ledger.hash_chain".to_string(),
        entry_count,
        safe_entry_count,
        rejected_entry_count,
        latest_transaction_id: latest.map(|entry| entry.transaction_id.clone()),
        latest_selected_source: latest.map(|entry| entry.selected_source.clone()),
        latest_chain_hash: latest.map(|entry| entry.chain_hash),
        latest_selected_checksum: latest.map(|entry| entry.selected_lines_checksum),
        latest_rollback_anchor: latest.and_then(|entry| entry.rollback_anchor.clone()),
        replay_visible_output,
        replay_selected_source,
        replay_selected_line_count,
        replay_selected_lines_checksum,
        validation,
        replay: if policy.include_replay_report {
            Some(replay)
        } else {
            None
        },
        entries,
        universal_property:
            "a_guarded_activation_ledger_is_replayable_only_when_the_hash_chain_and_latest_replay_commute".to_string(),
    }
}

pub fn activation_ledger_entries_from_records(
    records: &[TableViewActivationJournalRecord],
    policy: &TableViewActivationLedgerPolicy,
) -> Vec<TableViewActivationLedgerEntry> {
    let mut previous_chain_hash = None;
    records
        .iter()
        .take(policy.max_records)
        .map(|record| {
            let mut entry = activation_ledger_entry_from_record(
                record,
                previous_chain_hash,
                policy,
            );
            entry.record_hash = activation_ledger_entry_record_hash(&entry);
            entry.chain_hash = activation_ledger_entry_chain_hash(
                entry.previous_chain_hash,
                entry.record_hash,
            );
            previous_chain_hash = Some(entry.chain_hash);
            entry
        })
        .collect()
}

pub fn activation_ledger_entry_from_record(
    record: &TableViewActivationJournalRecord,
    previous_chain_hash: Option<u64>,
    policy: &TableViewActivationLedgerPolicy,
) -> TableViewActivationLedgerEntry {
    TableViewActivationLedgerEntry {
        class: "TableViewActivationLedgerEntry".to_string(),
        morphism: "table_view_activation_ledger.hash_record".to_string(),
        sequence: record.sequence,
        transaction_id: record.transaction_id.clone(),
        previous_chain_hash,
        record_hash: 0,
        chain_hash: 0,
        selected_source: record.selected_source.clone(),
        should_replace_visible_output: record.should_replace_visible_output,
        safe_to_apply: record.safe_to_apply,
        reason: record.reason.clone(),
        rollback_anchor: record.rollback_anchor.clone(),
        selected_line_count: record.selected_line_count,
        legacy_line_count: record.legacy_line_count,
        view_output_line_count: record.view_output_line_count,
        selected_lines_checksum: record.selected_lines_checksum,
        legacy_lines_checksum: record.legacy_lines_checksum,
        view_output_lines_checksum: record.view_output_lines_checksum,
        raw_equal: record.raw_equal,
        semantic_equal: record.semantic_equal,
        virtual_direct_cells_equal: record.virtual_direct_cells_equal,
        virtual_added_column_count: record.virtual_added_column_count,
        required_failed_count: record.required_failed_count,
        failed_required_checks: record.failed_required_checks.clone(),
        selected_lines_preview: if policy.include_record_previews {
            record
                .selected_lines_preview
                .iter()
                .take(policy.preview_limit)
                .cloned()
                .collect()
        } else {
            Vec::new()
        },
        universal_property:
            "each_ledger_entry_hashes_one_activation_record_and_the_previous_chain_hash"
                .to_string(),
    }
}

pub fn activation_ledger_entry_record_hash(entry: &TableViewActivationLedgerEntry) -> u64 {
    stable_line_checksum(&[
        entry.sequence.to_string(),
        entry.transaction_id.clone(),
        entry.selected_source.clone(),
        entry.should_replace_visible_output.to_string(),
        entry.safe_to_apply.to_string(),
        entry.reason.clone(),
        entry.rollback_anchor.clone().unwrap_or_default(),
        entry.selected_line_count.to_string(),
        entry.legacy_line_count.to_string(),
        entry.view_output_line_count.to_string(),
        entry.selected_lines_checksum.to_string(),
        entry.legacy_lines_checksum.to_string(),
        entry.view_output_lines_checksum.to_string(),
        entry.raw_equal.to_string(),
        entry.semantic_equal.to_string(),
        entry.virtual_direct_cells_equal.to_string(),
        entry.virtual_added_column_count.to_string(),
        entry.required_failed_count.to_string(),
        entry.failed_required_checks.join("|"),
    ])
}

pub fn activation_ledger_entry_chain_hash(
    previous_chain_hash: Option<u64>,
    record_hash: u64,
) -> u64 {
    stable_line_checksum(&[
        previous_chain_hash
            .map(|value| value.to_string())
            .unwrap_or_else(|| "GENESIS".to_string()),
        record_hash.to_string(),
        "table_view_activation_ledger.chain".to_string(),
    ])
}

pub fn validate_activation_ledger_entries(
    entries: &[TableViewActivationLedgerEntry],
    replay: Option<&TableViewActivationReplayReport>,
    policy: &TableViewActivationLedgerPolicy,
) -> TableViewActivationLedgerValidation {
    let mut failed_guards = Vec::new();
    let mut previous_chain_hash = None;
    let mut contiguous_sequence = true;
    let mut hash_chain_valid = true;

    for (expected_sequence, entry) in entries.iter().enumerate() {
        if entry.sequence != expected_sequence {
            contiguous_sequence = false;
        }
        if entry.previous_chain_hash != previous_chain_hash {
            hash_chain_valid = false;
        }
        let expected_record_hash = activation_ledger_entry_record_hash(entry);
        if entry.record_hash != expected_record_hash {
            hash_chain_valid = false;
        }
        let expected_chain_hash = activation_ledger_entry_chain_hash(
            entry.previous_chain_hash,
            expected_record_hash,
        );
        if entry.chain_hash != expected_chain_hash {
            hash_chain_valid = false;
        }
        previous_chain_hash = Some(entry.chain_hash);
    }

    let latest_replay_safe = replay
        .map(|report| report.replay_safe && (report.replay_visible_output || report.selected_source == "legacy_output"))
        .unwrap_or_else(|| entries.is_empty());

    if policy.require_contiguous_sequence && !contiguous_sequence {
        failed_guards.push("sequence_numbers_not_contiguous".to_string());
    }
    if policy.require_hash_chain && !hash_chain_valid {
        failed_guards.push("hash_chain_invalid".to_string());
    }
    if policy.require_latest_replay_safe && !latest_replay_safe {
        failed_guards.push("latest_replay_not_safe".to_string());
    }

    let status = if failed_guards.is_empty() {
        "ready"
    } else {
        "blocked"
    };

    TableViewActivationLedgerValidation {
        class: "TableViewActivationLedgerValidation".to_string(),
        status: status.to_string(),
        entry_count: entries.len(),
        contiguous_sequence,
        hash_chain_valid,
        latest_replay_safe,
        latest_chain_hash: previous_chain_hash,
        failed_guards,
        universal_property:
            "a_ledger_validates_exactly_when_sequences_and_hash_edges_form_one_chain_and_latest_replay_is_safe".to_string(),
    }
}

pub fn continuum_m_activation_ledger_smoke() -> TableViewActivationLedger {
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
        "stage44-smoke",
    );
    let transaction = crate::table_view_activation_transaction::continuum_m_activation_transaction_smoke();
    activation_ledger_for_cli_args(
        &args,
        &transaction.selected_lines,
        &config,
        &TableViewActivationLedgerPolicy::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_ledger_hash_chain_validates_safe_smoke() {
        let ledger = continuum_m_activation_ledger_smoke();
        assert_eq!(ledger.entry_count, 1);
        assert!(ledger.validation.is_ready());
        assert!(ledger.validation.hash_chain_valid);
        assert!(ledger.replay_visible_output);
        assert_eq!(ledger.replay_selected_source, "table_view_output");
    }

    #[test]
    fn activation_ledger_detects_hash_chain_tampering() {
        let mut ledger = continuum_m_activation_ledger_smoke();
        ledger.entries[0].transaction_id.push_str(":tampered");
        let validation = validate_activation_ledger_entries(
            &ledger.entries,
            ledger.replay.as_ref(),
            &TableViewActivationLedgerPolicy::default(),
        );
        assert!(!validation.is_ready());
        assert!(validation.failed_guards.contains(&"hash_chain_invalid".to_string()));
    }

    #[test]
    fn activation_ledger_detects_sequence_drift() {
        let mut ledger = continuum_m_activation_ledger_smoke();
        ledger.entries[0].sequence = 7;
        let validation = validate_activation_ledger_entries(
            &ledger.entries,
            ledger.replay.as_ref(),
            &TableViewActivationLedgerPolicy::default(),
        );
        assert!(!validation.is_ready());
        assert!(validation.failed_guards.contains(&"sequence_numbers_not_contiguous".to_string()));
    }
}
