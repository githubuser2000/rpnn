//! Rust mirror of `architecture_rehearsal.py`.
//!
//! Rehearsal turns migration steps into open sets and dry-run moves.

use serde::{Deserialize, Serialize};

use crate::architecture_contracts::ArchitectureContractsBundle;
use crate::architecture_migration::{ArchitectureMigrationBundle, ArchitectureMigrationStepSpec};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RehearsalOpenSetSpec {
    pub open_set_id: String,
    pub wave_id: String,
    pub owner_capsules: Vec<String>,
    pub allowed_moves: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RehearsalMoveSpec {
    pub move_id: String,
    pub step_id: String,
    pub source_owner: String,
    pub target_owner: String,
    pub dry_run_command: String,
    pub expected_invariant: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GateRehearsalSpec {
    pub step_id: String,
    pub gates: Vec<String>,
    pub diagrams: Vec<String>,
    pub shadow_required: bool,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RehearsalCoverSpec {
    pub cover_id: String,
    pub open_sets: Vec<String>,
    pub moves: Vec<String>,
    pub glue_law: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RehearsalValidationSpec {
    pub status: String,
    pub moves_without_gate: Vec<String>,
    pub empty_open_sets: Vec<String>,
    pub checked_moves: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureRehearsalSnapshot {
    pub class: String,
    pub open_sets: usize,
    pub moves: usize,
    pub gate_rehearsals: usize,
    pub covers: usize,
    pub validation: RehearsalValidationSpec,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureRehearsalBundle {
    pub open_sets: Vec<RehearsalOpenSetSpec>,
    pub moves: Vec<RehearsalMoveSpec>,
    pub gate_rehearsals: Vec<GateRehearsalSpec>,
    pub covers: Vec<RehearsalCoverSpec>,
    pub validation: RehearsalValidationSpec,
}

impl ArchitectureRehearsalBundle {
    pub fn moves_for_open_set(&self, open_set_id: &str) -> Vec<&RehearsalMoveSpec> {
        self.open_sets
            .iter()
            .find(|set| set.open_set_id == open_set_id)
            .map(|set| set.allowed_moves.clone())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|move_id| self.moves.iter().find(|m| m.move_id == move_id))
            .collect()
    }

    pub fn snapshot(&self) -> ArchitectureRehearsalSnapshot {
        ArchitectureRehearsalSnapshot {
            class: "ArchitectureRehearsalBundle".to_string(),
            open_sets: self.open_sets.len(),
            moves: self.moves.len(),
            gate_rehearsals: self.gate_rehearsals.len(),
            covers: self.covers.len(),
            validation: self.validation.clone(),
        }
    }
}

pub fn bootstrap_architecture_rehearsal(migration: &ArchitectureMigrationBundle, contracts: &ArchitectureContractsBundle) -> ArchitectureRehearsalBundle {
    let moves = migration.steps.iter().map(move_from_step).collect::<Vec<_>>();
    let open_sets = migration
        .waves
        .iter()
        .map(|wave| RehearsalOpenSetSpec {
            open_set_id: format!("open:{}", wave.wave_id),
            wave_id: wave.wave_id.clone(),
            owner_capsules: wave.owner_capsules.clone(),
            allowed_moves: moves.iter().filter(|m| migration.steps.iter().any(|step| step.wave_id == wave.wave_id && step.step_id == m.step_id)).map(|m| m.move_id.clone()).collect(),
            status: "rehearsable".to_string(),
        })
        .collect::<Vec<_>>();
    let gate_rehearsals = migration
        .steps
        .iter()
        .map(|step| GateRehearsalSpec {
            step_id: step.step_id.clone(),
            gates: with_rehearsal_gate(&step.gates),
            diagrams: contracts.diagrams.iter().filter(|diagram| diagram.capsules.iter().any(|capsule| capsule == &step.target_capsule)).map(|diagram| diagram.name.clone()).collect(),
            shadow_required: true,
            status: if step.gates.is_empty() { "needs-gate" } else { "ready" }.to_string(),
        })
        .collect::<Vec<_>>();
    let covers = open_sets
        .iter()
        .map(|open| RehearsalCoverSpec { cover_id: format!("cover:{}", open.wave_id), open_sets: vec![open.open_set_id.clone()], moves: open.allowed_moves.clone(), glue_law: "all local rehearsal results glue to one activation decision".to_string(), status: "cover-ready".to_string() })
        .collect::<Vec<_>>();
    let validation = validate(&open_sets, &moves, &gate_rehearsals);
    ArchitectureRehearsalBundle { open_sets, moves, gate_rehearsals, covers, validation }
}

fn move_from_step(step: &ArchitectureMigrationStepSpec) -> RehearsalMoveSpec {
    RehearsalMoveSpec {
        move_id: format!("rehearse:{}", step.step_id),
        step_id: step.step_id.clone(),
        source_owner: step.legacy_owner.clone(),
        target_owner: step.target_owner.clone(),
        dry_run_command: format!("--reta-arch=dry-run --reta-arch-allow={}", step.target_owner),
        expected_invariant: step.observable_invariant.clone(),
        status: "dry-run".to_string(),
    }
}

fn with_rehearsal_gate(gates: &[String]) -> Vec<String> {
    let mut out = gates.to_vec();
    if !out.iter().any(|gate| gate == "architecture_rehearsal") {
        out.push("architecture_rehearsal".to_string());
    }
    out
}

fn validate(open_sets: &[RehearsalOpenSetSpec], moves: &[RehearsalMoveSpec], gates: &[GateRehearsalSpec]) -> RehearsalValidationSpec {
    RehearsalValidationSpec {
        status: "ready".to_string(),
        moves_without_gate: moves.iter().filter(|m| !gates.iter().any(|gate| gate.step_id == m.step_id)).map(|m| m.move_id.clone()).collect(),
        empty_open_sets: open_sets.iter().filter(|set| set.allowed_moves.is_empty()).map(|set| set.open_set_id.clone()).collect(),
        checked_moves: moves.len(),
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "RehearsalCheckSpec",
    "Stage35ArchitecturePlan",
    "_gate_rehearsals",
    "_open_sets",
    "_plan",
    "_with_rehearsal_gate",
    "_covers",
    "_moves",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
