use crate::domain::categories::GeneratedInference;
use crate::domain::python_source_of_truth::{is_strict_generated_pair, source_generated_inference_for_pair};

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

    let mut source = source_generated_inference_for_pair(ober, unter).unwrap_or_default();
    if !is_strict_generated_pair(ober, unter) {
        source.direct_columns.extend(direct_columns.iter().copied());
        source.direct_columns.sort();
        source.direct_columns.dedup();
    }

    if source.generated_befehle.is_empty() && source.direct_columns.is_empty() {
        None
    } else {
        Some(GeneratedInference {
            generated_befehle: source.generated_befehle,
            required_columns: source.required_columns,
            direct_columns: source.direct_columns,
        })
    }
}
