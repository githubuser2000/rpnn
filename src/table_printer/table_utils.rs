// table_printer/table_utils.rs
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use terminal_size::{terminal_size, Width as TermWidth};
use crate::table_printer::config::{ColumnKind, MAX_COLUMN_WIDTH, MIN_COLUMN_WIDTH, COLUMN_OVERHEAD, MAX_COLUMNS_CAP};
use crate::retaAusgabe::{TableRow, TableCell};

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

// --- Hilfsfunktion: Konvertiere Daten in TableRow-Strukturen MIT KORREKTEN ZEILENNUMMERN ---
pub fn convert_to_table_rows(
    headers: &[String], 
    data: &[Vec<String>], 
    column_widths: &[usize],
    zeilen_bereiche: &[(usize, usize)],  // ÄNDERUNG: start_row → zeilen_bereiche
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
    
    // Datenzeilen erstellen MIT KORREKTEN ORIGINAL-ZEILENNUMMERN
    if zeilen_bereiche.is_empty() {
        // FALL 1: Keine speziellen Zeilenbereiche → fortlaufende Nummerierung (1-basiert)
        for (row_idx, row_data) in data.iter().enumerate() {
            let actual_line_num = 1 + row_idx;  // 1-basiert, da keine spezifischen Zeilen angegeben
            
            let mut cells = Vec::new();
            for (i, cell_content) in row_data.iter().enumerate() {
                let width = if i < column_widths.len() { column_widths[i] } else { MAX_COLUMN_WIDTH };
                let cell = TableCell::new(cell_content.clone(), width);
                cells.push(cell);
            }
            
            // Sicherstellen, dass genug Zellen vorhanden sind (mit leeren füllen)
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
    } else {
        // FALL 2: Diskontinuierliche Zeilenbereiche → korrekte Originalnummern verwenden
        let mut all_row_numbers = Vec::new();
        
        // Alle tatsächlichen Zeilennummern sammeln (in der Reihenfolge der Bereiche)
        for (von, bis) in zeilen_bereiche {
            for zeile in *von..=*bis {
                all_row_numbers.push(zeile);
            }
        }
        
        println!("📊 DEBUG: {} tatsächliche Zeilennummern: {:?}", 
                 all_row_numbers.len(), all_row_numbers);
        println!("📊 DEBUG: {} Datenzeilen vorhanden", data.len());
        
        // Sicherstellen, dass wir genug Daten für alle angeforderten Zeilen haben
        if all_row_numbers.len() != data.len() {
            println!("⚠️  WARNUNG: Angefordert {} Zeilen, aber nur {} Datenzeilen vorhanden", 
                     all_row_numbers.len(), data.len());
        }
        
        // Jeder Datenzeile ihre echte Zeilennummer zuweisen
        for (row_idx, row_data) in data.iter().enumerate() {
            let actual_line_num = if row_idx < all_row_numbers.len() {
                all_row_numbers[row_idx]
            } else {
                // Fallback: fortlaufende Nummerierung ab letzter bekannter Zeile
                if let Some(last) = all_row_numbers.last() {
                    last + row_idx - all_row_numbers.len() + 1
                } else {
                    1 + row_idx
                }
            };
            
            let mut cells = Vec::new();
            for (i, cell_content) in row_data.iter().enumerate() {
                let width = if i < column_widths.len() { column_widths[i] } else { MAX_COLUMN_WIDTH };
                let cell = TableCell::new(cell_content.clone(), width);
                cells.push(cell);
            }
            
            // Sicherstellen, dass genug Zellen vorhanden sind
            while cells.len() < headers.len() {
                let width = if cells.len() < column_widths.len() { 
                    column_widths[cells.len()] 
                } else { 
                    MAX_COLUMN_WIDTH 
                };
                cells.push(TableCell::new("".to_string(), width));
            }
            
            println!("📊 DEBUG: Zeile {} bekommt Original-Nummer {}", 
                     row_idx, actual_line_num);
            
            table_rows.push(TableRow::new(cells, actual_line_num as i32, actual_line_num as i32));
        }
    }
    
    println!("📊 DEBUG: {} TableRows erstellt (davon {} Header, {} Daten)", 
             table_rows.len(), 1, table_rows.len() - 1);
    
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

// --- Hilfsfunktion für Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compute_max_lengths() {
        let headers = vec!["Name".to_string(), "Alter".to_string()];
        let data = vec![
            vec!["Hans".to_string(), "25".to_string()],
            vec!["Anna".to_string(), "30".to_string()],
        ];
        
        let max_lengths = compute_max_lengths(&headers, &data);
        assert_eq!(max_lengths, vec![4, 2]); // "Name"=4, "Alter"=2
    }
    
    #[test]
    fn test_convert_to_table_rows_without_zeilen_bereiche() {
        let headers = vec!["Name".to_string(), "Age".to_string()];
        let data = vec![
            vec!["Hans".to_string(), "25".to_string()],
            vec!["Anna".to_string(), "30".to_string()],
        ];
        let column_widths = vec![10, 10];
        let zeilen_bereiche: Vec<(usize, usize)> = Vec::new(); // Leer = alle Zeilen
        
        let rows = convert_to_table_rows(&headers, &data, &column_widths, &zeilen_bereiche);
        
        assert_eq!(rows.len(), 3); // 1 Header + 2 Datenzeilen
        assert_eq!(rows[1].original_line_num, 1); // Erste Datenzeile = Zeile 1
        assert_eq!(rows[2].original_line_num, 2); // Zweite Datenzeile = Zeile 2
    }
    
    #[test]
    fn test_convert_to_table_rows_with_zeilen_bereiche() {
        let headers = vec!["Name".to_string(), "Age".to_string()];
        let data = vec![
            vec!["Zeile5".to_string(), "25".to_string()],
            vec!["Zeile6".to_string(), "30".to_string()],
            vec!["Zeile10".to_string(), "35".to_string()],
        ];
        let column_widths = vec![10, 10];
        let zeilen_bereiche = vec![(5, 6), (10, 10)]; // Zeilen 5-6 und 10
        
        let rows = convert_to_table_rows(&headers, &data, &column_widths, &zeilen_bereiche);
        
        assert_eq!(rows.len(), 4); // 1 Header + 3 Datenzeilen
        assert_eq!(rows[1].original_line_num, 5); // Erste Datenzeile = Zeile 5
        assert_eq!(rows[2].original_line_num, 6); // Zweite Datenzeile = Zeile 6
        assert_eq!(rows[3].original_line_num, 10); // Dritte Datenzeile = Zeile 10
    }
}
