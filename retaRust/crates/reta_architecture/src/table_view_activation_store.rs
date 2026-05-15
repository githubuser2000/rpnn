//! Line-oriented persistence witness for guarded table-view activations.
//!
//! Stage 42 introduced journals, Stage 43 guarded replay and Stage 44 folded
//! records into a hash-chain ledger.  This module makes that chain portable:
//! a journal + ledger can be encoded as a deterministic, line-oriented store,
//! parsed back, re-validated, and rolled back to legacy lines when any local
//! section fails to glue.  It is deliberately not an external database backend;
//! it is the typed store morphism that a file, SQLite table or FFI boundary can
//! use without changing the activation logic.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::table_view_activation_journal::{
    TableViewActivationJournal, TableViewActivationJournalPolicy,
    TableViewActivationJournalRecord, activation_journal_for_cli_args,
    activation_journal_from_records,
};
use crate::table_view_activation_ledger::{
    TableViewActivationLedger, TableViewActivationLedgerEntry,
    TableViewActivationLedgerPolicy, activation_ledger_from_journal,
};
use crate::table_view_activation_transaction::stable_line_checksum;

const DEFAULT_FORMAT_VERSION: &str = "reta-activation-store-v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationStorePolicy {
    pub format_version: String,
    pub include_selected_lines: bool,
    pub include_ledger_entries: bool,
    pub require_header: bool,
    pub require_selected_line_checksums: bool,
    pub require_ledger_hashes: bool,
    pub max_records: usize,
    pub preview_limit: usize,
}

impl Default for TableViewActivationStorePolicy {
    fn default() -> Self {
        Self {
            format_version: DEFAULT_FORMAT_VERSION.to_string(),
            include_selected_lines: true,
            include_ledger_entries: true,
            require_header: true,
            require_selected_line_checksums: true,
            require_ledger_hashes: true,
            max_records: 64,
            preview_limit: 8,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationStoreValidation {
    pub class: String,
    pub status: String,
    pub header_valid: bool,
    pub sequence_valid: bool,
    pub selected_line_checksums_valid: bool,
    pub ledger_hashes_present: bool,
    pub ledger_hashes_match: bool,
    pub ledger_validation_status: String,
    pub record_count: usize,
    pub selected_line_count: usize,
    pub ledger_entry_count: usize,
    pub failed_guards: Vec<String>,
    pub parse_errors: Vec<String>,
    pub universal_property: String,
}

impl TableViewActivationStoreValidation {
    pub fn is_ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationStore {
    pub class: String,
    pub morphism: String,
    pub format_version: String,
    pub line_count: usize,
    pub record_count: usize,
    pub safe_record_count: usize,
    pub rejected_record_count: usize,
    pub selected_line_count: usize,
    pub ledger_entry_count: usize,
    pub latest_transaction_id: Option<String>,
    pub latest_chain_hash: Option<u64>,
    pub text_checksum: u64,
    pub store_text: String,
    pub validation: TableViewActivationStoreValidation,
    pub journal: TableViewActivationJournal,
    pub ledger: TableViewActivationLedger,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationStoreParseReport {
    pub class: String,
    pub morphism: String,
    pub parsed: bool,
    pub validation: TableViewActivationStoreValidation,
    pub store: Option<TableViewActivationStore>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationStoreSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub store_guards: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationStoreBundle;

impl TableViewActivationStoreBundle {
    pub fn snapshot(&self) -> TableViewActivationStoreSnapshot {
        TableViewActivationStoreSnapshot {
            class: "TableViewActivationStoreSnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_store.encode_line_store".to_string(),
                "table_view_activation_store.parse_line_store".to_string(),
                "table_view_activation_store.validate_stored_hash_chain".to_string(),
                "table_view_activation_store.rollback_on_store_drift".to_string(),
            ],
            store_guards: vec![
                "header_version_matches".to_string(),
                "record_sequences_are_contiguous".to_string(),
                "selected_line_checksums_match".to_string(),
                "stored_ledger_hashes_match_recomputed_ledger".to_string(),
            ],
            universal_property:
                "a_stored_activation_section_replays_only_when_header_records_lines_and_ledger_hashes_glue_back_to_one_valid_ledger".to_string(),
        }
    }

    pub fn store_for_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        config: &ArchitectureSwitchConfig,
        policy: &TableViewActivationStorePolicy,
    ) -> TableViewActivationStore {
        activation_store_for_cli_args(args, legacy_lines, config, policy)
    }

    pub fn parse_text(
        &self,
        text: &str,
        fallback_legacy_lines: &[String],
        current_transaction_id: Option<&str>,
        policy: &TableViewActivationStorePolicy,
    ) -> TableViewActivationStoreParseReport {
        parse_activation_store_text(text, fallback_legacy_lines, current_transaction_id, policy)
    }
}

pub fn bootstrap_table_view_activation_store() -> TableViewActivationStoreBundle {
    TableViewActivationStoreBundle
}

pub fn activation_store_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationStorePolicy,
) -> TableViewActivationStore {
    let journal_policy = TableViewActivationJournalPolicy {
        include_selected_lines: policy.include_selected_lines,
        include_selected_preview: true,
        preview_limit: policy.preview_limit,
        max_records: policy.max_records,
    };
    let journal = activation_journal_for_cli_args(args, legacy_lines, config, &journal_policy);
    let ledger_policy = TableViewActivationLedgerPolicy {
        include_record_previews: true,
        include_replay_report: true,
        max_records: policy.max_records,
        preview_limit: policy.preview_limit,
        ..TableViewActivationLedgerPolicy::default()
    };
    let ledger = activation_ledger_from_journal(
        &journal,
        legacy_lines,
        journal.latest_transaction_id.as_deref(),
        &ledger_policy,
    );
    activation_store_from_journal_and_ledger(&journal, &ledger, policy)
}

pub fn activation_store_from_journal_and_ledger(
    journal: &TableViewActivationJournal,
    ledger: &TableViewActivationLedger,
    policy: &TableViewActivationStorePolicy,
) -> TableViewActivationStore {
    let store_text = activation_store_text_from_journal_and_ledger(journal, ledger, policy);
    let line_count = store_text.lines().count();
    let selected_line_count = journal
        .records
        .iter()
        .map(|record| record.selected_lines.len())
        .sum::<usize>();
    let ledger_hashes_match = compare_journal_ledger_hashes(journal, ledger).is_empty();
    let selected_line_checksums_valid = journal
        .records
        .iter()
        .all(|record| selected_lines_checksum_matches(record));
    let sequence_valid = journal
        .records
        .iter()
        .enumerate()
        .all(|(idx, record)| record.sequence == idx);
    let mut failed_guards = Vec::new();
    if !sequence_valid {
        failed_guards.push("record_sequences_not_contiguous".to_string());
    }
    if policy.require_selected_line_checksums && !selected_line_checksums_valid {
        failed_guards.push("selected_line_checksum_mismatch".to_string());
    }
    if policy.require_ledger_hashes && !ledger_hashes_match {
        failed_guards.push("ledger_hashes_do_not_match_journal".to_string());
    }
    if !ledger.validation.is_ready() {
        failed_guards.push("ledger_validation_not_ready".to_string());
    }
    let status = if failed_guards.is_empty() { "ready" } else { "blocked" };
    let validation = TableViewActivationStoreValidation {
        class: "TableViewActivationStoreValidation".to_string(),
        status: status.to_string(),
        header_valid: true,
        sequence_valid,
        selected_line_checksums_valid,
        ledger_hashes_present: policy.include_ledger_entries && !ledger.entries.is_empty(),
        ledger_hashes_match,
        ledger_validation_status: ledger.validation.status.clone(),
        record_count: journal.record_count,
        selected_line_count,
        ledger_entry_count: ledger.entry_count,
        failed_guards,
        parse_errors: Vec::new(),
        universal_property:
            "stored_activation_is_ready_exactly_when_local_journal_and_ledger_hash_sections_glue".to_string(),
    };
    TableViewActivationStore {
        class: "TableViewActivationStore".to_string(),
        morphism: "table_view_activation_store.encode_line_store".to_string(),
        format_version: policy.format_version.clone(),
        line_count,
        record_count: journal.record_count,
        safe_record_count: journal.safe_record_count,
        rejected_record_count: journal.rejected_record_count,
        selected_line_count,
        ledger_entry_count: ledger.entry_count,
        latest_transaction_id: journal.latest_transaction_id.clone(),
        latest_chain_hash: ledger.latest_chain_hash,
        text_checksum: stable_line_checksum(&store_text.lines().map(ToString::to_string).collect::<Vec<_>>()),
        store_text,
        validation,
        journal: journal.clone(),
        ledger: ledger.clone(),
        universal_property:
            "line_oriented_activation_store_is_a_persistent_presheaf_section_for_the_activation_ledger".to_string(),
    }
}

pub fn activation_store_text_from_journal_and_ledger(
    journal: &TableViewActivationJournal,
    ledger: &TableViewActivationLedger,
    policy: &TableViewActivationStorePolicy,
) -> String {
    let mut lines = Vec::new();
    lines.push(join_fields(&[
        "H".to_string(),
        policy.format_version.clone(),
        journal.record_count.to_string(),
        ledger.entry_count.to_string(),
        ledger.latest_chain_hash.map(|value| value.to_string()).unwrap_or_default(),
    ]));

    for record in journal.records.iter().take(policy.max_records) {
        lines.push(encode_journal_record(record));
        if policy.include_selected_lines {
            for (line_index, line) in record.selected_lines.iter().enumerate() {
                lines.push(join_fields(&[
                    "L".to_string(),
                    record.sequence.to_string(),
                    line_index.to_string(),
                    line.clone(),
                ]));
            }
        }
    }

    if policy.include_ledger_entries {
        for entry in ledger.entries.iter().take(policy.max_records) {
            lines.push(encode_ledger_entry(entry));
        }
    }

    let mut text = lines.join("\n");
    text.push('\n');
    text
}

pub fn parse_activation_store_text(
    text: &str,
    fallback_legacy_lines: &[String],
    current_transaction_id: Option<&str>,
    policy: &TableViewActivationStorePolicy,
) -> TableViewActivationStoreParseReport {
    let mut parse_errors = Vec::new();
    let mut header_valid = !policy.require_header;
    let mut record_stubs = BTreeMap::<usize, TableViewActivationJournalRecord>::new();
    let mut selected_lines = BTreeMap::<usize, Vec<(usize, String)>>::new();
    let mut stored_ledger_hashes = BTreeMap::<usize, (Option<u64>, u64, u64)>::new();

    for (line_no, raw_line) in text.lines().enumerate() {
        if raw_line.trim().is_empty() {
            continue;
        }
        let fields = split_fields(raw_line);
        if fields.is_empty() {
            continue;
        }
        match fields[0].as_str() {
            "H" => {
                header_valid = fields.get(1).map(|value| value == &policy.format_version).unwrap_or(false);
                if !header_valid {
                    parse_errors.push(format!("line {line_no}: header_version_mismatch"));
                }
            }
            "J" => match decode_journal_record(&fields) {
                Ok(record) => {
                    record_stubs.insert(record.sequence, record);
                }
                Err(error) => parse_errors.push(format!("line {line_no}: {error}")),
            },
            "L" => {
                if fields.len() != 4 {
                    parse_errors.push(format!("line {line_no}: selected_line_field_count"));
                    continue;
                }
                let Some(sequence) = parse_usize(&fields[1]) else {
                    parse_errors.push(format!("line {line_no}: selected_line_bad_sequence"));
                    continue;
                };
                let Some(index) = parse_usize(&fields[2]) else {
                    parse_errors.push(format!("line {line_no}: selected_line_bad_index"));
                    continue;
                };
                selected_lines
                    .entry(sequence)
                    .or_default()
                    .push((index, fields[3].clone()));
            }
            "G" => match decode_ledger_hash_fields(&fields) {
                Ok((sequence, previous, record_hash, chain_hash)) => {
                    stored_ledger_hashes.insert(sequence, (previous, record_hash, chain_hash));
                }
                Err(error) => parse_errors.push(format!("line {line_no}: {error}")),
            },
            other => parse_errors.push(format!("line {line_no}: unknown_record_kind:{other}")),
        }
    }

    let mut records = Vec::new();
    for (_, mut record) in record_stubs {
        if let Some(mut lines) = selected_lines.remove(&record.sequence) {
            lines.sort_by_key(|(index, _)| *index);
            record.selected_lines = lines.into_iter().map(|(_, line)| line).collect();
        }
        records.push(record);
    }
    records.sort_by_key(|record| record.sequence);

    let sequence_valid = records
        .iter()
        .enumerate()
        .all(|(idx, record)| record.sequence == idx);
    let selected_line_checksums_valid = records
        .iter()
        .all(|record| selected_lines_checksum_matches(record));
    let journal = activation_journal_from_records(records);
    let ledger_policy = TableViewActivationLedgerPolicy {
        include_record_previews: true,
        include_replay_report: true,
        max_records: policy.max_records,
        preview_limit: policy.preview_limit,
        ..TableViewActivationLedgerPolicy::default()
    };
    let ledger = activation_ledger_from_journal(
        &journal,
        fallback_legacy_lines,
        current_transaction_id.or(journal.latest_transaction_id.as_deref()),
        &ledger_policy,
    );
    let ledger_hashes_present = !stored_ledger_hashes.is_empty();
    let ledger_hash_mismatches = compare_stored_ledger_hashes(&stored_ledger_hashes, &ledger);
    let ledger_hashes_match = ledger_hash_mismatches.is_empty();

    let mut failed_guards = Vec::new();
    if policy.require_header && !header_valid {
        failed_guards.push("header_version_invalid".to_string());
    }
    if !sequence_valid {
        failed_guards.push("record_sequences_not_contiguous".to_string());
    }
    if policy.require_selected_line_checksums && !selected_line_checksums_valid {
        failed_guards.push("selected_line_checksum_mismatch".to_string());
    }
    if policy.require_ledger_hashes && !ledger_hashes_present {
        failed_guards.push("stored_ledger_hashes_missing".to_string());
    }
    if policy.require_ledger_hashes && !ledger_hashes_match {
        failed_guards.push("stored_ledger_hashes_mismatch".to_string());
        failed_guards.extend(ledger_hash_mismatches);
    }
    if !ledger.validation.is_ready() {
        failed_guards.push("ledger_validation_not_ready".to_string());
        failed_guards.extend(ledger.validation.failed_guards.clone());
    }
    failed_guards.extend(parse_errors.iter().cloned());

    let selected_line_count = journal
        .records
        .iter()
        .map(|record| record.selected_lines.len())
        .sum::<usize>();
    let ledger_entry_count = ledger.entry_count;
    let status = if failed_guards.is_empty() { "ready" } else { "blocked" };
    let validation = TableViewActivationStoreValidation {
        class: "TableViewActivationStoreValidation".to_string(),
        status: status.to_string(),
        header_valid,
        sequence_valid,
        selected_line_checksums_valid,
        ledger_hashes_present,
        ledger_hashes_match,
        ledger_validation_status: ledger.validation.status.clone(),
        record_count: journal.record_count,
        selected_line_count,
        ledger_entry_count,
        failed_guards,
        parse_errors: parse_errors.clone(),
        universal_property:
            "a_parsed_activation_store_is_ready_only_when_records_lines_and_hashes_reconstruct_the_same_ledger".to_string(),
    };

    let store = if parse_errors.is_empty() || !journal.records.is_empty() {
        Some(TableViewActivationStore {
            class: "TableViewActivationStore".to_string(),
            morphism: "table_view_activation_store.parse_line_store".to_string(),
            format_version: policy.format_version.clone(),
            line_count: text.lines().count(),
            record_count: journal.record_count,
            safe_record_count: journal.safe_record_count,
            rejected_record_count: journal.rejected_record_count,
            selected_line_count,
            ledger_entry_count,
            latest_transaction_id: journal.latest_transaction_id.clone(),
            latest_chain_hash: ledger.latest_chain_hash,
            text_checksum: stable_line_checksum(&text.lines().map(ToString::to_string).collect::<Vec<_>>()),
            store_text: text.to_string(),
            validation: validation.clone(),
            journal,
            ledger,
            universal_property:
                "parsed_activation_store_reconstructs_a_journal_and_hash_chain_from_stable_lines".to_string(),
        })
    } else {
        None
    };

    TableViewActivationStoreParseReport {
        class: "TableViewActivationStoreParseReport".to_string(),
        morphism: "table_view_activation_store.parse_line_store".to_string(),
        parsed: store.is_some(),
        validation,
        store,
        universal_property:
            "store_parse_is_a_partial_inverse_of_store_encode_when_all_local_sections_validate".to_string(),
    }
}

pub fn continuum_m_activation_store_smoke() -> TableViewActivationStore {
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
        "stage45-smoke",
    );
    let transaction = crate::table_view_activation_transaction::continuum_m_activation_transaction_smoke();
    activation_store_for_cli_args(
        &args,
        &transaction.selected_lines,
        &config,
        &TableViewActivationStorePolicy::default(),
    )
}

fn selected_lines_checksum_matches(record: &TableViewActivationJournalRecord) -> bool {
    !record.selected_lines.is_empty()
        && record.selected_lines.len() == record.selected_line_count
        && stable_line_checksum(&record.selected_lines) == record.selected_lines_checksum
}

fn compare_journal_ledger_hashes(
    journal: &TableViewActivationJournal,
    ledger: &TableViewActivationLedger,
) -> Vec<String> {
    let expected_records = journal.records.iter().map(|record| record.sequence).collect::<Vec<_>>();
    let actual_records = ledger.entries.iter().map(|entry| entry.sequence).collect::<Vec<_>>();
    if expected_records != actual_records {
        return vec!["journal_ledger_sequence_mismatch".to_string()];
    }
    Vec::new()
}

fn compare_stored_ledger_hashes(
    stored: &BTreeMap<usize, (Option<u64>, u64, u64)>,
    ledger: &TableViewActivationLedger,
) -> Vec<String> {
    let mut errors = Vec::new();
    for entry in &ledger.entries {
        match stored.get(&entry.sequence) {
            Some((previous, record_hash, chain_hash)) => {
                if previous != &entry.previous_chain_hash {
                    errors.push(format!("ledger_previous_hash_mismatch:{}", entry.sequence));
                }
                if record_hash != &entry.record_hash {
                    errors.push(format!("ledger_record_hash_mismatch:{}", entry.sequence));
                }
                if chain_hash != &entry.chain_hash {
                    errors.push(format!("ledger_chain_hash_mismatch:{}", entry.sequence));
                }
            }
            None => errors.push(format!("ledger_hash_missing:{}", entry.sequence)),
        }
    }
    for sequence in stored.keys() {
        if !ledger.entries.iter().any(|entry| &entry.sequence == sequence) {
            errors.push(format!("ledger_hash_extra:{sequence}"));
        }
    }
    errors
}

fn encode_journal_record(record: &TableViewActivationJournalRecord) -> String {
    join_fields(&[
        "J".to_string(),
        record.sequence.to_string(),
        record.transaction_id.clone(),
        record.switch_mode.clone(),
        record.selected_source.clone(),
        record.should_replace_visible_output.to_string(),
        record.safe_to_apply.to_string(),
        record.reason.clone(),
        record.commit_reason.clone(),
        record.rollback_anchor.clone().unwrap_or_default(),
        record.selected_line_count.to_string(),
        record.legacy_line_count.to_string(),
        record.view_output_line_count.to_string(),
        record.selected_lines_checksum.to_string(),
        record.legacy_lines_checksum.to_string(),
        record.view_output_lines_checksum.to_string(),
        record.raw_equal.to_string(),
        record.semantic_equal.to_string(),
        record.virtual_direct_cells_equal.to_string(),
        record.virtual_added_column_count.to_string(),
        record.required_failed_count.to_string(),
        join_list(&record.failed_required_checks),
        join_list(&record.selected_lines_preview),
        record.universal_property.clone(),
    ])
}

fn decode_journal_record(fields: &[String]) -> Result<TableViewActivationJournalRecord, String> {
    if fields.len() != 24 {
        return Err(format!("journal_record_field_count:{}", fields.len()));
    }
    Ok(TableViewActivationJournalRecord {
        class: "TableViewActivationJournalRecord".to_string(),
        morphism: "table_view_activation_journal.record_transaction".to_string(),
        sequence: parse_usize_required(&fields[1], "sequence")?,
        transaction_id: fields[2].clone(),
        switch_mode: fields[3].clone(),
        selected_source: fields[4].clone(),
        should_replace_visible_output: parse_bool_required(&fields[5], "should_replace")?,
        safe_to_apply: parse_bool_required(&fields[6], "safe_to_apply")?,
        reason: fields[7].clone(),
        commit_reason: fields[8].clone(),
        rollback_anchor: optional_string(&fields[9]),
        selected_line_count: parse_usize_required(&fields[10], "selected_line_count")?,
        legacy_line_count: parse_usize_required(&fields[11], "legacy_line_count")?,
        view_output_line_count: parse_usize_required(&fields[12], "view_output_line_count")?,
        selected_lines_checksum: parse_u64_required(&fields[13], "selected_checksum")?,
        legacy_lines_checksum: parse_u64_required(&fields[14], "legacy_checksum")?,
        view_output_lines_checksum: parse_u64_required(&fields[15], "view_checksum")?,
        raw_equal: parse_bool_required(&fields[16], "raw_equal")?,
        semantic_equal: parse_bool_required(&fields[17], "semantic_equal")?,
        virtual_direct_cells_equal: parse_bool_required(&fields[18], "virtual_direct")?,
        virtual_added_column_count: parse_usize_required(&fields[19], "virtual_added")?,
        required_failed_count: parse_usize_required(&fields[20], "required_failed_count")?,
        failed_required_checks: split_list(&fields[21]),
        selected_lines_preview: split_list(&fields[22]),
        selected_lines: Vec::new(),
        universal_property: fields[23].clone(),
    })
}

fn encode_ledger_entry(entry: &TableViewActivationLedgerEntry) -> String {
    join_fields(&[
        "G".to_string(),
        entry.sequence.to_string(),
        entry.previous_chain_hash.map(|value| value.to_string()).unwrap_or_default(),
        entry.record_hash.to_string(),
        entry.chain_hash.to_string(),
    ])
}

fn decode_ledger_hash_fields(fields: &[String]) -> Result<(usize, Option<u64>, u64, u64), String> {
    if fields.len() != 5 {
        return Err(format!("ledger_hash_field_count:{}", fields.len()));
    }
    let sequence = parse_usize_required(&fields[1], "ledger_sequence")?;
    let previous = if fields[2].is_empty() {
        None
    } else {
        Some(parse_u64_required(&fields[2], "previous_chain_hash")?)
    };
    let record_hash = parse_u64_required(&fields[3], "record_hash")?;
    let chain_hash = parse_u64_required(&fields[4], "chain_hash")?;
    Ok((sequence, previous, record_hash, chain_hash))
}

fn join_fields(fields: &[String]) -> String {
    fields
        .iter()
        .map(|field| escape_field(field))
        .collect::<Vec<_>>()
        .join("\t")
}

fn split_fields(line: &str) -> Vec<String> {
    line.split('\t').map(unescape_field).collect()
}

fn join_list(values: &[String]) -> String {
    values.join("\u{1f}")
}

fn split_list(value: &str) -> Vec<String> {
    if value.is_empty() {
        Vec::new()
    } else {
        value.split('\u{1f}').map(ToString::to_string).collect()
    }
}

fn escape_field(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\u{1f}' => out.push_str("\\x1f"),
            other => out.push(other),
        }
    }
    out
}

fn unescape_field(value: &str) -> String {
    let mut out = String::new();
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('x') => {
                if chars.next() == Some('1') && chars.next() == Some('f') {
                    out.push('\u{1f}');
                } else {
                    out.push_str("\\x");
                }
            }
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

fn optional_string(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn parse_bool_required(value: &str, name: &str) -> Result<bool, String> {
    value
        .parse::<bool>()
        .map_err(|_| format!("bad_bool:{name}:{value}"))
}

fn parse_usize(value: &str) -> Option<usize> {
    value.parse::<usize>().ok()
}

fn parse_usize_required(value: &str, name: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .map_err(|_| format!("bad_usize:{name}:{value}"))
}

fn parse_u64_required(value: &str, name: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|_| format!("bad_u64:{name}:{value}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_store_roundtrips_continuum_smoke() {
        let store = continuum_m_activation_store_smoke();
        assert!(store.validation.is_ready());
        let fallback = store
            .journal
            .records
            .last()
            .map(|record| record.selected_lines.clone())
            .unwrap_or_default();
        let report = parse_activation_store_text(
            &store.store_text,
            &fallback,
            store.latest_transaction_id.as_deref(),
            &TableViewActivationStorePolicy::default(),
        );
        assert!(report.parsed);
        assert!(report.validation.is_ready());
        let parsed = report.store.expect("parsed store");
        assert_eq!(parsed.latest_chain_hash, store.latest_chain_hash);
        assert_eq!(parsed.record_count, store.record_count);
    }

    #[test]
    fn activation_store_detects_tampered_record() {
        let store = continuum_m_activation_store_smoke();
        let tampered = store.store_text.replace("table_view_output", "tampered_output");
        let fallback = store
            .journal
            .records
            .last()
            .map(|record| record.selected_lines.clone())
            .unwrap_or_default();
        let report = parse_activation_store_text(
            &tampered,
            &fallback,
            store.latest_transaction_id.as_deref(),
            &TableViewActivationStorePolicy::default(),
        );
        assert!(!report.validation.is_ready());
        assert!(
            report
                .validation
                .failed_guards
                .iter()
                .any(|item| item.contains("ledger_record_hash_mismatch"))
        );
    }

    #[test]
    fn activation_store_escapes_selected_lines() {
        let text = "a\tb\n\\c\u{1f}d";
        let encoded = escape_field(text);
        assert_eq!(unescape_field(&encoded), text);
    }
}
