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

const CHUNK_MIN_COLUMN_WIDTH: usize = 21;
const POTENZ_HEADER: &str = "P";
const ZEILE_HEADER: &str = "Z";

fn effective_min_column_width() -> usize {
    CHUNK_MIN_COLUMN_WIDTH.max(MIN_COLUMN_WIDTH)
}

fn clamp_chunk_width(width: usize) -> usize {
    width.clamp(effective_min_column_width(), MAX_COLUMN_WIDTH)
}

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

fn row_has_visible_content(row: &[String]) -> bool {
    row.iter().any(|cell| {
        cell.lines()
            .map(str::trim)
            .any(|line| line.chars().count() > 2)
    })
}

fn sanitize_chunk_data_with_rows(
    chunk_data: &[Vec<String>],
    row_numbers: &[usize],
    keineleereninhalte: bool,
) -> (Vec<Vec<String>>, Vec<usize>) {
    if !keineleereninhalte {
        return (chunk_data.to_vec(), row_numbers.to_vec());
    }

    let mut new_data = Vec::new();
    let mut new_rows = Vec::new();

    for (row, &num) in chunk_data.iter().zip(row_numbers.iter()) {
        let cleaned_row: Vec<String> = row
            .iter()
            .map(|cell| filter_small_lines_in_cell(cell))
            .collect();

        if row_has_visible_content(&cleaned_row) {
            new_data.push(cleaned_row);
            new_rows.push(num);
        }
    }

    (new_data, new_rows)
}

fn sanitize_header_preserve_id(header: &str, global_index: usize) -> String {
    let trimmed = header.trim();

    if trimmed.is_empty() {
        return format!("SQL-Spalte {}", global_index + 1);
    }

    trimmed.to_string()
}

fn is_special_power(n: usize) -> bool {
    if n < 4 || n == 8 {
        return false;
    }

    let mut base = 2usize;
    while base.saturating_mul(base) <= n {
        let mut value = base.saturating_mul(base);

        while value < n {
            match value.checked_mul(base) {
                Some(next) => value = next,
                None => break,
            }
        }

        if value == n {
            return true;
        }

        base += 1;
    }

    false
}

fn next_special_power(after: usize) -> usize {
    let mut candidate = after.saturating_add(1);

    loop {
        if is_special_power(candidate) {
            return candidate;
        }
        candidate = candidate.saturating_add(1);
    }
}

fn power_bucket_for_line(line_number: usize) -> usize {
    if line_number == 0 {
        return 0;
    }

    let mut bucket = 1usize;
    let mut boundary = 4usize;

    while line_number > boundary {
        bucket += 1;
        boundary = next_special_power(boundary);
    }

    bucket
}

fn build_power_bucket_strings(line_numbers: &[usize]) -> Vec<String> {
    line_numbers
        .iter()
        .map(|&n| power_bucket_for_line(n).to_string())
        .collect()
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
        .unwrap_or(effective_min_column_width());

    clamp_chunk_width(guessed)
}

fn shrink_widths_to_budget(widths: &mut [usize], chunk_budget: usize, min_width: usize) {
    let mut current_total: usize = widths.iter().sum();

    while current_total > chunk_budget {
        let mut changed = false;

        for w in widths.iter_mut() {
            if *w > min_width && current_total > chunk_budget {
                *w -= 1;
                current_total -= 1;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }
}

fn stretch_last_column_to_fill_budget(widths: &mut [usize], chunk_budget: usize) {
    if widths.is_empty() {
        return;
    }

    let current_total: usize = widths.iter().sum();
    if current_total < chunk_budget {
        let extra = chunk_budget - current_total;
        let last_idx = widths.len() - 1;
        widths[last_idx] += extra;
    }
}

fn fit_widths_exactly_to_budget(widths: &mut [usize], budget: usize, min_width: usize) {
    if widths.is_empty() {
        return;
    }

    shrink_widths_to_budget(widths, budget, min_width);
    stretch_last_column_to_fill_budget(widths, budget);
}

fn determine_chunk_end(
    headers: &[String],
    data: &[Vec<String>],
    explizite_breiten: &[usize],
    start: usize,
    available_total: usize,
) -> usize {
    let min_width = effective_min_column_width();
    let squeeze_threshold = available_total.saturating_mul(2) / 5;

    let mut end = start;
    let mut used = 0usize;

    while end < headers.len() {
        let guessed_width = if let Some(breite) = get_explicit_width(explizite_breiten, end) {
            clamp_chunk_width(breite)
        } else {
            estimate_natural_width_for_chunking(&headers[end], data, end)
        };

        let needed = guessed_width + COLUMN_OVERHEAD + 1;

        if used + needed > available_total {
            let remaining_total = available_total.saturating_sub(used);
            let remaining_content = remaining_total.saturating_sub(COLUMN_OVERHEAD + 1);

            if end > start && remaining_total >= squeeze_threshold && remaining_content >= min_width {
                end += 1;
            } else if end == start {
                end += 1;
            }

            break;
        }

        used += needed;
        end += 1;
    }

    if end <= start {
        (start + 1).min(headers.len())
    } else {
        end
    }
}

fn build_chunk_widths(
    chunk_headers: &[String],
    chunk_data: &[Vec<String>],
    explizite_breiten: &[usize],
    start: usize,
    end: usize,
    available_total: usize,
) -> Vec<usize> {
    let min_width = effective_min_column_width();
    let chunk_overhead = chunk_headers.len() * (COLUMN_OVERHEAD + 1);
    let chunk_budget = available_total.saturating_sub(chunk_overhead);

    let mut chunk_widths =
        compute_column_widths_from_global_mass(chunk_headers, chunk_data, chunk_budget);

    if chunk_widths.len() != chunk_headers.len() {
        chunk_widths.resize(chunk_headers.len(), min_width);
    }

    for width in chunk_widths.iter_mut() {
        *width = clamp_chunk_width(*width);
    }

    for (local_i, global_i) in (start..end).enumerate() {
        if let Some(breite) = get_explicit_width(explizite_breiten, global_i) {
            chunk_widths[local_i] = clamp_chunk_width(breite);
        }
    }

    fit_widths_exactly_to_budget(&mut chunk_widths, chunk_budget, min_width);
    chunk_widths
}

fn build_meta_widths(
    power_buckets: &[String],
    line_numbers: &[usize],
    available_total: usize,
) -> (Vec<usize>, usize) {
    let power_width = power_buckets
        .iter()
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(1)
        .max(POTENZ_HEADER.chars().count());

    let line_width = line_numbers
        .iter()
        .map(|n| n.to_string().chars().count())
        .max()
        .unwrap_or(1)
        .max(ZEILE_HEADER.chars().count());

    let widths = vec![power_width, line_width];
    let overhead = widths.len() * (COLUMN_OVERHEAD + 1);
    let content = widths.iter().sum::<usize>();
    let reserved_total = (content + overhead).min(available_total);

    (widths, reserved_total)
}

fn prepend_meta_columns(
    chunk_headers: &[String],
    chunk_data: &[Vec<String>],
    chunk_line_numbers: &[usize],
) -> (Vec<String>, Vec<Vec<String>>, Vec<usize>) {
    let power_buckets = build_power_bucket_strings(chunk_line_numbers);

    let mut headers = vec![POTENZ_HEADER.to_string(), ZEILE_HEADER.to_string()];
    headers.extend(chunk_headers.iter().cloned());

    let mut data = Vec::with_capacity(chunk_data.len());
    for (idx, row) in chunk_data.iter().enumerate() {
        let mut new_row = Vec::with_capacity(row.len() + 2);
        new_row.push(power_buckets[idx].clone());
        new_row.push(chunk_line_numbers[idx].to_string());
        new_row.extend(row.iter().cloned());
        data.push(new_row);
    }

    let (meta_widths, _) = build_meta_widths(&power_buckets, chunk_line_numbers, usize::MAX);

    (headers, data, meta_widths)
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
    let render_width = available_total;
    let mut start = 0usize;

    while start < headers.len() {
        let all_power_buckets = build_power_bucket_strings(original_line_numbers);
        let (meta_widths_preview, meta_reserved_total) =
            build_meta_widths(&all_power_buckets, original_line_numbers, available_total);

        let data_available_total = available_total.saturating_sub(meta_reserved_total);

        let end = determine_chunk_end(
            headers,
            data,
            explizite_breiten,
            start,
            data_available_total.max(effective_min_column_width() + COLUMN_OVERHEAD + 1),
        );

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

        fit_widths_exactly_to_budget(&mut augmented_widths, total_budget, 1);

        let table_rows = convert_to_table_rows_with_line_numbers(
            &augmented_headers,
            &augmented_data,
            &augmented_widths,
            &chunk_line_numbers,
        );

        render_rows(render_width, augmented_widths, &table_rows, false);

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
) -> CliOutput<'a> {
    let mut output = CliOutput::new(tables, OutputSyntax::Plain);
    output.color_enabled = true;
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
) {
    let tables = Tables::new(Some(100));
    let mut output = build_output(&tables, render_width, column_widths, line_numbering);

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

    render_rows(term_width, column_widths, &table_rows, true);
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
        let end = determine_chunk_end(
            headers,
            data,
            explizite_breiten,
            start,
            available_total,
        );

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

        render_rows(render_width, chunk_widths, &table_rows, true);

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
    let table_rows =
        convert_to_table_rows(&sanitized_headers, data, &layout.column_widths, row_ranges);

    render_rows(layout.term_width, layout.column_widths, &table_rows, true);
}
