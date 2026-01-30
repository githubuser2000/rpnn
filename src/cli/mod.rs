// src/cli/mod.rs
mod parser;
mod bereich;
mod utils;

// Re-export der öffentlichen API
pub use bereich::TextBereich;
pub use parser::parse_cli_args;

// Interne Funktionen bleiben privat
use parser::parse_zeilenangabe_zu_bereichen;
use utils::sortiere_und_fasse_zusammen;
