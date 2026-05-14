//! Triangular parity harness for `py reta`, `py reta arch` and Rust.
//!
//! The port intentionally keeps two Python views useful: legacy Python remains
//! the behavioural oracle, while modular Python architecture remains the
//! transcompilation anchor.  This module names that triangle as data so callers
//! can generate probes, snapshots and future CI jobs without hard-coding the
//! same command sets in shell scripts.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::runtime_switch::ArchitectureSwitchConfig;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum ParityOracle {
    PyReta,
    PyRetaArch,
    RustReta,
    RustRetaPrompt,
}

impl ParityOracle {
    pub fn canonical(self) -> &'static str {
        match self {
            Self::PyReta => "py_reta",
            Self::PyRetaArch => "py_reta_arch",
            Self::RustReta => "rust_rreta",
            Self::RustRetaPrompt => "rust_rretaPrompt",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParityCommandCase {
    pub case_id: String,
    pub argv: Vec<String>,
    pub prompt_input: Option<String>,
    pub category: String,
    pub required_oracles: Vec<ParityOracle>,
    pub expected_invariants: Vec<String>,
    pub notes: String,
}

impl ParityCommandCase {
    pub fn shell_line(&self) -> String {
        self.argv.join(" ")
    }

    pub fn oracle_names(&self) -> Vec<String> {
        self.required_oracles
            .iter()
            .map(|oracle| oracle.canonical().to_string())
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParityProbePlan {
    pub case_id: String,
    pub command: String,
    pub run_py_reta: bool,
    pub run_py_reta_arch: bool,
    pub run_rust_reta: bool,
    pub run_rust_prompt: bool,
    pub compare_stdout: bool,
    pub compare_stderr: bool,
    pub compare_exit_code: bool,
    pub architecture_mode: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParityHarnessSnapshot {
    pub class: String,
    pub cases: usize,
    pub categories: BTreeMap<String, usize>,
    pub oracle_triangle: Vec<String>,
    pub invariants: Vec<String>,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParityHarnessBundle {
    pub cases: Vec<ParityCommandCase>,
    pub default_invariants: Vec<String>,
}

impl ParityHarnessBundle {
    pub fn plans_for_switch(&self, config: &ArchitectureSwitchConfig) -> Vec<ParityProbePlan> {
        self.cases
            .iter()
            .map(|case| ParityProbePlan {
                case_id: case.case_id.clone(),
                command: case.shell_line(),
                run_py_reta: case.required_oracles.contains(&ParityOracle::PyReta),
                run_py_reta_arch: case.required_oracles.contains(&ParityOracle::PyRetaArch),
                run_rust_reta: case.required_oracles.contains(&ParityOracle::RustReta),
                run_rust_prompt: case.required_oracles.contains(&ParityOracle::RustRetaPrompt),
                compare_stdout: true,
                compare_stderr: true,
                compare_exit_code: true,
                architecture_mode: config.mode.canonical().to_string(),
            })
            .collect()
    }

    pub fn cases_for_category(&self, category: &str) -> Vec<&ParityCommandCase> {
        self.cases
            .iter()
            .filter(|case| case.category == category)
            .collect()
    }

    pub fn snapshot(&self) -> ParityHarnessSnapshot {
        let mut categories: BTreeMap<String, usize> = BTreeMap::new();
        for case in &self.cases {
            *categories.entry(case.category.clone()).or_insert(0) += 1;
        }
        ParityHarnessSnapshot {
            class: "ParityHarnessBundle".to_string(),
            cases: self.cases.len(),
            categories,
            oracle_triangle: vec![
                ParityOracle::PyReta.canonical().to_string(),
                ParityOracle::PyRetaArch.canonical().to_string(),
                ParityOracle::RustReta.canonical().to_string(),
                ParityOracle::RustRetaPrompt.canonical().to_string(),
            ],
            invariants: self.default_invariants.clone(),
            universal_property: "py_reta_and_py_arch_agree_before_rust_commit".to_string(),
        }
    }
}

pub fn bootstrap_parity_harness() -> ParityHarnessBundle {
    let common = vec![ParityOracle::PyReta, ParityOracle::PyRetaArch, ParityOracle::RustReta];
    let prompt = vec![
        ParityOracle::PyReta,
        ParityOracle::PyRetaArch,
        ParityOracle::RustRetaPrompt,
    ];
    ParityHarnessBundle {
        cases: vec![
            ParityCommandCase {
                case_id: "row-basic-vorhervonausschnitt".to_string(),
                argv: vec![
                    "reta".to_string(),
                    "-zeilen".to_string(),
                    "--vorhervonausschnitt=1-3".to_string(),
                    "-spalten".to_string(),
                    "--breite=0".to_string(),
                ],
                prompt_input: None,
                category: "rows".to_string(),
                required_oracles: common.clone(),
                expected_invariants: vec!["same_stdout".to_string(), "same_exit_code".to_string()],
                notes: "minimal visible table slice".to_string(),
            },
            ParityCommandCase {
                case_id: "kontinuum-m-744-regression".to_string(),
                argv: vec![
                    "reta".to_string(),
                    "-zeilen".to_string(),
                    "--vorhervonausschnitt=1-1".to_string(),
                    "-spalten".to_string(),
                    "--kontinuum=m".to_string(),
                    "--breite=0".to_string(),
                ],
                prompt_input: None,
                category: "tag_schema".to_string(),
                required_oracles: common.clone(),
                expected_invariants: vec![
                    "contains_Neues_M_13_Kontinuum".to_string(),
                    "column_744_has_sternPolygon_and_keinParaOdMetaP".to_string(),
                ],
                notes: "guards the previous py-arch 744 drift".to_string(),
            },
            ParityCommandCase {
                case_id: "output-mode-markdown".to_string(),
                argv: vec![
                    "reta".to_string(),
                    "-zeilen".to_string(),
                    "--vorhervonausschnitt=1-2".to_string(),
                    "-ausgabe".to_string(),
                    "--art=markdown".to_string(),
                ],
                prompt_input: None,
                category: "output".to_string(),
                required_oracles: common.clone(),
                expected_invariants: vec!["same_rendered_rows".to_string()],
                notes: "renderer adapter smoke test".to_string(),
            },
            ParityCommandCase {
                case_id: "prompt-reta-basic".to_string(),
                argv: vec!["rp".to_string(), "reta -zeilen --vorhervonausschnitt=1-2".to_string()],
                prompt_input: Some("reta -zeilen --vorhervonausschnitt=1-2".to_string()),
                category: "prompt".to_string(),
                required_oracles: prompt.clone(),
                expected_invariants: vec!["same_compiled_reta_argv".to_string()],
                notes: "prompt command compiler should route to same reta argv".to_string(),
            },
            ParityCommandCase {
                case_id: "prompt-nested-completion-ausgabe".to_string(),
                argv: vec!["rp".to_string(), "reta -ausgabe --art=h".to_string()],
                prompt_input: Some("reta -ausgabe --art=h".to_string()),
                category: "completion".to_string(),
                required_oracles: prompt,
                expected_invariants: vec!["html_completion_available".to_string()],
                notes: "nested completion context after --art=".to_string(),
            },
        ],
        default_invariants: vec![
            "stdout_byte_parity".to_string(),
            "stderr_byte_parity".to_string(),
            "exit_code_parity".to_string(),
            "py_arch_must_not_diverge_from_py_reta_before_rust_commit".to_string(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_switch::{ArchitectureSwitchConfig, ArchitectureSwitchMode};

    #[test]
    fn parity_harness_contains_744_regression_case() {
        let bundle = bootstrap_parity_harness();
        assert!(bundle
            .cases
            .iter()
            .any(|case| case.case_id == "kontinuum-m-744-regression"));
    }

    #[test]
    fn dry_run_plans_keep_python_oracles() {
        let bundle = bootstrap_parity_harness();
        let config = ArchitectureSwitchConfig::default()
            .with_mode(ArchitectureSwitchMode::DryRun, "test");
        let plans = bundle.plans_for_switch(&config);
        assert!(plans.iter().any(|plan| plan.run_py_reta && plan.run_py_reta_arch));
        assert!(plans.iter().all(|plan| plan.architecture_mode == "dry-run"));
    }
}
