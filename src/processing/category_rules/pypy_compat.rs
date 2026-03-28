use crate::cli::TextBereich;
use crate::domain::parser::legacy_cli_typed::{matches_any_alias, LegacyOberToken};

pub fn map_fraction_category_to_pypy_compat(
    bereich: &mut TextBereich,
    ober: &str,
    unter: &str,
) -> bool {
    let n = match unter.trim().parse::<usize>() {
        Ok(v) if (2..=23).contains(&v) => v,
        _ => return false,
    };

    match LegacyOberToken::parse(ober) {
        LegacyOberToken::GebrochenRationalGalaxie => {
            bereich.pypy_compat.gebrochengalaxie.insert(n);
        }
        LegacyOberToken::GebrochenRationalUniversum => {
            bereich.pypy_compat.gebrochenuniversum.insert(n);
        }
        LegacyOberToken::GebrochenRationalGefuehle => {
            bereich.pypy_compat.gebrochenemotion.insert(n);
        }
        LegacyOberToken::GebrochenRationalStrukturgroesse => {
            bereich.pypy_compat.gebrochengroesse.insert(n);
        }
        _ => return false,
    }

    true
}

fn map_kombi_alias_to_index_galaxie(alias: &str) -> Option<usize> {
    if matches_any_alias(alias, &["Lebewesen", "tiere", "tier", "lebewesen"]) {
        return Some(1);
    }
    if matches_any_alias(alias, &["Berufe", "berufe", "beruf"]) {
        return Some(2);
    }
    if matches_any_alias(
        alias,
        &["Kreativität und Intelligenz", "kreativität", "kreativitaet", "intelligenz"],
    ) {
        return Some(3);
    }
    if matches_any_alias(alias, &["Liebe", "liebe"]) {
        return Some(4);
    }
    if matches_any_alias(alias, &["Männer", "männer", "maenner", "frauen"]) {
        return Some(7);
    }
    if matches_any_alias(
        alias,
        &[
            "Persönlichkeit evolutionär erwerben",
            "persönlichkeit",
            "persoenlichkeit",
            "evolution",
            "erwerben",
        ],
    ) {
        return Some(8);
    }
    if matches_any_alias(alias, &["Religion", "religion", "religionen"]) {
        return Some(9);
    }
    if matches_any_alias(alias, &["Motive Ziele", "motivation", "ziele", "ziel", "motive"]) {
        return Some(10);
    }
    if matches_any_alias(
        alias,
        &["Emotionen", "emotionen", "gefuehle", "gefühle", "gefuehl", "gefühl"],
    ) {
        return Some(12);
    }
    if matches_any_alias(alias, &["Personen", "personen", "berühmtheiten", "beruehmtheiten"]) {
        return Some(13);
    }
    if matches_any_alias(
        alias,
        &[
            "Wirtschaftssysteme",
            "wirtschaftssystem",
            "wirtschaftssysteme",
            "kombinierteswirtschaftssystem",
            "kombiniertewirtschaftssysteme",
        ],
    ) {
        return Some(16);
    }
    if matches_any_alias(alias, &["Eigentum und Besitz", "eigentum", "besitz"]) {
        return Some(17);
    }

    None
}

fn map_kombi_alias_to_index_universum(alias: &str) -> Option<usize> {
    if matches_any_alias(alias, &["Lebewesen", "tiere", "tier", "lebewesen"]) {
        return Some(1);
    }
    if matches_any_alias(alias, &["Berufe", "berufe", "beruf"]) {
        return Some(2);
    }
    if matches_any_alias(
        alias,
        &[
            "Transzendentalien Strukturalien",
            "transzendenz",
            "transzendentalien",
            "strukturalien",
            "alien",
        ],
    ) {
        return Some(5);
    }
    if matches_any_alias(alias, &["Primzahlkreuz", "leibnitz", "primzahlkreuz"]) {
        return Some(6);
    }
    if matches_any_alias(
        alias,
        &[
            "Persönlichkeit evolutionär erwerben",
            "persönlichkeit",
            "persoenlichkeit",
            "evolution",
            "erwerben",
        ],
    ) {
        return Some(8);
    }
    if matches_any_alias(alias, &["Religion", "religion", "religionen"]) {
        return Some(9);
    }
    if matches_any_alias(alias, &["Motive Ziele", "motivation", "motive", "ziele", "ziel"]) {
        return Some(10);
    }
    if matches_any_alias(alias, &["analytische Ontologie", "analytischeontologie", "ontologie"]) {
        return Some(11);
    }
    if matches_any_alias(alias, &["Personen", "personen", "berühmtheiten", "beruehmtheiten"]) {
        return Some(13);
    }
    if matches_any_alias(alias, &["Mechanismen der Zuechtung", "mechanismen", "wesen", "zuechten", "züchten"]) {
        return Some(14);
    }
    if matches_any_alias(alias, &["Gegentranszendentalien", "gegentranszendentalien", "gegenstrukturalien"]) {
        return Some(15);
    }
    if matches_any_alias(alias, &["Maschinen", "maschinen", "geräte", "geraete"]) {
        return Some(17);
    }
    if matches_any_alias(alias, &["Geist", "geist"]) {
        return Some(18);
    }
    if matches_any_alias(alias, &["Bewusstsein", "bewusstsein"]) {
        return Some(19);
    }

    None
}

pub fn map_kombi_category_to_pypy_compat(
    bereich: &mut TextBereich,
    ober: &str,
    unter: &str,
) -> bool {
    match LegacyOberToken::parse(ober) {
        LegacyOberToken::KombinationGalaxie => {
            if let Some(idx) = map_kombi_alias_to_index_galaxie(unter) {
                bereich.pypy_compat.kombi_galaxie.insert(idx);
                return true;
            }
        }
        LegacyOberToken::KombinationUniversum => {
            if let Some(idx) = map_kombi_alias_to_index_universum(unter) {
                bereich.pypy_compat.kombi_universum.insert(idx);
                return true;
            }
        }
        _ => {}
    }

    false
}
