//! Column-bucket selection layer transcompiled from
//! `python_arch_reference/reta_architecture/column_selection.py`.
//!
//! Reta's old `Program` uses a two-dimensional bucket space
//! `(positive|negative, bucket-type)`.  This module owns that schema in Rust so
//! the CLI parser, table generation and prompt completions can share the same
//! typed coordinates instead of re-creating Python namedtuple fields.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub const COLUMN_BUCKET_NAMES: [&str; 24] = [
    "ordinary",
    "generated1",
    "concat1",
    "kombi1",
    "boolAndTupleSet1",
    "gebroUni1",
    "gebrGal1",
    "generated2",
    "kombi2",
    "gebrEmo1",
    "gebrGroe1",
    "metakonkret",
    "ordinaryNot",
    "generate1dNot",
    "concat1Not",
    "kombi1Not",
    "boolAndTupleSet1Not",
    "gebroUni1Not",
    "gebrGal1Not",
    "generated2Not",
    "kombi2Not",
    "gebrEmo1Not",
    "gebrGroe1Not",
    "metakonkretNot",
];

pub const COLUMN_BUCKET_VALUES: [(u8, u8); 24] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (0, 5),
    (0, 6),
    (0, 7),
    (0, 8),
    (0, 9),
    (0, 10),
    (0, 11),
    (1, 0),
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (1, 6),
    (1, 7),
    (1, 8),
    (1, 9),
    (1, 10),
    (1, 11),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ColumnBucketKey {
    pub negation: u8,
    pub bucket: u8,
}

impl ColumnBucketKey {
    pub const fn new(negation: u8, bucket: u8) -> Self {
        Self { negation, bucket }
    }

    pub const fn positive(bucket: u8) -> Self {
        Self::new(0, bucket)
    }

    pub const fn negative(bucket: u8) -> Self {
        Self::new(1, bucket)
    }

    pub fn as_tuple(self) -> (u8, u8) {
        (self.negation, self.bucket)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColumnBucketSnapshot {
    pub name: String,
    pub negation: u8,
    pub bucket: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColumnSelectionSnapshot {
    pub class: String,
    pub bucket_names: Vec<String>,
    pub bucket_values: Vec<[u8; 2]>,
    pub positive_bucket_count: usize,
    pub negative_bucket_count: usize,
    pub universal_property: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColumnSelectionBundle;

impl ColumnSelectionBundle {
    pub fn bucket_names(&self) -> Vec<String> {
        COLUMN_BUCKET_NAMES.iter().map(|value| (*value).to_string()).collect()
    }

    pub fn bucket_values(&self) -> Vec<ColumnBucketKey> {
        COLUMN_BUCKET_VALUES
            .iter()
            .map(|(negation, bucket)| ColumnBucketKey::new(*negation, *bucket))
            .collect()
    }

    pub fn type_naming(&self) -> BTreeMap<String, ColumnBucketKey> {
        COLUMN_BUCKET_NAMES
            .iter()
            .zip(COLUMN_BUCKET_VALUES.iter())
            .map(|(name, (negation, bucket))| {
                ((*name).to_string(), ColumnBucketKey::new(*negation, *bucket))
            })
            .collect()
    }

    pub fn new_bucket_map(&self) -> BTreeMap<ColumnBucketKey, BTreeSet<i64>> {
        self.bucket_values()
            .into_iter()
            .map(|key| (key, BTreeSet::new()))
            .collect()
    }

    pub fn normalize_bucket_map(
        &self,
        buckets: &BTreeMap<ColumnBucketKey, BTreeSet<i64>>,
    ) -> BTreeMap<ColumnBucketKey, BTreeSet<i64>> {
        let mut out = buckets.clone();
        for bucket in 0..12u8 {
            let positive = ColumnBucketKey::positive(bucket);
            let negative = ColumnBucketKey::negative(bucket);
            if let Some(negative_values) = out.get(&negative).cloned() {
                if let Some(positive_values) = out.get_mut(&positive) {
                    for value in &negative_values {
                        positive_values.remove(value);
                    }
                }
                out.remove(&negative);
            }
        }
        out
    }

    pub fn snapshot(&self) -> ColumnSelectionSnapshot {
        ColumnSelectionSnapshot {
            class: "ColumnSelectionBundle".to_string(),
            bucket_names: self.bucket_names(),
            bucket_values: COLUMN_BUCKET_VALUES
                .iter()
                .map(|(negation, bucket)| [*negation, *bucket])
                .collect(),
            positive_bucket_count: 12,
            negative_bucket_count: 12,
            universal_property:
                "positive column buckets glue after subtracting matching negative local sections"
                    .to_string(),
        }
    }
}

pub fn bootstrap_column_selection() -> ColumnSelectionBundle {
    ColumnSelectionBundle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_match_python_namedtuple_order() {
        let bundle = bootstrap_column_selection();
        let naming = bundle.type_naming();
        assert_eq!(naming["ordinary"], ColumnBucketKey::positive(0));
        assert_eq!(naming["kombi2"], ColumnBucketKey::positive(8));
        assert_eq!(naming["metakonkretNot"], ColumnBucketKey::negative(11));
        assert_eq!(bundle.snapshot().bucket_names.len(), 24);
    }

    #[test]
    fn negative_bucket_subtracts_and_disappears() {
        let bundle = bootstrap_column_selection();
        let mut buckets = bundle.new_bucket_map();
        buckets.insert(ColumnBucketKey::positive(0), BTreeSet::from([1, 2, 3]));
        buckets.insert(ColumnBucketKey::negative(0), BTreeSet::from([2]));
        let normalized = bundle.normalize_bucket_map(&buckets);
        assert_eq!(normalized[&ColumnBucketKey::positive(0)], BTreeSet::from([1, 3]));
        assert!(!normalized.contains_key(&ColumnBucketKey::negative(0)));
    }
}
