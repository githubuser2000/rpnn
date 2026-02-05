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
    mut bereich: TextBereich,
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

    // DEBUG: Zeige aktuelle Status
    println!("=== 🔍 STATUS VOR SORTIERUNG ===");
    println!("Spaltenreihenfolge: {:?}", bereich.spaltenreihenfolgeundnurdiese);
    println!("Headers vor Sortierung: {} Stück", headers.len());
    println!("Daten vor Sortierung: {} Zeilen", data.len());
    
    // SORTIERUNG DER SPALTEN: NUR wenn spaltenreihenfolgeundnurdiese befüllt ist
    let (final_headers, final_data) = if !bereich.spaltenreihenfolgeundnurdiese.is_empty() {
        println!("=== 🔄 STARTE SPALTENSORTIERUNG ===");
        println!("Sortierindizes (1-basiert): {:?}", bereich.spaltenreihenfolgeundnurdiese);
        
        // WICHTIG: Konvertiere 1-basierte Indizes zu 0-basierten
        let null_basierte_indizes: Vec<usize> = bereich.spaltenreihenfolgeundnurdiese
            .iter()
            .map(|&i| {
                if i == 0 {
                    println!("⚠️  WARNUNG: Index 0 gefunden (sollte 1-basiert sein!)");
                    0
                } else {
                    i - 1  // 1 → 0, 2 → 1, 3 → 2, usw.
                }
            })
            .collect();
        
        println!("Sortierindizes (0-basiert): {:?}", null_basierte_indizes);
        
        // 1. Headers (Spaltenüberschriften) sortieren
        println!("➡️  Sortiere Headers...");
        let sorted_headers = match sort_by_indices(&headers, &null_basierte_indizes) {
            Ok(h) => {
                println!("✅ Headers sortiert: {} → {} Spalten", headers.len(), h.len());
                h
            },
            Err(e) => {
                println!("⚠️  Fehler beim Sortieren der Headers: {}", e);
                println!("⚠️  Verwende unsortierte Headers");
                headers.clone()
            }
        };
        
        // 2. Daten sortieren: JEDE ZEILE muss ihre SPALTEN in der gleichen Reihenfolge haben
        println!("➡️  Sortiere Datenzeilen...");
        let sorted_data: Vec<Vec<String>> = data.iter()
            .enumerate()
            .map(|(row_idx, row)| {
                match sort_by_indices(row, &null_basierte_indizes) {
                    Ok(sorted_row) => {
                        if row_idx == 0 {
                            println!("✅ Erste Zeile sortiert: {} → {} Spalten", 
                                     row.len(), sorted_row.len());
                        }
                        sorted_row
                    },
                    Err(e) => {
                        println!("⚠️  Fehler beim Sortieren von Zeile {}: {}", row_idx, e);
                        println!("⚠️  Verwende unsortierte Zeile");
                        row.clone()
                    }
                }
            })
            .collect();
        
        println!("=== ✅ SORTIERUNG ABGESCHLOSSEN ===");
        println!("Sortierte Headers: {} Spalten", sorted_headers.len());
        println!("Sortierte Daten: {} Zeilen", sorted_data.len());
        
        (sorted_headers, sorted_data)
    } else {
        // KEINE SORTIERUNG: Verwende Originaldaten
        println!("=== ℹ️  KEINE SORTIERUNG ===");
        println!("spaltenreihenfolgeundnurdiese ist leer oder nicht vorhanden");
        (headers.clone(), data.clone())
    };
    
    // Kontroll-Ausgabe
    println!("=== 📊 FINALE DATEN ===");
    println!("Finale Headers: {} Spalten", final_headers.len());
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
    
    print_table_chunked(&final_headers, &final_data, &bereich.zeilen_bereiche);
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
