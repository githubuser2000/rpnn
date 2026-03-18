use std::collections::BTreeSet;

use crate::reta_ausgabe::{CliOutput, OutputSyntax, TableRow, Tables};
use crate::table_printer::config::MAX_COLUMN_WIDTH;
use crate::table_printer::table_utils::{
    build_table_layout, chunk_bounds, compute_column_widths, compute_column_widths_optimized,
    compute_columns_per_table, compute_columns_per_table_optimized, compute_max_lengths,
    convert_to_table_rows, get_terminal_width, RowRange,
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

// Alte kompatible Signatur
pub fn print_table(
    headers: &[String],
    data: Vec<Vec<String>>,
    max_lengths: &[usize],
    row_ranges: &[RowRange],
) {
    let term_width = get_terminal_width();
    let column_widths = compute_column_widths(headers, max_lengths, term_width);
    let table_rows = convert_to_table_rows(headers, &data, &column_widths, row_ranges);

    render_rows(term_width, column_widths, &table_rows);
}

// Neue empfohlene Variante
pub fn print_table_optimized(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
) {
    let term_width = get_terminal_width();
    let column_widths = compute_column_widths_optimized(headers, data, term_width);
    let table_rows = convert_to_table_rows(headers, data, &column_widths, row_ranges);

    render_rows(term_width, column_widths, &table_rows);
}

// Alte kompatible Chunk-Variante
pub fn print_table_chunked(headers: &[String], data: &[Vec<String>], row_ranges: &[RowRange]) {
    let term_width = get_terminal_width();
    let max_lengths = compute_max_lengths(headers, data);

    let capped_lengths: Vec<usize> = max_lengths
        .iter()
        .map(|&w| w.min(MAX_COLUMN_WIDTH))
        .collect();

    let chunk_size = compute_columns_per_table(term_width, headers, &capped_lengths);
    let bounds = chunk_bounds(headers.len(), chunk_size);

    for (chunk_index, (start, end)) in bounds.iter().copied().enumerate() {
        let chunk_headers = &headers[start..end];
        let chunk_max_lengths = &capped_lengths[start..end];

        let chunk_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| {
                let slice_end = end.min(row.len());
                let mut partial = if start < row.len() {
                    row[start..slice_end].to_vec()
                } else {
                    Vec::new()
                };
                partial.resize(end - start, String::new());
                partial
            })
            .collect();

        if chunk_index > 0 {
            println!();
            println!("{}", "─".repeat(term_width));
            println!("Fortsetzung (Spalten {}-{}):", start + 1, end);
            println!("{}", "─".repeat(term_width));
        }

        print_table(chunk_headers, chunk_data, chunk_max_lengths, row_ranges);
    }
}

// Optimierte Chunk-Variante mit kompakteren Standardbreiten
pub fn print_table_chunked_optimized(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
) {
    let term_width = get_terminal_width();

    let mut start = 0usize;
    let mut chunk_index = 0usize;

    while start < headers.len() {
        let remaining_headers = &headers[start..];

        let remaining_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| {
                if start < row.len() {
                    row[start..].to_vec()
                } else {
                    Vec::new()
                }
            })
            .collect();

        let cols_in_chunk =
            compute_columns_per_table_optimized(term_width, remaining_headers, &remaining_data);

        let end = (start + cols_in_chunk).min(headers.len());
        let chunk_headers = &headers[start..end];

        let chunk_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| {
                let slice_end = end.min(row.len());
                let mut partial = if start < row.len() {
                    row[start..slice_end].to_vec()
                } else {
                    Vec::new()
                };
                partial.resize(end - start, String::new());
                partial
            })
            .collect();

        if chunk_index > 0 {
            println!();
            println!("{}", "─".repeat(term_width.saturating_sub(1)));
            println!("Fortsetzung (Spalten {}-{}):", start + 1, end);
            println!("{}", "─".repeat(term_width.saturating_sub(1)));
        }

        print_table_optimized(chunk_headers, &chunk_data, row_ranges);

        start = end;
        chunk_index += 1;
    }
}

pub fn print_table_auto(headers: &[String], data: &[Vec<String>], row_ranges: &[RowRange]) {
    let layout = build_table_layout(headers, data);
    let table_rows = convert_to_table_rows(headers, data, &layout.column_widths, row_ranges);

    render_rows(layout.term_width, layout.column_widths, &table_rows);
}
