use std::collections::{BTreeSet, HashMap};
use colored::*;
use unicode_width::{UnicodeWidthStr, UnicodeWidthChar};

#[derive(Debug, Clone, Copy)]
pub enum OutputSyntax {
    Plain,
    Markdown,
    BBCode,
    HTML,
    CSV,
    Emacs,
    Nichts,
}

impl OutputSyntax {
    pub fn begin_table(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "<table>",
            OutputSyntax::BBCode => "[table]",
            _ => "",
        }
    }
    
    pub fn end_table(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "</table>",
            OutputSyntax::BBCode => "[/table]",
            _ => "",
        }
    }
    
    pub fn generate_cell(self, _col_index: i32, _params: &HashMap<String, String>, _line_num: i32) -> String {
        match self {
            OutputSyntax::HTML => "<td>".to_string(),
            OutputSyntax::BBCode => "[td]".to_string(),
            OutputSyntax::Markdown => "|".to_string(),
            _ => "".to_string(),
        }
    }
    
    pub fn end_cell(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "</td>",
            OutputSyntax::BBCode => "[/td]",
            _ => "",
        }
    }
    
    pub fn colored_begin_col(self, _line_num: i32) -> &'static str {
        ""
    }
    
    pub fn end_zeile(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "</tr>",
            OutputSyntax::BBCode => "[/tr]",
            OutputSyntax::Markdown => "|",
            _ => "\n",
        }
    }
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    
    let mut result = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;
    
    for ch in text.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(1);
        
        // Wenn das Zeichen nicht in die aktuelle Zeile passt
        if current_width + ch_width > width {
            if !current_line.is_empty() {
                // Auffüllen auf volle Breite
                let padded_line = format!("{:<width$}", current_line, width = width);
                result.push(padded_line);
                current_line.clear();
                current_width = 0;
            }
            
            // Falls ein einzelnes Zeichen breiter als die ganze Breite ist
            if ch_width > width {
                // Füge es trotzdem hinzu und starte neue Zeile
                let single_char = ch.to_string();
                let padded_char = format!("{:<width$}", single_char, width = width);
                result.push(padded_char);
                continue;
            }
        }
        
        current_line.push(ch);
        current_width += ch_width;
    }
    
    // Letzte Zeile nicht vergessen
    if !current_line.is_empty() {
        let padded_line = format!("{:<width$}", current_line, width = width);
        result.push(padded_line);
    }
    
    // Falls nach der Umwandlung nichts rauskam (leerer String)
    if result.is_empty() {
        result.push(" ".repeat(width));
    }
    
    result
}

#[derive(Debug, Clone)]
pub struct TableCell {
    lines: Vec<String>,
}

impl TableCell {
    pub fn new(content: String, width: usize) -> Self {
        let lines: Vec<String> = content
            .split('\n')
            .flat_map(|line| wrap_text(line, width))
            .collect();
        
        TableCell { lines }
    }
    
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        self.lines.get(line_num).map(|s| s.as_str())
    }
    
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

#[derive(Debug, Clone)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub original_line_num: i32,
    pub display_line_num: i32,
}

impl TableRow {
    pub fn new(cells: Vec<TableCell>, original_line_num: i32, display_line_num: i32) -> Self {
        TableRow {
            cells,
            original_line_num,
            display_line_num,
        }
    }
    
    pub fn max_line_count(&self) -> usize {
        self.cells.iter()
            .map(|cell| cell.line_count())
            .max()
            .unwrap_or(0)
    }
}

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
                    format!("{}", text.red().bold())
                } else if is_empty {
                    text.to_string()
                } else {
                    text.to_string()
                }
            }
            _ => text.to_string(),
        }
    }
    
    pub fn cliout2(&mut self, text: &str) {
        println!("[DEBUG cliout2] Ausgabe: '{}'", text);
        self.resulting_output.push(text.to_string());
        
        if !matches!(self.out_type, OutputSyntax::Nichts) {
            println!("{}", text);
        } else {
            println!("[DEBUG] OutputSyntax::Nichts - keine Ausgabe");
        }
    }
    
    pub fn cli_out(
        &mut self,
        finally_display_lines: &BTreeSet<usize>,
        table: &[TableRow],
        rows_range: std::ops::Range<usize>,
    ) -> Vec<String> {
        println!("=== EINFACHE AUSGABE START ===");
        
        // Direkte Ausgabe
        for &line_idx in finally_display_lines {
            println!("Zeile {}:", line_idx);
            if let Some(row) = table.get(line_idx) {
                for line_num in rows_range.clone() {
                    let mut line = String::new();
                    for cell in &row.cells {
                        if let Some(content) = cell.get_line(line_num) {
                            line.push_str(content);
                            line.push_str(" | ");
                        }
                    }
                    if !line.trim().is_empty() {
                        self.cliout2(&line);
                    }
                }
            }
        }
        
        println!("=== EINFACHE AUSGABE ENDE ===");
        self.resulting_output.clone()
    }
    
    pub fn find_max_cell_text_len(
        &self,
        display_lines: &BTreeSet<usize>,
        table: &[TableRow],
        rows_range: &std::ops::Range<usize>,
    ) -> HashMap<usize, usize> {
        let mut max_cell_widths = HashMap::new();
        
        for &line_idx in display_lines {
            if let Some(row) = table.get(line_idx) {
                for (col_idx, cell) in row.cells.iter().enumerate() {
                    for line_num in rows_range.clone() {
                        if let Some(cell_content) = cell.get_line(line_num) {
                            let width = cell_content.len();
                            let current_max = max_cell_widths.entry(col_idx).or_insert(0);
                            if width > *current_max {
                                *current_max = width;
                            }
                        }
                    }
                }
            }
        }
        
        max_cell_widths
    }
    
    pub fn create_simple_table(&self) -> Vec<TableRow> {
        let header_cell1 = TableCell::new("Name".to_string(), 15);
        let header_cell2 = TableCell::new("Alter".to_string(), 10);
        let header_cell3 = TableCell::new("Stadt".to_string(), 20);
        
        let data_cell1_1 = TableCell::new("Hans".to_string(), 15);
        let data_cell1_2 = TableCell::new("25".to_string(), 10);
        let data_cell1_3 = TableCell::new("Berlin".to_string(), 20);
        
        let data_cell2_1 = TableCell::new("Anna".to_string(), 15);
        let data_cell2_2 = TableCell::new("30".to_string(), 10);
        let data_cell2_3 = TableCell::new("München".to_string(), 20);
        
        let data_cell3_1 = TableCell::new("Peter".to_string(), 15);
        let data_cell3_2 = TableCell::new("22".to_string(), 10);
        let data_cell3_3 = TableCell::new("Hamburg\n(geboren)".to_string(), 20);
        
        vec![
            TableRow::new(vec![header_cell1, header_cell2, header_cell3], 0, 0),
            TableRow::new(vec![data_cell1_1, data_cell1_2, data_cell1_3], 1, 1),
            TableRow::new(vec![data_cell2_1, data_cell2_2, data_cell2_3], 2, 2),
            TableRow::new(vec![data_cell3_1, data_cell3_2, data_cell3_3], 3, 3),
        ]
    }
}

#[derive(Debug)]
pub struct Tables {
    pub hoechste_zeile: HashMap<u32, i32>,
    pub keine_ueberschriften: bool,
    pub keine_leeren_inhalte: bool,
    pub spalten_vanilla_amount: usize,
    pub generated_spalten_parameter: HashMap<String, String>,
    pub religion_numbers: Vec<i32>,
}

impl Tables {
    pub fn new(hoechste_zeile: Option<i32>) -> Self {
        let default_hoechste_zeile = match hoechste_zeile {
            Some(value) => {
                let mut map = HashMap::new();
                map.insert(1024, value);
                map.insert(114, value);
                map
            }
            None => {
                let mut map = HashMap::new();
                map.insert(1024, 1024);
                map.insert(114, 163);
                map
            }
        };
        
        Tables {
            hoechste_zeile: default_hoechste_zeile,
            keine_ueberschriften: false,
            keine_leeren_inhalte: false,
            spalten_vanilla_amount: 0,
            generated_spalten_parameter: HashMap::new(),
            religion_numbers: Vec::new(),
        }
    }
}
