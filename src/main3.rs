use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Element {
    Text(String),
    Liste(Vec<Element>),
}

fn main() {
    // 1. CSV Einlesen
    let pfad = "csv/religion.csv";
    let matrix = match lese_csv_in_matrix(pfad) {
        Ok(m) => m,
        Err(_) => {
            eprintln!("Fehler: Datei {} nicht gefunden.", pfad);
            return;
        }
    };

    // 2. Argumente verarbeiten
    let args: Vec<String> = env::args().collect();
    
    // Wir erwarten Paare, z.B.: -zeile 1 -spalte 2
    // args[0] ist immer der Programmname, daher fangen wir bei 1 an.
    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        
        // ... innerhalb der while-Schleife ...
match arg.as_str() {
    "-zeile" | "--zeile" | "-spalte" | "--spalte" => {
        if i + 1 < args.len() {
            let wert = &args[i + 1];
            // Prüfung: Lässt sich der String in eine positive Ganzzahl umwandeln?
            if wert.parse::<usize>().is_ok() {
                println!("Filter gesetzt: {} = {}", arg, wert);
                i += 2; // Argument UND Wert überspringen
            } else {
                eprintln!("Fehler: Nach {} muss eine positive Nummer folgen, nicht '{}'.", arg, wert);
                return;
            }
        } else {
            eprintln!("Fehler: {} benötigt einen Wert.", arg);
            return;
        }
    }
    _ => {
        eprintln!("Fehler: Unbekanntes Argument '{}'. Erlaubt sind: -zeile, --zeile, -spalte, --spalte.", arg);
        return;
    }
}

            }

    println!("Erfolgreich validiert. Matrix-Vorschau:");
    //print_recursive(&matrix, 0);
}

fn lese_csv_in_matrix(dateipfad: &str) -> io::Result<Element> {
    let datei = File::open(dateipfad)?;
    let reader = io::BufReader::new(datei);
    let mut haupt_liste = Vec::new();

    for zeile_result in reader.lines() {
        let zeile = zeile_result?;
        let spalten: Vec<Element> = zeile
            .split(';')
            .map(|s| Element::Text(s.trim().to_string()))
            .collect();
        haupt_liste.push(Element::Liste(spalten));
    }
    Ok(Element::Liste(haupt_liste))
}

fn print_recursive(el: &Element, tiefe: usize) {
    let einrückung = "  ".repeat(tiefe);
    match el {
        Element::Text(t) => print!("\"{}\" ", t),
        Element::Liste(l) => {
            println!("\n{}[", einrückung);
            for kind in l { print_recursive(kind, tiefe + 1); }
            print!("\n{}]", einrückung);
        }
    }
}

