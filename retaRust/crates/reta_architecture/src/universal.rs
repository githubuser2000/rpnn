use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UniversalBundle {
    pub available: Vec<String>,
    pub universal_property: String,
}

impl UniversalBundle {
    pub fn new() -> Self {
        Self {
            available: vec![
                "merge_parameter_dicts".to_string(),
                "normalize_column_buckets".to_string(),
                "sync_tables".to_string(),
                "deterministic_glue".to_string(),
            ],
            universal_property:
                "local_compatible_sections_have_a_unique_deterministic_global_gluing".to_string(),
        }
    }
}

pub fn merge_parameter_dicts(
    maps: &[BTreeMap<String, BTreeSet<String>>],
) -> BTreeMap<String, BTreeSet<String>> {
    let mut merged = BTreeMap::<String, BTreeSet<String>>::new();
    for map in maps {
        for (key, values) in map {
            merged.entry(key.clone()).or_default().extend(values.iter().cloned());
        }
    }
    merged
}

pub fn normalize_column_buckets(
    spalten_arten: &BTreeMap<(i64, i64), BTreeSet<i64>>,
) -> BTreeMap<(i64, i64), BTreeSet<i64>> {
    let mut buckets = spalten_arten.clone();
    let max_type = (buckets.len() / 2) as i64;

    for bucket_type in 0..max_type {
        let positive_key = (0, bucket_type);
        let negative_key = (1, bucket_type);
        if let Some(negative) = buckets.get(&negative_key).cloned() {
            if let Some(positive) = buckets.get_mut(&positive_key) {
                for value in &negative {
                    positive.remove(value);
                }
            }
        }
    }

    for bucket_type in 0..max_type {
        let positive_key = (0, bucket_type);
        let negative_key = (1, bucket_type);
        if let Some(negative) = buckets.get(&negative_key).cloned() {
            if let Some(positive) = buckets.get_mut(&positive_key) {
                for value in &negative {
                    positive.remove(value);
                }
            }
            buckets.remove(&negative_key);
        }
    }

    buckets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_column_buckets_removes_negative_side() {
        let mut buckets = BTreeMap::new();
        buckets.insert((0, 0), BTreeSet::from([1, 2, 3]));
        buckets.insert((1, 0), BTreeSet::from([2]));
        let normalized = normalize_column_buckets(&buckets);
        assert_eq!(normalized.get(&(0, 0)).cloned().unwrap_or_default(), BTreeSet::from([1, 3]));
        assert!(!normalized.contains_key(&(1, 0)));
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "snapshot",
    "sync_generated_columns_from_tables",
    "sync_output_section_from_tables",
    "sync_tables",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 small-surface concrete wrappers.
pub fn snapshot() -> UniversalBundle {
    UniversalBundle::new()
}

pub fn sync_tables<T: Clone>(tables: &[T]) -> Vec<T> {
    tables.to_vec()
}

pub fn sync_generated_columns_from_tables<T: Clone>(tables: &[T]) -> Vec<T> {
    tables.to_vec()
}

pub fn sync_output_section_from_tables<T: Clone>(tables: &[T]) -> Vec<T> {
    tables.to_vec()
}
