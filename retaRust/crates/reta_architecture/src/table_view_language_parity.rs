//! Language-aware materialization parity for the Rust TableView path.
//!
//! Stage 55 made `religion.csv` own direct column 744.  Stage 56 added a
//! column-aware fallback from language variants back to the base CSV when a
//! variant still lacks that direct column.  Stage 57 moved the selected
//! language into the CLI-driven materialization path.  This module folds those
//! pieces into a compact witness: a localized table section may be used only
//! when it can satisfy the requested direct columns; otherwise the base section
//! must be the effective materialization source.

use serde::{Deserialize, Serialize};

use crate::csv_catalog::{csv_asset_by_name, csv_language_from_cli_args, CsvLanguage};
use crate::parameter_runtime::bootstrap_parameter_runtime;
use crate::table_materialization::{
    asset_name_for_language, materialize_cli_args, TableMaterializationConfig,
    TableMaterializationReport,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LanguageCellSignature {
    pub row_zero_based: usize,
    pub column_legacy: usize,
    pub value_preview: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageParityPolicy {
    pub require_base_fallback_for_incomplete_language_asset: bool,
    pub require_direct_744_when_selected: bool,
    pub include_cell_preview: bool,
    pub preview_cell_limit: usize,
}

impl Default for TableViewLanguageParityPolicy {
    fn default() -> Self {
        Self {
            require_base_fallback_for_incomplete_language_asset: true,
            require_direct_744_when_selected: true,
            include_cell_preview: true,
            preview_cell_limit: 8,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageParityReport {
    pub class: String,
    pub requested_language: String,
    pub effective_language: String,
    pub requested_asset_name: String,
    pub effective_asset_name: String,
    pub requested_asset_exists: bool,
    pub requested_asset_max_columns: usize,
    pub effective_asset_max_columns: usize,
    pub selected_columns_legacy: Vec<usize>,
    pub requested_asset_supports_selected_columns: bool,
    pub fallback_required: bool,
    pub fallback_applied: bool,
    pub direct_493_materialized: bool,
    pub direct_744_materialized: bool,
    pub missing_columns_legacy: Vec<usize>,
    pub language_variant_allowed_for_requested_columns: bool,
    pub cell_preview: Vec<LanguageCellSignature>,
    pub status: String,
    pub failed_guards: Vec<String>,
    pub universal_property: String,
}

impl TableViewLanguageParityReport {
    pub fn ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageParitySnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub smoke_status: String,
    pub smoke_requested_language: String,
    pub smoke_effective_asset_name: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageParityBundle {
    pub morphisms: Vec<String>,
    pub universal_property: String,
}

impl TableViewLanguageParityBundle {
    pub fn language_parity_for_cli_args<S: AsRef<str>>(
        &self,
        args: &[S],
        policy: &TableViewLanguageParityPolicy,
    ) -> TableViewLanguageParityReport {
        language_parity_for_cli_args(args, policy)
    }

    pub fn snapshot(&self) -> TableViewLanguageParitySnapshot {
        let smoke = continuum_m_language_parity_smoke();
        TableViewLanguageParitySnapshot {
            class: "TableViewLanguageParitySnapshot".to_string(),
            morphisms: self.morphisms.clone(),
            smoke_status: smoke.status,
            smoke_requested_language: smoke.requested_language,
            smoke_effective_asset_name: smoke.effective_asset_name,
            universal_property: self.universal_property.clone(),
        }
    }
}

pub fn bootstrap_table_view_language_parity() -> TableViewLanguageParityBundle {
    TableViewLanguageParityBundle {
        morphisms: vec![
            "table_view_language_parity.selected_language".to_string(),
            "table_view_language_parity.requested_asset_support".to_string(),
            "table_view_language_parity.base_fallback_guard".to_string(),
            "table_view_language_parity.direct_744_guard".to_string(),
            "table_view_language_parity.localized_493_guard".to_string(),
        ],
        universal_property: "localized_csv_sections_may_glue_only_when_they_cover_requested_direct_columns".to_string(),
    }
}

pub fn language_parity_for_cli_args<S: AsRef<str>>(
    args: &[S],
    policy: &TableViewLanguageParityPolicy,
) -> TableViewLanguageParityReport {
    let args_owned = args
        .iter()
        .map(|arg| arg.as_ref().to_string())
        .collect::<Vec<_>>();
    let requested_language = csv_language_from_cli_args(&args_owned);
    let requested_asset_name = asset_name_for_language("religion.csv", requested_language);
    let requested_asset = csv_asset_by_name(&requested_asset_name);
    let config = TableMaterializationConfig::from_cli_args(&args_owned);
    let effective_report = materialize_cli_args(&args_owned, &config);
    language_parity_from_report(
        &args_owned,
        requested_language,
        requested_asset_name,
        requested_asset.map(|asset| asset.max_columns).unwrap_or(0),
        &effective_report,
        policy,
    )
}

pub fn language_parity_from_report(
    args: &[String],
    requested_language: CsvLanguage,
    requested_asset_name: String,
    requested_asset_max_columns: usize,
    report: &TableMaterializationReport,
    policy: &TableViewLanguageParityPolicy,
) -> TableViewLanguageParityReport {
    let requested_asset_exists = csv_asset_by_name(&requested_asset_name).is_some();
    let selected_columns_legacy = selected_columns_for_language_parity(args, report);
    let requested_asset_supports_selected_columns = requested_asset_exists
        && selected_columns_legacy
            .iter()
            .all(|column| *column < requested_asset_max_columns);
    let effective_section = report.ordinary_sections.first();
    let effective_asset_name = effective_section
        .map(|section| section.asset_name.clone())
        .unwrap_or_default();
    let effective_language = effective_section
        .map(|section| section.language.clone())
        .unwrap_or_else(|| "none".to_string());
    let effective_asset_max_columns = effective_section
        .map(|section| section.source_max_columns)
        .unwrap_or(0);
    let missing_columns_legacy = effective_section
        .map(|section| section.missing_columns_legacy.clone())
        .unwrap_or_default();
    let fallback_applied = requested_language != CsvLanguage::Base
        && requested_asset_exists
        && !effective_asset_name.is_empty()
        && effective_asset_name != requested_asset_name;
    let fallback_required = requested_language != CsvLanguage::Base
        && requested_asset_exists
        && !requested_asset_supports_selected_columns
        && !selected_columns_legacy.is_empty();
    let direct_493_materialized = direct_column_materialized(effective_section, 493);
    let direct_744_materialized = direct_column_materialized(effective_section, 744);
    let selected_744 = selected_columns_legacy.contains(&744);
    let language_variant_allowed_for_requested_columns = requested_language == CsvLanguage::Base
        || requested_asset_supports_selected_columns;

    let mut failed_guards = Vec::new();
    if policy.require_base_fallback_for_incomplete_language_asset
        && fallback_required
        && !fallback_applied
    {
        failed_guards.push("language_asset_missing_requested_direct_columns_without_base_fallback".to_string());
    }
    if policy.require_direct_744_when_selected && selected_744 && !direct_744_materialized {
        failed_guards.push("selected_744_not_materialized_as_direct_csv_cell".to_string());
    }
    if requested_language == CsvLanguage::Base && fallback_applied {
        failed_guards.push("base_language_must_not_fallback_to_language_variant".to_string());
    }

    let cell_preview = if policy.include_cell_preview {
        effective_section
            .map(|section| {
                section
                    .rows
                    .iter()
                    .flat_map(|row| {
                        row.cells.iter().map(move |cell| LanguageCellSignature {
                            row_zero_based: row.source_row_zero_based,
                            column_legacy: cell.source_column_index,
                            value_preview: preview_text(&cell.value, 96),
                        })
                    })
                    .take(policy.preview_cell_limit)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    let status = if failed_guards.is_empty() {
        "ready"
    } else {
        "blocked"
    }
    .to_string();

    TableViewLanguageParityReport {
        class: "TableViewLanguageParityReport".to_string(),
        requested_language: requested_language.canonical().to_string(),
        effective_language,
        requested_asset_name,
        effective_asset_name,
        requested_asset_exists,
        requested_asset_max_columns,
        effective_asset_max_columns,
        selected_columns_legacy,
        requested_asset_supports_selected_columns,
        fallback_required,
        fallback_applied,
        direct_493_materialized,
        direct_744_materialized,
        missing_columns_legacy,
        language_variant_allowed_for_requested_columns,
        cell_preview,
        status,
        failed_guards,
        universal_property: "language_variant_projection_preserves_direct_column_coverage_or_glues_to_base_asset".to_string(),
    }
}

pub fn selected_columns_for_language_parity(
    args: &[String],
    report: &TableMaterializationReport,
) -> Vec<usize> {
    if !report.requested_column_order_legacy.is_empty() {
        return dedup_usize(report.requested_column_order_legacy.clone());
    }
    if let Some(section) = report.ordinary_sections.first() {
        return dedup_usize(section.selected_columns_legacy.clone());
    }
    let parsed = bootstrap_parameter_runtime().parse_cli_args(args);
    let mut columns = parsed
        .command_sets
        .selected_columns
        .iter()
        .filter_map(|column| usize::try_from(*column).ok())
        .collect::<Vec<_>>();
    columns.sort_unstable();
    dedup_usize(columns)
}

pub fn language_parity_status_for_cli_args<S: AsRef<str>>(args: &[S]) -> String {
    language_parity_for_cli_args(args, &TableViewLanguageParityPolicy::default()).status
}

pub fn continuum_m_language_parity_smoke() -> TableViewLanguageParityReport {
    let args = [
        "reta",
        "-language=english",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "--breite=0",
    ];
    language_parity_for_cli_args(&args, &TableViewLanguageParityPolicy::default())
}

fn direct_column_materialized(
    section: Option<&crate::table_materialization::MaterializedCsvSection>,
    column: usize,
) -> bool {
    section
        .map(|section| {
            section.selected_columns_legacy.contains(&column)
                && !section.missing_columns_legacy.contains(&column)
                && column < section.source_max_columns
        })
        .unwrap_or(false)
}

fn dedup_usize(values: Vec<usize>) -> Vec<usize> {
    let mut out = Vec::new();
    for value in values {
        if !out.contains(&value) {
            out.push(value);
        }
    }
    out
}

fn preview_text(value: &str, limit: usize) -> String {
    let mut out = String::new();
    for ch in value.chars().take(limit) {
        out.push(ch);
    }
    if value.chars().count() > limit {
        out.push('…');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_493_can_use_language_variant() {
        let args = [
            "reta",
            "-language=english",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--religion=493",
        ];
        let report = language_parity_for_cli_args(&args, &Default::default());
        assert!(report.ready());
        assert_eq!(report.requested_asset_name, "en-religion.csv");
        assert_eq!(report.effective_asset_name, "en-religion.csv");
        assert!(!report.fallback_required);
        assert!(!report.fallback_applied);
        assert!(report.direct_493_materialized);
    }

    #[test]
    fn english_kontinuum_m_falls_back_to_base_for_direct_744() {
        let report = continuum_m_language_parity_smoke();
        assert!(report.ready());
        assert_eq!(report.requested_language, "en");
        assert_eq!(report.requested_asset_name, "en-religion.csv");
        assert_eq!(report.effective_asset_name, "religion.csv");
        assert!(report.fallback_required);
        assert!(report.fallback_applied);
        assert!(report.direct_493_materialized);
        assert!(report.direct_744_materialized);
        assert!(report.selected_columns_legacy.contains(&493));
        assert!(report.selected_columns_legacy.contains(&744));
    }

    #[test]
    fn disabling_language_fallback_blocks_direct_744() {
        let args = [
            "reta",
            "-language=english",
            "--no-language-fallback",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
        ];
        let report = language_parity_for_cli_args(&args, &Default::default());
        assert_eq!(report.effective_asset_name, "en-religion.csv");
        assert!(report.fallback_required);
        assert!(!report.fallback_applied);
        assert!(!report.direct_744_materialized);
        assert_eq!(report.status, "blocked");
    }
}
