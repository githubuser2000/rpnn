
mod retaAusgabe;
mod cli;
mod data;
mod utils;
mod csv_importer;
mod column_manager;
mod data_fetcher;
mod table_printer;
mod ifIsZeilenAngabe;
mod columnCategories_complete;
mod inputHelp;
mod workflows;
mod tabellen_utils;
mod argument_verarbeiter;
mod kategorie_verarbeiter;
mod multiplesTeiler;

use workflows::main_workflow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    main_workflow()
}
