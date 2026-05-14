//! Rust mirror of `architecture_contracts.py`.
//!
//! Contracts are commutative diagrams and refactor laws.  They are metadata,
//! but they are now typed Rust data used by migration, validation and FFI plans.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::architecture_map::ArchitectureMapBundle;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiagramArrowSpec {
    pub source: String,
    pub target: String,
    pub label: String,
    pub code_owner: String,
    pub paradigm_terms: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CommutativeDiagramSpec {
    pub name: String,
    pub diagram_type: String,
    pub nodes: Vec<(String, String)>,
    pub top_path: Vec<DiagramArrowSpec>,
    pub bottom_path: Vec<DiagramArrowSpec>,
    pub equality: String,
    pub capsules: Vec<String>,
    pub categories: Vec<String>,
    pub functors: Vec<String>,
    pub natural_transformations: Vec<String>,
    pub verification: Vec<String>,
    pub stage_origin: String,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapsuleContractSpec {
    pub capsule: String,
    pub boundary: String,
    pub allowed_inputs: Vec<String>,
    pub guaranteed_outputs: Vec<String>,
    pub forbidden_leaks: Vec<String>,
    pub parity_oracles: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RefactorLawSpec {
    pub name: String,
    pub law_type: String,
    pub applies_to: Vec<String>,
    pub mathematical_reading: String,
    pub reta_reading: String,
    pub protected_paths: Vec<String>,
    pub evidence: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContractValidationSpec {
    pub status: String,
    pub unknown_capsules: Vec<String>,
    pub diagrams_without_verification: Vec<String>,
    pub laws_without_evidence: Vec<String>,
    pub checked_diagrams: usize,
    pub checked_laws: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureContractsSnapshot {
    pub class: String,
    pub diagrams: usize,
    pub contracts: usize,
    pub laws: usize,
    pub validation: ContractValidationSpec,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureContractsBundle {
    pub diagrams: Vec<CommutativeDiagramSpec>,
    pub capsule_contracts: Vec<CapsuleContractSpec>,
    pub laws: Vec<RefactorLawSpec>,
    pub validation: ContractValidationSpec,
    pub stage_plan: Vec<String>,
}

impl ArchitectureContractsBundle {
    pub fn diagram_named(&self, name: &str) -> Option<&CommutativeDiagramSpec> {
        self.diagrams.iter().find(|diagram| diagram.name == name)
    }

    pub fn laws_for_capsule(&self, capsule: &str) -> Vec<&RefactorLawSpec> {
        self.laws
            .iter()
            .filter(|law| law.applies_to.iter().any(|item| item == capsule))
            .collect()
    }

    pub fn snapshot(&self) -> ArchitectureContractsSnapshot {
        ArchitectureContractsSnapshot {
            class: "ArchitectureContractsBundle".to_string(),
            diagrams: self.diagrams.len(),
            contracts: self.capsule_contracts.len(),
            laws: self.laws.len(),
            validation: self.validation.clone(),
            universal_property: "commutative_contract_paths_protect_visible_reta_behaviour".to_string(),
        }
    }
}

pub fn bootstrap_architecture_contracts(map: Option<&ArchitectureMapBundle>) -> ArchitectureContractsBundle {
    let diagrams = default_diagrams();
    let capsule_contracts = default_contracts(map);
    let laws = default_laws();
    let validation = validate_contracts(&diagrams, &capsule_contracts, &laws, map);
    ArchitectureContractsBundle {
        diagrams,
        capsule_contracts,
        laws,
        validation,
        stage_plan: vec![
            "name-natural-diagrams".to_string(),
            "bind-capsule-contracts".to_string(),
            "guard-byte-parity".to_string(),
            "activate-only-after-witnesses".to_string(),
        ],
    }
}

fn arrow(source: &str, target: &str, label: &str, owner: &str, terms: &[&str]) -> DiagramArrowSpec {
    DiagramArrowSpec {
        source: source.to_string(),
        target: target.to_string(),
        label: label.to_string(),
        code_owner: owner.to_string(),
        paradigm_terms: terms.iter().map(|v| (*v).to_string()).collect(),
    }
}

fn diagram(name: &str, diagram_type: &str, top: Vec<DiagramArrowSpec>, bottom: Vec<DiagramArrowSpec>, equality: &str, capsules: &[&str], functors: &[&str], transformations: &[&str], verification: &[&str], stage: &str, description: &str) -> CommutativeDiagramSpec {
    let nodes = top
        .iter()
        .chain(bottom.iter())
        .flat_map(|a| [(a.source.clone(), a.source.clone()), (a.target.clone(), a.target.clone())])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    CommutativeDiagramSpec {
        name: name.to_string(),
        diagram_type: diagram_type.to_string(),
        nodes,
        top_path: top,
        bottom_path: bottom,
        equality: equality.to_string(),
        capsules: capsules.iter().map(|v| (*v).to_string()).collect(),
        categories: vec!["OpenRetaContextCategory".to_string(), "ExecutionNetworkCategory".to_string()],
        functors: functors.iter().map(|v| (*v).to_string()).collect(),
        natural_transformations: transformations.iter().map(|v| (*v).to_string()).collect(),
        verification: verification.iter().map(|v| (*v).to_string()).collect(),
        stage_origin: stage.to_string(),
        description: description.to_string(),
    }
}

fn default_diagrams() -> Vec<CommutativeDiagramSpec> {
    vec![
        diagram(
            "cli-parse-naturality",
            "naturality-square",
            vec![arrow("RawArgs", "ContextSelection", "py_arch_parse", "parameter_runtime.py", &["functor", "local_section"])],
            vec![arrow("RawArgs", "ContextSelection", "rust_parse", "parameter_runtime.rs", &["functor", "local_section"])],
            "py_reta_arch_parse == rust_parse before commit",
            &["SchemaTopologyCapsule", "LocalSectionCapsule"],
            &["InputFunctor"],
            &["ParseNaturality"],
            &["reta_architecture_cli_plan_json"],
            "stage-29",
            "CLI parsing must commute between Python architecture and Rust runtime.",
        ),
        diagram(
            "table-render-parity",
            "commutative-render-diagram",
            vec![arrow("PreparedTable", "VisibleOutput", "legacy_render", "reta.py", &["renderer"] )],
            vec![arrow("PreparedTable", "VisibleOutput", "rust_render", "table_output.rs", &["renderer", "functor"] )],
            "legacy_stdout == rust_stdout for committed adapter gates",
            &["TableCoreCapsule", "OutputRenderingCapsule"],
            &["TableFunctor", "OutputFunctor"],
            &["RenderNaturality"],
            &["output-mode-markdown", "row-basic-vorhervonausschnitt"],
            "stage-33",
            "Prepared tables may be rendered by Rust only after byte-visible parity gates pass.",
        ),
        diagram(
            "prompt-argv-gluing",
            "prompt-gluing-square",
            vec![arrow("PromptText", "RetaArgv", "python_prompt_compile", "LibRetaPrompt.py", &["prompt", "glue"] )],
            vec![arrow("PromptText", "RetaArgv", "rust_prompt_compile", "prompt_execution.rs", &["prompt", "glue"] )],
            "same prompt input compiles to same reta argv",
            &["InputPromptCapsule", "WorkflowGluingCapsule"],
            &["PromptFunctor"],
            &["PromptCompileNaturality"],
            &["prompt-reta-basic", "prompt-nested-completion-ausgabe"],
            "stage-41",
            "Prompt preparation/execution must glue to the same command sequence.",
        ),
        diagram(
            "execution-network-reduce",
            "universal-cover-diagram",
            vec![arrow("TaskCover", "OrderedResult", "serial_reduce", "dataflow.rs", &["cover", "universal_property"] )],
            vec![arrow("TaskCover", "OrderedResult", "threaded_reduce", "execution_network.rs", &["fifo", "lifo", "priority", "semaphore"] )],
            "all schedules glue to same ordered visible result",
            &["ExecutionNetworkCapsule"],
            &["ExecutionNetworkFunctor"],
            &["DeterministicGluing"],
            &["serial_vs_threaded_ordered_rows"],
            "stage-42",
            "Queue discipline may change internally; ordered output must not.",
        ),
    ]
}

fn contract(capsule: &str, boundary: &str, inputs: &[&str], outputs: &[&str], leaks: &[&str], oracles: &[&str]) -> CapsuleContractSpec {
    CapsuleContractSpec {
        capsule: capsule.to_string(),
        boundary: boundary.to_string(),
        allowed_inputs: inputs.iter().map(|v| (*v).to_string()).collect(),
        guaranteed_outputs: outputs.iter().map(|v| (*v).to_string()).collect(),
        forbidden_leaks: leaks.iter().map(|v| (*v).to_string()).collect(),
        parity_oracles: oracles.iter().map(|v| (*v).to_string()).collect(),
    }
}

fn default_contracts(map: Option<&ArchitectureMapBundle>) -> Vec<CapsuleContractSpec> {
    let mut contracts = vec![
        contract("SchemaTopologyCapsule", "parameter-and-tag facade", &["RawArgs", "column ids"], &["ContextSelection", "TableTag sections"], &["legacy mutable Program state"], &["py_reta_arch"]),
        contract("TableCoreCapsule", "prepared-table adapter", &["ExecutionPlan", "RowSelection"], &["PreparedTable"], &["direct stdout writes"], &["py_reta", "py_reta_arch", "rust_rreta"]),
        contract("OutputRenderingCapsule", "visible-output boundary", &["PreparedTable", "OutputMode"], &["stdout/stderr lines"], &["internal tag/debug structures"], &["py_reta", "rust_rreta"]),
        contract("InputPromptCapsule", "prompt input/completion boundary", &["PromptText", "cursor"], &["completion candidates", "PromptPlan"], &["table renderer mutation"], &["py_reta_arch", "rust_rretaPrompt"]),
        contract("ExecutionNetworkCapsule", "scheduler boundary", &["ExecutionTask cover"], &["ordered ExecutionResult"], &["non-deterministic visible order"], &["serial", "threaded"]),
        contract("GovernanceCapsule", "activation boundary", &["MigrationCandidate", "ParityProbe"], &["ActivationDecision"], &["commit without rollback"], &["contracts", "witnesses", "validation"]),
    ];
    if let Some(map) = map {
        for capsule in &map.capsules {
            if !contracts.iter().any(|contract| contract.capsule == capsule.name) {
                contracts.push(contract(&capsule.name, "default capsule contract", &["typed input"], &["typed output"], &["cross-capsule mutation"], &["architecture_validation"]));
            }
        }
    }
    contracts
}

fn law(name: &str, law_type: &str, applies_to: &[&str], math: &str, reta: &str, protected: &[&str], evidence: &[&str]) -> RefactorLawSpec {
    RefactorLawSpec { name: name.to_string(), law_type: law_type.to_string(), applies_to: applies_to.iter().map(|v| (*v).to_string()).collect(), mathematical_reading: math.to_string(), reta_reading: reta.to_string(), protected_paths: protected.iter().map(|v| (*v).to_string()).collect(), evidence: evidence.iter().map(|v| (*v).to_string()).collect() }
}

fn default_laws() -> Vec<RefactorLawSpec> {
    vec![
        law("no-visible-change-without-commit", "activation law", &["GovernanceCapsule"], "shadow sections cannot alter global output sheaf", "observe/dry-run flags must not change reta output", &["runtime_switch.rs", "reta_workflow_py.rs"], &["switch-flags-stripped"]),
        law("ordered-gluing", "universal property", &["ExecutionNetworkCapsule"], "all task covers reduce to same ordered colimit", "FIFO/LIFO/priority/threaded scheduling must glue to the same visible row order", &["dataflow.rs", "execution_network.rs"], &["execution-network-reduce"]),
        law("tag-744-regression", "schema law", &["SchemaTopologyCapsule"], "tag lookup is a stable natural transformation", "column 744 remains sternPolygon + keinParaOdMetaP", &["tag_schema.rs"], &["kontinuum-m-744-regression"]),
        law("two-python-oracles", "parity law", &["GovernanceCapsule"], "source and architecture functors commute before Rust commit", "py reta and py reta arch must agree before rreta commit", &["parity_harness.rs"], &["py_reta_vs_py_arch_vs_rust"]),
    ]
}

fn validate_contracts(diagrams: &[CommutativeDiagramSpec], contracts: &[CapsuleContractSpec], laws: &[RefactorLawSpec], map: Option<&ArchitectureMapBundle>) -> ContractValidationSpec {
    let known_capsules = map
        .map(|map| map.capsule_names().into_iter().collect::<BTreeSet<_>>())
        .unwrap_or_default();
    let unknown_capsules = if known_capsules.is_empty() {
        Vec::new()
    } else {
        diagrams
            .iter()
            .flat_map(|diagram| diagram.capsules.iter())
            .chain(contracts.iter().map(|contract| &contract.capsule))
            .filter(|name| !known_capsules.contains(*name))
            .cloned()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    };
    ContractValidationSpec {
        status: if unknown_capsules.is_empty() { "ready" } else { "needs-attention" }.to_string(),
        unknown_capsules,
        diagrams_without_verification: diagrams.iter().filter(|diagram| diagram.verification.is_empty()).map(|diagram| diagram.name.clone()).collect(),
        laws_without_evidence: laws.iter().filter(|law| law.evidence.is_empty()).map(|law| law.name.clone()).collect(),
        checked_diagrams: diagrams.len(),
        checked_laws: laws.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture_map::bootstrap_architecture_map;

    #[test]
    fn contracts_validate_against_map() {
        let map = bootstrap_architecture_map();
        let contracts = bootstrap_architecture_contracts(Some(&map));
        assert_eq!(contracts.validation.status, "ready");
        assert!(contracts.diagram_named("tag-744-regression").is_none());
        assert!(contracts.diagram_named("cli-parse-naturality").is_some());
    }
}
