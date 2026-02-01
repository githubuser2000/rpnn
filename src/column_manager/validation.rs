// file: column_manager/validation.rs
use crate::cli::TextBereich;

pub fn validate_spalten_input(
    bereich: &TextBereich,
    wurde_spalten_gesucht: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !bereich.spalten_gesucht {
        return Err("Kein Spalten-Input angegeben".into());
    }

    if wurde_spalten_gesucht && bereich.spalten_bereiche.is_empty() {
        return Err("--spalten wurde angegeben, aber keine Spalten gefunden".into());
    }

    Ok(())
}
