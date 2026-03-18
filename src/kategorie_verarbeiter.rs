use std::collections::BTreeSet;
use crate::cli::{TextBereich, parser::SpaltenNamen};
use crate::column_categories_complete::KategorieMap;

fn normalize_category_key(s: &str) -> String {
    s.to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
}

fn is_primzahlkreuz_pro_contra_request(ober: &str, unter: &str) -> bool {
    let ober = normalize_category_key(ober);
    let unter = normalize_category_key(unter);

    let ober_ok = matches!(
        ober.as_str(),
        "bedeutung" | "procontra" | "universum"
    );

    let unter_ok = matches!(
        unter.as_str(),
        "primzahlkreuzprocontra" | "primzahlkreuz"
    );

    ober_ok && unter_ok
}

// 4. Funktion: Verarbeitung der Kategorie-Map
pub fn verarbeite_kategorien(
    kategorie_map: &KategorieMap,
    bereich: &mut TextBereich,
    spalten_namen: &SpaltenNamen,
) -> Result<BTreeSet<String>, Box<dyn std::error::Error>> {
    let mut generated_befehle = BTreeSet::new();

    if is_primzahlkreuz_pro_contra_request(
        &spalten_namen.oberkategorie,
        &spalten_namen.unterkategorie,
    ) {
        generated_befehle.insert("primzahlkreuzprocontra".to_string());
        
        // ganz wichtig: sonst läuft später der "keine Spalten gefunden"-Fehler an
        bereich.spalten_gefunden = true;
        bereich.spalten_gesucht = false;
        bereich.spalten_gesucht2 = false;
        return Ok(generated_befehle);
    }

    let gefundene_spalten = kategorie_map.finde_spaltennummern_exakt(
        &spalten_namen.oberkategorie,
        &spalten_namen.unterkategorie,
    );

    if !gefundene_spalten.is_empty() {
        bereich.spalten_gefunden = true;
        bereich.spaltenreihenfolgeundnurdiese = gefundene_spalten
            .iter()
            .map(|&x| x as usize)
            .collect();
        println!(
            "✅ Kategorie gefunden: {} → {} : {:?}",
            spalten_namen.oberkategorie,
            spalten_namen.unterkategorie,
            gefundene_spalten
        );
    } else {
        println!(
            "⚠️ Keine Kategorie-Spalten gefunden für: {} → {}",
            spalten_namen.oberkategorie,
            spalten_namen.unterkategorie
        );
    }

    let _ = bereich;

    Ok(generated_befehle)
}
