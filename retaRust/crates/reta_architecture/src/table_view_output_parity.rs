//! Semantic parity helpers for materialized table-view output.
//!
//! Stage 24 compared the new `table_view_output` renderer against legacy output
//! with a strict line-by-line diff.  That remains the only commit criterion.
//! Stage 25 adds a second, safer diagnostic layer: mode-aware normalization and
//! semantic row/cell comparison.  It tells us whether a mismatch is only syntax
//! noise (HTML wrappers, Markdown separator rows, shell spacing) or an actual
//! table-cell mismatch, without changing visible behaviour by itself.

use serde::{Deserialize, Serialize};

use crate::output_syntax::OutputMode;
use crate::table_view_output::TableViewOutputReport;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputParityConfig {
    pub mode: OutputMode,
    pub trim_cells: bool,
    pub collapse_whitespace: bool,
    pub ignore_empty_lines: bool,
    pub ignore_markdown_separator_rows: bool,
    pub strip_ansi: bool,
}

impl Default for TableViewOutputParityConfig {
    fn default() -> Self {
        Self {
            mode: OutputMode::Shell,
            trim_cells: true,
            collapse_whitespace: true,
            ignore_empty_lines: true,
            ignore_markdown_separator_rows: true,
            strip_ansi: true,
        }
    }
}

impl TableViewOutputParityConfig {
    pub fn with_mode(mut self, mode: OutputMode) -> Self {
        self.mode = mode;
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NormalizedOutputLine {
    pub raw: String,
    pub cells: Vec<String>,
    pub canonical_line: String,
    pub ignored: bool,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NormalizedOutputReport {
    pub class: String,
    pub mode: String,
    pub source_line_count: usize,
    pub normalized_line_count: usize,
    pub semantic_row_count: usize,
    pub cell_count: usize,
    pub ignored_line_count: usize,
    pub canonical_lines: Vec<String>,
    pub semantic_rows: Vec<Vec<String>>,
    pub digest: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputParityReport {
    pub class: String,
    pub mode: String,
    pub raw_equal: bool,
    pub normalized_equal: bool,
    pub semantic_equal: bool,
    pub left: NormalizedOutputReport,
    pub right: NormalizedOutputReport,
    pub first_normalized_mismatch_index: Option<usize>,
    pub left_normalized_at_mismatch: Option<String>,
    pub right_normalized_at_mismatch: Option<String>,
    pub first_semantic_mismatch_index: Option<usize>,
    pub left_semantic_at_mismatch: Option<Vec<String>>,
    pub right_semantic_at_mismatch: Option<Vec<String>>,
    pub universal_property: String,
}

impl TableViewOutputParityReport {
    pub fn is_commit_safe_raw(&self) -> bool {
        self.raw_equal
    }

    pub fn is_semantically_close(&self) -> bool {
        self.semantic_equal || self.normalized_equal
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputParitySnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub normalization_modes: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableViewOutputParityBundle;

impl TableViewOutputParityBundle {
    pub fn snapshot(&self) -> TableViewOutputParitySnapshot {
        TableViewOutputParitySnapshot {
            class: "TableViewOutputParityBundle".to_string(),
            morphisms: vec![
                "table_view_output_parity.normalize_output_lines".to_string(),
                "table_view_output_parity.semantic_rows_from_lines".to_string(),
                "table_view_output_parity.compare_output_lines".to_string(),
                "table_view_output_parity.compare_table_view_output_to_legacy".to_string(),
                "table_view_output_parity.strip_ansi_escape_sequences".to_string(),
            ],
            normalization_modes: vec![
                OutputMode::Shell.canonical_name().to_string(),
                OutputMode::Csv.canonical_name().to_string(),
                OutputMode::Html.canonical_name().to_string(),
                OutputMode::Bbcode.canonical_name().to_string(),
                OutputMode::Emacs.canonical_name().to_string(),
                OutputMode::Markdown.canonical_name().to_string(),
                OutputMode::Nichts.canonical_name().to_string(),
            ],
            universal_property:
                "different output syntaxes can be normalized to the same table-cell presheaf before strict commit".to_string(),
        }
    }

    pub fn normalize_lines(
        &self,
        lines: &[String],
        config: &TableViewOutputParityConfig,
    ) -> NormalizedOutputReport {
        normalize_output_lines(lines, config)
    }

    pub fn compare_lines(
        &self,
        left: &[String],
        right: &[String],
        config: &TableViewOutputParityConfig,
    ) -> TableViewOutputParityReport {
        compare_output_lines(left, right, config)
    }

    pub fn compare_table_view_output_to_legacy(
        &self,
        report: &TableViewOutputReport,
        legacy_lines: &[String],
        config: &TableViewOutputParityConfig,
    ) -> TableViewOutputParityReport {
        compare_table_view_output_to_legacy(report, legacy_lines, config)
    }
}

pub fn bootstrap_table_view_output_parity() -> TableViewOutputParityBundle {
    TableViewOutputParityBundle
}

pub fn compare_table_view_output_to_legacy(
    report: &TableViewOutputReport,
    legacy_lines: &[String],
    config: &TableViewOutputParityConfig,
) -> TableViewOutputParityReport {
    compare_output_lines(legacy_lines, &report.rendered_lines, config)
}

pub fn compare_output_lines(
    left: &[String],
    right: &[String],
    config: &TableViewOutputParityConfig,
) -> TableViewOutputParityReport {
    let raw_equal = left == right;
    let left_report = normalize_output_lines(left, config);
    let right_report = normalize_output_lines(right, config);
    let normalized_equal = left_report.canonical_lines == right_report.canonical_lines;
    let semantic_equal = left_report.semantic_rows == right_report.semantic_rows;
    let first_normalized_mismatch_index = first_mismatch_index(
        &left_report.canonical_lines,
        &right_report.canonical_lines,
    );
    let left_normalized_at_mismatch = first_normalized_mismatch_index
        .and_then(|index| left_report.canonical_lines.get(index).cloned());
    let right_normalized_at_mismatch = first_normalized_mismatch_index
        .and_then(|index| right_report.canonical_lines.get(index).cloned());
    let first_semantic_mismatch_index = first_mismatch_index(
        &left_report.semantic_rows,
        &right_report.semantic_rows,
    );
    let left_semantic_at_mismatch = first_semantic_mismatch_index
        .and_then(|index| left_report.semantic_rows.get(index).cloned());
    let right_semantic_at_mismatch = first_semantic_mismatch_index
        .and_then(|index| right_report.semantic_rows.get(index).cloned());
    TableViewOutputParityReport {
        class: "TableViewOutputParityReport".to_string(),
        mode: config.mode.canonical_name().to_string(),
        raw_equal,
        normalized_equal,
        semantic_equal,
        left: left_report,
        right: right_report,
        first_normalized_mismatch_index,
        left_normalized_at_mismatch,
        right_normalized_at_mismatch,
        first_semantic_mismatch_index,
        left_semantic_at_mismatch,
        right_semantic_at_mismatch,
        universal_property:
            "strict_line_diff_remains_commit_guard_while_normalized_rows_explain_shadow_mismatches"
                .to_string(),
    }
}

pub fn normalize_output_lines(
    lines: &[String],
    config: &TableViewOutputParityConfig,
) -> NormalizedOutputReport {
    let mut normalized = Vec::new();
    let mut semantic_rows = Vec::new();
    let mut ignored_line_count = 0usize;
    let mut cell_count = 0usize;

    for raw in lines {
        let line = if config.strip_ansi {
            strip_ansi_escape_sequences(raw)
        } else {
            raw.clone()
        };
        let raw_trimmed = line.trim();
        let empty = raw_trimmed.is_empty();
        let cells = parse_line_as_cells(&line, config.mode)
            .into_iter()
            .map(|cell| canonicalize_cell(&cell, config))
            .filter(|cell| !cell.is_empty())
            .collect::<Vec<_>>();
        let markdown_separator = config.ignore_markdown_separator_rows
            && config.mode == OutputMode::Markdown
            && is_markdown_separator_row(&cells);
        let structural_markup = matches!(config.mode, OutputMode::Html | OutputMode::Bbcode)
            && cells.is_empty()
            && looks_like_table_markup(raw_trimmed);
        let ignored = (config.ignore_empty_lines && empty) || markdown_separator || structural_markup;
        let reason = if markdown_separator {
            "markdown_separator".to_string()
        } else if structural_markup {
            "structural_table_markup".to_string()
        } else if empty {
            "empty".to_string()
        } else {
            "kept".to_string()
        };
        let canonical_line = cells.join("\u{1f}");
        if ignored {
            ignored_line_count += 1;
        } else {
            cell_count += cells.len();
            semantic_rows.push(cells.clone());
        }
        normalized.push(NormalizedOutputLine {
            raw: raw.clone(),
            cells,
            canonical_line,
            ignored,
            reason,
        });
    }

    let canonical_lines = normalized
        .iter()
        .filter(|line| !line.ignored)
        .map(|line| line.canonical_line.clone())
        .collect::<Vec<_>>();
    let digest = stable_digest_for_rows(&semantic_rows);
    NormalizedOutputReport {
        class: "NormalizedOutputReport".to_string(),
        mode: config.mode.canonical_name().to_string(),
        source_line_count: lines.len(),
        normalized_line_count: canonical_lines.len(),
        semantic_row_count: semantic_rows.len(),
        cell_count,
        ignored_line_count,
        canonical_lines,
        semantic_rows,
        digest,
        universal_property:
            "syntax_wrappers_are_removed_but_cell_order_and_cell_text_are_preserved".to_string(),
    }
}

pub fn semantic_rows_from_lines(
    lines: &[String],
    config: &TableViewOutputParityConfig,
) -> Vec<Vec<String>> {
    normalize_output_lines(lines, config).semantic_rows
}

pub fn parse_line_as_cells(line: &str, mode: OutputMode) -> Vec<String> {
    match mode {
        OutputMode::Nichts => Vec::new(),
        OutputMode::Csv => parse_csv_line(line, ';'),
        OutputMode::Markdown => parse_pipe_line(line),
        OutputMode::Emacs => parse_pipe_line(line),
        OutputMode::Html => parse_html_cells(line),
        OutputMode::Bbcode => parse_bbcode_cells(line),
        OutputMode::Shell => parse_shell_line(line),
    }
}

pub fn canonicalize_cell(value: &str, config: &TableViewOutputParityConfig) -> String {
    let mut text = html_unescape(value).replace("&#91;", "[").replace("&#93;", "]");
    if config.trim_cells {
        text = text.trim().to_string();
    }
    if config.collapse_whitespace {
        text = collapse_whitespace(&text);
    }
    text
}

pub fn strip_ansi_escape_sequences(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                let _ = chars.next();
                while let Some(next) = chars.next() {
                    if ('@'..='~').contains(&next) {
                        break;
                    }
                }
                continue;
            }
        }
        out.push(ch);
    }
    out
}

fn parse_shell_line(line: &str) -> Vec<String> {
    if line.contains(" | ") {
        line.split(" | ").map(ToString::to_string).collect()
    } else if line.contains('│') {
        line.split('│').map(ToString::to_string).collect()
    } else if line.contains('\t') {
        line.split('\t').map(ToString::to_string).collect()
    } else {
        vec![line.to_string()]
    }
}

fn parse_pipe_line(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    if !trimmed.contains('|') {
        return parse_shell_line(line);
    }
    trimmed
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.replace("\\|", "|"))
        .collect()
}

fn parse_csv_line(line: &str, separator: char) -> Vec<String> {
    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '"' {
            if in_quotes && chars.peek() == Some(&'"') {
                let _ = chars.next();
                cell.push('"');
            } else {
                in_quotes = !in_quotes;
            }
        } else if ch == separator && !in_quotes {
            cells.push(cell.clone());
            cell.clear();
        } else {
            cell.push(ch);
        }
    }
    cells.push(cell);
    cells
}

fn parse_html_cells(line: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let lower = line.to_ascii_lowercase();
    let mut search_start = 0usize;
    while let Some(tag_relative) = lower[search_start..]
        .find("<td")
        .or_else(|| lower[search_start..].find("<th"))
    {
        let tag_start = search_start + tag_relative;
        let Some(content_start_relative) = lower[tag_start..].find('>') else {
            break;
        };
        let content_start = tag_start + content_start_relative + 1;
        let end_td = lower[content_start..].find("</td>");
        let end_th = lower[content_start..].find("</th>");
        let end_relative = match (end_td, end_th) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
        let Some(end_relative) = end_relative else {
            break;
        };
        let content_end = content_start + end_relative;
        cells.push(strip_html_tags(&line[content_start..content_end]));
        search_start = content_end + 5;
        if search_start >= line.len() {
            break;
        }
    }
    cells
}

fn parse_bbcode_cells(line: &str) -> Vec<String> {
    let lower = line.to_ascii_lowercase();
    let mut cells = Vec::new();
    let mut search_start = 0usize;
    while let Some(start_relative) = lower[search_start..].find("[td]") {
        let content_start = search_start + start_relative + 4;
        let Some(end_relative) = lower[content_start..].find("[/td]") else {
            break;
        };
        let content_end = content_start + end_relative;
        cells.push(line[content_start..content_end].to_string());
        search_start = content_end + 5;
        if search_start >= line.len() {
            break;
        }
    }
    cells
}

fn strip_html_tags(value: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for ch in value.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn html_unescape(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn collapse_whitespace(value: &str) -> String {
    let mut out = String::new();
    let mut pending_space = false;
    for ch in value.chars() {
        if ch.is_whitespace() {
            pending_space = true;
        } else {
            if pending_space && !out.is_empty() {
                out.push(' ');
            }
            pending_space = false;
            out.push(ch);
        }
    }
    out
}

fn is_markdown_separator_row(cells: &[String]) -> bool {
    !cells.is_empty()
        && cells.iter().all(|cell| {
            let stripped = cell.trim().trim_matches(':');
            !stripped.is_empty() && stripped.chars().all(|ch| ch == '-')
        })
}

fn looks_like_table_markup(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "<table>" | "</table>" | "<tr>" | "</tr>" | "[table]" | "[/table]"
    ) || value.starts_with("<table ")
}

fn first_mismatch_index<T: PartialEq>(left: &[T], right: &[T]) -> Option<usize> {
    let max_len = left.len().max(right.len());
    (0..max_len).find(|index| left.get(*index) != right.get(*index))
}

fn stable_digest_for_rows(rows: &[Vec<String>]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for row in rows {
        for cell in row {
            for byte in cell.as_bytes() {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(0x100000001b3);
            }
            hash ^= 0x1f;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0x1e;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_separator_is_ignored_semantically() {
        let config = TableViewOutputParityConfig::default().with_mode(OutputMode::Markdown);
        let left = vec!["| A |".to_string(), "| x |".to_string()];
        let right = vec![
            "| A |".to_string(),
            "| --- |".to_string(),
            "| x |".to_string(),
        ];
        let report = compare_output_lines(&left, &right, &config);
        assert!(!report.raw_equal);
        assert!(report.semantic_equal);
    }

    #[test]
    fn html_and_plain_shell_can_normalize_to_same_single_cell() {
        let html_config = TableViewOutputParityConfig::default().with_mode(OutputMode::Html);
        let report = compare_output_lines(
            &["<table border=0 id=\"bigtable\">".to_string(), "<tr>".to_string(), "<td>A &amp; B</td>".to_string(), "</tr>".to_string(), "</table>".to_string()],
            &["<td>A &amp; B</td>".to_string()],
            &html_config,
        );
        assert!(report.semantic_equal);
        assert_eq!(report.left.semantic_rows, vec![vec!["A & B".to_string()]]);
    }

    #[test]
    fn csv_quote_parser_keeps_separator_inside_cell() {
        let config = TableViewOutputParityConfig::default().with_mode(OutputMode::Csv);
        let rows = semantic_rows_from_lines(&["a;\"b;c\"".to_string()], &config);
        assert_eq!(rows, vec![vec!["a".to_string(), "b;c".to_string()]]);
    }
}
