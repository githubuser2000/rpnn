use crate::cli::TextBereich;
use super::normalize::{match_any_alias, normalize_category_key};

pub fn map_fraction_category_to_pypy_compat(
    bereich: &mut TextBereich,
    ober: &str,
    unter: &str,
) -> bool {
    let ober_n = normalize_category_key(ober);
    let unter_n = normalize_category_key(unter);

    let n = match unter_n.parse::<usize>() {
        Ok(v) if (2..=23).contains(&v) => v,
        _ => return false,
    };

    match ober_n.as_str() {
        "gebrochenrationalgalaxienm"
        | "gebrochenrationalgalaxien"
        | "gebrochenrationalgalaxiennm"
        | "gebrochengalaxie" => {
            bereich.pypy_compat.gebrochengalaxie.insert(n);
        }
        "gebrochenrationaluniversumnm"
        | "gebrochenrationaluniversum"
        | "gebrochenrationaluniversumn"
        | "gebrochenuniversum" => {
            bereich.pypy_compat.gebrochenuniversum.insert(n);
        }
        "gebrochenrationalgefuehlenm"
        | "gebrochenrationalgefuehle"
        | "gebrochenrationalemotionen"
        | "gebrochenemotion" => {
            bereich.pypy_compat.gebrochenemotion.insert(n);
        }
        "gebrochenrationalstrukturgroessenm"
        | "gebrochenrationalstrukturgroesse"
        | "gebrochenrationalgroesse"
        | "gebrochengroesse" => {
            bereich.pypy_compat.gebrochengroesse.insert(n);
        }
        _ => return false,
    }

    true
}

fn map_kombi_alias_to_index_galaxie(alias: &str) -> Option<usize> {
    let a = normalize_category_key(alias);

    if match_any_alias(&a, &["Lebewesen", "tiere", "tier", "lebewesen"]) { return Some(1); }
    if match_any_alias(&a, &["Berufe", "berufe", "beruf"]) { return Some(2); }
    if match_any_alias(&a, &["Kreativität_und_Intelligenz", "kreativität", "kreativitaet", "intelligenz"]) { return Some(3); }
    if match_any_alias(&a, &["Liebe", "liebe"]) { return Some(4); }
    if match_any_alias(&a, &["Männer", "männer", "maenner", "frauen"]) { return Some(7); }
    if match_any_alias(&a, &["Persönlichkeit_evolutionär_erwerben", "persönlichkeit", "persoenlichkeit", "evolution", "erwerben"]) { return Some(8); }
    if match_any_alias(&a, &["Religion", "religion", "religionen"]) { return Some(9); }
    if match_any_alias(&a, &["Motive_Ziele", "motivation", "ziele", "ziel", "motive"]) { return Some(10); }
    if match_any_alias(&a, &["Emotionen", "emotionen", "gefuehle", "gefühle", "gefuehl", "gefühl"]) { return Some(12); }
    if match_any_alias(&a, &["Personen", "personen", "berühmtheiten", "beruehmtheiten"]) { return Some(13); }
    if match_any_alias(&a, &["Wirtschaftssysteme", "wirtschaftssystem", "wirtschaftssysteme", "kombinierteswirtschaftssystem", "kombiniertewirtschaftssysteme"]) { return Some(16); }
    if match_any_alias(&a, &["Eigentum_und_Besitz", "eigentum", "besitz"]) { return Some(17); }

    None
}

fn map_kombi_alias_to_index_universum(alias: &str) -> Option<usize> {
    let a = normalize_category_key(alias);

    if match_any_alias(&a, &["Lebewesen", "tiere", "tier", "lebewesen"]) { return Some(1); }
    if match_any_alias(&a, &["Berufe", "berufe", "beruf"]) { return Some(2); }
    if match_any_alias(&a, &["Transzendentalien_Strukturalien", "transzendenz", "transzendentalien", "strukturalien", "alien"]) { return Some(5); }
    if match_any_alias(&a, &["Primzahlkreuz", "leibnitz", "primzahlkreuz"]) { return Some(6); }
    if match_any_alias(&a, &["Persönlichkeit_evolutionär_erwerben", "persönlichkeit", "persoenlichkeit", "evolution", "erwerben"]) { return Some(8); }
    if match_any_alias(&a, &["Religion", "religion", "religionen"]) { return Some(9); }
    if match_any_alias(&a, &["Motive_Ziele", "motivation", "motive", "ziele", "ziel"]) { return Some(10); }
    if match_any_alias(&a, &["analytische_Ontologie", "analytischeontologie", "ontologie"]) { return Some(11); }
    if match_any_alias(&a, &["Personen", "personen", "berühmtheiten", "beruehmtheiten"]) { return Some(13); }
    if match_any_alias(&a, &["Mechanismen_der_Zuechtung", "mechanismen", "wesen", "zuechten", "züchten"]) { return Some(14); }
    if match_any_alias(&a, &["Gegentranszendentalien", "gegentranszendentalien", "gegenstrukturalien"]) { return Some(15); }
    if match_any_alias(&a, &["Maschinen", "maschinen", "geräte", "geraete"]) { return Some(17); }
    if match_any_alias(&a, &["Geist", "geist"]) { return Some(18); }
    if match_any_alias(&a, &["Bewusstsein", "bewusstsein"]) { return Some(19); }

    None
}

pub fn map_kombi_category_to_pypy_compat(
    bereich: &mut TextBereich,
    ober: &str,
    unter: &str,
) -> bool {
    let ober_n = normalize_category_key(ober);

    match ober_n.as_str() {
        "kombinationgalaxie" | "kombigalaxie" => {
            if let Some(idx) = map_kombi_alias_to_index_galaxie(unter) {
                bereich.pypy_compat.kombi_galaxie.insert(idx);
                return true;
            }
        }
        "kombinationuniversum" | "kombiuniversum" => {
            if let Some(idx) = map_kombi_alias_to_index_universum(unter) {
                bereich.pypy_compat.kombi_universum.insert(idx);
                return true;
            }
        }
        _ => {}
    }

    false
}
