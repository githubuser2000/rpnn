use std::path::PathBuf;
use std::env;
use cli::parse_cli_args;
use data::Element;
use csv_importer::import_csvs_to_sqlite;
use table_printer::query_column_by_index;
use utils::print_recursive;
mod cli;
mod data;
mod utils;
mod csv_importer;
mod column_manager;
mod data_fetcher;
mod table_printer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CLI Argumente parsen
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
    let pfad2 = projPath.to_string_lossy().into_owned() + "/csv/concat-reli-001.csv";
    let dateien = [
        pfad1.as_str(),
        pfad2.as_str(),
    ];
    let conn = import_csvs_to_sqlite(&dateien)?;
    query_column_by_index(&conn, bereich)?; // Fragt die 1. Spalte ab
    Ok(())
}
