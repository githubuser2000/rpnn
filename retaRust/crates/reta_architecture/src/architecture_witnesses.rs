//! Rust mirror of `architecture_witnesses.py`.
//!
//! Witnesses bind abstract diagrams to concrete source anchors and probe names.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::architecture_contracts::ArchitectureContractsBundle;
use crate::architecture_map::ArchitectureMapBundle;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnchorWitnessSpec {
    pub owner: String,
    pub anchor: String,
    pub resolved: bool,
    pub candidate_paths: Vec<String>,
    pub capsule: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapsuleSliceSpec {
    pub capsule: String,
    pub owners: Vec<String>,
    pub anchors: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiagramWitnessSpec {
    pub diagram: String,
    pub probe_commands: Vec<String>,
    pub involved_capsules: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NaturalTransformationWitnessSpec {
    pub transformation: String,
    pub diagrams: Vec<String>,
    pub probe_commands: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RefactorObligationSpec {
    pub law: String,
    pub required_diagrams: Vec<String>,
    pub required_probes: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WitnessValidationSpec {
    pub status: String,
    pub unresolved_anchors: Vec<String>,
    pub diagrams_without_probe: Vec<String>,
    pub obligations_without_probe: Vec<String>,
    pub checked_witnesses: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureWitnessSnapshot {
    pub class: String,
    pub anchors: usize,
    pub capsule_slices: usize,
    pub diagram_witnesses: usize,
    pub naturality_witnesses: usize,
    pub obligations: usize,
    pub validation: WitnessValidationSpec,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureWitnessBundle {
    pub anchor_witnesses: Vec<AnchorWitnessSpec>,
    pub capsule_slices: Vec<CapsuleSliceSpec>,
    pub diagram_witnesses: Vec<DiagramWitnessSpec>,
    pub naturality_witnesses: Vec<NaturalTransformationWitnessSpec>,
    pub obligations: Vec<RefactorObligationSpec>,
    pub validation: WitnessValidationSpec,
}

impl ArchitectureWitnessBundle {
    pub fn probes_for_diagram(&self, diagram: &str) -> Vec<String> {
        self.diagram_witnesses
            .iter()
            .find(|witness| witness.diagram == diagram)
            .map(|witness| witness.probe_commands.clone())
            .unwrap_or_default()
    }

    pub fn snapshot(&self) -> ArchitectureWitnessSnapshot {
        ArchitectureWitnessSnapshot {
            class: "ArchitectureWitnessBundle".to_string(),
            anchors: self.anchor_witnesses.len(),
            capsule_slices: self.capsule_slices.len(),
            diagram_witnesses: self.diagram_witnesses.len(),
            naturality_witnesses: self.naturality_witnesses.len(),
            obligations: self.obligations.len(),
            validation: self.validation.clone(),
        }
    }
}

pub fn bootstrap_architecture_witnesses(map: &ArchitectureMapBundle, contracts: &ArchitectureContractsBundle) -> ArchitectureWitnessBundle {
    let anchor_witnesses = map
        .legacy_mappings
        .iter()
        .map(|mapping| AnchorWitnessSpec {
            owner: mapping.legacy_owner.clone(),
            anchor: mapping.target_owner.clone(),
            resolved: true,
            candidate_paths: vec![mapping.target_owner.clone(), mapping.legacy_owner.clone()],
            capsule: mapping.capsule.clone(),
        })
        .collect::<Vec<_>>();
    let capsule_slices = map
        .capsules
        .iter()
        .map(|capsule| {
            let owners = map
                .mappings_for_capsule(&capsule.name)
                .into_iter()
                .map(|mapping| mapping.legacy_owner.clone())
                .collect::<Vec<_>>();
            CapsuleSliceSpec {
                capsule: capsule.name.clone(),
                anchors: capsule.code_owners.clone(),
                status: if owners.is_empty() { "structural" } else { "witnessed" }.to_string(),
                owners,
            }
        })
        .collect::<Vec<_>>();
    let diagram_witnesses = contracts
        .diagrams
        .iter()
        .map(|diagram| DiagramWitnessSpec {
            diagram: diagram.name.clone(),
            probe_commands: probes_for_diagram(&diagram.name, &diagram.verification),
            involved_capsules: diagram.capsules.clone(),
            status: if diagram.verification.is_empty() { "needs-probe" } else { "witnessed" }.to_string(),
        })
        .collect::<Vec<_>>();
    let naturality_witnesses = contracts
        .diagrams
        .iter()
        .flat_map(|diagram| {
            diagram.natural_transformations.iter().map(move |nt| (nt.clone(), diagram.name.clone(), probes_for_diagram(&diagram.name, &diagram.verification)))
        })
        .fold(Vec::<NaturalTransformationWitnessSpec>::new(), |mut acc, (nt, diagram, probes)| {
            if let Some(existing) = acc.iter_mut().find(|item| item.transformation == nt) {
                existing.diagrams.push(diagram);
                existing.probe_commands.extend(probes);
            } else {
                acc.push(NaturalTransformationWitnessSpec { transformation: nt, diagrams: vec![diagram], probe_commands: probes, status: "witnessed".to_string() });
            }
            acc
        });
    let obligations = contracts
        .laws
        .iter()
        .map(|law| RefactorObligationSpec {
            law: law.name.clone(),
            required_diagrams: law.evidence.clone(),
            required_probes: law.evidence.clone(),
            status: if law.evidence.is_empty() { "needs-evidence" } else { "guarded" }.to_string(),
        })
        .collect::<Vec<_>>();
    let validation = validate_witnesses(&anchor_witnesses, &diagram_witnesses, &obligations);
    ArchitectureWitnessBundle { anchor_witnesses, capsule_slices, diagram_witnesses, naturality_witnesses, obligations, validation }
}

fn probes_for_diagram(name: &str, verification: &[String]) -> Vec<String> {
    let mut probes = verification.to_vec();
    match name {
        "table-render-parity" => probes.push("table_adapters.render".to_string()),
        "prompt-argv-gluing" => probes.push("prompt_execution.argv".to_string()),
        "execution-network-reduce" => probes.push("execution_network.threaded_ordered".to_string()),
        "cli-parse-naturality" => probes.push("runtime_switch.extract_architecture_switch_from_argv".to_string()),
        _ => {}
    }
    probes.into_iter().collect::<BTreeSet<_>>().into_iter().collect()
}

fn validate_witnesses(anchors: &[AnchorWitnessSpec], diagrams: &[DiagramWitnessSpec], obligations: &[RefactorObligationSpec]) -> WitnessValidationSpec {
    WitnessValidationSpec {
        status: "ready".to_string(),
        unresolved_anchors: anchors.iter().filter(|anchor| !anchor.resolved).map(|anchor| anchor.owner.clone()).collect(),
        diagrams_without_probe: diagrams.iter().filter(|diagram| diagram.probe_commands.is_empty()).map(|diagram| diagram.diagram.clone()).collect(),
        obligations_without_probe: obligations.iter().filter(|obligation| obligation.required_probes.is_empty()).map(|obligation| obligation.law.clone()).collect(),
        checked_witnesses: anchors.len() + diagrams.len() + obligations.len(),
    }
}
