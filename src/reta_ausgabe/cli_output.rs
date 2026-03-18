// src/reta_ausgabe/cli_output.rs
use std::collections::BTreeSet;

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
        CliOutput {
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

    pub fn colorize(&self, text: &str, line_num: i32, is_empty: bool) -> String {
        if !self.color_enabled {
            return text.to_string();
        }

        match self.out_type {
            OutputSyntax::Plain => {
                if line_num == 0 {
                    text.red().on_white().bold().to_string()
                } else if is_empty {
                    if line_num % 2 == 0 {
                        text.black().on_white().to_string()
                    } else {
                        text.white().on_black().to_string()
                    }
                } else if line_num % 2 == 0 {
                    text.black().on_white().to_string()
                } else {
                    text.white().on_black().to_string()
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
        self.column_widths
            .get(col_idx)
            .copied()
            .unwrap_or(fallback)
    }

    fn wrapped_cell_lines(&self, cell: &TableCell, width: usize) -> Vec<String> {
        word_wrap(&cell.original_content, width)
    }

    fn row_wrapped_lines(
        &self,
        row: &TableRow,
        visible_col_indices: &[usize],
    ) -> Vec<Vec<String>> {
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
        }

        let mut display_lines_list: Vec<usize> = finally_display_lines.iter().copied().collect();
        display_lines_list.sort();

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
            let row_height = wrapped_columns
                .iter()
                .map(|lines| lines.len())
                .max()
                .unwrap_or(1);

            for subline_idx in 0..row_height {
                let mut line_parts = Vec::new();

                if self.line_numbering {
                    let num_str = if row.original_line_num > 0 && subline_idx == 0 {
                        format!("{:4} ", row.original_line_num)
                    } else {
                        "     ".to_string()
                    };

                    let colored_num = self.colorize(&num_str, row.original_line_num, false);
                    line_parts.push(colored_num);
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
                        let csv_line = line_parts.join(";");
                        self.cliout2(&csv_line);
                    }
                    OutputSyntax::Markdown => {
                        let mut md_line = String::new();
                        if self.line_numbering {
                            md_line.push_str("| ");
                        }
                        md_line.push_str(&line_parts.join(" | "));
                        md_line.push_str(" |");
                        self.cliout2(&md_line);

                        if display_line_idx == 0 && subline_idx == 0 {
                            let separator = if self.line_numbering {
                                "|:---".repeat(visible_cols.len() + 1) + "|"
                            } else {
                                "|:---".repeat(visible_cols.len()) + "|"
                            };
                            self.cliout2(&separator);
                        }
                    }
                    _ => {
                        let mut full_line = String::new();
                        let colored_begin = self.out_type.colored_begin_col(row.original_line_num);
                        if !colored_begin.is_empty() {
                            full_line.push_str(colored_begin);
                        }

                        for part in &line_parts {
                            full_line.push_str(part);
                        }

                        full_line.push_str(self.out_type.end_zeile());
                        self.cliout2(&full_line);
                    }
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
    ) -> std::collections::HashMap<usize, usize> {
        let mut max_cell_widths = std::collections::HashMap::new();

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

    pub fn create_test_table(&self) -> Vec<TableRow> {
        let col_widths = vec![20, 15, 25];

        let header_cell1 = TableCell::new("Name und Vorname".to_string(), col_widths[0]);
        let header_cell2 = TableCell::new("Alter".to_string(), col_widths[1]);
        let header_cell3 =
            TableCell::new("Wohnort und Beschreibung".to_string(), col_widths[2]);

        let data_cell1_1 = TableCell::new("Hans Mustermann 😊".to_string(), col_widths[0]);
        let data_cell1_2 = TableCell::new("25 Jahre 🎂".to_string(), col_widths[1]);
        let data_cell1_3 = TableCell::new(
            "Berlin 🇩🇪, Hauptstadt von Deutschland, sehr schöne Stadt mit vielen Sehenswürdigkeiten 🏛️"
                .to_string(),
            col_widths[2],
        );

        let data_cell2_1 = TableCell::new("Anna Schmidt 👩".to_string(), col_widths[0]);
        let data_cell2_2 = TableCell::new("30".to_string(), col_widths[1]);
        let data_cell2_3 = TableCell::new(
            "München in Bayern, bekannt für das Oktoberfest 🍺 und die schönen Parks 🌳"
                .to_string(),
            col_widths[2],
        );

        let data_cell3_1 = TableCell::new("Peter-Ludwig Meyer 👨‍💼".to_string(), col_widths[0]);
        let data_cell3_2 = TableCell::new("22 Jahre alt ⭐".to_string(), col_widths[1]);
        let data_cell3_3 = TableCell::new(
            "Hamburg 🚢, Hafenstadt, geboren am 15. März 2000 📅, wohnt dort seit Geburt"
                .to_string(),
            col_widths[2],
        );

        let data_cell4_1 =
            TableCell::new("Emoji-Test: 😀😃😄😁😆😅😂🤣🥲".to_string(), col_widths[0]);
        let data_cell4_2 = TableCell::new("".to_string(), col_widths[1]);
        let data_cell4_3 = TableCell::new(
            "Chinesisch: 你好世界 🌍 Japanisch: こんにちは世界 🇯🇵 Koreanisch: 안녕하세요 세상 🇰🇷"
                .to_string(),
            col_widths[2],
        );

        let data_cell5_1 = TableCell::new(
            "𝕋𝕖𝕤𝕥 𝕨𝕚𝕥𝕙 𝔻𝕠𝕦𝕓𝕝𝕖-𝕊𝕥𝕣𝕦𝕔𝕜".to_string(),
            col_widths[0],
        );
        let data_cell5_2 = TableCell::new("𝒮𝒸𝓇𝒾𝓅𝓉 𝒯ℯ𝓍𝓉".to_string(), col_widths[1]);
        let data_cell5_3 = TableCell::new(
            "Mathematik: ∀x∈ℝ, ∃y∈ℚ: x² + y² = z² ∫₀¹ f(x) dx".to_string(),
            col_widths[2],
        );

        vec![
            TableRow::new(vec![header_cell1, header_cell2, header_cell3], 0, 0),
            TableRow::new(vec![data_cell1_1, data_cell1_2, data_cell1_3], 1, 1),
            TableRow::new(vec![data_cell2_1, data_cell2_2, data_cell2_3], 2, 2),
            TableRow::new(vec![data_cell3_1, data_cell3_2, data_cell3_3], 3, 3),
            TableRow::new(vec![data_cell4_1, data_cell4_2, data_cell4_3], 4, 4),
            TableRow::new(vec![data_cell5_1, data_cell5_2, data_cell5_3], 5, 5),
        ]
    }

    pub fn create_simple_table(&self) -> Vec<TableRow> {
        let header_cell1 = TableCell::new("Name".to_string(), 15);
        let header_cell2 = TableCell::new("Alter".to_string(), 10);
        let header_cell3 = TableCell::new("Stadt".to_string(), 20);

        let data_cell1_1 = TableCell::new("Hans".to_string(), 15);
        let data_cell1_2 = TableCell::new("25".to_string(), 10);
        let data_cell1_3 = TableCell::new("Berlin 🇩🇪".to_string(), 20);

        let data_cell2_1 = TableCell::new("Anna 👩".to_string(), 15);
        let data_cell2_2 = TableCell::new("30".to_string(), 10);
        let data_cell2_3 = TableCell::new("München 🏙️".to_string(), 20);

        let data_cell3_1 = TableCell::new("Peter 👨".to_string(), 15);
        let data_cell3_2 = TableCell::new("22".to_string(), 10);
        let data_cell3_3 = TableCell::new("Hamburg ⚓\n(geboren)".to_string(), 20);

        let data_cell4_1 = TableCell::new("Familie: 👨‍👩‍👧‍👦".to_string(), 15);
        let data_cell4_2 = TableCell::new("28".to_string(), 10);
        let data_cell4_3 = TableCell::new("Köln 🏛️ mit Kathedrale".to_string(), 20);

        vec![
            TableRow::new(vec![header_cell1, header_cell2, header_cell3], 0, 0),
            TableRow::new(vec![data_cell1_1, data_cell1_2, data_cell1_3], 1, 1),
            TableRow::new(vec![data_cell2_1, data_cell2_2, data_cell2_3], 2, 2),
            TableRow::new(vec![data_cell3_1, data_cell3_2, data_cell3_3], 3, 3),
            TableRow::new(vec![data_cell4_1, data_cell4_2, data_cell4_3], 4, 4),
        ]
    }
}
