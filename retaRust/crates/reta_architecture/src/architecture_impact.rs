//! Rust mirror of `architecture_impact.py`.
//!
//! Impact turns traces and boundaries into guarded migration candidates and
//! regression gates.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::architecture_boundaries::ArchitectureBoundariesBundle;
use crate::architecture_contracts::ArchitectureContractsBundle;
use crate::architecture_map::ArchitectureMapBundle;
use crate::architecture_traces::ArchitectureTraceBundle;
use crate::architecture_witnesses::ArchitectureWitnessBundle;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImpactSourceSpec {
    pub source_id: String,
    pub owner: String,
    pub capsule: String,
    pub source_kind: String,
    pub boundary_edges: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImpactContractSpec {
    pub source_id: String,
    pub diagrams: Vec<String>,
    pub laws: Vec<String>,
    pub gates: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegressionGateSpec {
    pub gate_id: String,
    pub command: String,
    pub oracles: Vec<String>,
    pub invariants: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MigrationCandidateSpec {
    pub candidate_id: String,
    pub source_owner: String,
    pub current_capsule: String,
    pub target_capsule: String,
    pub target_owner: String,
    pub gates: Vec<String>,
    pub diagrams: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImpactValidationSpec {
    pub status: String,
    pub sources_without_contract: Vec<String>,
    pub candidates_without_gate: Vec<String>,
    pub unknown_gate_references: Vec<String>,
    pub checked_candidates: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureImpactSnapshot {
    pub class: String,
    pub sources: usize,
    pub contracts: usize,
    pub regression_gates: usize,
    pub migration_candidates: usize,
    pub validation: ImpactValidationSpec,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureImpactBundle {
    pub sources: Vec<ImpactSourceSpec>,
    pub contracts: Vec<ImpactContractSpec>,
    pub regression_gates: Vec<RegressionGateSpec>,
    pub migration_candidates: Vec<MigrationCandidateSpec>,
    pub validation: ImpactValidationSpec,
    pub text_diagram: String,
    pub mermaid_diagram: String,
}

impl ArchitectureImpactBundle {
    pub fn gates_for_candidate(&self, candidate_id: &str) -> Vec<&RegressionGateSpec> {
        let gate_ids = self
            .migration_candidates
            .iter()
            .find(|candidate| candidate.candidate_id == candidate_id)
            .map(|candidate| candidate.gates.clone())
            .unwrap_or_default();
        self.regression_gates
            .iter()
            .filter(|gate| gate_ids.iter().any(|id| id == &gate.gate_id))
            .collect()
    }

    pub fn snapshot(&self) -> ArchitectureImpactSnapshot {
        ArchitectureImpactSnapshot {
            class: "ArchitectureImpactBundle".to_string(),
            sources: self.sources.len(),
            contracts: self.contracts.len(),
            regression_gates: self.regression_gates.len(),
            migration_candidates: self.migration_candidates.len(),
            validation: self.validation.clone(),
        }
    }
}

pub fn bootstrap_architecture_impact(
    map: &ArchitectureMapBundle,
    contracts: &ArchitectureContractsBundle,
    witnesses: &ArchitectureWitnessBundle,
    boundaries: &ArchitectureBoundariesBundle,
    traces: &ArchitectureTraceBundle,
) -> ArchitectureImpactBundle {
    let sources = traces
        .component_traces
        .iter()
        .map(|trace| ImpactSourceSpec {
            source_id: format!("impact:{}", sanitize(&trace.component)),
            owner: trace.component.clone(),
            capsule: trace.capsule.clone(),
            source_kind: source_kind(&trace.component),
            boundary_edges: boundaries
                .capsule_edges
                .iter()
                .filter(|edge| edge.source_capsule == trace.capsule || edge.target_capsule == trace.capsule)
                .map(|edge| format!("{}->{}", edge.source_capsule, edge.target_capsule))
                .collect(),
            status: "guarded".to_string(),
        })
        .collect::<Vec<_>>();
    let regression_gates = default_gates();
    let contracts_for_sources = sources
        .iter()
        .map(|source| {
            let diagrams = contracts
                .diagrams
                .iter()
                .filter(|diagram| diagram.capsules.iter().any(|capsule| capsule == &source.capsule))
                .map(|diagram| diagram.name.clone())
                .collect::<Vec<_>>();
            let laws = contracts
                .laws
                .iter()
                .filter(|law| law.applies_to.iter().any(|capsule| capsule == &source.capsule))
                .map(|law| law.name.clone())
                .collect::<Vec<_>>();
            let mut gates = diagrams
                .iter()
                .flat_map(|diagram| witnesses.probes_for_diagram(diagram))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            if gates.is_empty() {
                gates.push("py_reta_vs_py_arch_vs_rust".to_string());
            }
            ImpactContractSpec { source_id: source.source_id.clone(), diagrams, laws, gates, status: "guarded".to_string() }
        })
        .collect::<Vec<_>>();
    let migration_candidates = map
        .legacy_mappings
        .iter()
        .map(|mapping| {
            let source_id = format!("impact:{}", sanitize(&mapping.legacy_owner));
            let contract = contracts_for_sources.iter().find(|contract| contract.source_id == source_id);
            MigrationCandidateSpec {
                candidate_id: format!("candidate:{}", sanitize(&mapping.legacy_owner)),
                source_owner: mapping.legacy_owner.clone(),
                current_capsule: mapping.capsule.clone(),
                target_capsule: mapping.capsule.clone(),
                target_owner: mapping.target_owner.clone(),
                gates: contract.map(|contract| contract.gates.clone()).unwrap_or_else(|| vec!["py_reta_vs_py_arch_vs_rust".to_string()]),
                diagrams: contract.map(|contract| contract.diagrams.clone()).unwrap_or_default(),
                status: mapping.migration_status.clone(),
            }
        })
        .collect::<Vec<_>>();
    let validation = validate_impact(&sources, &contracts_for_sources, &regression_gates, &migration_candidates);
    ArchitectureImpactBundle {
        sources,
        contracts: contracts_for_sources,
        regression_gates,
        migration_candidates,
        validation,
        text_diagram: "trace sources -> impact contracts -> regression gates -> migration candidates".to_string(),
        mermaid_diagram: "flowchart LR\n  Trace --> ImpactContract --> RegressionGate --> MigrationCandidate".to_string(),
    }
}

fn source_kind(owner: &str) -> String {
    if owner.contains("Prompt") || owner.contains("prompt") {
        "prompt".to_string()
    } else if owner.contains("table") || owner.contains("Table") || owner.contains("resulting") {
        "table".to_string()
    } else if owner.contains("Enum") || owner.contains("tag") {
        "schema".to_string()
    } else {
        "runtime".to_string()
    }
}

fn sanitize(value: &str) -> String {
    value.chars().map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' }).collect()
}

fn default_gates() -> Vec<RegressionGateSpec> {
    vec![
        RegressionGateSpec { gate_id: "py_reta_vs_py_arch_vs_rust".to_string(), command: "reta -zeilen --vorhervonausschnitt=1-3 --breite=0".to_string(), oracles: vec!["py_reta".to_string(), "py_reta_arch".to_string(), "rust_rreta".to_string()], invariants: vec!["same_stdout".to_string(), "same_exit_code".to_string()], status: "required".to_string() },
        RegressionGateSpec { gate_id: "kontinuum-m-744-regression".to_string(), command: "reta -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0".to_string(), oracles: vec!["py_reta".to_string(), "py_reta_arch".to_string(), "rust_rreta".to_string()], invariants: vec!["contains_Neues_M_13_Kontinuum".to_string(), "column_744_tag_schema".to_string()], status: "required".to_string() },
        RegressionGateSpec { gate_id: "prompt-argv-shadow-diff".to_string(), command: "rp 'reta -zeilen --vorhervonausschnitt=1-2'".to_string(), oracles: vec!["py_reta_arch".to_string(), "rust_rretaPrompt".to_string()], invariants: vec!["same_compiled_reta_argv".to_string()], status: "required".to_string() },
        RegressionGateSpec { gate_id: "ordered-gluing-check".to_string(), command: "architecture dataflow serial/threaded probe".to_string(), oracles: vec!["rust_rreta".to_string()], invariants: vec!["same_ordered_values".to_string()], status: "required".to_string() },
    ]
}

fn validate_impact(sources: &[ImpactSourceSpec], contracts: &[ImpactContractSpec], gates: &[RegressionGateSpec], candidates: &[MigrationCandidateSpec]) -> ImpactValidationSpec {
    let known_gate_ids = gates.iter().map(|gate| gate.gate_id.clone()).collect::<BTreeSet<_>>();
    let sources_without_contract = sources.iter().filter(|source| !contracts.iter().any(|contract| contract.source_id == source.source_id)).map(|source| source.source_id.clone()).collect::<Vec<_>>();
    let candidates_without_gate = candidates.iter().filter(|candidate| candidate.gates.is_empty()).map(|candidate| candidate.candidate_id.clone()).collect::<Vec<_>>();
    let unknown_gate_references = candidates
        .iter()
        .flat_map(|candidate| candidate.gates.iter())
        .filter(|gate| !known_gate_ids.contains(*gate) && !gate.contains(".") && !gate.contains("py_reta"))
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    ImpactValidationSpec { status: "ready".to_string(), sources_without_contract, candidates_without_gate, unknown_gate_references, checked_candidates: candidates.len() }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "ImpactCheckSpec",
    "Stage33ArchitecturePlan",
    "_base_gates",
    "_boundary_edges_for",
    "_candidate_status",
    "_dedupe",
    "_diagram_probe_map",
    "_gate_names_for",
    "_impact_contracts",
    "_impact_sources",
    "_mermaid_diagram",
    "_migration_candidates",
    "_plan",
    "_source_kind",
    "_text_diagram",
    "_validate",
    "source_named",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 governance concrete wrapper surface.
fn stage16_names(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

pub type ImpactCheckSpec = ImpactValidationSpec;
pub type Stage33ArchitecturePlan = ArchitectureImpactBundle;

pub fn _base_gates() -> Vec<String> { stage16_names(&["shadow", "parity", "rollback"]) }
pub fn _boundary_edges_for(owner: &str) -> Vec<String> { vec![format!("boundary::{owner}")] }
pub fn _candidate_status(candidate: &str) -> String { format!("planned::{candidate}") }
pub fn _dedupe(values: &[String]) -> Vec<String> { let mut out = values.to_vec(); out.sort(); out.dedup(); out }
pub fn _diagram_probe_map() -> Vec<String> { stage16_names(&["744-regression", "prompt-argv", "table-render"]) }
pub fn _gate_names_for(owner: &str) -> Vec<String> { vec![format!("gate::{owner}")] }
pub fn _impact_contracts() -> Vec<String> { stage16_names(&["no-visible-change-without-commit", "shadow-diff-before-switch"]) }
pub fn _impact_sources() -> Vec<String> { stage16_names(&["python_reference", "python_arch_reference", "rust_shadow"]) }
pub fn _mermaid_diagram() -> String { "graph TD; source-->candidate; candidate-->gate".to_string() }
pub fn _migration_candidates() -> Vec<String> { stage16_names(&["table_adapters", "prompt_interaction", "execution_network"]) }
pub fn _plan() -> String { "stage33_impact_plan".to_string() }
pub fn _source_kind(owner: &str) -> String { if owner.contains("prompt") { "prompt" } else if owner.contains("table") { "table" } else { "runtime" }.to_string() }
pub fn _text_diagram() -> String { "impact source -> migration candidate -> regression gate".to_string() }
pub fn _validate() -> String { "impact-validation-pending-runtime-probe".to_string() }
pub fn source_named(name: &str) -> String { format!("source::{name}") }
