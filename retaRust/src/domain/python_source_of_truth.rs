use crate::shared::words_py::{PyValue, StoreParameterEntry, Words};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactPythonColumn {
    pub parameter_main_name: String,
    pub parameter_name: String,
    pub column_numbers: Vec<i64>,
    pub source_entry_index: usize,
    pub source_data_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactPythonColumnMeta {
    pub column_number: i64,
    pub direct_matches: Vec<ExactPythonColumn>,
}

fn primary_name(names: &[String]) -> String {
    names.first().cloned().unwrap_or_default()
}

fn extract_direct_columns(data: &[PyValue]) -> Vec<i64> {
    let mut out = Vec::new();
    for value in data {
        match value {
            PyValue::Int(v) => out.push(*v),
            // direct columns only; tuples are generator/meta shapes and stay out here on purpose
            PyValue::Tuple(values) => {
                if values.iter().all(|item| matches!(item, PyValue::Int(_))) {
                    // keep strict direct-column semantics: tuples are not direct columns
                }
            }
            PyValue::Str(_) | PyValue::Bool(_) | PyValue::NoneValue => {}
        }
    }
    out
}

fn entry_to_direct_column(entry: &StoreParameterEntry, entry_index: usize) -> Option<ExactPythonColumn> {
    let main_name = primary_name(&entry.parameterMainNames);
    let parameter_name = primary_name(&entry.parameterNames);

    for (data_index, data) in entry.datas.iter().enumerate() {
        let columns = extract_direct_columns(data);
        if !columns.is_empty() {
            return Some(ExactPythonColumn {
                parameter_main_name: main_name,
                parameter_name,
                column_numbers: columns,
                source_entry_index: entry_index,
                source_data_index: data_index,
            });
        }
    }

    None
}

pub fn exact_all_direct_columns(words: &Words) -> Vec<ExactPythonColumn> {
    words
        .paraNdataMatrix
        .iter()
        .enumerate()
        .filter_map(|(entry_index, entry)| entry_to_direct_column(entry, entry_index))
        .collect()
}

pub fn exact_all_direct_columns_for_pair(
    words: &Words,
    parameter_main_name: &str,
    parameter_name: &str,
) -> Vec<ExactPythonColumn> {
    exact_all_direct_columns(words)
        .into_iter()
        .filter(|entry| {
            entry.parameter_main_name == parameter_main_name && entry.parameter_name == parameter_name
        })
        .collect()
}

pub fn exact_meta_for_column(words: &Words, column_number: i64) -> Option<ExactPythonColumnMeta> {
    let direct_matches: Vec<ExactPythonColumn> = exact_all_direct_columns(words)
        .into_iter()
        .filter(|entry| entry.column_numbers.contains(&column_number))
        .collect();

    if direct_matches.is_empty() {
        None
    } else {
        Some(ExactPythonColumnMeta {
            column_number,
            direct_matches,
        })
    }
}

pub fn all_parameter_main_names(words: &Words) -> Vec<String> {
    let mut out = Vec::new();
    for entry in &words.paraNdataMatrix {
        let name = primary_name(&entry.parameterMainNames);
        if !name.is_empty() && !out.contains(&name) {
            out.push(name);
        }
    }
    out
}

pub fn parameter_names_for_main(words: &Words, parameter_main_name: &str) -> Vec<String> {
    let mut out = Vec::new();
    for entry in &words.paraNdataMatrix {
        if primary_name(&entry.parameterMainNames) == parameter_main_name {
            let name = primary_name(&entry.parameterNames);
            if !name.is_empty() && !out.contains(&name) {
                out.push(name);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::words_py::Words;

    #[test]
    fn direct_pair_lookup_finds_menschliches_motive() {
        let words = Words::new();
        let hits = exact_all_direct_columns_for_pair(&words, "Menschliches", "Motive")
            .into_iter()
            .flat_map(|entry| entry.column_numbers)
            .collect::<Vec<_>>();
        assert!(!hits.is_empty());
    }

    #[test]
    fn exact_meta_for_known_column_returns_match() {
        let words = Words::new();
        let meta = exact_meta_for_column(&words, 5).expect("known direct column should resolve");
        assert!(!meta.direct_matches.is_empty());
    }
}
