use rusqlite::{Connection, params_from_iter};
use csv::ReaderBuilder;
use std::collections::HashSet;
use crate::cli::TextBereich;
use comfy_table::{Table, ColumnConstraint, Width, ContentArrangement, TableComponent};
use terminal_size::{Width as TermWidth, terminal_size};
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

    let query = format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        selected_names.join(", "),
        anzahl,
        bereich.von_zeile
    );

    Ok((query, selected_names))
}

pub fn fetch_data_with_stats(
    conn: &Connection,
    query: &str,
    column_count: usize,
    header_lengths: &[usize],
) -> Result<(Vec<Vec<String>>, Vec<usize>), Box<dyn std::error::Error>> {

    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query([])?;

    let mut all_data = Vec::new();
    let mut max_lengths = header_lengths.to_vec();

    while let Some(row) = rows.next()? {
        let mut values = Vec::new();
        for i in 0..column_count {
            let val: String = row.get(i).unwrap_or_default();
            let len = val.chars().count();
            if len > max_lengths[i] {
                max_lengths[i] = len;
            }
            values.push(val);
        }
        all_data.push(values);
    }

    Ok((all_data, max_lengths))
}

pub fn print_table(
    headers: &[String],
    data: Vec<Vec<String>>,
    max_lengths: &[usize],
) {
    let mut table = comfy_table::Table::new();

    let term_width = terminal_size::terminal_size()
        .map(|(terminal_size::Width(w), _)| w)
        .unwrap_or(100);

    table
        .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
        .set_width(term_width)
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_header(headers);

    let total: usize = max_lengths.iter().sum();

    for (i, len) in max_lengths.iter().enumerate() {
        let percent = ((*len as f32 / total as f32) * 100.0) as u16;
        table.column_mut(i)
            .unwrap()
            .set_constraint(ColumnConstraint::UpperBoundary(
                Width::Percentage(percent.max(5))
            ));
    }

    for row in data {
        table.add_row(row);
    }

    if !headers.is_empty() {
        println!("{table}");
    } else {
        println!("Keine Daten für den gewählten Bereich gefunden.");
    }
}

pub fn query_column_by_index(
    conn: &Connection,
    bereich: TextBereich,
) -> Result<(), Box<dyn std::error::Error>> {

    let column_names = get_column_names(conn)?;
    let (query, headers) = build_column_query(&column_names, bereich)?;

    let header_lengths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    let (data, max_lengths) =
        fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    print_table(&headers, data, &max_lengths);

    Ok(())
}
