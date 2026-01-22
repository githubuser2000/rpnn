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

// Strukturen für Tabellendaten
#[derive(Debug, Clone)]
pub struct TableCell {
    lines: Vec<String>,
}

impl TableCell {
    pub fn new(content: String, width: usize) -> Self {
        let lines: Vec<String> = content
            .split('\n')
            .map(|line| line.to_string())
            .collect();
        
        // Kürze oder fülle jede Zeile auf die gewünschte visuelle Breite
        let formatted_lines: Vec<String> = lines
            .iter()
            .map(|line| {
                let line_width = UnicodeWidthStr::width(line.as_str());
                
                if line_width > width {
                    // UTF-8 sichere Kürzung basierend auf visueller Breite
                    let mut result = String::new();
                    let mut current_width = 0;
                    
                    for ch in line.chars() {
                        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0); // Verwende UnicodeWidthChar
                        if current_width + ch_width > width {
                            result.push('…');
                            break;
                        }
                        result.push(ch);
                        current_width += ch_width;
                    }
                    
                    result
                } else {
                    // Auffüllen mit Leerzeichen
                    format!("{:<width$}", line, width = width)
                }
            })
            .collect();
        
        TableCell { lines: formatted_lines }
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
                    // Header-Zeile
                    text.red().on_white().bold().to_string()
                } else if is_empty {
                    // Leere Zellen
                    if line_num % 2 == 0 {
                        text.black().on_white().to_string()
                    } else {
                        text.white().on_black().to_string()
                    }
                } else {
                    // Reguläre Zellen mit spezieller Farbcodierung
                    if line_num % 2 == 0 {
                        text.black().on_white().to_string()
                    } else {
                        text.white().on_black().to_string()
                    }
                }
            }
            _ => text.to_string(),
        }
    }
    
    pub fn cliout2(&mut self, text: &str) {
        self.resulting_output.push(text.to_string());
        
        // Nur ausgeben, wenn nicht NichtsSyntax
        if !matches!(self.out_type, OutputSyntax::Nichts) {
            println!("{}", text);
        }
    }
    
    pub fn cli_out(
        &mut self,
        finally_display_lines: &BTreeSet<usize>,
        table: &[TableRow],
        rows_range: std::ops::Range<usize>,
    ) -> Vec<String> {
        // Kopiere hier die komplette Implementierung der cli_out-Methode
        // aus deiner originalen retaAusgabe.rs Datei
        // Stelle sicher, dass sie vollständig ist
        
        // Hier nur ein Platzhalter - du musst die echte Implementierung einfügen
        if finally_display_lines.is_empty() {
            return Vec::new();
        }
        
        // Beginne Tabelle falls benötigt
        if matches!(self.out_type, OutputSyntax::HTML | OutputSyntax::BBCode) {
            self.cliout2(self.out_type.begin_table());
        }
        
        // Vereinfachte Rückgabe - ersetze durch echte Logik
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

// Haupt-Tabellen-Struktur
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
