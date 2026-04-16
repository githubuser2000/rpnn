use csv::ReaderBuilder;
use rusqlite::Connection;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub fn import_csvs_to_sqlite<P: AsRef<Path>>(pfade: &[P]) -> Result<Connection, Box<dyn Error>> {
    let mut all_headers: Vec<String> = Vec::new();
    let mut data_map: HashMap<usize, Vec<String>> = HashMap::new();
    let mut gesamt_spalten_anzahl = 0usize;

    for (file_idx, pfad) in pfade.iter().enumerate() {
        let mut rdr = ReaderBuilder::new()
            .delimiter(b';')
            .quoting(true)
            .trim(csv::Trim::All)
            .from_path(pfad.as_ref())?;

        let headers = rdr.headers()?.clone();
        let spalten_in_dieser_datei = headers.len();

        for (i, h) in headers.iter().enumerate() {
            let name = if h.trim().is_empty() {
                format!("f{}_s{}", file_idx + 1, i + 1)
            } else {
                h.trim().to_string()
            };
            all_headers.push(name);
        }

        let mut current_row = 0usize;

        for result in rdr.records() {
            let record = result?;
            let row = data_map.entry(current_row).or_insert_with(Vec::new);

            while row.len() < gesamt_spalten_anzahl {
                row.push("?".to_string());
            }

            for field in record.iter() {
                row.push(field.to_string());
            }

            while row.len() < gesamt_spalten_anzahl + spalten_in_dieser_datei {
                row.push("?".to_string());
            }

            current_row += 1;
        }

        for row in data_map.values_mut() {
            while row.len() < gesamt_spalten_anzahl + spalten_in_dieser_datei {
                row.push("?".to_string());
            }
        }

        gesamt_spalten_anzahl += spalten_in_dieser_datei;
    }

    let mut conn = Connection::open_in_memory()?;
    let tx = conn.transaction()?;

    let create_columns = all_headers
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let safe_name = name.replace('"', "\"\"");
            format!("\"{} (ID_{})\" TEXT", safe_name, i + 1)
        })
        .collect::<Vec<_>>()
        .join(", ");

    tx.execute(&format!("CREATE TABLE csv_data ({})", create_columns), [])?;

    {
        let placeholders = vec!["?"; gesamt_spalten_anzahl].join(", ");
        let mut stmt = tx.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;

        let mut indices: Vec<usize> = data_map.keys().copied().collect();
        indices.sort_unstable();

        for idx in indices {
            let row = data_map
                .get(&idx)
                .ok_or_else(|| format!("Fehlende Zeile im data_map für Index {}", idx))?;

            if row.len() != gesamt_spalten_anzahl {
                return Err(format!(
                    "Zeile {} hat {} Werte, erwartet wurden {}",
                    idx,
                    row.len(),
                    gesamt_spalten_anzahl
                )
                .into());
            }

            stmt.execute(rusqlite::params_from_iter(row.iter()))?;
        }
    }

    tx.commit()?;
    Ok(conn)
}
