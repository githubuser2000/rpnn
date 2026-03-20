use std::collections::BTreeSet;

use crate::reta_ausgabe::{CliOutput, OutputSyntax, TableRow, Tables};
use crate::table_printer::config::{COLUMN_OVERHEAD, MAX_COLUMN_WIDTH, MIN_COLUMN_WIDTH};
use crate::table_printer::table_utils::{
    build_table_layout,
    compute_column_stats,
    compute_column_widths_from_global_mass,
    convert_to_table_rows,
    convert_to_table_rows_with_line_numbers,
    convert_to_table_rows_with_offset,
    get_terminal_width,
    RowRange,
};

fn get_explicit_width(explizite_breiten: &[usize], index: usize) -> Option<usize> {
    match explizite_breiten.len() {
        0 => None,
        1 => Some(explizite_breiten[0]),
        _ => explizite_breiten.get(index).copied(),
    }
}

fn filter_small_lines_in_cell(cell: &str) -> String {
    cell.lines()
        .map(str::trim)
        .filter(|line| line.chars().count() > 2)
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>()
        .join("\n")
}

fn sanitize_chunk_data(chunk_data: &[Vec<String>], keineleereninhalte: bool) -> Vec<Vec<String>> {
    if !keineleereninhalte {
        return chunk_data.to_vec();
    }

    chunk_data
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| filter_small_lines_in_cell(cell))
                .collect()
        })
        .collect()
}

fn sanitize_header_preserve_id(header: &str, global_index: usize) -> String {
    let trimmed = header.trim();

    if trimmed.is_empty() {
        return format!("SQL-Spalte {}", global_index + 1);
    }

    trimmed.to_string()
}

pub fn print_table_chunked_with_line_numbers(
    headers: &[String],
    data: &[Vec<String>],
    explizite_breiten: &[usize],
    original_line_numbers: &[usize],
    keineleereninhalte: bool,
) {
    let term_width = get_terminal_width();
    let available_total = term_width.saturating_sub(3);
    let mut start = 0usize;

    while start < headers.len() {
        let mut end = start;
        let mut used = 0usize;

        while end < headers.len() {
            let guessed_width = if let Some(breite) = get_explicit_width(explizite_breiten, end) {
                breite.clamp(MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH)
            } else {
                estimate_natural_width_for_chunking(&headers[end], data, end)
            };
            let needed = guessed_width + COLUMN_OVERHEAD + 1;

            if used + needed > available_total {
                if end == start {
                    end += 1;
                }
                break;
            }

            used += needed;
            end += 1;
        }

        if end <= start {
            end = (start + 1).min(headers.len());
        }

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

        let chunk_overhead = chunk_headers.len() * (COLUMN_OVERHEAD + 1);
        let chunk_budget = available_total.saturating_sub(chunk_overhead);

        let mut chunk_widths =
            compute_column_widths_from_global_mass(&chunk_headers, &chunk_data, chunk_budget);

        for (local_i, global_i) in (start..end).enumerate() {
            if let Some(breite) = get_explicit_width(explizite_breiten, global_i) {
                chunk_widths[local_i] = breite.clamp(MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH);
            }
        }

        let current_sum: usize = chunk_widths.iter().sum();
        if current_sum > chunk_budget {
            let mut shrinkable = chunk_widths.clone();
            let mut current_total: usize = shrinkable.iter().sum();

            while current_total > chunk_budget {
                let mut changed = false;

                for w in shrinkable.iter_mut() {
                    if *w > MIN_COLUMN_WIDTH && current_total > chunk_budget {
                        *w -= 1;
                        current_total -= 1;
                        changed = true;
                    }
                }

                if !changed {
                    break;
                }
            }

            chunk_widths = shrinkable;
        }

        let table_rows = convert_to_table_rows_with_line_numbers(
            &chunk_headers,
            &chunk_data,
            &chunk_widths,
            original_line_numbers,
        );

        render_rows(term_width, chunk_widths, &table_rows);

        if end < headers.len() {
            println!();
        }

        start = end;
    }
}

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

pub fn print_table(headers: &[String], data: &[Vec<String>], row_ranges: &[RowRange]) {
    print_table_with_offset(headers, data, row_ranges, 1);
}

pub fn print_table_with_offset(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
    original_start_line: usize,
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

    let column_widths =
        compute_column_widths_from_global_mass(&sanitized_headers, data, available_budget);

    let table_rows = convert_to_table_rows_with_offset(
        &sanitized_headers,
        data,
        &column_widths,
        row_ranges,
        original_start_line,
    );

    render_rows(term_width, column_widths, &table_rows);
}

fn estimate_natural_width_for_chunking(
    header: &String,
    data: &[Vec<String>],
    col_idx: usize,
) -> usize {
    let single_header = vec![header.clone()];

    let single_col_data: Vec<Vec<String>> = data
        .iter()
        .map(|row| vec![row.get(col_idx).cloned().unwrap_or_default()])
        .collect();

    let stats = compute_column_stats(&single_header, &single_col_data);
    let guessed = stats
        .first()
        .map(|s| s.avg_width.ceil() as usize)
        .unwrap_or(MIN_COLUMN_WIDTH);

    guessed.clamp(MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH)
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
    let mut start = 0usize;

    while start < headers.len() {
        let mut end = start;
        let mut used = 0usize;

        while end < headers.len() {
            let guessed_width = if let Some(breite) = get_explicit_width(explizite_breiten, end) {
                breite.clamp(MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH)
            } else {
                estimate_natural_width_for_chunking(&headers[end], data, end)
            };

            let needed = guessed_width + COLUMN_OVERHEAD + 1;

            if used + needed > available_total {
                if end == start {
                    end += 1;
                }
                break;
            }

            used += needed;
            end += 1;
        }

        if end <= start {
            end = (start + 1).min(headers.len());
        }

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

        let chunk_overhead = chunk_headers.len() * (COLUMN_OVERHEAD + 1);
        let chunk_budget = available_total.saturating_sub(chunk_overhead);

        let mut chunk_widths =
            compute_column_widths_from_global_mass(&chunk_headers, &chunk_data, chunk_budget);

        for (local_i, global_i) in (start..end).enumerate() {
            if let Some(breite) = get_explicit_width(explizite_breiten, global_i) {
                chunk_widths[local_i] = breite.clamp(MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH);
            }
        }

        let current_sum: usize = chunk_widths.iter().sum();
        if current_sum > chunk_budget {
            let mut shrinkable = chunk_widths.clone();
            let mut current_total: usize = shrinkable.iter().sum();

            while current_total > chunk_budget {
                let mut changed = false;

                for w in shrinkable.iter_mut() {
                    if *w > MIN_COLUMN_WIDTH && current_total > chunk_budget {
                        *w -= 1;
                        current_total -= 1;
                        changed = true;
                    }
                }

                if !changed {
                    break;
                }
            }

            chunk_widths = shrinkable;
        }

        let table_rows = convert_to_table_rows_with_offset(
            &chunk_headers,
            &chunk_data,
            &chunk_widths,
            row_ranges,
            original_start_line,
        );

        render_rows(term_width, chunk_widths, &table_rows);

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

    render_rows(layout.term_width, layout.column_widths, &table_rows);
}
