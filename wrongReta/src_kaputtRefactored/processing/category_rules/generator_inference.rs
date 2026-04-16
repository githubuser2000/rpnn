use std::collections::BTreeSet;
use super::normalize::{contains_any_alias, normalize_category_key};

pub fn infer_generator_only_request(ober: &str, unter: &str) -> BTreeSet<String> {
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
        && contains_any_alias(&unter_n, &["primzahlkreuzprocontra", "primzahlkreuz"]) {
        out.insert("primzahlkreuzprocontra".to_string());
    }
    if (is_menschliches || ober_n == "grundstrukturen") && contains_any_alias(&unter_n, &["liebe", "ethik"]) {
        out.insert("lovepolygon".to_string());
    }
    if (is_planet || is_menschliches || ober_n == "grundstrukturen")
        && contains_any_alias(&unter_n, &["gleichheit", "freiheit", "dominieren", "ordnung", "ordnen", "ordnenundfiltern", "filterung", "ungleichheit"]) {
        out.insert("gleichheitfreiheit".to_string());
    }
    if is_universum && contains_any_alias(&unter_n, &["geist", "bewusstsein", "emotion", "emotionen", "gefuehl", "gefuehle", "gefühl", "gefühle", "energie", "materie", "topologie"]) {
        out.insert("geistemotionenergiematerietopologie".to_string());
    }
    if is_bedeutung && contains_any_alias(&unter_n, &["gestirn", "mond", "sonne", "planet", "evolution", "intelligenz", "kreativ", "kreativitaet", "kreativität", "lernen", "erwerben"]) {
        out.insert("primcreativitytype".to_string());
        out.insert("mondexponzierenlogarithmustyp".to_string());
    }
    if (is_bedeutung || is_galaxie) && contains_any_alias(&unter_n, &["primzahlen", "vielfache", "vielfacher", "multis", "multiplikationen", "offenbarung", "offenbarungjohannes"]) {
        out.insert("vervielfachezeile".to_string());
    }
    if contains_any_alias(&unter_n, &["modallogik", "modal", "modus", "modi", "sein", "zustaende", "zustände"])
        || contains_any_alias(&ober_n, &["modallogik", "modal", "modus", "modi", "sein", "zustaende", "zustände"]) {
        out.insert("modallogik".to_string());
    }
    if is_prim_generated_group {
        if contains_any_alias(&unter_n, &["motivgleichfoermig", "motivgleichförmig", "motivegleichfoermigepolygone", "motivegleichförmige polygone"]) { out.insert("primmotivgleichf".to_string()); }
        if contains_any_alias(&unter_n, &["strukturgleichfoermig", "strukturgleichförmig", "strukturgleichfoermigepolygone", "strukturgleichförmige polygone"]) { out.insert("primstrukgleichf".to_string()); }
        if contains_any_alias(&unter_n, &["motivstern", "motivesternpolygone", "motivesternpolygon"]) { out.insert("primmotivstern".to_string()); }
        if contains_any_alias(&unter_n, &["strukturstern", "struktursternpolygone", "struktursternpolygon"]) { out.insert("primstrukstern".to_string()); }
        if contains_any_alias(&unter_n, &["motivgebrstern", "motivsternpolygongebrochenrational", "motivsternpolygongebrochen-rational"]) { out.insert("primmotivsterngebr".to_string()); }
        if contains_any_alias(&unter_n, &["strukgebrstern", "struktursternpolyongebrochenrational", "struktursternpolygongebrochen-rational"]) { out.insert("primstruksterngebr".to_string()); }
        if contains_any_alias(&unter_n, &["motivgebrgleichf", "motivgleichfoermigepolygonegebrochenrational", "motivgleichförmigepolygonegebrochen-rational"]) { out.insert("primmotivgleichfgebr".to_string()); }
        if contains_any_alias(&unter_n, &["strukgebrgleichf", "strukturgleichfoermigepolygonegebrochenrational", "strukturgleichförmigepolygonegebrochen-rational"]) { out.insert("primstrukgleichfgebr".to_string()); }
    }

    out
}
