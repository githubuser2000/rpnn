use std::fs::File;                                                          use std::io::{self, BufRead};                                               use std::path::Path;
use std::env;
use std::error::Error;
use csv::ReaderBuilder;
use rusqlite::{params_from_iter, Connection, Result};
use std::collections::HashSet;
#[derive(Debug)]
enum Element {
    Text(String),
    Liste(Vec<Element>),
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let hauptParameter = vec!["zeilen1", "zeilen2", "spalten","ausgabe"];
    let zeilenParameter  = vec!["von", "bis", "ohnevon", "ohnebis"];
    let spaltenParameter = vec!["spaltennummer", "universum", "cluster", "galaxie", "kontinuum"];
    let ausgabeParameter = vec!["nichtsleeres", "unfarbig"];

    fn text(s: &str) -> Element {
        Element::Text(String::from(s))
    }

    fn list(elements: Vec<Element>) -> Element {
        Element::Liste(elements)
    }

    // Jetzt viel lesbarer
    let meine_liste = list(vec![
    list(vec![
        text("zeilen"),
            list(vec![
                text("nummer")
            ]),
        ]),
        list(vec![
           text("spalten"),
          list(vec![
                text("nummer")
            ]),
        ]),
    ]);
    println!("Struktur wurde erstellt. Hier ist die rekursive Ausgabe:");
    print_recursive(&meine_liste, 0);
    let args: Vec<String> = env::args().collect();
    //deep_match(&args, &meine_liste, 0);
    //deep_match_proper(&args, &meine_liste);
    //parse_cli_args(args)
    let (dashes, params) = parse_cli_args(&args);
    // Oder: &args[..]
    // let (dashes, params) = parse_cli_args(&args[..]);
    let pfad = "/data/data/com.termux/files/home/Eigene-Dateien/rpnn/csv/religion.csv";

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


    use std::collections::HashSet;

    // ... (nachdem du headers = rdr.headers()? geholt hast)

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




    /*
    // Spaltennamen sicher machen (Anführungszeichen und Eindeutigkeit)
    let create_columns = headers.iter()
        .enumerate()
        .map(|(i, s)| {
            let name = s.trim();
            if name.is_empty() {
                format!("\"spalte_{}\" TEXT", i)
            } else {
                format!("\"{}\" TEXT", name.replace("\"", "\"\""))
            }
        })
        .collect::<Vec<_>>()
        .join(", ");

    conn.execute(&format!("CREATE TABLE csv_data ({})", create_columns), [])?;
    */

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




/*

    // 1. Einlesen (bei Fehler sofortiger Abbruch der main)
    let matrix = lese_csv_in_matrix(pfad)?; 
    println!("csv Datei erfolgreich eingelesen.");

    // 2. Datenbank-Setup
    let mut conn = Connection::open_in_memory()?;
    
    if matrix.is_empty() { return Ok(()); }
    let spalten = &matrix[0];
/*
    // 3. Tabelle bauen
    let create_columns = spalten.iter()
        .map(|s| format!("\"{}\" TEXT", s))
        .collect::<Vec<_>>()
        .join(", ");
    conn.execute(&format!("CREATE TABLE csv_data ({})", create_columns), [])?;
*/
    // 3. SQL Spalten-String bauen (mit Trim und Absicherung)
    let create_columns = spalten.iter()
    .enumerate()
    .map(|(i, s)| {
        let clean_name = s.trim().replace("\"", "\"\""); // Anführungszeichen im Namen verdoppeln
        if clean_name.is_empty() {
            format!("spalte_{} TEXT", i) // Falls Spaltenname leer ist
        } else {
            format!("\"{}\" TEXT", clean_name)
        }
    })
    .collect::<Vec<_>>()
    .join(", ");

    let create_table_sql = format!("CREATE TABLE csv_data ({})", create_columns);

// DEBUG: Gib das SQL einmal aus, bevor du es ausführst!
// println!("DEBUG SQL: {}", create_table_sql);

    conn.execute(&create_table_sql, [])?;


    // 4. Daten einfügen (Transaktion)
    let tx = conn.transaction()?;
    {
        let placeholders = vec!["?"; spalten.len()].join(", ");
        let mut stmt = tx.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;
        for zeile in matrix.iter().skip(1) {
            stmt.execute(params_from_iter(zeile))?;
        }
    } 
    tx.commit()?;

    println!("Erfolg: {} Zeilen geladen.", matrix.len() - 1);
    Ok(())
    */
    Ok(())
}

fn print_recursive(el: &Element, tiefe: usize) {
    let einrückung = "  ".repeat(tiefe);
    match el {
        Element::Text(t) => {
            println!("{}- {}", einrückung, t);
        }
        Element::Liste(l) => {
            println!("{}[Liste]:", einrückung);
            for kind in l {
                print_recursive(kind, tiefe + 1);
            }
        }
    }
}
fn parse_cli_args(args: &[String]) -> (Vec<usize>, Vec<String>) {
    let mut minuses = Vec::with_capacity(args.len());
    let mut params = Vec::with_capacity(args.len());
    let mut params2: Vec<String>;
    let mut paramsPerParam: Vec<Vec<String>> = vec![vec![String::new()]];
    let mut dash_count_before = 0;
    
    for (i, arg) in args.iter().enumerate() {
        let mut dash_count = 0;
        
        // Zähle aufeinanderfolgende Minuszeichen am Anfang
        for c in arg.chars() {
            if c == '-' {
                dash_count += 1;
            } else {
                break;
            }
        }
        if let Some(letztes) = paramsPerParam.last_mut() {
            println!("ever");
            if dash_count > dash_count_before {
                letztes.push(arg.clone());
                println!("Argument dazu {}", arg);
            }
            else {
                params2 = Vec::new();
                params2.push(arg.clone());
                letztes.push(arg.clone());
                paramsPerParam.push(params2);
                println!("Argument neu {}", arg);
            }
        }
        
        // Extrahiere Parameter ohne Minuszeichen
        let param = if dash_count > 0 {
            arg.chars().skip(dash_count).collect()
        } else {
            arg.clone()
        };
        
        // LÖSUNG: Zuerst drucken, dann moven
        println!("Argument {}: '{}' → {} Minuszeichen → '{}'", 
                i + 1, arg, dash_count, param);
        
        minuses.push(dash_count);
        params.push(param);  
        dash_count_before=dash_count;
    }
    
    (minuses, params)
}

//fn lese_csv_in_matrix(dateipfad: &str) -> io::Result<Element> {
/*
fn lese_csv_in_matrix(dateipfad: &str) Result<Vec<Vec<String>>, Box<dyn std::error::Error>>  {
    let datei = File::open(dateipfad)?;
    let reader = io::BufReader::new(datei);
    let mut haupt_liste = Vec::new();                                       
    for zeile_result in reader.lines() {                                            let zeile = zeile_result?;                                                  // Splitte die Zeile am Semikolon
        let spalten: Vec<Element> = zeile                                               .split(';')
            .map(|s| Element::Text(s.trim().to_string()))                               .collect();                                                     
        // Füge die Zeile als Liste zur Hauptliste hinzu
        haupt_liste.push(Element::Liste(spalten));
    }

    Ok(Element::Liste(haupt_liste))

}


fn lese_csv_in_matrix(dateipfad: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let datei = File::open(dateipfad)?;
    let reader = io::BufReader::new(datei);
    
    let mut haupt_liste = Vec::new();

    for zeile_result in reader.lines() {
        let zeile = zeile_result?;
        
        // Hier splitten wir und sammeln direkt Strings, keine "Elements"
        let spalten: Vec<String> = zeile
            .split(';')
            .map(|s| s.trim().to_string())
            .collect();
        
        // Füge die Zeile (Vec<String>) zur Hauptliste (Vec<Vec<String>>) hinzu
        haupt_liste.push(spalten);
    }

    Ok(haupt_liste) // Gibt jetzt den fertigen Vec<Vec<String>> zurück
}
*/




fn lese_csv_in_matrix(dateipfad: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    // Der ReaderBuilder erlaubt es, das Trennzeichen (Delimiter) einzustellen
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')      // Hier sagst du: Trenne bei Semikolon
        .quoting(true)        // Beachte Anführungszeichen (Standard: true)
        .trim(csv::Trim::All) // Entfernt Leerzeichen um die Werte
        .from_path(dateipfad)?;

    let mut matrix = Vec::new();

    // 1. Header (erste Zeile) auslesen
    let header = rdr.headers()?;
    let header_vec: Vec<String> = header.iter().map(|s| s.to_string()).collect();
    matrix.push(header_vec);

    // 2. Datenzeilen auslesen
    for result in rdr.records() {
        let record = result?;
        let zeile: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        matrix.push(zeile);
    }

    Ok(matrix)
}

