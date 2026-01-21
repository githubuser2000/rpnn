use rusqlite::Connection;
use crate::cli::TextBereich;

pub fn get_column_names(conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
    let names = stmt
        .query_map([], |row| row.get(1))?
        .collect::<Result<Vec<String>, _>>()?;
    Ok(names)
}

pub fn build_column_query(
    column_names: &[String],
    bereich: TextBereich,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    let mut selected_names = Vec::new();

    for i in bereich.von_spalte..=bereich.bis_spalte {
        if let Some(name) = column_names.get(i.saturating_sub(1)) {
            selected_names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
        } else {
            return Err(format!("Spalte Nummer {} nicht gefunden", i).into());
        }
    }

    let anzahl = bereich.bis_zeile.saturating_sub(bereich.von_zeile);
    println!( "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        selected_names.join(", "),
        anzahl,
        bereich.von_zeile);
    let query = format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        selected_names.join(", "),
        anzahl,
        bereich.von_zeile
    );

    Ok((query, selected_names))
}
