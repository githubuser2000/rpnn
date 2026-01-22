use comfy_table::{ColumnConstraint, ContentArrangement, Table, Width};
use terminal_size::{terminal_size, Width as TermWidth};
use crate::cli::TextBereich;
use crate::column_manager::{get_column_names, build_column_query};
use rusqlite::Connection;
const COLUMN_OVERHEAD: usize = 5;
const MIN_COLUMN_WIDTH: usize = 10;
const MAX_COLUMNS_CAP: usize = 6;

#[derive(Copy, Clone)]
enum ColumnKind {
    Id,
    Number,
    ShortText,
    LongText,
}

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
        ColumnKind::Id        => 6,
        ColumnKind::Number    => 8,
        ColumnKind::ShortText => 14,
        ColumnKind::LongText  => 20,
    }
}


fn compute_columns_per_table(
    term_width: usize,
    headers: &[String],
    max_lengths: &[usize],
) -> usize {
    if headers.is_empty() {
        return 1;
    }

    let effective_widths: Vec<usize> = headers
        .iter()
        .zip(max_lengths)
        .map(|(h, &w)| {
            let kind = infer_column_kind(h);
            let min = min_width_for_kind(kind).max(MIN_COLUMN_WIDTH);
            w.max(min) + COLUMN_OVERHEAD
        })
        .collect();

    let avg_width =
        effective_widths.iter().sum::<usize>() / effective_widths.len();

    let cols = term_width / avg_width;

    cols.clamp(1, MAX_COLUMNS_CAP)
}


pub fn print_table_chunked(
    headers: &[String],
    data: &[Vec<String>],
    max_lengths: &[usize],
) {
    let term_width = terminal_size()
        .map(|(TermWidth(w), _)| w as usize)
        .unwrap_or(100);

    let cols_per_table =
        compute_columns_per_table(term_width, headers, max_lengths);

    for start in (0..headers.len()).step_by(cols_per_table) {
        let end = (start + cols_per_table).min(headers.len());

        println!();
        // println!("Spalten {}–{}", start + 1, end);

        let chunk_headers = &headers[start..end];
        let chunk_max_lengths = &max_lengths[start..end];

        let chunk_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| row[start..end].to_vec())
            .collect();

        print_table(chunk_headers, chunk_data, chunk_max_lengths);
    }
}

pub fn print_table(
    headers: &[String],
    data: Vec<Vec<String>>,
    max_lengths: &[usize],
) {
    let mut table = Table::new();

    let term_width = terminal_size()
        .map(|(TermWidth(w), _)| w)
        .unwrap_or(100);

    table
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(term_width)
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_header(headers);

    let total: usize = max_lengths.iter().sum::<usize>().max(1);

    for (i, len) in max_lengths.iter().enumerate() {
        let percent = ((*len as f32 / total as f32) * 100.0) as u16;
        table
            .column_mut(i)
            .unwrap()
            .set_constraint(ColumnConstraint::UpperBoundary(
                Width::Percentage(percent.max(5)),
            ));
    }

    for row in data {
        table.add_row(row);
    }

    if !headers.is_empty() {
        println!("{table}");
    } else {
        println!("Keine Daten für den gewählten Bereich gefunden.");
    }
}

pub fn query_column_by_index(
    conn: &Connection,
    bereich: TextBereich,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::column_manager::{get_column_names, build_column_query};
    use crate::data_fetcher::fetch_data_with_stats;
    
    let column_names = get_column_names(conn)?;
    let (query, headers) = build_column_query(&column_names, bereich)?;

    let header_lengths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    let (data, max_lengths) =
        fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    print_table_chunked(&headers, &data, &max_lengths);
    println!();
    

    Ok(())
}
