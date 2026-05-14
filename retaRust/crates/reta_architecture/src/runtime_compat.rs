//! Architecture-local compatibility runtime helpers transcompiled from
//! `python_arch_reference/reta_architecture/runtime_compat.py`.
//!
//! This gathers names that migrated architecture modules used to import from
//! legacy `libs.center`: row-ranges, arithmetic, console IO and small constants.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::arithmetic::{bootstrap_arithmetic_morphisms, ArithmeticMorphismBundle};
use crate::console_io::{bootstrap_console_io_morphisms, ConsoleIOMorphismBundle};
use crate::row_ranges::{bootstrap_row_range_morphisms, RowRangeMorphismBundle};
use crate::split_i18n::{build_split_i18n_proxy, SplitI18nProxy};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(i32)]
pub enum NPmEnum {
    GalN = 2,
    Gal1pN = 3,
    UniN = 4,
    Uni1pN = 5,
    EmoN = 6,
    Emo1pN = 7,
    GroeN = 8,
    Groe1pN = 9,
}

impl NPmEnum {
    pub fn gal() -> (Self, Self) {
        (Self::GalN, Self::Gal1pN)
    }
    pub fn uni() -> (Self, Self) {
        (Self::UniN, Self::Uni1pN)
    }
    pub fn emo() -> (Self, Self) {
        (Self::EmoN, Self::Emo1pN)
    }
    pub fn groe() -> (Self, Self) {
        (Self::GroeN, Self::Groe1pN)
    }
    pub fn n() -> [Self; 4] {
        [Self::GalN, Self::UniN, Self::EmoN, Self::GroeN]
    }
    pub fn eins_pn() -> [Self; 4] {
        [Self::Gal1pN, Self::Uni1pN, Self::Emo1pN, Self::Groe1pN]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeCompatSnapshot {
    pub class: String,
    pub info_log: bool,
    pub output: bool,
    pub kpattern: String,
    pub source_modules: Vec<String>,
    pub morphisms: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeCompatBundle {
    pub repo_root: String,
    pub i18n: SplitI18nProxy,
    pub row_range_morphisms: RowRangeMorphismBundle,
    pub arithmetic_morphisms: ArithmeticMorphismBundle,
    pub console_io_morphisms: ConsoleIOMorphismBundle,
    pub info_log: bool,
    pub output: bool,
    pub kpattern: String,
    pub multiplikationen: Vec<String>,
    pub primzahlkreuz_pro_contra_strs: Vec<String>,
}

impl RuntimeCompatBundle {
    pub fn bereich_to_numbers2(
        &self,
        text: &str,
        vielfache: bool,
        max_zahl: i64,
        allow_less_eq_zero: bool,
    ) -> BTreeSet<i64> {
        self.row_range_morphisms
            .range_to_numbers(text, vielfache, max_zahl, allow_less_eq_zero)
    }

    pub fn is_zeilen_angabe(&self, text: &str) -> bool {
        self.row_range_morphisms.is_row_range(text)
    }

    pub fn chunks<T: Clone>(&self, values: &[T], size: usize) -> Vec<Vec<T>> {
        self.console_io_morphisms.chunks(values, size)
    }

    pub fn cliout(&self, text: &str, color: bool, stype: &str) -> Option<String> {
        self.console_io_morphisms
            .cliout(text, color, stype, self.output)
    }

    pub fn multiples(&self, value: i64, include_one: bool) -> Vec<(i64, i64)> {
        self.arithmetic_morphisms.multiples(value, include_one)
    }

    pub fn teiler(&self, text: &str) -> (Vec<String>, BTreeSet<i64>) {
        self.arithmetic_morphisms.divisors_for_range(text)
    }

    pub fn text_hat_ziffer(&self, text: &str) -> bool {
        self.arithmetic_morphisms.has_digit(text)
    }

    pub fn snapshot(&self) -> RuntimeCompatSnapshot {
        RuntimeCompatSnapshot {
            class: "RuntimeCompatBundle".to_string(),
            info_log: self.info_log,
            output: self.output,
            kpattern: self.kpattern.clone(),
            source_modules: self.i18n.source_modules.clone(),
            morphisms: vec![
                "BereichToNumbers2".to_string(),
                "isZeilenAngabe".to_string(),
                "retaPromptHilfe".to_string(),
                "retaHilfe".to_string(),
                "getTextWrapThings".to_string(),
                "chunks".to_string(),
                "cliout".to_string(),
                "multiples".to_string(),
                "teiler".to_string(),
                "invert_dict_B".to_string(),
                "textHatZiffer".to_string(),
                "primfaktoren".to_string(),
                "primRepeat".to_string(),
                "primRepeat2".to_string(),
                "moduloA".to_string(),
            ],
        }
    }
}

pub fn bootstrap_runtime_compat(repo_root: Option<String>, argv: &[String]) -> RuntimeCompatBundle {
    let i18n = build_split_i18n_proxy(None);
    let row_range_morphisms = bootstrap_row_range_morphisms(None);
    let arithmetic_morphisms =
        bootstrap_arithmetic_morphisms(Some(row_range_morphisms.clone()), None);
    let console_io_morphisms = bootstrap_console_io_morphisms(repo_root.clone());
    let info_log = argv.iter().any(|arg| arg == "-debug" || arg == "-d");
    RuntimeCompatBundle {
        repo_root: repo_root.unwrap_or_else(|| ".".to_string()),
        i18n,
        kpattern: row_range_morphisms.syntax.comma_split_pattern.clone(),
        row_range_morphisms,
        arithmetic_morphisms,
        console_io_morphisms,
        info_log,
        output: true,
        multiplikationen: Vec::new(),
        primzahlkreuz_pro_contra_strs: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_compat_exposes_range_wrapper() {
        let compat = bootstrap_runtime_compat(None, &[]);
        assert!(compat.is_zeilen_angabe("1-3"));
        assert_eq!(compat.bereich_to_numbers2("1-3", false, 10, false).len(), 3);
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "nPmEnum",
    "alxp",
    "unique_everseen",
    "x",
    "getTextWrapThings",
    "invert_dict_B",
    "moduloA",
    "primRepeat",
    "primRepeat2",
    "primfaktoren",
    "retaHilfe",
    "retaPromptHilfe",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
