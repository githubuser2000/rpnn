use rusqlite::Connection;
use csv::ReaderBuilder;

pub fn import_csv_to_sqlite(pfad: &str) -> Result<Connection, Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .quoting(true)
        .trim(csv::Trim::All)
        .from_path(pfad)?;

    let conn = Connection::open_in_memory()?;
    let headers = rdr.headers()?.clone();
    let spalten_anzahl = headers.len();

    let mut existierende_namen = std::collections::HashSet::new();
    let create_columns = headers
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let mut name = if s.is_empty() {
                format!("spalte_{}", i)
            } else {
                s.to_string()
            };

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

    conn.execute(&format!("CREATE TABLE csv_data ({})", create_columns), [])?;

    let placeholders = vec!["?"; spalten_anzahl].join(", ");
    {
        let mut stmt = conn.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;
        for result in rdr.records() {
            let record = result?;
            stmt.execute(rusqlite::params_from_iter(record.iter()))?;
        }
    }

    Ok(conn)
}
