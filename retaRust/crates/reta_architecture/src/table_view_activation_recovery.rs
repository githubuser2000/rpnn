//! Recovery witness for file-backed table-view activation stores.
//!
//! Stage 47 made guarded activation stores writable to files.  This module adds
//! the inverse boundary: an existing activation-store file can be read, parsed,
//! compared with the current transaction and legacy checksum, and only then used
//! as a recovery candidate.  The default policy is diagnostic only; visible
//! recovery requires an explicit flag and still has to pass replay guards.

use std::fs;

use serde::{Deserialize, Serialize};

use crate::persistence::stable_digest_text;
use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::table_view_activation_replay::{
    TableViewActivationReplayPolicy, TableViewActivationReplayReport,
    activation_replay_from_journal,
};
use crate::table_view_activation_store::{
    TableViewActivationStorePolicy, parse_activation_store_text,
};
use crate::table_view_activation_transaction::{
    TableViewActivationTransactionPolicy, stable_line_checksum,
    table_view_activation_transaction_for_cli_args,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationRecoveryPolicy {
    pub store_policy: TableViewActivationStorePolicy,
    pub replay_policy: TableViewActivationReplayPolicy,
    pub file_path: Option<String>,
    pub read_file: bool,
    pub parse_after_read: bool,
    pub require_file_path: bool,
    pub require_parse_ready: bool,
    pub require_replay_safe: bool,
    pub allow_visible_recovery: bool,
    pub include_selected_lines: bool,
    pub include_file_preview: bool,
    pub preview_limit: usize,
}

impl Default for TableViewActivationRecoveryPolicy {
    fn default() -> Self {
        Self {
            store_policy: TableViewActivationStorePolicy::default(),
            replay_policy: TableViewActivationReplayPolicy::default(),
            file_path: None,
            read_file: true,
            parse_after_read: true,
            require_file_path: true,
            require_parse_ready: true,
            require_replay_safe: true,
            allow_visible_recovery: false,
            include_selected_lines: true,
            include_file_preview: true,
            preview_limit: 8,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationRecoveryReport {
    pub class: String,
    pub morphism: String,
    pub status: String,
    pub enabled: bool,
    pub path: Option<String>,
    pub current_transaction_id: Option<String>,
    pub current_legacy_checksum: u64,
    pub read_file: bool,
    pub read_error: Option<String>,
    pub read_text_digest: Option<String>,
    pub read_byte_len: Option<usize>,
    pub parsed: bool,
    pub parse_ready: bool,
    pub parse_validation_status: Option<String>,
    pub parse_failed_guards: Vec<String>,
    pub store_latest_transaction_id: Option<String>,
    pub store_record_count: usize,
    pub store_selected_line_count: usize,
    pub store_text_checksum: Option<u64>,
    pub store_line_count: usize,
    pub replay_visible_output: bool,
    pub replay_safe: bool,
    pub selected_source: String,
    pub reason: String,
    pub selected_line_count: usize,
    pub selected_lines_checksum: u64,
    pub selected_lines_preview: Vec<String>,
    pub selected_lines: Vec<String>,
    pub recover_visible_output: bool,
    pub replay_report: Option<TableViewActivationReplayReport>,
    pub file_preview: Vec<String>,
    pub failed_guards: Vec<String>,
    pub universal_property: String,
}

impl TableViewActivationRecoveryReport {
    pub fn is_ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationRecoverySnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub recovery_guards: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationRecoveryBundle;

impl TableViewActivationRecoveryBundle {
    pub fn snapshot(&self) -> TableViewActivationRecoverySnapshot {
        TableViewActivationRecoverySnapshot {
            class: "TableViewActivationRecoverySnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_recovery.read_existing_store_file".to_string(),
                "table_view_activation_recovery.parse_existing_store".to_string(),
                "table_view_activation_recovery.match_current_transaction".to_string(),
                "table_view_activation_recovery.replay_or_rollback".to_string(),
            ],
            recovery_guards: vec![
                "recovery_file_path_present".to_string(),
                "store_file_readable".to_string(),
                "parsed_store_validation_ready".to_string(),
                "journal_replay_matches_current_transaction_and_legacy_checksum".to_string(),
                "visible_recovery_requires_explicit_policy".to_string(),
            ],
            universal_property:
                "file_recovery_is_identity_on_current_safe_activation_store_and_rollback_on_file_or_checksum_drift".to_string(),
        }
    }

    pub fn recovery_for_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        config: &ArchitectureSwitchConfig,
        policy: &TableViewActivationRecoveryPolicy,
    ) -> TableViewActivationRecoveryReport {
        activation_recovery_for_cli_args(args, legacy_lines, config, policy)
    }
}

pub fn bootstrap_table_view_activation_recovery() -> TableViewActivationRecoveryBundle {
    TableViewActivationRecoveryBundle
}

pub fn activation_recovery_policy_from_cli_args(
    argv: &[String],
    base: &TableViewActivationRecoveryPolicy,
) -> (TableViewActivationRecoveryPolicy, bool) {
    let mut policy = base.clone();
    let mut enabled = false;
    if let Ok(path) = std::env::var("RETA_ARCH_ACTIVATION_RECOVERY_FILE") {
        if !path.trim().is_empty() {
            policy.file_path = Some(path);
            enabled = true;
        }
    }
    if let Ok(value) = std::env::var("RETA_ARCH_ACTIVATION_RECOVERY") {
        if truthy(&value) {
            enabled = true;
        }
    }
    let mut index = 0usize;
    while index < argv.len() {
        let arg = &argv[index];
        match arg.as_str() {
            "--activation-recovery" | "--activation-store-recovery" | "--reta-arch-recovery" => {
                enabled = true;
            }
            "--activation-recovery-allow-replay" | "--activation-store-recovery-allow-replay" => {
                policy.allow_visible_recovery = true;
                enabled = true;
            }
            "--activation-recovery-no-replay" | "--activation-store-recovery-no-replay" => {
                policy.allow_visible_recovery = false;
                enabled = true;
            }
            "--no-activation-recovery" | "--reta-arch-no-recovery" => {
                enabled = false;
                policy.file_path = None;
            }
            "--activation-recovery-file"
            | "--activation-recover-file"
            | "--activation-store-recover"
            | "--reta-arch-recovery-file"
            | "--reta-arch-recover-file" => {
                if let Some(value) = argv.get(index + 1) {
                    policy.file_path = Some(value.clone());
                    enabled = true;
                    index += 1;
                }
            }
            _ if arg.starts_with("--activation-recovery-file=")
                || arg.starts_with("--activation-recover-file=")
                || arg.starts_with("--activation-store-recover=")
                || arg.starts_with("--reta-arch-recovery-file=")
                || arg.starts_with("--reta-arch-recover-file=") =>
            {
                if let Some((_, value)) = arg.split_once('=') {
                    policy.file_path = Some(value.to_string());
                    enabled = true;
                }
            }
            _ => {}
        }
        index += 1;
    }
    (policy, enabled)
}

pub fn activation_recovery_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationRecoveryPolicy,
) -> TableViewActivationRecoveryReport {
    let current_transaction = table_view_activation_transaction_for_cli_args(
        args,
        legacy_lines,
        config,
        &TableViewActivationTransactionPolicy::default(),
    );
    read_activation_store_file_for_recovery(
        policy.file_path.as_deref(),
        legacy_lines,
        Some(current_transaction.transaction_id.as_str()),
        policy,
    )
}

pub fn read_activation_store_file_for_recovery(
    path: Option<&str>,
    legacy_lines: &[String],
    current_transaction_id: Option<&str>,
    policy: &TableViewActivationRecoveryPolicy,
) -> TableViewActivationRecoveryReport {
    let current_legacy_checksum = stable_line_checksum(legacy_lines);
    let mut failed_guards = Vec::new();
    let path_string = path.map(ToString::to_string).or_else(|| policy.file_path.clone());

    if policy.require_file_path && path_string.is_none() {
        failed_guards.push("recovery_file_path_missing".to_string());
    }

    let mut read_error = None;
    let loaded_text = if policy.read_file {
        match path_string.as_ref() {
            Some(path) => match fs::read_to_string(path) {
                Ok(text) => Some(text),
                Err(error) => {
                    read_error = Some(error.to_string());
                    None
                }
            },
            None => None,
        }
    } else {
        None
    };
    let read_file = loaded_text.is_some();
    if policy.read_file && path_string.is_some() && !read_file {
        failed_guards.push("recovery_file_read_failed".to_string());
    }
    if let Some(error) = read_error.as_ref() {
        failed_guards.push(format!("read_error:{error}"));
    }

    let read_text_digest = loaded_text.as_ref().map(|text| stable_digest_text(text));
    let read_byte_len = loaded_text.as_ref().map(|text| text.as_bytes().len());
    let file_preview = if policy.include_file_preview {
        loaded_text
            .as_deref()
            .unwrap_or("")
            .lines()
            .take(policy.preview_limit)
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let parse_report = if policy.parse_after_read {
        loaded_text.as_ref().map(|text| {
            parse_activation_store_text(
                text,
                legacy_lines,
                current_transaction_id,
                &policy.store_policy,
            )
        })
    } else {
        None
    };
    let parsed = parse_report.as_ref().map(|report| report.parsed).unwrap_or(false);
    let parse_ready = parse_report
        .as_ref()
        .map(|report| report.validation.is_ready())
        .unwrap_or(!policy.parse_after_read);
    let parse_validation_status = parse_report
        .as_ref()
        .map(|report| report.validation.status.clone());
    let parse_failed_guards = parse_report
        .as_ref()
        .map(|report| report.validation.failed_guards.clone())
        .unwrap_or_default();
    if policy.require_parse_ready && !parse_ready {
        failed_guards.push("recovered_store_parse_not_ready".to_string());
    }

    let store = parse_report.as_ref().and_then(|report| report.store.as_ref());
    let replay_report = store.map(|store| {
        activation_replay_from_journal(
            &store.journal,
            legacy_lines,
            current_transaction_id,
            &policy.replay_policy,
        )
    });
    let replay_visible_output = replay_report
        .as_ref()
        .map(|report| report.replay_visible_output)
        .unwrap_or(false);
    let replay_safe = replay_report
        .as_ref()
        .map(|report| report.replay_safe)
        .unwrap_or(false);
    if policy.require_replay_safe && !replay_safe {
        failed_guards.push("recovered_store_replay_not_safe".to_string());
    }

    let candidate_lines = replay_report
        .as_ref()
        .map(|report| report.selected_lines.clone())
        .unwrap_or_else(|| legacy_lines.to_vec());
    let selected_line_count = candidate_lines.len();
    let selected_lines_checksum = stable_line_checksum(&candidate_lines);
    let selected_lines_preview = candidate_lines
        .iter()
        .take(policy.preview_limit)
        .cloned()
        .collect::<Vec<_>>();
    let selected_lines = if policy.include_selected_lines {
        candidate_lines
    } else {
        Vec::new()
    };

    let recover_visible_output = policy.allow_visible_recovery && failed_guards.is_empty() && replay_visible_output;

    let status = if path_string.is_none() && !policy.require_file_path {
        "disabled"
    } else if failed_guards.is_empty() {
        "ready"
    } else {
        "blocked"
    };
    let selected_source = if recover_visible_output {
        "activation_file_recovery".to_string()
    } else if replay_safe {
        "activation_file_recovery_candidate".to_string()
    } else {
        "legacy_output".to_string()
    };
    let reason = if recover_visible_output {
        "existing_activation_store_replayed_under_current_guards".to_string()
    } else if replay_safe {
        "existing_activation_store_is_replay_safe_but_visible_recovery_is_diagnostic_only".to_string()
    } else if !failed_guards.is_empty() {
        format!("rollback_to_legacy:{failed_guards:?}")
    } else {
        "no_recovery_file_loaded".to_string()
    };

    TableViewActivationRecoveryReport {
        class: "TableViewActivationRecoveryReport".to_string(),
        morphism: "table_view_activation_recovery.read_parse_replay_existing_store".to_string(),
        status: status.to_string(),
        enabled: path_string.is_some() || !failed_guards.is_empty(),
        path: path_string,
        current_transaction_id: current_transaction_id.map(ToString::to_string),
        current_legacy_checksum,
        read_file,
        read_error,
        read_text_digest,
        read_byte_len,
        parsed,
        parse_ready,
        parse_validation_status,
        parse_failed_guards,
        store_latest_transaction_id: store.and_then(|store| store.latest_transaction_id.clone()),
        store_record_count: store.map(|store| store.record_count).unwrap_or(0),
        store_selected_line_count: store.map(|store| store.selected_line_count).unwrap_or(0),
        store_text_checksum: store.map(|store| store.text_checksum),
        store_line_count: store.map(|store| store.line_count).unwrap_or(0),
        replay_visible_output,
        replay_safe,
        selected_source,
        reason,
        selected_line_count,
        selected_lines_checksum,
        selected_lines_preview,
        selected_lines,
        recover_visible_output,
        replay_report,
        file_preview,
        failed_guards,
        universal_property:
            "activation_file_recovery_replays_only_when_existing_store_matches_current_transaction_and_legacy_checksum".to_string(),
    }
}

pub fn continuum_m_activation_recovery_smoke() -> TableViewActivationRecoveryReport {
    let store = crate::table_view_activation_store::continuum_m_activation_store_smoke();
    let path = std::env::temp_dir().join(format!(
        "reta-stage48-recovery-{:016x}.txt",
        store.text_checksum
    ));
    let _ = fs::write(&path, store.store_text.as_bytes());
    let legacy_lines = store
        .journal
        .records
        .last()
        .map(|record| record.selected_lines.clone())
        .unwrap_or_default();
    let mut policy = TableViewActivationRecoveryPolicy::default();
    policy.file_path = Some(path.to_string_lossy().into_owned());
    let report = read_activation_store_file_for_recovery(
        policy.file_path.as_deref(),
        &legacy_lines,
        store.latest_transaction_id.as_deref(),
        &policy,
    );
    let _ = fs::remove_file(path);
    report
}

fn truthy(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on" | "recover" | "recovery"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_recovery_replays_existing_safe_store_as_candidate() {
        let report = continuum_m_activation_recovery_smoke();
        assert!(report.is_ready());
        assert!(report.read_file);
        assert!(report.parsed);
        assert!(report.parse_ready);
        assert!(report.replay_safe);
        assert_eq!(report.selected_source, "activation_file_recovery_candidate");
        assert!(!report.recover_visible_output);
    }

    #[test]
    fn activation_recovery_allows_visible_recovery_only_when_policy_allows_it() {
        let store = crate::table_view_activation_store::continuum_m_activation_store_smoke();
        let path = std::env::temp_dir().join(format!(
            "reta-stage48-recovery-allow-{:016x}.txt",
            store.text_checksum
        ));
        let _ = fs::write(&path, store.store_text.as_bytes());
        let legacy_lines = store
            .journal
            .records
            .last()
            .map(|record| record.selected_lines.clone())
            .unwrap_or_default();
        let mut policy = TableViewActivationRecoveryPolicy::default();
        policy.file_path = Some(path.to_string_lossy().into_owned());
        policy.allow_visible_recovery = true;
        let report = read_activation_store_file_for_recovery(
            policy.file_path.as_deref(),
            &legacy_lines,
            store.latest_transaction_id.as_deref(),
            &policy,
        );
        let _ = fs::remove_file(path);
        assert!(report.is_ready());
        assert!(report.recover_visible_output);
    }

    #[test]
    fn activation_recovery_rolls_back_on_missing_file() {
        let policy = TableViewActivationRecoveryPolicy {
            file_path: Some("/definitely/missing/reta-activation-store.txt".to_string()),
            ..TableViewActivationRecoveryPolicy::default()
        };
        let report = read_activation_store_file_for_recovery(
            policy.file_path.as_deref(),
            &["legacy".to_string()],
            Some("tx"),
            &policy,
        );
        assert_eq!(report.status, "blocked");
        assert!(!report.recover_visible_output);
        assert!(report.failed_guards.iter().any(|value| value.contains("read")));
    }
}
