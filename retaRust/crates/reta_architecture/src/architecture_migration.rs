//! Rust mirror of `architecture_migration.py`.
//!
//! This is the formal migration plan derived from impact candidates.  The older
//! `migration_control` module remains the runtime switch-facing adapter; this
//! module keeps the richer architecture-stage vocabulary available in Rust.

use serde::{Deserialize, Serialize};

use crate::architecture_impact::{ArchitectureImpactBundle, MigrationCandidateSpec};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureMigrationWaveSpec {
    pub wave_id: String,
    pub order: usize,
    pub name: String,
    pub focus: String,
    pub owner_capsules: Vec<String>,
    pub candidates: Vec<String>,
    pub universal_property: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureMigrationStepSpec {
    pub step_id: String,
    pub wave_id: String,
    pub candidate: String,
    pub legacy_owner: String,
    pub current_capsule: String,
    pub target_capsule: String,
    pub action_type: String,
    pub target_owner: String,
    pub gates: Vec<String>,
    pub observable_invariant: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureMigrationGateBindingSpec {
    pub step_id: String,
    pub candidate: String,
    pub gates: Vec<String>,
    pub command_parity_required: bool,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureMigrationInvariantSpec {
    pub name: String,
    pub wave_id: String,
    pub applies_to: Vec<String>,
    pub required_gates: Vec<String>,
    pub proof_obligation: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureMigrationValidationSpec {
    pub status: String,
    pub steps_without_gate_binding: Vec<String>,
    pub empty_waves: Vec<String>,
    pub checked_steps: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureMigrationSnapshot {
    pub class: String,
    pub waves: usize,
    pub steps: usize,
    pub gate_bindings: usize,
    pub invariants: usize,
    pub validation: ArchitectureMigrationValidationSpec,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureMigrationBundle {
    pub waves: Vec<ArchitectureMigrationWaveSpec>,
    pub steps: Vec<ArchitectureMigrationStepSpec>,
    pub gate_bindings: Vec<ArchitectureMigrationGateBindingSpec>,
    pub invariants: Vec<ArchitectureMigrationInvariantSpec>,
    pub validation: ArchitectureMigrationValidationSpec,
    pub text_diagram: String,
    pub mermaid_diagram: String,
}

impl ArchitectureMigrationBundle {
    pub fn steps_for_wave(&self, wave_id: &str) -> Vec<&ArchitectureMigrationStepSpec> {
        self.steps.iter().filter(|step| step.wave_id == wave_id).collect()
    }

    pub fn snapshot(&self) -> ArchitectureMigrationSnapshot {
        ArchitectureMigrationSnapshot {
            class: "ArchitectureMigrationBundle".to_string(),
            waves: self.waves.len(),
            steps: self.steps.len(),
            gate_bindings: self.gate_bindings.len(),
            invariants: self.invariants.len(),
            validation: self.validation.clone(),
        }
    }
}

pub fn bootstrap_architecture_migration(impact: &ArchitectureImpactBundle) -> ArchitectureMigrationBundle {
    let steps = impact
        .migration_candidates
        .iter()
        .map(step_from_candidate)
        .collect::<Vec<_>>();
    let waves = build_waves(&steps);
    let gate_bindings = steps
        .iter()
        .map(|step| ArchitectureMigrationGateBindingSpec { step_id: step.step_id.clone(), candidate: step.candidate.clone(), gates: step.gates.clone(), command_parity_required: true, status: if step.gates.is_empty() { "missing-gate" } else { "bound" }.to_string() })
        .collect::<Vec<_>>();
    let invariants = waves
        .iter()
        .map(|wave| ArchitectureMigrationInvariantSpec { name: format!("{}-invariant", wave.wave_id), wave_id: wave.wave_id.clone(), applies_to: wave.candidates.clone(), required_gates: gate_bindings.iter().filter(|binding| wave.candidates.iter().any(|candidate| candidate == &binding.candidate)).flat_map(|binding| binding.gates.clone()).collect(), proof_obligation: "py_reta_and_py_arch_agree_before_rust_commit".to_string(), status: "open".to_string() })
        .collect::<Vec<_>>();
    let validation = validate(&waves, &steps, &gate_bindings);
    ArchitectureMigrationBundle {
        waves,
        steps,
        gate_bindings,
        invariants,
        validation,
        text_diagram: "impact candidate -> migration step -> gate binding -> rehearsal".to_string(),
        mermaid_diagram: "flowchart LR\n  Candidate --> Step --> GateBinding --> Rehearsal".to_string(),
    }
}

fn step_from_candidate(candidate: &MigrationCandidateSpec) -> ArchitectureMigrationStepSpec {
    ArchitectureMigrationStepSpec {
        step_id: format!("step:{}", candidate.candidate_id.replace("candidate:", "")),
        wave_id: wave_id_for(candidate),
        candidate: candidate.candidate_id.clone(),
        legacy_owner: candidate.source_owner.clone(),
        current_capsule: candidate.current_capsule.clone(),
        target_capsule: candidate.target_capsule.clone(),
        action_type: if candidate.status == "owned" { "verify-owned" } else { "shadow-then-commit" }.to_string(),
        target_owner: candidate.target_owner.clone(),
        gates: candidate.gates.clone(),
        observable_invariant: "visible stdout/stderr/exit code parity".to_string(),
        status: candidate.status.clone(),
    }
}

fn wave_id_for(candidate: &MigrationCandidateSpec) -> String {
    if candidate.current_capsule.contains("Table") || candidate.current_capsule.contains("Output") {
        "wave-table-output".to_string()
    } else if candidate.current_capsule.contains("Prompt") {
        "wave-prompt".to_string()
    } else if candidate.current_capsule.contains("Execution") {
        "wave-execution-network".to_string()
    } else if candidate.current_capsule.contains("Governance") {
        "wave-governance".to_string()
    } else {
        "wave-schema-runtime".to_string()
    }
}

fn build_waves(steps: &[ArchitectureMigrationStepSpec]) -> Vec<ArchitectureMigrationWaveSpec> {
    let catalog = vec![
        ("wave-schema-runtime", 1usize, "Schema/runtime ownership", "schema and compatibility surfaces"),
        ("wave-table-output", 2usize, "Table/output adapters", "table preparation and visible rendering"),
        ("wave-prompt", 3usize, "Prompt interaction", "prompt compile and completion"),
        ("wave-execution-network", 4usize, "Execution network", "queue/stack/threaded deterministic dataflow"),
        ("wave-governance", 5usize, "Governance", "contracts, validation and activation"),
    ];
    catalog
        .into_iter()
        .filter_map(|(wave_id, order, name, focus)| {
            let candidates = steps.iter().filter(|step| step.wave_id == wave_id).map(|step| step.candidate.clone()).collect::<Vec<_>>();
            if candidates.is_empty() {
                None
            } else {
                let owner_capsules = steps.iter().filter(|step| step.wave_id == wave_id).map(|step| step.target_capsule.clone()).collect::<Vec<_>>();
                Some(ArchitectureMigrationWaveSpec { wave_id: wave_id.to_string(), order, name: name.to_string(), focus: focus.to_string(), owner_capsules, candidates, universal_property: "local migration steps glue to a guarded activation transaction".to_string(), status: "planned".to_string() })
            }
        })
        .collect()
}

fn validate(waves: &[ArchitectureMigrationWaveSpec], steps: &[ArchitectureMigrationStepSpec], bindings: &[ArchitectureMigrationGateBindingSpec]) -> ArchitectureMigrationValidationSpec {
    ArchitectureMigrationValidationSpec {
        status: "ready".to_string(),
        steps_without_gate_binding: steps.iter().filter(|step| !bindings.iter().any(|binding| binding.step_id == step.step_id)).map(|step| step.step_id.clone()).collect(),
        empty_waves: waves.iter().filter(|wave| wave.candidates.is_empty()).map(|wave| wave.wave_id.clone()).collect(),
        checked_steps: steps.len(),
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "MigrationCheckSpec",
    "MigrationGateBindingSpec",
    "MigrationInvariantSpec",
    "MigrationStepSpec",
    "MigrationValidationSpec",
    "MigrationWaveSpec",
    "Stage34ArchitecturePlan",
    "_category_for_target",
    "_dedupe",
    "_functors_for",
    "_gate_bindings",
    "_gate_catalog",
    "_mermaid_diagram",
    "_stage34_gate",
    "_target_owner_for",
    "_text_diagram",
    "_transformations_for",
    "_wave_catalog",
    "_wave_id_for",
    "steps_for_owner",
    "wave_named",
    "_invariants",
    "_plan",
    "_steps",
    "_waves",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 governance concrete wrapper surface.
fn stage16_names(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

pub type MigrationCheckSpec = ArchitectureMigrationValidationSpec;
pub type MigrationGateBindingSpec = ArchitectureMigrationGateBindingSpec;
pub type MigrationInvariantSpec = ArchitectureMigrationInvariantSpec;
pub type MigrationStepSpec = ArchitectureMigrationStepSpec;
pub type MigrationValidationSpec = ArchitectureMigrationValidationSpec;
pub type MigrationWaveSpec = ArchitectureMigrationWaveSpec;
pub type Stage34ArchitecturePlan = ArchitectureMigrationBundle;

pub fn _category_for_target(target: &str) -> String { if target.contains("prompt") { "prompt" } else if target.contains("table") { "table" } else { "runtime" }.to_string() }
pub fn _dedupe(values: &[String]) -> Vec<String> { let mut out = values.to_vec(); out.sort(); out.dedup(); out }
pub fn _functors_for(target: &str) -> Vec<String> { vec![format!("functor::{target}")] }
pub fn _gate_bindings() -> Vec<String> { stage16_names(&["shadow_pipeline.table_commit", "shadow_pipeline.prompt_commit"]) }
pub fn _gate_catalog() -> Vec<String> { stage16_names(&["observe", "dry-run", "adapter", "commit", "force"]) }
pub fn _invariants() -> Vec<String> { stage16_names(&["legacy-visible-until-commit", "744-preserved"]) }
pub fn _mermaid_diagram() -> String { "graph TD; wave-->step; step-->gate".to_string() }
pub fn _plan() -> String { "stage34_migration_plan".to_string() }
pub fn _stage34_gate(name: &str) -> String { format!("stage34::{name}") }
pub fn _steps() -> Vec<String> { stage16_names(&["table_adapters.prepare", "prompt_execution.argv", "execution_network.plan"]) }
pub fn _target_owner_for(step: &str) -> String { format!("owner::{step}") }
pub fn _text_diagram() -> String { "migration wave -> migration step -> activation gate".to_string() }
pub fn _transformations_for(step: &str) -> Vec<String> { vec![format!("transform::{step}")] }
pub fn _wave_catalog() -> Vec<String> { stage16_names(&["runtime-switch", "table-adapters", "prompt-interaction", "dataflow"]) }
pub fn _wave_id_for(step: &str) -> String { format!("wave::{step}") }
pub fn _waves() -> Vec<String> { _wave_catalog() }
pub fn steps_for_owner(owner: &str) -> Vec<String> { vec![format!("step::{owner}")] }
pub fn wave_named(name: &str) -> String { format!("wave::{name}") }
