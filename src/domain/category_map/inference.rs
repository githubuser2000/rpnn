use crate::domain::categories::GeneratedInference;
use crate::processing::category_rules::generator_inference::infer_generator_only_request;

pub fn infer_generated_pair_from_direct_columns<F>(
    ober: &str,
    unter: &str,
    mut find_direct: F,
) -> Option<GeneratedInference>
where
    F: FnMut(&str, &str) -> Vec<u32>,
{
    let mut direct_columns = find_direct(ober, unter);
    direct_columns.sort();
    direct_columns.dedup();

    let mut generated_befehle: Vec<String> = infer_generator_only_request(ober, unter).into_iter().collect();
    let mut required_columns = Vec::<u32>::new();

    for generator in &generated_befehle {
        match generator.as_str() {
            "lovepolygon" => required_columns.push(9),
            "gleichheitfreiheit" => required_columns.push(132),
            "geistemotionenergiematerietopologie" => required_columns.push(242),
            "primcreativitytype" | "mondexponzierenlogarithmustyp" => required_columns.push(64),
            "vervielfachezeile" => { required_columns.push(19); required_columns.push(90); }
            _ => {}
        }
    }

    generated_befehle.sort();
    generated_befehle.dedup();
    required_columns.sort();
    required_columns.dedup();

    if generated_befehle.is_empty() && direct_columns.is_empty() {
        None
    } else {
        Some(GeneratedInference { generated_befehle, required_columns, direct_columns })
    }
}
