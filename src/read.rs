use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Element {
    Text(String),
    Liste(Vec<Element>),
}

fn main() {
    // Pfad zur Datei: Wir gehen davon aus, dass das Programm
    // aus dem Hauptverzeichnis gestartet wird.
    let pfad = "csv/religion.csv";

    match lese_csv_in_matrix(pfad) {
        Ok(matrix) => {
            println!("Datei erfolgreich eingelesen:");
            print_recursive(&matrix, 0);
        }
        Err(e) => eprintln!("Fehler beim Lesen der Datei: {}", e),
    }
}

fn lese_csv_in_matrix(dateipfad: &str) -> io::Result<Element> {
    let datei = File::open(dateipfad)?;
    let reader = io::BufReader::new(datei);
    let mut haupt_liste = Vec::new();

    for zeile_result in reader.lines() {
        let zeile = zeile_result?;
        // Splitte die Zeile am Semikolon
        let spalten: Vec<Element> = zeile
            .split(';')
            .map(|s| Element::Text(s.trim().to_string()))
            .collect();
        
        // Füge die Zeile als Liste zur Hauptliste hinzu
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
            for kind in l {
                print_recursive(kind, tiefe + 1);
            }
            print!("\n{}]", einrückung);
        }
    }
}

