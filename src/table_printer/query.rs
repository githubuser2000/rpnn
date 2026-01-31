// table_printer/query.rs
use rusqlite::Connection;
use crate::cli::TextBereich;
use crate::column_manager::{get_column_names, build_column_query};
use crate::data_fetcher::fetch_data_with_stats;
use crate::table_printer::printer::print_table_chunked;
use unicode_width::UnicodeWidthStr;
use std::process;

// --- Query-Funktion ---
pub fn query_column_by_index(
    conn: &Connection,
    mut bereich: TextBereich, // mutable copy
    wurde_spalten_gesucht: bool,
) -> Result<TextBereich, Box<dyn std::error::Error>> { // <-- TextBereich zurückgeben
    let column_names = get_column_names(conn)?;
    
    let (query, headers) = build_column_query(&column_names, &mut bereich, wurde_spalten_gesucht)?;
    if !bereich.spalten_gefunden {
        println!("❌ FEHLER: Spalten wurden nicht gefunden!");
        process::exit(1);
    }
    
    // Berechne Header-Längen mit Unicode-Unterstützung
    let header_lengths: Vec<usize> = headers.iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();
    
    let (data, _max_lengths) =
    fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    // KORREKTUR: Übergabe von zeilen_bereiche statt start_row_num
    print_table_chunked(&headers, &data, &bereich.zeilen_bereiche);
    println!("Spalten wurden gefunden: {}", bereich.spalten_gefunden);
    Ok(bereich)
}
