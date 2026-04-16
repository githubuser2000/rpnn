use rusqlite::{Connection, Result};
pub fn get_column_names(conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
    let names = stmt.query_map([], |row| row.get(1))?
        .collect::<Result<Vec<String>, _>>()?;
    Ok(names)
}

pub fn fetch_data(conn: &Connection, names: &[String], bereich: &TextBereich) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let targets_sql = names.join(", ");
    let anzahl = if bereich.bis_zeile >= bereich.von_zeile {
        bereich.bis_zeile - bereich.von_zeile
    } else { 0 };

    println!( "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        targets_sql, anzahl, bereich.von_zeile);
    let query = format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        targets_sql, anzahl, bereich.von_zeile
    );

    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;
    let mut data = Vec::new();

    while let Some(row) = rows.next()? {
        let mut row_values = Vec::new();
        for i in 0..names.len() {
            row_values.push(row.get(i).unwrap_or_default());
        }
        data.push(row_values);
    }
    Ok(data)
}

