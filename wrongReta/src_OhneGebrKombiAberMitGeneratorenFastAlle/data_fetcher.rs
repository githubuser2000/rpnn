use rusqlite::Connection;

pub fn fetch_data_with_stats(
    conn: &Connection,
    query: &str,
    column_count: usize,
    header_lengths: &[usize],
) -> Result<(Vec<Vec<String>>, Vec<usize>), Box<dyn std::error::Error>> {
    /*println!("=== 📊 START Datenabfrage ===");
    println!("🔍 Query: {}", query);
    println!("📋 Erwartete Spaltenanzahl: {}", column_count);
    println!("📏 Header-Längen: {:?}", header_lengths);*/
    
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query([])?;

    let mut all_data = Vec::new();
    let mut max_lengths = header_lengths.to_vec();
    let mut row_counter = 0;
    let mut zeilennummer = 1; // 1-basierte Zählung für Ausgabe

    //println!("📈 Lese Datenbankzeilen...");
    
    while let Some(row) = rows.next()? {
        row_counter += 1;
        //println!("\n📄 Zeile {} (DB-Row {}):", zeilennummer, row_counter);
        
        let mut values = Vec::new();
        for i in 0..column_count {
            let val: String = row.get(i).unwrap_or_default();
            
            // Debug-Info für diese Zelle
            let len = val.chars().count();
            let old_max = max_lengths[i];
            
            /*if len > old_max {
                max_lengths[i] = len;
                println!("  Spalte {}: '{}' (Länge: {} → {}!)", 
                         i + 1, 
                         val.chars().take(30).collect::<String>() + if val.len() > 30 { "..." } else { "" }, 
                         old_max, len);
            } else {
                println!("  Spalte {}: '{}' (Länge: {})", 
                         i + 1, 
                         val.chars().take(30).collect::<String>() + if val.len() > 30 { "..." } else { "" }, 
                         len);
            }*/
            
            values.push(val);
        }
        
        all_data.push(values);
        zeilennummer += 1;
    }

    /*println!("\n=== ✅ DATENABFRAGE ERFOLGREICH ===");
    println!("📊 Statistiken:");
    println!("  - Gelesene Zeilen: {}", row_counter);
    println!("  - Gespeicherte Zeilen: {}", all_data.len());
    println!("  - Finale Spaltenbreiten: {:?}", max_lengths);*/
    
    if row_counter == 0 {
        println!("⚠️  WARNUNG: Keine Daten gefunden!");
    }

    Ok((all_data, max_lengths))
}
