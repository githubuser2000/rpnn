// file: column_manager.rs
use rusqlite::Connection;
use crate::cli::TextBereich;
use std::process;

pub fn get_column_names(conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
    let names = stmt
        .query_map([], |row| row.get(1))?
        .collect::<Result<Vec<String>, _>>()?;
    Ok(names)
}

fn validate_spalten_input(
    bereich: &TextBereich,
    wurde_spalten_gesucht: bool,
) -> Result<(), Box<dyn std::error::Error>> {

    if !bereich.spalten_gesucht {
        return Err("Kein Spalten-Input angegeben".into());
    }

    if wurde_spalten_gesucht && bereich.spalten_bereiche.is_empty() {
        return Err(
            "--spalten wurde angegeben, aber keine Spalten gefunden".into()
        );
    }

    Ok(())
}

pub fn build_column_query(
    column_names: &[String],
    bereich: &mut TextBereich,
    wurde_spalten_gesucht: bool,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {

    validate_spalten_input(bereich, wurde_spalten_gesucht)?;

    let spalten_nummern =
        collect_spalten_nummern(bereich, wurde_spalten_gesucht)?;

    let selected_names =
        resolve_spaltennamen(column_names, &spalten_nummern)?;

    let columns_clause = selected_names.join(", ");

    let query =
        build_row_query(&columns_clause, bereich)?;

    Ok((query, selected_names))
}

fn collect_spalten_nummern(
    bereich: &mut TextBereich,
    wurde_spalten_gesucht: bool,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {

    let mut nums = Vec::new();

    if !bereich.spalten_bereiche.is_empty() {
        bereich.spalten_gefunden = true;

        for &(von, bis) in &bereich.spalten_bereiche {
            for i in von..=bis {
                nums.push(i);
            }
        }
    }
    else if bereich.von_spalte > 0 && bereich.bis_spalte > 0 {
        if bereich.von_spalte > bereich.bis_spalte {
            return Err("Startspalte > Endspalte".into());
        }
        for i in bereich.von_spalte..=bereich.bis_spalte {
            nums.push(i);
        }
    }
    else {
        if wurde_spalten_gesucht {
            return Err("Spalten wurden gesucht, aber keine verarbeitet".into());
        }
        nums.push(1);
    }

    nums.sort();
    nums.dedup();

    if nums.is_empty() {
        return Err("Keine Spaltennummern ausgewählt".into());
    }

    Ok(nums)
}

fn resolve_spaltennamen(
    column_names: &[String],
    spalten_nummern: &[usize],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {

    let mut names = Vec::new();

    for &nr in spalten_nummern {
        if nr == 0 || nr > column_names.len() {
            return Err(format!(
                "Spalte {} existiert nicht", nr
            ).into());
        }

        let name = &column_names[nr - 1];
        names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
    }

    Ok(names)
}

fn build_row_query(
    columns_clause: &str,
    bereich: &TextBereich,
) -> Result<String, Box<dyn std::error::Error>> {

    if !bereich.zeilen_bereiche.is_empty() {
        return build_query_with_row_ranges_enhanced(
            columns_clause,
            &bereich.zeilen_bereiche,
        );
    }

    if bereich.von_zeile > 0 && bereich.bis_zeile > 0 {
        return build_query_with_continuous_range(
            columns_clause,
            bereich,
        );
    }

    Ok(format!(
        "SELECT {} FROM csv_data",
        columns_clause
    ))
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
            .map(|n| (n - 1).to_string())  // 0-basiert für SQL
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
    
    // PRÜFUNG: Sind Spaltennummern vorhanden?
    if spalten_nummern.is_empty() {
        println!("❌ FEHLER: Keine Spaltennummern angegeben");
        return Err("Keine Spaltennummern angegeben".into());
    }
    
    let mut selected_names = Vec::new();
    
    // Überprüfen ob Spaltennummern existieren
    for &nummer in spalten_nummern {
        if nummer == 0 || nummer > column_names.len() {
            println!("❌ FEHLER: Spaltennummer {} existiert nicht (Tabelle hat {} Spalten)", 
                     nummer, column_names.len());
            return Err(format!("Spaltennummer {} existiert nicht (Tabelle hat {} Spalten)", 
                               nummer, column_names.len()).into());
        }
        
        if let Some(name) = column_names.get(nummer.saturating_sub(1)) {
            selected_names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
        } else {
            println!("❌ FEHLER: Spaltennummer {} nicht gefunden", nummer);
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
