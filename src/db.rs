use csv::ReaderBuilder;
use rusqlite::{params_from_iter, Connection, Result};
use std::collections::HashSet;

pub fn import_csv_to_sqlite(pfad: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. CSV-Reader konfigurieren
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')      // Trennzeichen Semikolon
        .quoting(true)        // Behandelt "..." korrekt
        .trim(csv::Trim::All) // Entfernt Leerzeichen
        .from_path(pfad)?;

    // 2. Datenbank im RAM erstellen
    let mut conn = Connection::open_in_memory()?;

    // 3. Header auslesen und Tabelle erstellen
    let headers = rdr.headers()?;
    let spalten_anzahl = headers.len();

    // Spaltennamen aufbereiten
    let mut existierende_namen = HashSet::new();
    let create_columns = headers.iter()
        .enumerate()
        .map(|(i, s)| {
            let mut name = s.trim().to_string();

            // 1. Falls Name leer ist, nenne ihn "spalte_N"
            if name.is_empty() {
                name = format!("spalte_{}", i);
            }

            // 2. Falls der Name ein Duplikat ist, hänge eine Nummer an
            let mut finaler_name = name.clone();
            let mut counter = 2;
            while existierende_namen.contains(&finaler_name) {
                finaler_name = format!("{}_{}", name, counter);
                counter += 1;
            }

            existierende_namen.insert(finaler_name.clone());

            // 3. In SQL-Anführungszeichen packen und Sonderzeichen sicher machen
            format!("\"{}\" TEXT", finaler_name.replace("\"", "\"\""))
        })
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!("CREATE TABLE csv_data ({})", create_columns);
    conn.execute(&sql, [])?;

    // 4. Daten streamen (Transaktion für Speed)
    let tx = conn.transaction()?;
    {
        let placeholders = vec!["?"; spalten_anzahl].join(", ");
        let mut stmt = tx.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;

        let mut zeilen_zaehler = 0;
        for result in rdr.records() {
            let record = result?;
            // Der Record lässt sich direkt als Iterator an params_from_iter übergeben
            stmt.execute(params_from_iter(record.iter()))?;
            zeilen_zaehler += 1;
        }
        println!("{} Zeilen erfolgreich importiert.", zeilen_zaehler);
    }
    tx.commit()?;

    // Test-Abfrage: Zeige die ersten 5 Zeilen
    println!("Vorschau der ersten 3 Einträge:");
    let mut stmt = conn.prepare("SELECT * FROM csv_data LIMIT 3")?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        // Hier könntest du auf einzelne Spalten zugreifen
        println!("{:?}", row);
    }

    Ok(())
}
