use csv::ReaderBuilder;
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;
use rusqlite::Connection;

pub fn import_csvs_to_sqlite<P: AsRef<Path>>(pfade: &[P]) -> Result<Connection, Box<dyn Error>> {
    let mut all_headers = Vec::new();
    // row_idx -> Vec<Spaltenwerte>
    let mut data_map: HashMap<usize, Vec<String>> = HashMap::new();
    let mut gesamt_spalten_anzahl = 0;

    for (file_idx, pfad) in pfade.iter().enumerate() {
        let mut rdr = ReaderBuilder::new()
            .delimiter(b';')
            .quoting(true)
            .trim(csv::Trim::All)
            .from_path(pfad.as_ref())?;

        let headers = rdr.headers()?;
        let spalten_in_dieser_datei = headers.len();

        // Header sammeln und eindeutig machen
        for (i, h) in headers.iter().enumerate() {
            let name = if h.is_empty() { format!("f{}_s{}", file_idx, i) } else { h.to_string() };
            all_headers.push(name);
        }

        // Daten einlesen und horizontal an die Map hängen
        let mut current_row = 0;
        for result in rdr.records() {
            let record = result?;
// Falsch: .or_insert_with(Vec::new()); 
// Richtig:
let row = data_map.entry(current_row).or_insert_with(|| Vec::new());

// ODER noch kürzer:
let row = data_map.entry(current_row).or_insert_with(Vec::new);

            // Falls vorherige Dateien weniger Zeilen hatten, fülle links mit "?" auf
            while row.len() < gesamt_spalten_anzahl {
                row.push("?".to_string());
            }

            // Aktuelle Daten anhängen
            for field in record.iter() {
                row.push(field.to_string());
            }
            current_row += 1;
        }

        // Falls diese Datei kürzer war als vorherige, fülle die restlichen Zeilen rechts auf
        for row in data_map.values_mut() {
            while row.len() < gesamt_spalten_anzahl + spalten_in_dieser_datei {
                row.push("?".to_string());
            }
        }

        gesamt_spalten_anzahl += spalten_in_dieser_datei;
    }

    // --- Jetzt erst in die Datenbank schreiben ---
        // --- Jetzt erst in die Datenbank schreiben ---
    let mut conn = Connection::open_in_memory()?;
    let tx = conn.transaction()?;

    // 1. Tabelle erstellen
    let create_columns = all_headers.iter()
        .enumerate()
        .map(|(i, n)| {
            format!("\"{} (ID_{})\" TEXT", n.replace("\"", "\"\""), i)
        })
        .collect::<Vec<_>>()
        .join(", ");
    
    tx.execute(&format!("CREATE TABLE csv_data ({})", create_columns), [])?;

    // 2. Daten einfügen (In einem Block, damit stmt vor tx.commit() gelöscht wird)
    {
        let placeholders = vec!["?"; gesamt_spalten_anzahl].join(", ");
        let mut stmt = tx.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;

        let mut indices: Vec<_> = data_map.keys().collect();
        indices.sort();

        for idx in indices {
            let row = &data_map[idx];
            stmt.execute(rusqlite::params_from_iter(row.iter()))?;
        }
    } // <--- Hier wird 'stmt' gedroppt, der Borrow von 'tx' endet.

    tx.commit()?; // Jetzt gehört tx wieder uns und kann konsumiert werden.
    
    Ok(conn) // Jetzt gehört conn wieder uns und kann zurückgegeben werden.
}

