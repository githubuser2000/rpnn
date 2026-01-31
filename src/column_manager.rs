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
    println!("🔍 Build query mit Bereich: {:?}", bereich);
    println!("  Spaltenbereiche: {:?}", bereich.spalten_bereiche);
    println!("  von_spalte: {}, bis_spalte: {}", bereich.von_spalte, bereich.bis_spalte);
    
    let mut selected_names = Vec::new();
    let mut spalten_nummern = Vec::new();

    // Fall 1: Diskret definierte Spaltenbereiche (z.B. [(10,10), (18,18), (42,42)])
    if !bereich.spalten_bereiche.is_empty() {
        println!("📊 Verwende diskrete Spaltenbereiche");
        
        for &(von, bis) in &bereich.spalten_bereiche {
            for i in von..=bis {
                spalten_nummern.push(i);
            }
        }
        
        // Sortieren und Duplikate entfernen
        spalten_nummern.sort();
        spalten_nummern.dedup();
        
        println!("📈 Eindeutige Spaltennummern: {:?}", spalten_nummern);
        
        // Überprüfen ob Spaltennummern existieren
        for &nummer in &spalten_nummern {
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
    } 
    // Fall 2: Kontinuierlicher Spaltenbereich (Legacy)
    else if bereich.von_spalte > 0 && bereich.bis_spalte > 0 {
        println!("📊 Verwende kontinuierlichen Spaltenbereich");
        
        // Validate column indices
        if bereich.von_spalte == 0 || bereich.bis_spalte == 0 {
            return Err("Spaltenindizes müssen bei 1 beginnen".into());
        }

        if bereich.von_spalte > bereich.bis_spalte {
            return Err("Startspalte muss kleiner oder gleich Endspalte sein".into());
        }

        // Collect selected column names
        for i in bereich.von_spalte..=bereich.bis_spalte {
            spalten_nummern.push(i);
            
            if let Some(name) = column_names.get(i.saturating_sub(1)) {
                selected_names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
            } else {
                return Err(format!("Spaltennummer {} nicht gefunden", i).into());
            }
        }
    }
    // Fall 3: Keine Spalten angegeben - Standard auf Spalte 1
    else {
        println!("⚠️  Keine Spalten angegeben - verwende Spalte 1 als Standard");
        spalten_nummern.push(1);
        
        if let Some(name) = column_names.get(0) {
            selected_names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
        } else {
            return Err("Tabelle hat keine Spalten".into());
        }
    }
    
    let columns_clause = selected_names.join(", ");
    println!("✅ Ausgewählte Spalten: {}", columns_clause);
    println!("📋 Anzahl ausgewählter Spalten: {}", selected_names.len());

    // Determine which rows to select
    let query = if !bereich.zeilen_bereiche.is_empty() {
        // PRIORITÄT: Use zeilen_bereiche if it has entries (individual rows/ranges)
        println!("🔍 Verwende zeilen_bereiche für Zeilenauswahl: {:?}", bereich.zeilen_bereiche);
        build_query_with_row_ranges_enhanced(&columns_clause, &bereich.zeilen_bereiche)
    } else {
        // FALLBACK: Use continuous row range (von_zeile/bis_zeile)
        println!("📊 Verwende kontinuierlichen Zeilenbereich: {} bis {}", 
                 bereich.von_zeile, bereich.bis_zeile);
        build_query_with_continuous_range(&columns_clause, &bereich)
    }?;

    println!("✅ Generierte Query: {}", query);

    Ok((query, selected_names))
}

fn build_query_with_continuous_range(
    columns_clause: &str,
    bereich: &TextBereich,
) -> Result<String, Box<dyn std::error::Error>> {
    // Validate row indices
    if bereich.von_zeile == 0 {
        return Err("Zeilenindizes müssen bei 1 beginnen".into());
    }

    if bereich.bis_zeile < bereich.von_zeile {
        return Err("Endzeile muss größer oder gleich Startzeile sein".into());
    }

    // Calculate number of rows (inclusive range)
    let anzahl = bereich.bis_zeile - bereich.von_zeile + 1;

    if anzahl == 0 {
        return Err("Ungültiger Zeilenbereich".into());
    }

    // Build query for continuous range
    Ok(format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        columns_clause,
        anzahl,
        bereich.von_zeile.saturating_sub(1)  // OFFSET is 0-based
    ))
}

// ENHANCED VERSION: Besser für einzelne Zeilen und Bereiche
fn build_query_with_row_ranges_enhanced(
    columns_clause: &str,
    zeilen_bereiche: &[(usize, usize)],
) -> Result<String, Box<dyn std::error::Error>> {
    if zeilen_bereiche.is_empty() {
        return Err("Zeilenbereiche dürfen nicht leer sein".into());
    }

    println!("📈 Verarbeite {} Zeilenbereiche", zeilen_bereiche.len());
    
    // Unterscheide zwischen einzelnen Zeilen und Bereichen
    let mut einzelne_zeilen = Vec::new();
    let mut bereiche = Vec::new();
    
    for &(start, end) in zeilen_bereiche {
        if start == end {
            // Einzelne Zeile
            einzelne_zeilen.push(start);
        } else {
            // Bereich
            bereiche.push((start, end));
        }
    }
    
    println!("  - {} einzelne Zeilen", einzelne_zeilen.len());
    println!("  - {} Bereiche", bereiche.len());
    
    // OPTIMIERUNG: Wenn nur einzelne Zeilen, optimierte Query
    if bereiche.is_empty() && !einzelne_zeilen.is_empty() {
        println!("⚡ Verwende optimierte Query für einzelne Zeilen");
        let row_numbers_str = einzelne_zeilen
            .iter()
            .map(|n| (n - 1).to_string())
            .collect::<Vec<_>>()
            .join(", ");
            
        return Ok(format!(
            "SELECT {} FROM (
                SELECT *, ROW_NUMBER() OVER (ORDER BY rowid) - 1 as row_num
                FROM csv_data
            ) numbered_data
            WHERE row_num IN ({})
            ORDER BY row_num",
            columns_clause, row_numbers_str
        ));
    }
    
    // Allgemeiner Fall (Bereiche oder gemischt)
    let mut all_row_numbers = Vec::new();
    
    // Einzelne Zeilen hinzufügen
    all_row_numbers.extend(einzelne_zeilen);
    
    // Bereiche hinzufügen
    for (start, end) in bereiche {
        for row in start..=end {
            all_row_numbers.push(row);
        }
    }
    
    // Entferne Duplikate und sortiere
    all_row_numbers.sort();
    all_row_numbers.dedup();
    
    println!("📋 Insgesamt {} eindeutige Zeilen", all_row_numbers.len());
    
    if all_row_numbers.is_empty() {
        return Err("Keine gültigen Zeilen ausgewählt".into());
    }
    
    // Standard-Query
    let row_numbers_str = all_row_numbers
        .iter()
        .map(|n| (n - 1).to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let query = format!(
        "SELECT {} FROM (
            SELECT *, ROW_NUMBER() OVER (ORDER BY rowid) - 1 as row_num
            FROM csv_data
        ) numbered_data
        WHERE row_num IN ({})
        ORDER BY row_num",
        columns_clause, row_numbers_str
    );

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
