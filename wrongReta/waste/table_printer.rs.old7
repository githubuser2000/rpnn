use comfy_table::{ColumnConstraint, ContentArrangement, Table, Width};
use terminal_size::{terminal_size, Width as TermWidth};
use crate::cli::TextBereich;
use crate::column_manager::{get_column_names, build_column_query};
use crate::data_fetcher::fetch_data_with_stats;
use rusqlite::Connection;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

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

// --- Print one table chunk ---
pub fn print_table(headers: &[String], data: Vec<Vec<String>>, max_lengths: &[usize]) {
    let mut table = Table::new();
    let term_width = terminal_size().map(|(TermWidth(w), _)| w).unwrap_or(100);

    let display_headers: Vec<String> = headers.iter().map(|h| truncate_cell(h, MAX_COLUMN_WIDTH)).collect();
    table.set_header(&display_headers);

    table
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(term_width)
        .load_preset(comfy_table::presets::UTF8_FULL);

    let total: usize = max_lengths.iter().sum::<usize>().max(1);
    for (i, &len) in max_lengths.iter().enumerate() {
        let percent = (len as f32 / total as f32 * 100.0) as u16;
        table.column_mut(i).unwrap().set_constraint(ColumnConstraint::UpperBoundary(Width::Percentage(percent.max(5))));
    }

    for row in data {
        let truncated_row: Vec<String> = row.iter().map(|c| truncate_cell(c, MAX_COLUMN_WIDTH)).collect();
        table.add_row(truncated_row);
    }

    if !headers.is_empty() { println!("{table}"); }
}

// --- Print table in automatic chunks ---
pub fn print_table_chunked(headers: &[String], data: &[Vec<String>]) {
    let term_width = terminal_size().map(|(TermWidth(w), _)| w as usize).unwrap_or(100);

    // max_lengths automatisch berechnen
    let mut max_lengths: Vec<usize> = headers.iter().map(|h| UnicodeWidthStr::width(h.as_str())).collect();
    for row in data {
        for (i, cell) in row.iter().enumerate() {
            if i < max_lengths.len() {
                max_lengths[i] = max_lengths[i].max(UnicodeWidthStr::width(cell.as_str()));
            }
        }
    }

    let mut start = 0;
    while start < headers.len() {
        let remaining_headers = &headers[start..];
        let remaining_lengths = &max_lengths[start..];

        let capped_lengths: Vec<usize> = remaining_lengths.iter().map(|&w| w.min(MAX_COLUMN_WIDTH)).collect();
        let cols_per_table = compute_columns_per_table(term_width, remaining_headers, &capped_lengths);
        let end = (start + cols_per_table).min(headers.len());

        let chunk_headers = &headers[start..end];
        let chunk_max_lengths = &capped_lengths[..end - start];

        let chunk_data: Vec<Vec<String>> = data.iter()
            .map(|row| {
                let mut r = row[start..end].to_vec();
                r.resize(end - start, "".to_string());
                r
            })
            .collect();

        print_table(chunk_headers, chunk_data, chunk_max_lengths);

        start = end;
    }
}

// --- Query-Funktion bleibt gleich ---
pub fn query_column_by_index(conn: &Connection, bereich: TextBereich) -> Result<(), Box<dyn std::error::Error>> {
    let column_names = get_column_names(conn)?;
    let (query, headers) = build_column_query(&column_names, bereich)?;
    let header_lengths: Vec<usize> = headers.iter().map(|h| UnicodeWidthStr::width(h.as_str())).collect();
    let (data, _max_lengths) = fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    print_table_chunked(&headers, &data);
    println!();
    Ok(())
}
