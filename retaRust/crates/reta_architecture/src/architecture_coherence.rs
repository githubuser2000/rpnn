//! Rust mirror of `architecture_coherence.py`.
//!
//! Coherence checks whether capsule routes, laws and naturality witnesses line
//! up before a migration wave is allowed to commit.

use serde::{Deserialize, Serialize};

use crate::architecture_contracts::ArchitectureContractsBundle;
use crate::architecture_map::ArchitectureMapBundle;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapsuleCoherenceSpec {
    pub capsule: String,
    pub owned_flows: Vec<String>,
    pub owned_contracts: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctorialRouteSpec {
    pub name: String,
    pub source: String,
    pub target: String,
    pub functor: String,
    pub route_kind: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NaturalityCoherenceSpec {
    pub transformation: String,
    pub diagrams: Vec<String>,
    pub protected_capsules: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LawCoherenceSpec {
    pub law: String,
    pub applies_to: Vec<String>,
    pub evidence: Vec<String>,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CoherenceValidationSpec {
    pub status: String,
    pub capsules_without_contract: Vec<String>,
    pub routes_without_functor: Vec<String>,
    pub laws_without_evidence: Vec<String>,
    pub checked_capsules: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureCoherenceSnapshot {
    pub class: String,
    pub capsules: usize,
    pub functorial_routes: usize,
    pub naturality: usize,
    pub laws: usize,
    pub validation: CoherenceValidationSpec,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArchitectureCoherenceBundle {
    pub capsule_coherence: Vec<CapsuleCoherenceSpec>,
    pub functorial_routes: Vec<FunctorialRouteSpec>,
    pub naturality: Vec<NaturalityCoherenceSpec>,
    pub laws: Vec<LawCoherenceSpec>,
    pub validation: CoherenceValidationSpec,
}

impl ArchitectureCoherenceBundle {
    pub fn status_for_capsule(&self, capsule: &str) -> Option<&str> {
        self.capsule_coherence
            .iter()
            .find(|item| item.capsule == capsule)
            .map(|item| item.status.as_str())
    }

    pub fn snapshot(&self) -> ArchitectureCoherenceSnapshot {
        ArchitectureCoherenceSnapshot {
            class: "ArchitectureCoherenceBundle".to_string(),
            capsules: self.capsule_coherence.len(),
            functorial_routes: self.functorial_routes.len(),
            naturality: self.naturality.len(),
            laws: self.laws.len(),
            validation: self.validation.clone(),
        }
    }
}

pub fn bootstrap_architecture_coherence(map: &ArchitectureMapBundle, contracts: &ArchitectureContractsBundle) -> ArchitectureCoherenceBundle {
    let capsule_coherence = map
        .capsules
        .iter()
        .map(|capsule| {
            let owned_flows = map
                .flows
                .iter()
                .filter(|flow| flow.owner.ends_with(".rs") && capsule.code_owners.iter().any(|owner| owner == &flow.owner))
                .map(|flow| flow.morphism.clone())
                .collect::<Vec<_>>();
            let owned_contracts = contracts
                .capsule_contracts
                .iter()
                .filter(|contract| contract.capsule == capsule.name)
                .map(|contract| contract.boundary.clone())
                .collect::<Vec<_>>();
            CapsuleCoherenceSpec {
                capsule: capsule.name.clone(),
                status: if owned_contracts.is_empty() { "structural" } else { "coherent" }.to_string(),
                owned_flows,
                owned_contracts,
            }
        })
        .collect::<Vec<_>>();
    let functorial_routes = map
        .flows
        .iter()
        .map(|flow| FunctorialRouteSpec {
            name: flow.morphism.clone(),
            source: flow.source.clone(),
            target: flow.target.clone(),
            functor: flow.functor.clone(),
            route_kind: if flow.functor.contains("Execution") { "execution" } else { "semantic" }.to_string(),
            status: "coherent".to_string(),
        })
        .collect::<Vec<_>>();
    let naturality = contracts
        .diagrams
        .iter()
        .flat_map(|diagram| {
            diagram.natural_transformations.iter().map(move |nt| NaturalityCoherenceSpec {
                transformation: nt.clone(),
                diagrams: vec![diagram.name.clone()],
                protected_capsules: diagram.capsules.clone(),
                status: if diagram.verification.is_empty() { "needs-witness" } else { "coherent" }.to_string(),
            })
        })
        .collect::<Vec<_>>();
    let laws = contracts
        .laws
        .iter()
        .map(|law| LawCoherenceSpec {
            law: law.name.clone(),
            applies_to: law.applies_to.clone(),
            evidence: law.evidence.clone(),
            status: if law.evidence.is_empty() { "needs-evidence" } else { "coherent" }.to_string(),
        })
        .collect::<Vec<_>>();
    let validation = validate(&capsule_coherence, &functorial_routes, &laws);
    ArchitectureCoherenceBundle { capsule_coherence, functorial_routes, naturality, laws, validation }
}

fn validate(capsules: &[CapsuleCoherenceSpec], routes: &[FunctorialRouteSpec], laws: &[LawCoherenceSpec]) -> CoherenceValidationSpec {
    let capsules_without_contract = capsules.iter().filter(|item| item.owned_contracts.is_empty() && item.capsule != "RetaArchitectureRoot").map(|item| item.capsule.clone()).collect::<Vec<_>>();
    let routes_without_functor = routes.iter().filter(|route| route.functor.is_empty()).map(|route| route.name.clone()).collect::<Vec<_>>();
    let laws_without_evidence = laws.iter().filter(|law| law.evidence.is_empty()).map(|law| law.law.clone()).collect::<Vec<_>>();
    let ok = routes_without_functor.is_empty() && laws_without_evidence.is_empty();
    CoherenceValidationSpec {
        status: if ok { "ready" } else { "needs-attention" }.to_string(),
        capsules_without_contract,
        routes_without_functor,
        laws_without_evidence,
        checked_capsules: capsules.len(),
    }
}
