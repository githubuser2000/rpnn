//! Rust mirror of `architecture_traces.py`.
//!
//! Traces connect components, capsules and stage history into explainable
//! routes.  They do not run code; they make route ownership auditable.

use serde::{Deserialize, Serialize};

use crate::architecture_coherence::ArchitectureCoherenceBundle;
use crate::architecture_contracts::ArchitectureContractsBundle;
use crate::architecture_map::ArchitectureMapBundle;
use crate::architecture_witnesses::ArchitectureWitnessBundle;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TraceHopSpec {
    pub from: String,
    pub to: String,
    pub via: String,
    pub category: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetaComponentTraceSpec {
    pub component: String,
    pub owner: String,
    pub capsule: String,
    pub route: Vec<TraceHopSpec>,
    pub diagrams: Vec<String>,
    pub witnesses: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapsuleTraceSpec {
    pub capsule: String,
    pub route_kind: String,
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
    pub coherence_status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StageHistoryTraceSpec {
    pub stage: String,
    pub focus: String,
    pub capsule: String,
    pub moved_to: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TraceValidationSpec {
    pub status: String,
    pub components_without_route: Vec<String>,
    pub capsules_without_trace: Vec<String>,
    pub checked_components: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureTraceSnapshot {
    pub class: String,
    pub component_traces: usize,
    pub capsule_traces: usize,
    pub stage_traces: usize,
    pub validation: TraceValidationSpec,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureTraceBundle {
    pub component_traces: Vec<RetaComponentTraceSpec>,
    pub capsule_traces: Vec<CapsuleTraceSpec>,
    pub stage_traces: Vec<StageHistoryTraceSpec>,
    pub validation: TraceValidationSpec,
    pub text_diagram: String,
    pub mermaid_diagram: String,
}

impl ArchitectureTraceBundle {
    pub fn trace_for_component(&self, component: &str) -> Option<&RetaComponentTraceSpec> {
        self.component_traces.iter().find(|trace| trace.component == component)
    }

    pub fn snapshot(&self) -> ArchitectureTraceSnapshot {
        ArchitectureTraceSnapshot {
            class: "ArchitectureTraceBundle".to_string(),
            component_traces: self.component_traces.len(),
            capsule_traces: self.capsule_traces.len(),
            stage_traces: self.stage_traces.len(),
            validation: self.validation.clone(),
        }
    }
}

pub fn bootstrap_architecture_traces(
    map: &ArchitectureMapBundle,
    contracts: &ArchitectureContractsBundle,
    witnesses: &ArchitectureWitnessBundle,
    coherence: &ArchitectureCoherenceBundle,
) -> ArchitectureTraceBundle {
    let component_traces = map
        .legacy_mappings
        .iter()
        .map(|mapping| RetaComponentTraceSpec {
            component: mapping.legacy_owner.clone(),
            owner: mapping.target_owner.clone(),
            capsule: mapping.capsule.clone(),
            route: vec![
                TraceHopSpec { from: mapping.legacy_owner.clone(), to: mapping.capsule.clone(), via: "legacy-owner-map".to_string(), category: "ArchitectureMapCategory".to_string() },
                TraceHopSpec { from: mapping.capsule.clone(), to: mapping.target_owner.clone(), via: "target-owner".to_string(), category: "RustArchitectureCategory".to_string() },
            ],
            diagrams: contracts
                .diagrams
                .iter()
                .filter(|diagram| diagram.capsules.iter().any(|capsule| capsule == &mapping.capsule))
                .map(|diagram| diagram.name.clone())
                .collect(),
            witnesses: witnesses
                .anchor_witnesses
                .iter()
                .filter(|witness| witness.owner == mapping.legacy_owner)
                .map(|witness| witness.anchor.clone())
                .collect(),
            status: "traced".to_string(),
        })
        .collect::<Vec<_>>();
    let capsule_traces = map
        .capsules
        .iter()
        .map(|capsule| CapsuleTraceSpec {
            capsule: capsule.name.clone(),
            route_kind: if capsule.name.contains("Execution") { "runtime" } else { "semantic" }.to_string(),
            inbound: capsule.inbound.clone(),
            outbound: capsule.outbound.clone(),
            coherence_status: coherence.status_for_capsule(&capsule.name).unwrap_or("structural").to_string(),
        })
        .collect::<Vec<_>>();
    let stage_traces = map
        .stage_steps
        .iter()
        .map(|stage| StageHistoryTraceSpec { stage: stage.stage.clone(), focus: stage.focus.clone(), capsule: stage.capsule.clone(), moved_to: stage.moved_to.clone(), status: "recorded".to_string() })
        .collect::<Vec<_>>();
    let validation = TraceValidationSpec {
        status: "ready".to_string(),
        components_without_route: component_traces.iter().filter(|trace| trace.route.is_empty()).map(|trace| trace.component.clone()).collect(),
        capsules_without_trace: map
            .capsules
            .iter()
            .filter(|capsule| !capsule_traces.iter().any(|trace| trace.capsule == capsule.name))
            .map(|capsule| capsule.name.clone())
            .collect(),
        checked_components: component_traces.len(),
    };
    ArchitectureTraceBundle {
        component_traces,
        capsule_traces,
        stage_traces,
        validation,
        text_diagram: "legacy component -> capsule -> rust owner -> parity witness".to_string(),
        mermaid_diagram: "flowchart LR\n  LegacySurface --> Capsule --> RustOwner --> Witness".to_string(),
    }
}
