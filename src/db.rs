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

    // --- START DER STATISTISCHEN MESSUNG ---
    let mut all_data = Vec::new();
    let mut max_lengths: Vec<usize> = selected_names.iter().map(|n| n.len()).collect();

    while let Some(row) = rows.next()? {
        let mut row_values = Vec::new();
        for i in 0..selected_names.len() {
            let val: String = row.get(i).unwrap_or_default();
            // Wir messen die reale Länge (Unicode-sicher)
            let current_len = val.chars().count();
            if current_len > max_lengths[i] {
                max_lengths[i] = current_len;
            }
            row_values.push(val);
        }
        all_data.push(row_values);
    }

    // --- TABELLEN-KONFIGURATION ---
    let mut table = comfy_table::Table::new();
    
    // Terminalbreite ermitteln
    let term_width = if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
        w
    } else { 100 };

    table.set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
         .set_width(term_width)
         .load_preset(comfy_table::presets::UTF8_FULL)
         .set_header(&selected_names);

    // Spaltenbreiten gewichtet verteilen
    let gesamt_zeichen: usize = max_lengths.iter().sum();

    for i in 0..selected_names.len() {
        // Anteil dieser Spalte am Gesamtvorkommen berechnen
        let anteil = max_lengths[i] as f32 / gesamt_zeichen as f32;
        let prozent = (anteil * 100.0) as u16;
        
        // Mindestbreite von 5% garantieren, damit nichts verschwindet
        let column = table.column_mut(i).unwrap();
        column.set_constraint(ColumnConstraint::UpperBoundary(Width::Percentage(prozent.max(5))));
    }

    // Daten einfüllen
    for row in all_data {
        table.add_row(row);
    }

    // Finale Ausgabe
    if !selected_names.is_empty() {
        println!("{table}");
    } else {
        println!("Keine Daten für den gewählten Bereich gefunden.");
    }

    Ok(())
}







/*
let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;

    let anzahl_spalten = selected_names.len();
    let mut all_data = Vec::new();
    
    // 1. Daten zwischenspeichern und Längen messen
    // Initialisiere max_lengths mit der Länge der Header-Namen
    let mut max_lengths: Vec<usize> = selected_names.iter().map(|s| s.len()).collect();

    while let Some(row) = rows.next()? {
        let mut row_values = Vec::new();
        for i in 0..anzahl_spalten {
            let val: String = row.get(i).unwrap_or_default();
            // Statistisches Messen: Was ist der längste Text in dieser Spalte?
            let len = val.chars().count();
            if len > max_lengths[i] {
                max_lengths[i] = len;
            }
            row_values.push(val);
        }
        all_data.push(row_values);
    }

    // 2. Terminalbreite für die Gewichtung nutzen
    let term_width = if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
        w as usize
    } else { 100 };

    let gesamt_zeichen: usize = max_lengths.iter().sum();

    // 3. Tabelle erstellen
    let mut table = comfy_table::Table::new();
    table.set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
         .set_width(term_width as u16)
         .set_header(&selected_names);

    // 4. Spaltenbreiten gewichtet setzen
    for i in 0..anzahl_spalten {
        let anteil = max_lengths[i] as f32 / gesamt_zeichen as f32;
        let berechnete_breite = (anteil * term_width as f32) as u16;
        
        let column = table.column_mut(i).unwrap();
        // Wir setzen eine untere Grenze von 10, damit schmale Spalten lesbar bleiben
        column.set_constraint(comfy_table::ColumnConstraint::Percentage((anteil * 100.0) as u16));
    }

    // 5. Daten in die Tabelle füllen
    for row in all_data {
        table.add_row(row);
    }

    println!("{table}");
    Ok(())
}
*/




/*
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;
    let anzahl_spalten = selected_names.len();

let mut table = Table::new();
    
    // 1. Terminal-Breite abfragen
    let term_width = if let Some((TermWidth(w), _)) = terminal_size() {
        w as u16
    } else {
        100 // Ein solider Standardwert
    };

    // 2. Dynamisches Layout aktivieren
    table
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(term_width)
        // Optional: Verschönere die Optik, damit die Trennung klar bleibt
        .load_preset(comfy_table::presets::UTF8_FULL)
        .set_header(&selected_names);

    while let Some(row) = rows.next()? {
        let mut row_cells = Vec::new();
        for i in 0..selected_names.len() {
            let value: String = row.get(i).unwrap_or_default();
            row_cells.push(value);
        }
        table.add_row(row_cells);
    }

    // Falls die Tabelle immer noch zu gequetscht aussieht,
    // kannst du eine Mindestbreite pro Spalte erzwingen:
    // table.column_iter_mut().for_each(|c| { c.set_constraint(ColumnConstraint::LowerBoundary(Width::Fixed(10))); });

    println!("\nErgebnis der Abfrage:\n{table}");

    Ok(())
}

*/
/*
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
}*/

/*
let mut table = Table::new();
    
    // 1. Terminal-Breite ermitteln
    let term_width = if let Some((TermWidth(w), _)) = terminal_size() {
        w
    } else {
        80 // Fallback, falls Breite nicht ermittelbar
    };

    // 2. Tabellen-Layout konfigurieren
    table
        .set_content_arrangement(ContentArrangement::Dynamic) // Erlaubt Umbrüche
        .set_width(term_width); // Begrenzt Tabelle auf Fensterbreite

    // Header setzen
    table.set_header(&selected_names);

    // 3. Zeilen hinzufügen
    while let Some(row) = rows.next()? {
        let mut row_cells = Vec::new();
        for i in 0..selected_names.len() {
            let value: String = row.get(i).unwrap_or_default();
            row_cells.push(value);
        }
        table.add_row(row_cells);
    }

    // 4. Ausgabe
    println!("{table}");

    Ok(())
}

*/


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
