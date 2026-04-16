use std::collections::BTreeSet;
use crate::cli::{TextBereich, parser::SpaltenNamen};
use crate::column_categories_complete::KategorieMap;

fn normalize_category_key(s: &str) -> String {
    s.to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}

fn contains_any_alias(token: &str, aliases: &[&str]) -> bool {
    let t = normalize_category_key(token);
    aliases.iter().any(|a| normalize_category_key(a) == t)
}
fn map_fraction_category_to_pypy_compat(
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



fn match_any_alias(token: &str, aliases: &[&str]) -> bool {
    aliases.iter().any(|a| normalize_category_key(a) == token)
}

fn map_kombi_alias_to_index_galaxie(alias: &str) -> Option<usize> {
    let a = normalize_category_key(alias);

    if match_any_alias(&a, &["Lebewesen", "tiere", "tier", "lebewesen"]) {
        return Some(1);
    }
    if match_any_alias(&a, &["Berufe", "berufe", "beruf"]) {
        return Some(2);
    }
    if match_any_alias(&a, &["Kreativität_und_Intelligenz", "kreativität", "kreativitaet", "intelligenz"]) {
        return Some(3);
    }
    if match_any_alias(&a, &["Liebe", "liebe"]) {
        return Some(4);
    }
    if match_any_alias(&a, &["Männer", "männer", "maenner", "frauen"]) {
        return Some(7);
    }
    if match_any_alias(&a, &["Persönlichkeit_evolutionär_erwerben", "persönlichkeit", "persoenlichkeit", "evolution", "erwerben"]) {
        return Some(8);
    }
    if match_any_alias(&a, &["Religion", "religion", "religionen"]) {
        return Some(9);
    }
    if match_any_alias(&a, &["Motive_Ziele", "motivation", "ziele", "ziel", "motive"]) {
        return Some(10);
    }
    if match_any_alias(&a, &["Emotionen", "emotionen", "gefuehle", "gefühle", "gefuehl", "gefühl"]) {
        return Some(12);
    }
    if match_any_alias(&a, &["Personen", "personen", "berühmtheiten", "beruehmtheiten"]) {
        return Some(13);
    }
    if match_any_alias(&a, &["Wirtschaftssysteme", "wirtschaftssystem", "wirtschaftssysteme", "kombinierteswirtschaftssystem", "kombiniertewirtschaftssysteme"]) {
        return Some(16);
    }
    if match_any_alias(&a, &["Eigentum_und_Besitz", "eigentum", "besitz"]) {
        return Some(17);
    }

    None
}

fn map_kombi_alias_to_index_universum(alias: &str) -> Option<usize> {
    let a = normalize_category_key(alias);

    if match_any_alias(&a, &["Lebewesen", "tiere", "tier", "lebewesen"]) {
        return Some(1);
    }
    if match_any_alias(&a, &["Berufe", "berufe", "beruf"]) {
        return Some(2);
    }
    if match_any_alias(&a, &["Transzendentalien_Strukturalien", "transzendenz", "transzendentalien", "strukturalien", "alien"]) {
        return Some(5);
    }
    if match_any_alias(&a, &["Primzahlkreuz", "leibnitz", "primzahlkreuz"]) {
        return Some(6);
    }
    if match_any_alias(&a, &["Persönlichkeit_evolutionär_erwerben", "persönlichkeit", "persoenlichkeit", "evolution", "erwerben"]) {
        return Some(8);
    }
    if match_any_alias(&a, &["Religion", "religion", "religionen"]) {
        return Some(9);
    }
    if match_any_alias(&a, &["Motive_Ziele", "motivation", "motive", "ziele", "ziel"]) {
        return Some(10);
    }
    if match_any_alias(&a, &["analytische_Ontologie", "analytischeontologie", "ontologie"]) {
        return Some(11);
    }
    if match_any_alias(&a, &["Personen", "personen", "berühmtheiten", "beruehmtheiten"]) {
        return Some(13);
    }
    if match_any_alias(&a, &["Mechanismen_der_Zuechtung", "mechanismen", "wesen", "zuechten", "züchten"]) {
        return Some(14);
    }
    if match_any_alias(&a, &["Gegentranszendentalien", "gegentranszendentalien", "gegenstrukturalien"]) {
        return Some(15);
    }
    if match_any_alias(&a, &["Maschinen", "maschinen", "geräte", "geraete"]) {
        return Some(17);
    }
    if match_any_alias(&a, &["Geist", "geist"]) {
        return Some(18);
    }
    if match_any_alias(&a, &["Bewusstsein", "bewusstsein"]) {
        return Some(19);
    }

    None
}

fn map_kombi_category_to_pypy_compat(
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
    let is_primvielfache = matches!(ober_n.as_str(), "primvielfache" | "primvielfach" | "primvielfaches");
let is_multiplikationen = matches!(ober_n.as_str(), "multiplikationen" | "multiplikation");
let is_prim_generated_group = is_primvielfache || is_multiplikationen;

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

if is_prim_generated_group {
        if contains_any_alias(&unter_n, &["motivgleichfoermig", "motivgleichförmig", "motivegleichfoermigepolygone", "motivegleichförmige polygone"]) {
            out.insert("primmotivgleichf".to_string());
        }
        if contains_any_alias(&unter_n, &["strukturgleichfoermig", "strukturgleichförmig", "strukturgleichfoermigepolygone", "strukturgleichförmige polygone"]) {
            out.insert("primstrukgleichf".to_string());
        }
        if contains_any_alias(&unter_n, &["motivstern", "motivesternpolygone", "motivesternpolygon"]) {
            out.insert("primmotivstern".to_string());
        }
        if contains_any_alias(&unter_n, &["strukturstern", "struktursternpolygone", "struktursternpolygon"]) {
            out.insert("primstrukstern".to_string());
        }
        if contains_any_alias(&unter_n, &["motivgebrstern", "motivsternpolygongebrochenrational", "motivsternpolygongebrochen-rational"]) {
            out.insert("primmotivsterngebr".to_string());
        }
        if contains_any_alias(&unter_n, &["strukgebrstern", "struktursternpolyongebrochenrational", "struktursternpolygongebrochen-rational"]) {
            out.insert("primstruksterngebr".to_string());
        }
        if contains_any_alias(&unter_n, &["motivgebrgleichf", "motivgleichfoermigepolygonegebrochenrational", "motivgleichförmigepolygonegebrochen-rational"]) {
            out.insert("primmotivgleichfgebr".to_string());
        }
        if contains_any_alias(&unter_n, &["strukgebrgleichf", "strukturgleichfoermigepolygonegebrochenrational", "strukturgleichförmigepolygonegebrochen-rational"]) {
            out.insert("primstrukgleichfgebr".to_string());
        }
    }

    out
}

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
        bereich.spalten_gefunden = true;
        bereich.spalten_gesucht = true;
        bereich.spalten_gesucht2 = false;
    }

    let gefundene_spalten = kategorie_map.finde_spaltennummern_exakt(
        &spalten_namen.oberkategorie,
        &spalten_namen.unterkategorie,
    );

    if !gefundene_spalten.is_empty() {
        bereich.spalten_gefunden = true;
        bereich.spalten_gesucht = true;
        bereich.spalten_gesucht2 = false;

        // Nicht nur beim ersten Treffer setzen, sondern weitere exakte Treffer
        // an bestehende Auswahl anhängen.
        let mut combined = bereich.spaltenreihenfolgeundnurdiese.clone();
        combined.extend(gefundene_spalten.iter().map(|&x| x as usize));
        combined.sort_unstable();
        combined.dedup();
        bereich.spaltenreihenfolgeundnurdiese = combined.clone();

        bereich.exact_visible_columns.extend(combined.iter().copied());
        bereich.exact_visible_columns.sort_unstable();
        bereich.exact_visible_columns.dedup();

        bereich.spalten_bereiche = combined.iter().map(|&x| (x, x)).collect();

        if let Some(&(first, _)) = bereich.spalten_bereiche.first() {
            bereich.von_spalte = first;
        }
        if let Some(&(_, last)) = bereich.spalten_bereiche.last() {
            bereich.bis_spalte = last;
        }

        // WICHTIG:
        // Bei exaktem Treffer KEINE Generator-Heuristik mehr ausführen.
        return Ok(generated_befehle);
    }

    generated_befehle.extend(infer_generator_only_request(
        &spalten_namen.oberkategorie,
        &spalten_namen.unterkategorie,
    ));

    if !generated_befehle.is_empty() {
        bereich.spalten_gefunden = true;
        bereich.spalten_gesucht = false;
        bereich.spalten_gesucht2 = false;
        return Ok(generated_befehle);
    }

    if bereich.spalten_gefunden {
        return Ok(generated_befehle);
    }

    if spalten_namen.oberkategorie.trim().is_empty()
        && spalten_namen.unterkategorie.trim().is_empty()
    {
        return Ok(generated_befehle);
    }

    println!(
        "⚠️ Keine Kategorie-Spalten gefunden für: {} → {}",
        spalten_namen.oberkategorie,
        spalten_namen.unterkategorie
    );

    Ok(generated_befehle)
}
