// file: column_manager/validation.rs
use crate::cli::TextBereich;

pub fn validate_spalten_input(
    bereich: &TextBereich,
) -> Result<(), Box<dyn std::error::Error>> {
    if !bereich.spalten_gesucht {
        return Err("Kein Spalten-Input angegeben".into());
    }

    if bereich.spalten_gesucht2 && bereich.spalten_bereiche.is_empty() {
        return Err("--spaltenname wurde angegeben, aber keine Spalten gefunden".into());
    }

    Ok(())
}
