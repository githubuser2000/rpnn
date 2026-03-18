// table_printer/query.rs
use std::collections::BTreeSet;
use std::process;
use rusqlite::Connection;
use crate::cli::TextBereich;
use crate::column_manager::{get_column_names, build_column_query};
use crate::data_fetcher::fetch_data_with_stats;
use crate::table_printer::printer::print_table_chunked;
use unicode_width::UnicodeWidthStr;
use crate::generated_columns::{apply_generated_columns, ParametersMain};
// --- Query-Funktion ---

pub fn query_column_by_index(
    conn: &Connection,
    mut bereich: TextBereich,
    generated_befehle: &BTreeSet<String>,
    parameters_main: &ParametersMain,
) -> Result<TextBereich, Box<dyn std::error::Error>> {
   let column_names = get_column_names(conn)?;
    
    let (query, headers) = build_column_query(&column_names, &mut bereich)?;
    println!("Headerslänge vor Sortierung: {}", headers.len());
    
    if !bereich.spalten_gefunden {
        println!("❌ FEHLER: Spalten wurden nicht gefunden!");
        process::exit(1);
    }
    
    // Berechne Header-Längen mit Unicode-Unterstützung
    let header_lengths: Vec<usize> = headers.iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();
    
    let (data, _max_lengths) = fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    /* DEBUG: Zeige aktuelle Status
    println!("=== 🔍 STATUS VOR SORTIERUNG ===");
    println!("Spaltenreihenfolge: {:?}", bereich.spaltenreihenfolgeundnurdiese);
    println!("Headers vor Sortierung: {} Stück", headers.len());
    println!("Daten vor Sortierung: {} Zeilen", data.len());
    */
    // SORTIERUNG DER SPALTEN: NUR wenn spaltenreihenfolgeundnurdiese befüllt ist
   let (mut final_headers, mut final_data) =
        if !bereich.spaltenreihenfolgeundnurdiese.is_empty() {
            let null_basierte_indizes: Vec<usize> = bereich
                .spaltenreihenfolgeundnurdiese
                .iter()
                .map(|&i| if i == 0 { 0 } else { i - 1 })
                .collect();

            let sorted_headers = sort_by_indices(&headers, &null_basierte_indizes)
                .unwrap_or_else(|_| headers.clone());

            let sorted_data: Vec<Vec<String>> = data
                .iter()
                .map(|row| {
                    sort_by_indices(row, &null_basierte_indizes)
                        .unwrap_or_else(|_| row.clone())
                })
                .collect();

            (sorted_headers, sorted_data)
        } else {
            (headers.clone(), data.clone())
        };

    apply_generated_columns(
        &mut final_headers,
        &mut final_data,
        &bereich,
        generated_befehle,
        parameters_main,
    )?;
 
   // Kontroll-Ausgabe
    //println!("=== 📊 FINALE DATEN ===");
    //println!("Finale Headers: {} Spalten", final_headers.len());
    for (i, header) in final_headers.iter().enumerate() {
        let original_index = if !bereich.spaltenreihenfolgeundnurdiese.is_empty() && i < bereich.spaltenreihenfolgeundnurdiese.len() {
            format!("(ursprünglich Spalte {})", bereich.spaltenreihenfolgeundnurdiese[i])
        } else {
            "".to_string()
        };
        println!("  Ausgabe-Spalte {} {}: '{}'", i + 1, original_index, header);
    }
    
    println!("Finale Daten: {} Zeilen", final_data.len());
    if !final_data.is_empty() {
        println!("Erste Zeile hat {} Spalten", final_data[0].len());
        for (i, value) in final_data[0].iter().enumerate() {
            let spalten_nr = if !bereich.spaltenreihenfolgeundnurdiese.is_empty() && i < bereich.spaltenreihenfolgeundnurdiese.len() {
                format!("(Spalte {})", bereich.spaltenreihenfolgeundnurdiese[i])
            } else {
                "".to_string()
            };
            println!("  Wert {} {}: '{}'", i + 1, spalten_nr, value);
        }
    }
    
    print_table_chunked(&final_headers, &final_data, &bereich.zeilen_bereiche, &bereich.breiten);
    //println!("Spalten wurden gefunden: {}", bereich.spalten_gefunden);
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
