//! Output rendering for materialized table views.
//!
//! Stage 23 sits one step after `table_view`: Rust can already materialize CSV
//! sections and keep virtual/non-direct columns as witnesses.  This module turns
//! a `MaterializedTableView` into deterministic output-mode lines.  It is still
//! guarded by the shadow/commit pipeline; the goal is not to replace the legacy
//! renderer blindly, but to make the Rust renderer explicit, testable and
//! mode-aware.

use serde::{Deserialize, Serialize};

use crate::output_syntax::OutputMode;
use crate::parameter_runtime::bootstrap_parameter_runtime;
use crate::table_materialization::{bootstrap_table_materialization, TableMaterializationConfig};
use crate::table_view::{
    bootstrap_table_view, MaterializedTableView, MaterializedTableViewConfig,
    MaterializedTableViewRow, VirtualColumnDisplayPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputConfig {
    pub mode: OutputMode,
    pub shell_separator: String,
    pub csv_separator: char,
    pub include_markdown_header_separator: bool,
    pub include_empty_rows: bool,
    pub virtual_column_policy: VirtualColumnDisplayPolicy,
}

impl Default for TableViewOutputConfig {
    fn default() -> Self {
        Self {
            mode: OutputMode::Shell,
            shell_separator: " | ".to_string(),
            csv_separator: ';',
            include_markdown_header_separator: true,
            include_empty_rows: true,
            virtual_column_policy: VirtualColumnDisplayPolicy::Suppress,
        }
    }
}

impl TableViewOutputConfig {
    pub fn with_mode(mut self, mode: OutputMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_virtual_policy(mut self, policy: VirtualColumnDisplayPolicy) -> Self {
        self.virtual_column_policy = policy;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub output_modes: Vec<String>,
    pub default_virtual_policy: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputReport {
    pub class: String,
    pub mode: String,
    pub row_count: usize,
    pub cell_count: usize,
    pub virtual_cell_count: usize,
    pub rendered_line_count: usize,
    pub rendered_lines: Vec<String>,
    pub rendered_text: String,
    pub table_view_policy: String,
    pub continuum_m_direct_header_present: bool,
    pub continuum_m_virtual_744_kept_as_witness: bool,
    pub visible_output_is_empty: bool,
    pub universal_property: String,
}

impl TableViewOutputReport {
    pub fn contains_text(&self, needle: &str) -> bool {
        self.rendered_lines.iter().any(|line| line.contains(needle))
            || self.rendered_text.contains(needle)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputBundle;

impl TableViewOutputBundle {
    pub fn snapshot(&self) -> TableViewOutputSnapshot {
        TableViewOutputSnapshot {
            class: "TableViewOutputBundle".to_string(),
            morphisms: vec![
                "render_materialized_table_view".to_string(),
                "render_table_view_rows_as_mode".to_string(),
                "render_table_view_for_cli_args".to_string(),
                "csv_escape_cell".to_string(),
                "html_escape_cell".to_string(),
                "markdown_escape_cell".to_string(),
            ],
            output_modes: vec![
                OutputMode::Shell.canonical_name().to_string(),
                OutputMode::Csv.canonical_name().to_string(),
                OutputMode::Html.canonical_name().to_string(),
                OutputMode::Bbcode.canonical_name().to_string(),
                OutputMode::Emacs.canonical_name().to_string(),
                OutputMode::Markdown.canonical_name().to_string(),
                OutputMode::Nichts.canonical_name().to_string(),
            ],
            default_virtual_policy: VirtualColumnDisplayPolicy::Suppress.canonical().to_string(),
            universal_property:
                "one materialized table view has deterministic images in every output syntax"
                    .to_string(),
        }
    }

    pub fn render_view(
        &self,
        view: &MaterializedTableView,
        config: &TableViewOutputConfig,
    ) -> TableViewOutputReport {
        render_materialized_table_view(view, config)
    }

    pub fn render_cli_args<S: AsRef<str>>(
        &self,
        args: &[S],
        materialization_config: &TableMaterializationConfig,
        config: &TableViewOutputConfig,
    ) -> TableViewOutputReport {
        render_table_view_for_cli_args(args, materialization_config, config)
    }
}

pub fn bootstrap_table_view_output() -> TableViewOutputBundle {
    TableViewOutputBundle
}

pub fn render_table_view_for_cli_args<S: AsRef<str>>(
    args: &[S],
    materialization_config: &TableMaterializationConfig,
    config: &TableViewOutputConfig,
) -> TableViewOutputReport {
    let parsed = bootstrap_parameter_runtime().parse_cli_args(args);
    let mode = parsed.selected_output_mode.unwrap_or(config.mode);
    let view_config = MaterializedTableViewConfig::default()
        .with_virtual_policy(config.virtual_column_policy);
    let report = bootstrap_table_materialization()
        .materialize_command_sets(&parsed.command_sets, materialization_config);
    let view = bootstrap_table_view().view_from_report(&report, &view_config);
    let mut mode_config = config.clone();
    mode_config.mode = mode;
    render_materialized_table_view(&view, &mode_config)
}

pub fn render_materialized_table_view(
    view: &MaterializedTableView,
    config: &TableViewOutputConfig,
) -> TableViewOutputReport {
    let rendered_lines = render_table_view_rows_as_mode(&view.rows, config);
    let rendered_text = rendered_lines.join("\n");
    let cell_count = view.rows.iter().map(|row| row.cells.len()).sum::<usize>();
    TableViewOutputReport {
        class: "TableViewOutputReport".to_string(),
        mode: config.mode.canonical_name().to_string(),
        row_count: view.row_count,
        cell_count,
        virtual_cell_count: view.rendered_virtual_cell_count,
        rendered_line_count: rendered_lines.len(),
        rendered_lines,
        rendered_text,
        table_view_policy: view.policy.clone(),
        continuum_m_direct_header_present: view.continuum_m_direct_header_present,
        continuum_m_virtual_744_kept_as_witness: view.continuum_m_virtual_744_kept_as_witness,
        visible_output_is_empty: matches!(config.mode, OutputMode::Nichts),
        universal_property:
            "formatting changes syntax only; materialized row/column order stays unchanged"
                .to_string(),
    }
}

pub fn render_table_view_rows_as_mode(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    match config.mode {
        OutputMode::Nichts => Vec::new(),
        OutputMode::Shell => render_shell_rows(rows, config),
        OutputMode::Csv => render_csv_rows(rows, config.csv_separator, config.include_empty_rows),
        OutputMode::Markdown => render_markdown_rows(rows, config),
        OutputMode::Emacs => render_pipe_rows(rows, config.include_empty_rows),
        OutputMode::Html => render_html_rows(rows, config.include_empty_rows),
        OutputMode::Bbcode => render_bbcode_rows(rows, config.include_empty_rows),
    }
}

pub fn render_shell_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    rows.iter()
        .filter_map(|row| {
            let values = row_values(row);
            if values.is_empty() && !config.include_empty_rows {
                None
            } else {
                Some(values.join(&config.shell_separator))
            }
        })
        .collect()
}

pub fn render_csv_rows(
    rows: &[MaterializedTableViewRow],
    separator: char,
    include_empty_rows: bool,
) -> Vec<String> {
    rows.iter()
        .filter_map(|row| {
            let values = row_values(row);
            if values.is_empty() && !include_empty_rows {
                None
            } else {
                Some(
                    values
                        .iter()
                        .map(|cell| csv_escape_cell(cell, separator))
                        .collect::<Vec<_>>()
                        .join(&separator.to_string()),
                )
            }
        })
        .collect()
}

pub fn render_pipe_rows(rows: &[MaterializedTableViewRow], include_empty_rows: bool) -> Vec<String> {
    rows.iter()
        .filter_map(|row| {
            let values = row_values(row);
            if values.is_empty() && !include_empty_rows {
                None
            } else {
                Some(format!("|{}|", values.join("|")))
            }
        })
        .collect()
}

pub fn render_markdown_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    let mut out = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let values = row_values(row);
        if values.is_empty() && !config.include_empty_rows {
            continue;
        }
        let escaped = values
            .iter()
            .map(|cell| markdown_escape_cell(cell))
            .collect::<Vec<_>>();
        out.push(format!("| {} |", escaped.join(" | ")));
        if index == 0 && config.include_markdown_header_separator && !escaped.is_empty() {
            out.push(format!(
                "| {} |",
                escaped
                    .iter()
                    .map(|_| "---".to_string())
                    .collect::<Vec<_>>()
                    .join(" | ")
            ));
        }
    }
    out
}

pub fn render_html_rows(rows: &[MaterializedTableViewRow], include_empty_rows: bool) -> Vec<String> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut out = vec![r#"<table border=0 id="bigtable">"#.to_string()];
    for row in rows {
        let values = row_values(row);
        if values.is_empty() && !include_empty_rows {
            continue;
        }
        out.push("<tr>".to_string());
        for value in values {
            out.push(format!("<td>{}</td>", html_escape_cell(&value)));
        }
        out.push("</tr>".to_string());
    }
    out.push("</table>".to_string());
    out
}

pub fn render_bbcode_rows(rows: &[MaterializedTableViewRow], include_empty_rows: bool) -> Vec<String> {
    if rows.is_empty() {
        return Vec::new();
    }
    let mut out = vec!["[table]".to_string()];
    for row in rows {
        let values = row_values(row);
        if values.is_empty() && !include_empty_rows {
            continue;
        }
        let cells = values
            .iter()
            .map(|value| format!("[td]{}[/td]", bbcode_escape_cell(value)))
            .collect::<Vec<_>>()
            .join("");
        out.push(format!("[tr]{cells}[/tr]"));
    }
    out.push("[/table]".to_string());
    out
}

pub fn row_values(row: &MaterializedTableViewRow) -> Vec<String> {
    row.cells.iter().map(|cell| cell.value.clone()).collect()
}

pub fn csv_escape_cell(value: &str, separator: char) -> String {
    let must_quote = value.contains(separator)
        || value.contains('\n')
        || value.contains('\r')
        || value.contains('"');
    if !must_quote {
        return value.to_string();
    }
    format!("\"{}\"", value.replace('"', "\"\""))
}

pub fn html_escape_cell(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn markdown_escape_cell(value: &str) -> String {
    value.replace('\\', "\\\\").replace('|', "\\|").replace('\n', " ")
}

pub fn bbcode_escape_cell(value: &str) -> String {
    value.replace('[', "&#91;").replace(']', "&#93;")
}

pub fn continuum_m_table_view_output_smoke(mode: OutputMode) -> TableViewOutputReport {
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "-ausgabe".to_string(),
        format!("--art={}", mode.canonical_name()),
        "--breite=0".to_string(),
    ];
    render_table_view_for_cli_args(
        &args,
        &TableMaterializationConfig::default(),
        &TableViewOutputConfig::default().with_mode(mode),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_output_renders_continuum_m_direct_column_without_virtual_744_by_default() {
        let report = continuum_m_table_view_output_smoke(OutputMode::Shell);
        assert_eq!(report.mode, "shell");
        assert!(report.continuum_m_direct_header_present);
        assert!(report.continuum_m_virtual_744_kept_as_witness);
        assert_eq!(report.virtual_cell_count, 0);
        assert!(report.contains_text("M Kontinuum"));
        assert!(!report.contains_text("744:sternPolygon"));
    }

    #[test]
    fn csv_output_quotes_separator_and_html_output_escapes_markup() {
        assert_eq!(csv_escape_cell("a;b", ';'), "\"a;b\"");
        assert_eq!(csv_escape_cell("plain", ';'), "plain");
        assert_eq!(html_escape_cell("a<b&c>"), "a&lt;b&amp;c&gt;");
    }

    #[test]
    fn markdown_output_adds_separator_after_header() {
        let report = continuum_m_table_view_output_smoke(OutputMode::Markdown);
        assert_eq!(report.mode, "markdown");
        assert!(report.rendered_lines.iter().any(|line| line.contains("---")));
    }

    #[test]
    fn nichts_output_is_empty_but_keeps_report_metadata() {
        let report = continuum_m_table_view_output_smoke(OutputMode::Nichts);
        assert_eq!(report.mode, "nichts");
        assert!(report.rendered_lines.is_empty());
        assert!(report.visible_output_is_empty);
        assert!(report.continuum_m_virtual_744_kept_as_witness);
    }
}
