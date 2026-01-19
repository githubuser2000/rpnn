use rusqlite::{Connection, params_from_iter};
use csv::ReaderBuilder;
use std::collections::HashSet;
use crate::cli::TextBereich;
use comfy_table::Table;
pub fn import_csv_to_sqlite(pfad: &str) -> Result<Connection, Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .quoting(true)
        .trim(csv::Trim::All)
        .from_path(pfad)?;

    let conn = Connection::open_in_memory()?;
    let headers = rdr.headers()?.clone();
    let spalten_anzahl = headers.len();

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

    let placeholders = vec!["?"; spalten_anzahl].join(", ");
    {
        let mut stmt = conn.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;
        for result in rdr.records() {
            let record = result?;
            stmt.execute(params_from_iter(record.iter()))?;
        }
    }

    Ok(conn) // Hier geben wir die Connection zurück!
}

pub fn query_column_by_index(conn: &Connection, col_index: usize, bereich: TextBereich) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt_info = conn.prepare("PRAGMA table_info(csv_data)")?;
    let column_names: Vec<String> = stmt_info.query_map([], |row| row.get(1))?
        .collect::<Result<Vec<_>, _>>()?;



    // 1. Spaltennamen im Bereich sammeln
    let mut selected_names = Vec::new();
    
    for i in bereich.von_spalte..=bereich.bis_spalte {
        // Falls dein Index 1-basiert ist, nutzen wir i-1
        if let Some(name) = column_names.get(i.saturating_sub(1)) {
            // SQL-Escaping für jeden Namen
            selected_names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
        } else {
            return Err(format!("Spalte Nummer {} nicht gefunden", i).into());
        }
    }
    
    // 2. Die Namen mit Komma verbinden (z.B. "Spalte1", "Spalte2")
    let targets_string = selected_names.join(", ");

    let anzahl = if bereich.bis_zeile >= bereich.von_zeile {
        bereich.bis_zeile - bereich.von_zeile
    } else {
        0
    };

    // 3. Das SQL Statement bauen
    let query = format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        targets_string,
        anzahl,
        bereich.von_zeile
    );

    /*let target_name = column_names.get(col_index - 1)
        .ok_or(format!("Tabelle hat keine Spalte Nummer {}", col_index))?;
    */

    /*let query = format!(
        "SELECT \"{}\" FROM csv_data LIMIT {} OFFSET {}",
        target_name.replace("\"", "\"\""),
        anzahl,
        bereich.von_zeile
    );
    let query = format!(
    "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
    targets_string,
    anzahl,
    bereich.von_zeile
    );*/


    //let query = format!("SELECT \"{}\" FROM csv_data LIMIT 10", target_name.replace("\"", "\"\""));
    

    // ... (vorheriger Code bleibt gleich) ...

    //let mut stmt = conn.prepare(&query)?;
    //let mut rows = stmt.query([])?;

    // Wir berechnen, wie viele Spalten wir eigentlich angefordert haben
    //let anzahl_spalten = selected_names.len();

    println!("Inhalt von Spalten {}:", targets_string);
   /* 
    while let Some(row) = rows.next()? {
        let mut zeile_ergebnis = Vec::new();

        // Iteriere über alle Spalten-Indizes dieser einen Zeile
        for i in 0..anzahl_spalten {
            // Hole den Wert der i-ten Spalte als String
            let value: String = row.get(i).unwrap_or_else(|_| "NULL".to_string());
            zeile_ergebnis.push(value);
        }

        // Verbinde die Werte der Spalten für die Ausgabe, z.B. mit einem Trennstrich
        println!("> {}", zeile_ergebnis.join(" | "));
    }
*/
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;
    let anzahl_spalten = selected_names.len();

    let mut table = Table::new();
    
    // Header setzen (die Spaltennamen)
    table.set_header(&selected_names);

    while let Some(row) = rows.next()? {
        let mut row_cells = Vec::new();
        for i in 0..anzahl_spalten {
            let value: String = row.get(i).unwrap_or_default();
            row_cells.push(value);
        }
        table.add_row(row_cells);
    }

    // Die Tabelle formatiert sich selbst mit gleichen/passenden Breiten
    println!("{table}");
    Ok(())
}





    /*
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;

    println!("Inhalt von Spalten {}:", targets_string);
    while let Some(row) = rows.next()? {
        let value: String = row.get(0)?;
        println!("> {}", value);
    }
    Ok(())
}
*/
