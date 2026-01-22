use std::collections::{BTreeSet, HashMap};
use colored::*;

// Enum für verschiedene Ausgabesyntax-Typen
#[derive(Debug, Clone, Copy)]
enum OutputSyntax {
    Plain,
    Markdown,
    BBCode,
    HTML,
    CSV,
    Emacs,
    Nichts,
}

impl OutputSyntax {
    fn begin_table(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "<table>",
            OutputSyntax::BBCode => "[table]",
            _ => "",
        }
    }
    
    fn end_table(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "</table>",
            OutputSyntax::BBCode => "[/table]",
            _ => "",
        }
    }
    
    fn generate_cell(self, _col_index: i32, _params: &HashMap<String, String>, _line_num: i32) -> String {
        match self {
            OutputSyntax::HTML => "<td>".to_string(),
            OutputSyntax::BBCode => "[td]".to_string(),
            OutputSyntax::Markdown => "|".to_string(),
            _ => "".to_string(),
        }
    }
    
    fn end_cell(self) -> &'static str {
        match self {
            OutputSyntax::HTML => "</td>",
            OutputSyntax::BBCode => "[/td]",
            _ => "",
        }
    }
    
    fn colored_begin_col(self, _line_num: i32) -> &'static str {
        ""
    }
    
    fn end_zeile(self) -> &'static str {
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
struct TableCell {
    lines: Vec<String>,
}

impl TableCell {
    fn new(content: String, width: usize) -> Self {
        let lines: Vec<String> = content
            .split('\n')
            .map(|line| line.to_string())
            .collect();
        
        // Kürze oder fülle jede Zeile auf die gewünschte Breite
        let formatted_lines: Vec<String> = lines
            .iter()
            .map(|line| {
                if line.len() > width {
                    line[..width].to_string()
                } else {
                    format!("{:width$}", line, width = width)
                }
            })
            .collect();
        
        TableCell { lines: formatted_lines }
    }
    
    fn get_line(&self, line_num: usize) -> Option<&str> {
        self.lines.get(line_num).map(|s| s.as_str())
    }
    
    fn line_count(&self) -> usize {
        self.lines.len()
    }
}

#[derive(Debug, Clone)]
struct TableRow {
    cells: Vec<TableCell>,
    original_line_num: i32,
    display_line_num: i32,
}

impl TableRow {
    fn new(cells: Vec<TableCell>, original_line_num: i32, display_line_num: i32) -> Self {
        TableRow {
            cells,
            original_line_num,
            display_line_num,
        }
    }
    
    fn max_line_count(&self) -> usize {
        self.cells.iter()
            .map(|cell| cell.line_count())
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
struct CliOutput<'a> {
    out_type: OutputSyntax,
    color_enabled: bool,
    one_table: bool,
    table_width: usize,
    column_widths: Vec<usize>,
    line_numbering: bool,
    resulting_output: Vec<String>,
    tables_ref: &'a Tables,
}

impl<'a> CliOutput<'a> {
    fn new(tables: &'a Tables, out_type: OutputSyntax) -> Self {
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
    
    fn colorize(&self, text: &str, line_num: i32, is_empty: bool) -> String {
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
    
    fn cliout2(&mut self, text: &str) {
        self.resulting_output.push(text.to_string());
        
        // Nur ausgeben, wenn nicht NichtsSyntax
        if !matches!(self.out_type, OutputSyntax::Nichts) {
            println!("{}", text);
        }
    }
    
    fn cli_out(
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
    
    fn find_max_cell_text_len(
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
    
    // Hilfsfunktion für einfachere Tabellenerstellung
    fn create_simple_table(&self) -> Vec<TableRow> {
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
struct Tables {
    hoechste_zeile: HashMap<u32, i32>,
    keine_ueberschriften: bool,
    keine_leeren_inhalte: bool,
    spalten_vanilla_amount: usize,
    generated_spalten_parameter: HashMap<String, String>,
    religion_numbers: Vec<i32>,
}

impl Tables {
    fn new(hoechste_zeile: Option<i32>) -> Self {
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

// Testfunktionen
fn test_colorize() {
    println!("\n=== Test colorize() Funktion ===");
    let tables = Tables::new(Some(100));
    
    // Test mit Plain-Output
    let mut output = CliOutput::new(&tables, OutputSyntax::Plain);
    output.color_enabled = true;
    
    // Erste Testsammlung - mit Referenz verwenden
    let tests1 = vec![
        ("Header Zeile", 0, false),
        ("Gerade Zeile", 2, false),
        ("Ungerade Zeile", 3, false),
        ("Leere Zelle gerade", 4, true),
        ("Leere Zelle ungerade", 5, true),
    ];
    
    // Erste Schleife - mit Referenz iterieren
    println!("--- Mit Farben (color_enabled = true) ---");
    for &(text, line_num, is_empty) in &tests1 {
        let colored = output.colorize(text, line_num, is_empty);
        println!("Line {} (empty: {}): {}", line_num, is_empty, colored);
    }
    
    // Test ohne Farben - neue Testsammlung
    println!("\n--- Ohne Farben (color_enabled = false) ---");
    output.color_enabled = false;
    let tests2 = vec![
        ("Test ohne Farben 1", 1, false),
        ("Test ohne Farben 2", 2, true),
        ("Test ohne Farben 3", 3, false),
    ];
    
    for &(text, line_num, is_empty) in &tests2 {
        let colored = output.colorize(text, line_num, is_empty);
        println!("Line {} (empty: {}): {}", line_num, is_empty, colored);
    }
}

fn test_cliout2() {
    println!("\n=== Test cliout2() Funktion ===");
    let tables = Tables::new(Some(100));
    
    println!("--- Mit Plain Output (sollte ausgegeben werden) ---");
    let mut output1 = CliOutput::new(&tables, OutputSyntax::Plain);
    output1.cliout2("Erste Zeile");
    output1.cliout2("Zweite Zeile");
    println!("Gespeicherte Ausgaben: {:?}", output1.resulting_output);
    
    println!("\n--- Mit Nichts Output (sollte NICHT ausgegeben werden) ---");
    let mut output2 = CliOutput::new(&tables, OutputSyntax::Nichts);
    output2.cliout2("Unsichtbare Zeile 1");
    output2.cliout2("Unsichtbare Zeile 2");
    println!("Gespeicherte Ausgaben trotzdem: {:?}", output2.resulting_output);
}

fn test_cli_out_plain() {
    println!("\n=== Test cli_out() Funktion - Plain Format ===");
    let tables = Tables::new(Some(100));
    
    let mut output = CliOutput::new(&tables, OutputSyntax::Plain);
    output.color_enabled = true;
    output.table_width = 60;
    output.line_numbering = true;
    
    let table_data = output.create_simple_table();
    let display_lines: BTreeSet<usize> = [0, 1, 2, 3].iter().copied().collect();
    let rows_range = 0..2; // Zellen können bis zu 2 Zeilen haben
    
    println!("\n--- Tabellenausgabe (Plain mit Farben) ---");
    let result = output.cli_out(&display_lines, &table_data, rows_range);
    println!("\nGesamtanzahl ausgegebener Zeilen: {}", result.len());
}

fn test_cli_out_markdown() {
    println!("\n=== Test cli_out() Funktion - Markdown Format ===");
    let tables = Tables::new(Some(100));
    
    let mut output = CliOutput::new(&tables, OutputSyntax::Markdown);
    output.color_enabled = false; // Markdown braucht keine Terminalfarben
    output.table_width = 60;
    output.line_numbering = true;
    
    let table_data = output.create_simple_table();
    let display_lines: BTreeSet<usize> = [0, 1, 2, 3].iter().copied().collect();
    let rows_range = 0..2;
    
    println!("\n--- Tabellenausgabe (Markdown) ---");
    let result = output.cli_out(&display_lines, &table_data, rows_range);
    println!("\nGesamtanzahl ausgegebener Zeilen: {}", result.len());
}

fn test_cli_out_csv() {
    println!("\n=== Test cli_out() Funktion - CSV Format ===");
    let tables = Tables::new(Some(100));
    
    let mut output = CliOutput::new(&tables, OutputSyntax::CSV);
    output.color_enabled = false; // CSV braucht keine Farben
    output.table_width = 60;
    output.line_numbering = true;
    
    let table_data = output.create_simple_table();
    let display_lines: BTreeSet<usize> = [0, 1, 2, 3].iter().copied().collect();
    let rows_range = 0..2;
    
    println!("\n--- Tabellenausgabe (CSV) ---");
    let result = output.cli_out(&display_lines, &table_data, rows_range);
    println!("\nGesamtanzahl ausgegebener Zeilen: {}", result.len());
}

fn test_cli_out_html() {
    println!("\n=== Test cli_out() Funktion - HTML Format ===");
    let tables = Tables::new(Some(100));
    
    let mut output = CliOutput::new(&tables, OutputSyntax::HTML);
    output.color_enabled = false; // HTML hat eigene Farben
    output.table_width = 60;
    output.line_numbering = true;
    output.one_table = true; // Für HTML eine Tabelle
    
    let table_data = output.create_simple_table();
    let display_lines: BTreeSet<usize> = [0, 1, 2, 3].iter().copied().collect();
    let rows_range = 0..2;
    
    println!("\n--- Tabellenausgabe (HTML) ---");
    let result = output.cli_out(&display_lines, &table_data, rows_range);
    println!("\nGesamtanzahl ausgegebener Zeilen: {}", result.len());
}

// Beispiel-Nutzung
fn main() {
    println!("=== TESTAUSGABEN FÜR TABELLEN-HANDLING ===\n");
    
    // Teste einzelne Funktionen
    test_colorize();
    test_cliout2();
    
    // Teste verschiedene Ausgabeformate
    test_cli_out_plain();
    test_cli_out_markdown();
    test_cli_out_csv();
    test_cli_out_html();
    
    println!("\n=== ALLE TESTS ABGESCHLOSSEN ===");
}
