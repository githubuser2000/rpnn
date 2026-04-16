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

mod data_fetcher;
mod column_categories_complete;
mod generated_columns_words_registry;

use app::workflow::main_workflow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_workflow()
}
