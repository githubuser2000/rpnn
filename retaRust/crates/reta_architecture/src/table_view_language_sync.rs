//! Language-asset synchronization witness for the TableView CSV path.
//!
//! Stage 55 made the base `religion.csv` own column 744 directly.  The
//! translated `*-religion.csv` assets may lag behind the base asset.  The
//! coverage and parity guards prevent unsafe commits; this module makes the
//! current synchronization state explicit and machine-checkable.

use serde::{Deserialize, Serialize};

use crate::csv_catalog::{CsvLanguage, csv_asset_by_base_and_language, csv_cell_by_name};
use crate::table_view_language_coverage::{
    LanguageAssetCoverage, TableViewLanguageCoveragePolicy, TableViewLanguageCoverageReport,
    language_coverage_for_cli_args,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LanguageColumnSyncAction {
    pub language: String,
    pub target_asset_name: String,
    pub base_asset_name: String,
    pub column_legacy: usize,
    pub target_has_column: bool,
    pub target_current_width: usize,
    pub base_current_width: usize,
    pub action: String,
    pub status: String,
    pub source_header_preview: String,
    pub source_first_value_preview: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageSyncPolicy {
    pub include_base_language: bool,
    pub include_source_previews: bool,
    pub max_preview_chars: usize,
    pub required_columns_only: bool,
    pub require_no_pending_actions_for_ready: bool,
}

impl Default for TableViewLanguageSyncPolicy {
    fn default() -> Self {
        Self {
            include_base_language: false,
            include_source_previews: true,
            max_preview_chars: 96,
            required_columns_only: true,
            require_no_pending_actions_for_ready: false,
        }
    }
}

impl TableViewLanguageSyncPolicy {
    pub fn strict() -> Self {
        Self {
            require_no_pending_actions_for_ready: true,
            ..Self::default()
        }
    }

    pub fn diagnostic() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageSyncReport {
    pub class: String,
    pub base_asset_name: String,
    pub base_max_columns: usize,
    pub requested_language: String,
    pub effective_asset_name: String,
    pub required_columns_legacy: Vec<usize>,
    pub actions: Vec<LanguageColumnSyncAction>,
    pub pending_action_count: usize,
    pub ready_action_count: usize,
    pub pending_languages: Vec<String>,
    pub pending_columns: Vec<usize>,
    pub target_assets: Vec<String>,
    pub status: String,
    pub failed_guards: Vec<String>,
    pub universal_property: String,
}

impl TableViewLanguageSyncReport {
    pub fn ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageSyncSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub smoke_status: String,
    pub smoke_pending_action_count: usize,
    pub smoke_pending_languages: Vec<String>,
    pub smoke_pending_columns: Vec<usize>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageSyncBundle {
    pub morphisms: Vec<String>,
    pub universal_property: String,
}

impl TableViewLanguageSyncBundle {
    pub fn sync_for_cli_args<S: AsRef<str>>(
        &self,
        args: &[S],
        policy: &TableViewLanguageSyncPolicy,
    ) -> TableViewLanguageSyncReport {
        language_sync_for_cli_args(args, policy)
    }

    pub fn snapshot(&self) -> TableViewLanguageSyncSnapshot {
        let smoke = continuum_m_language_sync_smoke();
        TableViewLanguageSyncSnapshot {
            class: "TableViewLanguageSyncSnapshot".to_string(),
            morphisms: self.morphisms.clone(),
            smoke_status: smoke.status,
            smoke_pending_action_count: smoke.pending_action_count,
            smoke_pending_languages: smoke.pending_languages,
            smoke_pending_columns: smoke.pending_columns,
            universal_property: self.universal_property.clone(),
        }
    }
}

pub fn bootstrap_table_view_language_sync() -> TableViewLanguageSyncBundle {
    TableViewLanguageSyncBundle {
        morphisms: vec![
            "table_view_language_sync.detect_missing_variant_columns".to_string(),
            "table_view_language_sync.project_base_column_payload".to_string(),
            "table_view_language_sync.translation_backlog_report".to_string(),
            "table_view_language_sync.no_commit_without_language_coverage".to_string(),
        ],
        universal_property: "translated_language_assets_can_replace_base_fallback_only_after_missing_columns_are_glued_from_the_base_cover".to_string(),
    }
}

pub fn language_sync_for_cli_args<S: AsRef<str>>(
    args: &[S],
    policy: &TableViewLanguageSyncPolicy,
) -> TableViewLanguageSyncReport {
    let args_owned = args
        .iter()
        .map(|arg| arg.as_ref().to_string())
        .collect::<Vec<_>>();
    let coverage = language_coverage_for_cli_args(
        &args_owned,
        &TableViewLanguageCoveragePolicy::default(),
    );
    language_sync_from_coverage(&coverage, policy)
}

pub fn language_sync_from_coverage(
    coverage: &TableViewLanguageCoverageReport,
    policy: &TableViewLanguageSyncPolicy,
) -> TableViewLanguageSyncReport {
    let base_asset_name = if coverage.base_asset_name.is_empty() {
        "religion.csv".to_string()
    } else {
        coverage.base_asset_name.clone()
    };
    let mut actions = Vec::new();
    for asset_coverage in &coverage.language_assets {
        if !policy.include_base_language && asset_coverage.language == CsvLanguage::Base.canonical() {
            continue;
        }
        actions.extend(language_sync_actions_for_asset(
            &base_asset_name,
            asset_coverage,
            &coverage.required_columns_legacy,
            coverage.base_max_columns,
            policy,
        ));
    }
    actions.sort_by(|left, right| {
        left.language
            .cmp(&right.language)
            .then(left.column_legacy.cmp(&right.column_legacy))
            .then(left.target_asset_name.cmp(&right.target_asset_name))
    });
    let pending_action_count = actions
        .iter()
        .filter(|action| action.status == "pending")
        .count();
    let ready_action_count = actions
        .iter()
        .filter(|action| action.status == "ready")
        .count();
    let pending_languages = dedup_sorted(
        actions
            .iter()
            .filter(|action| action.status == "pending")
            .map(|action| action.language.clone())
            .collect(),
    );
    let pending_columns = dedup_sorted_usize(
        actions
            .iter()
            .filter(|action| action.status == "pending")
            .map(|action| action.column_legacy)
            .collect(),
    );
    let target_assets = dedup_sorted(
        actions
            .iter()
            .filter(|action| action.status == "pending")
            .map(|action| action.target_asset_name.clone())
            .collect(),
    );
    let mut failed_guards = Vec::new();
    if policy.require_no_pending_actions_for_ready && pending_action_count > 0 {
        failed_guards.push("pending_language_sync_actions".to_string());
    }
    let status = if failed_guards.is_empty() {
        "ready"
    } else {
        "blocked"
    }
    .to_string();
    TableViewLanguageSyncReport {
        class: "TableViewLanguageSyncReport".to_string(),
        base_asset_name,
        base_max_columns: coverage.base_max_columns,
        requested_language: coverage.requested_language.clone(),
        effective_asset_name: coverage.effective_asset_name.clone(),
        required_columns_legacy: coverage.required_columns_legacy.clone(),
        actions,
        pending_action_count,
        ready_action_count,
        pending_languages,
        pending_columns,
        target_assets,
        status,
        failed_guards,
        universal_property: "language_sync_actions_describe_the_minimal_column_gluing_needed_before_translation_assets_cover_the_base_direct_columns".to_string(),
    }
}

pub fn language_sync_actions_for_asset(
    base_asset_name: &str,
    asset_coverage: &LanguageAssetCoverage,
    required_columns_legacy: &[usize],
    base_max_columns: usize,
    policy: &TableViewLanguageSyncPolicy,
) -> Vec<LanguageColumnSyncAction> {
    let columns = if policy.required_columns_only {
        required_columns_legacy.to_vec()
    } else {
        (0..base_max_columns).collect::<Vec<_>>()
    };
    let mut actions = Vec::new();
    for column in columns {
        let target_has_column = asset_coverage.exists && column < asset_coverage.max_columns;
        if target_has_column {
            actions.push(language_sync_action(
                base_asset_name,
                asset_coverage,
                column,
                true,
                "none",
                "ready",
                policy,
            ));
        } else if column < base_max_columns {
            actions.push(language_sync_action(
                base_asset_name,
                asset_coverage,
                column,
                false,
                "append_missing_direct_column",
                "pending",
                policy,
            ));
        }
    }
    actions
}

pub fn language_sync_action(
    base_asset_name: &str,
    asset_coverage: &LanguageAssetCoverage,
    column_legacy: usize,
    target_has_column: bool,
    action: &str,
    status: &str,
    policy: &TableViewLanguageSyncPolicy,
) -> LanguageColumnSyncAction {
    let (source_header_preview, source_first_value_preview) = if policy.include_source_previews {
        (
            preview_csv_cell(base_asset_name, 1, column_legacy, policy.max_preview_chars),
            preview_csv_cell(base_asset_name, 2, column_legacy, policy.max_preview_chars),
        )
    } else {
        (String::new(), String::new())
    };
    LanguageColumnSyncAction {
        language: asset_coverage.language.clone(),
        target_asset_name: asset_coverage.asset_name.clone(),
        base_asset_name: base_asset_name.to_string(),
        column_legacy,
        target_has_column,
        target_current_width: asset_coverage.max_columns,
        base_current_width: asset_coverage.base_max_columns,
        action: action.to_string(),
        status: status.to_string(),
        source_header_preview,
        source_first_value_preview,
        universal_property: "missing_language_column_is_a_local_section_that_must_glue_to_the_base_column_before_language_fallback_can_be_removed".to_string(),
    }
}

pub fn preview_csv_cell(
    asset_name: &str,
    row_one_based: usize,
    column_legacy: usize,
    max_chars: usize,
) -> String {
    csv_cell_by_name(asset_name, row_one_based, column_legacy + 1)
        .map(|text| preview_text(&text, max_chars))
        .unwrap_or_default()
}

pub fn preview_text(text: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let mut value = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        value.push('…');
    }
    value
}

pub fn continuum_m_language_sync_smoke() -> TableViewLanguageSyncReport {
    let args = [
        "reta",
        "-language=english",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "--breite=0",
    ];
    language_sync_for_cli_args(&args, &TableViewLanguageSyncPolicy::default())
}

pub fn language_sync_status_for_cli_args<S: AsRef<str>>(args: &[S]) -> String {
    language_sync_for_cli_args(args, &TableViewLanguageSyncPolicy::default()).status
}

pub fn language_sync_pending_actions_for_cli_args<S: AsRef<str>>(args: &[S]) -> usize {
    language_sync_for_cli_args(args, &TableViewLanguageSyncPolicy::default()).pending_action_count
}

pub fn language_asset_name_for_sync(base_name: &str, language: CsvLanguage) -> String {
    csv_asset_by_base_and_language(base_name, language)
        .map(|asset| asset.name.to_string())
        .unwrap_or_else(|| {
            let prefix = match language {
                CsvLanguage::Base => "",
                CsvLanguage::English => "en-",
                CsvLanguage::Chinese => "cn-",
                CsvLanguage::Vietnamese => "vn-",
                CsvLanguage::Korean => "kr-",
            };
            format!("{}{}", prefix, base_name)
        })
}

fn dedup_sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn dedup_sorted_usize(mut values: Vec<usize>) -> Vec<usize> {
    values.sort_unstable();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuum_m_reports_synced_744_translation_actions() {
        let report = continuum_m_language_sync_smoke();
        assert!(report.ready());
        assert!(report.required_columns_legacy.contains(&744));
        assert_eq!(report.pending_action_count, 0);
        assert!(report.pending_columns.is_empty());
        assert!(report.pending_languages.is_empty());
        assert!(report
            .actions
            .iter()
            .any(|action| action.column_legacy == 744
                && action.language == "en"
                && action.action == "none"
                && action.status == "ready"
                && action.source_header_preview.contains("Neues M")));
    }

    #[test]
    fn strict_policy_is_ready_after_language_actions_are_done() {
        let report = language_sync_for_cli_args(
            &[
                "reta",
                "-language=english",
                "-spalten",
                "--kontinuum=m",
            ],
            &TableViewLanguageSyncPolicy::strict(),
        );
        assert_eq!(report.status, "ready");
        assert!(report.failed_guards.is_empty());
    }

    #[test]
    fn single_localized_column_has_no_pending_sync_action() {
        let report = language_sync_for_cli_args(
            &[
                "reta",
                "-language=english",
                "-spalten",
                "--religion=493",
            ],
            &TableViewLanguageSyncPolicy::default(),
        );
        assert_eq!(report.pending_action_count, 0);
        assert!(report.ready());
    }

    #[test]
    fn asset_name_helper_keeps_existing_language_asset_names() {
        assert_eq!(
            language_asset_name_for_sync("religion.csv", CsvLanguage::English),
            "en-religion.csv"
        );
    }
}
