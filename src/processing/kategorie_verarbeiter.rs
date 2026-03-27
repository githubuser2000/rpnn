use std::collections::BTreeSet;

use crate::cli::{parser::SpaltenNamen, TextBereich};
use crate::domain::categories::KategorieMap;
use crate::domain::spalten_anfrage::SpaltenAnfrage;
use crate::processing::category_rules::exact_columns::merge_exact_columns_into_bereich;
use crate::processing::category_rules::pypy_compat::{
    map_fraction_category_to_pypy_compat,
    map_kombi_category_to_pypy_compat,
};

pub fn verarbeite_kategorien(
    kategorie_map: &KategorieMap,
    bereich: &mut TextBereich,
    spalten_namen: &SpaltenNamen,
) -> Result<BTreeSet<String>, Box<dyn std::error::Error>> {
    let mut generated_befehle = BTreeSet::new();

    let fraction_requested = map_fraction_category_to_pypy_compat(
        bereich,
        &spalten_namen.oberkategorie,
        &spalten_namen.unterkategorie,
    );

    let kombi_requested = map_kombi_category_to_pypy_compat(
        bereich,
        &spalten_namen.oberkategorie,
        &spalten_namen.unterkategorie,
    );

    if fraction_requested || kombi_requested {
        bereich.mark_columns_resolved();
    }

    let request = match SpaltenAnfrage::parse(
        &spalten_namen.oberkategorie,
        &spalten_namen.unterkategorie,
    ) {
        Ok(req) => Some(req),
        Err(_) => None,
    };

    let gefundene_spalten = if let Some(req) = &request {
        kategorie_map.finde_spaltennummern_fuer_request(req)
    } else {
        kategorie_map.finde_spaltennummern_exakt(
            &spalten_namen.oberkategorie,
            &spalten_namen.unterkategorie,
        )
    };

    if !gefundene_spalten.is_empty() {
        merge_exact_columns_into_bereich(bereich, gefundene_spalten);
        return Ok(generated_befehle);
    }

    if let Some(req) = &request {
        generated_befehle.extend(req.generated_befehle_hint());
    }

    if generated_befehle.is_empty() {
        if let Some(req) = &request {
            if let Some(inference) = kategorie_map.infer_generated_request(req) {
                generated_befehle.extend(inference.generated_befehle);
            }
        }
    }

    if !generated_befehle.is_empty() {
        bereich.mark_columns_resolved();
        return Ok(generated_befehle);
    }

    if bereich.columns_resolved() {
        return Ok(generated_befehle);
    }

    if spalten_namen.oberkategorie.trim().is_empty()
        && spalten_namen.unterkategorie.trim().is_empty()
    {
        return Ok(generated_befehle);
    }

    println!(
        "⚠️ Keine Kategorie-Spalten gefunden für: {} → {}",
        spalten_namen.oberkategorie, spalten_namen.unterkategorie
    );

    Ok(generated_befehle)
}
