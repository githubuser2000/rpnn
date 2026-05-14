//! Numeric morphisms transcompiled from
//! `python_arch_reference/reta_architecture/number_theory.py`.
//!
//! These helpers are deliberately dependency-light.  They are used by table
//! rendering, generated columns and prompt checks, and keep the legacy Reta
//! names close to the Python source while also exposing idiomatic Rust aliases.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NumberTheorySnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub legacy_owner: String,
    pub dependency_profile: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct NumberTheoryBundle;

impl NumberTheoryBundle {
    pub fn snapshot(&self) -> NumberTheorySnapshot {
        NumberTheorySnapshot {
            class: "NumberTheoryBundle".to_string(),
            morphisms: vec![
                "moon_number".to_string(),
                "prime_factors".to_string(),
                "divisor_generator".to_string(),
                "prime_repeat".to_string(),
                "prime_creativity".to_string(),
                "prime_multiple".to_string(),
                "is_prime_multiple".to_string(),
                "prime_cross_candidate".to_string(),
                "prime_cross_inner_candidate".to_string(),
                "prime_cross_outer_candidate".to_string(),
            ],
            legacy_owner: "libs.lib4tables".to_string(),
            dependency_profile: "math-only".to_string(),
        }
    }

    pub fn moon_number(&self, num: i64) -> (Vec<i64>, Vec<i64>) {
        moon_number(num)
    }

    pub fn prime_factors(&self, value: i64) -> Vec<i64> {
        prime_factors(value)
    }

    pub fn divisor_generator(&self, value: i64) -> Vec<i64> {
        divisor_generator(value)
    }

    pub fn prime_repeat(&self, values: &[i64]) -> Vec<(i64, i64)> {
        prime_repeat(values)
    }

    pub fn prime_creativity(&self, value: i64) -> i64 {
        prime_creativity(value)
    }

    pub fn prime_multiple(&self, value: i64) -> Vec<(i64, i64)> {
        prime_multiple(value)
    }

    pub fn is_prime_multiple(&self, value: i64, multiples: &[i64]) -> bool {
        is_prime_multiple(value, multiples)
    }
}

pub fn bootstrap_number_theory() -> NumberTheoryBundle {
    NumberTheoryBundle
}

pub fn moon_number(num: i64) -> (Vec<i64>, Vec<i64>) {
    let mut results = Vec::new();
    let mut exponents = Vec::new();
    if num <= 2 {
        return (results, exponents);
    }
    for exponent in 2..num {
        let one_result = (num as f64).powf(1.0 / exponent as f64);
        if (one_result.round() * 100000.0).round() as i64
            == (one_result * 100000.0).round() as i64
        {
            results.push(one_result.round() as i64);
            exponents.push(exponent - 2);
        }
    }
    (results, exponents)
}

pub fn prime_factors(value: i64) -> Vec<i64> {
    let mut factors = Vec::new();
    let mut remaining = value;
    while remaining > 1 {
        let mut candidate = 2i64;
        let mut found = false;
        let mut prime = remaining;
        while candidate * candidate <= value && !found {
            if remaining % candidate == 0 {
                found = true;
                prime = candidate;
            } else {
                candidate += 1;
            }
        }
        if !found {
            prime = remaining;
        }
        factors.push(prime);
        remaining /= prime;
    }
    factors
}

pub fn divisor_generator(value: i64) -> Vec<i64> {
    if value <= 0 {
        return Vec::new();
    }
    let mut small = Vec::new();
    let mut large = Vec::new();
    let root = (value as f64).sqrt() as i64;
    for candidate in 1..=root {
        if value % candidate == 0 {
            small.push(candidate);
            if candidate * candidate != value {
                large.push(value / candidate);
            }
        }
    }
    large.reverse();
    small.extend(large);
    small
}

pub fn prime_repeat(values: &[i64]) -> Vec<(i64, i64)> {
    let mut reversed = values.to_vec();
    reversed.reverse();
    let mut grouped: Vec<(i64, i64)> = Vec::new();
    let mut count = 1i64;
    let mut previous: Option<i64> = None;

    for value in reversed {
        if previous == Some(value) {
            count += 1;
        } else {
            count = 1;
        }
        grouped.push((value, count));
        previous = Some(value);
    }
    grouped.reverse();

    let mut result = Vec::new();
    let mut previous_out: Option<i64> = None;
    for (value, amount) in grouped {
        if previous_out != Some(value) {
            result.push((value, amount));
        }
        previous_out = Some(value);
    }
    result
}

pub fn prime_creativity(num: i64) -> i64 {
    if num == 0 {
        return 0;
    }
    let repeated = prime_repeat(&prime_factors(num));
    if repeated.len() == 1 && repeated[0].1 == 1 {
        return 1;
    }
    if repeated.len() == 1 {
        return 3;
    }
    if repeated.is_empty() {
        return 0;
    }

    let mut intersection: Option<std::collections::BTreeSet<i64>> = None;
    for (_prime, amount) in repeated {
        let divisors = divisor_generator(amount)
            .into_iter()
            .filter(|value| *value != 1)
            .collect::<std::collections::BTreeSet<_>>();
        if divisors.is_empty() {
            intersection = None;
            break;
        }
        intersection = Some(match intersection {
            Some(existing) => existing.intersection(&divisors).copied().collect(),
            None => divisors,
        });
    }

    match intersection {
        Some(values) if !values.is_empty() => 3,
        Some(_) => 2,
        None => 2,
    }
}

pub fn prime_multiple(value: i64) -> Vec<(i64, i64)> {
    let mut multiples = vec![(1, value)];
    for (prime, _amount) in prime_repeat(&prime_factors(value)) {
        if prime != 0 {
            multiples.push((prime, value / prime));
        }
    }
    multiples
}

pub fn is_prime_multiple(value: i64, multiples: &[i64]) -> bool {
    let known = prime_multiple(value);
    multiples
        .iter()
        .any(|wanted| known.iter().any(|(_prime, multiple)| wanted == multiple))
}

pub fn could_be_prime_number_primzahlkreuz(num: i64) -> bool {
    matches!(num.rem_euclid(24), 1 | 5 | 7 | 11 | 13 | 17 | 19 | 23)
}

pub fn could_be_prime_number_primzahlkreuz_fuer_innen(num: i64) -> bool {
    matches!(num.rem_euclid(24), 5 | 11 | 17 | 23)
}

pub fn could_be_prime_number_primzahlkreuz_fuer_aussen(num: i64) -> bool {
    matches!(num.rem_euclid(24), 1 | 7 | 13 | 19)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factors_preserve_repetitions() {
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_repeat(&prime_factors(12)), vec![(2, 2), (3, 1)]);
    }

    #[test]
    fn prime_creativity_matches_legacy_categories() {
        assert_eq!(prime_creativity(0), 0);
        assert_eq!(prime_creativity(7), 1);
        assert_eq!(prime_creativity(12), 2);
        assert_eq!(prime_creativity(8), 3);
    }

    #[test]
    fn primzahlkreuz_candidates_match_modulo_classes() {
        assert!(could_be_prime_number_primzahlkreuz(23));
        assert!(could_be_prime_number_primzahlkreuz_fuer_innen(11));
        assert!(could_be_prime_number_primzahlkreuz_fuer_aussen(13));
        assert!(!could_be_prime_number_primzahlkreuz(9));
    }
}



// Stage 15: concrete legacy-name wrappers for the number-theory surface.
// They keep the historical Python spelling callable from Rust while routing to
// the typed snake_case morphisms above.
pub fn primFak(value: i64) -> Vec<i64> {
    prime_factors(value)
}

pub fn primRepeat(values: &[i64]) -> Vec<(i64, i64)> {
    prime_repeat(values)
}

pub fn primCreativity(value: i64) -> i64 {
    prime_creativity(value)
}

pub fn primMultiple(value: i64) -> Vec<(i64, i64)> {
    prime_multiple(value)
}

pub fn isPrimMultiple(value: i64, multiples: &[i64]) -> bool {
    is_prime_multiple(value, multiples)
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "isPrimMultiple",
    "primCreativity",
    "primFak",
    "primMultiple",
    "primRepeat",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
