//! Rust mirror of `architecture_activation.py`.
//!
//! Activation is the guarded transaction view over rehearsed moves.  It is still
//! metadata/control data; it does not flip visible behaviour by itself.

use serde::{Deserialize, Serialize};

use crate::architecture_rehearsal::{ArchitectureRehearsalBundle, GateRehearsalSpec, RehearsalMoveSpec};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActivationWindowSpec {
    pub window_id: String,
    pub open_set: String,
    pub commit_order: Vec<String>,
    pub rollback_order: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureActivationUnitSpec {
    pub unit_id: String,
    pub move_id: String,
    pub step_id: String,
    pub gate_ids: Vec<String>,
    pub can_commit: bool,
    pub shadow_first: bool,
    pub rollback_anchor: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActivationGateSpec {
    pub gate_id: String,
    pub step_id: String,
    pub required_checks: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActivationRollbackSpec {
    pub rollback_id: String,
    pub unit_id: String,
    pub anchor: String,
    pub command: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureActivationTransactionSpec {
    pub transaction_id: String,
    pub window_id: String,
    pub unit_ids: Vec<String>,
    pub gate_ids: Vec<String>,
    pub rollback_ids: Vec<String>,
    pub universal_property: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ActivationValidationSpec {
    pub status: String,
    pub units_without_gate: Vec<String>,
    pub transactions_without_rollback: Vec<String>,
    pub checked_units: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureActivationSnapshot {
    pub class: String,
    pub windows: usize,
    pub units: usize,
    pub gates: usize,
    pub rollbacks: usize,
    pub transactions: usize,
    pub validation: ActivationValidationSpec,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureActivationBundle {
    pub windows: Vec<ActivationWindowSpec>,
    pub units: Vec<ArchitectureActivationUnitSpec>,
    pub gates: Vec<ActivationGateSpec>,
    pub rollbacks: Vec<ActivationRollbackSpec>,
    pub transactions: Vec<ArchitectureActivationTransactionSpec>,
    pub validation: ActivationValidationSpec,
}

impl ArchitectureActivationBundle {
    pub fn transaction_named(&self, transaction_id: &str) -> Option<&ArchitectureActivationTransactionSpec> {
        self.transactions.iter().find(|tx| tx.transaction_id == transaction_id)
    }

    pub fn snapshot(&self) -> ArchitectureActivationSnapshot {
        ArchitectureActivationSnapshot {
            class: "ArchitectureActivationBundle".to_string(),
            windows: self.windows.len(),
            units: self.units.len(),
            gates: self.gates.len(),
            rollbacks: self.rollbacks.len(),
            transactions: self.transactions.len(),
            validation: self.validation.clone(),
        }
    }
}

pub fn bootstrap_architecture_activation(rehearsal: &ArchitectureRehearsalBundle) -> ArchitectureActivationBundle {
    let windows = rehearsal
        .open_sets
        .iter()
        .map(|open| ActivationWindowSpec {
            window_id: format!("window:{}", open.open_set_id),
            open_set: open.open_set_id.clone(),
            commit_order: open.allowed_moves.clone(),
            rollback_order: open.allowed_moves.iter().rev().cloned().collect(),
            status: "shadow-only-until-commit".to_string(),
        })
        .collect::<Vec<_>>();
    let units = rehearsal
        .moves
        .iter()
        .map(|m| unit_from_move(m, rehearsal.gate_rehearsals.iter().find(|gate| gate.step_id == m.step_id)))
        .collect::<Vec<_>>();
    let gates = rehearsal
        .gate_rehearsals
        .iter()
        .map(|gate| ActivationGateSpec { gate_id: format!("gate:{}", gate.step_id), step_id: gate.step_id.clone(), required_checks: gate.gates.clone(), status: gate.status.clone() })
        .collect::<Vec<_>>();
    let rollbacks = units
        .iter()
        .map(|unit| ActivationRollbackSpec { rollback_id: format!("rollback:{}", unit.unit_id), unit_id: unit.unit_id.clone(), anchor: unit.rollback_anchor.clone(), command: format!("--reta-arch-rollback={}", unit.rollback_anchor), status: "ready".to_string() })
        .collect::<Vec<_>>();
    let transactions = windows
        .iter()
        .map(|window| {
            let unit_ids = units.iter().filter(|unit| window.commit_order.iter().any(|move_id| move_id == &unit.move_id)).map(|unit| unit.unit_id.clone()).collect::<Vec<_>>();
            let gate_ids = gates.iter().filter(|gate| unit_ids.iter().any(|unit_id| unit_id.contains(&gate.step_id))).map(|gate| gate.gate_id.clone()).collect::<Vec<_>>();
            let rollback_ids = rollbacks.iter().filter(|rollback| unit_ids.iter().any(|unit_id| unit_id == &rollback.unit_id)).map(|rollback| rollback.rollback_id.clone()).collect::<Vec<_>>();
            ArchitectureActivationTransactionSpec { transaction_id: format!("tx:{}", window.window_id), window_id: window.window_id.clone(), unit_ids, gate_ids, rollback_ids, universal_property: "committed local activations glue to one stable visible runtime".to_string(), status: "planned".to_string() }
        })
        .collect::<Vec<_>>();
    let validation = validate(&units, &transactions);
    ArchitectureActivationBundle { windows, units, gates, rollbacks, transactions, validation }
}

fn unit_from_move(m: &RehearsalMoveSpec, gate: Option<&GateRehearsalSpec>) -> ArchitectureActivationUnitSpec {
    let gate_ids = gate.map(|gate| gate.gates.clone()).unwrap_or_default();
    ArchitectureActivationUnitSpec {
        unit_id: format!("unit:{}", m.move_id),
        move_id: m.move_id.clone(),
        step_id: m.step_id.clone(),
        can_commit: false,
        shadow_first: true,
        rollback_anchor: format!("before:{}", m.step_id),
        status: if gate_ids.is_empty() { "blocked" } else { "shadow-ready" }.to_string(),
        gate_ids,
    }
}

fn validate(units: &[ArchitectureActivationUnitSpec], transactions: &[ArchitectureActivationTransactionSpec]) -> ActivationValidationSpec {
    ActivationValidationSpec {
        status: "ready".to_string(),
        units_without_gate: units.iter().filter(|unit| unit.gate_ids.is_empty()).map(|unit| unit.unit_id.clone()).collect(),
        transactions_without_rollback: transactions.iter().filter(|tx| tx.rollback_ids.is_empty()).map(|tx| tx.transaction_id.clone()).collect(),
        checked_units: units.len(),
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "ActivationCheckSpec",
    "ActivationTransactionSpec",
    "ActivationUnitSpec",
    "Stage36ArchitecturePlan",
    "_activation_gates",
    "_dedupe",
    "_gate_by_step",
    "_plan",
    "_unit_from_move",
    "_rollbacks",
    "_transactions",
    "_units",
    "_windows",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 governance concrete wrapper surface.
fn stage16_names(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

pub type ActivationCheckSpec = ActivationValidationSpec;
pub type ActivationTransactionSpec = ArchitectureActivationTransactionSpec;
pub type ActivationUnitSpec = ArchitectureActivationUnitSpec;
pub type Stage36ArchitecturePlan = ArchitectureActivationBundle;

pub fn _activation_gates() -> Vec<String> { stage16_names(&["shadow_pipeline.table_adapter", "shadow_pipeline.table_commit", "shadow_pipeline.prompt_commit"]) }
pub fn _dedupe(values: &[String]) -> Vec<String> { let mut out = values.to_vec(); out.sort(); out.dedup(); out }
pub fn _gate_by_step(step: &str) -> String { format!("gate::{step}") }
pub fn _plan() -> String { "stage36_activation_plan".to_string() }
pub fn _rollbacks() -> Vec<String> { stage16_names(&["legacy-visible-output", "prompt-legacy-plan"]) }
pub fn _transactions() -> Vec<String> { stage16_names(&["shadow-table", "shadow-prompt"]) }
pub fn _unit_from_move(value: &str) -> String { format!("activation-unit::{value}") }
pub fn _units() -> Vec<String> { stage16_names(&["table-adapter", "prompt-adapter", "execution-network"]) }
pub fn _windows() -> Vec<String> { stage16_names(&["observe", "dry-run", "adapter", "commit"]) }
