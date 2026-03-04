use std::path::PathBuf;
use std::env;
use crate::column_categories_complete::lade_kategorie_map;
use crate::csv_importer::import_csvs_to_sqlite;
use crate::table_printer::query_column_by_index;
use crate::column_manager::get_column_names;
//use crate::utils::print_recursive;

use crate::tabellen_utils::{test_simple_table, show_usage};
use crate::argument_verarbeiter::SpaltenVerarbeiter;
use crate::kategorie_verarbeiter::verarbeite_kategorien;

// 5. Funktion: Haupt-Workflow
pub fn main_workflow() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== START TABELLEN-TEST ===");
    //test_simple_table();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        show_usage();
        return Ok(());
    }

    // 1. Lade Kategorie-Daten
    println!("\n📂 Lade Kategorie-Daten...");
    let kategorie_map = lade_kategorie_map();

    // 2. Verarbeite CLI-Argumente und Spaltennamen
    let verarbeiter = SpaltenVerarbeiter::new(&args, &kategorie_map);
    let (bereich, spalten_namen) = verarbeiter.verarbeite_zu_tupel()?;

    // 3. Verarbeite Kategorien
    verarbeite_kategorien(&kategorie_map, &bereich, &spalten_namen)?;
    
    // 4. Datenstruktur erstellen und ausgeben
    /*let meine_liste = create_example_structure();
    println!("Struktur wurde erstellt: {:?}", meine_liste);
    println!("Struktur wurde erstellt. Hier ist die rekursive Ausgabe:");
    print_recursive(&meine_liste, 0);*/

    // 5. CSV-Daten importieren
    let proj_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pfad1 = proj_path.to_string_lossy().into_owned() + "/csv/religion.csv";
    let pfad2 = proj_path.to_string_lossy().into_owned() + "/csv/merged_filtered.csv";
    let dateien = [pfad1, pfad2];

    let conn = import_csvs_to_sqlite(&dateien)?;

    // 6. Spalten abfragen
    query_column_by_index(&conn, bereich)?;

    // 7. Spaltennamen abrufen
    let column_names = get_column_names(&conn)?;
    println!("Die Tabelle hat {} Spalten.", column_names.len());
    
    Ok(())
}
