//! Semantic parity helpers for materialized table-view output.
//!
//! Stage 24 compared the new `table_view_output` renderer against legacy output
//! with a strict line-by-line diff.  That remains the only commit criterion.
//! Stage 25 adds a second, safer diagnostic layer: mode-aware normalization and
//! semantic row/cell comparison.  It tells us whether a mismatch is only syntax
//! noise (HTML wrappers, Markdown separator rows, shell spacing) or an actual
//! table-cell mismatch, without changing visible behaviour by itself.
//! Stage 35 makes that normalization style-aware for HTML/BBCode: styled rows,
//! styled cells, catalog classes and multi-line cell wrappers are parsed as the
//! same semantic table cells, while raw line equality remains the commit guard.

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
    /// Parse HTML/BBCode as a whole document instead of independent lines so
    /// legacy multi-line <td>...</td> and [td=...]...[/td] wrappers normalize
    /// to the same semantic cells as compact Rust renderer lines.
    pub style_aware_markup: bool,
    /// Ignore row/cell style wrappers during semantic comparison.  This is a
    /// diagnostic-only relaxation; raw line equality is still required before
    /// any visible commit can happen.
    pub ignore_style_wrappers: bool,
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
            style_aware_markup: true,
            ignore_style_wrappers: true,
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
    pub style_wrapper_line_count: usize,
    pub document_normalized: bool,
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
                "table_view_output_parity.parse_markup_document_rows".to_string(),
                "table_view_output_parity.parse_bbcode_cells".to_string(),
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
    if config.style_aware_markup && matches!(config.mode, OutputMode::Html | OutputMode::Bbcode) {
        return normalize_markup_document_lines(lines, config);
    }

    let mut normalized = Vec::new();
    let mut semantic_rows = Vec::new();
    let mut ignored_line_count = 0usize;
    let mut style_wrapper_line_count = 0usize;

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
        if config.ignore_style_wrappers && is_style_wrapper_line(raw_trimmed, config.mode) {
            style_wrapper_line_count += 1;
        }
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

    normalized_output_report(
        lines.len(),
        normalized,
        semantic_rows,
        ignored_line_count,
        style_wrapper_line_count,
        false,
        config,
    )
}

fn normalize_markup_document_lines(
    lines: &[String],
    config: &TableViewOutputParityConfig,
) -> NormalizedOutputReport {
    let document = if config.strip_ansi {
        strip_ansi_escape_sequences(&lines.join("\n"))
    } else {
        lines.join("\n")
    };
    let mut semantic_rows = parse_markup_document_rows(&document, config.mode)
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| canonicalize_cell(&cell, config))
                .filter(|cell| !cell.is_empty())
                .collect::<Vec<_>>()
        })
        .filter(|row: &Vec<String>| !row.is_empty() || !config.ignore_empty_lines)
        .collect::<Vec<_>>();

    // Some legacy dumps contain only plain text lines after structural tags.
    // If no cell wrappers are found, fall back to the line-local parser instead
    // of fabricating equality.
    if semantic_rows.is_empty() && !document_looks_like_markup_table(&document, config.mode) {
        let mut local_config = config.clone();
        local_config.style_aware_markup = false;
        return normalize_output_lines(lines, &local_config);
    }

    if config.ignore_empty_lines {
        semantic_rows.retain(|row| !row.is_empty());
    }

    let normalized = semantic_rows
        .iter()
        .enumerate()
        .map(|(index, cells)| NormalizedOutputLine {
            raw: format!("<semantic-row:{index}>"),
            cells: cells.clone(),
            canonical_line: cells.join("\u{1f}"),
            ignored: false,
            reason: "document_markup_cells".to_string(),
        })
        .collect::<Vec<_>>();
    let style_wrapper_line_count = lines
        .iter()
        .filter(|line| is_style_wrapper_line(line.trim(), config.mode))
        .count();
    let ignored_line_count = lines.len().saturating_sub(normalized.len());
    normalized_output_report(
        lines.len(),
        normalized,
        semantic_rows,
        ignored_line_count,
        style_wrapper_line_count,
        true,
        config,
    )
}

fn normalized_output_report(
    source_line_count: usize,
    normalized: Vec<NormalizedOutputLine>,
    semantic_rows: Vec<Vec<String>>,
    ignored_line_count: usize,
    style_wrapper_line_count: usize,
    document_normalized: bool,
    config: &TableViewOutputParityConfig,
) -> NormalizedOutputReport {
    let canonical_lines = normalized
        .iter()
        .filter(|line| !line.ignored)
        .map(|line| line.canonical_line.clone())
        .collect::<Vec<_>>();
    let cell_count = semantic_rows.iter().map(Vec::len).sum::<usize>();
    let digest = stable_digest_for_rows(&semantic_rows);
    NormalizedOutputReport {
        class: "NormalizedOutputReport".to_string(),
        mode: config.mode.canonical_name().to_string(),
        source_line_count,
        normalized_line_count: canonical_lines.len(),
        semantic_row_count: semantic_rows.len(),
        cell_count,
        ignored_line_count,
        style_wrapper_line_count,
        document_normalized,
        canonical_lines,
        semantic_rows,
        digest,
        universal_property:
            "syntax_and_style_wrappers_are_removed_but_cell_order_and_cell_text_are_preserved".to_string(),
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

pub fn parse_markup_document_rows(document: &str, mode: OutputMode) -> Vec<Vec<String>> {
    match mode {
        OutputMode::Html => parse_html_document_rows(document),
        OutputMode::Bbcode => parse_bbcode_document_rows(document),
        _ => document
            .lines()
            .map(|line| parse_line_as_cells(line, mode))
            .filter(|cells| !cells.is_empty())
            .collect(),
    }
}

fn parse_html_document_rows(document: &str) -> Vec<Vec<String>> {
    let lower = document.to_ascii_lowercase();
    let mut rows = Vec::new();
    let mut search_start = 0usize;
    while let Some(start_relative) = lower[search_start..].find("<tr") {
        let row_start = search_start + start_relative;
        let Some(open_end_relative) = lower[row_start..].find('>') else {
            break;
        };
        let content_start = row_start + open_end_relative + 1;
        let Some(end_relative) = lower[content_start..].find("</tr>") else {
            break;
        };
        let content_end = content_start + end_relative;
        let cells = parse_html_cells(&document[content_start..content_end]);
        if !cells.is_empty() {
            rows.push(cells);
        }
        search_start = content_end + 5;
        if search_start >= document.len() {
            break;
        }
    }
    if rows.is_empty() {
        let cells = parse_html_cells(document);
        if !cells.is_empty() {
            rows.push(cells);
        }
    }
    rows
}

fn parse_bbcode_document_rows(document: &str) -> Vec<Vec<String>> {
    let lower = document.to_ascii_lowercase();
    let mut rows = Vec::new();
    let mut search_start = 0usize;
    while let Some(row_start_relative) = find_bbcode_open_tag(&lower[search_start..], "tr") {
        let row_start = search_start + row_start_relative;
        let Some(open_end_relative) = lower[row_start..].find(']') else {
            break;
        };
        let content_start = row_start + open_end_relative + 1;
        let Some(end_relative) = lower[content_start..].find("[/tr]") else {
            break;
        };
        let content_end = content_start + end_relative;
        let cells = parse_bbcode_cells(&document[content_start..content_end]);
        if !cells.is_empty() {
            rows.push(cells);
        }
        search_start = content_end + 5;
        if search_start >= document.len() {
            break;
        }
    }
    if rows.is_empty() {
        let cells = parse_bbcode_cells(document);
        if !cells.is_empty() {
            rows.push(cells);
        }
    }
    rows
}

fn parse_bbcode_cells(line: &str) -> Vec<String> {
    let lower = line.to_ascii_lowercase();
    let mut cells = Vec::new();
    let mut search_start = 0usize;
    while let Some(start_relative) = find_bbcode_open_tag(&lower[search_start..], "td")
        .or_else(|| find_bbcode_open_tag(&lower[search_start..], "th"))
    {
        let tag_start = search_start + start_relative;
        let Some(open_end_relative) = lower[tag_start..].find(']') else {
            break;
        };
        let content_start = tag_start + open_end_relative + 1;
        let closing = if lower[tag_start + 1..].starts_with("th") { "[/th]" } else { "[/td]" };
        let Some(end_relative) = lower[content_start..].find(closing) else {
            break;
        };
        let content_end = content_start + end_relative;
        cells.push(line[content_start..content_end].to_string());
        search_start = content_end + closing.len();
        if search_start >= line.len() {
            break;
        }
    }
    cells
}

fn find_bbcode_open_tag(haystack_lower: &str, tag: &str) -> Option<usize> {
    let needle = format!("[{tag}");
    let mut cursor = 0usize;
    while let Some(relative) = haystack_lower[cursor..].find(&needle) {
        let start = cursor + relative;
        let after = haystack_lower[start + needle.len()..].chars().next();
        if matches!(after, Some(']') | Some('=') | Some(' ')) {
            return Some(start);
        }
        cursor = start + needle.len();
    }
    None
}

fn document_looks_like_markup_table(document: &str, mode: OutputMode) -> bool {
    let lower = document.to_ascii_lowercase();
    match mode {
        OutputMode::Html => lower.contains("<table") || lower.contains("<tr") || lower.contains("<td") || lower.contains("<th"),
        OutputMode::Bbcode => lower.contains("[table") || find_bbcode_open_tag(&lower, "tr").is_some() || find_bbcode_open_tag(&lower, "td").is_some() || find_bbcode_open_tag(&lower, "th").is_some(),
        _ => false,
    }
}

fn is_style_wrapper_line(value: &str, mode: OutputMode) -> bool {
    let lower = value.trim().to_ascii_lowercase();
    match mode {
        OutputMode::Html => {
            (lower.starts_with("<tr") || lower.starts_with("<td") || lower.starts_with("<th"))
                && (lower.contains("class=") || lower.contains("style=") || lower.contains("background-color"))
        }
        OutputMode::Bbcode => {
            (lower.starts_with("[tr") || lower.starts_with("[td") || lower.starts_with("[th"))
                && (lower.contains("=") || lower.contains("background-color"))
        }
        _ => false,
    }
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
    let lower = value.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "<table>"
            | "</table>"
            | "<tr>"
            | "</tr>"
            | "<td>"
            | "</td>"
            | "<th>"
            | "</th>"
            | "[table]"
            | "[/table]"
            | "[tr]"
            | "[/tr]"
            | "[td]"
            | "[/td]"
            | "[th]"
            | "[/th]"
    ) || lower.starts_with("<table ")
        || lower.starts_with("<tr ")
        || lower.starts_with("<td ")
        || lower.starts_with("<th ")
        || lower.starts_with("[tr=")
        || lower.starts_with("[td=")
        || lower.starts_with("[th=")
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

    #[test]
    fn styled_bbcode_cells_normalize_like_plain_bbcode_cells() {
        let config = TableViewOutputParityConfig::default().with_mode(OutputMode::Bbcode);
        let styled = vec![
            "[table]".to_string(),
            r#"[tr="background-color:#66ff66;color:#000000;"][td=""]A[/td][td=""]B[/td][/tr]"#.to_string(),
            "[/table]".to_string(),
        ];
        let plain = vec!["[tr][td]A[/td][td]B[/td][/tr]".to_string()];
        let report = compare_output_lines(&plain, &styled, &config);
        assert!(!report.raw_equal);
        assert!(report.semantic_equal);
        assert!(report.right.style_wrapper_line_count >= 1);
        assert!(report.right.document_normalized);
    }

    #[test]
    fn multiline_html_cells_normalize_to_same_semantic_rows_as_compact_html() {
        let config = TableViewOutputParityConfig::default().with_mode(OutputMode::Html);
        let multiline = vec![
            r#"<table border=0 id="bigtable">"#.to_string(),
            r#"<tr style="background-color:#66ff66;color:#000000;">"#.to_string(),
            r#"<td class="z_0 r_493 catalog" style="color:#000;">"#.to_string(),
            "A &amp; B".to_string(),
            "</td>".to_string(),
            "</tr>".to_string(),
            "</table>".to_string(),
        ];
        let compact = vec!["<tr><td>A &amp; B</td></tr>".to_string()];
        let report = compare_output_lines(&compact, &multiline, &config);
        assert!(report.semantic_equal);
        assert_eq!(report.left.semantic_rows, vec![vec!["A & B".to_string()]]);
        assert!(report.right.style_wrapper_line_count >= 2);
    }

    #[test]
    fn style_aware_normalization_does_not_make_raw_commit_safe() {
        let config = TableViewOutputParityConfig::default().with_mode(OutputMode::Html);
        let plain = vec!["<tr><td>A</td></tr>".to_string()];
        let styled = vec![r#"<tr style="background-color:#fff;"><td class="x">A</td></tr>"#.to_string()];
        let report = compare_output_lines(&plain, &styled, &config);
        assert!(report.semantic_equal);
        assert!(!report.is_commit_safe_raw());
    }

}
