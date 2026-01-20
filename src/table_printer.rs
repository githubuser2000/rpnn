use comfy_table::{ColumnConstraint, ContentArrangement, Table, Width};
use terminal_size::{terminal_size, Width as TermWidth};
use crate::cli::TextBereich;
use crate::column_manager::{get_column_names, build_column_query};
use rusqlite::Connection;

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

    let total: usize = max_lengths.iter().sum();

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

    print_table(&headers, data, &max_lengths);

    Ok(())
}
