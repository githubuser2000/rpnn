use std::collections::BTreeSet;
use crate::cli::{TextBereich, parser::SpaltenNamen};
use crate::column_categories_complete::KategorieMap;

fn normalize_category_key(s: &str) -> String {
    s.to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
}

fn contains_any_alias(token: &str, aliases: &[&str]) -> bool {
    let t = normalize_category_key(token);
    aliases.iter().any(|a| normalize_category_key(a) == t)
}

fn infer_generator_only_request(ober: &str, unter: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();

    let ober_n = normalize_category_key(ober);
    let unter_n = normalize_category_key(unter);

    let is_bedeutung = matches!(ober_n.as_str(), "bedeutung" | "wichtigsteverstehen" | "wichtigste");
    let is_procontra = matches!(ober_n.as_str(), "procontra" | "dagegendafuer");
    let is_universum = matches!(ober_n.as_str(), "universum" | "multiversum" | "grundstrukturen");
    let is_planet = ober_n == "planet" || ober_n == "planet(10undoder12)";
    let is_menschliches = ober_n == "menschliches";
    let is_galaxie = matches!(ober_n.as_str(), "galaxie" | "alteschriften" | "kreis" | "galaxien" | "kreise");

    if (is_bedeutung || is_procontra || is_universum)
        && contains_any_alias(&unter_n, &["primzahlkreuzprocontra", "primzahlkreuz"])
    {
        out.insert("primzahlkreuzprocontra".to_string());
    }

    if (is_menschliches || ober_n == "grundstrukturen")
        && contains_any_alias(&unter_n, &["liebe", "ethik"])
    {
        out.insert("lovepolygon".to_string());
    }

    if (is_planet || is_menschliches || ober_n == "grundstrukturen")
        && contains_any_alias(
            &unter_n,
            &[
                "gleichheit",
                "freiheit",
                "dominieren",
                "ordnung",
                "ordnen",
                "ordnenundfiltern",
                "filterung",
                "ungleichheit",
            ],
        )
    {
        out.insert("gleichheitfreiheit".to_string());
    }

    if is_universum
        && contains_any_alias(
            &unter_n,
            &[
                "geist",
                "bewusstsein",
                "emotion",
                "emotionen",
                "gefuehl",
                "gefuehle",
                "gefühl",
                "gefühle",
                "energie",
                "materie",
                "topologie",
            ],
        )
    {
        out.insert("geistemotionenergiematerietopologie".to_string());
    }

    if is_bedeutung
        && contains_any_alias(
            &unter_n,
            &[
                "gestirn",
                "mond",
                "sonne",
                "planet",
                "evolution",
                "intelligenz",
                "kreativ",
                "kreativitaet",
                "kreativität",
                "lernen",
                "erwerben",
            ],
        )
    {
        out.insert("primcreativitytype".to_string());
        out.insert("mondexponzierenlogarithmustyp".to_string());
    }

    if (is_bedeutung || is_galaxie)
        && contains_any_alias(
            &unter_n,
            &[
                "primzahlen",
                "vielfache",
                "vielfacher",
                "multis",
                "multiplikationen",
                "offenbarung",
                "offenbarungjohannes",
            ],
        )
    {
        out.insert("vervielfachezeile".to_string());
    }

    if contains_any_alias(
        &unter_n,
        &["modallogik", "modal", "modus", "modi", "sein", "zustaende", "zustände"],
    ) || contains_any_alias(
        &ober_n,
        &["modallogik", "modal", "modus", "modi", "sein", "zustaende", "zustände"],
    ) {
        out.insert("modallogik".to_string());
    }

    out
}

pub fn verarbeite_kategorien(
    kategorie_map: &KategorieMap,
    bereich: &mut TextBereich,
    spalten_namen: &SpaltenNamen,
) -> Result<BTreeSet<String>, Box<dyn std::error::Error>> {
    let mut generated_befehle = BTreeSet::new();

    let gefundene_spalten = kategorie_map.finde_spaltennummern_exakt(
        &spalten_namen.oberkategorie,
        &spalten_namen.unterkategorie,
    );

    if !gefundene_spalten.is_empty() {
        bereich.spalten_gefunden = true;

        // Wichtig: Eine explizit vom Benutzer gesetzte Reihenfolge wie
        // `--spaltenreihenfolgeundnurdiese 3,2,1` darf hier NICHT überschrieben werden.
        // Diese Reihenfolge bezieht sich auf die durch die Kategorie gefundenen Spalten
        // und wird später in query.rs auf die bereits selektierten Spalten angewendet.
        if bereich.spaltenreihenfolgeundnurdiese.is_empty() {
            bereich.spaltenreihenfolgeundnurdiese = gefundene_spalten
                .iter()
                .map(|&x| x as usize)
                .collect();
        }

        println!(
            "✅ Kategorie gefunden: {} → {} : {:?}",
            spalten_namen.oberkategorie,
            spalten_namen.unterkategorie,
            gefundene_spalten
        );
    }

    generated_befehle.extend(infer_generator_only_request(
        &spalten_namen.oberkategorie,
        &spalten_namen.unterkategorie,
    ));

    if !generated_befehle.is_empty() {
        bereich.spalten_gefunden = true;
        bereich.spalten_gesucht = false;
        bereich.spalten_gesucht2 = false;
        println!(
            "✅ Generator erkannt: {} → {} : {:?}",
            spalten_namen.oberkategorie,
            spalten_namen.unterkategorie,
            generated_befehle
        );
        return Ok(generated_befehle);
    }

    if bereich.spalten_gefunden {
        return Ok(generated_befehle);
    }

    println!(
        "⚠️ Keine Kategorie-Spalten gefunden für: {} → {}",
        spalten_namen.oberkategorie,
        spalten_namen.unterkategorie
    );

    Ok(generated_befehle)
}
