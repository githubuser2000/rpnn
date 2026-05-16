//! Language-variant coverage witness for CSV-backed TableView materialization.
//!
//! Stage 55 made the base `religion.csv` own direct column 744.  Stages 56-59
//! added column-aware language fallback and commit guards.  This module exposes
//! the remaining practical question explicitly: which language CSV assets are
//! already complete for a requested set of direct columns, and which still need
//! to be synchronized with the base asset before they can safely avoid fallback?

use serde::{Deserialize, Serialize};

use crate::csv_catalog::{
    csv_asset_by_base_and_language, csv_base_asset, csv_language_from_cli_args, CsvLanguage,
};
use crate::table_materialization::{
    asset_name_for_language, materialize_cli_args, TableMaterializationConfig,
    TableMaterializationReport,
};
use crate::table_view_language_parity::selected_columns_for_language_parity;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LanguageAssetCoverage {
    pub language: String,
    pub asset_name: String,
    pub exists: bool,
    pub row_count: usize,
    pub max_columns: usize,
    pub base_name: String,
    pub base_max_columns: usize,
    pub column_gap_to_base: usize,
    pub stale_relative_to_base: bool,
    pub supports_required_columns: bool,
    pub missing_required_columns: Vec<usize>,
    pub direct_493_available: bool,
    pub direct_744_available: bool,
    pub header_preview: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageCoveragePolicy {
    pub include_cell_header_preview: bool,
    pub require_base_asset_support: bool,
    pub require_requested_language_support_when_fallback_disabled: bool,
    pub require_744_translation_if_language_asset_claims_current_base_width: bool,
}

impl Default for TableViewLanguageCoveragePolicy {
    fn default() -> Self {
        Self {
            include_cell_header_preview: true,
            require_base_asset_support: true,
            require_requested_language_support_when_fallback_disabled: true,
            require_744_translation_if_language_asset_claims_current_base_width: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageCoverageReport {
    pub class: String,
    pub base_asset_name: String,
    pub base_asset_exists: bool,
    pub base_max_columns: usize,
    pub requested_language: String,
    pub requested_asset_name: String,
    pub effective_asset_name: String,
    pub fallback_enabled: bool,
    pub fallback_required: bool,
    pub fallback_applied: bool,
    pub required_columns_legacy: Vec<usize>,
    pub language_assets: Vec<LanguageAssetCoverage>,
    pub stale_language_count: usize,
    pub languages_missing_744: Vec<String>,
    pub requested_language_supports_required_columns: bool,
    pub base_supports_required_columns: bool,
    pub all_language_assets_support_required_columns: bool,
    pub status: String,
    pub failed_guards: Vec<String>,
    pub universal_property: String,
}

impl TableViewLanguageCoverageReport {
    pub fn ready(&self) -> bool {
        self.status == "ready"
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageCoverageSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub smoke_status: String,
    pub smoke_required_columns: Vec<usize>,
    pub smoke_stale_language_count: usize,
    pub smoke_languages_missing_744: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewLanguageCoverageBundle {
    pub morphisms: Vec<String>,
    pub universal_property: String,
}

impl TableViewLanguageCoverageBundle {
    pub fn coverage_for_cli_args<S: AsRef<str>>(
        &self,
        args: &[S],
        policy: &TableViewLanguageCoveragePolicy,
    ) -> TableViewLanguageCoverageReport {
        language_coverage_for_cli_args(args, policy)
    }

    pub fn snapshot(&self) -> TableViewLanguageCoverageSnapshot {
        let smoke = continuum_m_language_coverage_smoke();
        TableViewLanguageCoverageSnapshot {
            class: "TableViewLanguageCoverageSnapshot".to_string(),
            morphisms: self.morphisms.clone(),
            smoke_status: smoke.status,
            smoke_required_columns: smoke.required_columns_legacy,
            smoke_stale_language_count: smoke.stale_language_count,
            smoke_languages_missing_744: smoke.languages_missing_744,
            universal_property: self.universal_property.clone(),
        }
    }
}

pub fn bootstrap_table_view_language_coverage() -> TableViewLanguageCoverageBundle {
    TableViewLanguageCoverageBundle {
        morphisms: vec![
            "table_view_language_coverage.required_column_projection".to_string(),
            "table_view_language_coverage.variant_asset_widths".to_string(),
            "table_view_language_coverage.stale_variant_detection".to_string(),
            "table_view_language_coverage.translation_gap_report".to_string(),
            "table_view_language_coverage.fallback_readiness_witness".to_string(),
        ],
        universal_property: "language_assets_form_a_cover_only_after_each_variant_supports_requested_direct_columns".to_string(),
    }
}

pub fn language_coverage_for_cli_args<S: AsRef<str>>(
    args: &[S],
    policy: &TableViewLanguageCoveragePolicy,
) -> TableViewLanguageCoverageReport {
    let args_owned = args
        .iter()
        .map(|arg| arg.as_ref().to_string())
        .collect::<Vec<_>>();
    let config = TableMaterializationConfig::from_cli_args(&args_owned);
    let report = materialize_cli_args(&args_owned, &config);
    language_coverage_from_report(&args_owned, &report, policy)
}

pub fn language_coverage_from_report(
    args: &[String],
    report: &TableMaterializationReport,
    policy: &TableViewLanguageCoveragePolicy,
) -> TableViewLanguageCoverageReport {
    let requested_language = csv_language_from_cli_args(args);
    let requested_asset_name = asset_name_for_language("religion.csv", requested_language);
    let base_asset = csv_base_asset("religion.csv");
    let base_max_columns = base_asset.map(|asset| asset.max_columns).unwrap_or(0);
    let required_columns_legacy = selected_columns_for_language_parity(args, report);
    let language_assets = language_asset_coverages(
        "religion.csv",
        &required_columns_legacy,
        policy.include_cell_header_preview,
    );
    let requested_language_supports_required_columns = language_assets
        .iter()
        .find(|coverage| coverage.language == requested_language.canonical())
        .map(|coverage| coverage.supports_required_columns)
        .unwrap_or(false);
    let base_supports_required_columns = language_assets
        .iter()
        .find(|coverage| coverage.language == CsvLanguage::Base.canonical())
        .map(|coverage| coverage.supports_required_columns)
        .unwrap_or(false);
    let all_language_assets_support_required_columns = language_assets
        .iter()
        .filter(|coverage| coverage.exists)
        .all(|coverage| coverage.supports_required_columns);
    let stale_language_count = language_assets
        .iter()
        .filter(|coverage| coverage.exists && coverage.stale_relative_to_base)
        .count();
    let languages_missing_744 = language_assets
        .iter()
        .filter(|coverage| coverage.exists && !coverage.direct_744_available)
        .map(|coverage| coverage.language.clone())
        .collect::<Vec<_>>();
    let effective_asset_name = report
        .ordinary_sections
        .first()
        .map(|section| section.asset_name.clone())
        .unwrap_or_default();
    let fallback_enabled = TableMaterializationConfig::from_cli_args(args)
        .fallback_to_base_for_missing_language_columns;
    let fallback_required = requested_language != CsvLanguage::Base
        && !requested_language_supports_required_columns
        && !required_columns_legacy.is_empty();
    let fallback_applied = requested_language != CsvLanguage::Base
        && !effective_asset_name.is_empty()
        && effective_asset_name != requested_asset_name;

    let mut failed_guards = Vec::new();
    if policy.require_base_asset_support && !base_supports_required_columns {
        failed_guards.push("base_asset_missing_required_direct_columns".to_string());
    }
    if policy.require_requested_language_support_when_fallback_disabled
        && !fallback_enabled
        && fallback_required
    {
        failed_guards.push("fallback_disabled_for_incomplete_requested_language_asset".to_string());
    }
    if policy.require_744_translation_if_language_asset_claims_current_base_width {
        for coverage in &language_assets {
            if coverage.exists
                && coverage.language != CsvLanguage::Base.canonical()
                && coverage.max_columns >= base_max_columns
                && !coverage.direct_744_available
            {
                failed_guards.push(format!(
                    "language_asset_claims_base_width_but_lacks_744:{}",
                    coverage.language
                ));
            }
        }
    }

    let status = if failed_guards.is_empty() {
        "ready"
    } else {
        "blocked"
    }
    .to_string();

    TableViewLanguageCoverageReport {
        class: "TableViewLanguageCoverageReport".to_string(),
        base_asset_name: "religion.csv".to_string(),
        base_asset_exists: base_asset.is_some(),
        base_max_columns,
        requested_language: requested_language.canonical().to_string(),
        requested_asset_name,
        effective_asset_name,
        fallback_enabled,
        fallback_required,
        fallback_applied,
        required_columns_legacy,
        language_assets,
        stale_language_count,
        languages_missing_744,
        requested_language_supports_required_columns,
        base_supports_required_columns,
        all_language_assets_support_required_columns,
        status,
        failed_guards,
        universal_property: "language_assets_form_a_cover_only_after_each_variant_supports_requested_direct_columns".to_string(),
    }
}

pub fn language_asset_coverages(
    base_name: &str,
    required_columns_legacy: &[usize],
    include_header_preview: bool,
) -> Vec<LanguageAssetCoverage> {
    let base_max_columns = csv_base_asset(base_name)
        .map(|asset| asset.max_columns)
        .unwrap_or(0);
    [
        CsvLanguage::Base,
        CsvLanguage::English,
        CsvLanguage::Chinese,
        CsvLanguage::Vietnamese,
        CsvLanguage::Korean,
    ]
    .iter()
    .copied()
    .map(|language| {
        language_asset_coverage(
            base_name,
            language,
            required_columns_legacy,
            base_max_columns,
            include_header_preview,
        )
    })
    .collect()
}

pub fn language_asset_coverage(
    base_name: &str,
    language: CsvLanguage,
    required_columns_legacy: &[usize],
    base_max_columns: usize,
    include_header_preview: bool,
) -> LanguageAssetCoverage {
    let asset = csv_asset_by_base_and_language(base_name, language);
    let max_columns = asset.map(|asset| asset.max_columns).unwrap_or(0);
    let missing_required_columns = required_columns_legacy
        .iter()
        .copied()
        .filter(|column| asset.map(|asset| *column >= asset.max_columns).unwrap_or(true))
        .collect::<Vec<_>>();
    let supports_required_columns = asset.is_some() && missing_required_columns.is_empty();
    let asset_name = asset
        .map(|asset| asset.name.to_string())
        .unwrap_or_else(|| asset_name_for_language(base_name, language));
    LanguageAssetCoverage {
        language: language.canonical().to_string(),
        asset_name,
        exists: asset.is_some(),
        row_count: asset.map(|asset| asset.row_count).unwrap_or(0),
        max_columns,
        base_name: base_name.to_string(),
        base_max_columns,
        column_gap_to_base: base_max_columns.saturating_sub(max_columns),
        stale_relative_to_base: asset.is_some() && max_columns < base_max_columns,
        supports_required_columns,
        missing_required_columns,
        direct_493_available: asset.map(|asset| 493 < asset.max_columns).unwrap_or(false),
        direct_744_available: asset.map(|asset| 744 < asset.max_columns).unwrap_or(false),
        header_preview: if include_header_preview {
            asset
                .map(|asset| asset.header_preview.to_string())
                .unwrap_or_default()
        } else {
            String::new()
        },
    }
}

pub fn continuum_m_language_coverage_smoke() -> TableViewLanguageCoverageReport {
    let args = [
        "reta",
        "-language=english",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "--breite=0",
    ];
    language_coverage_for_cli_args(&args, &TableViewLanguageCoveragePolicy::default())
}

pub fn language_coverage_status_for_cli_args<S: AsRef<str>>(args: &[S]) -> String {
    language_coverage_for_cli_args(args, &TableViewLanguageCoveragePolicy::default()).status
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuum_m_reports_stale_language_assets_and_base_744() {
        let report = continuum_m_language_coverage_smoke();
        assert!(report.ready());
        assert_eq!(report.base_max_columns, 745);
        assert!(report.required_columns_legacy.contains(&493));
        assert!(report.required_columns_legacy.contains(&744));
        assert!(report.base_supports_required_columns);
        assert!(report.stale_language_count >= 1);
        assert!(report.languages_missing_744.contains(&"en".to_string()));
        assert_eq!(report.effective_asset_name, "religion.csv");
        assert!(report.fallback_required);
        assert!(report.fallback_applied);
    }

    #[test]
    fn english_religion_493_remains_language_covered() {
        let args = [
            "reta",
            "-language=english",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--religion=493",
        ];
        let report = language_coverage_for_cli_args(&args, &Default::default());
        assert!(report.ready());
        assert_eq!(report.requested_asset_name, "en-religion.csv");
        assert_eq!(report.effective_asset_name, "en-religion.csv");
        assert!(report.requested_language_supports_required_columns);
        assert!(!report.fallback_required);
    }

    #[test]
    fn disabling_fallback_blocks_incomplete_language_asset() {
        let args = [
            "reta",
            "-language=english",
            "--no-language-fallback",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
        ];
        let report = language_coverage_for_cli_args(&args, &Default::default());
        assert_eq!(report.status, "blocked");
        assert!(report
            .failed_guards
            .contains(&"fallback_disabled_for_incomplete_requested_language_asset".to_string()));
    }
}
