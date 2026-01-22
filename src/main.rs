use std::path::PathBuf;
use std::env;
use cli::parse_cli_args;
use csv_importer::import_csvs_to_sqlite;
use table_printer::{query_column_by_index, print_table};
use column_manager::get_column_names;
use utils::print_recursive;
use retaAusgabe::{Tables, CliOutput, OutputSyntax};

mod cli;
mod data;
mod utils;
mod csv_importer;
mod column_manager;
mod data_fetcher;
mod table_printer;
mod retaAusgabe;

pub fn test_simple_table() {
    // Direkt in main.rs testen
    let tables = Tables::new(Some(100));
    let mut output = CliOutput::new(&tables, OutputSyntax::Plain);
    output.cliout2("TEST: Dies sollte ausgegeben werden");
    
    println!("\n=== wurde oben TEST ausgegeben? EINFACHER TABELLEN-TEST ===");

    // Erstelle einfache Testdaten
    let headers = vec![
        "Name".to_string(),
        "Alter".to_string(),
        "Stadt".to_string(),
    ];

    let data = vec![
        vec!["Hans".to_string(), "25".to_string(), "Berlin".to_string()],
        vec!["Anna".to_string(), "30".to_string(), "München".to_string()],
        vec!["Peter".to_string(), "22".to_string(), "Hamburg".to_string()],
    ];

    // Berechne max_lengths
    let mut max_lengths = vec![0, 0, 0];
    for (i, header) in headers.iter().enumerate() {
        max_lengths[i] = max_lengths[i].max(header.len());
    }
    for row in &data {
        for (i, cell) in row.iter().enumerate() {
            if i < max_lengths.len() {
                max_lengths[i] = max_lengths[i].max(cell.len());
            }
        }
    }

    println!("Headers: {:?}", headers);
    println!("Max lengths: {:?}", max_lengths);
    println!("Data rows: {}", data.len());

    // Direkter Aufruf
    print_table(&headers, data, &max_lengths);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== START TABELLEN-TEST ===");
    test_simple_table();
    
    // ... restlicher Code
    let args: Vec<String> = env::args().collect();
    
    // Wenn nur der Programmname vorhanden ist (Länge = 1)
    if args.len() == 1 {
        println!("Benutzung: mein-rpnn --zeilevon 2 --zeilebis 4 --spaltevon 2 --spaltebis 5");
        return Ok(());
    }
    
    let (dashes, params, bereich) = parse_cli_args(&args);
    
    // Beispiel-Struktur erstellen
    let meine_liste = data::create_example_structure();
    println!("Struktur wurde erstellt. Hier ist die rekursive Ausgabe:");
    print_recursive(&meine_liste, 0);
    
    // CSV in SQLite importieren
    let mut projPath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pfad1 = projPath.to_string_lossy().into_owned() + "/csv/religion.csv";
    let pfad2 = projPath.to_string_lossy().into_owned() + "/csv/merged_filtered.csv";
    let dateien = [
        pfad1,
        pfad2,
    ];
    
    let conn = import_csvs_to_sqlite(&dateien)?;
    query_column_by_index(&conn, bereich)?; // Fragt die 1. Spalte ab
    
    let column_names = get_column_names(&conn)?;
    println!("Die Tabelle hat {} Spalten.", column_names.len());
    
    Ok(())
}
