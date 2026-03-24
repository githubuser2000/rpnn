// file: column_manager/validation.rs
use std::collections::BTreeSet;
use crate::cli::TextBereich;

fn normalize_token(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss")
        .replace('-', "")
        .replace('_', "")
        .replace(' ', "")
}

/// Gleiche Pair-Logik wie in table_printer/query.rs, aber leichtgewichtig für die Validierung.
/// So kann `--spaltenname planet ordnen` schon in der Validation als potenziell
/// generierter Fall akzeptiert werden, statt sofort mit "keine Spalten gefunden"
/// abzubrechen.
pub fn is_generated_pair_alias(ober: &str, unter: &str) -> bool {
    let ober = normalize_token(ober);
    let unter = normalize_token(unter);

    let is_ober = |aliases: &[&str]| aliases.iter().any(|a| ober == normalize_token(a));
    let is_unter = |aliases: &[&str]| aliases.iter().any(|a| unter == normalize_token(a));

    (is_ober(&["procontra", "bedeutung", "grundstrukturen"])
        && is_unter(&["primzahlkreuz", "nachvollziehen"]))
        || (is_ober(&["menschliches"]) && is_unter(&["liebe", "ethik"]))
        || (is_ober(&["planet", "menschliches", "grundstrukturen"])
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
        || (is_ober(&["wichtigste", "bedeutung"])
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
        || (is_ober(&["bedeutung", "wichtigste", "galaxie"])
            && is_unter(&[
                "primzahlen",
                "vielfache",
                "vielfacher",
                "multis",
                "multiplikationen",
            ]))
}

fn validate_spalten_input_inner(
    bereich: &TextBereich,
    generated_pair_detected: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !bereich.spalten_gesucht {
        return Err("Kein Spalten-Input angegeben".into());
    }

    if bereich.spalten_gesucht2
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
