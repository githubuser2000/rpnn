use std::path::PathBuf;
use std::env;
use cli::parse_cli_args;
use data::Element;
use db::import_csv_to_sqlite;
use db::query_column_by_index;
use utils::print_recursive;
mod cli;
mod data;
mod db;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CLI Argumente parsen
    let args: Vec<String> = env::args().collect();
    let (dashes, params, bereich) = parse_cli_args(&args);
    
    // Beispiel-Struktur erstellen
    let meine_liste = data::create_example_structure();
    println!("Struktur wurde erstellt. Hier ist die rekursive Ausgabe:");
    print_recursive(&meine_liste, 0);
    
    // CSV in SQLite importieren
    let mut projPath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pfad = projPath.to_string_lossy().into_owned() + "/csv/religion.csv";
    let conn = import_csv_to_sqlite(&pfad)?;
    query_column_by_index(&conn, 1, bereich)?; // Fragt die 1. Spalte ab
    Ok(())
}
