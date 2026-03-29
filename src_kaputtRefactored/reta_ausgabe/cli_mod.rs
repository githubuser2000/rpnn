// reta_ausgabe-mod.rs
pub mod reta_ausgabe_output_syntax;
pub mod reta_ausgabe_utils;
pub mod reta_ausgabe_table_cell;
pub mod reta_ausgabe_tables;
pub mod reta_ausgabe_cli_output;

// Re-export häufig verwendete Typen
pub use reta_ausgabe_output_syntax::OutputSyntax;
pub use reta_ausgabe_table_cell::{TableCell, TableRow};
pub use reta_ausgabe_tables::Tables;
pub use reta_ausgabe_cli_output::CliOutput;
