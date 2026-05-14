//! Legacy numbering / counting projection for materialized table views.
//!
//! Stage 29 separates the historical two-column numbering prefix from the
//! renderer.  The old Python renderer prepends a `Zählung` column and a
//! `Nummerierung` column when numbering is enabled, and removes both through
//! `--keinenummerierung`.  Rust now has the same projection as a typed,
//! policy-controlled morphism.  It stays disabled in the visible shadow path by
//! default so legacy output remains the oracle until a commit gate allows it.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::number_theory::moon_number;
use crate::table_view::MaterializedTableViewRow;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TableViewNumberingMode {
    Disabled,
    LegacyPair,
    NumberOnly,
    CountingOnly,
}

impl TableViewNumberingMode {
    pub const fn canonical(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::LegacyPair => "legacy-pair",
            Self::NumberOnly => "number-only",
            Self::CountingOnly => "counting-only",
        }
    }

    pub const fn column_count(self) -> usize {
        match self {
            Self::Disabled => 0,
            Self::LegacyPair => 2,
            Self::NumberOnly | Self::CountingOnly => 1,
        }
    }

    pub const fn includes_counting(self) -> bool {
        matches!(self, Self::LegacyPair | Self::CountingOnly)
    }

    pub const fn includes_numbering(self) -> bool {
        matches!(self, Self::LegacyPair | Self::NumberOnly)
    }
}

impl Default for TableViewNumberingMode {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewNumberingConfig {
    pub mode: TableViewNumberingMode,
    pub zaehlung_header: String,
    pub nummerierung_header: String,
    pub empty_header_value: String,
    pub suppress_header_labels: bool,
    pub use_display_index_for_missing_rows: bool,
}

impl Default for TableViewNumberingConfig {
    fn default() -> Self {
        Self::disabled()
    }
}

impl TableViewNumberingConfig {
    pub fn disabled() -> Self {
        Self {
            mode: TableViewNumberingMode::Disabled,
            zaehlung_header: "Zählung".to_string(),
            nummerierung_header: "Nummerierung".to_string(),
            empty_header_value: String::new(),
            suppress_header_labels: false,
            use_display_index_for_missing_rows: true,
        }
    }

    pub fn legacy_pair() -> Self {
        Self {
            mode: TableViewNumberingMode::LegacyPair,
            ..Self::disabled()
        }
    }

    pub fn number_only() -> Self {
        Self {
            mode: TableViewNumberingMode::NumberOnly,
            ..Self::disabled()
        }
    }

    pub fn counting_only() -> Self {
        Self {
            mode: TableViewNumberingMode::CountingOnly,
            ..Self::disabled()
        }
    }

    pub fn disabled_by_keinenummerierung(mut self) -> Self {
        self.mode = TableViewNumberingMode::Disabled;
        self
    }

    pub fn is_enabled(&self) -> bool {
        self.mode != TableViewNumberingMode::Disabled
    }

    pub fn column_count(&self) -> usize {
        self.mode.column_count()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewNumberingCell {
    pub column_kind: String,
    pub source_row_zero_based: usize,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewNumberingProjection {
    pub class: String,
    pub mode: String,
    pub source_row_zero_based: usize,
    pub display_index: usize,
    pub values: Vec<String>,
    pub cells: Vec<TableViewNumberingCell>,
    pub zaehlung: Option<i64>,
    pub nummerierung: Option<i64>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewNumberingReport {
    pub class: String,
    pub mode: String,
    pub row_count: usize,
    pub max_source_row_zero_based: usize,
    pub numbering_column_count: usize,
    pub header_values: Vec<String>,
    pub first_data_values: Vec<String>,
    pub zaehlung_sample: Vec<(i64, i64)>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewNumberingSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub default_mode: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewNumberingBundle;

impl TableViewNumberingBundle {
    pub fn snapshot(&self) -> TableViewNumberingSnapshot {
        TableViewNumberingSnapshot {
            class: "TableViewNumberingSnapshot".to_string(),
            morphisms: vec![
                "legacy_zaehlung_map".to_string(),
                "legacy_zaehlung_for_row".to_string(),
                "numbering_projection_for_source_row".to_string(),
                "numbering_values_for_source_row".to_string(),
                "numbering_report_for_rows".to_string(),
            ],
            default_mode: TableViewNumberingMode::Disabled.canonical().to_string(),
            universal_property:
                "row numbering is a prefix morphism; disabling numbering leaves materialized cells unchanged"
                    .to_string(),
        }
    }

    pub fn projection_for_row(
        &self,
        source_row_zero_based: usize,
        display_index: usize,
        config: &TableViewNumberingConfig,
    ) -> TableViewNumberingProjection {
        numbering_projection_for_source_row(source_row_zero_based, display_index, config)
    }

    pub fn report_for_rows(
        &self,
        rows: &[MaterializedTableViewRow],
        config: &TableViewNumberingConfig,
    ) -> TableViewNumberingReport {
        numbering_report_for_rows(rows, config)
    }
}

pub fn bootstrap_table_view_numbering() -> TableViewNumberingBundle {
    TableViewNumberingBundle
}

/// Build the legacy counting groups up to `max_row`.
///
/// Python starts with `isMoon = True` and opens a new counting group whenever a
/// moon-number segment switches to a non-moon-number segment.  This reproduces
/// the stable group ids used by `Prepare.zeileWhichZaehlung` without mutating a
/// renderer-owned `prepare.zaehlungen` object.
pub fn legacy_zaehlung_map(max_row: i64) -> BTreeMap<i64, i64> {
    let mut out = BTreeMap::new();
    if max_row <= 0 {
        return out;
    }
    let mut is_moon = true;
    let mut zaehlung = 0i64;
    for row in 1..=max_row {
        let was_moon = is_moon;
        is_moon = !moon_number(row).0.is_empty();
        if was_moon && !is_moon {
            zaehlung += 1;
        }
        out.insert(row, zaehlung);
    }
    out
}

pub fn legacy_zaehlung_for_row(row: i64) -> i64 {
    if row <= 0 {
        return 0;
    }
    legacy_zaehlung_map(row).get(&row).copied().unwrap_or(0)
}

pub fn numbering_projection_for_source_row(
    source_row_zero_based: usize,
    display_index: usize,
    config: &TableViewNumberingConfig,
) -> TableViewNumberingProjection {
    let mut cells = Vec::new();
    let mut values = Vec::new();
    let source_row = source_row_zero_based as i64;
    let is_header = source_row_zero_based == 0;
    let header_value = |label: &str, config: &TableViewNumberingConfig| {
        if config.suppress_header_labels {
            config.empty_header_value.clone()
        } else {
            label.to_string()
        }
    };
    let row_number = if source_row_zero_based == 0 && config.use_display_index_for_missing_rows {
        display_index as i64
    } else {
        source_row
    };
    let zaehlung = (!is_header).then(|| legacy_zaehlung_for_row(source_row));
    let nummerierung = (!is_header).then_some(row_number);

    if config.mode.includes_counting() {
        let value = if is_header {
            header_value(&config.zaehlung_header, config)
        } else {
            zaehlung.unwrap_or_default().to_string()
        };
        values.push(value.clone());
        cells.push(TableViewNumberingCell {
            column_kind: "zaehlung".to_string(),
            source_row_zero_based,
            value,
        });
    }

    if config.mode.includes_numbering() {
        let value = if is_header {
            header_value(&config.nummerierung_header, config)
        } else {
            nummerierung.unwrap_or_default().to_string()
        };
        values.push(value.clone());
        cells.push(TableViewNumberingCell {
            column_kind: "nummerierung".to_string(),
            source_row_zero_based,
            value,
        });
    }

    TableViewNumberingProjection {
        class: "TableViewNumberingProjection".to_string(),
        mode: config.mode.canonical().to_string(),
        source_row_zero_based,
        display_index,
        values,
        cells,
        zaehlung,
        nummerierung,
        universal_property:
            "numbering prefix depends only on source row and display policy, not on cell rendering mode"
                .to_string(),
    }
}

pub fn numbering_values_for_source_row(
    source_row_zero_based: usize,
    display_index: usize,
    config: &TableViewNumberingConfig,
) -> Vec<String> {
    numbering_projection_for_source_row(source_row_zero_based, display_index, config).values
}

pub fn numbering_report_for_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewNumberingConfig,
) -> TableViewNumberingReport {
    let max_source_row_zero_based = rows
        .iter()
        .map(|row| row.source_row_zero_based)
        .max()
        .unwrap_or_default();
    let header_values = rows
        .iter()
        .find(|row| row.source_row_zero_based == 0)
        .map(|row| numbering_values_for_source_row(row.source_row_zero_based, 0, config))
        .unwrap_or_else(|| numbering_values_for_source_row(0, 0, config));
    let first_data_values = rows
        .iter()
        .enumerate()
        .find(|(_index, row)| row.source_row_zero_based > 0)
        .map(|(index, row)| numbering_values_for_source_row(row.source_row_zero_based, index, config))
        .unwrap_or_default();
    let max_sample = max_source_row_zero_based.max(12) as i64;
    let zaehlung_sample = legacy_zaehlung_map(max_sample)
        .into_iter()
        .take(12)
        .collect::<Vec<_>>();
    TableViewNumberingReport {
        class: "TableViewNumberingReport".to_string(),
        mode: config.mode.canonical().to_string(),
        row_count: rows.len(),
        max_source_row_zero_based,
        numbering_column_count: config.column_count(),
        header_values,
        first_data_values,
        zaehlung_sample,
        universal_property:
            "legacy numbering prefixes commute with table view output mode projection"
                .to_string(),
    }
}

pub fn numbering_smoke_report() -> TableViewNumberingReport {
    let rows = crate::table_view::continuum_m_table_view_smoke(
        crate::table_view::VirtualColumnDisplayPolicy::Suppress,
    )
    .rows;
    numbering_report_for_rows(&rows, &TableViewNumberingConfig::legacy_pair())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_zaehlung_matches_initial_python_intervals() {
        let map = legacy_zaehlung_map(12);
        assert_eq!(map.get(&1), Some(&1));
        assert_eq!(map.get(&4), Some(&1));
        assert_eq!(map.get(&5), Some(&2));
        assert_eq!(map.get(&9), Some(&2));
        assert_eq!(map.get(&10), Some(&3));
    }

    #[test]
    fn legacy_pair_adds_two_header_and_data_values() {
        let config = TableViewNumberingConfig::legacy_pair();
        let header = numbering_values_for_source_row(0, 0, &config);
        let row = numbering_values_for_source_row(5, 1, &config);
        assert_eq!(header, vec!["Zählung".to_string(), "Nummerierung".to_string()]);
        assert_eq!(row, vec!["2".to_string(), "5".to_string()]);
    }

    #[test]
    fn disabled_numbering_is_empty_prefix() {
        assert!(numbering_values_for_source_row(
            5,
            1,
            &TableViewNumberingConfig::disabled()
        )
        .is_empty());
    }
}
