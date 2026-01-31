// file: column_manager.rs
use rusqlite::Connection;
use crate::cli::TextBereich;

pub fn get_column_names(conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
    let names = stmt
        .query_map([], |row| row.get(1))?
        .collect::<Result<Vec<String>, _>>()?;
    Ok(names)
}

pub fn build_column_query(
    column_names: &[String],
    bereich: TextBereich,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    println!("=== 🔍 START Build Query ===");
    println!("📊 Eingabe-Daten:");
    println!("  - Verfügbare Spalten insgesamt: {}", column_names.len());
    println!("  - TextBereich Struktur: {:?}", bereich);
    println!("  - Spaltenbereiche: {:?}", bereich.spalten_bereiche);
    println!("  - von_spalte: {}, bis_spalte: {}", bereich.von_spalte, bereich.bis_spalte);
    println!("  - Zeilenbereiche: {:?}", bereich.zeilen_bereiche);
    println!("  - von_zeile: {}, bis_zeile: {}", bereich.von_zeile, bereich.bis_zeile);
    
    // Debug: Zeige erste 10 verfügbare Spaltennamen
    println!("\n📋 Verfügbare Spaltennamen (erste 15):");
    for (i, name) in column_names.iter().take(15).enumerate() {
        println!("  {}. \"{}\"", i + 1, name);
    }
    if column_names.len() > 15 {
        println!("  ... und {} weitere", column_names.len() - 15);
    }
    
    let mut selected_names = Vec::new();
    let mut spalten_nummern = Vec::new();

    println!("\n=== 🔧 SPALTENAUSWAHL ===");
    // KONSISTENTE LOGIK: PRIORITÄT für diskrete Spaltenbereiche
    if !bereich.spalten_bereiche.is_empty() {
        println!("📊 MODUS: Diskrete Spaltenbereiche");
        println!("  Eingabe-Bereiche: {:?}", bereich.spalten_bereiche);
        
        for &(von, bis) in &bereich.spalten_bereiche {
            println!("  Verarbeite Bereich {} bis {}", von, bis);
            for i in von..=bis {
                spalten_nummern.push(i);
                println!("    → Hinzugefügt Spalte {}", i);
            }
        }
        
        // Sortieren und Duplikate entfernen
        let original_count = spalten_nummern.len();
        spalten_nummern.sort();
        spalten_nummern.dedup();
        
        println!("\n  📈 Spaltennummern-Verarbeitung:");
        println!("    - Vor Sortierung/Dedup: {} Nummern", original_count);
        println!("    - Nach Dedup: {} eindeutige Nummern", spalten_nummern.len());
        println!("    - Eindeutige Spaltennummern: {:?}", spalten_nummern);
        
        if original_count > spalten_nummern.len() {
            println!("    ⚠️  {} Duplikate wurden entfernt", original_count - spalten_nummern.len());
        }
    } 
    // FALLBACK: Kontinuierlicher Spaltenbereich (nur wenn keine diskreten Bereiche)
    else if bereich.von_spalte > 0 && bereich.bis_spalte > 0 {
        println!("📊 MODUS: Kontinuierlicher Spaltenbereich (Fallback)");
        println!("  Keine diskreten Spaltenbereiche gefunden");
        println!("  Verwende kontinuierlichen Bereich: {} bis {}", 
                 bereich.von_spalte, bereich.bis_spalte);
        
        // Validate column indices
        if bereich.von_spalte == 0 || bereich.bis_spalte == 0 {
            return Err("Spaltenindizes müssen bei 1 beginnen".into());
        }

        if bereich.von_spalte > bereich.bis_spalte {
            return Err("Startspalte muss kleiner oder gleich Endspalte sein".into());
        }

        // Collect column numbers for continuous range
        println!("  Hinzugefügte Spaltennummern:");
        for i in bereich.von_spalte..=bereich.bis_spalte {
            spalten_nummern.push(i);
            println!("    → Spalte {}", i);
        }
    }
    // Fall 3: Keine Spalten angegeben - Standard auf Spalte 1
    else {
        println!("📊 MODUS: Standard-Spalte (Fallback)");
        println!("⚠️  Keine Spalten angegeben - verwende Spalte 1 als Standard");
        spalten_nummern.push(1);
        println!("  → Hinzugefügt Spalte 1");
    }
    
    println!("\n=== 🔎 SPALTENNAMEN ZUORDNUNG ===");
    // Jetzt: Überprüfen ob alle Spaltennummern existieren und Namen holen
    println!("  Verarbeite {} Spaltennummern:", spalten_nummern.len());
    
    for (index, &nummer) in spalten_nummern.iter().enumerate() {
        println!("  [{}/{}] Prüfe Spalte {}:", index + 1, spalten_nummern.len(), nummer);
        
        if nummer == 0 {
            println!("    ❌ FEHLER: Spaltennummer 0 ist ungültig (muss >= 1 sein)");
            return Err("Spaltennummer 0 ist ungültig".into());
        }
        
        if nummer > column_names.len() {
            println!("    ❌ FEHLER: Spaltennummer {} existiert nicht", nummer);
            println!("    ℹ️  Tabelle hat nur {} Spalten", column_names.len());
            return Err(format!("Spaltennummer {} existiert nicht (Tabelle hat {} Spalten)", 
                               nummer, column_names.len()).into());
        }
        
        if let Some(name) = column_names.get(nummer.saturating_sub(1)) {
            let quoted_name = format!("\"{}\"", name.replace("\"", "\"\""));
            selected_names.push(quoted_name.clone());
            println!("    ✅ OK: Spalte {} = '{}' → SQL: {}", nummer, name, quoted_name);
        } else {
            println!("    ❌ FEHLER: Spaltennummer {} nicht gefunden", nummer);
            return Err(format!("Spaltennummer {} nicht gefunden", nummer).into());
        }
    }
    
    let columns_clause = selected_names.join(", ");
    println!("\n=== ✅ ERGEBNIS SPALTENAUSWAHL ===");
    println!("📋 Ausgewählte Spalten ({} total):", selected_names.len());
    for (i, col) in selected_names.iter().enumerate() {
        println!("  {}. {}", i + 1, col);
    }
    println!("📝 SQL Columns Clause: {}", columns_clause);

    println!("\n=== 🔧 ZEILENAUSWAHL ===");
    // KONSISTENTE LOGIK für Zeilen: PRIORITÄT für diskrete Zeilenbereiche
    let query = if !bereich.zeilen_bereiche.is_empty() {
        // PRIORITÄT: Use zeilen_bereiche if it has entries (individual rows/ranges)
        println!("📊 MODUS: Diskrete Zeilenbereiche");
        println!("  Eingabe-Bereiche: {:?}", bereich.zeilen_bereiche);
        println!("  Anzahl Bereiche: {}", bereich.zeilen_bereiche.len());
        
        let result = build_query_with_row_ranges_enhanced(&columns_clause, &bereich.zeilen_bereiche);
        println!("  ✅ Query generiert");
        result
    } else if bereich.von_zeile > 0 && bereich.bis_zeile > 0 {
        // FALLBACK: Use continuous row range (nur wenn keine diskreten Bereiche)
        println!("📊 MODUS: Kontinuierlicher Zeilenbereich (Fallback)");
        println!("  Keine diskreten Zeilenbereiche gefunden");
        println!("  Verwende kontinuierlichen Bereich: {} bis {}", 
                 bereich.von_zeile, bereich.bis_zeile);
        
        let result = build_query_with_continuous_range(&columns_clause, &bereich);
        println!("  ✅ Query generiert");
        result
    } else {
        // STANDARD: Alle Zeilen
        println!("📊 MODUS: Alle Zeilen (Standard)");
        println!("  Keine Zeileneinschränkung - verwende alle Zeilen");
        let query = format!("SELECT {} FROM csv_data", columns_clause);
        println!("  ✅ Query generiert: {}", query);
        Ok(query)
    }?;

    println!("\n=== ✅ FINALE QUERY ===");
    println!("📋 Endgültige SQL-Query:");
    println!("{}", query);
    println!("=================================");

    Ok((query, selected_names))
}

fn build_query_with_continuous_range(
    columns_clause: &str,
    bereich: &TextBereich,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("  📐 Berechne kontinuierlichen Zeilenbereich:");
    println!("    - Startzeile: {}", bereich.von_zeile);
    println!("    - Endzeile: {}", bereich.bis_zeile);
    
    // Validate row indices
    if bereich.von_zeile == 0 {
        println!("    ❌ FEHLER: Zeilenindizes müssen bei 1 beginnen");
        return Err("Zeilenindizes müssen bei 1 beginnen".into());
    }

    if bereich.bis_zeile < bereich.von_zeile {
        println!("    ❌ FEHLER: Endzeile {} < Startzeile {}", bereich.bis_zeile, bereich.von_zeile);
        return Err("Endzeile muss größer oder gleich Startzeile sein".into());
    }

    // Calculate number of rows (inclusive range)
    let anzahl = bereich.bis_zeile - bereich.von_zeile + 1;
    println!("    - Anzahl Zeilen (inklusive): {}", anzahl);

    if anzahl == 0 {
        println!("    ❌ FEHLER: Ungültiger Zeilenbereich (Anzahl = 0)");
        return Err("Ungültiger Zeilenbereich".into());
    }

    // Offset berechnen (0-basiert)
    let offset = bereich.von_zeile.saturating_sub(1);
    println!("    - SQL OFFSET (0-basiert): {}", offset);
    println!("    - SQL LIMIT: {}", anzahl);

    // Build query for continuous range
    let query = format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        columns_clause,
        anzahl,
        offset
    );
    
    println!("    ✅ Kontinuierliche Query generiert");
    Ok(query)
}

// ENHANCED VERSION: Besser für einzelne Zeilen und Bereiche
fn build_query_with_row_ranges_enhanced(
    columns_clause: &str,
    zeilen_bereiche: &[(usize, usize)],
) -> Result<String, Box<dyn std::error::Error>> {
    println!("  📐 Verarbeite diskrete Zeilenbereiche:");
    
    if zeilen_bereiche.is_empty() {
        println!("    ❌ FEHLER: Zeilenbereiche dürfen nicht leer sein");
        return Err("Zeilenbereiche dürfen nicht leer sein".into());
    }

    println!("    - Eingabe: {} Zeilenbereiche", zeilen_bereiche.len());
    println!("    - Bereiche: {:?}", zeilen_bereiche);
    
    // Unterscheide zwischen einzelnen Zeilen und Bereichen
    let mut einzelne_zeilen = Vec::new();
    let mut bereiche = Vec::new();
    
    for &(start, end) in zeilen_bereiche {
        if start == end {
            // Einzelne Zeile
            einzelne_zeilen.push(start);
            println!("      → Einzelne Zeile: {}", start);
        } else {
            // Bereich
            bereiche.push((start, end));
            println!("      → Bereich: {} bis {} ({} Zeilen)", start, end, end - start + 1);
        }
    }
    
    println!("\n    📊 Analyse:");
    println!("      - {} einzelne Zeilen", einzelne_zeilen.len());
    println!("      - {} Bereiche", bereiche.len());
    
    // OPTIMIERUNG: Wenn nur einzelne Zeilen, optimierte Query
    if bereiche.is_empty() && !einzelne_zeilen.is_empty() {
        println!("    ⚡ MODUS: Optimierte Query für einzelne Zeilen");
        
        let row_numbers_str = einzelne_zeilen
            .iter()
            .map(|n| (n - 1).to_string())
            .collect::<Vec<_>>()
            .join(", ");
            
        println!("      - 0-basierte Zeilennummern: {}", row_numbers_str);
        println!("      - Anzahl Zeilen: {}", einzelne_zeilen.len());
        
        let query = format!(
            "SELECT {} FROM (
                SELECT *, ROW_NUMBER() OVER (ORDER BY rowid) - 1 as row_num
                FROM csv_data
            ) numbered_data
            WHERE row_num IN ({})
            ORDER BY row_num",
            columns_clause, row_numbers_str
        );
        
        println!("      ✅ Optimierte Query generiert");
        return Ok(query);
    }
    
    // Allgemeiner Fall (Bereiche oder gemischt)
    println!("    📋 MODUS: Allgemeine Query für gemischte Bereiche");
    
    let mut all_row_numbers = Vec::new();
    
    // Einzelne Zeilen hinzufügen
    for &zeile in &einzelne_zeilen {
        all_row_numbers.push(zeile);
        println!("      → Hinzugefügt einzelne Zeile: {}", zeile);
    }
    
    // Bereiche hinzufügen
    for (start, end) in &bereiche {
        let anzahl_im_bereich = end - start + 1;
        println!("      → Verarbeite Bereich {}..{} ({} Zeilen)", start, end, anzahl_im_bereich);
        
        for row in *start..=*end {
            all_row_numbers.push(row);
        }
    }
    
    let vor_dedup = all_row_numbers.len();
    println!("    📈 Vor Dedup: {} Zeilennummern", vor_dedup);
    
    // Entferne Duplikate und sortiere
    all_row_numbers.sort();
    all_row_numbers.dedup();
    
    println!("    📊 Nach Dedup:");
    println!("      - Eindeutige Zeilen: {}", all_row_numbers.len());
    println!("      - Entfernte Duplikate: {}", vor_dedup - all_row_numbers.len());
    
    if all_row_numbers.is_empty() {
        println!("    ❌ FEHLER: Keine gültigen Zeilen ausgewählt");
        return Err("Keine gültigen Zeilen ausgewählt".into());
    }
    
    // Konvertiere zu 0-basierten Nummern für SQL
    let row_numbers_str = all_row_numbers
        .iter()
        .map(|n| (n - 1).to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!("    📝 0-basierte Zeilennummern für SQL: {}", row_numbers_str);
    println!("    📋 Gesamt: {} eindeutige Zeilen", all_row_numbers.len());

    let query = format!(
        "SELECT {} FROM (
            SELECT *, ROW_NUMBER() OVER (ORDER BY rowid) - 1 as row_num
            FROM csv_data
        ) numbered_data
        WHERE row_num IN ({})
        ORDER BY row_num",
        columns_clause, row_numbers_str
    );

    println!("      ✅ Allgemeine Query generiert");
    Ok(query)
}

// Neue Funktion, die direkt mit Spaltennummern arbeitet
pub fn build_column_query_with_specific_columns(
    column_names: &[String],
    spalten_nummern: &[usize],
    zeilen_bereiche: &[(usize, usize)],
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    println!("🔍 Baue Query mit spezifischen Spaltennummern: {:?}", spalten_nummern);
    
    let mut selected_names = Vec::new();
    
    // Überprüfen ob Spaltennummern existieren
    for &nummer in spalten_nummern {
        if nummer == 0 || nummer > column_names.len() {
            return Err(format!("Spaltennummer {} existiert nicht (Tabelle hat {} Spalten)", 
                               nummer, column_names.len()).into());
        }
        
        if let Some(name) = column_names.get(nummer.saturating_sub(1)) {
            selected_names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
        } else {
            return Err(format!("Spaltennummer {} nicht gefunden", nummer).into());
        }
    }
    
    let columns_clause = selected_names.join(", ");
    println!("✅ Ausgewählte Spalten: {}", columns_clause);

    // Zeilenauswahl bestimmen
    let query = if !zeilen_bereiche.is_empty() {
        build_query_with_row_ranges_enhanced(&columns_clause, zeilen_bereiche)
    } else {
        // Standardmäßig alle Zeilen
        Ok(format!("SELECT {} FROM csv_data", columns_clause))
    }?;

    println!("✅ Generierte Query: {}", query);

    Ok((query, selected_names))
}
