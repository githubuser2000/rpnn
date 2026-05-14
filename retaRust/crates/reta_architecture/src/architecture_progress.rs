//! Rust mirror of `architecture_progress.py`.
//!
//! Progress summarizes which legacy surfaces are owned, shadowed or still open.

use serde::{Deserialize, Serialize};

use crate::architecture_activation::ArchitectureActivationBundle;
use crate::architecture_map::ArchitectureMapBundle;
use crate::architecture_migration::ArchitectureMigrationBundle;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LegacySurfaceProgressSpec {
    pub legacy_owner: String,
    pub capsule: String,
    pub target_owner: String,
    pub migration_status: String,
    pub wrapper_like: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MigrationExecutionSpec {
    pub step_id: String,
    pub legacy_owner: String,
    pub target_owner: String,
    pub execution_status: String,
    pub gates: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WaveExecutionSpec {
    pub wave_id: String,
    pub planned_steps: usize,
    pub shadow_ready_steps: usize,
    pub committed_steps: usize,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutstandingWorkItemSpec {
    pub item_id: String,
    pub owner: String,
    pub reason: String,
    pub suggested_next_gate: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProgressValidationSpec {
    pub status: String,
    pub unmapped_surfaces: Vec<String>,
    pub waves_without_steps: Vec<String>,
    pub outstanding_items: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureProgressSnapshot {
    pub class: String,
    pub surfaces: usize,
    pub step_progress: usize,
    pub wave_progress: usize,
    pub outstanding_work: usize,
    pub validation: ProgressValidationSpec,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureProgressBundle {
    pub surfaces: Vec<LegacySurfaceProgressSpec>,
    pub step_progress: Vec<MigrationExecutionSpec>,
    pub wave_progress: Vec<WaveExecutionSpec>,
    pub outstanding_work: Vec<OutstandingWorkItemSpec>,
    pub validation: ProgressValidationSpec,
    pub text_diagram: String,
    pub mermaid_diagram: String,
}

impl ArchitectureProgressBundle {
    pub fn outstanding_for_owner(&self, owner: &str) -> Vec<&OutstandingWorkItemSpec> {
        self.outstanding_work.iter().filter(|item| item.owner == owner).collect()
    }

    pub fn snapshot(&self) -> ArchitectureProgressSnapshot {
        ArchitectureProgressSnapshot {
            class: "ArchitectureProgressBundle".to_string(),
            surfaces: self.surfaces.len(),
            step_progress: self.step_progress.len(),
            wave_progress: self.wave_progress.len(),
            outstanding_work: self.outstanding_work.len(),
            validation: self.validation.clone(),
        }
    }
}

pub fn bootstrap_architecture_progress(map: &ArchitectureMapBundle, migration: &ArchitectureMigrationBundle, activation: &ArchitectureActivationBundle) -> ArchitectureProgressBundle {
    let surfaces = map
        .legacy_mappings
        .iter()
        .map(|mapping| LegacySurfaceProgressSpec { legacy_owner: mapping.legacy_owner.clone(), capsule: mapping.capsule.clone(), target_owner: mapping.target_owner.clone(), migration_status: mapping.migration_status.clone(), wrapper_like: mapping.migration_status == "owned" })
        .collect::<Vec<_>>();
    let step_progress = migration
        .steps
        .iter()
        .map(|step| {
            let unit = activation.units.iter().find(|unit| unit.step_id == step.step_id);
            MigrationExecutionSpec { step_id: step.step_id.clone(), legacy_owner: step.legacy_owner.clone(), target_owner: step.target_owner.clone(), execution_status: unit.map(|unit| unit.status.clone()).unwrap_or_else(|| "not-rehearsed".to_string()), gates: step.gates.clone() }
        })
        .collect::<Vec<_>>();
    let wave_progress = migration
        .waves
        .iter()
        .map(|wave| {
            let steps = step_progress.iter().filter(|step| migration.steps.iter().any(|mstep| mstep.wave_id == wave.wave_id && mstep.step_id == step.step_id)).collect::<Vec<_>>();
            let shadow_ready_steps = steps.iter().filter(|step| step.execution_status.contains("ready")).count();
            WaveExecutionSpec { wave_id: wave.wave_id.clone(), planned_steps: steps.len(), shadow_ready_steps, committed_steps: 0, status: if shadow_ready_steps == steps.len() { "shadow-ready" } else { "partial" }.to_string() }
        })
        .collect::<Vec<_>>();
    let outstanding_work = surfaces
        .iter()
        .filter(|surface| surface.migration_status != "owned")
        .map(|surface| OutstandingWorkItemSpec { item_id: format!("todo:{}", surface.legacy_owner.replace('/', ":")), owner: surface.legacy_owner.clone(), reason: format!("status={}", surface.migration_status), suggested_next_gate: "py_reta_vs_py_arch_vs_rust".to_string() })
        .collect::<Vec<_>>();
    let validation = ProgressValidationSpec { status: "ready".to_string(), unmapped_surfaces: Vec::new(), waves_without_steps: wave_progress.iter().filter(|wave| wave.planned_steps == 0).map(|wave| wave.wave_id.clone()).collect(), outstanding_items: outstanding_work.len() };
    ArchitectureProgressBundle { surfaces, step_progress, wave_progress, outstanding_work, validation, text_diagram: "surface -> step -> wave -> outstanding work".to_string(), mermaid_diagram: "flowchart LR\n  Surface --> Step --> Wave --> Outstanding".to_string() }
}
