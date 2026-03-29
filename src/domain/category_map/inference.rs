use crate::domain::python_source_of_truth::source_generated_inference_for_pair;
use crate::domain::categories::GeneratedInference;

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

    let source = source_generated_inference_for_pair(ober, unter);
    let mut generated_befehle = source.generated_befehle;
    let mut required_columns = source.required_columns;

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
