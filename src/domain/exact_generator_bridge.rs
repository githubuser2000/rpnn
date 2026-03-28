
use std::collections::BTreeSet;

use crate::domain::spalten_anfrage::SpaltenAnfrage;

use crate::domain::exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};
use crate::domain::parser::legacy_cli_typed::{matches_any_alias, LegacyOberToken};

#[derive(Debug, Clone, Default)]
pub struct ExactResolved {
    pub direct_columns: Vec<usize>,
    pub modal_pairs: Vec<(usize, usize)>,
    pub meta_konkret_specs: Vec<(usize, usize)>,
    pub generated_befehle: BTreeSet<String>,
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
    let mut out = ExactResolved::default();

    for (aliases, pair) in META_KONKRET_MAPPINGS {
        if matches_any_alias(value, aliases) {
            out.meta_konkret_specs.push(*pair);
            out.generated_befehle.insert("universummetakonkret".to_string());
            out.direct_columns.push(6);
            out.direct_columns.push(132);
            out.direct_columns.push(199);
            out.direct_columns.push(202);
            dedup_vec(&mut out.direct_columns);
            return Some(out);
        }
    }

    None
}

fn resolve_eigenschaften_like(value: &str) -> Option<ExactResolved> {
    let mut out = ExactResolved::default();

    for (aliases, direct_columns, maybe_pair) in EIGENSCHAFT_MAPPINGS {
        if matches_any_alias(value, aliases) {
            for &col in *direct_columns {
                push_unique(&mut out.direct_columns, col + 1);
            }
            if let Some((a, b)) = maybe_pair {
                out.modal_pairs.push((*a, *b));
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
    match LegacyOberToken::parse(ober) {
        LegacyOberToken::UniversumMetaKonkret => resolve_meta_konkret(unter),
        LegacyOberToken::Eigenschaften
        | LegacyOberToken::EigenschaftenN
        | LegacyOberToken::Eigenschaften1ProN => resolve_eigenschaften_like(unter),
        _ => None,
    }
}

pub fn try_run_exact_generator_bridge(
    _args: &[String],
) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(false)
}

pub fn resolve_exact_generator_for_request(request: &SpaltenAnfrage) -> Option<ExactResolved> {
    let (ober, unter) = request.ober_unter_cli_pair();
    resolve_exact_generator(&ober, &unter)
}
