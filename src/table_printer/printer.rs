use std::collections::BTreeSet;

use crate::reta_ausgabe::{CliOutput, OutputSyntax, TableRow, Tables};
use crate::table_printer::config::COLUMN_OVERHEAD;
use crate::table_printer::meta_columns::{
    build_meta_widths, build_power_bucket_strings, prepend_meta_columns,
};
use crate::table_printer::sanitize::{
    sanitize_chunk_data, sanitize_chunk_data_with_rows, sanitize_header_for_output, sanitize_header_preserve_id,
};
use crate::table_printer::table_utils::{
    build_table_layout,
    convert_to_table_rows,
    convert_to_table_rows_with_line_numbers,
    convert_to_table_rows_with_offset,
    get_terminal_width,
    RowRange,
};
use crate::table_printer::widths::{
    build_chunk_widths, clamp_explicit_width, determine_chunk_end, effective_min_column_width,
    get_explicit_width, stretch_last_column_to_fill_budget,
};

pub fn print_table_chunked_with_line_numbers(
    headers: &[String],
    data: &[Vec<String>],
    explizite_breiten: &[usize],
    original_line_numbers: &[usize],
    keineleereninhalte: bool,
    out_type: OutputSyntax,
    pretty_output: bool,
) {
    let term_width = get_terminal_width();
    let available_total = term_width.saturating_sub(1);
    let render_width = available_total;
    let mut start = 0usize;

    while start < headers.len() {
        let forced_end = if matches!(out_type, OutputSyntax::HTML) {
            Some(headers.len())
        } else {
            None
        };
        let all_power_buckets = build_power_bucket_strings(original_line_numbers);
        let (meta_widths_preview, meta_reserved_total) =
            build_meta_widths(&all_power_buckets, original_line_numbers, available_total);

        let data_available_total = available_total.saturating_sub(meta_reserved_total);

        let end = forced_end.unwrap_or_else(|| determine_chunk_end(
            headers,
            data,
            explizite_breiten,
            start,
            data_available_total.max(effective_min_column_width() + COLUMN_OVERHEAD + 1),
        ));

        let chunk_headers: Vec<String> = (start..end)
            .map(|global_i| {
                let structured = !matches!(out_type, OutputSyntax::Plain);
                let base = sanitize_header_for_output(&headers[global_i], global_i, structured);
                if structured {
                    format!("{} [[IDX:{}]]", base, global_i + 1)
                } else {
                    sanitize_header_preserve_id(&headers[global_i], global_i)
                }
            })
            .collect();

        let raw_chunk_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| {
                (start..end)
                    .map(|i| row.get(i).cloned().unwrap_or_default())
                    .collect()
            })
            .collect();

        let (chunk_data, chunk_line_numbers) = sanitize_chunk_data_with_rows(
            &raw_chunk_data,
            original_line_numbers,
            keineleereninhalte,
        );

        let data_chunk_widths = build_chunk_widths(
            &chunk_headers,
            &chunk_data,
            explizite_breiten,
            start,
            end,
            data_available_total.max(effective_min_column_width() + COLUMN_OVERHEAD + 1),
        );

        let (augmented_headers, augmented_data, mut meta_widths) =
            prepend_meta_columns(&chunk_headers, &chunk_data, &chunk_line_numbers);

        if meta_widths.is_empty() {
            meta_widths = meta_widths_preview;
        }

        let mut augmented_widths = meta_widths;
        augmented_widths.extend(data_chunk_widths);

        let total_overhead = augmented_widths.len() * (COLUMN_OVERHEAD + 1);
        let total_budget = render_width.saturating_sub(total_overhead);

        stretch_last_column_to_fill_budget(&mut augmented_widths, total_budget);

        let table_rows = convert_to_table_rows_with_line_numbers(
            &augmented_headers,
            &augmented_data,
            &augmented_widths,
            &chunk_line_numbers,
        );

        render_rows(render_width, augmented_widths, &table_rows, false, out_type, pretty_output);

        if end < headers.len() {
            println!();
        }

        start = end;
    }
}

fn build_output<'a>(
    tables: &'a Tables,
    render_width: usize,
    column_widths: Vec<usize>,
    line_numbering: bool,
    out_type: OutputSyntax,
    pretty_output: bool,
) -> CliOutput<'a> {
    let mut output = CliOutput::new(tables, out_type);
    output.pretty_output = pretty_output;
    output.color_enabled = out_type.uses_terminal_colors();
    output.table_width = render_width;
    output.column_widths = column_widths;
    output.line_numbering = line_numbering;
    output.one_table = true;
    output
}

fn render_rows(
    render_width: usize,
    column_widths: Vec<usize>,
    table_rows: &[TableRow],
    line_numbering: bool,
    out_type: OutputSyntax,
    pretty_output: bool,
) {
    let tables = Tables::new(Some(100));
    let mut output = build_output(&tables, render_width, column_widths, line_numbering, out_type, pretty_output);

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
    explizite_breiten: &[usize],
) {
    print_table_with_offset(headers, data, row_ranges, 1, explizite_breiten);
}

pub fn print_table_with_offset(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
    original_start_line: usize,
    explizite_breiten: &[usize],
) {
    let sanitized_headers: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| sanitize_header_preserve_id(h, i))
        .collect();

    let term_width = get_terminal_width();
    let available_budget = term_width
        .saturating_sub(1)
        .saturating_sub(sanitized_headers.len() * COLUMN_OVERHEAD);

    let mut column_widths = crate::table_printer::table_utils::compute_column_widths_from_global_mass(
        &sanitized_headers,
        data,
        available_budget,
    );

    for (i, width) in column_widths.iter_mut().enumerate() {
        if let Some(explicit) = get_explicit_width(explizite_breiten, i) {
            *width = clamp_explicit_width(explicit);
        }
    }

    let table_rows = convert_to_table_rows_with_offset(
        &sanitized_headers,
        data,
        &column_widths,
        row_ranges,
        original_start_line,
    );

    let available_total = term_width.saturating_sub(3);
    render_rows(available_total, column_widths, &table_rows, true, OutputSyntax::Plain, false);
}

pub fn print_table_chunked(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
    explizite_breiten: &[usize],
    keineleereninhalte: bool,
) {
    print_table_chunked_with_offset(
        headers,
        data,
        row_ranges,
        explizite_breiten,
        1,
        keineleereninhalte,
    );
}

pub fn print_table_chunked_with_offset(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
    explizite_breiten: &[usize],
    original_start_line: usize,
    keineleereninhalte: bool,
) {
    let term_width = get_terminal_width();
    let available_total = term_width.saturating_sub(3);
    let render_width = available_total;
    let mut start = 0usize;

    while start < headers.len() {
        let end = determine_chunk_end(headers, data, explizite_breiten, start, available_total);

        let chunk_headers: Vec<String> = (start..end)
            .map(|global_i| sanitize_header_preserve_id(&headers[global_i], global_i))
            .collect();

        let raw_chunk_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| {
                (start..end)
                    .map(|i| row.get(i).cloned().unwrap_or_default())
                    .collect()
            })
            .collect();

        let chunk_data = sanitize_chunk_data(&raw_chunk_data, keineleereninhalte);

        let chunk_widths = build_chunk_widths(
            &chunk_headers,
            &chunk_data,
            explizite_breiten,
            start,
            end,
            available_total,
        );

        let table_rows = convert_to_table_rows_with_offset(
            &chunk_headers,
            &chunk_data,
            &chunk_widths,
            row_ranges,
            original_start_line,
        );

        render_rows(render_width, chunk_widths, &table_rows, true, OutputSyntax::Plain, false);

        if end < headers.len() {
            println!();
        }

        start = end;
    }
}

pub fn print_table_auto(headers: &[String], data: &[Vec<String>], row_ranges: &[RowRange]) {
    let sanitized_headers: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| sanitize_header_preserve_id(h, i))
        .collect();

    let layout = build_table_layout(&sanitized_headers, data);
    let table_rows = convert_to_table_rows(&sanitized_headers, data, &layout.column_widths, row_ranges);

    render_rows(layout.term_width, layout.column_widths, &table_rows, true, OutputSyntax::Plain, false);
}
