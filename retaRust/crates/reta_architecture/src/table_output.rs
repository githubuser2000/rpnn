//! Table output rendering morphisms transcompiled from
//! `python_arch_reference/reta_architecture/table_output.py`.
//!
//! This stage does not remove the legacy byte-exact renderer yet.  It gives
//! Rust a typed renderer plan for the already-prepared table section: column
//! selection, maximum cell width discovery, row-width decisions, ANSI color
//! selection and deterministic rendering for the major output modes.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::number_theory::{moon_number, prime_factors};
use crate::output_syntax::{colored_begin_col, generate_cell_begin, OutputMode};
use crate::table_preparation::{PreparedCell, PreparedTable};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BreakoutReason {
    EmptyTable,
    WidthBoundary,
    SuppressedByMode,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableOutputConfig {
    pub mode: OutputMode,
    pub one_table: bool,
    pub color: bool,
    pub numbering: bool,
    pub textheight: usize,
    pub textwidth: usize,
    pub breiten: Vec<usize>,
    pub shell_rows_amount: usize,
    pub keine_ueberschriften: bool,
    pub keine_leeren_inhalte: bool,
    pub nichts_output_yes: bool,
}

impl Default for TableOutputConfig {
    fn default() -> Self {
        Self {
            mode: OutputMode::Shell,
            one_table: false,
            color: true,
            numbering: true,
            textheight: 0,
            textwidth: 21,
            breiten: Vec::new(),
            shell_rows_amount: 0,
            keine_ueberschriften: false,
            keine_leeren_inhalte: false,
            nichts_output_yes: false,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableRenderResult {
    pub resulting_table: Vec<String>,
    pub max_cell_text_len: BTreeMap<usize, usize>,
    pub finally_display_lines: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableOutputBundle;

impl TableOutputBundle {
    pub fn only_that_columns<T: Clone>(
        &self,
        table: &[Vec<T>],
        only_that_columns: &[usize],
    ) -> Vec<Vec<T>> {
        only_that_columns_fn(table, only_that_columns)
    }

    pub fn max_cell_text_len(
        &self,
        table: &PreparedTable,
        rows_range: &[usize],
    ) -> BTreeMap<usize, usize> {
        max_cell_text_len(table, rows_range)
    }

    pub fn determine_row_width(
        &self,
        index: usize,
        max_cell_text_len: &BTreeMap<usize, usize>,
        config: &TableOutputConfig,
    ) -> usize {
        determine_row_width(index, max_cell_text_len, config)
    }

    pub fn colorize(&self, text: &str, num: i64, rest: bool) -> String {
        colorize(text, num, rest)
    }

    pub fn render_prepared_table(
        &self,
        finally_display_lines_set: &BTreeSet<usize>,
        new_table: &PreparedTable,
        numlen: usize,
        rows_range: &[usize],
        config: &TableOutputConfig,
    ) -> TableRenderResult {
        render_prepared_table(
            finally_display_lines_set,
            new_table,
            numlen,
            rows_range,
            config,
        )
    }

    pub fn snapshot(&self) -> TableOutputBundleSnapshot {
        TableOutputBundleSnapshot {
            class: "TableOutputBundle".to_string(),
            output_class: "TableOutput".to_string(),
            responsibility: "table-output-rendering-morphism".to_string(),
            legacy_nested_class: "Tables.Output".to_string(),
            morphisms: vec![
                "only_that_columns".to_string(),
                "cliOut/render_prepared_table".to_string(),
                "findMaxCellTextLen".to_string(),
                "determineRowWidth".to_string(),
                "cliout2".to_string(),
                "colorize".to_string(),
            ],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableOutputBundleSnapshot {
    pub class: String,
    pub output_class: String,
    pub responsibility: String,
    pub legacy_nested_class: String,
    pub morphisms: Vec<String>,
}

pub fn bootstrap_table_output() -> TableOutputBundle {
    TableOutputBundle
}

pub fn only_that_columns_fn<T: Clone>(
    table: &[Vec<T>],
    only_that_columns: &[usize],
) -> Vec<Vec<T>> {
    if only_that_columns.is_empty() {
        return table.to_vec();
    }
    let mut new_table = Vec::new();
    for row in table {
        let mut new_col = Vec::new();
        for column in only_that_columns {
            if *column == 0 {
                continue;
            }
            if let Some(value) = row.get(column - 1) {
                new_col.push(value.clone());
            }
        }
        new_table.push(new_col);
    }
    if new_table.is_empty() {
        table.to_vec()
    } else {
        new_table
    }
}

pub fn max_cell_text_len(table: &PreparedTable, rows_range: &[usize]) -> BTreeMap<usize, usize> {
    let mut max_cell_text_len: BTreeMap<usize, usize> = BTreeMap::new();
    for row in table {
        for (cell_index, cell) in row.iter().enumerate() {
            let indices = if rows_range.is_empty() {
                (0..cell.len()).collect::<Vec<_>>()
            } else {
                rows_range.to_vec()
            };
            for line_index in indices {
                if let Some(text) = cell.get(line_index) {
                    let len = text.chars().count();
                    max_cell_text_len
                        .entry(cell_index)
                        .and_modify(|old| *old = (*old).max(len))
                        .or_insert(len);
                }
            }
        }
    }
    max_cell_text_len
}

pub fn determine_row_width(
    index: usize,
    max_cell_text_len: &BTreeMap<usize, usize>,
    config: &TableOutputConfig,
) -> usize {
    let certaintextwidth = config
        .breiten
        .get(index)
        .copied()
        .unwrap_or(config.textwidth);
    let max_len = max_cell_text_len.get(&index).copied().unwrap_or(0);
    if certaintextwidth > max_len
        || (certaintextwidth == 0 && !matches!(config.mode, OutputMode::Bbcode | OutputMode::Html))
    {
        max_len
    } else {
        certaintextwidth
    }
}

pub fn colorize(text: &str, num: i64, rest: bool) -> String {
    if num == 0 {
        return format!("\u{1b}[41m\u{1b}[30m\u{1b}[4m{text}\u{1b}[0m");
    }
    if rest {
        return if num % 2 == 0 {
            format!("\u{1b}[47m\u{1b}[30m{text}\u{1b}[0m\u{1b}[0m")
        } else {
            format!("\u{1b}[40m\u{1b}[37m{text}\u{1b}[0m\u{1b}[0m")
        };
    }
    if !moon_number(num).1.is_empty() {
        return if num % 2 == 0 {
            format!("\u{1b}[106m\u{1b}[30m{text}\u{1b}[0m\u{1b}[0m")
        } else {
            format!("\u{1b}[46m\u{1b}[30m{text}\u{1b}[0m\u{1b}[0m")
        };
    }
    if prime_factors(num).len() == 1 {
        return if num % 2 == 0 {
            format!("\u{1b}[103m\u{1b}[30m\u{1b}[1m{text}\u{1b}[0m")
        } else {
            format!("\u{1b}[43m\u{1b}[30m{text}\u{1b}[0m\u{1b}[0m")
        };
    }
    if num % 2 == 0 {
        format!("\u{1b}[47m\u{1b}[30m{text}\u{1b}[0m\u{1b}[0m")
    } else {
        format!("\u{1b}[100m\u{1b}[37m{text}\u{1b}[0m\u{1b}[0m")
    }
}

pub fn render_prepared_table(
    finally_display_lines_set: &BTreeSet<usize>,
    new_table: &PreparedTable,
    numlen: usize,
    rows_range: &[usize],
    config: &TableOutputConfig,
) -> TableRenderResult {
    if finally_display_lines_set.is_empty()
        || (finally_display_lines_set.len() == 1 && finally_display_lines_set.contains(&0))
        || config.nichts_output_yes
    {
        return TableRenderResult::default();
    }

    let max_len = max_cell_text_len(new_table, rows_range);
    let mut finally_display_lines = finally_display_lines_set
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    finally_display_lines.sort_by(|left, right| {
        left.parse::<usize>()
            .unwrap_or(usize::MAX)
            .cmp(&right.parse::<usize>().unwrap_or(usize::MAX))
    });
    if let Some(first) = finally_display_lines.first_mut() {
        *first = String::new();
    }

    let mut resulting_table = Vec::new();
    if matches!(config.mode, OutputMode::Html | OutputMode::Bbcode) {
        resulting_table.push(config.mode.syntax_markup().begin_table);
    }

    for (big_line_number, row) in new_table.iter().enumerate() {
        if big_line_number == 0 && config.keine_ueberschriften {
            continue;
        }
        let original_line_number = finally_display_lines
            .get(big_line_number)
            .cloned()
            .unwrap_or_default();
        let numeric_line = original_line_number.parse::<i64>().unwrap_or(0);
        let subline_count = row.iter().map(PreparedCell::len).max().unwrap_or(0);
        let max_subline = if config.textheight == 0 {
            subline_count
        } else {
            subline_count.min(config.textheight)
        };
        for subline_index in 0..max_subline {
            let mut cells = Vec::new();
            if config.numbering {
                cells.push(numbering_zaehlung_cell(config.mode, numeric_line));
                cells.push(numbering_line_cell(
                    config.mode,
                    &original_line_number,
                    subline_index,
                    numlen,
                ));
            }
            let mut empty_entries = 0usize;
            for (cell_index, cell) in row.iter().enumerate() {
                let width = determine_row_width(cell_index, &max_len, config);
                let entry = cell
                    .get(subline_index)
                    .cloned()
                    .unwrap_or_default()
                    .replace('\n', "");
                if entry.trim().is_empty()
                    || (config.keine_leeren_inhalte && entry.trim().chars().count() < 2)
                {
                    empty_entries += 1;
                }
                let padded = pad_right(&entry, width);
                cells.push(render_cell(
                    config.mode,
                    cell_index as i64,
                    &padded,
                    numeric_line,
                    config.color,
                ));
            }
            if empty_entries == row.len() && config.keine_leeren_inhalte {
                continue;
            }
            resulting_table.push(render_row(config.mode, &cells, numeric_line));
            if matches!(config.mode, OutputMode::Markdown)
                && big_line_number == 0
                && subline_index == 0
            {
                resulting_table.push(markdown_separator(cells.len()));
            }
            if matches!(config.mode, OutputMode::Emacs)
                && (big_line_number == 0 || is_prime_power_line(numeric_line))
            {
                resulting_table.push(emacs_separator(cells.len()));
            }
        }
    }

    if matches!(config.mode, OutputMode::Html | OutputMode::Bbcode) {
        resulting_table.push(config.mode.syntax_markup().end_table);
    }

    TableRenderResult {
        resulting_table,
        max_cell_text_len: max_len,
        finally_display_lines,
    }
}

fn numbering_zaehlung_cell(mode: OutputMode, numeric_line: i64) -> String {
    let marker = if numeric_line > 0 && numeric_line % 2 == 0 {
        "█"
    } else {
        " "
    };
    render_cell(mode, -2, marker, numeric_line, false)
}

fn numbering_line_cell(
    mode: OutputMode,
    original_line_number: &str,
    subline_index: usize,
    numlen: usize,
) -> String {
    let mut visible = if subline_index == 0 {
        format!("{} ", original_line_number)
    } else {
        String::new()
    };
    while visible.chars().count() < numlen + 1 {
        visible.insert(0, ' ');
    }
    render_cell(
        mode,
        -1,
        &visible,
        original_line_number.parse::<i64>().unwrap_or(0),
        false,
    )
}

fn render_cell(mode: OutputMode, index: i64, text: &str, line: i64, color: bool) -> String {
    match mode {
        OutputMode::Csv => text.to_string(),
        OutputMode::Shell if color => colorize(text, line, false),
        OutputMode::Shell | OutputMode::Nichts => text.to_string(),
        _ => format!(
            "{}{}{}",
            generate_cell_begin(mode, index, None, Some(line), &[]),
            text,
            mode.syntax_markup().end_cell
        ),
    }
}

fn render_row(mode: OutputMode, cells: &[String], numeric_line: i64) -> String {
    match mode {
        OutputMode::Csv => csv_join_semicolon(cells),
        OutputMode::Shell | OutputMode::Nichts => cells.join(" "),
        OutputMode::Markdown | OutputMode::Emacs => {
            format!("{}{}", cells.join(""), mode.syntax_markup().end_row)
        }
        OutputMode::Html | OutputMode::Bbcode => format!(
            "{}{}{}",
            colored_begin_col(mode, numeric_line, false),
            cells.join(" "),
            mode.syntax_markup().end_row
        ),
    }
}

fn csv_join_semicolon(cells: &[String]) -> String {
    let escaped = cells.iter().map(|cell| {
        if cell.contains(';') || cell.contains('"') || cell.contains('\n') {
            format!("\"{}\"", cell.replace('"', "\"\""))
        } else {
            cell.clone()
        }
    });
    format!("{}\n", escaped.collect::<Vec<_>>().join(";"))
}

fn markdown_separator(columns: usize) -> String {
    format!("{}|", "|:--:".repeat(columns))
}

fn emacs_separator(columns: usize) -> String {
    if columns == 0 {
        return "|".to_string();
    }
    format!("|----{}|", "+----".repeat(columns.saturating_sub(1)))
}

fn is_prime_power_line(value: i64) -> bool {
    let factors = prime_factors(value);
    !factors.is_empty()
        && factors.iter().copied().collect::<BTreeSet<_>>().len() == 1
        && factors.len() != 1
}

fn pad_right(text: &str, width: usize) -> String {
    let mut out = text.to_string();
    while out.chars().count() < width {
        out.push(' ');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_that_columns_uses_python_one_based_indices() {
        let table = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            only_that_columns_fn(&table, &[1, 3]),
            vec![vec![1, 3], vec![4, 6]]
        );
    }

    #[test]
    fn max_cell_width_reads_subcells() {
        let table = vec![vec![vec!["a".to_string(), "abcd".to_string()]]];
        let widths = max_cell_text_len(&table, &[0, 1]);
        assert_eq!(widths[&0], 4);
    }

    #[test]
    fn shell_renderer_produces_lines() {
        let table = vec![
            vec![vec!["head".to_string()]],
            vec![vec!["value".to_string()]],
        ];
        let result = render_prepared_table(
            &BTreeSet::from([0usize, 1usize]),
            &table,
            1,
            &[0],
            &TableOutputConfig {
                color: false,
                shell_rows_amount: 80,
                ..TableOutputConfig::default()
            },
        );
        assert!(!result.resulting_table.is_empty());
        assert!(result
            .resulting_table
            .iter()
            .any(|line| line.contains("value")));
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "BreakoutException",
    "__init__",
    "breitenn",
    "create",
    "nummeriere",
    "outType",
    "textHeight",
    "textWidth",
    "TableOutput",
    "cliOut",
    "cliout2",
    "color",
    "findMaxCellTextLen",
    "oneTable",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
