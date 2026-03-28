use std::collections::BTreeSet;

use crate::cli::TextBereich;
use crate::domain::parser::legacy_cli_typed::matches_any_alias;

fn token_is(token: &str, aliases: &[&str]) -> bool {
    matches_any_alias(token, aliases)
}

fn generated_alias_present(generated_befehle: &BTreeSet<String>, aliases: &[&str]) -> bool {
    generated_befehle
        .iter()
        .any(|token| token_is(token, aliases))
}

/// Best-effort Auflösung für generierte Spaltenpaare aus words.py.
/// Das erste Wort ist die Hauptkategorie (`ParametersMain.*`), das zweite Wort
/// ein Alias des konkreten Eintrags in `paraNdataMatrix`.
///
/// Rückgabe:
/// - `generated_befehle` wird mit dem passenden Generator-Tag ergänzt
/// - `required_columns` bekommt die Basisspalten, die der Generator erwartet
pub fn try_resolve_generated_pair(
    ober: &str,
    unter: &str,
    generated_befehle: &mut BTreeSet<String>,
    required_columns: &mut BTreeSet<usize>,
) -> bool {
    let is_ober = |aliases: &[&str]| matches_any_alias(ober, aliases);
    let is_unter = |aliases: &[&str]| matches_any_alias(unter, aliases);

    // words.py:
    // - ParametersMain.procontra + ("Primzahlkreuz pro contra", "primzahlkreuz")
    // - ParametersMain.bedeutung + ("Primzahlkreuz pro contra", primzahlkreuzWort)
    // -> {"primzahlkreuzprocontra"}
    if is_ober(&["procontra", "pro_contra", "bedeutung", "grundstrukturen"])
        && is_unter(&[
            "primzahlkreuz",
            "primzahlkreuz pro contra",
            "nachvollziehen",
            "nachvollziehen emotional oder geistig durch primzahl kreuz algorithmus",
        ])
    {
        generated_befehle.insert("primzahlkreuzprocontra".to_string());
        return true;
    }

    // words.py:
    // ParametersMain.menschliches + ("Liebe", "liebe", "ethik") -> {8, 9, 28, ...}
    if is_ober(&["menschliches"]) && is_unter(&["liebe", "ethik"]) {
        generated_befehle.insert("lovepolygon".to_string());
        required_columns.insert(9);
        return true;
    }

    // words.py:
    // ParametersMain.menschliches + ("Gleichheit_Freiheit", ...)
    if is_ober(&["planet", "planet_(10_und_oder_12)", "menschliches", "grundstrukturen"])
        && is_unter(&[
            "ordnen",
            "ordnung",
            "filterung",
            "gleichheitfreiheit",
            "gleichheitfreiheitordnung",
            "gleichheit freiheit",
            "ungleichheit",
            "dominieren",
            "gleichheit",
            "freiheit",
            "ordnung und filterung 12 und 1pro12",
        ])
    {
        generated_befehle.insert("gleichheitfreiheit".to_string());
        required_columns.insert(132);
        return true;
    }

    // Geist/Emotion/etc.
    if is_ober(&["universum", "multiversum", "grundstrukturen", "menschliches"])
        && is_unter(&[
            "geist",
            "bewusstsein",
            "emotionen",
            "gefuehle",
            "gefuehl",
            "gefühl",
            "gefühle",
        ])
    {
        generated_befehle.insert("geistemotionenergiematerietopologie".to_string());
        required_columns.insert(242);
        return true;
    }

    // 64-getriebene Generatoren
    if is_ober(&["wichtigste", "wichtigstes_zum_verstehen", "bedeutung"])
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
        ])
    {
        generated_befehle.insert("primcreativitytype".to_string());
        generated_befehle.insert("mondexponzierenlogarithmustyp".to_string());
        required_columns.insert(64);
        return true;
    }

    // concatVervielfacheZeile arbeitet mit 19 / 90.
    if is_ober(&["bedeutung", "wichtigste", "wichtigstes_zum_verstehen", "galaxie"])
        && is_unter(&[
            "primzahlen",
            "vielfache",
            "vielfacher",
            "multis",
            "multiplikationen",
        ])
    {
        generated_befehle.insert("vervielfachezeile".to_string());
        required_columns.insert(19);
        required_columns.insert(90);
        return true;
    }

    // Neu: primvielfache / multiplikationen -> Polygon-/Stern-Generatorfamilie
    let is_prim_generated_group = is_ober(&[
        "primvielfache",
        "primvielfach",
        "primvielfaches",
        "multiplikationen",
        "multiplikation",
    ]);

    if is_prim_generated_group && is_unter(&["motivgleichfoermig"]) {
        generated_befehle.insert("primmotgleichf".to_string());
        return true;
    }

    if is_prim_generated_group && is_unter(&["strukturgleichfoermig"]) {
        generated_befehle.insert("primstrukgleichf".to_string());
        return true;
    }

    if is_prim_generated_group && is_unter(&["motivstern"]) {
        generated_befehle.insert("primmotivstern".to_string());
        return true;
    }

    if is_prim_generated_group && is_unter(&["strukturstern"]) {
        generated_befehle.insert("primstrukturstern".to_string());
        return true;
    }

    if is_prim_generated_group && is_unter(&["motivgebrstern"]) {
        generated_befehle.insert("primmotivsterngebr".to_string());
        return true;
    }

    if is_prim_generated_group && is_unter(&["strukgebrstern"]) {
        generated_befehle.insert("primstruktursterngebr".to_string());
        return true;
    }

    if is_prim_generated_group && is_unter(&["motivgebrgleichf"]) {
        generated_befehle.insert("primmotgleichfgebr".to_string());
        return true;
    }

    if is_prim_generated_group && is_unter(&["strukgebrgleichf"]) {
        generated_befehle.insert("primstrukgleichfgebr".to_string());
        return true;
    }

    false
}

fn requires_full_table_for_generated(generated_befehle: &BTreeSet<String>) -> bool {
    generated_alias_present(generated_befehle, &[
        "primzahlkreuzprocontra",
        "lovepolygon",
        "gleichheitfreiheit",
        "geistemotionenergiematerietopologie",
        "primcreativitytype",
        "mondexponzierenlogarithmustyp",
        "vervielfachezeile",
        "modallogik",
    ])
}

fn build_original_line_numbers(bereich: &TextBereich, data_len: usize) -> Vec<usize> {
    if !bereich.zeilen_bereiche.is_empty() {
        let mut nums = Vec::new();

        for &(from, to) in &bereich.zeilen_bereiche {
            if from == 0 || to == 0 || from > to {
                continue;
            }

            for n in from..=to {
                nums.push(n);
            }
        }

        nums.sort_unstable();
        nums.dedup();

        if nums.len() > data_len {
            nums.truncate(data_len);
        }

        return nums;
    }

    if bereich.von_zeile > 0 && bereich.bis_zeile >= bereich.von_zeile {
        let mut nums: Vec<usize> = (bereich.von_zeile..=bereich.bis_zeile).collect();

        if nums.len() > data_len {
            nums.truncate(data_len);
        }

        return nums;
    }

    (1..=data_len).collect()
}

pub fn should_compute_generated_from_full_table(
    bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
    data_len: usize,
) -> (bool, Vec<usize>) {
    let original_line_numbers = build_original_line_numbers(bereich, data_len);
    let use_full_table = requires_full_table_for_generated(generated_befehle);
    (use_full_table, original_line_numbers)
}
