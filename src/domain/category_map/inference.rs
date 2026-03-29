use super::normalize::normalize_key;
use crate::domain::categories::GeneratedInference;
use crate::domain::python_source_of_truth::source_generated_inference_for_pair;

pub fn infer_generated_pair_from_direct_columns<F>(
    ober: &str,
    unter: &str,
    mut find_direct: F,
) -> Option<GeneratedInference>
where
    F: FnMut(&str, &str) -> Vec<u32>,
{
    let _ober_n = normalize_key(ober);
    let _unter_n = normalize_key(unter);

    let mut direct_columns = find_direct(ober, unter);
    direct_columns.sort_unstable();
    direct_columns.dedup();

    let mut source = source_generated_inference_for_pair(ober, unter).unwrap_or_default();
    if source.direct_columns.is_empty() {
        source.direct_columns = direct_columns.clone();
    }
    if source.required_columns.is_empty() {
        source.required_columns = source.direct_columns.clone();
    }

    source.generated_befehle.sort();
    source.generated_befehle.dedup();
    source.required_columns.sort_unstable();
    source.required_columns.dedup();
    source.direct_columns.sort_unstable();
    source.direct_columns.dedup();

    if source.generated_befehle.is_empty() && source.direct_columns.is_empty() {
        None
    } else {
        Some(source)
    }
}
