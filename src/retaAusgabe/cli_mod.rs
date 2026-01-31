// retaAusgabe-mod.rs
pub mod retaAusgabe_output_syntax;
pub mod retaAusgabe_utils;
pub mod retaAusgabe_table_cell;
pub mod retaAusgabe_tables;
pub mod retaAusgabe_cli_output;

// Re-export häufig verwendete Typen
pub use retaAusgabe_output_syntax::OutputSyntax;
pub use retaAusgabe_table_cell::{TableCell, TableRow};
pub use retaAusgabe_tables::Tables;
pub use retaAusgabe_cli_output::CliOutput;
