
mod cli;
mod data;
mod utils;
mod csv_importer;
mod column_manager;
mod data_fetcher;
mod table_printer;
mod workflows;
mod tabellen_utils;
mod argument_verarbeiter;
mod kategorie_verarbeiter;
mod reta_ausgabe;  // war retaAusgabe
mod if_is_zeilen_angabe;  // war if_is_zeilen_angabe
mod column_categories_complete;  // war column_categories_complete
mod input_help;  // war input_help
mod multiples_teiler;  // war multiples_teiler
use workflows::main_workflow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_workflow()
}
