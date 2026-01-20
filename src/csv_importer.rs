use rusqlite::Connection;
use csv::ReaderBuilder;
use std::error::Error;
use std::path::Path;

pub fn import_csvs_to_sqlite<P: AsRef<Path>>(pfade: &[P]) -> Result<Connection, Box<dyn Error>> {
    let mut conn = Connection::open_in_memory()?;
    let mut table_created = false;
    let mut ziel_spalten_anzahl = 0;

    // Wir nutzen eine Transaktion für deutlich mehr Speed
    let tx = conn.transaction()?;

    for pfad in pfade {
        let mut rdr = ReaderBuilder::new()
            .delimiter(b';')
            .quoting(true)
            .trim(csv::Trim::All)
            .from_path(pfad.as_ref())?;

        let headers = rdr.headers()?.clone();
        
        if !table_created {
            ziel_spalten_anzahl = headers.len();
            let mut existierende_namen = std::collections::HashSet::new();
            let create_columns = headers
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let name = if s.is_empty() { format!("spalte_{}", i) } else { s.to_string() };
                    let mut finaler_name = name.clone();
                    let mut counter = 2;
                    while existierende_namen.contains(&finaler_name) {
                        finaler_name = format!("{}_{}", name, counter);
                        counter += 1;
                    }
                    existierende_namen.insert(finaler_name.clone());
                    format!("\"{}\" TEXT", finaler_name.replace("\"", "\"\""))
                })
                .collect::<Vec<_>>()
                .join(", ");

            tx.execute(&format!("CREATE TABLE csv_data ({})", create_columns), [])?;
            table_created = true;
        }

        let placeholders = vec!["?"; ziel_spalten_anzahl].join(", ");
        let mut stmt = tx.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;

        for result in rdr.records() {
            let record = result?;
            let mut values: Vec<String> = Vec::with_capacity(ziel_spalten_anzahl);
            
            for i in 0..ziel_spalten_anzahl {
                let wert = record.get(i)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "?".to_string());
                values.push(wert);
            }
            stmt.execute(rusqlite::params_from_iter(values.iter()))?;
        }
    } // Ende der for-Schleife über die Pfade

    tx.commit()?; // Transaktion abschließen
    Ok(conn)
}

