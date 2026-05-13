from __future__ import annotations

"""Stage-33 impact and migration-gate layer for the categorical Reta architecture.

Stage 32 made traces and Python import boundaries navigable.  Stage 33 turns
those routes into an impact calculus: when a legacy reta component or a new
architecture module is touched, the architecture can show the affected capsule,
category, functor/natural-transformation route, commutative diagrams, refactor
laws, witnesses and regression gates.

The module is deliberately metadata-only.  It does not move runtime behaviour;
it tells later stages which guarded route they must keep commutative before they
move behaviour.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Mapping, Sequence

from .architecture_boundaries import ArchitectureBoundariesBundle
from .architecture_coherence import ArchitectureCoherenceBundle
from .architecture_contracts import ArchitectureContractsBundle
from .architecture_map import ArchitectureMapBundle
from .architecture_traces import ArchitectureTraceBundle
from .architecture_witnesses import ArchitectureWitnessBundle
from .category_theory import CategoryTheoryBundle


@dataclass(frozen=True)
class ImpactSourceSpec:
    """One old/new reta owner whose changes can be traced through the architecture."""

    source: str
    source_kind: str
    capsules: Sequence[str]
    categories: Sequence[str]
    functors: Sequence[str]
    natural_transformations: Sequence[str]
    diagrams: Sequence[str]
    laws: Sequence[str]
    boundary_edges: Sequence[str]
    route_hops: Sequence[str]
    reading: str

    def snapshot(self) -> dict:
        return {
            "source": self.source,
            "source_kind": self.source_kind,
            "capsules": list(self.capsules),
            "categories": list(self.categories),
            "functors": list(self.functors),
            "natural_transformations": list(self.natural_transformations),
            "diagrams": list(self.diagrams),
            "laws": list(self.laws),
            "boundary_edges": list(self.boundary_edges),
            "route_hops": list(self.route_hops),
            "reading": self.reading,
        }


@dataclass(frozen=True)
class ImpactContractSpec:
    """Affected contracts for one impact source."""

    source: str
    affected_capsules: Sequence[str]
    affected_diagrams: Sequence[str]
    affected_laws: Sequence[str]
    affected_natural_transformations: Sequence[str]
    required_gates: Sequence[str]
    required_probes: Sequence[str]
    impact_reading: str

    def snapshot(self) -> dict:
        return {
            "source": self.source,
            "affected_capsules": list(self.affected_capsules),
            "affected_diagrams": list(self.affected_diagrams),
            "affected_laws": list(self.affected_laws),
            "affected_natural_transformations": list(self.affected_natural_transformations),
            "required_gates": list(self.required_gates),
            "required_probes": list(self.required_probes),
            "impact_reading": self.impact_reading,
        }


@dataclass(frozen=True)
class RegressionGateSpec:
    """A probe/test gate required by the impact calculus."""

    name: str
    gate_type: str
    protects: Sequence[str]
    command: str
    required_for: Sequence[str]
    stage_origin: str
    status: str
    reading: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "gate_type": self.gate_type,
            "protects": list(self.protects),
            "command": self.command,
            "required_for": list(self.required_for),
            "stage_origin": self.stage_origin,
            "status": self.status,
            "reading": self.reading,
        }


@dataclass(frozen=True)
class MigrationCandidateSpec:
    """A guarded future extraction/migration candidate, not an executed move."""

    candidate: str
    legacy_owner: str
    current_capsule: str
    target_capsule: str
    move_kind: str
    mathematical_reading: str
    affected_diagrams: Sequence[str]
    gates: Sequence[str]
    next_action: str
    status: str

    def snapshot(self) -> dict:
        return {
            "candidate": self.candidate,
            "legacy_owner": self.legacy_owner,
            "current_capsule": self.current_capsule,
            "target_capsule": self.target_capsule,
            "move_kind": self.move_kind,
            "mathematical_reading": self.mathematical_reading,
            "affected_diagrams": list(self.affected_diagrams),
            "gates": list(self.gates),
            "next_action": self.next_action,
            "status": self.status,
        }


@dataclass(frozen=True)
class ImpactCheckSpec:
    name: str
    status: str
    failed_items: Sequence[str]
    checked_count: int
    reading: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "status": self.status,
            "failed_items": list(self.failed_items),
            "checked_count": self.checked_count,
            "reading": self.reading,
        }


@dataclass(frozen=True)
class ImpactValidationSpec:
    status: str
    missing_sources: Sequence[str]
    sources_without_contracts: Sequence[str]
    candidates_without_gates: Sequence[str]
    unknown_capsules: Sequence[str]
    uncovered_natural_transformations: Sequence[str]
    checks: Sequence[ImpactCheckSpec]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "missing_sources": list(self.missing_sources),
            "sources_without_contracts": list(self.sources_without_contracts),
            "candidates_without_gates": list(self.candidates_without_gates),
            "unknown_capsules": list(self.unknown_capsules),
            "uncovered_natural_transformations": list(self.uncovered_natural_transformations),
            "checks": [item.snapshot() for item in self.checks],
        }


@dataclass(frozen=True)
class Stage33ArchitecturePlan:
    planned_after_stage_32: Sequence[str]
    implemented_in_stage_33: Sequence[str]
    inherited_from_previous_stages: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_after_stage_32": list(self.planned_after_stage_32),
            "implemented_in_stage_33": list(self.implemented_in_stage_33),
            "inherited_from_previous_stages": list(self.inherited_from_previous_stages),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class ArchitectureImpactBundle:
    impact_sources: Sequence[ImpactSourceSpec]
    impact_contracts: Sequence[ImpactContractSpec]
    regression_gates: Sequence[RegressionGateSpec]
    migration_candidates: Sequence[MigrationCandidateSpec]
    validation: ImpactValidationSpec
    text_diagram: str
    mermaid_diagram: str
    plan: Stage33ArchitecturePlan

    def source_named(self, source: str) -> ImpactSourceSpec:
        for item in self.impact_sources:
            if item.source == source:
                return item
        raise KeyError(f"Unknown impact source: {source}")

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "stage": 33,
            "purpose": "Impact- und Migration-Gate-Schicht: alte/neue reta-Owner werden auf Kapseln, Diagramme, Gesetze, Witnesses, Boundary-Kanten und Regression-Gates projiziert.",
            "paradigm": [
                "topology",
                "morphism",
                "universal_property",
                "presheaf",
                "sheaf",
                "category",
                "functor",
                "natural_transformation",
                "impact",
                "migration_gate",
            ],
            "counts": {
                "impact_sources": len(self.impact_sources),
                "impact_contracts": len(self.impact_contracts),
                "regression_gates": len(self.regression_gates),
                "migration_candidates": len(self.migration_candidates),
                "checks": len(self.validation.checks),
            },
            "impact_sources": [item.snapshot() for item in self.impact_sources],
            "impact_contracts": [item.snapshot() for item in self.impact_contracts],
            "regression_gates": [item.snapshot() for item in self.regression_gates],
            "migration_candidates": [item.snapshot() for item in self.migration_candidates],
            "validation": self.validation.snapshot(),
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
            "plan": self.plan.snapshot(),
        }


def _dedupe(items: Sequence[str]) -> tuple[str, ...]:
    return tuple(dict.fromkeys(item for item in items if item))


def _source_kind(source: str) -> str:
    if source.startswith("reta_architecture/"):
        return "architecture module"
    if source.startswith("libs/") or source in {"reta.py", "retaPrompt.py"}:
        return "legacy compatibility surface"
    if source.startswith("csv/") or "*.csv" in source:
        return "local section data"
    return "repository owner"


def _boundary_edges_for(source: str, boundaries: ArchitectureBoundariesBundle) -> tuple[str, ...]:
    edges: list[str] = []
    for edge in boundaries.import_edges:
        if edge.importer == source or edge.imported == source:
            edges.append(f"{edge.importer}->{edge.imported}:{edge.importer_capsule}->{edge.imported_capsule}")
    return tuple(edges[:8])


def _impact_sources(traces: ArchitectureTraceBundle, boundaries: ArchitectureBoundariesBundle) -> tuple[ImpactSourceSpec, ...]:
    items: list[ImpactSourceSpec] = []
    for trace in traces.component_traces:
        route_hops = tuple(f"{hop.source}->{hop.target}" for hop in trace.route)
        items.append(
            ImpactSourceSpec(
                source=trace.legacy_owner,
                source_kind=_source_kind(trace.legacy_owner),
                capsules=tuple(trace.primary_capsules),
                categories=tuple(trace.categories),
                functors=tuple(trace.functors),
                natural_transformations=tuple(trace.natural_transformations),
                diagrams=tuple(trace.diagrams),
                laws=tuple(trace.laws),
                boundary_edges=_boundary_edges_for(trace.legacy_owner, boundaries),
                route_hops=route_hops,
                reading=f"{trace.legacy_owner} wird als Impact-Quelle über Trace-Route und Boundary-Graph auf seine Refactor-Gates abgebildet.",
            )
        )
    return tuple(items)


def _base_gates() -> tuple[RegressionGateSpec, ...]:
    return (
        RegressionGateSpec("CategoryTheoryProbeGate", "probe", ("categories", "functors", "natural transformations"), "python -B -S reta_architecture_probe_py.py category-theory-json", ("categorical references",), "Stage 27", "required", "Kategorien, Funktoren und natürliche Transformationen müssen referenziell geschlossen bleiben."),
        RegressionGateSpec("ArchitectureMapProbeGate", "probe", ("capsules", "flows", "stage map"), "python -B -S reta_architecture_probe_py.py architecture-map-json", ("capsule ownership",), "Stage 28", "required", "Kapselkarte und Datenflüsse müssen den geänderten Owner weiterhin enthalten."),
        RegressionGateSpec("ArchitectureContractsProbeGate", "probe", ("commutative diagrams", "laws"), "python -B -S reta_architecture_probe_py.py architecture-contracts-json", ("diagram contracts",), "Stage 29", "required", "Kommutierende Diagramme und Refactor-Gesetze müssen gültig bleiben."),
        RegressionGateSpec("ArchitectureWitnessProbeGate", "probe", ("witnesses", "obligations"), "python -B -S reta_architecture_probe_py.py architecture-witnesses-json", ("repository anchors",), "Stage 30", "required", "Jeder Vertrag braucht einen Datei-/Probe-/Test-Witness."),
        RegressionGateSpec("ArchitectureCoherenceProbeGate", "probe", ("coherence matrix",), "python -B -S reta_architecture_probe_py.py architecture-coherence-json", ("cross-layer coherence",), "Stage 31", "required", "Kapsel, Kategorie, Funktor, Transformation, Diagramm und Witness müssen zusammenpassen."),
        RegressionGateSpec("ArchitectureTraceProbeGate", "probe", ("trace routes",), "python -B -S reta_architecture_probe_py.py architecture-traces-json", ("component traces",), "Stage 32", "required", "Die alte Komponente muss weiter navigierbar bleiben."),
        RegressionGateSpec("ArchitectureBoundaryProbeGate", "probe", ("module ownership", "import boundaries"), "python -B -S reta_architecture_probe_py.py architecture-boundaries-json", ("capsule boundaries",), "Stage 32", "required", "Importkanten müssen als Kapselgrenzen sichtbar bleiben."),
        RegressionGateSpec("ArchitectureImpactSelfGate", "probe", ("impact contracts", "migration candidates"), "python -B -S reta_architecture_probe_py.py architecture-impact-json", ("Stage 33 impact calculus",), "Stage 33", "required", "Die Impact-Schicht muss sich selbst vollständig aus Trace und Boundary rekonstruieren."),
        RegressionGateSpec("ArchitectureRegressionGate", "unittest", ("architecture regression tests",), "python -m unittest tests.test_architecture_refactor -v", ("all staged architecture metadata",), "Stages 1-33", "required", "Die Architektur-Regressionssuite muss grün bleiben."),
        RegressionGateSpec("CommandParityGate", "unittest", ("legacy CLI parity",), "python -m unittest tests.test_command_parity -v", ("CompatibilityCapsule", "OutputRenderingCapsule"), "Stages 1-33", "required", "Beobachtbare Legacy-Ausgabe muss gegen die alte reta-Referenz paritätisch bleiben."),
    )


def _diagram_probe_map(witnesses: ArchitectureWitnessBundle) -> Mapping[str, tuple[str, ...]]:
    return {item.diagram: tuple(item.probe_commands) for item in witnesses.diagram_witnesses}


def _gate_names_for(source: ImpactSourceSpec, diagram_probe_map: Mapping[str, tuple[str, ...]]) -> tuple[str, ...]:
    gates = [
        "CategoryTheoryProbeGate",
        "ArchitectureMapProbeGate",
        "ArchitectureContractsProbeGate",
        "ArchitectureWitnessProbeGate",
        "ArchitectureCoherenceProbeGate",
        "ArchitectureTraceProbeGate",
        "ArchitectureBoundaryProbeGate",
        "ArchitectureImpactSelfGate",
        "ArchitectureRegressionGate",
    ]
    if "CompatibilityCapsule" in source.capsules or source.source.startswith(("reta.py", "retaPrompt.py", "libs/")):
        gates.append("CommandParityGate")
    if any("Output" in diagram or "Legacy" in diagram for diagram in source.diagrams):
        gates.append("CommandParityGate")
    return _dedupe(gates)


def _impact_contracts(sources: Sequence[ImpactSourceSpec], witnesses: ArchitectureWitnessBundle) -> tuple[ImpactContractSpec, ...]:
    probes_by_diagram = _diagram_probe_map(witnesses)
    items: list[ImpactContractSpec] = []
    for source in sources:
        probes: list[str] = []
        for diagram in source.diagrams:
            probes.extend(probes_by_diagram.get(diagram, ()))
        probes.extend((
            "python -B -S reta_architecture_probe_py.py architecture-impact-json",
            "python -B -S reta_architecture_probe_py.py architecture-validation-json",
        ))
        items.append(
            ImpactContractSpec(
                source=source.source,
                affected_capsules=tuple(source.capsules),
                affected_diagrams=tuple(source.diagrams),
                affected_laws=tuple(source.laws),
                affected_natural_transformations=tuple(source.natural_transformations),
                required_gates=_gate_names_for(source, probes_by_diagram),
                required_probes=_dedupe(probes),
                impact_reading=f"Änderungen an {source.source} müssen die betroffenen Diagramme {', '.join(source.diagrams) or 'über Kapselvertrag'} und Gates passieren.",
            )
        )
    return tuple(items)


def _candidate_status(owner: str) -> str:
    if owner.startswith("reta_architecture/"):
        return "already_architecture_owned"
    if owner.startswith("libs/") or owner in {"reta.py", "retaPrompt.py"}:
        return "guarded_legacy_surface"
    return "guarded_data_or_document_owner"


def _migration_candidates(architecture_map: ArchitectureMapBundle, contracts: Sequence[ImpactContractSpec]) -> tuple[MigrationCandidateSpec, ...]:
    by_source = {item.source: item for item in contracts}
    items: list[MigrationCandidateSpec] = []
    for mapping in architecture_map.legacy_mappings:
        contract = by_source.get(mapping.legacy_owner)
        gates = tuple(contract.required_gates if contract else ("ArchitectureImpactSelfGate",))
        diagrams = tuple(contract.affected_diagrams if contract else ())
        is_legacy_surface = mapping.legacy_owner in {"reta.py", "retaPrompt.py"} or mapping.legacy_owner.startswith("libs/")
        current_capsule = "CompatibilityCapsule" if is_legacy_surface else mapping.new_capsule
        status = _candidate_status(mapping.legacy_owner)
        next_action = (
            "nur mit CommandParityGate und Output-/Legacy-Diagrammen weiter extrahieren"
            if status == "guarded_legacy_surface"
            else "als Architektur-Owner behalten und bei Änderungen Impact-Gates prüfen"
        )
        items.append(
            MigrationCandidateSpec(
                candidate=f"Stage33Guard::{mapping.legacy_owner}",
                legacy_owner=mapping.legacy_owner,
                current_capsule=current_capsule,
                target_capsule=mapping.new_capsule,
                move_kind="guarded extraction" if status == "guarded_legacy_surface" else "guarded maintenance",
                mathematical_reading=f"{mapping.legacy_owner} wird als Morphismus/Objekt über {mapping.new_capsule} und die zugehörigen natürlichen Transformationen kontrolliert.",
                affected_diagrams=diagrams,
                gates=gates,
                next_action=next_action,
                status=status,
            )
        )
    return tuple(items)


def _validate(
    architecture_map: ArchitectureMapBundle,
    category_theory: CategoryTheoryBundle,
    sources: Sequence[ImpactSourceSpec],
    contracts: Sequence[ImpactContractSpec],
    gates: Sequence[RegressionGateSpec],
    candidates: Sequence[MigrationCandidateSpec],
) -> ImpactValidationSpec:
    source_names = {item.source for item in sources}
    mapping_names = {item.legacy_owner for item in architecture_map.legacy_mappings}
    contract_names = {item.source for item in contracts}
    gate_names = {item.name for item in gates}
    capsule_names = {item.name for item in architecture_map.capsules}
    transformation_names = {item.name for item in category_theory.natural_transformations}
    used_transformations = {name for item in sources for name in item.natural_transformations}

    missing_sources = tuple(sorted(mapping_names - source_names))
    sources_without_contracts = tuple(sorted(source_names - contract_names))
    candidates_without_gates = tuple(sorted(item.candidate for item in candidates if not item.gates or any(gate not in gate_names for gate in item.gates)))
    unknown_capsules = tuple(sorted({capsule for item in sources for capsule in item.capsules if capsule not in capsule_names}))
    uncovered_transformations = tuple(sorted(name for name in used_transformations if name not in transformation_names))

    checks = (
        ImpactCheckSpec("ImpactSourceCoverageCheck", "passed" if not missing_sources else "failed", missing_sources, len(mapping_names), "Jede Legacy-/Architektur-Mapping-Zeile braucht eine Impact-Quelle."),
        ImpactCheckSpec("ImpactContractCoverageCheck", "passed" if not sources_without_contracts else "failed", sources_without_contracts, len(source_names), "Jede Impact-Quelle braucht einen betroffenen Vertrags-/Gate-Satz."),
        ImpactCheckSpec("RegressionGateCoverageCheck", "passed" if not candidates_without_gates else "failed", candidates_without_gates, len(candidates), "Jeder Migrationskandidat braucht konkrete Gates."),
        ImpactCheckSpec("ImpactCapsuleReferenceCheck", "passed" if not unknown_capsules else "failed", unknown_capsules, sum(len(item.capsules) for item in sources), "Impact-Kapseln müssen in der Architekturkarte existieren."),
        ImpactCheckSpec("ImpactNaturalityReferenceCheck", "passed" if not uncovered_transformations else "failed", uncovered_transformations, len(used_transformations), "Impact-Transformationen müssen in CategoryTheoryBundle existieren."),
    )
    failures = missing_sources + sources_without_contracts + candidates_without_gates + unknown_capsules + uncovered_transformations
    return ImpactValidationSpec(
        status="passed" if not failures else "failed",
        missing_sources=missing_sources,
        sources_without_contracts=sources_without_contracts,
        candidates_without_gates=candidates_without_gates,
        unknown_capsules=unknown_capsules,
        uncovered_natural_transformations=uncovered_transformations,
        checks=checks,
    )


def _text_diagram() -> str:
    return """ArchitectureImpactBundle
├─ ImpactSourceSpec
│  └─ old/new reta owner → capsule → category/functor/natural transformation
├─ ImpactContractSpec
│  └─ source → affected diagrams/laws/transformations → required probes
├─ RegressionGateSpec
│  └─ category/map/contract/witness/coherence/trace/boundary/impact/parity gates
├─ MigrationCandidateSpec
│  └─ future extraction candidates are guarded, not silently moved
└─ ImpactValidationSpec
   └─ Stage-33 coverage over sources, contracts, gates, capsules and naturality
"""


def _mermaid_diagram() -> str:
    return """```mermaid
flowchart TD
    Trace[ArchitectureTraceBundle<br/>old owner routes] --> Impact[ArchitectureImpactBundle]
    Boundary[ArchitectureBoundariesBundle<br/>module/import edges] --> Impact
    Contracts[ArchitectureContractsBundle<br/>diagrams + laws] --> Impact
    Witness[ArchitectureWitnessBundle<br/>probes + obligations] --> Impact
    Impact --> Sources[Impact sources]
    Impact --> Affected[Affected contracts]
    Impact --> Gates[Regression gates]
    Impact --> Candidates[Migration candidates]
    Gates --> Future[Future stage<br/>move only when gates pass]
```
"""


def _plan() -> Stage33ArchitecturePlan:
    return Stage33ArchitecturePlan(
        planned_after_stage_32=(
            "Trace- und Boundary-Schichten nicht nur anzeigen, sondern als Impact-Route für spätere Umbauten nutzbar machen.",
            "Für jede alte reta-Komponente zeigen, welche Kapseln, Diagramme, Gesetze, natürlichen Transformationen, Probes und Tests berührt werden.",
            "Migrationen als guarded candidates modellieren, nicht als stillschweigende Verhaltensänderung.",
        ),
        implemented_in_stage_33=(
            "reta_architecture/architecture_impact.py",
            "architecture-impact-json and architecture-impact-md probe commands",
            "ArchitectureMap stage step and CategoricalMetaCapsule containment for ArchitectureImpactBundle",
            "CategoryTheory impact category/functors/natural transformations",
            "ArchitectureContracts impact diagrams and impact-gate law",
        ),
        inherited_from_previous_stages=(
            "Stage 27 CategoryTheoryBundle",
            "Stage 28 ArchitectureMapBundle",
            "Stage 29 ArchitectureContractsBundle",
            "Stage 30 ArchitectureWitnessBundle",
            "Stage 31 ArchitectureValidation/Coherence bundles",
            "Stage 32 ArchitectureTrace/Boundary bundles",
        ),
        behaviour_change="keine beabsichtigte Laufzeit-/CLI-/Prompt-/Tabellen-/Output-Verhaltensänderung; Stage 33 ist eine Impact- und Migration-Gate-Metaschicht",
    )


def bootstrap_architecture_impact(
    *,
    repo_root: Path,
    category_theory: CategoryTheoryBundle,
    architecture_map: ArchitectureMapBundle,
    architecture_contracts: ArchitectureContractsBundle,
    architecture_witnesses: ArchitectureWitnessBundle,
    architecture_coherence: ArchitectureCoherenceBundle,
    architecture_traces: ArchitectureTraceBundle,
    architecture_boundaries: ArchitectureBoundariesBundle,
) -> ArchitectureImpactBundle:
    # repo_root, contracts and coherence are intentionally part of the signature:
    # they are the Stage-33 categorical inputs even when the current metadata can
    # be derived from traces/witnesses/boundaries alone.
    del repo_root, architecture_contracts, architecture_coherence
    sources = _impact_sources(architecture_traces, architecture_boundaries)
    gates = _base_gates()
    contracts = _impact_contracts(sources, architecture_witnesses)
    candidates = _migration_candidates(architecture_map, contracts)
    validation = _validate(architecture_map, category_theory, sources, contracts, gates, candidates)
    return ArchitectureImpactBundle(
        impact_sources=sources,
        impact_contracts=contracts,
        regression_gates=gates,
        migration_candidates=candidates,
        validation=validation,
        text_diagram=_text_diagram(),
        mermaid_diagram=_mermaid_diagram(),
        plan=_plan(),
    )
