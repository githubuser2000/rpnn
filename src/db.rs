use rusqlite::{Connection, params_from_iter};
use csv::ReaderBuilder;
use std::collections::HashSet;
use crate::cli::TextBereich;
use comfy_table::{Table, ColumnConstraint, Width, ContentArrangement, TableComponent};
use terminal_size::{Width as TermWidth, terminal_size};
use std::error::Error;

pub fn import_csv_to_sqlite(
    pfad: &str,
) -> Result<Connection, Box<dyn std::error::Error>> {

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .quoting(true)
        .trim(csv::Trim::All)
        .from_path(pfad)?;

    let mut conn = Connection::open_in_memory()?;

    let headers = rdr.headers()?.clone();
    let spalten_anzahl = headers.len();

    // Spaltennamen eindeutig machen
    let mut existierende_namen = HashSet::new();
    let mut finale_spalten: Vec<String> = Vec::new();

    let create_columns = headers.iter().enumerate().map(|(i, s)| {
        let basis = if s.is_empty() {
            format!("spalte_{}", i)
        } else {
            s.to_string()
        };

        let mut name = basis.clone();
        let mut counter = 2;
        while existierende_namen.contains(&name) {
            name = format!("{}_{}", basis, counter);
            counter += 1;
        }

        existierende_namen.insert(name.clone());
        finale_spalten.push(name.clone());

        format!("\"{}\" TEXT DEFAULT '?'", name.replace("\"", "\"\""))
    }).collect::<Vec<_>>().join(", ");

    // Tabelle mit stabiler Zeilen-ID
    conn.execute(
        &format!(
            "CREATE TABLE csv_data (
                row_idx INTEGER PRIMARY KEY,
                {}
            )",
            create_columns
        ),
        [],
    )?;

    // INSERT vorbereiten (explizite Spaltenliste)
    let placeholders = vec!["?"; spalten_anzahl + 1].join(", ");
    let column_list = std::iter::once("row_idx".to_string())
        .chain(finale_spalten.iter().map(|s| format!("\"{}\"", s)))
        .collect::<Vec<_>>()
        .join(", ");

    let insert_sql = format!(
        "INSERT INTO csv_data ({}) VALUES ({})",
        column_list, placeholders
    );

    let mut tx = conn.transaction()?;
    {
         let mut stmt = tx.prepare(&insert_sql)?;
     
     
         let mut row_idx: i64 = 1;
     
         for result in rdr.records() {
             let record = result?;
     
             let mut values: Vec<String> = Vec::with_capacity(spalten_anzahl + 1);
             values.push(row_idx.to_string());
     
             for v in record.iter() {
                 if v.is_empty() {
                     values.push("?".to_string());
                 } else {
                     values.push(v.to_string());
                 }
             }
     
             // Auffüllen, falls CSV-Zeile kürzer als Header
             while values.len() < spalten_anzahl + 1 {
                 values.push("?".to_string());
             }
     
             stmt.execute(params_from_iter(values))?;
             row_idx += 1;
         }
     
    }
    tx.commit()?;
    Ok(conn)
}

fn existing_columns(conn: &Connection) -> Result<HashSet<String>, rusqlite::Error> {
    let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(1))?;

    let mut set = HashSet::new();
    for r in rows {
        set.insert(r?);
    }
    Ok(set)
}


pub fn get_column_names(conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
    let names = stmt
        .query_map([], |row| row.get(1))?
        .collect::<Result<Vec<String>, _>>()?;
    Ok(names)
}


/// Hängt eine CSV rechts an die bestehende Tabelle `csv_data` an.
/// Voraussetzungen:
/// - Tabelle existiert
/// - `row_idx INTEGER PRIMARY KEY` existiert
/// - alle Zellen verwenden "?" als Missing-Wert
pub fn append_csv_right(
    conn: &mut Connection,
    pfad: &str,
    csv_index: usize, // z.B. 2, 3, 4 ... zur Herkunftsmarkierung
) -> Result<(), Box<dyn Error>> {

    // CSV öffnen
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .quoting(true)
        .trim(csv::Trim::All)
        .from_path(pfad)?;

    let headers = rdr.headers()?.clone();
    let csv_col_count = headers.len();

    // Existierende Spalten abfragen
    let mut existing_cols: HashSet<String> = {
        let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(1))?;
        let mut set = HashSet::new();
        for r in rows { set.insert(r?); }
        set
    };

    // Neue Spaltennamen erzeugen (kollisionsfrei)
    let mut final_headers: Vec<String> = Vec::with_capacity(csv_col_count);
    for (i, h) in headers.iter().enumerate() {
        let base = if h.is_empty() {
            format!("spalte_{}", i)
        } else { h.to_string() };

        let tagged = format!("{}__csv{}", base, csv_index);

        let mut name = tagged.clone();
        let mut counter = 2;
        while existing_cols.contains(&name) {
            name = format!("{}_{}", tagged, counter);
            counter += 1;
        }

        existing_cols.insert(name.clone());
        final_headers.push(name);
    }

    // Neue Spalten anlegen (DEFAULT '?')
    for col in &final_headers {
        conn.execute(
            &format!("ALTER TABLE csv_data ADD COLUMN \"{}\" TEXT DEFAULT '?'", col.replace("\"", "\"\"")),
            [],
        )?;
    }

    // Anzahl existierender Zeilen
    let existing_rows: i64 = conn.query_row("SELECT COUNT(*) FROM csv_data", [], |r| r.get(0))?;

    // Transaktion starten
    let mut tx = conn.transaction()?;

    // UPDATE bestehender Zeilen
    {
        let assigns = final_headers
            .iter()
            .map(|h| format!("\"{}\" = ?", h))
            .collect::<Vec<_>>()
            .join(", ");

        let update_sql = format!("UPDATE csv_data SET {} WHERE row_idx = ?", assigns);
        let mut update_stmt = tx.prepare(&update_sql)?;

        let mut row_idx: i64 = 1;

        for result in rdr.records() {
            let record = result?;
            if row_idx > existing_rows { break; }

            let mut values: Vec<String> = record.iter()
                .map(|s| if s.is_empty() { "?".to_string() } else { s.to_string() })
                .collect();

            // Zeilen auffüllen, falls CSV kürzer
            while values.len() < csv_col_count {
                values.push("?".to_string());
            }

            values.push(row_idx.to_string()); // row_idx für WHERE
            update_stmt.execute(params_from_iter(values))?;

            row_idx += 1;
        }
    }

    // INSERT neue Zeilen, falls CSV länger als bestehende Tabelle
    {
        let insert_columns = std::iter::once("row_idx".to_string())
            .chain(final_headers.iter().map(|h| format!("\"{}\"", h)))
            .collect::<Vec<_>>()
            .join(", ");

        let insert_placeholders = vec!["?"; csv_col_count + 1].join(", ");
        let insert_sql = format!("INSERT INTO csv_data ({}) VALUES ({})", insert_columns, insert_placeholders);
        let mut insert_stmt = tx.prepare(&insert_sql)?;

        let mut row_idx: i64 = existing_rows + 1;

        for result in rdr.records().skip(existing_rows as usize) {
            let record = result?;

            let mut values: Vec<String> = Vec::with_capacity(csv_col_count + 1);
            values.push(row_idx.to_string());

            for v in record.iter() {
                values.push(if v.is_empty() { "?".to_string() } else { v.to_string() });
            }

            // Zeilen auffüllen, falls CSV kürzer
            while values.len() < csv_col_count + 1 {
                values.push("?".to_string());
            }

            insert_stmt.execute(params_from_iter(values))?;
            row_idx += 1;
        }
    }

    tx.commit()?; // ✅ stmt-Scope vorbei, commit geht
    Ok(())
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

    // Korrekte Anzahl der Zeilen inkl. von/bis
    let anzahl = bereich.bis_zeile.saturating_sub(bereich.von_zeile).saturating_add(1);

    // OFFSET 1-basiert korrigiert
    let offset = bereich.von_zeile.saturating_sub(1);

    let query = format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        selected_names.join(", "),
        anzahl,
        offset
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
