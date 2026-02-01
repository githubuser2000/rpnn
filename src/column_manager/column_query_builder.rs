// file: column_manager/column_query_builder.rs
use crate::cli::TextBereich;
use super::column_selector::{collect_spalten_nummern, resolve_spaltennamen};
use super::row_query_builder::build_row_query;
use super::validation::validate_spalten_input;

pub fn build_column_query(
    column_names: &[String],
    bereich: &mut TextBereich,
    wurde_spalten_gesucht: bool,
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    validate_spalten_input(bereich, wurde_spalten_gesucht)?;

    let spalten_nummern = collect_spalten_nummern(bereich, wurde_spalten_gesucht)?;
    let selected_names = resolve_spaltennamen(column_names, &spalten_nummern)?;
    let columns_clause = selected_names.join(", ");

    let query = build_row_query(&columns_clause, bereich)?;

    Ok((query, selected_names))
}

pub fn build_column_query_with_specific_columns(
    column_names: &[String],
    spalten_nummern: &[usize],
    zeilen_bereiche: &[(usize, usize)],
) -> Result<(String, Vec<String>), Box<dyn std::error::Error>> {
    println!("🔍 Baue Query mit spezifischen Spaltennummern: {:?}", spalten_nummern);
    
    if spalten_nummern.is_empty() {
        println!("❌ FEHLER: Keine Spaltennummern angegeben");
        return Err("Keine Spaltennummern angegeben".into());
    }
    
    let mut selected_names = Vec::new();
    
    for &nummer in spalten_nummern {
        if nummer == 0 || nummer > column_names.len() {
            println!("❌ FEHLER: Spaltennummer {} existiert nicht (Tabelle hat {} Spalten)", 
                     nummer, column_names.len());
            return Err(format!("Spaltennummer {} existiert nicht (Tabelle hat {} Spalten)", 
                               nummer, column_names.len()).into());
        }
        
        if let Some(name) = column_names.get(nummer.saturating_sub(1)) {
            selected_names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
        } else {
            println!("❌ FEHLER: Spaltennummer {} nicht gefunden", nummer);
            return Err(format!("Spaltennummer {} nicht gefunden", nummer).into());
        }
    }
    
    let columns_clause = selected_names.join(", ");
    println!("✅ Ausgewählte Spalten: {}", columns_clause);

    use super::row_query_builder::{build_query_with_row_ranges_enhanced, build_row_query};
    
    let bereich = TextBereich {
        zeilen_bereiche: zeilen_bereiche.to_vec(),
        ..Default::default()
    };
    
    let query = build_row_query(&columns_clause, &bereich)?;
    
    println!("✅ Generierte Query: {}", query);
    
    Ok((query, selected_names))
}
