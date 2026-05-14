//! Policy-controlled row style projection for materialized table-view output.
//!
//! Stage 32 connects the legacy `coloredBeginCol` semantics from
//! `output_syntax.py` to the Rust `MaterializedTableView` output path.  The
//! projection is disabled by default: row colouring remains a shadow/diffable
//! local section until explicit CLI policy and commit gates allow it.

use serde::{Deserialize, Serialize};

use crate::output_syntax::{colored_begin_col, OutputMode};
use crate::table_view::MaterializedTableViewRow;

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum TableViewRowStylePolicy {
    /// Render plain rows (`<tr>` / `[tr]`) and keep legacy colour data as a witness only.
    #[default]
    Plain,
    /// Use `output_syntax::colored_begin_col`, the Rust image of Python's
    /// `coloredBeginCol`, for HTML/BBCode row starts.
    LegacyColoredBeginCol,
}

impl TableViewRowStylePolicy {
    pub fn canonical(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::LegacyColoredBeginCol => "legacy-colored-begin-col",
        }
    }

    pub fn is_colored(self) -> bool {
        matches!(self, Self::LegacyColoredBeginCol)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewRowStyleConfig {
    pub enabled: bool,
    pub policy: TableViewRowStylePolicy,
    /// Apply legacy colours to the header/source row as well.  Python's
    /// `coloredBeginCol(0)` has a distinct red witness, but this remains
    /// policy-controlled to avoid accidental visible changes.
    pub include_header_row: bool,
    /// Continuation rows caused by cell wrapping use the historical `rest=True`
    /// branch, which intentionally returns an uncoloured row start.
    pub rest_rows_plain: bool,
    pub apply_html: bool,
    pub apply_bbcode: bool,
}

impl Default for TableViewRowStyleConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            policy: TableViewRowStylePolicy::Plain,
            include_header_row: true,
            rest_rows_plain: true,
            apply_html: true,
            apply_bbcode: true,
        }
    }
}

impl TableViewRowStyleConfig {
    pub fn disabled() -> Self {
        Self::default()
    }

    pub fn legacy_colored() -> Self {
        Self {
            enabled: true,
            policy: TableViewRowStylePolicy::LegacyColoredBeginCol,
            ..Self::default()
        }
    }

    pub fn activates_mode(&self, mode: OutputMode) -> bool {
        self.enabled
            && self.policy.is_colored()
            && match mode {
                OutputMode::Html => self.apply_html,
                OutputMode::Bbcode => self.apply_bbcode,
                _ => false,
            }
    }

    pub fn without_color(mut self) -> Self {
        self.enabled = false;
        self.policy = TableViewRowStylePolicy::Plain;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewRowStyle {
    pub mode: String,
    pub source_row_zero_based: usize,
    pub row_number_for_style: i64,
    pub rest: bool,
    pub styled: bool,
    pub begin_row: String,
    pub end_row: String,
    pub color_signature: String,
    pub source: String,
}

impl TableViewRowStyle {
    pub fn is_colored(&self) -> bool {
        self.begin_row.contains("background-color") || self.begin_row.contains("style=")
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewRowStyleReport {
    pub class: String,
    pub enabled: bool,
    pub policy: String,
    pub mode: String,
    pub row_count: usize,
    pub styled_row_count: usize,
    pub colored_row_count: usize,
    pub header_row_colored: bool,
    pub html_row_style_count: usize,
    pub bbcode_row_style_count: usize,
    pub rows: Vec<TableViewRowStyle>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewRowStyleSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub policies: Vec<String>,
    pub default_enabled: bool,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewRowStyleBundle;

impl TableViewRowStyleBundle {
    pub fn snapshot(&self) -> TableViewRowStyleSnapshot {
        TableViewRowStyleSnapshot {
            class: "TableViewRowStyleSnapshot".to_string(),
            morphisms: vec![
                "row_style_for_row".to_string(),
                "row_style_for_source_row".to_string(),
                "row_style_report_for_rows".to_string(),
                "styled_begin_row_for_row".to_string(),
                "continuum_m_row_style_smoke".to_string(),
            ],
            policies: vec![
                TableViewRowStylePolicy::Plain.canonical().to_string(),
                TableViewRowStylePolicy::LegacyColoredBeginCol.canonical().to_string(),
            ],
            default_enabled: TableViewRowStyleConfig::default().enabled,
            universal_property:
                "row colour is a deterministic output-syntax projection that never changes table cells"
                    .to_string(),
        }
    }

    pub fn report(
        &self,
        rows: &[MaterializedTableViewRow],
        mode: OutputMode,
        config: &TableViewRowStyleConfig,
        suppress_headers: bool,
        include_empty_rows: bool,
    ) -> TableViewRowStyleReport {
        row_style_report_for_rows(rows, mode, config, suppress_headers, include_empty_rows)
    }
}

pub fn bootstrap_table_view_row_styles() -> TableViewRowStyleBundle {
    TableViewRowStyleBundle
}

pub fn row_style_for_row(
    row: &MaterializedTableViewRow,
    mode: OutputMode,
    config: &TableViewRowStyleConfig,
) -> TableViewRowStyle {
    row_style_for_source_row(row.source_row_zero_based, mode, false, config)
}

pub fn row_style_for_source_row(
    source_row_zero_based: usize,
    mode: OutputMode,
    rest: bool,
    config: &TableViewRowStyleConfig,
) -> TableViewRowStyle {
    let default_begin = default_begin_row(mode);
    let default_end = default_end_row(mode);
    let should_style = config.activates_mode(mode)
        && (!rest || !config.rest_rows_plain)
        && (config.include_header_row || source_row_zero_based != 0);
    let row_number = source_row_zero_based as i64;
    let begin_row = if should_style {
        colored_begin_col(mode, row_number, rest)
    } else {
        default_begin.clone()
    };
    let color_signature = color_signature(&begin_row);
    TableViewRowStyle {
        mode: mode.canonical_name().to_string(),
        source_row_zero_based,
        row_number_for_style: row_number,
        rest,
        styled: should_style && begin_row != default_begin,
        begin_row,
        end_row: default_end,
        color_signature,
        source: if should_style {
            "output_syntax.colored_begin_col".to_string()
        } else {
            "plain-row".to_string()
        },
    }
}

pub fn styled_begin_row_for_row(
    row: &MaterializedTableViewRow,
    mode: OutputMode,
    rest: bool,
    config: &TableViewRowStyleConfig,
) -> String {
    row_style_for_source_row(row.source_row_zero_based, mode, rest, config).begin_row
}

pub fn row_style_report_for_rows(
    rows: &[MaterializedTableViewRow],
    mode: OutputMode,
    config: &TableViewRowStyleConfig,
    suppress_headers: bool,
    include_empty_rows: bool,
) -> TableViewRowStyleReport {
    let row_styles = rows
        .iter()
        .filter(|row| !(suppress_headers && row.source_row_zero_based == 0))
        .filter(|row| include_empty_rows || !row.cells.is_empty())
        .map(|row| row_style_for_row(row, mode, config))
        .collect::<Vec<_>>();
    let styled_row_count = row_styles.iter().filter(|row| row.styled).count();
    let colored_row_count = row_styles.iter().filter(|row| row.is_colored()).count();
    TableViewRowStyleReport {
        class: "TableViewRowStyleReport".to_string(),
        enabled: config.enabled,
        policy: config.policy.canonical().to_string(),
        mode: mode.canonical_name().to_string(),
        row_count: row_styles.len(),
        styled_row_count,
        colored_row_count,
        header_row_colored: row_styles
            .iter()
            .any(|row| row.source_row_zero_based == 0 && row.is_colored()),
        html_row_style_count: row_styles
            .iter()
            .filter(|row| row.mode == "html" && row.is_colored())
            .count(),
        bbcode_row_style_count: row_styles
            .iter()
            .filter(|row| row.mode == "bbcode" && row.is_colored())
            .count(),
        rows: row_styles,
        universal_property:
            "row-style projection is natural in output mode: it changes row wrappers, not cells"
                .to_string(),
    }
}

pub fn continuum_m_row_style_smoke(mode: OutputMode) -> TableViewRowStyleReport {
    let args = [
        "reta",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "-ausgabe",
        "--spaltenreihenfolgeundnurdiese=744,493",
    ];
    let view = crate::table_view::view_for_cli_args(
        &args,
        &crate::table_materialization::TableMaterializationConfig::default(),
        &crate::table_view::MaterializedTableViewConfig::default()
            .with_virtual_policy(crate::table_view::VirtualColumnDisplayPolicy::TagSummary),
    );
    row_style_report_for_rows(
        &view.rows,
        mode,
        &TableViewRowStyleConfig::legacy_colored(),
        false,
        true,
    )
}

fn default_begin_row(mode: OutputMode) -> String {
    match mode {
        OutputMode::Html => "<tr>".to_string(),
        OutputMode::Bbcode => "[tr]".to_string(),
        _ => mode.syntax_markup().begin_row,
    }
}

fn default_end_row(mode: OutputMode) -> String {
    mode.syntax_markup()
        .end_row
        .trim_end_matches('\n')
        .to_string()
}

fn color_signature(begin_row: &str) -> String {
    if let Some(start) = begin_row.find("background-color:") {
        let from = start + "background-color:".len();
        let tail = &begin_row[from..];
        let color = tail
            .split([';', '"', '\'', ']'])
            .next()
            .unwrap_or_default()
            .trim();
        return color.to_string();
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table_materialization::TableMaterializationConfig;
    use crate::table_view::{
        view_for_cli_args, MaterializedTableViewConfig, VirtualColumnDisplayPolicy,
    };

    #[test]
    fn row_styles_are_disabled_by_default() {
        let report = continuum_m_row_style_smoke(OutputMode::Shell);
        assert_eq!(report.mode, "shell");
        assert_eq!(report.colored_row_count, 0);
        assert_eq!(TableViewRowStyleConfig::default().enabled, false);
    }

    #[test]
    fn legacy_html_row_style_uses_colored_begin_col() {
        let report = continuum_m_row_style_smoke(OutputMode::Html);
        assert!(report.enabled);
        assert!(report.colored_row_count >= 1);
        assert!(report.header_row_colored);
        assert!(report
            .rows
            .iter()
            .any(|row| row.begin_row.contains("background-color")));
    }

    #[test]
    fn legacy_bbcode_row_style_uses_colored_begin_col() {
        let report = continuum_m_row_style_smoke(OutputMode::Bbcode);
        assert!(report.bbcode_row_style_count >= 1);
        assert!(report
            .rows
            .iter()
            .any(|row| row.begin_row.starts_with("[tr=")));
    }

    #[test]
    fn nocolor_policy_can_disable_styles() {
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
            &MaterializedTableViewConfig::default()
                .with_virtual_policy(VirtualColumnDisplayPolicy::TagSummary),
        );
        let config = TableViewRowStyleConfig::legacy_colored().without_color();
        let report = row_style_report_for_rows(&view.rows, OutputMode::Html, &config, false, true);
        assert!(!report.enabled);
        assert_eq!(report.colored_row_count, 0);
    }
}
