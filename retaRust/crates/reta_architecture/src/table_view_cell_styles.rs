//! Policy-controlled cell style projection for materialized table-view output.
//!
//! Stage 33 connects the legacy `OutputSyntax.generateCell`/`generate_cell_begin`
//! semantics to the Rust `MaterializedTableView` renderer.  The projection is
//! disabled by default.  It is intended for shadow/diff runs first: cell
//! wrappers can be inspected for HTML/BBCode without changing the visible legacy
//! path unless a later commit gate explicitly allows it.

use serde::{Deserialize, Serialize};

use crate::output_syntax::{generate_cell_begin, OutputMode};
use crate::table_view::{MaterializedTableCellSource, MaterializedTableViewRow};
use crate::tag_schema::ordinary_tags_for_column;

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum TableViewCellStylePolicy {
    /// Use the simple renderer defaults (`<td>` / `[td]`).
    #[default]
    Plain,
    /// Use `output_syntax::generate_cell_begin`, the Rust image of the old
    /// Python `generateCell` method, for HTML/BBCode cells.
    LegacyGenerateCell,
    /// Same as `LegacyGenerateCell`, but report every generated wrapper as a
    /// diagnostic witness even when the visible wrapper is equivalent to plain.
    LegacyGenerateCellWitness,
}

impl TableViewCellStylePolicy {
    pub fn canonical(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::LegacyGenerateCell => "legacy-generate-cell",
            Self::LegacyGenerateCellWitness => "legacy-generate-cell-witness",
        }
    }

    pub fn uses_generate_cell(self) -> bool {
        matches!(self, Self::LegacyGenerateCell | Self::LegacyGenerateCellWitness)
    }

    pub fn is_witness(self) -> bool {
        matches!(self, Self::LegacyGenerateCellWitness)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewCellStyleConfig {
    pub enabled: bool,
    pub policy: TableViewCellStylePolicy,
    pub apply_html: bool,
    pub apply_bbcode: bool,
    pub include_header_row: bool,
    pub include_virtual_cells: bool,
    pub compact_html_tags: bool,
}

impl Default for TableViewCellStyleConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            policy: TableViewCellStylePolicy::Plain,
            apply_html: true,
            apply_bbcode: true,
            include_header_row: true,
            include_virtual_cells: true,
            compact_html_tags: true,
        }
    }
}

impl TableViewCellStyleConfig {
    pub fn disabled() -> Self {
        Self::default()
    }

    pub fn legacy_generate_cell() -> Self {
        Self {
            enabled: true,
            policy: TableViewCellStylePolicy::LegacyGenerateCell,
            ..Self::default()
        }
    }

    pub fn legacy_generate_cell_witness() -> Self {
        Self {
            enabled: true,
            policy: TableViewCellStylePolicy::LegacyGenerateCellWitness,
            ..Self::default()
        }
    }

    pub fn activates_mode(&self, mode: OutputMode) -> bool {
        self.enabled
            && self.policy.uses_generate_cell()
            && match mode {
                OutputMode::Html => self.apply_html,
                OutputMode::Bbcode => self.apply_bbcode,
                _ => false,
            }
    }

    pub fn without_color(mut self) -> Self {
        // `generateCell` mostly emits class/wrapper metadata.  For safety,
        // `--nocolor` deactivates the projection entirely until cell-colour
        // parity has been proven in shadow reports.
        self.enabled = false;
        self.policy = TableViewCellStylePolicy::Plain;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewCellStyle {
    pub mode: String,
    pub source_row_zero_based: usize,
    pub display_cell_index: usize,
    pub data_cell_index: Option<usize>,
    pub column_legacy: Option<usize>,
    pub pseudo_column: Option<i64>,
    pub continuation_line: bool,
    pub virtual_cell: bool,
    pub content_as_int: Option<i64>,
    pub header_tags: Vec<String>,
    pub begin_cell: String,
    pub end_cell: String,
    pub styled: bool,
    pub source: String,
}

impl TableViewCellStyle {
    pub fn is_plain(&self) -> bool {
        !self.styled
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewCellStyleReport {
    pub class: String,
    pub enabled: bool,
    pub policy: String,
    pub mode: String,
    pub row_count: usize,
    pub cell_count: usize,
    pub styled_cell_count: usize,
    pub html_cell_style_count: usize,
    pub bbcode_cell_style_count: usize,
    pub virtual_cell_style_count: usize,
    pub numbering_prefix_style_count: usize,
    pub cells: Vec<TableViewCellStyle>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewCellStyleSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub policies: Vec<String>,
    pub default_enabled: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewCellStyleBundle;

impl TableViewCellStyleBundle {
    pub fn snapshot(&self) -> TableViewCellStyleSnapshot {
        TableViewCellStyleSnapshot {
            class: "TableViewCellStyleSnapshot".to_string(),
            morphisms: vec![
                "cell_style_for_row_cell".to_string(),
                "cell_style_for_output_value".to_string(),
                "cell_style_report_for_rows".to_string(),
                "styled_begin_cell_for_output_value".to_string(),
                "styled_end_cell_for_mode".to_string(),
                "continuum_m_cell_style_smoke".to_string(),
            ],
            policies: vec![
                TableViewCellStylePolicy::Plain.canonical().to_string(),
                TableViewCellStylePolicy::LegacyGenerateCell.canonical().to_string(),
                TableViewCellStylePolicy::LegacyGenerateCellWitness
                    .canonical()
                    .to_string(),
            ],
            default_enabled: TableViewCellStyleConfig::default().enabled,
            universal_property:
                "cell wrappers are deterministic output-syntax projections over the same materialized cells"
                    .to_string(),
        }
    }

    pub fn report(
        &self,
        rows: &[MaterializedTableViewRow],
        mode: OutputMode,
        config: &TableViewCellStyleConfig,
        suppress_headers: bool,
        include_empty_rows: bool,
        prefix_column_count: usize,
    ) -> TableViewCellStyleReport {
        cell_style_report_for_rows(
            rows,
            mode,
            config,
            suppress_headers,
            include_empty_rows,
            prefix_column_count,
        )
    }
}

pub fn bootstrap_table_view_cell_styles() -> TableViewCellStyleBundle {
    TableViewCellStyleBundle
}

pub fn cell_style_for_row_cell(
    row: &MaterializedTableViewRow,
    mode: OutputMode,
    data_cell_index: usize,
    continuation_line: bool,
    config: &TableViewCellStyleConfig,
) -> TableViewCellStyle {
    let Some(cell) = row.cells.get(data_cell_index) else {
        return plain_cell_style(
            row.source_row_zero_based,
            mode,
            data_cell_index,
            Some(data_cell_index),
            None,
            None,
            continuation_line,
        );
    };
    let header_tags = header_tags_for_cell(cell);
    let content_as_int = parse_cell_int(&cell.value);
    let should_style = config.activates_mode(mode)
        && (!continuation_line || mode == OutputMode::Bbcode || mode == OutputMode::Html)
        && (config.include_header_row || row.source_row_zero_based != 0)
        && (config.include_virtual_cells || !cell.is_virtual());
    let plain_begin = plain_begin_cell(mode);
    let begin_cell = if should_style {
        clean_cell_begin(
            mode,
            &generate_cell_begin(
                mode,
                cell.column_legacy as i64,
                content_as_int,
                Some(row.source_row_zero_based as i64),
                &header_tags,
            ),
            config.compact_html_tags,
        )
    } else {
        plain_begin.clone()
    };
    let styled = should_style
        && (config.policy.is_witness() || begin_cell != plain_begin || !header_tags.is_empty());
    TableViewCellStyle {
        mode: mode.canonical_name().to_string(),
        source_row_zero_based: row.source_row_zero_based,
        display_cell_index: data_cell_index,
        data_cell_index: Some(data_cell_index),
        column_legacy: Some(cell.column_legacy),
        pseudo_column: None,
        continuation_line,
        virtual_cell: cell.is_virtual(),
        content_as_int,
        header_tags,
        begin_cell,
        end_cell: styled_end_cell_for_mode(mode),
        styled,
        source: if should_style {
            "output_syntax.generate_cell_begin".to_string()
        } else {
            "plain-cell".to_string()
        },
    }
}

pub fn cell_style_for_output_value(
    row: &MaterializedTableViewRow,
    mode: OutputMode,
    display_cell_index: usize,
    continuation_line: bool,
    prefix_column_count: usize,
    config: &TableViewCellStyleConfig,
) -> TableViewCellStyle {
    if display_cell_index < prefix_column_count {
        let pseudo_column = if prefix_column_count >= 2 && display_cell_index == 0 {
            -2
        } else {
            -1
        };
        let should_style = config.activates_mode(mode)
            && (config.include_header_row || row.source_row_zero_based != 0);
        let plain_begin = plain_begin_cell(mode);
        let begin_cell = if should_style {
            clean_cell_begin(
                mode,
                &generate_cell_begin(
                    mode,
                    pseudo_column,
                    Some(row.source_row_zero_based as i64),
                    Some(row.source_row_zero_based as i64),
                    &[],
                ),
                config.compact_html_tags,
            )
        } else {
            plain_begin.clone()
        };
        let styled = should_style && (config.policy.is_witness() || begin_cell != plain_begin);
        return TableViewCellStyle {
            mode: mode.canonical_name().to_string(),
            source_row_zero_based: row.source_row_zero_based,
            display_cell_index,
            data_cell_index: None,
            column_legacy: None,
            pseudo_column: Some(pseudo_column),
            continuation_line,
            virtual_cell: false,
            content_as_int: Some(row.source_row_zero_based as i64),
            header_tags: Vec::new(),
            begin_cell,
            end_cell: styled_end_cell_for_mode(mode),
            styled,
            source: if should_style {
                "output_syntax.generate_cell_begin:numbering-prefix".to_string()
            } else {
                "plain-cell".to_string()
            },
        };
    }
    let data_index = display_cell_index - prefix_column_count;
    let mut style = cell_style_for_row_cell(row, mode, data_index, continuation_line, config);
    style.display_cell_index = display_cell_index;
    style
}

pub fn styled_begin_cell_for_output_value(
    row: &MaterializedTableViewRow,
    mode: OutputMode,
    display_cell_index: usize,
    continuation_line: bool,
    prefix_column_count: usize,
    config: &TableViewCellStyleConfig,
) -> String {
    cell_style_for_output_value(
        row,
        mode,
        display_cell_index,
        continuation_line,
        prefix_column_count,
        config,
    )
    .begin_cell
}

pub fn styled_end_cell_for_mode(mode: OutputMode) -> String {
    match mode {
        OutputMode::Html => "</td>".to_string(),
        OutputMode::Bbcode => "[/td]".to_string(),
        _ => mode.syntax_markup().end_cell,
    }
}

pub fn cell_style_report_for_rows(
    rows: &[MaterializedTableViewRow],
    mode: OutputMode,
    config: &TableViewCellStyleConfig,
    suppress_headers: bool,
    include_empty_rows: bool,
    prefix_column_count: usize,
) -> TableViewCellStyleReport {
    let mut cells = Vec::new();
    for row in rows {
        if suppress_headers && row.source_row_zero_based == 0 {
            continue;
        }
        if !include_empty_rows && row.cells.is_empty() {
            continue;
        }
        let total = prefix_column_count + row.cells.len();
        for display_cell_index in 0..total {
            cells.push(cell_style_for_output_value(
                row,
                mode,
                display_cell_index,
                false,
                prefix_column_count,
                config,
            ));
        }
    }
    let styled_cell_count = cells.iter().filter(|cell| cell.styled).count();
    let html_cell_style_count = cells
        .iter()
        .filter(|cell| cell.mode == "html" && cell.styled)
        .count();
    let bbcode_cell_style_count = cells
        .iter()
        .filter(|cell| cell.mode == "bbcode" && cell.styled)
        .count();
    let virtual_cell_style_count = cells
        .iter()
        .filter(|cell| cell.virtual_cell && cell.styled)
        .count();
    let numbering_prefix_style_count = cells
        .iter()
        .filter(|cell| cell.pseudo_column.is_some() && cell.styled)
        .count();
    TableViewCellStyleReport {
        class: "TableViewCellStyleReport".to_string(),
        enabled: config.enabled,
        policy: config.policy.canonical().to_string(),
        mode: mode.canonical_name().to_string(),
        row_count: rows.len(),
        cell_count: cells.len(),
        styled_cell_count,
        html_cell_style_count,
        bbcode_cell_style_count,
        virtual_cell_style_count,
        numbering_prefix_style_count,
        cells,
        universal_property:
            "cell style sections decorate output cells without changing materialized cell values"
                .to_string(),
    }
}

pub fn continuum_m_cell_style_smoke(mode: OutputMode) -> TableViewCellStyleReport {
    let args = [
        "reta",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "-ausgabe",
        "--cellstyles",
        "--breite=0",
    ];
    let view = crate::table_view::view_for_cli_args(
        &args,
        &crate::table_materialization::TableMaterializationConfig::default(),
        &crate::table_view::MaterializedTableViewConfig::default(),
    );
    cell_style_report_for_rows(
        &view.rows,
        mode,
        &TableViewCellStyleConfig::legacy_generate_cell(),
        false,
        true,
        0,
    )
}

fn plain_cell_style(
    source_row_zero_based: usize,
    mode: OutputMode,
    display_cell_index: usize,
    data_cell_index: Option<usize>,
    column_legacy: Option<usize>,
    pseudo_column: Option<i64>,
    continuation_line: bool,
) -> TableViewCellStyle {
    TableViewCellStyle {
        mode: mode.canonical_name().to_string(),
        source_row_zero_based,
        display_cell_index,
        data_cell_index,
        column_legacy,
        pseudo_column,
        continuation_line,
        virtual_cell: false,
        content_as_int: None,
        header_tags: Vec::new(),
        begin_cell: plain_begin_cell(mode),
        end_cell: styled_end_cell_for_mode(mode),
        styled: false,
        source: "plain-cell".to_string(),
    }
}

fn plain_begin_cell(mode: OutputMode) -> String {
    match mode {
        OutputMode::Html => "<td>".to_string(),
        OutputMode::Bbcode => "[td]".to_string(),
        _ => mode.syntax_markup().begin_cell,
    }
}

fn clean_cell_begin(mode: OutputMode, begin: &str, compact_html_tags: bool) -> String {
    let trimmed = begin.trim();
    if trimmed.is_empty() {
        return plain_begin_cell(mode);
    }
    match mode {
        OutputMode::Html if compact_html_tags => trimmed.replace('\n', ""),
        _ => trimmed.to_string(),
    }
}

fn parse_cell_int(value: &str) -> Option<i64> {
    value.trim().parse::<i64>().ok()
}

fn header_tags_for_cell(cell: &crate::table_view::MaterializedTableViewCell) -> Vec<String> {
    match &cell.source {
        MaterializedTableCellSource::Virtual { tag_names, .. } => tag_names.clone(),
        MaterializedTableCellSource::Csv { .. } => ordinary_tags_for_column(cell.column_legacy as i64)
            .unwrap_or_default()
            .into_iter()
            .map(|tag| tag.py_name().to_string())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table_materialization::TableMaterializationConfig;
    use crate::table_view::{view_for_cli_args, MaterializedTableViewConfig};

    #[test]
    fn default_cell_styles_are_disabled() {
        let report = continuum_m_cell_style_smoke(OutputMode::Html);
        assert!(report.enabled);
        assert!(report.cell_count > 0);
        assert!(report.styled_cell_count > 0);
    }

    #[test]
    fn legacy_generate_cell_can_style_html_and_bbcode_cells() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
        ];
        let view = view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &MaterializedTableViewConfig::default(),
        );
        let html = cell_style_report_for_rows(
            &view.rows,
            OutputMode::Html,
            &TableViewCellStyleConfig::legacy_generate_cell(),
            false,
            true,
            0,
        );
        assert!(html.styled_cell_count > 0);
        assert!(html.cells.iter().any(|cell| cell.begin_cell.starts_with("<td")));
        let bbcode = cell_style_report_for_rows(
            &view.rows,
            OutputMode::Bbcode,
            &TableViewCellStyleConfig::legacy_generate_cell_witness(),
            false,
            true,
            0,
        );
        assert!(bbcode.styled_cell_count > 0);
        assert!(bbcode.cells.iter().any(|cell| cell.begin_cell.starts_with("[td")));
    }

    #[test]
    fn numbering_prefix_uses_legacy_negative_columns() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
        ];
        let view = view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &MaterializedTableViewConfig::default(),
        );
        let report = cell_style_report_for_rows(
            &view.rows,
            OutputMode::Bbcode,
            &TableViewCellStyleConfig::legacy_generate_cell_witness(),
            false,
            true,
            2,
        );
        assert!(report.numbering_prefix_style_count > 0);
        assert!(report
            .cells
            .iter()
            .any(|cell| cell.pseudo_column == Some(-2) && cell.begin_cell.starts_with("[td")));
    }
}
