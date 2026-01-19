use rusqlite::{Connection, params_from_iter};
use csv::ReaderBuilder;
use std::collections::HashSet;

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

pub fn query_column_by_index(conn: &Connection, col_index: usize, von_zeile : usize, bis_zeile: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut stmt_info = conn.prepare("PRAGMA table_info(csv_data)")?;
    let column_names: Vec<String> = stmt_info.query_map([], |row| row.get(1))?
        .collect::<Result<Vec<_>, _>>()?;

    let target_name = column_names.get(col_index - 1)
        .ok_or(format!("Tabelle hat keine Spalte Nummer {}", col_index))?;


    let anzahl = if bis_zeile >= von_zeile {
        bis_zeile - von_zeile
    } else {
        0
    };

    let query = format!(
        "SELECT \"{}\" FROM csv_data LIMIT {} OFFSET {}",
        target_name.replace("\"", "\"\""),
        anzahl,
        von_zeile
    );
    //let query = format!("SELECT \"{}\" FROM csv_data LIMIT 10", target_name.replace("\"", "\"\""));
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;

    println!("Inhalt von Spalte {}:", target_name);
    while let Some(row) = rows.next()? {
        let value: String = row.get(0)?;
        println!("> {}", value);
    }
    Ok(())
}

