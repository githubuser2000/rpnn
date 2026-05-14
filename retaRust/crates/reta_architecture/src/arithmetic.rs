//! Center-level arithmetic morphisms transcompiled from
//! `python_arch_reference/reta_architecture/arithmetic.py`.
//!
//! This module owns the small deterministic arithmetic helpers that the legacy
//! `libs.center` facade used to expose: factor pairs, divisor expansion over
//! row-range syntax, prime factor formatting, dictionary inversion and digit
//! detection.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};

pub type ClassifierName = String;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArithmeticSnapshot {
    pub class: String,
    pub stage: u32,
    pub legacy_owner: String,
    pub capsule: String,
    pub category: String,
    pub functor: String,
    pub natural_transformation: String,
    pub morphisms: Vec<String>,
    pub compatibility_names: Vec<String>,
    pub row_range_stage: u32,
    pub has_classifier: bool,
    pub observable_invariant: String,
}

pub fn factor_pairs(value: i64, include_one: bool) -> Vec<(i64, i64)> {
    let value = value.abs();
    if value == 0 {
        return if include_one {
            vec![(0, 1)]
        } else {
            Vec::new()
        };
    }
    let mut out = Vec::new();
    let upper = (value as f64).sqrt().floor() as i64 + 1;
    for divisor in 2..upper {
        if divisor != 0 && value % divisor == 0 {
            out.push((value / divisor, divisor));
        }
    }
    if include_one {
        out.push((value, 1));
    }
    out
}

pub fn prime_factors_legacy(value: i64, modulo: bool) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut remaining = value.abs();
    if remaining <= 1 {
        return factors;
    }
    while remaining > 1 {
        let mut candidate = 2i64;
        let mut found = false;
        let mut prime = remaining;
        while candidate * candidate <= value.abs().max(remaining) && !found {
            if remaining % candidate == 0 {
                found = true;
                prime = candidate;
            } else {
                candidate += 1;
            }
        }
        let factor = if found { prime } else { remaining };
        factors.push(if modulo { factor % 24 } else { factor });
        remaining /= factor.max(1);
    }
    factors
}

fn grouped_reversed_counts(values: &mut Vec<i64>) -> Vec<(i64, usize)> {
    values.reverse();
    let mut grouped = Vec::new();
    let mut count = 1usize;
    let mut previous: Option<i64> = None;
    for value in values.iter().copied() {
        if previous == Some(value) {
            count += 1;
        } else {
            count = 1;
        }
        grouped.push((value, count));
        previous = Some(value);
    }
    grouped.reverse();
    grouped
}

pub fn prime_repeat_legacy(values: &mut Vec<i64>) -> Vec<String> {
    let grouped = grouped_reversed_counts(values);
    let mut previous: Option<i64> = None;
    let mut result = Vec::new();
    for (value, amount) in grouped {
        if previous != Some(value) {
            if amount == 1 {
                result.push(value.to_string());
            } else {
                result.push(format!("{value}^{amount}"));
            }
        }
        previous = Some(value);
    }
    result
}

pub fn prime_repeat_pairs(values: &mut Vec<i64>) -> Vec<(i64, usize)> {
    let grouped = grouped_reversed_counts(values);
    let mut previous: Option<i64> = None;
    let mut result = Vec::new();
    for (value, amount) in grouped {
        if previous != Some(value) {
            result.push((value, amount));
        }
        previous = Some(value);
    }
    result
}

pub fn invert_int_value_dict(source: &[(String, Vec<String>)]) -> BTreeMap<i64, Vec<String>> {
    let mut inverted: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    for (key, values) in source {
        for value in values {
            if let Ok(int_value) = value.parse::<i64>() {
                let entry = inverted.entry(int_value).or_default();
                if !entry.contains(key) {
                    entry.push(key.clone());
                }
            }
        }
    }
    inverted
}

pub fn has_digit(text: impl AsRef<str>) -> bool {
    text.as_ref().chars().any(|ch| ch.is_ascii_digit())
}

pub fn divisor_range(
    range_expression: &str,
    row_ranges: Option<&RowRangeMorphismBundle>,
) -> (Vec<String>, BTreeSet<i64>) {
    let owned;
    let row_ranges = match row_ranges {
        Some(value) => value,
        None => {
            owned = bootstrap_row_range_morphisms(None);
            &owned
        }
    };
    let numbers = row_ranges.range_to_numbers(range_expression, false, 0, false);
    let mut divisor_values = BTreeSet::new();
    for number in numbers {
        for (left, right) in factor_pairs(number, true) {
            divisor_values.insert(left);
            divisor_values.insert(right);
        }
    }
    if divisor_values != BTreeSet::from([1]) {
        divisor_values.remove(&1);
    }
    let string_values = divisor_values.iter().map(ToString::to_string).collect();
    (string_values, divisor_values)
}

pub fn modulo_table_lines(values: &[i64], classifier_name: Option<&str>) -> Vec<String> {
    let classify = |value: i64| -> String {
        match classifier_name {
            Some(name) if !name.is_empty() => format!("{name}({value})"),
            _ => String::new(),
        }
    };
    let mut lines = Vec::new();
    for raw in values {
        for divisor in 2..26 {
            let modulo = raw.rem_euclid(divisor);
            let complement = divisor - modulo;
            lines.push(format!(
                "{raw} % {divisor} = {modulo} {}, {complement} {}",
                classify(modulo),
                classify(complement)
            ));
        }
    }
    lines
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArithmeticMorphismBundle {
    pub row_ranges: RowRangeMorphismBundle,
    pub classifier_name: Option<ClassifierName>,
    pub legacy_owner: String,
    pub activated_stage: u32,
}

impl ArithmeticMorphismBundle {
    pub fn multiples(&self, value: i64, include_one: bool) -> Vec<(i64, i64)> {
        factor_pairs(value, include_one)
    }

    pub fn divisors_for_range(&self, range_expression: &str) -> (Vec<String>, BTreeSet<i64>) {
        divisor_range(range_expression, Some(&self.row_ranges))
    }

    pub fn prime_factors(&self, value: i64, modulo: bool) -> Vec<i64> {
        prime_factors_legacy(value, modulo)
    }

    pub fn prime_repeat(&self, values: &mut Vec<i64>) -> Vec<String> {
        prime_repeat_legacy(values)
    }

    pub fn prime_repeat_pairs(&self, values: &mut Vec<i64>) -> Vec<(i64, usize)> {
        prime_repeat_pairs(values)
    }

    pub fn invert_dict(&self, source: &[(String, Vec<String>)]) -> BTreeMap<i64, Vec<String>> {
        invert_int_value_dict(source)
    }

    pub fn has_digit(&self, text: impl AsRef<str>) -> bool {
        has_digit(text)
    }

    pub fn modulo_lines(&self, values: &[i64]) -> Vec<String> {
        modulo_table_lines(values, self.classifier_name.as_deref())
    }

    pub fn snapshot(&self) -> ArithmeticSnapshot {
        ArithmeticSnapshot {
            class: "ArithmeticMorphismBundle".to_string(),
            stage: self.activated_stage,
            legacy_owner: self.legacy_owner.clone(),
            capsule: "InputPromptCapsule".to_string(),
            category: "ActivatedArithmeticCategory".to_string(),
            functor: "ArithmeticActivationFunctor".to_string(),
            natural_transformation: "CenterArithmeticToArchitectureTransformation".to_string(),
            row_range_stage: self.row_ranges.activated_stage,
            has_classifier: self.classifier_name.is_some(),
            morphisms: vec![
                "factor_pairs".to_string(),
                "divisor_range".to_string(),
                "prime_factors".to_string(),
                "prime_repeat_legacy".to_string(),
                "prime_repeat_pairs".to_string(),
                "invert_int_value_dict".to_string(),
                "has_digit".to_string(),
                "modulo_table_lines".to_string(),
            ],
            compatibility_names: vec![
                "multiples".to_string(),
                "teiler".to_string(),
                "primfaktoren".to_string(),
                "primRepeat".to_string(),
                "primRepeat2".to_string(),
                "invert_dict_B".to_string(),
                "textHatZiffer".to_string(),
                "moduloA".to_string(),
            ],
            observable_invariant: "center arithmetic wrappers and ArithmeticMorphismBundle return identical factor, divisor, prime-repeat and digit results".to_string(),
        }
    }
}

pub fn bootstrap_arithmetic_morphisms(
    row_ranges: Option<RowRangeMorphismBundle>,
    classifier_name: Option<String>,
) -> ArithmeticMorphismBundle {
    ArithmeticMorphismBundle {
        row_ranges: row_ranges.unwrap_or_else(|| bootstrap_row_range_morphisms(None)),
        classifier_name,
        legacy_owner: "libs.center".to_string(),
        activated_stage: 38,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_pairs_follow_legacy_shape() {
        assert_eq!(factor_pairs(12, true), vec![(6, 2), (4, 3), (12, 1)]);
        assert_eq!(factor_pairs(13, false), Vec::<(i64, i64)>::new());
    }

    #[test]
    fn divisor_range_uses_row_ranges() {
        let (_strings, values) = divisor_range("6", None);
        assert!(values.contains(&2));
        assert!(values.contains(&3));
        assert!(values.contains(&6));
        assert!(!values.contains(&1));
    }

    #[test]
    fn prime_repeat_keeps_reverse_side_effect() {
        let mut values = vec![2, 2, 3];
        let repeated = prime_repeat_legacy(&mut values);
        assert_eq!(values, vec![3, 2, 2]);
        assert_eq!(repeated, vec!["2^2".to_string(), "3".to_string()]);
    }
}



// Stage 15: concrete legacy-name wrapper for the modulo table printer.
pub fn print_modulo_table(values: &[i64], classifier_name: Option<&str>) -> String {
    modulo_table_lines(values, classifier_name).join("\n")
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "print_modulo_table",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
