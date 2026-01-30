use terminal_size::{terminal_size, Width as TermWidth};
use crate::cli::TextBereich;
use crate::column_manager::{get_column_names, build_column_query};
use crate::data_fetcher::fetch_data_with_stats;
use rusqlite::Connection;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use std::collections::BTreeSet;
use crate::retaAusgabe::{CliOutput, Tables, OutputSyntax, TableRow, TableCell};
const MIN_COLUMN_WIDTH: usize = 10;
const MAX_COLUMNS_CAP: usize = 6;
const MAX_COLUMN_WIDTH: usize = 34;
const COLUMN_OVERHEAD: usize = 5;

#[derive(Copy, Clone)]
enum ColumnKind {
    Id,
    Number,
    ShortText,
    LongText,
}

// --- Column Kind & Min Width ---
fn infer_column_kind(header: &str) -> ColumnKind {
    let h = header.to_lowercase();
    if h == "id" || h.ends_with("_id") {
        ColumnKind::Id
    } else if h.contains("count") || h.contains("num") {
        ColumnKind::Number
    } else if h.contains("name") || h.contains("title") {
        ColumnKind::ShortText
    } else {
        ColumnKind::LongText
    }
}

fn min_width_for_kind(kind: ColumnKind) -> usize {
    match kind {
        ColumnKind::Id => 6,
        ColumnKind::Number => 8,
        ColumnKind::ShortText => 14,
        ColumnKind::LongText => 20,
    }
}

// --- UTF8 Truncation ---
fn truncate_cell(content: &str, max_width: usize) -> String {
    let mut width = 0;
    let mut truncated = String::new();

    for ch in content.chars() {
        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width > max_width {
            truncated.push('…');
            break;
        }
        truncated.push(ch);
        width += ch_width;
    }
    truncated
}

// --- Compute how many columns fit ---
fn compute_columns_per_table(term_width: usize, headers: &[String], max_lengths: &[usize]) -> usize {
    if headers.is_empty() { return 1; }

    let mut used_width = 0;
    let mut cols = 0;

    for (h, &w) in headers.iter().zip(max_lengths) {
        let kind = infer_column_kind(h);
        let min = min_width_for_kind(kind).max(MIN_COLUMN_WIDTH);
        let col_width = w.max(min).min(MAX_COLUMN_WIDTH) + COLUMN_OVERHEAD;

        if cols >= 2 && used_width + col_width > term_width { break; }

        used_width += col_width;
        cols += 1;

        if cols >= MAX_COLUMNS_CAP { break; }
    }

    if headers.len() < 2 { headers.len() } else { cols.clamp(2, MAX_COLUMNS_CAP) }
}

// --- Hilfsfunktion: Konvertiere Daten in TableRow-Strukturen ---
fn convert_to_table_rows(
    headers: &[String], 
    data: &[Vec<String>], 
    column_widths: &[usize],
    start_row_num: usize, // NEU: Startzeilennummer für die Ausgabe
) -> Vec<TableRow> {
    let mut table_rows = Vec::new();
    
    // Header-Zeile erstellen
    let mut header_cells = Vec::new();
    for (i, header) in headers.iter().enumerate() {
        let width = if i < column_widths.len() { column_widths[i] } else { MAX_COLUMN_WIDTH };
        let cell = TableCell::new(header.clone(), width);
        header_cells.push(cell);
    }
    table_rows.push(TableRow::new(header_cells, 0, 0));
    
    // Datenzeilen erstellen
    for (row_idx, row_data) in data.iter().enumerate() {
        // WICHTIG: Berechne die tatsächliche Zeilennummer
        let actual_line_num = start_row_num + row_idx;
        let mut cells = Vec::new();
        for (i, cell_content) in row_data.iter().enumerate() {
            let width = if i < column_widths.len() { column_widths[i] } else { MAX_COLUMN_WIDTH };
            let cell = TableCell::new(cell_content.clone(), width);
            cells.push(cell);
        }
        // Sicherstellen, dass alle Spalten vorhanden sind
        while cells.len() < headers.len() {
            let width = if cells.len() < column_widths.len() { 
                column_widths[cells.len()] 
            } else { 
                MAX_COLUMN_WIDTH 
            };
            cells.push(TableCell::new("".to_string(), width));
        }
        // Hier die tatsächliche Zeilennummer setzen!
        table_rows.push(TableRow::new(cells, actual_line_num as i32, actual_line_num as i32));
    }
    
    table_rows
}

// --- Print one table chunk (mit retaAusgabe) ---
pub fn print_table(headers: &[String], data: Vec<Vec<String>>, max_lengths: &[usize], start_row: usize) {
    let tables = Tables::new(Some(100));
    
    // Bestimme Terminalbreite für die Ausgabe
    let term_width = terminal_size().map(|(TermWidth(w), _)| w as usize).unwrap_or(80);
    
    // Berechne Spaltenbreiten unter Berücksichtigung von Unicode
    let mut column_widths = Vec::new();
    for (i, header) in headers.iter().enumerate() {
        let header_width = UnicodeWidthStr::width(header.as_str());
        let max_data_width = if i < max_lengths.len() { max_lengths[i] } else { 0 };
        let kind = infer_column_kind(header);
        let min_width = min_width_for_kind(kind);
        
        // Wähle die maximale Breite, berücksichtige aber Terminalgrenzen
        let desired_width = header_width.max(max_data_width).max(min_width);
        let capped_width = desired_width.min(MAX_COLUMN_WIDTH).min(term_width / 2);
        column_widths.push(capped_width);
    }
    
    // Konvertiere Daten in TableRow-Strukturen mit korrekter Startzeilennummer
    let table_rows = convert_to_table_rows(headers, &data, &column_widths, start_row);
    
    // Erstelle CliOutput für Plain-Text mit Farben
    let mut output = CliOutput::new(&tables, OutputSyntax::Plain);
    output.color_enabled = true;
    output.table_width = term_width;
    output.column_widths = column_widths.clone();
    output.line_numbering = true; // Zeilennummern aktivieren
    output.one_table = true; // Eine zusammenhängende Tabelle
    
    // Erstelle Display-Lines Set
    let display_lines: BTreeSet<usize> = (0..table_rows.len()).collect();
    
    // Bestimme maximale Zeilenanzahl pro Zelle für rows_range
    let max_lines_in_cells = table_rows.iter()
        .map(|row: &TableRow| row.max_line_count())
        .max()
        .unwrap_or(1);
    let rows_range = 0..max_lines_in_cells;
    
    // Gib Tabelle aus
    output.cli_out(&display_lines, &table_rows, rows_range);
}

// --- Print table in automatic chunks (angepasst für retaAusgabe) ---
pub fn print_table_chunked(headers: &[String], data: &[Vec<String>], start_row: usize) {
    let term_width = terminal_size().map(|(TermWidth(w), _)| w as usize).unwrap_or(100);

    // max_lengths automatisch berechnen mit Unicode-Unterstützung
    let mut max_lengths: Vec<usize> = headers.iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();
    
    for row in data {
        for (i, cell) in row.iter().enumerate() {
            if i < max_lengths.len() {
                max_lengths[i] = max_lengths[i].max(UnicodeWidthStr::width(cell.as_str()));
            }
        }
    }

    let mut start = 0;
    let mut chunk_num = 0;
    
    while start < headers.len() {
        let remaining_headers = &headers[start..];
        let remaining_lengths = &max_lengths[start..];

        let capped_lengths: Vec<usize> = remaining_lengths.iter()
            .map(|&w| w.min(MAX_COLUMN_WIDTH))
            .collect();
        
        let cols_per_table = compute_columns_per_table(term_width, remaining_headers, &capped_lengths);
        let end = (start + cols_per_table).min(headers.len());

        let chunk_headers = &headers[start..end];
        let chunk_max_lengths = &capped_lengths[..end - start];

        // Extrahiere Daten für diesen Chunk
        let chunk_data: Vec<Vec<String>> = data.iter()
            .map(|row| {
                let mut r = row[start..end.min(row.len())].to_vec();
                // Fülle fehlende Zellen mit leeren Strings
                while r.len() < end - start {
                    r.push("".to_string());
                }
                r
            })
            .collect();

        // Gib Chunk als separate Tabelle aus
        if chunk_num > 0 {
            println!("\n{}", "─".repeat(term_width));
            println!("Fortsetzung (Spalten {}-{}):", start + 1, end);
            println!("{}", "─".repeat(term_width));
        }
        
        // Beachte: start_row wird hier verwendet
        print_table(chunk_headers, chunk_data, chunk_max_lengths, start_row);
        
        start = end;
        chunk_num += 1;
    }
}

// --- Query-Funktion ---
pub fn query_column_by_index(conn: &Connection, bereich: TextBereich) -> Result<(), Box<dyn std::error::Error>> {
    let column_names = get_column_names(conn)?;
    let (query, headers) = build_column_query(&column_names, bereich.clone())?; // NEU
    
    // Berechne Header-Längen mit Unicode-Unterstützung
    let header_lengths: Vec<usize> = headers.iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();
    
    let (data, _max_lengths) = fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    // Bestimme die Startzeilennummer für die Ausgabe
    let start_row_num = if !bereich.zeilen_bereiche.is_empty() {
        // Erster Wert aus zeilen_bereiche
        bereich.zeilen_bereiche[0].0
    } else {
        // von_zeile
        bereich.von_zeile
    };

    print_table_chunked(&headers, &data, start_row_num);
    println!();
    Ok(())
}

// --- Zusätzliche Hilfsfunktion für einfache Tabellen ---
pub fn print_simple_table(headers: &[&str], data: &[Vec<&str>]) {
    let headers_strings: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
    let data_strings: Vec<Vec<String>> = data.iter()
        .map(|row| row.iter().map(|s| s.to_string()).collect())
        .collect();
    
    // Berechne max_lengths
    let mut max_lengths: Vec<usize> = headers_strings.iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();
    
    for row in &data_strings {
        for (i, cell) in row.iter().enumerate() {
            if i < max_lengths.len() {
                max_lengths[i] = max_lengths[i].max(UnicodeWidthStr::width(cell.as_str()));
            }
        }
    }
    
    // Startzeilennummer für einfache Tabellen ist 1
    print_table(&headers_strings, data_strings, &max_lengths, 1);
}
