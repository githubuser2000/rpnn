
use std::collections::BTreeSet;

use crate::domain::exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};

#[derive(Debug, Clone, Default)]
pub struct ExactResolved {
    pub direct_columns: Vec<usize>,
    pub modal_pairs: Vec<(usize, usize)>,
    pub meta_konkret_specs: Vec<(usize, usize)>,
    pub generated_befehle: BTreeSet<String>,
}

fn normalize_key(s: &str) -> String {
    s.to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}

fn dedup_vec<T: Ord + Clone>(items: &mut Vec<T>) {
    items.sort();
    items.dedup();
}

fn push_unique<T: PartialEq + Copy>(vec: &mut Vec<T>, value: T) {
    if !vec.contains(&value) {
        vec.push(value);
    }
}

fn resolve_meta_konkret(value: &str) -> Option<ExactResolved> {
    let key = normalize_key(value);
    let mut out = ExactResolved::default();

    for (aliases, pair) in META_KONKRET_MAPPINGS {
        if aliases.iter().any(|a| normalize_key(a) == key) {
            out.meta_konkret_specs.push(*pair);
            out.generated_befehle.insert("universummetakonkret".to_string());
            // Basis-Spalten für den Generator: Strukturalie + inverse Strukturalie.
            out.direct_columns.push(6);   // Python 0-based 5
            out.direct_columns.push(132); // Python 0-based 131
            // Zusätzliche Textspalten, die der Python-Generator im Universums-Fall referenziert.
            out.direct_columns.push(199); // Python 0-based 198
            out.direct_columns.push(202); // Python 0-based 201
            dedup_vec(&mut out.direct_columns);
            return Some(out);
        }
    }

    None
}

fn resolve_eigenschaften_like(value: &str) -> Option<ExactResolved> {
    let key = normalize_key(value);
    let mut out = ExactResolved::default();

    for (aliases, direct_columns, maybe_pair) in EIGENSCHAFT_MAPPINGS {
        if aliases.iter().any(|a| normalize_key(a) == key) {
            for &col in *direct_columns {
                // Rust/CLI arbeitet 1-basiert
                push_unique(&mut out.direct_columns, col + 1);
            }
            if let Some((a, b)) = maybe_pair {
                out.modal_pairs.push((*a, *b)); // hier 0-basiert lassen, concat_modallogik erwartet 0-basiert
                out.generated_befehle.insert("modallogik".to_string());
                push_unique(&mut out.direct_columns, *a + 1);
                push_unique(&mut out.direct_columns, *b + 1);
            }
            dedup_vec(&mut out.direct_columns);
            dedup_vec(&mut out.modal_pairs);
            return Some(out);
        }
    }

    None
}

pub fn resolve_exact_generator(ober: &str, unter: &str) -> Option<ExactResolved> {
    let ober_n = normalize_key(ober);

    match ober_n.as_str() {
        "universummetakonkret" | "metakonkret" => resolve_meta_konkret(unter),

        // Python-kompatible Aliasfamilie für ParametersMain.konzept2 / Eigenschaften_1/n
        "eigenschaft"
        | "eigenschaften"
        | "eigenschaftenn"
        | "eigenschaftenn1"
        | "eigenschaften1n"
        | "konzept"
        | "konzepte"
        | "konzept2"
        | "konzepte2" => resolve_eigenschaften_like(unter),

        _ => None,
    }
}

// Altpfad absichtlich inert.
pub fn try_run_exact_generator_bridge(
    _args: &[String],
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(false)
}
