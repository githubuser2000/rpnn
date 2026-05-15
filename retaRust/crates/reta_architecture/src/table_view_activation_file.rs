//! File-backed witness for guarded table-view activation stores.
//!
//! Stage 45 made activation stores line-oriented.  Stage 46 connected that
//! store to the in-memory persistence/audit/cache morphisms.  This module adds
//! the next boundary: a dependency-free file backend that writes the store text,
//! reads it back, parses it again and blocks when any file, checksum or parse
//! guard fails.  It intentionally does not relax visible-output commit rules.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::persistence::stable_digest_text;
use crate::runtime_switch::ArchitectureSwitchConfig;
use crate::table_view_activation_store::{
    TableViewActivationStore, TableViewActivationStoreParseReport, TableViewActivationStorePolicy,
    activation_store_for_cli_args, parse_activation_store_text,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationFilePolicy {
    pub store_policy: TableViewActivationStorePolicy,
    pub file_path: Option<String>,
    pub default_directory: String,
    pub write_store_file: bool,
    pub create_parent_directories: bool,
    pub atomic_write: bool,
    pub backup_existing_file: bool,
    pub read_after_write: bool,
    pub parse_after_read: bool,
    pub include_file_preview: bool,
    pub preview_limit: usize,
}

impl Default for TableViewActivationFilePolicy {
    fn default() -> Self {
        Self {
            store_policy: TableViewActivationStorePolicy::default(),
            file_path: None,
            default_directory: "target/reta_arch/activation-store".to_string(),
            write_store_file: true,
            create_parent_directories: true,
            atomic_write: true,
            backup_existing_file: true,
            read_after_write: true,
            parse_after_read: true,
            include_file_preview: true,
            preview_limit: 8,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationFileReport {
    pub class: String,
    pub morphism: String,
    pub status: String,
    pub path: String,
    pub backup_path: Option<String>,
    pub source_transaction_id: Option<String>,
    pub source_store_validation_status: String,
    pub source_line_count: usize,
    pub source_record_count: usize,
    pub source_selected_line_count: usize,
    pub source_text_checksum: u64,
    pub source_text_digest: String,
    pub source_byte_len: usize,
    pub wrote_file: bool,
    pub write_error: Option<String>,
    pub read_file: bool,
    pub read_error: Option<String>,
    pub read_text_digest: Option<String>,
    pub read_byte_len: Option<usize>,
    pub read_matches_source: bool,
    pub parse_ready: bool,
    pub parse_validation_status: Option<String>,
    pub parse_failed_guards: Vec<String>,
    pub file_preview: Vec<String>,
    pub failed_guards: Vec<String>,
    pub universal_property: String,
}

impl TableViewActivationFileReport {
    pub fn is_ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationFileSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub file_guards: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewActivationFileBundle;

impl TableViewActivationFileBundle {
    pub fn snapshot(&self) -> TableViewActivationFileSnapshot {
        TableViewActivationFileSnapshot {
            class: "TableViewActivationFileSnapshot".to_string(),
            morphisms: vec![
                "table_view_activation_file.choose_file_path".to_string(),
                "table_view_activation_file.atomic_write_store".to_string(),
                "table_view_activation_file.read_store_file".to_string(),
                "table_view_activation_file.parse_read_store".to_string(),
                "table_view_activation_file.rollback_on_file_drift".to_string(),
            ],
            file_guards: vec![
                "source_store_validation_ready".to_string(),
                "file_write_succeeded".to_string(),
                "read_digest_matches_source_digest".to_string(),
                "parsed_read_store_validation_ready".to_string(),
            ],
            universal_property:
                "file_backed_activation_store_is_ready_only_when_write_read_parse_and_digest_are_identity".to_string(),
        }
    }

    pub fn write_store(
        &self,
        activation_store: &TableViewActivationStore,
        fallback_legacy_lines: &[String],
        current_transaction_id: Option<&str>,
        policy: &TableViewActivationFilePolicy,
    ) -> TableViewActivationFileReport {
        write_activation_store_file(
            activation_store,
            fallback_legacy_lines,
            current_transaction_id,
            policy,
        )
    }

    pub fn file_for_cli_args(
        &self,
        args: &[String],
        legacy_lines: &[String],
        switch_config: &ArchitectureSwitchConfig,
        policy: &TableViewActivationFilePolicy,
    ) -> TableViewActivationFileReport {
        activation_file_for_cli_args(args, legacy_lines, switch_config, policy)
    }
}

pub fn bootstrap_table_view_activation_file() -> TableViewActivationFileBundle {
    TableViewActivationFileBundle
}


pub fn activation_file_policy_from_cli_args(
    argv: &[String],
    base: &TableViewActivationFilePolicy,
) -> (TableViewActivationFilePolicy, bool) {
    let mut policy = base.clone();
    let mut enabled = false;
    if let Ok(path) = std::env::var("RETA_ARCH_ACTIVATION_FILE") {
        if !path.trim().is_empty() {
            policy.file_path = Some(path);
            enabled = true;
        }
    }
    if let Ok(dir) = std::env::var("RETA_ARCH_ACTIVATION_DIR") {
        if !dir.trim().is_empty() {
            policy.default_directory = dir;
            enabled = true;
        }
    }
    let mut index = 0usize;
    while index < argv.len() {
        let arg = &argv[index];
        match arg.as_str() {
            "--activation-store-file" | "--reta-arch-activation-file" => {
                if let Some(value) = argv.get(index + 1) {
                    policy.file_path = Some(value.clone());
                    enabled = true;
                    index += 1;
                }
            }
            "--activation-store-dir" | "--reta-arch-activation-dir" => {
                if let Some(value) = argv.get(index + 1) {
                    policy.default_directory = value.clone();
                    enabled = true;
                    index += 1;
                }
            }
            "--activation-store-no-atomic" => {
                policy.atomic_write = false;
                enabled = true;
            }
            "--activation-store-no-backup" => {
                policy.backup_existing_file = false;
                enabled = true;
            }
            "--activation-store-no-readback" => {
                policy.read_after_write = false;
                policy.parse_after_read = false;
                enabled = true;
            }
            "--no-activation-store-file" | "--reta-arch-no-activation-file" => {
                policy.write_store_file = false;
                enabled = false;
            }
            _ if arg.starts_with("--activation-store-file=")
                || arg.starts_with("--reta-arch-activation-file=") =>
            {
                if let Some((_, value)) = arg.split_once('=') {
                    policy.file_path = Some(value.to_string());
                    enabled = true;
                }
            }
            _ if arg.starts_with("--activation-store-dir=")
                || arg.starts_with("--reta-arch-activation-dir=") =>
            {
                if let Some((_, value)) = arg.split_once('=') {
                    policy.default_directory = value.to_string();
                    enabled = true;
                }
            }
            _ => {}
        }
        index += 1;
    }
    (policy, enabled)
}

pub fn activation_file_for_cli_args(
    args: &[String],
    legacy_lines: &[String],
    switch_config: &ArchitectureSwitchConfig,
    policy: &TableViewActivationFilePolicy,
) -> TableViewActivationFileReport {
    let activation_store = activation_store_for_cli_args(
        args,
        legacy_lines,
        switch_config,
        &policy.store_policy,
    );
    let current_transaction_id = activation_store.latest_transaction_id.as_deref();
    write_activation_store_file(
        &activation_store,
        legacy_lines,
        current_transaction_id,
        policy,
    )
}

pub fn default_activation_store_file_path(
    activation_store: &TableViewActivationStore,
    policy: &TableViewActivationFilePolicy,
) -> String {
    let basename = activation_store
        .latest_transaction_id
        .as_deref()
        .map(sanitize_path_component)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| format!("activation-store-{:016x}", activation_store.text_checksum));
    Path::new(&policy.default_directory)
        .join(format!("{basename}.reta-activation-store.txt"))
        .to_string_lossy()
        .into_owned()
}

pub fn write_activation_store_file(
    activation_store: &TableViewActivationStore,
    fallback_legacy_lines: &[String],
    current_transaction_id: Option<&str>,
    policy: &TableViewActivationFilePolicy,
) -> TableViewActivationFileReport {
    let path = policy
        .file_path
        .clone()
        .unwrap_or_else(|| default_activation_store_file_path(activation_store, policy));
    let path_buf = PathBuf::from(&path);
    let source_text_digest = stable_digest_text(&activation_store.store_text);
    let source_byte_len = activation_store.store_text.as_bytes().len();
    let mut failed_guards = Vec::new();
    let mut write_error = None;
    let mut read_error = None;
    let mut wrote_file = false;
    let mut backup_path = None;

    if !activation_store.validation.is_ready() {
        failed_guards.push("source_store_validation_not_ready".to_string());
    }

    if policy.write_store_file {
        if let Some(parent) = path_buf.parent() {
            if !parent.as_os_str().is_empty() && policy.create_parent_directories {
                if let Err(error) = fs::create_dir_all(parent) {
                    write_error = Some(format!("create_parent_directories_failed: {error}"));
                }
            }
        }
        if write_error.is_none() && policy.backup_existing_file && path_buf.exists() {
            let backup = backup_path_for(&path_buf);
            match fs::copy(&path_buf, &backup) {
                Ok(_) => backup_path = Some(backup.to_string_lossy().into_owned()),
                Err(error) => write_error = Some(format!("backup_existing_file_failed: {error}")),
            }
        }
        if write_error.is_none() {
            let write_result = if policy.atomic_write {
                atomic_write_text(&path_buf, &activation_store.store_text)
            } else {
                fs::write(&path_buf, activation_store.store_text.as_bytes())
                    .map_err(|error| error.to_string())
            };
            match write_result {
                Ok(()) => wrote_file = true,
                Err(error) => write_error = Some(error),
            }
        }
    }

    if policy.write_store_file && !wrote_file {
        failed_guards.push("file_write_failed".to_string());
    }
    if let Some(error) = write_error.as_ref() {
        failed_guards.push(format!("write_error:{error}"));
    }

    let loaded_text = if policy.read_after_write {
        match fs::read_to_string(&path_buf) {
            Ok(text) => Some(text),
            Err(error) => {
                read_error = Some(error.to_string());
                None
            }
        }
    } else {
        None
    };

    let read_file = loaded_text.is_some();
    let read_text_digest = loaded_text.as_ref().map(|text| stable_digest_text(text));
    let read_byte_len = loaded_text.as_ref().map(|text| text.as_bytes().len());
    let read_matches_source = read_text_digest
        .as_ref()
        .map(|digest| digest == &source_text_digest)
        .unwrap_or(!policy.read_after_write);
    if policy.read_after_write && !read_file {
        failed_guards.push("file_read_failed".to_string());
    }
    if let Some(error) = read_error.as_ref() {
        failed_guards.push(format!("read_error:{error}"));
    }
    if policy.read_after_write && !read_matches_source {
        failed_guards.push("read_digest_mismatch".to_string());
    }

    let parse_report = if policy.parse_after_read {
        loaded_text.as_ref().map(|text| {
            parse_activation_store_text(
                text,
                fallback_legacy_lines,
                current_transaction_id.or(activation_store.latest_transaction_id.as_deref()),
                &policy.store_policy,
            )
        })
    } else {
        None
    };
    let parse_ready = parse_report
        .as_ref()
        .map(|report| report.validation.is_ready())
        .unwrap_or(!policy.parse_after_read);
    let parse_validation_status = parse_report
        .as_ref()
        .map(|report| report.validation.status.clone());
    let parse_failed_guards = parse_report_failed_guards(parse_report.as_ref());
    if policy.parse_after_read && !parse_ready {
        failed_guards.push("parsed_read_store_not_ready".to_string());
    }

    let file_preview = if policy.include_file_preview {
        loaded_text
            .as_deref()
            .unwrap_or(activation_store.store_text.as_str())
            .lines()
            .take(policy.preview_limit)
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let status = if failed_guards.is_empty() {
        "ready"
    } else {
        "blocked"
    };

    TableViewActivationFileReport {
        class: "TableViewActivationFileReport".to_string(),
        morphism: "table_view_activation_file.write_read_parse_store_file".to_string(),
        status: status.to_string(),
        path,
        backup_path,
        source_transaction_id: activation_store.latest_transaction_id.clone(),
        source_store_validation_status: activation_store.validation.status.clone(),
        source_line_count: activation_store.line_count,
        source_record_count: activation_store.record_count,
        source_selected_line_count: activation_store.selected_line_count,
        source_text_checksum: activation_store.text_checksum,
        source_text_digest,
        source_byte_len,
        wrote_file,
        write_error,
        read_file,
        read_error,
        read_text_digest,
        read_byte_len,
        read_matches_source,
        parse_ready,
        parse_validation_status,
        parse_failed_guards,
        file_preview,
        failed_guards,
        universal_property:
            "activation_file_is_ready_exactly_when_store_write_read_digest_and_parse_all_commute".to_string(),
    }
}

pub fn read_activation_store_file(
    path: impl AsRef<Path>,
    fallback_legacy_lines: &[String],
    current_transaction_id: Option<&str>,
    policy: &TableViewActivationStorePolicy,
) -> TableViewActivationStoreParseReport {
    match fs::read_to_string(path) {
        Ok(text) => parse_activation_store_text(
            &text,
            fallback_legacy_lines,
            current_transaction_id,
            policy,
        ),
        Err(error) => TableViewActivationStoreParseReport {
            class: "TableViewActivationStoreParseReport".to_string(),
            morphism: "table_view_activation_file.read_activation_store_file".to_string(),
            parsed: false,
            validation: crate::table_view_activation_store::TableViewActivationStoreValidation {
                class: "TableViewActivationStoreValidation".to_string(),
                status: "blocked".to_string(),
                header_valid: false,
                sequence_valid: false,
                selected_line_checksums_valid: false,
                ledger_hashes_present: false,
                ledger_hashes_match: false,
                ledger_validation_status: "not_loaded".to_string(),
                record_count: 0,
                selected_line_count: 0,
                ledger_entry_count: 0,
                failed_guards: vec!["file_read_failed".to_string()],
                parse_errors: vec![error.to_string()],
                universal_property:
                    "file_backed_activation_store_must_be_readable_before_parse".to_string(),
            },
            store: None,
            universal_property:
                "file_backed_activation_store_must_be_readable_before_parse".to_string(),
        },
    }
}

pub fn continuum_m_activation_file_smoke() -> TableViewActivationFileReport {
    let activation_store = crate::table_view_activation_store::continuum_m_activation_store_smoke();
    let mut policy = TableViewActivationFilePolicy::default();
    policy.file_path = Some(
        std::env::temp_dir()
            .join(format!(
                "reta-stage47-activation-{:016x}.txt",
                activation_store.text_checksum
            ))
            .to_string_lossy()
            .into_owned(),
    );
    let legacy_lines = activation_store
        .journal
        .records
        .last()
        .map(|record| record.selected_lines.clone())
        .unwrap_or_default();
    let report = write_activation_store_file(
        &activation_store,
        &legacy_lines,
        activation_store.latest_transaction_id.as_deref(),
        &policy,
    );
    let _ = fs::remove_file(&report.path);
    report
}

fn parse_report_failed_guards(
    report: Option<&TableViewActivationStoreParseReport>,
) -> Vec<String> {
    report
        .map(|value| value.validation.failed_guards.clone())
        .unwrap_or_default()
}

fn backup_path_for(path: &Path) -> PathBuf {
    let mut backup = path.as_os_str().to_os_string();
    backup.push(".bak");
    PathBuf::from(backup)
}

fn atomic_write_text(path: &Path, text: &str) -> Result<(), String> {
    let mut tmp = path.as_os_str().to_os_string();
    tmp.push(format!(".tmp-{}", std::process::id()));
    let tmp_path = PathBuf::from(tmp);
    fs::write(&tmp_path, text.as_bytes()).map_err(|error| error.to_string())?;
    fs::rename(&tmp_path, path).map_err(|error| {
        let _ = fs::remove_file(&tmp_path);
        error.to_string()
    })
}

fn sanitize_path_component(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activation_file_roundtrips_ready_store() {
        let activation_store = crate::table_view_activation_store::continuum_m_activation_store_smoke();
        let mut policy = TableViewActivationFilePolicy::default();
        policy.file_path = Some(
            std::env::temp_dir()
                .join(format!(
                    "reta-test-activation-file-{:016x}.txt",
                    activation_store.text_checksum
                ))
                .to_string_lossy()
                .into_owned(),
        );
        let legacy_lines = activation_store
            .journal
            .records
            .last()
            .map(|record| record.selected_lines.clone())
            .unwrap_or_default();
        let report = write_activation_store_file(
            &activation_store,
            &legacy_lines,
            activation_store.latest_transaction_id.as_deref(),
            &policy,
        );
        assert!(report.is_ready(), "{report:?}");
        assert!(report.wrote_file);
        assert!(report.read_matches_source);
        assert!(report.parse_ready);
        let _ = fs::remove_file(report.path);
    }

    #[test]
    fn activation_file_detects_read_digest_mismatch() {
        let activation_store = crate::table_view_activation_store::continuum_m_activation_store_smoke();
        let mut policy = TableViewActivationFilePolicy::default();
        let path = std::env::temp_dir()
            .join(format!(
                "reta-test-activation-file-mismatch-{:016x}.txt",
                activation_store.text_checksum
            ))
            .to_string_lossy()
            .into_owned();
        policy.file_path = Some(path.clone());
        policy.write_store_file = false;
        fs::write(&path, "tampered\n").expect("write tampered activation file");
        let report = write_activation_store_file(
            &activation_store,
            &[],
            activation_store.latest_transaction_id.as_deref(),
            &policy,
        );
        assert!(!report.is_ready());
        assert!(!report.read_matches_source);
        assert!(report.failed_guards.contains(&"read_digest_mismatch".to_string()));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn sanitize_path_component_removes_path_separators() {
        assert_eq!(sanitize_path_component("a/b:c d"), "a_b_c_d");
    }
}
