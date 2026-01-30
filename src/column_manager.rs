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
    let mut selected_names = Vec::new();

    // Validate column indices
    if bereich.von_spalte == 0 || bereich.bis_spalte == 0 {
        return Err("Column indices must start from 1".into());
    }

    if bereich.von_spalte > bereich.bis_spalte {
        return Err("Start column must be less than or equal to end column".into());
    }

    // Collect selected column names
    for i in bereich.von_spalte..=bereich.bis_spalte {
        if let Some(name) = column_names.get(i.saturating_sub(1)) {
            selected_names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
        } else {
            return Err(format!("Column number {} not found", i).into());
        }
    }

    let columns_clause = selected_names.join(", ");

    // Determine which rows to select
    let query = if !bereich.zeilen_bereiche.is_empty() {
        // PRIORITÄT: Use zeilen_bereiche if it has entries (individual rows/ranges)
        println!("🔍 Verwende zeilen_bereiche für Zeilenauswahl");
        build_query_with_row_ranges_enhanced(&columns_clause, &bereich.zeilen_bereiche)
    } else {
        // FALLBACK: Use continuous row range (von_zeile/bis_zeile)
        println!("📊 Verwende kontinuierlichen Bereich von_zeile/bis_zeile");
        build_query_with_continuous_range(
            &columns_clause, 
            bereich.von_zeile, 
            bereich.bis_zeile
        )
    }?;

    println!("✅ Generierte Query: {}", query);

    Ok((query, selected_names))
}

fn build_query_with_continuous_range(
    columns_clause: &str,
    von_zeile: usize,
    bis_zeile: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    // Validate row indices
    if von_zeile == 0 {
        return Err("Row indices must start from 1".into());
    }

    if bis_zeile < von_zeile {
        return Err("End row must be greater than or equal to start row".into());
    }

    // Calculate number of rows (inclusive range)
    let anzahl = bis_zeile - von_zeile + 1;

    if anzahl == 0 {
        return Err("Invalid row range".into());
    }

    // Build query for continuous range
    Ok(format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        columns_clause,
        anzahl,
        von_zeile.saturating_sub(1)  // OFFSET is 0-based
    ))
}

// ENHANCED VERSION: Besser für einzelne Zeilen und Bereiche
fn build_query_with_row_ranges_enhanced(
    columns_clause: &str,
    zeilen_bereiche: &[(usize, usize)],
) -> Result<String, Box<dyn std::error::Error>> {
    if zeilen_bereiche.is_empty() {
        return Err("Row ranges cannot be empty".into());
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
        return Err("No valid rows selected".into());
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
// Behalte die alte Funktion für Kompatibilität, aber rufe die erweiterte auf
fn build_query_with_row_ranges(
    columns_clause: &str,
    zeilen_bereiche: &[(usize, usize)],
) -> Result<String, Box<dyn std::error::Error>> {
    // Rufe einfach die erweiterte Version auf
    build_query_with_row_ranges_enhanced(columns_clause, zeilen_bereiche)
}

// Alternative simpler version if you prefer a single function with pattern matching:
pub fn build_column_query_alternative(
    column_names: &[String],
    bereich: TextBereich,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    let mut selected_names = Vec::new();

    // Collect selected column names (same as before)
    for i in bereich.von_spalte..=bereich.bis_spalte {
        if let Some(name) = column_names.get(i.saturating_sub(1)) {
            selected_names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
        } else {
            return Err(format!("Column number {} not found", i).into());
        }
    }

    let columns_clause = selected_names.join(", ");
    let query = match (bereich.zeilen_bereiche.is_empty(), bereich.spalten_bereiche.is_empty()) {
        (false, _) => {
            // Use zeilen_bereiche for rows
            build_query_with_row_ranges_enhanced(&columns_clause, &bereich.zeilen_bereiche)?
        }
        (true, false) => {
            // Use spalten_bereiche for columns (if you want to support this too)
            // You would need to modify the column selection logic
            return Err("Column ranges not yet implemented".into());
        }
        (true, true) => {
            // Use continuous ranges for both
            build_query_with_continuous_range(
                &columns_clause,
                bereich.von_zeile,
                bereich.bis_zeile
            )?
        }
    };
    
    println!("Generated query: {}", query);

    Ok((query, selected_names))
}
