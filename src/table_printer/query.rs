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
) -> Result<TextBereich, Box<dyn std::error::Error>> { // <-- TextBereich zurückgeben
    let column_names = get_column_names(conn)?;
    
    let (query, headers) = build_column_query(&column_names, &mut bereich)?;
    println!("Headerslänge {}", headers.len());
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
        /*
        if !spaltenreihenfolgeundnurdiese.is_empty() {
            bereich.spalten_bereiche = sort_by_indices(&bereich.spalten_bereiche, &spaltenreihenfolgeundnurdiese).unwrap();
        }
        */
    print_table_chunked(&headers, &data, &bereich.zeilen_bereiche);
    println!("Spalten wurden gefunden: {}", bereich.spalten_gefunden);
    Ok(bereich)
}

fn sort_by_indices<T: Clone>(values: &Vec<T>, indices: &[usize]) -> Result<Vec<T>, String> {
    // Wenn der Index-Vektor leer ist, gibt einen leeren Vektor zurück
    if indices.is_empty() {
        return Ok(Vec::new());
    }
    
    // Finde den maximalen Index
    let max_index = indices.iter().max().copied().unwrap_or(0);
    
    // Überprüfe, ob alle Indizes gültig sind
    if max_index >= values.len() {
        return Err(format!(
            "Index {} ist außerhalb der Grenzen (0..{})",
            max_index,
            values.len() - 1
        ));
    }
    
    // Erstelle den sortierten Vektor basierend auf den Indizes
    let result = indices
        .iter()
        .map(|&i| {
            // Diese Prüfung haben wir bereits oben durchgeführt, 
            // aber zur Sicherheit behalten wir sie bei
            if i >= values.len() {
                panic!("Unerwarteter Fehler: Index {} außerhalb der Grenzen", i);
            }
            values[i].clone()
        })
        .collect();
    
    Ok(result)
}
