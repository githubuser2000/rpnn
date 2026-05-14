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



pub fn _ensure_runtime_dependencies() -> bool {
    true
}

pub fn spalte_meta_konkret_abstrakt_is_ganz_zahlig(zahl: Rational, inverse: bool) -> bool {
    spalte_meta_konkret_abstrakt_is_ganzzahlig(zahl, inverse)
}

pub fn spalte_meta_kontret_theorie_abstrakt_etc_1(value: i64) -> String {
    format!("meta-konkret-theorie-abstrakt:{value}")
}

pub fn spalte_meta_kontret_theorie_abstrakt_etc(value: i64) -> String {
    spalte_meta_kontret_theorie_abstrakt_etc_1(value)
}

pub fn spalte_meta_konkret_theorie_abstrakt_set_html_parameters(enabled: bool) -> Vec<(String, String)> {
    vec![("html".to_string(), enabled.to_string())]
}

pub fn spalte_meta_konkret_theorie_abstrakt_vorwort_behandlung_wie_vorwort_meta(prefix: &str, repetitions: usize) -> String {
    make_vorwort(prefix, repetitions)
}

pub fn spalte_meta_konkret_theorie_abstrakt_main_part(value: i64) -> String {
    meta_number_signature(value)
}

pub fn spalte_meta_konkret_theorie_abstrakt_main_part_inserting_text(value: i64, text: &str) -> String {
    if text.trim().is_empty() {
        meta_number_signature(value)
    } else {
        format!("{} {}", meta_number_signature(value), text.trim())
    }
}

pub fn spalte_meta_konkret_theorie_abstrakt_get_gebr_rat_univ_strukturalie(value: i64) -> String {
    match value.rem_euclid(3) {
        0 => "gebrochen-rational".to_string(),
        1 => "universell".to_string(),
        _ => "strukturalie".to_string(),
    }
}

pub fn spalte_meta_konkret_abstrakt_ueberschriften_und_tags() -> Vec<String> {
    vec!["Meta".to_string(), "Konkret".to_string(), "Theorie".to_string(), "Abstrakt".to_string()]
}

pub fn get_all_brueche(max_denominator: i64) -> Vec<Rational> {
    find_all_brueche_and_their_combinations(max_denominator)
}

pub fn read_one_csv_and_return(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|line| line.split(',').map(|cell| cell.trim().to_string()).collect())
        .collect()
}

pub fn switching(zahl1: Rational, zahl2: Rational, choose_inverse: bool) -> (Rational, Rational) {
    switching_meta_pair(zahl1, zahl2, choose_inverse)
}

pub fn prim_answer(value: i64) -> String {
    format!("{:?}", spalte_fuer_gegen_innen_aussen_seitlich_prim(value))
}

pub fn prim_answer2(value: i64) -> String {
    format!("{}:{}", value, prim_answer(value))
}

#[allow(non_snake_case)]
pub fn PrimAnswer(value: i64) -> String {
    prim_answer(value)
}

#[allow(non_snake_case)]
pub fn PrimAnswer2(value: i64) -> String {
    prim_answer2(value)
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
