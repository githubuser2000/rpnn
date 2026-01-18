use csv::ReaderBuilder;
use rusqlite::{params_from_iter, Connection, Result};
use std::collections::HashSet;



pub fn import_csv_to_sqlite(pfad: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .quoting(true)
        .trim(csv::Trim::All)
        .from_path(pfad)?;

    let mut conn = Connection::open_in_memory()?;
    let headers = rdr.headers()?.clone(); // Klonen für spätere Nutzung
    let spalten_anzahl = headers.len();

    // 1. Tabelle erstellen (Dein Code-Teil leicht gekürzt für die Übersicht)
    let mut existierende_namen = HashSet::new();
    let create_columns = headers.iter().enumerate().map(|(i, s)| {
        let mut name = if s.is_empty() { format!("spalte_{}", i) } else { s.to_string() };
        let mut finaler_name = name.clone();
        let mut counter = 2;
        while existierende_namen.contains(&finaler_name) {
            finaler_name = format!("{}_{}", name, counter);
            counter += 1;
        }
        existierende_namen.insert(finaler_name.clone());
        format!("\"{}\" TEXT", finaler_name.replace("\"", "\"\""))
    }).collect::<Vec<_>>().join(", ");

    conn.execute(&format!("CREATE TABLE csv_data ({})", create_columns), [])?;

    // 2. DATEN IMPORTIEREN (Wichtig: Erst importieren, dann SELECT)
    let tx = conn.transaction()?;
    {
        let placeholders = vec!["?"; spalten_anzahl].join(", ");
        let mut stmt = tx.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;

        for result in rdr.records() {
            let record = result?;
            stmt.execute(params_from_iter(record.iter()))?;
        }
    }
    tx.commit()?; // Transaktion abschließen!

    // 3. DYNAMISCHE ABFRAGE DER ERSTEN SPALTE
    // Name der ersten Spalte via PRAGMA holen
    let mut stmt_info = conn.prepare("PRAGMA table_info(csv_data)")?;
    let first_column_name: String = stmt_info.query_row([], |row| row.get(1))?;

    // SQL-String mit format! zusammenbauen
    let query = format!("SELECT \"{}\" FROM csv_data LIMIT 10", first_column_name.replace("\"", "\"\""));
    
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;

    println!("Vorschau der ersten Spalte ({})", first_column_name);
    while let Some(row) = rows.next()? {
        let value: String = row.get(0)?; // Hier Index 0, da wir nur eine Spalte im SELECT haben
        println!("> {}", value);
    }

    Ok(())
}

