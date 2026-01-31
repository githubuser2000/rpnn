// table_printer/table_utils.rs
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use terminal_size::{terminal_size, Width as TermWidth};
use crate::table_printer::config::{ColumnKind, MAX_COLUMN_WIDTH, MIN_COLUMN_WIDTH, COLUMN_OVERHEAD, MAX_COLUMNS_CAP};
use crate::retaAusgabe::{TableRow, TableCell};  // Importiere TableRow und TableCell

// --- UTF8 Truncation ---
pub fn truncate_cell(content: &str, max_width: usize) -> String {
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
pub fn compute_columns_per_table(term_width: usize, headers: &[String], max_lengths: &[usize]) -> usize {
    if headers.is_empty() { return 1; }

    let mut used_width = 0;
    let mut cols = 0;

    for (h, &w) in headers.iter().zip(max_lengths) {
        let kind = ColumnKind::infer_from_header(h);
        let min = kind.min_width().max(MIN_COLUMN_WIDTH);
        let col_width = w.max(min).min(MAX_COLUMN_WIDTH) + COLUMN_OVERHEAD;

        if cols >= 2 && used_width + col_width > term_width { break; }

        used_width += col_width;
        cols += 1;

        if cols >= MAX_COLUMNS_CAP { break; }
    }

    if headers.len() < 2 { headers.len() } else { cols.clamp(2, MAX_COLUMNS_CAP) }
}

// --- Hilfsfunktion: Konvertiere Daten in TableRow-Strukturen ---
pub fn convert_to_table_rows(
    headers: &[String], 
    data: &[Vec<String>], 
    column_widths: &[usize],
    start_row_num: usize,
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
        let actual_line_num = start_row_num + row_idx;
        let mut cells = Vec::new();
        for (i, cell_content) in row_data.iter().enumerate() {
            let width = if i < column_widths.len() { column_widths[i] } else { MAX_COLUMN_WIDTH };
            let cell = TableCell::new(cell_content.clone(), width);
            cells.push(cell);
        }
        while cells.len() < headers.len() {
            let width = if cells.len() < column_widths.len() { 
                column_widths[cells.len()] 
            } else { 
                MAX_COLUMN_WIDTH 
            };
            cells.push(TableCell::new("".to_string(), width));
        }
        table_rows.push(TableRow::new(cells, actual_line_num as i32, actual_line_num as i32));
    }
    
    table_rows
}

// --- Get terminal width ---
pub fn get_terminal_width() -> usize {
    terminal_size().map(|(TermWidth(w), _)| w as usize).unwrap_or(80)
}

// --- Compute max lengths from data ---
pub fn compute_max_lengths(headers: &[String], data: &[Vec<String>]) -> Vec<usize> {
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
    
    max_lengths
}

// --- Compute column widths ---
pub fn compute_column_widths(headers: &[String], max_lengths: &[usize], term_width: usize) -> Vec<usize> {
    let mut column_widths = Vec::new();
    for (i, header) in headers.iter().enumerate() {
        let header_width = UnicodeWidthStr::width(header.as_str());
        let max_data_width = if i < max_lengths.len() { max_lengths[i] } else { 0 };
        let kind = ColumnKind::infer_from_header(header);
        let min_width = kind.min_width();
        
        let desired_width = header_width.max(max_data_width).max(min_width);
        let capped_width = desired_width.min(MAX_COLUMN_WIDTH).min(term_width / 2);
        column_widths.push(capped_width);
    }
    column_widths
}
