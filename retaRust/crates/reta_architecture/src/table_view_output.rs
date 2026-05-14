//! Output rendering for materialized table views.
//!
//! Stage 23 introduced deterministic output-mode projections for a
//! `MaterializedTableView`.  Stage 28 moves the next visible-output knobs into
//! the same typed path: output flags such as `--keineueberschriften`,
//! `--keineleereninhalte`, `--breite=…`, `--breiten=…`, `--dontwrap`,
//! `--nocolor`, `--justtext`, `--onetable`, `--endlessscreen` and `--endless`.
//! Stage 29 adds the legacy numbering/counting prefix as an explicit projection
//! instead of leaving it hidden inside the renderer.
//! The legacy renderer is still the behaviour oracle; this module makes those
//! options inspectable and shadow-comparable before any guarded commit.

use serde::{Deserialize, Serialize};

use crate::output_syntax::OutputMode;
use crate::parameter_runtime::bootstrap_parameter_runtime;
use crate::table_materialization::{bootstrap_table_materialization, TableMaterializationConfig};
use crate::table_view::{
    bootstrap_table_view, MaterializedTableView, MaterializedTableViewConfig,
    MaterializedTableViewRow, VirtualColumnDisplayPolicy,
};
use crate::table_view_numbering::{
    numbering_values_for_source_row, TableViewNumberingConfig, TableViewNumberingMode,
};
use crate::table_wrapping::alxwrap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputCliOptions {
    pub nocolor: bool,
    pub justtext: bool,
    pub onetable: bool,
    pub endlessscreen: bool,
    pub endless: bool,
    pub dontwrap: bool,
    pub keineleereninhalte: bool,
    pub keinenummerierung: bool,
    pub keineueberschriften: bool,
    pub width: Option<usize>,
    pub widths: Vec<usize>,
    pub recognized_option_count: usize,
    pub unknown_output_options: Vec<String>,
}

impl Default for TableViewOutputCliOptions {
    fn default() -> Self {
        Self {
            nocolor: false,
            justtext: false,
            onetable: false,
            endlessscreen: false,
            endless: false,
            dontwrap: false,
            keineleereninhalte: false,
            keinenummerierung: false,
            keineueberschriften: false,
            width: None,
            widths: Vec::new(),
            recognized_option_count: 0,
            unknown_output_options: Vec::new(),
        }
    }
}

impl TableViewOutputCliOptions {
    pub fn from_args<S: AsRef<str>>(args: &[S]) -> Self {
        parse_table_view_output_cli_options(args)
    }

    pub fn has_visible_layout_effect(&self) -> bool {
        self.keineleereninhalte
            || self.keineueberschriften
            || self.width.is_some()
            || !self.widths.is_empty()
            || self.dontwrap
    }

    pub fn apply_to_config(&self, base: &TableViewOutputConfig) -> TableViewOutputConfig {
        let mut config = base.clone();
        config.nocolor |= self.nocolor;
        config.justtext |= self.justtext;
        config.onetable |= self.onetable;
        config.endlessscreen |= self.endlessscreen;
        config.endless |= self.endless;
        config.dontwrap |= self.dontwrap;
        config.suppress_headers |= self.keineueberschriften;
        if self.keineleereninhalte {
            config.include_empty_rows = false;
        }
        if self.keinenummerierung {
            config.include_row_numbers = false;
            config.numbering = config.numbering.clone().disabled_by_keinenummerierung();
        }
        if !self.widths.is_empty() {
            config.per_column_widths = self.widths.clone();
        }
        if self.width.is_some() || self.dontwrap {
            config.wrap_cell_width = self.width;
        }
        if config.dontwrap {
            config.wrap_cell_width = None;
            config.per_column_widths.clear();
        }
        config.cli_options = self.clone();
        config
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputConfig {
    pub mode: OutputMode,
    pub shell_separator: String,
    pub csv_separator: char,
    pub include_markdown_header_separator: bool,
    pub include_empty_rows: bool,
    pub virtual_column_policy: VirtualColumnDisplayPolicy,
    /// Drop source/header rows before output rendering.  This corresponds to
    /// Python's `--keineueberschriften` flag.
    pub suppress_headers: bool,
    /// Rust keeps numbering disabled by default because older stages did not
    /// render row numbers.  The flag is still modeled so `--keinenummerierung`
    /// has an explicit target and future parity work can turn numbering on via
    /// policy instead of implicit renderer side effects.
    pub include_row_numbers: bool,
    pub row_number_header: String,
    pub numbering: TableViewNumberingConfig,
    pub wrap_cell_width: Option<usize>,
    pub per_column_widths: Vec<usize>,
    pub dontwrap: bool,
    pub nocolor: bool,
    pub justtext: bool,
    pub onetable: bool,
    pub endlessscreen: bool,
    pub endless: bool,
    pub cli_options: TableViewOutputCliOptions,
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
            suppress_headers: false,
            include_row_numbers: false,
            row_number_header: "#".to_string(),
            numbering: TableViewNumberingConfig::disabled(),
            wrap_cell_width: None,
            per_column_widths: Vec::new(),
            dontwrap: false,
            nocolor: false,
            justtext: false,
            onetable: false,
            endlessscreen: false,
            endless: false,
            cli_options: TableViewOutputCliOptions::default(),
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

    pub fn with_cli_options(mut self, options: TableViewOutputCliOptions) -> Self {
        self = options.apply_to_config(&self);
        self
    }

    pub fn with_wrap_width(mut self, width: Option<usize>) -> Self {
        self.wrap_cell_width = width.filter(|value| *value > 0);
        self
    }

    pub fn with_legacy_numbering(mut self) -> Self {
        self.include_row_numbers = true;
        self.numbering = TableViewNumberingConfig::legacy_pair();
        self
    }

    pub fn with_numbering(mut self, numbering: TableViewNumberingConfig) -> Self {
        self.include_row_numbers = numbering.is_enabled();
        self.numbering = numbering;
        self
    }

    pub fn width_for_cell(&self, cell_index: usize) -> Option<usize> {
        if self.dontwrap || self.mode.force_zero_width() {
            return None;
        }
        self.per_column_widths
            .get(cell_index)
            .copied()
            .filter(|value| *value > 0)
            .or(self.wrap_cell_width.filter(|value| *value > 0))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub output_modes: Vec<String>,
    pub default_virtual_policy: String,
    pub stage28_cli_options: Vec<String>,
    pub stage29_numbering_modes: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputReport {
    pub class: String,
    pub mode: String,
    pub row_count: usize,
    pub rendered_row_count: usize,
    pub cell_count: usize,
    pub virtual_cell_count: usize,
    pub rendered_line_count: usize,
    pub rendered_lines: Vec<String>,
    pub rendered_text: String,
    pub table_view_policy: String,
    pub suppress_headers: bool,
    pub include_empty_rows: bool,
    pub include_row_numbers: bool,
    pub numbering_mode: String,
    pub numbering_column_count: usize,
    pub wrap_cell_width: Option<usize>,
    pub per_column_width_count: usize,
    pub dontwrap: bool,
    pub nocolor: bool,
    pub justtext: bool,
    pub onetable: bool,
    pub endlessscreen: bool,
    pub endless: bool,
    pub cli_options: TableViewOutputCliOptions,
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
                "parse_table_view_output_cli_options".to_string(),
                "render_materialized_table_view".to_string(),
                "render_table_view_rows_as_mode".to_string(),
                "render_table_view_for_cli_args".to_string(),
                "rendered_row_value_lines".to_string(),
                "wrap_output_cell".to_string(),
                "numbering_values_for_source_row".to_string(),
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
            stage28_cli_options: vec![
                "nocolor".to_string(),
                "justtext".to_string(),
                "onetable".to_string(),
                "endlessscreen".to_string(),
                "endless".to_string(),
                "dontwrap".to_string(),
                "breite".to_string(),
                "breiten".to_string(),
                "keineleereninhalte".to_string(),
                "keinenummerierung".to_string(),
                "keineueberschriften".to_string(),
            ],
            stage29_numbering_modes: vec![
                TableViewNumberingMode::Disabled.canonical().to_string(),
                TableViewNumberingMode::LegacyPair.canonical().to_string(),
                TableViewNumberingMode::NumberOnly.canonical().to_string(),
                TableViewNumberingMode::CountingOnly.canonical().to_string(),
            ],
            universal_property:
                "one materialized table view has deterministic images in every output syntax, output-option and numbering context"
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

pub fn parse_table_view_output_cli_options<S: AsRef<str>>(args: &[S]) -> TableViewOutputCliOptions {
    let mut options = TableViewOutputCliOptions::default();
    let mut active_output_context = false;
    for arg in args {
        let raw = arg.as_ref();
        match raw {
            "-ausgabe" | "-a" => {
                active_output_context = true;
                continue;
            }
            "-zeilen" | "-z" | "-spalten" | "-s" | "-kombination" | "-k" | "-debug" => {
                active_output_context = false;
            }
            _ => {}
        }
        let Some(body) = raw.strip_prefix("--") else {
            continue;
        };
        let (key, value) = split_output_option(body);
        let recognized = match key.as_str() {
            "nocolor" => {
                options.nocolor = true;
                true
            }
            "justtext" => {
                options.justtext = true;
                true
            }
            "onetable" => {
                options.onetable = true;
                true
            }
            "endlessscreen" => {
                options.endlessscreen = true;
                true
            }
            "endless" => {
                options.endless = true;
                true
            }
            "dontwrap" => {
                options.dontwrap = true;
                true
            }
            "keineleereninhalte" => {
                options.keineleereninhalte = true;
                true
            }
            "keinenummerierung" => {
                options.keinenummerierung = true;
                true
            }
            "keineueberschriften" => {
                options.keineueberschriften = true;
                true
            }
            "breite" => {
                if let Some(value) = value.as_deref() {
                    options.width = parse_positive_width(value);
                }
                true
            }
            "breiten" => {
                if let Some(value) = value.as_deref() {
                    options.widths = parse_width_list(value);
                }
                true
            }
            // `art` and `spaltenreihenfolgeundnurdiese` are parsed by the
            // parameter runtime.  They are output-context options, but they do
            // not change this renderer option bundle directly.
            "art" | "spaltenreihenfolgeundnurdiese" => true,
            _ => false,
        };
        if recognized {
            options.recognized_option_count += 1;
        } else if active_output_context {
            options.unknown_output_options.push(key);
        }
    }
    if options.dontwrap {
        options.width = None;
        options.widths.clear();
    }
    options
}

fn split_output_option(body: &str) -> (String, Option<String>) {
    match body.split_once('=') {
        Some((key, value)) => (key.to_string(), Some(value.to_string())),
        None => (body.to_string(), None),
    }
}

fn parse_positive_width(value: &str) -> Option<usize> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let parsed = trimmed.parse::<i64>().ok()?.unsigned_abs() as usize;
    (parsed > 0).then_some(parsed)
}

fn parse_width_list(value: &str) -> Vec<usize> {
    value
        .split(',')
        .filter_map(parse_positive_width)
        .collect::<Vec<_>>()
}

pub fn render_table_view_for_cli_args<S: AsRef<str>>(
    args: &[S],
    materialization_config: &TableMaterializationConfig,
    config: &TableViewOutputConfig,
) -> TableViewOutputReport {
    let parsed = bootstrap_parameter_runtime().parse_cli_args(args);
    let cli_options = parse_table_view_output_cli_options(args);
    let mut mode_config = config.clone().with_cli_options(cli_options);
    let mode = parsed.selected_output_mode.unwrap_or(mode_config.mode);
    mode_config.mode = mode;
    if mode.force_zero_width() {
        mode_config.wrap_cell_width = None;
        mode_config.per_column_widths.clear();
    }
    let view_config = MaterializedTableViewConfig::default()
        .with_virtual_policy(mode_config.virtual_column_policy);
    let report = bootstrap_table_materialization()
        .materialize_command_sets(&parsed.command_sets, materialization_config);
    let view = bootstrap_table_view().view_from_report(&report, &view_config);
    render_materialized_table_view(&view, &mode_config)
}

pub fn render_materialized_table_view(
    view: &MaterializedTableView,
    config: &TableViewOutputConfig,
) -> TableViewOutputReport {
    let rendered_lines = render_table_view_rows_as_mode(&view.rows, config);
    let rendered_text = rendered_lines.join("\n");
    let visible_output_is_empty = rendered_text.is_empty();
    let cell_count = view.rows.iter().map(|row| row.cells.len()).sum::<usize>();
    let rendered_row_count = filtered_output_rows(&view.rows, config).len();
    TableViewOutputReport {
        class: "TableViewOutputReport".to_string(),
        mode: config.mode.canonical_name().to_string(),
        row_count: view.row_count,
        rendered_row_count,
        cell_count,
        virtual_cell_count: view.rendered_virtual_cell_count,
        rendered_line_count: rendered_lines.len(),
        rendered_lines,
        rendered_text,
        table_view_policy: view.policy.clone(),
        suppress_headers: config.suppress_headers,
        include_empty_rows: config.include_empty_rows,
        include_row_numbers: config.include_row_numbers,
        numbering_mode: config.numbering.mode.canonical().to_string(),
        numbering_column_count: config.numbering.column_count(),
        wrap_cell_width: config.wrap_cell_width,
        per_column_width_count: config.per_column_widths.len(),
        dontwrap: config.dontwrap,
        nocolor: config.nocolor,
        justtext: config.justtext,
        onetable: config.onetable,
        endlessscreen: config.endlessscreen,
        endless: config.endless,
        cli_options: config.cli_options.clone(),
        continuum_m_direct_header_present: view.continuum_m_direct_header_present,
        continuum_m_virtual_744_kept_as_witness: view.continuum_m_virtual_744_kept_as_witness,
        visible_output_is_empty,
        universal_property:
            "output flags and numbering change only the selected output projection; materialized local sections stay unchanged"
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
        OutputMode::Csv => render_csv_rows_with_config(rows, config),
        OutputMode::Markdown => render_markdown_rows(rows, config),
        OutputMode::Emacs => render_pipe_rows_with_config(rows, config),
        OutputMode::Html => render_html_rows_with_config(rows, config),
        OutputMode::Bbcode => render_bbcode_rows_with_config(rows, config),
    }
}

pub fn filtered_output_rows<'a>(
    rows: &'a [MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<&'a MaterializedTableViewRow> {
    rows.iter()
        .filter(|row| !(config.suppress_headers && row.source_row_zero_based == 0))
        .filter(|row| config.include_empty_rows || !row_values(row).is_empty())
        .collect()
}

pub fn rendered_row_value_lines(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    for (display_index, row) in filtered_output_rows(rows, config).into_iter().enumerate() {
        out.extend(expand_row_to_value_lines(row, config, display_index));
    }
    out
}

pub fn expand_row_to_value_lines(
    row: &MaterializedTableViewRow,
    config: &TableViewOutputConfig,
    display_index: usize,
) -> Vec<Vec<String>> {
    let values = row_values_with_options(row, config, display_index);
    if values.is_empty() {
        return vec![Vec::new()];
    }
    let wrapped = values
        .iter()
        .enumerate()
        .map(|(index, value)| wrap_output_cell(value, config.width_for_cell(index)))
        .collect::<Vec<_>>();
    let height = wrapped.iter().map(Vec::len).max().unwrap_or(1).max(1);
    (0..height)
        .map(|line_index| {
            wrapped
                .iter()
                .map(|cell_lines| cell_lines.get(line_index).cloned().unwrap_or_default())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn row_values_with_options(
    row: &MaterializedTableViewRow,
    config: &TableViewOutputConfig,
    display_index: usize,
) -> Vec<String> {
    let mut values = row_values(row);
    if config.numbering.is_enabled() {
        let mut prefix = numbering_values_for_source_row(
            row.source_row_zero_based,
            display_index,
            &config.numbering,
        );
        prefix.extend(values);
        return prefix;
    }
    if config.include_row_numbers {
        let value = if row.source_row_zero_based == 0 {
            config.row_number_header.clone()
        } else {
            row.source_row_zero_based.to_string()
        };
        let fallback = (display_index + 1).to_string();
        values.insert(0, if value.is_empty() { fallback } else { value });
    }
    values
}

pub fn wrap_output_cell(value: &str, width: Option<usize>) -> Vec<String> {
    match width.filter(|width| *width > 0) {
        Some(width) if value.chars().count() > width => alxwrap(value, width, None),
        _ => vec![value.to_string()],
    }
}

pub fn render_shell_rows(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    rendered_row_value_lines(rows, config)
        .into_iter()
        .filter_map(|values| {
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
    let config = TableViewOutputConfig {
        csv_separator: separator,
        include_empty_rows,
        mode: OutputMode::Csv,
        ..TableViewOutputConfig::default()
    };
    render_csv_rows_with_config(rows, &config)
}

pub fn render_csv_rows_with_config(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    rendered_row_value_lines(rows, config)
        .into_iter()
        .filter_map(|values| {
            if values.is_empty() && !config.include_empty_rows {
                None
            } else {
                Some(
                    values
                        .iter()
                        .map(|cell| csv_escape_cell(cell, config.csv_separator))
                        .collect::<Vec<_>>()
                        .join(&config.csv_separator.to_string()),
                )
            }
        })
        .collect()
}

pub fn render_pipe_rows(
    rows: &[MaterializedTableViewRow],
    include_empty_rows: bool,
) -> Vec<String> {
    let config = TableViewOutputConfig {
        include_empty_rows,
        mode: OutputMode::Emacs,
        ..TableViewOutputConfig::default()
    };
    render_pipe_rows_with_config(rows, &config)
}

pub fn render_pipe_rows_with_config(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    rendered_row_value_lines(rows, config)
        .into_iter()
        .filter_map(|values| {
            if values.is_empty() && !config.include_empty_rows {
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
    let mut emitted_data_row_count = 0usize;
    for values in rendered_row_value_lines(rows, config) {
        if values.is_empty() && !config.include_empty_rows {
            continue;
        }
        let escaped = values
            .iter()
            .map(|cell| markdown_escape_cell(cell))
            .collect::<Vec<_>>();
        out.push(format!("| {} |", escaped.join(" | ")));
        if emitted_data_row_count == 0
            && config.include_markdown_header_separator
            && !config.suppress_headers
            && !escaped.is_empty()
        {
            out.push(format!(
                "| {} |",
                escaped
                    .iter()
                    .map(|_| "---".to_string())
                    .collect::<Vec<_>>()
                    .join(" | ")
            ));
        }
        emitted_data_row_count += 1;
    }
    out
}

pub fn render_html_rows(
    rows: &[MaterializedTableViewRow],
    include_empty_rows: bool,
) -> Vec<String> {
    let config = TableViewOutputConfig {
        include_empty_rows,
        mode: OutputMode::Html,
        ..TableViewOutputConfig::default()
    };
    render_html_rows_with_config(rows, &config)
}

pub fn render_html_rows_with_config(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    let row_lines = rendered_row_value_lines(rows, config);
    if row_lines.is_empty() {
        return Vec::new();
    }
    let mut out = vec![r#"<table border=0 id="bigtable">"#.to_string()];
    for values in row_lines {
        if values.is_empty() && !config.include_empty_rows {
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

pub fn render_bbcode_rows(
    rows: &[MaterializedTableViewRow],
    include_empty_rows: bool,
) -> Vec<String> {
    let config = TableViewOutputConfig {
        include_empty_rows,
        mode: OutputMode::Bbcode,
        ..TableViewOutputConfig::default()
    };
    render_bbcode_rows_with_config(rows, &config)
}

pub fn render_bbcode_rows_with_config(
    rows: &[MaterializedTableViewRow],
    config: &TableViewOutputConfig,
) -> Vec<String> {
    let row_lines = rendered_row_value_lines(rows, config);
    if row_lines.is_empty() {
        return Vec::new();
    }
    let mut out = vec!["[table]".to_string()];
    for values in row_lines {
        if values.is_empty() && !config.include_empty_rows {
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
    value
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\n', " ")
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

pub fn output_flags_smoke() -> TableViewOutputReport {
    let args = vec![
        "reta".to_string(),
        "-zeilen".to_string(),
        "--vorhervonausschnitt=1-1".to_string(),
        "-spalten".to_string(),
        "--kontinuum=m".to_string(),
        "-ausgabe".to_string(),
        "--keineueberschriften".to_string(),
        "--keineleereninhalte".to_string(),
        "--breite=8".to_string(),
    ];
    render_table_view_for_cli_args(
        &args,
        &TableMaterializationConfig::default(),
        &TableViewOutputConfig::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table_view::{MaterializedTableCellSource, MaterializedTableViewCell};

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
        assert!(report
            .rendered_lines
            .iter()
            .any(|line| line.contains("---")));
    }

    #[test]
    fn nichts_output_is_empty_but_keeps_report_metadata() {
        let report = continuum_m_table_view_output_smoke(OutputMode::Nichts);
        assert_eq!(report.mode, "nichts");
        assert!(report.rendered_lines.is_empty());
        assert!(report.visible_output_is_empty);
        assert!(report.continuum_m_virtual_744_kept_as_witness);
    }

    #[test]
    fn output_cli_options_parse_width_and_header_flags() {
        let options = parse_table_view_output_cli_options(&[
            "reta",
            "-ausgabe",
            "--keineueberschriften",
            "--keineleereninhalte",
            "--breite=8",
            "--breiten=3,5,0",
            "--nocolor",
        ]);
        assert!(options.keineueberschriften);
        assert!(options.keineleereninhalte);
        assert_eq!(options.width, Some(8));
        assert_eq!(options.widths, vec![3, 5]);
        assert!(options.nocolor);
    }

    #[test]
    fn keineueberschriften_suppresses_header_but_keeps_data() {
        let report = output_flags_smoke();
        assert!(report.suppress_headers);
        assert!(!report.contains_text("M Kontinuum"));
        assert!(report.contains_text("Wege"));
    }

    #[test]
    fn positive_width_wraps_shell_cells_but_breite_zero_does_not() {
        assert_eq!(wrap_output_cell("abcdef", Some(2)), vec!["ab", "cd", "ef"]);
        assert_eq!(wrap_output_cell("abcdef", Some(0)), vec!["abcdef"]);
        assert_eq!(wrap_output_cell("abcdef", None), vec!["abcdef"]);
    }

    #[test]
    fn keineleereninhalte_filters_empty_rows() {
        let row = MaterializedTableViewRow {
            asset_name: "test.csv".to_string(),
            source_row_zero_based: 1,
            cells: Vec::new(),
        };
        let config = TableViewOutputConfig {
            include_empty_rows: false,
            ..TableViewOutputConfig::default()
        };
        assert!(render_shell_rows(&[row], &config).is_empty());
    }

    #[test]
    fn legacy_numbering_projection_adds_zaehlung_and_nummerierung_columns() {
        let args = vec![
            "reta".to_string(),
            "-zeilen".to_string(),
            "--vorhervonausschnitt=1-1".to_string(),
            "-spalten".to_string(),
            "--kontinuum=m".to_string(),
        ];
        let report = render_table_view_for_cli_args(
            &args,
            &TableMaterializationConfig::default(),
            &TableViewOutputConfig::default().with_legacy_numbering(),
        );
        assert_eq!(report.numbering_mode, "legacy-pair");
        assert_eq!(report.numbering_column_count, 2);
        assert!(report.rendered_lines[0].contains("Zählung"));
        assert!(report.rendered_lines[0].contains("Nummerierung"));
    }

    #[test]
    fn row_number_policy_is_explicit_and_does_not_default_on() {
        let row = MaterializedTableViewRow {
            asset_name: "test.csv".to_string(),
            source_row_zero_based: 7,
            cells: vec![MaterializedTableViewCell {
                column_legacy: 493,
                source_row_zero_based: 7,
                value: "x".to_string(),
                source: MaterializedTableCellSource::Csv {
                    asset_name: "test.csv".to_string(),
                    source_column_index: 493,
                },
            }],
        };
        let default_values = row_values_with_options(&row, &TableViewOutputConfig::default(), 0);
        assert_eq!(default_values, vec!["x".to_string()]);
        let numbered = row_values_with_options(
            &row,
            &TableViewOutputConfig {
                include_row_numbers: true,
                ..TableViewOutputConfig::default()
            },
            0,
        );
        assert_eq!(numbered, vec!["7".to_string(), "x".to_string()]);
    }
}
