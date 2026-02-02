// src/retaAusgabe/mod.rs
pub mod output_syntax;
pub mod utils;
pub mod table_cell;
pub mod tables;
pub mod cli_output;

// Re-export
pub use output_syntax::OutputSyntax;
pub use table_cell::{TableCell, TableRow};
pub use tables::Tables;
pub use cli_output::CliOutput;
