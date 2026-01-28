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

// Übernommene word_wrap Funktion aus main.rs
fn word_wrap(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    
    let mut result = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    
    if words.is_empty() {
        return vec!["".to_string()];
    }
    
    let mut current_line = String::new();
    
    for word in words {
        if current_line.len() + word.len() + if current_line.is_empty() { 0 } else { 1 } <= width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                result.push(current_line);
            }
            current_line = word.to_string();
            
            // Wenn ein einzelnes Wort länger als width ist, teile es hart
            if word.len() > width {
                let chunks: Vec<String> = word
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(width)
                    .map(|chunk| chunk.iter().collect())
                    .collect();
                
                for (i, chunk) in chunks.into_iter().enumerate() {
                    if i == 0 {
                        current_line = chunk;
                        result.push(current_line.clone());
                    } else {
                        result.push(chunk);
                    }
                }
                current_line.clear();
            }
        }
    }
    
    if !current_line.is_empty() {
        result.push(current_line);
    }
    
    if result.is_empty() {
        result.push("".to_string());
    }
    
    result
}

#[derive(Debug, Clone)]
pub struct TableCell {
    lines: Vec<String>,
    original_content: String,
}

impl TableCell {
    pub fn new(content: String, width: usize) -> Self {
        // Wende Wortumbruch an
        let lines = word_wrap(&content, width);
        
        TableCell { 
            lines,
            original_content: content,
        }
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
    
    // Übernommene colorize Funktion aus main.rs
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
    
    // Übernommene cli_out Funktion aus main.rs
    pub fn cli_out(
        &mut self,
        finally_display_lines: &BTreeSet<usize>,
        table: &[TableRow],
        rows_range: std::ops::Range<usize>,
    ) -> Vec<String> {
        if finally_display_lines.is_empty() {
            return Vec::new();
        }
        
        // Beginne Tabelle falls benötigt
        if matches!(self.out_type, OutputSyntax::HTML | OutputSyntax::BBCode) {
            self.cliout2(self.out_type.begin_table());
        }
        
        // Bestimme maximale Zellenbreiten
        let max_cell_widths = self.find_max_cell_text_len(finally_display_lines, table, &rows_range);
        
        // Konvertiere Set zu sortierter Liste
        let mut display_lines_list: Vec<usize> = finally_display_lines.iter().copied().collect();
        display_lines_list.sort();
        
        // Berechne die Anzahl der Zeilen pro Zelle
        let mut current_subcell_index: i32 = 0;
        let mut last_subcell_index: i32 = -1;
        
        while current_subcell_index > last_subcell_index && !table.is_empty() {
            last_subcell_index = current_subcell_index;
            
            // Durchlaufe alle anzuzeigenden Zeilen
            for &display_line_idx in &display_lines_list {
                if let Some(row) = table.get(display_line_idx) {
                    // Überspringe Header, wenn keine Überschriften gewünscht
                    if display_line_idx == 0 && self.tables_ref.keine_ueberschriften {
                        continue;
                    }
                    
                    // Durchlaufe alle Zeilen innerhalb der Zelle
                    for line_num in rows_range.clone() {
                        let mut line_parts = Vec::new();
                        
                        // Füge Zeilennummer hinzu, wenn gewünscht
                        if self.line_numbering {
                            let num_str = if row.original_line_num > 0 {
                                format!("{:4} ", row.original_line_num)
                            } else {
                                "     ".to_string()
                            };
                            
                            let colored_num = self.colorize(&num_str, row.original_line_num, false);
                            line_parts.push(colored_num);
                        }
                        
                        // Durchlaufe alle Spalten
                        let mut max_col_index_in_row: i32 = -1;
                        let mut total_width = if self.line_numbering { 5 } else { 0 };
                        let mut entries_in_row = 0;
                        let mut empty_entries = 0;
                        
                        for (col_idx, cell) in row.cells.iter().enumerate() {
                            let col_idx_i32 = col_idx as i32;
                            
                            if col_idx_i32 <= current_subcell_index || self.one_table {
                                // Bestimme Breite für diese Spalte
                                let cell_width = if col_idx < self.column_widths.len() {
                                    self.column_widths[col_idx]
                                } else {
                                    *max_cell_widths.get(&col_idx).unwrap_or(&self.table_width)
                                };
                                
                                total_width += cell_width + 1;
                                
                                if total_width < self.table_width || self.one_table {
                                    max_col_index_in_row = col_idx_i32;
                                    
                                    // Hole den Zelleninhalt für diese Zeile
                                    if let Some(cell_content) = cell.get_line(line_num) {
                                        entries_in_row += 1;
                                        
                                        if cell_content.trim().is_empty() {
                                            empty_entries += 1;
                                        }
                                        
                                        let formatted_content = if matches!(self.out_type, OutputSyntax::CSV) {
                                            cell_content.to_string()
                                        } else {
                                            let padded = format!("{:width$}", cell_content, width = cell_width);
                                            self.colorize(&padded, row.original_line_num, cell_content.trim().is_empty())
                                        };
                                        
                                        if matches!(self.out_type, OutputSyntax::CSV) {
                                            line_parts.push(formatted_content);
                                        } else {
                                            line_parts.push(formatted_content);
                                            line_parts.push(" ".to_string());
                                        }
                                    } else {
                                        // Leere Zelle
                                        let empty_cell = " ".repeat(cell_width);
                                        let colored_empty = self.colorize(&empty_cell, row.original_line_num, true);
                                        
                                        if matches!(self.out_type, OutputSyntax::CSV) {
                                            line_parts.push(colored_empty);
                                        } else {
                                            line_parts.push(colored_empty);
                                            line_parts.push(" ".to_string());
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Aktualisiere current_subcell_index basierend auf dieser Zeile
                        if max_col_index_in_row > current_subcell_index {
                            current_subcell_index = max_col_index_in_row;
                        }
                        
                        // Wenn die Zeile nicht komplett leer ist, gib sie aus
                        if empty_entries != entries_in_row {
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
                                    
                                    // Füge Header-Trennlinie hinzu
                                    if display_line_idx == 0 {
                                        let separator = if self.line_numbering {
                                            "|:---" .repeat(row.cells.len() + 1) + "|"
                                        } else {
                                            "|:---" .repeat(row.cells.len()) + "|"
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
                                    
                                    full_line.push_str(&self.out_type.end_zeile());
                                    self.cliout2(&full_line);
                                }
                            }
                        }
                    }
                }
            }
            
            if self.one_table {
                break;
            }
            
            // Wenn current_subcell_index sich nicht geändert hat, beende die Schleife
            if current_subcell_index <= last_subcell_index {
                break;
            }
        }
        
        // Beende Tabelle falls benötigt
        if matches!(self.out_type, OutputSyntax::HTML | OutputSyntax::BBCode) {
            self.cliout2(self.out_type.end_table());
        }
        
        self.resulting_output.clone()
    }
    
    // Übernommene find_max_cell_text_len Funktion aus main.rs
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
    
    // Übernommene create_test_table Funktion aus main.rs
    pub fn create_test_table(&self) -> Vec<TableRow> {
        // Breiten für die Spalten
        let col_widths = vec![20, 15, 25];
        
        let header_cell1 = TableCell::new("Name und Vorname".to_string(), col_widths[0]);
        let header_cell2 = TableCell::new("Alter".to_string(), col_widths[1]);
        let header_cell3 = TableCell::new("Wohnort und Beschreibung".to_string(), col_widths[2]);
        
        // Längere Texte zum Testen des Wortumbruchs
        let data_cell1_1 = TableCell::new("Hans Mustermann".to_string(), col_widths[0]);
        let data_cell1_2 = TableCell::new("25 Jahre".to_string(), col_widths[1]);
        let data_cell1_3 = TableCell::new("Berlin, Hauptstadt von Deutschland, sehr schöne Stadt mit vielen Sehenswürdigkeiten".to_string(), col_widths[2]);
        
        let data_cell2_1 = TableCell::new("Anna Schmidt".to_string(), col_widths[0]);
        let data_cell2_2 = TableCell::new("30".to_string(), col_widths[1]);
        let data_cell2_3 = TableCell::new("München in Bayern, bekannt für das Oktoberfest und die schönen Parks".to_string(), col_widths[2]);
        
        let data_cell3_1 = TableCell::new("Peter-Ludwig Meyer".to_string(), col_widths[0]);
        let data_cell3_2 = TableCell::new("22 Jahre alt".to_string(), col_widths[1]);
        let data_cell3_3 = TableCell::new("Hamburg, Hafenstadt, geboren am 15. März 2000, wohnt dort seit Geburt".to_string(), col_widths[2]);
        
        let data_cell4_1 = TableCell::new("Ein sehr langer Name der den Wortumbruch testen soll".to_string(), col_widths[0]);
        let data_cell4_2 = TableCell::new("".to_string(), col_widths[1]); // Leere Zelle
        let data_cell4_3 = TableCell::new("Superkalifragilistikexpialigetisch obwohl das eigentlich ein sehr sehr langes Wort ist das den Umbruch testet".to_string(), col_widths[2]);
        
        vec![
            TableRow::new(vec![header_cell1, header_cell2, header_cell3], 0, 0),
            TableRow::new(vec![data_cell1_1, data_cell1_2, data_cell1_3], 1, 1),
            TableRow::new(vec![data_cell2_1, data_cell2_2, data_cell2_3], 2, 2),
            TableRow::new(vec![data_cell3_1, data_cell3_2, data_cell3_3], 3, 3),
            TableRow::new(vec![data_cell4_1, data_cell4_2, data_cell4_3], 4, 4),
        ]
    }
    
    // Behalte die einfache Tabellenfunktion bei
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
