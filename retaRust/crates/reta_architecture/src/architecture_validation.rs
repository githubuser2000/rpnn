//! Rust mirror of `architecture_validation.py`.
//!
//! Validation is the cross-layer summary over map, contracts, witnesses,
//! coherence, boundaries, impact, migration, rehearsal, activation and progress.

use serde::{Deserialize, Serialize};

use crate::architecture_activation::ArchitectureActivationBundle;
use crate::architecture_boundaries::ArchitectureBoundariesBundle;
use crate::architecture_coherence::ArchitectureCoherenceBundle;
use crate::architecture_contracts::ArchitectureContractsBundle;
use crate::architecture_impact::ArchitectureImpactBundle;
use crate::architecture_map::ArchitectureMapBundle;
use crate::architecture_migration::ArchitectureMigrationBundle;
use crate::architecture_progress::ArchitectureProgressBundle;
use crate::architecture_rehearsal::ArchitectureRehearsalBundle;
use crate::architecture_traces::ArchitectureTraceBundle;
use crate::architecture_witnesses::ArchitectureWitnessBundle;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureValidationCheckSpec {
    pub name: String,
    pub layer: String,
    pub status: String,
    pub failed_items: Vec<String>,
    pub checked_count: usize,
    pub reading: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureValidationLayerSpec {
    pub layer: String,
    pub passed: usize,
    pub failed: usize,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureValidationSummarySpec {
    pub status: String,
    pub passed_checks: usize,
    pub failed_checks: usize,
    pub total_checks: usize,
    pub failed_items: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureValidationSnapshot {
    pub class: String,
    pub checks: usize,
    pub layers: usize,
    pub summary: ArchitectureValidationSummarySpec,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureValidationBundle {
    pub checks: Vec<ArchitectureValidationCheckSpec>,
    pub layers: Vec<ArchitectureValidationLayerSpec>,
    pub summary: ArchitectureValidationSummarySpec,
    pub stage_plan: Vec<String>,
}

impl ArchitectureValidationBundle {
    pub fn is_ready(&self) -> bool {
        self.summary.status == "ready"
    }

    pub fn snapshot(&self) -> ArchitectureValidationSnapshot {
        ArchitectureValidationSnapshot { class: "ArchitectureValidationBundle".to_string(), checks: self.checks.len(), layers: self.layers.len(), summary: self.summary.clone() }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn bootstrap_architecture_validation(
    map: &ArchitectureMapBundle,
    contracts: &ArchitectureContractsBundle,
    witnesses: &ArchitectureWitnessBundle,
    coherence: &ArchitectureCoherenceBundle,
    boundaries: &ArchitectureBoundariesBundle,
    traces: &ArchitectureTraceBundle,
    impact: &ArchitectureImpactBundle,
    migration: &ArchitectureMigrationBundle,
    rehearsal: &ArchitectureRehearsalBundle,
    activation: &ArchitectureActivationBundle,
    progress: &ArchitectureProgressBundle,
) -> ArchitectureValidationBundle {
    let checks = vec![
        check("architecture-map-capsules", "map", map.capsules.is_empty(), Vec::new(), map.capsules.len(), "capsule map must not be empty"),
        check("architecture-contracts", "contracts", contracts.validation.status != "ready", contracts.validation.unknown_capsules.clone(), contracts.diagrams.len(), "contracts must refer to known capsules"),
        check("architecture-witnesses", "witnesses", witnesses.validation.status != "ready", witnesses.validation.unresolved_anchors.clone(), witnesses.anchor_witnesses.len(), "witness anchors must resolve to owners"),
        check("architecture-coherence", "coherence", coherence.validation.status != "ready", coherence.validation.routes_without_functor.clone(), coherence.functorial_routes.len(), "all routes need named functors"),
        check("architecture-boundaries", "boundaries", boundaries.validation.status != "ready", boundaries.validation.forbidden_cross_edges.clone(), boundaries.import_edges.len(), "capsule boundary edges must be allowed"),
        check("architecture-traces", "traces", traces.validation.status != "ready", traces.validation.components_without_route.clone(), traces.component_traces.len(), "components must have route traces"),
        check("architecture-impact", "impact", impact.validation.status != "ready", impact.validation.candidates_without_gate.clone(), impact.migration_candidates.len(), "migration candidates need gates"),
        check("architecture-migration", "migration", migration.validation.status != "ready", migration.validation.steps_without_gate_binding.clone(), migration.steps.len(), "migration steps need gate bindings"),
        check("architecture-rehearsal", "rehearsal", rehearsal.validation.status != "ready", rehearsal.validation.moves_without_gate.clone(), rehearsal.moves.len(), "rehearsal moves need gate rehearsals"),
        check("architecture-activation", "activation", activation.validation.status != "ready", activation.validation.units_without_gate.clone(), activation.units.len(), "activation units need gates"),
        check("architecture-progress", "progress", progress.validation.status != "ready", progress.validation.unmapped_surfaces.clone(), progress.surfaces.len(), "progress must cover mapped surfaces"),
    ];
    let layers = layers_from_checks(&checks);
    let summary = summary_from_checks(&checks);
    ArchitectureValidationBundle {
        checks,
        layers,
        summary,
        stage_plan: vec![
            "validate-map".to_string(),
            "validate-contracts".to_string(),
            "validate-witnesses".to_string(),
            "validate-activation".to_string(),
            "validate-progress".to_string(),
        ],
    }
}

fn check(name: &str, layer: &str, failed: bool, failed_items: Vec<String>, checked_count: usize, reading: &str) -> ArchitectureValidationCheckSpec {
    ArchitectureValidationCheckSpec { name: name.to_string(), layer: layer.to_string(), status: if failed { "failed" } else { "passed" }.to_string(), failed_items, checked_count, reading: reading.to_string() }
}

fn layers_from_checks(checks: &[ArchitectureValidationCheckSpec]) -> Vec<ArchitectureValidationLayerSpec> {
    let mut layers = Vec::<ArchitectureValidationLayerSpec>::new();
    for check in checks {
        if let Some(layer) = layers.iter_mut().find(|layer| layer.layer == check.layer) {
            if check.status == "passed" { layer.passed += 1; } else { layer.failed += 1; layer.status = "failed".to_string(); }
        } else {
            layers.push(ArchitectureValidationLayerSpec { layer: check.layer.clone(), passed: usize::from(check.status == "passed"), failed: usize::from(check.status != "passed"), status: if check.status == "passed" { "ready" } else { "failed" }.to_string() });
        }
    }
    layers
}

fn summary_from_checks(checks: &[ArchitectureValidationCheckSpec]) -> ArchitectureValidationSummarySpec {
    let failed_items = checks.iter().flat_map(|check| check.failed_items.clone()).collect::<Vec<_>>();
    let failed_checks = checks.iter().filter(|check| check.status != "passed").count();
    ArchitectureValidationSummarySpec { status: if failed_checks == 0 { "ready" } else { "needs-attention" }.to_string(), passed_checks: checks.len() - failed_checks, failed_checks, total_checks: checks.len(), failed_items }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "Stage31ArchitecturePlan",
    "_activation_checks",
    "_arithmetic_checks",
    "_boundary_checks",
    "_category_checks",
    "_console_io_checks",
    "_contract_checks",
    "_impact_checks",
    "_map_checks",
    "_migration_checks",
    "_nested_completion_checks",
    "_plan",
    "_rehearsal_checks",
    "_repo_checks",
    "_row_range_checks",
    "_trace_checks",
    "_witness_checks",
    "_word_completion_checks",
    "assert_passed",
    "check_named",
    "_layers",
    "_passed",
    "_summary",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}
