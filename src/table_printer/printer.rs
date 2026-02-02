// table_printer/printer.rs
use std::collections::BTreeSet;
use crate::table_printer::config::MAX_COLUMN_WIDTH;
use crate::table_printer::table_utils::{
    compute_columns_per_table, convert_to_table_rows, get_terminal_width,
    compute_max_lengths, compute_column_widths
};
use crate::retaAusgabe::{CliOutput, Tables, OutputSyntax, TableRow};  // TableRow importiert

// --- Print one table chunk (mit retaAusgabe) ---
pub fn print_table(headers: &[String], data: Vec<Vec<String>>, max_lengths: &[usize], zeilen_bereiche: &[(usize, usize)]) {
    let tables = Tables::new(Some(100));
    let term_width = get_terminal_width();
    
    let column_widths = compute_column_widths(headers, max_lengths, term_width);
    let table_rows = convert_to_table_rows(headers, &data, &column_widths, zeilen_bereiche); // zeilen_bereiche übergeben!
   let mut output = CliOutput::new(&tables, OutputSyntax::Plain);
    output.color_enabled = true;
    output.table_width = term_width;
    output.column_widths = column_widths.clone();
    output.line_numbering = true;
    output.one_table = true;
    
    let display_lines: BTreeSet<usize> = (0..table_rows.len()).collect();
    let max_lines_in_cells = table_rows.iter()
        .map(|row: &TableRow| row.max_line_count())
        .max()
        .unwrap_or(1);
    let rows_range = 0..max_lines_in_cells;
    
    output.cli_out(&display_lines, &table_rows, rows_range);
}

// ÄNDERUNG auch hier:
pub fn print_table_chunked(headers: &[String], data: &[Vec<String>], zeilen_bereiche: &[(usize, usize)]) {  // Parameter geändert
    let term_width = get_terminal_width();
    let max_lengths = compute_max_lengths(headers, data);

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

        let chunk_data: Vec<Vec<String>> = data.iter()
            .map(|row| {
                let mut r = row[start..end.min(row.len())].to_vec();
                while r.len() < end - start {
                    r.push("".to_string());
                }
                r
            })
            .collect();

        if chunk_num > 0 {
            println!("\n{}", "─".repeat(term_width));
            println!("Fortsetzung (Spalten {}-{}):", start + 1, end);
            println!("{}", "─".repeat(term_width));
        }
        
        // ÄNDERUNG: zeilen_bereiche übergeben
        print_table(chunk_headers, chunk_data, chunk_max_lengths, zeilen_bereiche);
        
        start = end;
        chunk_num += 1;
    }
}


