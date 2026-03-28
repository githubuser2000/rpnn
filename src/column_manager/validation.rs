// file: column_manager/validation.rs
use std::collections::BTreeSet;
use crate::cli::TextBereich;
use crate::domain::parser::legacy_cli_typed::matches_any_alias;

/// Gleiche Pair-Logik wie in table_printer/query.rs, aber leichtgewichtig für die Validierung.
/// So kann `--spaltenname planet ordnen` schon in der Validation als potenziell
/// generierter Fall akzeptiert werden, statt sofort mit "keine Spalten gefunden"
/// abzubrechen.
pub fn is_generated_pair_alias(ober: &str, unter: &str) -> bool {
    let is_ober = |aliases: &[&str]| matches_any_alias(ober, aliases);
    let is_unter = |aliases: &[&str]| matches_any_alias(unter, aliases);

    let is_prim_generated_group =
        is_ober(&["primvielfache", "primvielfach", "primvielfaches", "multiplikationen", "multiplikation"]);

    (is_ober(&["procontra", "pro_contra", "bedeutung", "grundstrukturen"])
        && is_unter(&["primzahlkreuz", "nachvollziehen"]))
        || (is_ober(&["menschliches"]) && is_unter(&["liebe", "ethik"]))
        || (is_ober(&["planet", "planet_(10_und_oder_12)", "menschliches", "grundstrukturen"])
            && is_unter(&[
                "ordnen",
                "ordnung",
                "filterung",
                "gleichheitfreiheit",
                "ungleichheit",
                "dominieren",
                "gleichheit",
                "freiheit",
            ]))
        || (is_ober(&["universum", "multiversum", "grundstrukturen", "menschliches"])
            && is_unter(&[
                "geist",
                "bewusstsein",
                "emotionen",
                "gefuehle",
                "gefuehl",
                "gefühl",
                "gefühle",
            ]))
        || (is_ober(&["wichtigste", "wichtigstes_zum_verstehen", "bedeutung"])
            && is_unter(&[
                "gestirn",
                "mond",
                "sonne",
                "planet",
                "evolution",
                "erwerben",
                "persoenlichkeit",
                "persönlichkeit",
                "kreativitaet",
                "kreativität",
                "intelligenz",
            ]))
        || (is_ober(&["bedeutung", "wichtigste", "wichtigstes_zum_verstehen", "galaxie", "multiplikationen", "primvielfache"])
            && is_unter(&[
                "primzahlen",
                "vielfache",
                "vielfacher",
                "multis",
                "multiplikationen",
            ]))
        || (is_prim_generated_group
            && is_unter(&[
                "motivgleichfoermig",
                "strukturgleichfoermig",
                "motivstern",
                "strukturstern",
                "motivgebrstern",
                "strukgebrstern",
                "motivgebrgleichf",
                "strukgebrgleichf",
            ]))
}

fn validate_spalten_input_inner(
    bereich: &TextBereich,
    generated_pair_detected: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !bereich.columns_requested() {
        return Err("Kein Spalten-Input angegeben".into());
    }

    if bereich.columns_pending()
        && bereich.spalten_bereiche.is_empty()
        && !generated_pair_detected
    {
        return Err("--spaltenname wurde angegeben, aber keine Spalten gefunden".into());
    }

    Ok(())
}

/// Rückwärtskompatibel: alter Aufruf verhält sich wie bisher.
pub fn validate_spalten_input(
    bereich: &TextBereich,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_spalten_input_inner(bereich, false)
}

/// Neuer Aufruf für `--spaltenname ober unter`, wenn vor der Validation geprüft werden soll,
/// ob das Paar ein generierter Fall aus words.py ist.
pub fn validate_spalten_input_with_pair(
    bereich: &TextBereich,
    ober: &str,
    unter: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_spalten_input_inner(bereich, is_generated_pair_alias(ober, unter))
}

/// Optionaler Aufruf, falls der Parser die Auflösung schon durchgeführt hat und die
/// generierten Befehle bereits kennt.
pub fn validate_spalten_input_with_generated(
    bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let generated_pair_detected = !generated_befehle.is_empty();
    validate_spalten_input_inner(bereich, generated_pair_detected)
}
