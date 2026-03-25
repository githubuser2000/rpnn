use std::collections::{BTreeSet, HashMap};

use colored::*;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use crate::column_categories_complete::KategorieMap;
use crate::reta_ausgabe::output_syntax::OutputSyntax;
use crate::reta_ausgabe::table_cell::{TableCell, TableRow};
use crate::reta_ausgabe::tables::Tables;
use crate::reta_ausgabe::utils::{unicode_pad, word_wrap};

#[derive(Debug)]
pub struct CliOutput<'a> {
    pub out_type: OutputSyntax,
    pub color_enabled: bool,
    pub pretty_output: bool,
    pub one_table: bool,
    pub table_width: usize,
    pub column_widths: Vec<usize>,
    pub line_numbering: bool,
    pub resulting_output: Vec<String>,
    pub tables_ref: &'a Tables,
}

impl<'a> CliOutput<'a> {
    pub fn new(tables: &'a Tables, out_type: OutputSyntax) -> Self {
        CliOutput {
            out_type,
            color_enabled: true,
            pretty_output: false,
            one_table: false,
            table_width: 80,
            column_widths: Vec::new(),
            line_numbering: true,
            resulting_output: Vec::new(),
            tables_ref: tables,
        }
    }

    fn is_perfect_power(n: i32) -> bool {
        if n < 4 {
            return false;
        }
        let n64 = n as i64;
        let max_exp = 31 - (n as u32).leading_zeros();
        for exp in 2..=max_exp {
            let base = (n as f64).powf(1.0 / exp as f64).round() as i64;
            if base > 1 && base.pow(exp) == n64 {
                return true;
            }
        }
        false
    }

    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let mut d = 3;
        while d * d <= n {
            if n % d == 0 {
                return false;
            }
            d += 2;
        }
        true
    }

    pub fn colorize(&self, text: &str, line_num: i32, is_empty: bool) -> String {
        if !self.color_enabled {
            return text.to_string();
        }
        match self.out_type {
            OutputSyntax::Plain => {
                if line_num == 0 {
                    return text.red().on_white().bold().to_string();
                }
                if is_empty {
                    return if line_num % 2 == 0 {
                        text.black().on_white().to_string()
                    } else {
                        text.white().on_black().to_string()
                    };
                }
                if Self::is_perfect_power(line_num) {
                    return if line_num % 2 == 0 {
                        text.black().on_cyan().to_string()
                    } else {
                        text.black().on_bright_cyan().to_string()
                    };
                }
                if Self::is_prime(line_num) {
                    return if line_num % 2 == 0 {
                        text.black().on_yellow().bold().to_string()
                    } else {
                        text.black().on_bright_yellow().to_string()
                    };
                }
                if line_num % 2 == 0 {
                    text.black().on_white().to_string()
                } else {
                    text.white().on_bright_black().to_string()
                }
            }
            _ => text.to_string(),
        }
    }


    fn syntect_extension(&self) -> Option<&'static str> {
        match self.out_type {
            OutputSyntax::HTML => Some("html"),
            OutputSyntax::BBCode => Some("xml"),
            OutputSyntax::CSV => Some("csv"),
            OutputSyntax::Markdown | OutputSyntax::Emacs => Some("md"),
            _ => None,
        }
    }

    fn pretty_with_syntect(&self, text: &str) -> Option<String> {
        let ext = self.syntect_extension()?;
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = ts
            .themes
            .get("base16-ocean.dark")
            .or_else(|| ts.themes.values().next())?;
        let syntax = ps
            .find_syntax_by_extension(ext)
            .unwrap_or_else(|| ps.find_syntax_plain_text());
        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut out = String::new();
        for line in LinesWithEndings::from(text) {
            if let Ok(ranges) = highlighter.highlight_line(line, &ps) {
                out.push_str(&as_24_bit_terminal_escaped(&ranges, false));
            } else {
                out.push_str(line);
            }
        }
        Some(out)
    }

    fn pretty_text(&self, text: &str) -> String {
        self.pretty_with_syntect(text).unwrap_or_else(|| text.to_string())
    }

    pub fn cliout2(&mut self, text: &str) {
        self.resulting_output.push(text.to_string());
        if !matches!(self.out_type, OutputSyntax::Nichts) {
            if self.pretty_output && !matches!(self.out_type, OutputSyntax::Plain) {
                print!("{}", self.pretty_text(text));
            } else {
                print!("{}", text);
            }
        }
    }

    fn effective_width_for_col(&self, col_idx: usize, fallback: usize) -> usize {
        self.column_widths.get(col_idx).copied().unwrap_or(fallback)
    }

    fn wrapped_cell_lines(&self, cell: &TableCell, width: usize) -> Vec<String> {
        if matches!(self.out_type, OutputSyntax::Plain) {
            word_wrap(&cell.original_content, width)
        } else {
            vec![cell.original_content.clone()]
        }
    }

    fn row_wrapped_lines(&self, row: &TableRow, visible_col_indices: &[usize]) -> Vec<Vec<String>> {
        visible_col_indices
            .iter()
            .map(|&col_idx| {
                let width = self.effective_width_for_col(col_idx, self.table_width);
                if let Some(cell) = row.cells.get(col_idx) {
                    self.wrapped_cell_lines(cell, width)
                } else {
                    vec![String::new()]
                }
            })
            .collect()
    }

    fn visible_columns_for_row(&self, row: &TableRow) -> Vec<usize> {
        let numbering_width = if self.line_numbering { 5 } else { 0 };
        let mut total_width = numbering_width;
        let mut visible = Vec::new();
        for col_idx in 0..row.cells.len() {
            let width = self.effective_width_for_col(col_idx, self.table_width);
            let additional = width + 1;
            if self.one_table || total_width + additional < self.table_width {
                visible.push(col_idx);
                total_width += additional;
            } else if visible.is_empty() {
                visible.push(col_idx);
                break;
            } else {
                break;
            }
        }
        visible
    }

    fn strip_hidden_idx<'b>(&self, s: &'b str) -> (&'b str, Option<usize>) {
        if let Some((left, right)) = s.split_once('\u{1f}') {
            if let Some(rest) = right.strip_prefix("IDX:") {
                return (left, rest.parse::<usize>().ok());
            }
        }
        (s, None)
    }

    fn escape_html(content: &str) -> String {
        content
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    fn strip_trailing_id_suffix(text: &str) -> &str {
        let trimmed = text.trim_end();
        if let Some(pos) = trimmed.rfind(" (ID_") {
            if trimmed.ends_with(')') {
                return trimmed[..pos].trim_end();
            }
        }
        trimmed
    }

    fn normalize_meta_label(label: &str, ober: &str) -> String {
        let mut out = label.replace(' ', "_");
        out = out.replace(",", "");
        out = out.replace('/', "_");
        out = out.replace('→', "_");
        out = out.replace("(", "_(");
        while out.contains("__") {
            out = out.replace("__", "_");
        }
        if ober == "Universum" && (out == "Geist_(15)" || out == "Geist(15)") {
            return "Geist__(15)".to_string();
        }
        out
    }

    pub fn header_meta_class(global_idx: usize) -> Option<String> {
        let map = KategorieMap::new();
        let matches: Vec<_> = map
            .alle_eintraege
            .iter()
            .filter(|e| e.spaltennummern.iter().any(|&n| n > 0 && (n as usize).saturating_sub(1) == global_idx))
            .collect();
        if matches.is_empty() {
            return None;
        }

        let mut p1 = String::new();
        let mut labels = Vec::new();
        for e in &matches {
            p1.push('✗');
            p1.push_str(&e.oberkategorie);
            p1.push(',');
            labels.push(Self::normalize_meta_label(&e.unterkategorie, &e.oberkategorie));
        }
        while labels.len() < matches.len() + 3 {
            labels.push(String::new());
        }
        let p2 = labels
            .iter()
            .enumerate()
            .map(|(i, s)| format!("p3_{}_{}", i, s))
            .collect::<Vec<_>>()
            .join(",")
            + ",";

        let p4 = if labels.iter().any(|s| s.contains("Geist") || s.contains("nachvollziehen")) {
            if matches.iter().any(|e| e.unterkategorie.contains("nachvollziehen")) && matches.len() >= 4 {
                if global_idx + 1 == 243 || global_idx + 1 == 427 { "4,0" } else { "0,4" }
            } else if matches.len() >= 3 {
                if global_idx + 1 == 427 { "4,0" } else { "0,4" }
            } else {
                ""
            }
        } else {
            ""
        };

        Some(format!("p1_{},, p2_{} p4_{}", p1, p2, p4))
    }

    fn html_row_style(line_num: i32) -> &'static str {
        match line_num {
            0 => "background-color:#ff2222;color:#002222;",
            1 => "background-color:#555500;color:#aaaaff;",
            2 => "background-color:#66ff66;color:#000000;",
            3 => "background-color:#009900;color:#ffffff;",
            _ => "background-color:#555500;color:#aaaaff;",
        }
    }

    fn render_html_table(&mut self, display_lines_list: &[usize], table: &[TableRow]) {
        self.cliout2(self.out_type.begin_table());
        for &display_line_idx in display_lines_list {
            let Some(row) = table.get(display_line_idx) else { continue; };
            if display_line_idx == 0 && self.tables_ref.keine_ueberschriften {
                continue;
            }
            let visible_cols = self.visible_columns_for_row(row);
            if visible_cols.is_empty() {
                continue;
            }
            let mut cells = Vec::new();
            for (visible_pos, &col_idx) in visible_cols.iter().enumerate() {
                let raw = row.cells.get(col_idx).map(|c| c.original_content.as_str()).unwrap_or("");
                let (display, hidden_idx) = self.strip_hidden_idx(raw);
                let display = if display_line_idx == 0 && visible_pos >= 2 { Self::strip_trailing_id_suffix(display) } else { display };
                let escaped = Self::escape_html(display);
                if display_line_idx == 0 {
                    if visible_pos == 0 {
                        cells.push("<td class=\"z_0 r_0 p1_✗Zählung,, p2_p3_0_, p4_\" style=\"background-color:#ffffff;color:#000000;\"> </td>".to_string());
                    } else if visible_pos == 1 {
                        cells.push("<td class=\"z_0 r_1 p1_✗Nummerierung,, p2_p3_0_, p4_\"> </td>".to_string());
                    } else {
                        let extra = hidden_idx.and_then(Self::header_meta_class);
                        let class_attr = if let Some(extra) = extra {
                            format!(" class=\"z_0 r_{} {}\"", visible_pos, extra)
                        } else {
                            format!(" class=\"z_0 r_{}\"", visible_pos)
                        };
                        cells.push(format!("<td{}> {} </td>", class_attr, escaped));
                    }
                } else if visible_pos == 0 {
                    cells.push(format!("<td style=\"background-color:#ffffff;color:#000000;\"> {} </td>", escaped));
                } else {
                    cells.push(format!("<td> {} </td>", escaped));
                }
            }
            let row_html = format!("<tr style=\"{}\"> {} </tr>\n", Self::html_row_style(row.original_line_num), cells.join(" "));
            self.cliout2(&row_html);
        }
        self.cliout2(self.out_type.end_table());
    }

    pub fn cli_out(
        &mut self,
        finally_display_lines: &BTreeSet<usize>,
        table: &[TableRow],
        _rows_range: std::ops::Range<usize>,
    ) -> Vec<String> {
        if finally_display_lines.is_empty() {
            return Vec::new();
        }

        let mut display_lines_list: Vec<usize> = finally_display_lines.iter().copied().collect();
        display_lines_list.sort();

        if matches!(self.out_type, OutputSyntax::HTML) {
            self.render_html_table(&display_lines_list, table);
            return self.resulting_output.clone();
        }

        if matches!(self.out_type, OutputSyntax::BBCode) {
            self.cliout2(self.out_type.begin_table());
        }

        for &display_line_idx in &display_lines_list {
            let Some(row) = table.get(display_line_idx) else { continue; };
            if display_line_idx == 0 && self.tables_ref.keine_ueberschriften {
                continue;
            }
            let visible_cols = self.visible_columns_for_row(row);
            if visible_cols.is_empty() {
                continue;
            }
            let wrapped_columns = self.row_wrapped_lines(row, &visible_cols);
            let row_height = wrapped_columns.iter().map(|lines| lines.len()).max().unwrap_or(1);

            for subline_idx in 0..row_height {
                let mut line_parts = Vec::new();
                if self.line_numbering {
                    let num_str = if row.original_line_num > 0 && subline_idx == 0 {
                        format!("{:4} ", row.original_line_num)
                    } else {
                        "     ".to_string()
                    };
                    line_parts.push(self.colorize(&num_str, row.original_line_num, false));
                }
                let mut entries_in_row = 0usize;
                let mut empty_entries = 0usize;
                for (visible_pos, &col_idx) in visible_cols.iter().enumerate() {
                    let width = self.effective_width_for_col(col_idx, self.table_width);
                    let content = wrapped_columns
                        .get(visible_pos)
                        .and_then(|lines| lines.get(subline_idx))
                        .map(|s| s.as_str())
                        .unwrap_or("");
                    entries_in_row += 1;
                    if content.trim().is_empty() {
                        empty_entries += 1;
                    }
                    let formatted_content = if matches!(self.out_type, OutputSyntax::CSV) {
                        content.to_string()
                    } else {
                        let padded = unicode_pad(content, width, true);
                        self.colorize(&padded, row.original_line_num, content.trim().is_empty())
                    };
                    if matches!(self.out_type, OutputSyntax::CSV) {
                        line_parts.push(formatted_content);
                    } else {
                        line_parts.push(formatted_content);
                        line_parts.push(" ".to_string());
                    }
                }
                if empty_entries == entries_in_row {
                    continue;
                }
                match self.out_type {
                    OutputSyntax::CSV => {
                        self.cliout2(&(line_parts.join(";") + "\n"));
                    }
                    OutputSyntax::Markdown => {
                        let mut md_line = String::new();
                        if self.line_numbering {
                            md_line.push_str("| ");
                        }
                        md_line.push_str(&line_parts.join(" | "));
                        md_line.push_str(" |\n");
                        self.cliout2(&md_line);
                        if display_line_idx == 0 && subline_idx == 0 {
                            let separator = if self.line_numbering {
                                "|:---".repeat(visible_cols.len() + 1) + "|\n"
                            } else {
                                "|:---".repeat(visible_cols.len()) + "|\n"
                            };
                            self.cliout2(&separator);
                        }
                    }
                    OutputSyntax::Emacs => {
                        let mut line = String::new();
                        line.push('|');
                        line.push_str(&line_parts.join("|"));
                        line.push_str("\n");
                        self.cliout2(&line);
                    }
                    OutputSyntax::BBCode => {
                        let mut line = String::from("[tr]");
                        for part in &line_parts {
                            line.push_str("[td]");
                            line.push_str(part);
                            line.push_str("[/td]");
                        }
                        line.push_str("[/tr]\n");
                        self.cliout2(&line);
                    }
                    OutputSyntax::Plain => {
                        let mut full_line = String::new();
                        for part in &line_parts { full_line.push_str(part); }
                        full_line.push('\n');
                        self.cliout2(&full_line);
                    }
                    OutputSyntax::Nichts => {}
                    OutputSyntax::HTML => unreachable!(),
                }
            }
        }
        if matches!(self.out_type, OutputSyntax::BBCode) {
            self.cliout2(self.out_type.end_table());
        }
        self.resulting_output.clone()
    }

    pub fn find_max_cell_text_len(
        &self,
        _display_lines: &BTreeSet<usize>,
        _table: &[TableRow],
        _rows_range: &std::ops::Range<usize>,
    ) -> HashMap<usize, usize> {
        HashMap::new()
    }
}
