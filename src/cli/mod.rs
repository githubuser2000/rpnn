// src/cli/mod.rs
mod bereich;
pub mod parser;
pub mod utils;

pub use bereich::TextBereich;
// Exportiere die parse_cli_args Funktion aus dem parser-Modul
pub use parser::parse_cli_args;
