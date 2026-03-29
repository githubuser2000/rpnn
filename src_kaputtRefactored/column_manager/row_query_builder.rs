// file: column_manager/row_query_builder.rs
use crate::cli::TextBereich;

pub fn build_row_query(
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

    Ok(format!("SELECT {} FROM csv_data", columns_clause))
}

pub fn build_query_with_continuous_range(
    columns_clause: &str,
    bereich: &TextBereich,
) -> Result<String, Box<dyn std::error::Error>> {
    //println!("  📐 Berechne kontinuierlichen Zeilenbereich:");
    //println!("    - Startzeile: {}", bereich.von_zeile);
    //println!("    - Endzeile: {}", bereich.bis_zeile);
    
    if bereich.von_zeile == 0 {
        println!("    ❌ FEHLER: Zeilenindizes müssen bei 1 beginnen");
        return Err("Zeilenindizes müssen bei 1 beginnen".into());
    }

    if bereich.bis_zeile < bereich.von_zeile {
        println!("    ❌ FEHLER: Endzeile {} < Startzeile {}", bereich.bis_zeile, bereich.von_zeile);
        return Err("Endzeile muss größer oder gleich Startzeile sein".into());
    }

    let anzahl = bereich.bis_zeile - bereich.von_zeile + 1;
    //println!("    - Anzahl Zeilen (inklusive): {}", anzahl);

    if anzahl == 0 {
        println!("    ❌ FEHLER: Ungültiger Zeilenbereich (Anzahl = 0)");
        return Err("Ungültiger Zeilenbereich".into());
    }

    let offset = bereich.von_zeile.saturating_sub(1);
    //println!("    - SQL OFFSET (0-basiert): {}", offset);
    //println!("    - SQL LIMIT: {}", anzahl);

    let query = format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        columns_clause, anzahl, offset
    );
    
    //println!("    ✅ Kontinuierliche Query generiert");
    Ok(query)
}

pub fn build_query_with_row_ranges_enhanced(
    columns_clause: &str,
    zeilen_bereiche: &[(usize, usize)],
) -> Result<String, Box<dyn std::error::Error>> {
    //println!("  📐 Verarbeite diskrete Zeilenbereiche:");
    
    if zeilen_bereiche.is_empty() {
        println!("    ❌ FEHLER: Zeilenbereiche dürfen nicht leer sein");
        return Err("Zeilenbereiche dürfen nicht leer sein".into());
    }

    //println!("    - Eingabe: {} Zeilenbereiche", zeilen_bereiche.len());
    //println!("    - Bereiche: {:?}", zeilen_bereiche);
    
    let mut einzelne_zeilen = Vec::new();
    let mut bereiche = Vec::new();
    
    for &(start, end) in zeilen_bereiche {
        if start == end {
            einzelne_zeilen.push(start);
            //println!("      → Einzelne Zeile: {}", start);
        } else {
            bereiche.push((start, end));
            //println!("      → Bereich: {} bis {} ({} Zeilen)", start, end, end - start + 1);
        }
    }
    
    //println!("\n    📊 Analyse:");
    //println!("      - {} einzelne Zeilen", einzelne_zeilen.len());
    //println!("      - {} Bereiche", bereiche.len());
    
    if bereiche.is_empty() && !einzelne_zeilen.is_empty() {
        //println!("    ⚡ MODUS: Optimierte Query für einzelne Zeilen");
        
        let row_numbers_str = einzelne_zeilen
            .iter()
            .map(|n| (n - 1).to_string())
            .collect::<Vec<_>>()
            .join(", ");
            
        //println!("      - 0-basierte Zeilennummern: {}", row_numbers_str);
        //println!("      - Anzahl Zeilen: {}", einzelne_zeilen.len());
        
        let query = format!(
            "SELECT {} FROM (
                SELECT *, ROW_NUMBER() OVER (ORDER BY rowid) - 1 as row_num
                FROM csv_data
            ) numbered_data
            WHERE row_num IN ({})
            ORDER BY row_num",
            columns_clause, row_numbers_str
        );
        
        //println!("      ✅ Optimierte Query generiert");
        return Ok(query);
    }
    
    //println!("    📋 MODUS: Allgemeine Query für gemischte Bereiche");
    
    let mut all_row_numbers = Vec::new();
    
    for &zeile in &einzelne_zeilen {
        all_row_numbers.push(zeile);
        //println!("      → Hinzugefügt einzelne Zeile: {}", zeile);
    }
    
    for (start, end) in &bereiche {
        let anzahl_im_bereich = end - start + 1;
        //println!("      → Verarbeite Bereich {}..{} ({} Zeilen)", start, end, anzahl_im_bereich);
        
        for row in *start..=*end {
            all_row_numbers.push(row);
        }
    }
    
    let vor_dedup = all_row_numbers.len();
    //println!("    📈 Vor Dedup: {} Zeilennummern", vor_dedup);
    
    all_row_numbers.sort();
    all_row_numbers.dedup();
    
    //println!("    📊 Nach Dedup:");
    //println!("      - Eindeutige Zeilen: {}", all_row_numbers.len());
    //println!("      - Entfernte Duplikate: {}", vor_dedup - all_row_numbers.len());
    
    if all_row_numbers.is_empty() {
        println!("    ❌ FEHLER: Keine gültigen Zeilen ausgewählt");
        return Err("Keine gültigen Zeilen ausgewählt".into());
    }
    
    let row_numbers_str = all_row_numbers
        .iter()
        .map(|n| (n - 1).to_string())
        .collect::<Vec<_>>()
        .join(", ");

    //println!("    📝 0-basierte Zeilennummern für SQL: {}", row_numbers_str);
    //println!("    📋 Gesamt: {} eindeutige Zeilen", all_row_numbers.len());

    let query = format!(
        "SELECT {} FROM (
            SELECT *, ROW_NUMBER() OVER (ORDER BY rowid) - 1 as row_num
            FROM csv_data
        ) numbered_data
        WHERE row_num IN ({})
        ORDER BY row_num",
        columns_clause, row_numbers_str
    );

    //println!("      ✅ Allgemeine Query generiert");
    Ok(query)
}
