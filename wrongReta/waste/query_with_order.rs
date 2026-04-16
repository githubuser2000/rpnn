// src/table_printer/query_with_order.rs
use rusqlite::Connection;
use crate::cli::TextBereich;
use crate::column_manager::build_column_query;
use crate::data_fetcher::fetch_data_with_stats;
use super::print_table;

pub fn query_columns_with_order(
    conn: &Connection,
    bereich: &TextBereich,
    column_names: &[String],
    reihenfolge_indices: &[usize],
    wurde_spalten_gesucht: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 ===== QUERY MIT SPALTENREIHENFOLGE =====");
    
    // 1. Bestimme ALLE selektierten Spalten (wie ursprünglich gedacht)
    let mut alle_selektierten_spalten = Vec::new();
    
    // A: Aus spalten_bereiche (durch Kategorie-Suche oder manuelle Bereiche)
    if !bereich.spalten_bereiche.is_empty() {
        println!("📊 Verwende spalten_bereiche für Selektion");
        for &(von, bis) in &bereich.spalten_bereiche {
            for i in von..=bis {
                alle_selektierten_spalten.push(i);
            }
        }
    }
    // B: Aus von_spalte/bis_spalte (falls spalten_bereiche leer)
    else if bereich.von_spalte <= bereich.bis_spalte && bereich.bis_spalte != usize::MAX {
        println!("📊 Verwende von_spalte/bis_spalte für Selektion");
        for i in bereich.von_spalte..=bereich.bis_spalte {
            alle_selektierten_spalten.push(i);
        }
    }
    // C: Keine Spalten selektiert -> Fehler
    else {
        return Err("❌ KEINE Spalten selektiert! Verwende --spaltenname oder --spaltevon/--spaltebis".into());
    }
    
    // Sortieren und Duplikate entfernen
    alle_selektierten_spalten.sort();
    alle_selektierten_spalten.dedup();
    
    println!("📊 Alle selektierten Spalten (CSV-Nummern): {:?}", alle_selektierten_spalten);
    println!("📊 Anzahl selektierter Spalten: {}", alle_selektierten_spalten.len());
    println!("📊 Gewünschte Reihenfolge (Indizes): {:?}", reihenfolge_indices);
    
    // 2. Reihenfolge auf selektierte Spalten anwenden
    let mut csv_spalten_in_reihenfolge = Vec::new();
    
    for &index in reihenfolge_indices {
        if index == 0 {
            println!("❌ FEHLER: Index 0 ist ungültig (muss bei 1 beginnen)");
            continue;
        }
        
        if index > alle_selektierten_spalten.len() {
            println!("⚠️  WARNUNG: Index {} überspringen (nur {} selektierte Spalten)", 
                     index, alle_selektierten_spalten.len());
            continue;
        }
        
        let csv_spalten_nr = alle_selektierten_spalten[index - 1]; // -1 weil 1-basiert
        csv_spalten_in_reihenfolge.push(csv_spalten_nr);
        println!("  → Reihenfolge-Index {} → CSV-Spalte {}", index, csv_spalten_nr);
    }
    
    if csv_spalten_in_reihenfolge.is_empty() {
        return Err("❌ KEINE gültigen Spalten in der Reihenfolge!".into());
    }
    
    println!("📊 Finale CSV-Spalten in Reihenfolge: {:?}", csv_spalten_in_reihenfolge);
    
    // 3. Temporären Bereich für Query erstellen
    let mut temp_bereich = bereich.clone();
    temp_bereich.spalten_bereiche.clear();
    
    // Jede Spalte als einzelnes Bereichspaar hinzufügen
    for &csv_nr in &csv_spalten_in_reihenfolge {
        temp_bereich.spalten_bereiche.push((csv_nr, csv_nr));
    }
    
    // 4. Query mit diesen Spalten bauen
    let (query, selected_names) = build_column_query(
        column_names,
        &mut temp_bereich,
        wurde_spalten_gesucht
    )?;
    
    println!("📋 SQL Query: {}", query);
    
    // 5. Daten abrufen
    let (all_data, max_lengths) = fetch_data_with_stats(
        conn,
        &query,
        selected_names.len(),
        &vec![0; selected_names.len()],
    )?;
    
    if all_data.is_empty() {
        println!("❌ Keine Daten gefunden!");
        return Ok(());
    }
    
    println!("📊 Gefundene Datenzeilen: {}", all_data.len());
    
    // 6. Headers aufbereiten
    let headers: Vec<String> = selected_names
        .iter()
        .map(|name| name.trim_matches('"').to_string())
        .collect();
    
    // 7. Tabelle ausgeben
    print_table(
        &headers,
        all_data,
        &max_lengths,
        &bereich.zeilen_bereiche
    );
    
    // 8. Debug-Info
    println!("\n📊 ===== DEBUG-INFO =====");
    println!("Original selektierte Spalten: {:?}", alle_selektierten_spalten);
    println!("Reihenfolge angewendet: {:?}", reihenfolge_indices);
    println!("Resultierende CSV-Spalten: {:?}", csv_spalten_in_reihenfolge);
    
    for (i, &csv_nr) in csv_spalten_in_reihenfolge.iter().enumerate() {
        if i < headers.len() {
            println!("  Ausgabe-Position {}: CSV-Spalte {} → '{}'", 
                     i + 1, csv_nr, headers[i]);
        }
    }
    
    Ok(())
}
