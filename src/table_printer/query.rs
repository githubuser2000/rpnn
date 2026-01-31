// table_printer/query.rs
use rusqlite::Connection;
use crate::cli::TextBereich;
use crate::column_manager::{get_column_names, build_column_query};
use crate::data_fetcher::fetch_data_with_stats;
use crate::table_printer::printer::print_table_chunked;
use unicode_width::UnicodeWidthStr;  // Import hinzugefügt

// --- Query-Funktion ---
pub fn query_column_by_index(conn: &Connection, bereich: TextBereich) -> Result<(), Box<dyn std::error::Error>> {
    let column_names = get_column_names(conn)?;
    let (query, headers) = build_column_query(&column_names, bereich.clone())?;
    
    // Berechne Header-Längen mit Unicode-Unterstützung
    let header_lengths: Vec<usize> = headers.iter()
        .map(|h| h.chars().count())  // Einfacher für Test
        .collect();
    
    let (data, _max_lengths) = fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    // ÄNDERUNG: zeilen_bereiche direkt übergeben
    print_table_chunked(&headers, &data, &bereich.zeilen_bereiche);
    println!();
    Ok(())
}
