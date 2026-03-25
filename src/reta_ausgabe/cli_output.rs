use std::collections::{BTreeSet, HashMap};

use colored::*;
use unicode_width::UnicodeWidthStr;

use crate::reta_ausgabe::output_syntax::OutputSyntax;
use crate::reta_ausgabe::table_cell::{TableCell, TableRow};
use crate::reta_ausgabe::tables::Tables;
use crate::reta_ausgabe::utils::{unicode_pad, word_wrap};

#[derive(Debug)]
pub struct CliOutput<'a> {
    pub out_type: OutputSyntax,
    pub color_enabled: bool,
    pub one_table: bool,
    pub table_width: usize,
    pub column_widths: Vec<usize>,
    pub line_numbering: bool,
    pub resulting_output: Vec<String>,
    pub tables_ref: &'a Tables,
}

impl<'a> CliOutput<'a> {
    pub fn new(tables: &'a Tables, out_type: OutputSyntax) -> Self {
        Self {
            out_type,
            color_enabled: true,
            one_table: false,
            table_width: 80,
            column_widths: Vec::new(),
            line_numbering: true,
            resulting_output: Vec::new(),
            tables_ref: tables,
        }
    }

    fn is_perfect_power(n: i32) -> bool {
        if n < 4 || n == 8 {
            return false;
        }

        let mut base = 2i32;
        while base.saturating_mul(base) <= n {
            let mut value = base.saturating_mul(base);
            while value < n {
                match value.checked_mul(base) {
                    Some(next) => value = next,
                    None => break,
                }
            }
            if value == n {
                return true;
            }
            base += 1;
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

    fn prim_creativity_type(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        if Self::is_perfect_power(n) {
            1
        } else if Self::is_prime(n) || n == 1 {
            2
        } else {
            3
        }
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

    pub fn cliout2(&mut self, text: &str) {
        self.resulting_output.push(text.to_string());
        if !matches!(self.out_type, OutputSyntax::Nichts) {
            print!("{}", text);
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
        if !matches!(self.out_type, OutputSyntax::Plain) {
            return (0..row.cells.len()).collect();
        }

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

    fn escape_html(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    fn escape_markdown(text: &str) -> String {
        text.replace('|', r"\|")
    }

    fn escape_csv(text: &str) -> String {
        let needs_quotes = text.contains(';') || text.contains('\n') || text.contains('"');
        if needs_quotes {
            format!("\"{}\"", text.replace('"', "\"\""))
        } else {
            text.to_string()
        }
    }

    fn line_number_title(&self) -> &'static str {
        "Z"
    }

    fn html_row_style(line_num: i32) -> &'static str {
        let n = line_num.max(0);
        let number_type = Self::prim_creativity_type(n);

        if n == 0 {
            "background-color:#ff2222;color:#002222;"
        } else if number_type == 1 {
            if n % 2 == 0 {
                "background-color:#66ff66;color:#000000;"
            } else {
                "background-color:#009900;color:#ffffff;"
            }
        } else if number_type == 2 || n == 1 {
            if n % 2 == 0 {
                "background-color:#ffff66;color:#000099;"
            } else {
                "background-color:#555500;color:#aaaaff;"
            }
        } else if n % 2 == 0 {
            "background-color:#9999ff;color:#202000;"
        } else {
            "background-color:#000099;color:#ffff66;"
        }
    }

    fn html_cell_attrs(row_line_num: i32, col_idx: usize, content: &str) -> String {
        if row_line_num == 0 {
            return match col_idx {
                0 => " class=\"z_0 r_0 p1_✗Zählung,, p2_p3_0_, p4_\" style=\"background-color:#ffffff;color:#000000;\"".to_string(),
                1 => " class=\"z_0 r_1 p1_✗Nummerierung,, p2_p3_0_, p4_\"".to_string(),
                _ => format!(" class=\"z_0 r_{}\"", col_idx),
            };
        }

        if col_idx == 0 || col_idx == 1 {
            if content.parse::<i32>().ok().map(|n| n % 2 == 0).unwrap_or(false) {
                " style=\"background-color:#000000;color:#ffffff;\"".to_string()
            } else {
                " style=\"background-color:#ffffff;color:#000000;\"".to_string()
            }
        } else {
            String::new()
        }
    }

    fn html_display_cell_content(row_line_num: i32, col_idx: usize, content: &str) -> String {
        if row_line_num == 0 && (col_idx == 0 || col_idx == 1) {
            String::new()
        } else {
            Self::escape_html(content)
        }
    }

    fn render_structured_row(&self, row_line_num: i32, cells: &[String]) -> String {
        match self.out_type {
            OutputSyntax::HTML => {
                let inner = cells
                    .iter()
                    .enumerate()
                    .map(|(col_idx, cell)| {
                        let attrs = Self::html_cell_attrs(row_line_num, col_idx, cell);
                        let content = Self::html_display_cell_content(row_line_num, col_idx, cell);
                        format!(" <td{}> {} </td>", attrs, content)
                    })
                    .collect::<Vec<_>>()
                    .join("");
                format!("<tr style=\"{}\">{}</tr>\n", Self::html_row_style(row_line_num), inner)
            }
            OutputSyntax::BBCode => {
                let inner = cells
                    .iter()
                    .map(|cell| format!("[td]{}[/td]", cell))
                    .collect::<Vec<_>>()
                    .join("");
                format!("[tr]{}[/tr]\n", inner)
            }
            OutputSyntax::CSV => {
                let line = cells.iter().map(|c| Self::escape_csv(c)).collect::<Vec<_>>().join(";");
                format!("{}\n", line)
            }
            OutputSyntax::Markdown => {
                let line = cells.iter().map(|c| Self::escape_markdown(c)).collect::<Vec<_>>().join(" | ");
                format!("| {} |\n", line)
            }
            OutputSyntax::Emacs => {
                let line = cells.iter().map(|c| Self::escape_markdown(c)).collect::<Vec<_>>().join(" | ");
                format!("| {} |\n", line)
            }
            OutputSyntax::Plain | OutputSyntax::Nichts => String::new(),
        }
    }

    fn render_separator_row(&self, width: usize) -> String {
        match self.out_type {
            OutputSyntax::Markdown => {
                let parts = (0..width).map(|_| "---").collect::<Vec<_>>().join(" | ");
                format!("| {} |\n", parts)
            }
            OutputSyntax::Emacs => {
                let parts = (0..width).map(|_| "---").collect::<Vec<_>>().join("-+-");
                format!("|-{}-|\n", parts)
            }
            _ => String::new(),
        }
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

        if matches!(self.out_type, OutputSyntax::HTML | OutputSyntax::BBCode) {
            self.cliout2(self.out_type.begin_table());
            self.cliout2("\n");
        }

        let mut display_lines_list: Vec<usize> = finally_display_lines.iter().copied().collect();
        display_lines_list.sort_unstable();

        for &display_line_idx in &display_lines_list {
            let Some(row) = table.get(display_line_idx) else {
                continue;
            };

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
                let mut raw_cells = Vec::new();
                let mut plain_parts = Vec::new();

                if self.line_numbering {
                    let num_str = if display_line_idx == 0 && subline_idx == 0 {
                        self.line_number_title().to_string()
                    } else if row.original_line_num > 0 && subline_idx == 0 {
                        row.original_line_num.to_string()
                    } else {
                        String::new()
                    };

                    if matches!(self.out_type, OutputSyntax::Plain) {
                        let shown = if display_line_idx == 0 && subline_idx == 0 {
                            format!("{:>4} ", self.line_number_title())
                        } else if row.original_line_num > 0 && subline_idx == 0 {
                            format!("{:4} ", row.original_line_num)
                        } else {
                            "     ".to_string()
                        };
                        plain_parts.push(self.colorize(&shown, row.original_line_num, false));
                    } else {
                        raw_cells.push(num_str);
                    }
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

                    if matches!(self.out_type, OutputSyntax::Plain) {
                        let padded = unicode_pad(content, width, true);
                        let colored = self.colorize(&padded, row.original_line_num, content.trim().is_empty());
                        plain_parts.push(colored);
                        plain_parts.push(" ".to_string());
                    } else {
                        raw_cells.push(content.to_string());
                    }
                }

                if empty_entries == entries_in_row {
                    continue;
                }

                match self.out_type {
                    OutputSyntax::Plain => {
                        let mut full_line = String::new();
                        let colored_begin = self.out_type.colored_begin_col(row.original_line_num);
                        if !colored_begin.is_empty() {
                            full_line.push_str(colored_begin);
                        }
                        for part in &plain_parts {
                            full_line.push_str(part);
                        }
                        full_line.push_str(self.out_type.end_zeile());
                        self.cliout2(&full_line);
                    }
                    OutputSyntax::Markdown | OutputSyntax::Emacs | OutputSyntax::HTML | OutputSyntax::BBCode | OutputSyntax::CSV => {
                        let row_text = self.render_structured_row(row.original_line_num, &raw_cells);
                        self.cliout2(&row_text);

                        if display_line_idx == 0 && subline_idx == 0 {
                            let separator = self.render_separator_row(raw_cells.len());
                            if !separator.is_empty() {
                                self.cliout2(&separator);
                            }
                        }
                    }
                    OutputSyntax::Nichts => {}
                }
            }
        }

        if matches!(self.out_type, OutputSyntax::HTML | OutputSyntax::BBCode) {
            self.cliout2(self.out_type.end_table());
        }

        self.resulting_output.clone()
    }

    pub fn find_max_cell_text_len(
        &self,
        display_lines: &BTreeSet<usize>,
        table: &[TableRow],
        _rows_range: &std::ops::Range<usize>,
    ) -> HashMap<usize, usize> {
        let mut max_cell_widths = HashMap::new();

        for &line_idx in display_lines {
            if let Some(row) = table.get(line_idx) {
                for (col_idx, cell) in row.cells.iter().enumerate() {
                    let width = UnicodeWidthStr::width(cell.original_content.as_str());
                    let current_max = max_cell_widths.entry(col_idx).or_insert(0);
                    if width > *current_max {
                        *current_max = width;
                    }
                }
            }
        }

        max_cell_widths
    }
}
