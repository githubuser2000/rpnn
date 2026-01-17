use std::fs::File;                                                          use std::io::{self, BufRead};                                               use std::path::Path;
use std::env;
use rusqlite::{params_from_iter, Connection, Result};
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
    // 1. Einlesen (bei Fehler sofortiger Abbruch der main)
    let matrix = lese_csv_in_matrix(pfad)?; 
    println!("csv Datei erfolgreich eingelesen.");

    // 2. Datenbank-Setup
    let mut conn = Connection::open_in_memory()?;
    
    if matrix.is_empty() { return Ok(()); }
    let spalten = &matrix[0];

    // 3. Tabelle bauen
    let create_columns = spalten.iter()
        .map(|s| format!("\"{}\" TEXT", s))
        .collect::<Vec<_>>()
        .join(", ");
    conn.execute(&format!("CREATE TABLE csv_data ({})", create_columns), [])?;

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

    /*                                                                                match lese_csv_in_matrix(pfad) {
        Ok(matrix) => {
        println!("csv Datei in vec vec string erfolgreich eingelesen:");

        // 1. Verbindung richtig öffnen (mit ?)
        let mut conn = Connection::open_in_memory()?; 

        // 2. Den Header (erste Zeile) holen
        // Wir stellen sicher, dass die Matrix nicht leer ist
        if matrix.is_empty() {
            return Ok(()); 
        }
        let spalten = &matrix[0]; // Das ist jetzt ein Vec<String> (die erste Zeile)

        // 3. SQL Spalten-String bauen
        let create_columns = spalten.iter()
            .map(|s| format!("\"{}\" TEXT", s)) // Anführungszeichen helfen bei Sonderzeichen
            .collect::<Vec<_>>()
            .join(", ");

        let create_table_sql = format!("CREATE TABLE csv_data ({})", create_columns);
        
        // 4. Tabelle erstellen
        conn.execute(&create_table_sql, [])?;
        
        println!("Tabelle erfolgreich erstellt.");
        
        // 2. Transaktion starten (EXTREM wichtig für Performance)
        let tx = conn.transaction()?;

        // 3. Insert Statement vorbereiten
        let placeholders = vec!["?"; spalten.len()].join(", ");
        let insert_sql = format!("INSERT INTO csv_data VALUES ({})", placeholders);
        {
            let mut stmt = tx.prepare(&insert_sql)?;
        
            // Über die Daten iterieren (Header überspringen)
            for zeile in matrix.iter().skip(1) {
                // params_from_iter erlaubt es, einen Vector von Strings direkt zu übergeben
                stmt.execute(params_from_iter(zeile))?;
            }
        } // stmt muss hier zerstört werden, bevor tx.commit() gerufen wird

        // 4. Alles auf einmal in den RAM schreiben
        tx.commit()?;

        println!("Erfolg: {} Zeilen in RAM-DB geladen.", matrix.len() - 1);
        Ok(())
        }                                                                           Err(e) => eprintln!("Fehler beim Lesen der Datei: {}", e),
    }
    Ok(())
    */
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
*/

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

