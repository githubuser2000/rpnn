use crate::domain::python_source_of_truth::source_generated_inference_for_pair;
use std::collections::BTreeSet;
use super::normalize::{contains_any_alias, normalize_category_key};

pub fn infer_generator_only_request(ober: &str, unter: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let source = source_generated_inference_for_pair(ober, unter);
    out.extend(source.generated_befehle);

    let ober_n = normalize_category_key(ober);
    let unter_n = normalize_category_key(unter);
    let is_primvielfache = matches!(ober_n.as_str(), "primvielfache" | "primvielfach" | "primvielfaches");
    let is_multiplikationen = matches!(ober_n.as_str(), "multiplikationen" | "multiplikation");
    let is_prim_generated_group = is_primvielfache || is_multiplikationen;

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
