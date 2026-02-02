use std::path::PathBuf;
use std::env;
use cli::parse_cli_args;
use csv_importer::import_csvs_to_sqlite;
use table_printer::{query_column_by_index, print_table};
use column_manager::get_column_names;
use utils::print_recursive;
use columnCategories_complete::lade_kategorie_map;
use cli::TextBereich;
use cli::parser::SpaltenNamen;
use retaAusgabe::{Tables, CliOutput, OutputSyntax};

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
