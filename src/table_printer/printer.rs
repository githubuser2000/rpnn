use std::collections::BTreeSet;

use crate::reta_ausgabe::{CliOutput, OutputSyntax, TableRow, Tables};
use crate::table_printer::config::{COLUMN_OVERHEAD, MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH};
use crate::table_printer::table_utils::{
    build_table_layout,
    compute_column_stats,
    compute_column_widths_from_global_mass,
    convert_to_table_rows,
    get_terminal_width,
    shrink_widths_to_fit_budget,
    RowRange,
};

fn build_output<'a>(
    tables: &'a Tables,
    term_width: usize,
    column_widths: Vec<usize>,
) -> CliOutput<'a> {
    let mut output = CliOutput::new(tables, OutputSyntax::Plain);
    output.color_enabled = true;
    output.table_width = term_width;
    output.column_widths = column_widths;
    output.line_numbering = true;
    output.one_table = true;
    output
}

fn render_rows(term_width: usize, column_widths: Vec<usize>, table_rows: &[TableRow]) {
    let tables = Tables::new(Some(100));
    let mut output = build_output(&tables, term_width, column_widths);

    let display_lines: BTreeSet<usize> = (0..table_rows.len()).collect();

    let max_lines_in_cells = table_rows
        .iter()
        .map(TableRow::max_line_count)
        .max()
        .unwrap_or(1);

    let rows_range = 0..max_lines_in_cells;
    output.cli_out(&display_lines, table_rows, rows_range);
}

pub fn print_table(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
) {
    let term_width = get_terminal_width();
    let available_budget = term_width
        .saturating_sub(1)
        .saturating_sub(headers.len() * COLUMN_OVERHEAD);

    let column_widths =
        compute_column_widths_from_global_mass(headers, data, available_budget);

    let table_rows = convert_to_table_rows(headers, data, &column_widths, row_ranges);
    render_rows(term_width, column_widths, &table_rows);
}

pub fn print_table_chunked(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
) {
    let term_width = get_terminal_width();
    // 3 Zeichen Puffer für den rechten Rand (Scrollbar/Terminal-Varianz)
    let available_total = term_width.saturating_sub(3); 

    // Wir berechnen das Budget so, dass pro Spalte Platz für den Content + Trenner ist
    let overhead_per_col = COLUMN_OVERHEAD + 1; 
    let full_budget = available_total.saturating_sub(headers.len() * overhead_per_col);
    
    let global_widths = compute_column_widths_from_global_mass(headers, data, full_budget);

    let mut start = 0usize;
    while start < headers.len() {
        let mut used = 0usize;
        let mut end = start;

        while end < headers.len() {
            let col_width = global_widths[end];
            // Wir rechnen: VERDOPPELTE Spaltenbreite + Overhead + 1 Sicherheitszeichen
            let needed = (col_width * 2) + COLUMN_OVERHEAD + 1;

            // Wenn die nächste Spalte (die jetzt sehr breit sein kann) nicht mehr passt:
            if used + needed > available_total {
                // Wenn noch gar keine Spalte im Chunk ist, müssen wir diese eine nehmen,
                // auch wenn sie breiter als das Terminal ist (wird dann abgeschnitten/umgebrochen).
                if end == start {
                    end += 1;
                }
                break; 
            }

            used += needed;
            end += 1;
        }

        if end == start { end = start + 1; }

        let chunk_headers = &headers[start..end];
        // Auch hier die Breiten verdoppeln
        let chunk_widths: Vec<usize> = global_widths[start..end]
            .iter()
            .map(|&w| (w * 2).min(MAX_COLUMN_WIDTH))
            .collect();
        
        let chunk_data: Vec<Vec<String>> = data.iter()
            .map(|row| {
                let mut partial = if start < row.len() {
                    row[start..end.min(row.len())].to_vec()
                } else { Vec::new() };
                partial.resize(end - start, String::new());
                partial
            }).collect();

        // Anzeige-Logik
        if start > 0 {
            println!("\n{}", "─".repeat(available_total));
            println!("Fortsetzung (Spalten {}-{}):", start + 1, end);
        }

        let table_rows = convert_to_table_rows(chunk_headers, &chunk_data, &chunk_widths, row_ranges);
        render_rows(term_width, chunk_widths, &table_rows);

        start = end;
    }
}

pub fn print_table_auto(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
) {
    let layout = build_table_layout(headers, data);
    let table_rows = convert_to_table_rows(headers, data, &layout.column_widths, row_ranges);

    render_rows(layout.term_width, layout.column_widths, &table_rows);
}
