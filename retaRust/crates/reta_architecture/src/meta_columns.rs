//! Meta-column morphisms transcompiled from
//! `python_arch_reference/reta_architecture/meta_columns.py`.
//!
//! This module owns the typed shape of meta/concrete/theory/abstract column
//! generation.  It exposes deterministic helpers for integer/fraction checks,
//! preword generation and prime-cross classification while the legacy renderer
//! remains the behavior oracle.

use serde::{Deserialize, Serialize};

use crate::number_theory::{could_be_prime_number_primzahlkreuz, could_be_prime_number_primzahlkreuz_fuer_aussen, could_be_prime_number_primzahlkreuz_fuer_innen, prime_creativity, prime_factors, prime_repeat};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MetaColumnSpec {
    pub method_name: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl MetaColumnSpec {
    pub fn new(method_name: &str, description: &str, tags: &[&str]) -> Self {
        Self {
            method_name: method_name.to_string(),
            description: description.to_string(),
            tags: tags.iter().map(|item| item.to_string()).collect(),
        }
    }

    pub fn snapshot(&self) -> MetaColumnSpecSnapshot {
        MetaColumnSpecSnapshot {
            method_name: self.method_name.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MetaColumnSpecSnapshot {
    pub method_name: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MetaColumnsBundle {
    pub specs: Vec<MetaColumnSpec>,
}

impl MetaColumnsBundle {
    pub fn snapshot(&self) -> MetaColumnsSnapshot {
        MetaColumnsSnapshot {
            class: "MetaColumnsBundle".to_string(),
            count: self.specs.len(),
            morphisms: self.specs.iter().map(MetaColumnSpec::snapshot).collect(),
        }
    }

    pub fn is_integral_meta_value(&self, zahl: Rational, inverse: bool) -> bool {
        spalte_meta_konkret_abstrakt_is_ganzzahlig(zahl, inverse)
    }

    pub fn prime_cross_column(&self, value: i64) -> PrimeCrossColumnClass {
        spalte_fuer_gegen_innen_aussen_seitlich_prim(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MetaColumnsSnapshot {
    pub class: String,
    pub count: usize,
    pub morphisms: Vec<MetaColumnSpecSnapshot>,
}

pub fn bootstrap_meta_columns() -> MetaColumnsBundle {
    MetaColumnsBundle {
        specs: vec![
            MetaColumnSpec::new(
                "spalteMetaKontretTheorieAbstrakt_etc_1",
                "Entry point for generated meta/concrete/theory/abstract columns.",
                &["meta", "theorie", "abstrakt", "konkret"],
            ),
            MetaColumnSpec::new(
                "spalteFuerGegenInnenAussenSeitlichPrim",
                "Classifies prime-cross generated columns as pro/contra/inside/outside/sideways.",
                &["primzahlkreuz", "meta"],
            ),
            MetaColumnSpec::new(
                "readOneCSVAndReturn",
                "CSV section cache used by meta and fractional generated-column morphisms.",
                &["prägarbe", "csv"],
            ),
        ],
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: i64,
}

impl Rational {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        assert!(denominator != 0, "Rational denominator must not be zero");
        let sign = if denominator < 0 { -1 } else { 1 };
        let gcd = gcd_i64(numerator.abs(), denominator.abs()).max(1);
        Self {
            numerator: sign * numerator / gcd,
            denominator: denominator.abs() / gcd,
        }
    }

    pub fn reciprocal(self) -> Self {
        Self::new(self.denominator, self.numerator)
    }

    pub fn is_integer(self) -> bool {
        self.denominator == 1 || self.numerator % self.denominator == 0
    }

    pub fn checked_integer(self) -> Option<i64> {
        if self.is_integer() {
            Some(self.numerator / self.denominator)
        } else {
            None
        }
    }
}

pub fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

pub fn spalte_meta_konkret_abstrakt_is_ganzzahlig(zahl: Rational, inverse: bool) -> bool {
    let value = if inverse { zahl.reciprocal() } else { zahl };
    value.is_integer()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MetaVorwort {
    pub prefix: String,
    pub repetitions: usize,
}

pub fn make_vorwort(prefix: &str, repetitions: usize) -> String {
    if repetitions == 0 {
        String::new()
    } else {
        std::iter::repeat(prefix).take(repetitions).collect::<Vec<_>>().join("")
    }
}

pub fn switching_meta_pair(zahl1: Rational, zahl2: Rational, choose_inverse: bool) -> (Rational, Rational) {
    if choose_inverse {
        (zahl1.reciprocal(), zahl2.reciprocal())
    } else {
        (zahl1, zahl2)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PrimeCrossColumnClass {
    Pro,
    Contra,
    Innen,
    Aussen,
    Seitlich,
    NichtPrimkreuz,
}

pub fn spalte_fuer_gegen_innen_aussen_seitlich_prim(value: i64) -> PrimeCrossColumnClass {
    if !could_be_prime_number_primzahlkreuz(value) {
        PrimeCrossColumnClass::NichtPrimkreuz
    } else if could_be_prime_number_primzahlkreuz_fuer_innen(value) {
        PrimeCrossColumnClass::Innen
    } else if could_be_prime_number_primzahlkreuz_fuer_aussen(value) {
        PrimeCrossColumnClass::Aussen
    } else if value.rem_euclid(2) == 0 {
        PrimeCrossColumnClass::Contra
    } else {
        PrimeCrossColumnClass::Pro
    }
}

pub fn find_all_brueche_and_their_combinations(max_denominator: i64) -> Vec<Rational> {
    let mut out = Vec::new();
    for denominator in 1..=max_denominator.max(1) {
        for numerator in 1..=denominator {
            out.push(Rational::new(numerator, denominator));
        }
    }
    out.sort();
    out.dedup();
    out
}

pub fn meta_number_signature(value: i64) -> String {
    let factors = prime_factors(value);
    let repeats = prime_repeat(&factors)
        .into_iter()
        .map(|(prime, count)| format!("{prime}^{count}"))
        .collect::<Vec<_>>()
        .join("*");
    format!("creativity={} factors={}", prime_creativity(value), repeats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_normalizes_and_checks_integer() {
        assert_eq!(Rational::new(2, 4), Rational::new(1, 2));
        assert!(spalte_meta_konkret_abstrakt_is_ganzzahlig(Rational::new(4, 2), false));
    }

    #[test]
    fn bundle_exposes_three_python_morphisms() {
        let bundle = bootstrap_meta_columns();
        assert_eq!(bundle.snapshot().count, 3);
    }
}
