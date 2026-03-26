mod app;
mod processing;
mod data_access;
mod domain;
mod lib4tables_enum;

mod cli;
mod column_manager;
mod data;
mod if_is_zeilen_angabe;
mod input_help;
mod multiples_teiler;
mod reta_ausgabe;
mod table_printer;

mod workflows;
mod argument_verarbeiter;
mod kategorie_verarbeiter;
mod csv_importer;
mod data_fetcher;
mod tabellen_utils;
mod column_categories_complete;
mod generated_columns_words_registry;
mod exact_generator_bridge;
mod python_exact_mappings;
mod pypy_compat;

use app::workflow::main_workflow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_workflow()
}
