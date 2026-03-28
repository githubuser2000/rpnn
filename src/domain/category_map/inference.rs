use super::normalize::normalize_key;
use crate::domain::categories::GeneratedInference;

pub fn infer_generated_pair_from_direct_columns<F>(
    ober: &str,
    unter: &str,
    mut find_direct: F,
) -> Option<GeneratedInference>
where
    F: FnMut(&str, &str) -> Vec<u32>,
{
    let ober_n = normalize_key(ober);
    let unter_n = normalize_key(unter);

    let mut direct_columns = find_direct(ober, unter);
    direct_columns.sort();
    direct_columns.dedup();

    let has = |n: u32| direct_columns.contains(&n);

    let mut generated_befehle = Vec::<String>::new();
    let mut required_columns = Vec::<u32>::new();

    if matches!(ober_n.as_str(), "procontra" | "bedeutung" | "universum")
        && matches!(unter_n.as_str(), "primzahlkreuz" | "primzahlkreuzprocontra")
    {
        generated_befehle.push("primzahlkreuzprocontra".to_string());
    }
    if has(9) { generated_befehle.push("lovepolygon".to_string()); required_columns.push(9); }
    if has(132) { generated_befehle.push("gleichheitfreiheit".to_string()); required_columns.push(132); }
    if has(242) { generated_befehle.push("geistemotionenergiematerietopologie".to_string()); required_columns.push(242); }
    if has(64) {
        generated_befehle.push("primcreativitytype".to_string());
        generated_befehle.push("mondexponzierenlogarithmustyp".to_string());
        required_columns.push(64);
    }
    if has(19) || has(90) {
        generated_befehle.push("vervielfachezeile".to_string());
        if has(19) { required_columns.push(19); }
        if has(90) { required_columns.push(90); }
    }

    generated_befehle.sort(); generated_befehle.dedup();
    required_columns.sort(); required_columns.dedup();

    if generated_befehle.is_empty() && direct_columns.is_empty() {
        None
    } else {
        Some(GeneratedInference { generated_befehle, required_columns, direct_columns })
    }
}
