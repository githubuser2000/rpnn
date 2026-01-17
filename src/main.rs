use std::env;
use cli::parse_cli_args;
use data::Element;
use db::import_csv_to_sqlite;
use utils::print_recursive;

mod cli;
mod data;
mod db;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CLI Argumente parsen
    let args: Vec<String> = env::args().collect();
    let (dashes, params) = parse_cli_args(&args);
    
    // Beispiel-Struktur erstellen
    let meine_liste = data::create_example_structure();
    println!("Struktur wurde erstellt. Hier ist die rekursive Ausgabe:");
    print_recursive(&meine_liste, 0);
    
    // CSV in SQLite importieren
    let pfad = "/data/data/com.termux/files/home/Eigene-Dateien/rpnn/csv/religion.csv";
    import_csv_to_sqlite(pfad)?;
    
    Ok(())
}
