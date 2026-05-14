//! Renderable table-view composition for materialized Reta table sections.
//!
//! Stage 22 sits after `table_materialization`: the CSV catalog, parameter
//! matrix and HTML/tag witnesses are already known.  This module glues those
//! local sections into an ordered Rust table view, while keeping virtual
//! columns explicit and policy-controlled.  That is the safer bridge toward a
//! future renderer commit: the legacy path can stay visible while Rust can
//! render and diff a deterministic view of the same materialized sections.

use serde::{Deserialize, Serialize};

use crate::table_materialization::{
    bootstrap_table_materialization, MaterializedCsvSection, TableMaterializationConfig, TableMaterializationReport, VirtualColumnMaterialization,
};

/// How virtual/non-direct columns are represented in a renderable table view.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VirtualColumnDisplayPolicy {
    /// Keep virtual columns in the diagnostics but do not emit cells for them.
    Suppress,
    /// Emit the HTML/text witness if one exists, otherwise `?`.
    Placeholder,
    /// Emit a compact tag witness such as `744:sternPolygon,keinParaOdMetaP`.
    TagSummary,
    /// Emit a fuller diagnostic witness with reason and predecessor metadata.
    Witness,
}

impl VirtualColumnDisplayPolicy {
    pub fn canonical(self) -> &'static str {
        match self {
            Self::Suppress => "suppress",
            Self::Placeholder => "placeholder",
            Self::TagSummary => "tag-summary",
            Self::Witness => "witness",
        }
    }

    pub fn renders_virtual_cells(self) -> bool {
        !matches!(self, Self::Suppress)
    }
}

impl Default for VirtualColumnDisplayPolicy {
    fn default() -> Self {
        Self::Suppress
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaterializedTableViewConfig {
    pub virtual_column_policy: VirtualColumnDisplayPolicy,
    pub suppress_question_mark_virtuals: bool,
    pub cell_separator: String,
    pub empty_cell: String,
}

impl Default for MaterializedTableViewConfig {
    fn default() -> Self {
        Self {
            virtual_column_policy: VirtualColumnDisplayPolicy::Suppress,
            suppress_question_mark_virtuals: true,
            cell_separator: " | ".to_string(),
            empty_cell: String::new(),
        }
    }
}

impl MaterializedTableViewConfig {
    pub fn with_virtual_policy(mut self, policy: VirtualColumnDisplayPolicy) -> Self {
        self.virtual_column_policy = policy;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MaterializedTableCellSource {
    Csv {
        asset_name: String,
        source_column_index: usize,
    },
    Virtual {
        asset_name: String,
        reason: String,
        tag_names: Vec<String>,
        html_class_text: Option<String>,
        predecessor_source_column_index: Option<usize>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaterializedTableViewCell {
    pub column_legacy: usize,
    pub source_row_zero_based: usize,
    pub value: String,
    pub source: MaterializedTableCellSource,
}

impl MaterializedTableViewCell {
    pub fn is_virtual(&self) -> bool {
        matches!(self.source, MaterializedTableCellSource::Virtual { .. })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaterializedTableViewRow {
    pub asset_name: String,
    pub source_row_zero_based: usize,
    pub cells: Vec<MaterializedTableViewCell>,
}

impl MaterializedTableViewRow {
    pub fn rendered_values(&self) -> Vec<String> {
        self.cells.iter().map(|cell| cell.value.clone()).collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaterializedTableView {
    pub class: String,
    pub policy: String,
    pub selected_column_count: usize,
    pub direct_column_count: usize,
    pub virtual_column_count: usize,
    pub rendered_virtual_cell_count: usize,
    pub csv_cell_count: usize,
    pub row_count: usize,
    pub rows: Vec<MaterializedTableViewRow>,
    pub rendered_lines: Vec<String>,
    pub warnings: Vec<String>,
    pub continuum_m_direct_header_present: bool,
    pub continuum_m_virtual_744_kept_as_witness: bool,
    pub universal_property: String,
}

impl MaterializedTableView {
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn rendered_text(&self) -> String {
        self.rendered_lines.join("\n")
    }

    pub fn contains_text(&self, needle: &str) -> bool {
        self.rendered_lines.iter().any(|line| line.contains(needle))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub default_virtual_policy: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewBundle {
    pub morphisms: Vec<String>,
    pub default_virtual_policy: VirtualColumnDisplayPolicy,
    pub universal_property: String,
}

impl TableViewBundle {
    pub fn view_from_report(
        &self,
        report: &TableMaterializationReport,
        config: &MaterializedTableViewConfig,
    ) -> MaterializedTableView {
        build_materialized_table_view(report, config)
    }

    pub fn view_for_cli_args<S: AsRef<str>>(
        &self,
        args: &[S],
        materialization_config: &TableMaterializationConfig,
        view_config: &MaterializedTableViewConfig,
    ) -> MaterializedTableView {
        let report = bootstrap_table_materialization().materialize_cli_args(args, materialization_config);
        self.view_from_report(&report, view_config)
    }

    pub fn snapshot(&self) -> TableViewSnapshot {
        TableViewSnapshot {
            class: "TableViewSnapshot".to_string(),
            morphisms: self.morphisms.clone(),
            default_virtual_policy: self.default_virtual_policy.canonical().to_string(),
            universal_property: self.universal_property.clone(),
        }
    }
}

pub fn bootstrap_table_view() -> TableViewBundle {
    TableViewBundle {
        morphisms: vec![
            "build_materialized_table_view".to_string(),
            "render_table_view_lines".to_string(),
            "virtual_column_value".to_string(),
            "view_for_cli_args".to_string(),
            "kontinuum_m_table_view_smoke".to_string(),
        ],
        default_virtual_policy: VirtualColumnDisplayPolicy::Suppress,
        universal_property:
            "materialized local sections glue into one ordered table view independent of traversal/scheduling"
                .to_string(),
    }
}

pub fn build_materialized_table_view(
    report: &TableMaterializationReport,
    config: &MaterializedTableViewConfig,
) -> MaterializedTableView {
    let mut rows = Vec::new();
    let mut warnings = Vec::new();
    let mut csv_cell_count = 0usize;
    let mut rendered_virtual_cell_count = 0usize;
    let mut direct_columns = std::collections::BTreeSet::new();

    for section in &report.ordinary_sections {
        for row in &section.rows {
            let mut view_cells = Vec::new();
            for column in &section.selected_columns_legacy {
                if let Some(cell) = row
                    .cells
                    .iter()
                    .find(|cell| cell.source_column_index == *column)
                {
                    direct_columns.insert(*column);
                    csv_cell_count += 1;
                    view_cells.push(MaterializedTableViewCell {
                        column_legacy: *column,
                        source_row_zero_based: row.source_row_zero_based,
                        value: cell.value.clone(),
                        source: MaterializedTableCellSource::Csv {
                            asset_name: section.asset_name.clone(),
                            source_column_index: cell.source_column_index,
                        },
                    });
                    continue;
                }

                if let Some(virtual_column) = virtual_column_for(report, section, *column) {
                    match virtual_column_value(virtual_column, config.virtual_column_policy) {
                        Some(value)
                            if !(config.suppress_question_mark_virtuals
                                && value.trim() == "?"
                                && config.virtual_column_policy == VirtualColumnDisplayPolicy::Placeholder) =>
                        {
                            rendered_virtual_cell_count += 1;
                            view_cells.push(MaterializedTableViewCell {
                                column_legacy: *column,
                                source_row_zero_based: row.source_row_zero_based,
                                value,
                                source: MaterializedTableCellSource::Virtual {
                                    asset_name: virtual_column.asset_name.clone(),
                                    reason: virtual_column.reason.clone(),
                                    tag_names: virtual_column.tag_names.clone(),
                                    html_class_text: virtual_column.html_class_text.clone(),
                                    predecessor_source_column_index: virtual_column
                                        .predecessor_source_column_index,
                                },
                            });
                        }
                        Some(_) | None => {
                            if config.virtual_column_policy.renders_virtual_cells() {
                                warnings.push(format!(
                                    "virtual column {} suppressed by view policy {}",
                                    column,
                                    config.virtual_column_policy.canonical()
                                ));
                            }
                        }
                    }
                }
            }
            rows.push(MaterializedTableViewRow {
                asset_name: section.asset_name.clone(),
                source_row_zero_based: row.source_row_zero_based,
                cells: view_cells,
            });
        }
    }

    for symbolic in &report.symbolic_sections {
        for section in &symbolic.sections {
            append_symbolic_section_rows(
                &mut rows,
                section,
                &mut direct_columns,
                &mut csv_cell_count,
            );
        }
    }

    let rendered_lines = render_table_view_lines(&rows, &config.cell_separator, &config.empty_cell);
    let continuum_m_direct_header_present = rows.iter().any(|row| {
        row.source_row_zero_based == 0
            && row.cells.iter().any(|cell| {
                cell.column_legacy == 493 && cell.value.contains("M Kontinuum")
            })
    });
    let continuum_m_virtual_744_kept_as_witness = report
        .virtual_columns
        .iter()
        .any(|column| column.column_legacy == 744);

    MaterializedTableView {
        class: "MaterializedTableView".to_string(),
        policy: config.virtual_column_policy.canonical().to_string(),
        selected_column_count: report.selected_column_count,
        direct_column_count: direct_columns.len(),
        virtual_column_count: report.virtual_column_count,
        rendered_virtual_cell_count,
        csv_cell_count,
        row_count: rows.len(),
        rows,
        rendered_lines,
        warnings,
        continuum_m_direct_header_present,
        continuum_m_virtual_744_kept_as_witness,
        universal_property:
            "each selected local section has one deterministic row/column image before output formatting"
                .to_string(),
    }
}

fn append_symbolic_section_rows(
    rows: &mut Vec<MaterializedTableViewRow>,
    section: &MaterializedCsvSection,
    direct_columns: &mut std::collections::BTreeSet<usize>,
    csv_cell_count: &mut usize,
) {
    for row in &section.rows {
        let cells = row
            .cells
            .iter()
            .map(|cell| {
                direct_columns.insert(cell.source_column_index);
                *csv_cell_count += 1;
                MaterializedTableViewCell {
                    column_legacy: cell.source_column_index,
                    source_row_zero_based: row.source_row_zero_based,
                    value: cell.value.clone(),
                    source: MaterializedTableCellSource::Csv {
                        asset_name: section.asset_name.clone(),
                        source_column_index: cell.source_column_index,
                    },
                }
            })
            .collect::<Vec<_>>();
        rows.push(MaterializedTableViewRow {
            asset_name: section.asset_name.clone(),
            source_row_zero_based: row.source_row_zero_based,
            cells,
        });
    }
}

pub fn render_table_view_lines(
    rows: &[MaterializedTableViewRow],
    separator: &str,
    empty_cell: &str,
) -> Vec<String> {
    rows.iter()
        .map(|row| {
            if row.cells.is_empty() {
                empty_cell.to_string()
            } else {
                row.cells
                    .iter()
                    .map(|cell| cell.value.trim().to_string())
                    .collect::<Vec<_>>()
                    .join(separator)
            }
        })
        .collect()
}

pub fn virtual_column_value(
    virtual_column: &VirtualColumnMaterialization,
    policy: VirtualColumnDisplayPolicy,
) -> Option<String> {
    match policy {
        VirtualColumnDisplayPolicy::Suppress => None,
        VirtualColumnDisplayPolicy::Placeholder => Some(
            virtual_column
                .html_class_text
                .clone()
                .filter(|text| !text.trim().is_empty())
                .unwrap_or_else(|| "?".to_string()),
        ),
        VirtualColumnDisplayPolicy::TagSummary => Some(format!(
            "{}:{}",
            virtual_column.column_legacy,
            if virtual_column.tag_names.is_empty() {
                "untagged".to_string()
            } else {
                virtual_column.tag_names.join(",")
            }
        )),
        VirtualColumnDisplayPolicy::Witness => Some(format!(
            "virtual:{}:{}:{}:{}",
            virtual_column.column_legacy,
            virtual_column.reason,
            virtual_column
                .tag_names
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(","),
            virtual_column
                .predecessor_header
                .clone()
                .unwrap_or_else(|| "no-predecessor".to_string())
        )),
    }
}

fn virtual_column_for<'a>(
    report: &'a TableMaterializationReport,
    section: &MaterializedCsvSection,
    column: usize,
) -> Option<&'a VirtualColumnMaterialization> {
    report
        .virtual_columns
        .iter()
        .find(|virtual_column| {
            virtual_column.column_legacy == column && virtual_column.asset_name == section.asset_name
        })
}

pub fn view_for_cli_args<S: AsRef<str>>(
    args: &[S],
    materialization_config: &TableMaterializationConfig,
    view_config: &MaterializedTableViewConfig,
) -> MaterializedTableView {
    bootstrap_table_view().view_for_cli_args(args, materialization_config, view_config)
}

pub fn continuum_m_table_view_smoke(policy: VirtualColumnDisplayPolicy) -> MaterializedTableView {
    let args = [
        "reta",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "--breite=0",
    ];
    let config = MaterializedTableViewConfig::default().with_virtual_policy(policy);
    view_for_cli_args(&args, &TableMaterializationConfig::default(), &config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuum_m_view_suppresses_virtual_744_by_default_but_keeps_witness() {
        let view = continuum_m_table_view_smoke(VirtualColumnDisplayPolicy::Suppress);
        assert!(view.continuum_m_direct_header_present);
        assert!(view.continuum_m_virtual_744_kept_as_witness);
        assert_eq!(view.rendered_virtual_cell_count, 0);
        assert!(view.contains_text("M Kontinuum"));
    }

    #[test]
    fn continuum_m_view_can_render_virtual_744_as_tag_summary() {
        let view = continuum_m_table_view_smoke(VirtualColumnDisplayPolicy::TagSummary);
        assert!(view.rendered_virtual_cell_count > 0);
        assert!(view.contains_text("744:sternPolygon,keinParaOdMetaP"));
    }
}

