from __future__ import annotations

"""Stage-32 trace navigation for the categorical Reta architecture.

The earlier stages made topology, presheaves, sheaves, morphisms, universal
constructions, categories, functors, natural transformations, contracts,
witnesses, validation and coherence explicit.  Stage 32 adds a navigable index:
legacy reta owner -> capsule -> category/functor/natural transformation ->
diagram -> witness -> law.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Sequence

from .architecture_coherence import ArchitectureCoherenceBundle
from .architecture_contracts import ArchitectureContractsBundle
from .architecture_map import ArchitectureMapBundle
from .architecture_witnesses import ArchitectureWitnessBundle
from .category_theory import CategoryTheoryBundle


@dataclass(frozen=True)
class TraceHopSpec:
    source: str
    target: str
    relation: str
    categorical_kind: str
    evidence: Sequence[str]

    def snapshot(self) -> dict:
        return {
            "source": self.source,
            "target": self.target,
            "relation": self.relation,
            "categorical_kind": self.categorical_kind,
            "evidence": list(self.evidence),
        }


@dataclass(frozen=True)
class RetaComponentTraceSpec:
    legacy_owner: str
    primary_capsules: Sequence[str]
    categories: Sequence[str]
    functors: Sequence[str]
    natural_transformations: Sequence[str]
    diagrams: Sequence[str]
    witnesses: Sequence[str]
    laws: Sequence[str]
    route: Sequence[TraceHopSpec]
    reading: str

    def snapshot(self) -> dict:
        return {
            "legacy_owner": self.legacy_owner,
            "primary_capsules": list(self.primary_capsules),
            "categories": list(self.categories),
            "functors": list(self.functors),
            "natural_transformations": list(self.natural_transformations),
            "diagrams": list(self.diagrams),
            "witnesses": list(self.witnesses),
            "laws": list(self.laws),
            "route": [item.snapshot() for item in self.route],
            "reading": self.reading,
        }


@dataclass(frozen=True)
class CapsuleTraceSpec:
    capsule: str
    category: str
    functors: Sequence[str]
    natural_transformations: Sequence[str]
    diagrams: Sequence[str]
    laws: Sequence[str]
    witnesses: Sequence[str]
    code_owners: Sequence[str]
    reading: str

    def snapshot(self) -> dict:
        return {
            "capsule": self.capsule,
            "category": self.category,
            "functors": list(self.functors),
            "natural_transformations": list(self.natural_transformations),
            "diagrams": list(self.diagrams),
            "laws": list(self.laws),
            "witnesses": list(self.witnesses),
            "code_owners": list(self.code_owners),
            "reading": self.reading,
        }


@dataclass(frozen=True)
class StageHistoryTraceSpec:
    stage: str
    capsule: str
    moved_to: Sequence[str]
    paradigms: Sequence[str]
    trace_target: str

    def snapshot(self) -> dict:
        return {
            "stage": self.stage,
            "capsule": self.capsule,
            "moved_to": list(self.moved_to),
            "paradigms": list(self.paradigms),
            "trace_target": self.trace_target,
        }


@dataclass(frozen=True)
class TraceValidationSpec:
    status: str
    missing_component_traces: Sequence[str]
    missing_capsule_traces: Sequence[str]
    missing_stage_traces: Sequence[str]
    missing_stage32_documents: Sequence[str]
    unresolved_hops: Sequence[str]
    routes_needing_attention: Sequence[str]
    transformations_needing_attention: Sequence[str]
    route_hop_count: int
    component_trace_count: int

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "missing_component_traces": list(self.missing_component_traces),
            "missing_capsule_traces": list(self.missing_capsule_traces),
            "missing_stage_traces": list(self.missing_stage_traces),
            "missing_stage32_documents": list(self.missing_stage32_documents),
            "unresolved_hops": list(self.unresolved_hops),
            "routes_needing_attention": list(self.routes_needing_attention),
            "transformations_needing_attention": list(self.transformations_needing_attention),
            "route_hop_count": self.route_hop_count,
            "component_trace_count": self.component_trace_count,
        }


@dataclass(frozen=True)
class Stage32ArchitecturePlan:
    planned_after_stage_31: Sequence[str]
    implemented_in_stage_32: Sequence[str]
    inherited_from_previous_stages: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_after_stage_31": list(self.planned_after_stage_31),
            "implemented_in_stage_32": list(self.implemented_in_stage_32),
            "inherited_from_previous_stages": list(self.inherited_from_previous_stages),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class ArchitectureTraceBundle:
    component_traces: Sequence[RetaComponentTraceSpec]
    capsule_traces: Sequence[CapsuleTraceSpec]
    stage_traces: Sequence[StageHistoryTraceSpec]
    validation: TraceValidationSpec
    text_diagram: str
    mermaid_diagram: str
    plan: Stage32ArchitecturePlan

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "stage": 32,
            "paradigm": ["topology", "morphism", "universal_property", "presheaf", "sheaf", "category", "functor", "natural_transformation", "trace"],
            "counts": {
                "component_traces": len(self.component_traces),
                "capsule_traces": len(self.capsule_traces),
                "stage_traces": len(self.stage_traces),
                "route_hops": sum(len(item.route) for item in self.component_traces),
            },
            "component_traces": [item.snapshot() for item in self.component_traces],
            "capsule_traces": [item.snapshot() for item in self.capsule_traces],
            "stage_traces": [item.snapshot() for item in self.stage_traces],
            "validation": self.validation.snapshot(),
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
            "plan": self.plan.snapshot(),
        }


def _category_for_capsule(capsule: str) -> str:
    return {
        "SchemaTopologyCapsule": "OpenRetaContextCategory",
        "LocalSectionCapsule": "LocalSectionCategory",
        "SemanticSheafCapsule": "CanonicalSemanticSheafCategory",
        "InputPromptCapsule": "LocalSectionCategory",
        "WorkflowGluingCapsule": "UniversalConstructionCategory",
        "TableCoreCapsule": "TableSectionCategory",
        "GeneratedRelationCapsule": "GeneratedColumnEndomorphismCategory",
        "OutputRenderingCapsule": "OutputFormatCategory",
        "CompatibilityCapsule": "LegacyFacadeCategory",
        "CategoricalMetaCapsule": "ArchitectureCoherenceCategory",
        "RetaArchitectureRoot": "ArchitectureCoherenceCategory",
    }.get(capsule, "ArchitectureTraceCategory")


def _route(owner: str, capsule: str, functors: Sequence[str], transformations: Sequence[str], diagrams: Sequence[str], witnesses: Sequence[str], laws: Sequence[str]) -> tuple[TraceHopSpec, ...]:
    hops = [
        TraceHopSpec(owner, capsule, "legacy_owner_to_capsule", "morphism", ("architecture-map-json",)),
        TraceHopSpec(capsule, _category_for_capsule(capsule), "capsule_to_category", "category", ("category-theory-json", "architecture-coherence-json")),
    ]
    if functors:
        hops.append(TraceHopSpec(_category_for_capsule(capsule), functors[0], "category_to_functor", "functor", ("category-theory-json",)))
    if transformations:
        hops.append(TraceHopSpec(functors[0] if functors else capsule, transformations[0], "functor_to_natural_transformation", "natural_transformation", ("category-theory-json",)))
    if diagrams:
        hops.append(TraceHopSpec(transformations[0] if transformations else capsule, diagrams[0], "naturality_to_diagram", "commutative_diagram", ("architecture-contracts-json",)))
    if witnesses:
        hops.append(TraceHopSpec(diagrams[0] if diagrams else capsule, witnesses[0], "diagram_to_witness", "witness", ("architecture-witnesses-json",)))
    if laws:
        hops.append(TraceHopSpec(witnesses[0] if witnesses else capsule, laws[0], "witness_to_law", "refactor_law", ("architecture-contracts-json",)))
    return tuple(hops)


def _component_traces(architecture_map: ArchitectureMapBundle, architecture_contracts: ArchitectureContractsBundle, architecture_witnesses: ArchitectureWitnessBundle) -> tuple[RetaComponentTraceSpec, ...]:
    by_capsule = {c.capsule: c for c in architecture_contracts.capsule_contracts}
    law_by_capsule: dict[str, list[str]] = {}
    for law in architecture_contracts.laws:
        for cap in law.applies_to:
            law_by_capsule.setdefault(cap, []).append(law.name)
    witness_by_capsule = {s.capsule: list(s.witness_anchors) for s in architecture_witnesses.capsule_slices}
    result = []
    for item in architecture_map.legacy_mappings:
        capsule = item.new_capsule.split(" + ")[0]
        contract = by_capsule.get(capsule)
        functor = (contract.primary_functor_or_transformation,) if contract else tuple(item.paradigm_role[:1])
        transformations = tuple(x for x in functor if x.endswith("Transformation"))
        functors = tuple(x for x in functor if not x.endswith("Transformation"))
        diagrams = tuple(contract.protected_by if contract else ())
        witnesses = tuple(witness_by_capsule.get(capsule, ())[:4])
        laws = tuple(law_by_capsule.get(capsule, ())[:4])
        route = _route(item.legacy_owner, capsule, functors, transformations, diagrams, witnesses, laws)
        result.append(RetaComponentTraceSpec(item.legacy_owner, (capsule,), (_category_for_capsule(capsule),), functors, transformations, diagrams, witnesses, laws, route, f"{item.legacy_owner} wird über {capsule} in die neue Architekturspur eingeordnet."))
    return tuple(result)


def _capsule_traces(architecture_map: ArchitectureMapBundle, architecture_coherence: ArchitectureCoherenceBundle, architecture_witnesses: ArchitectureWitnessBundle) -> tuple[CapsuleTraceSpec, ...]:
    coh = {item.capsule: item for item in architecture_coherence.capsule_coherence}
    witness = {item.capsule: item.witness_anchors for item in architecture_witnesses.capsule_slices}
    result = []
    for capsule in architecture_map.capsules:
        row = coh.get(capsule.name)
        result.append(CapsuleTraceSpec(
            capsule=capsule.name,
            category=row.category if row else _category_for_capsule(capsule.name),
            functors=tuple(row.functors if row else ()),
            natural_transformations=tuple(row.natural_transformations if row else ()),
            diagrams=tuple(row.diagrams if row else ()),
            laws=tuple(row.laws if row else ()),
            witnesses=tuple(witness.get(capsule.name, ())[:6]),
            code_owners=tuple(capsule.code_owners),
            reading=f"{capsule.name} ist als Kapseltrace von Codebesitz bis Kategorie/Funktor/Diagramm navigierbar.",
        ))
    return tuple(result)


def _stage_traces(architecture_map: ArchitectureMapBundle) -> tuple[StageHistoryTraceSpec, ...]:
    return tuple(StageHistoryTraceSpec(item.stage, item.capsule, tuple(item.moved_to), tuple(item.paradigm_shift.split("; ")), item.moved_to[0] if item.moved_to else item.capsule) for item in architecture_map.stage_steps)


def _validate(repo_root: Path, architecture_map: ArchitectureMapBundle, component_traces: Sequence[RetaComponentTraceSpec], capsule_traces: Sequence[CapsuleTraceSpec], stage_traces: Sequence[StageHistoryTraceSpec], category_theory: CategoryTheoryBundle) -> TraceValidationSpec:
    component_names = {item.legacy_owner for item in architecture_map.legacy_mappings}
    traced_components = {item.legacy_owner for item in component_traces}
    capsule_names = {item.name for item in architecture_map.capsules}
    traced_capsules = {item.capsule for item in capsule_traces}
    stage_names = {item.stage for item in architecture_map.stage_steps}
    traced_stages = {item.stage for item in stage_traces}
    required_docs = ("STAGE32_CHANGES.md", "ARCHITECTURE_TRACES_STAGE32.md", "ARCHITECTURE_BOUNDARIES_STAGE32.md")
    missing_docs = tuple(name for name in required_docs if not (repo_root / name).exists())
    symbolic_targets = set()
    for trace in component_traces:
        symbolic_targets.update(trace.diagrams)
        symbolic_targets.update(trace.witnesses)
        symbolic_targets.update(trace.laws)
    for trace in capsule_traces:
        symbolic_targets.update(trace.diagrams)
        symbolic_targets.update(trace.witnesses)
        symbolic_targets.update(trace.laws)
    known_targets = component_names | traced_components | capsule_names | traced_capsules | stage_names | traced_stages | symbolic_targets | {c.name for c in category_theory.categories} | {f.name for f in category_theory.functors} | {n.name for n in category_theory.natural_transformations}

    def endpoint_known(value: object) -> bool:
        text = str(value)
        return (
            value in known_targets
            or text.endswith((".py", ".md", ".json", "-json", "-md", "Law", "Square"))
            or "*" in text
            or "tests" in text
            or "validation" in text
        )

    unresolved = tuple(sorted(
        f"{hop.source}->{hop.target}"
        for trace in component_traces
        for hop in trace.route
        if not endpoint_known(hop.source) or not endpoint_known(hop.target)
    ))
    attention = tuple(item.legacy_owner for item in component_traces if len(item.route) < 3)
    trans_attention = tuple(item.legacy_owner for item in component_traces if not item.functors and not item.natural_transformations)
    failures = tuple(sorted((component_names - traced_components) | (traced_components - component_names))) + tuple(sorted((capsule_names - traced_capsules) | (traced_capsules - capsule_names))) + tuple(sorted((stage_names - traced_stages) | (traced_stages - stage_names))) + missing_docs + unresolved
    return TraceValidationSpec("passed" if not failures else "attention", tuple(sorted(component_names - traced_components)), tuple(sorted(capsule_names - traced_capsules)), tuple(sorted(stage_names - traced_stages)), missing_docs, unresolved, attention, trans_attention, sum(len(item.route) for item in component_traces), len(component_traces))


def _text_diagram() -> str:
    return """ArchitectureTraceBundle
├─ legacy owner traces
│  └─ reta.py / retaPrompt.py / libs / i18n / csv / reta_architecture
├─ capsule traces
│  └─ capsule → category → functor/transformation → diagram → witness → law
└─ stage history traces
   └─ Stage 1 … Stage 32
"""


def _mermaid_diagram() -> str:
    return """```mermaid
flowchart TD
    Legacy[alte reta-Komponente] --> Capsule[Architektur-Kapsel]
    Capsule --> Category[math Kategorie]
    Category --> Functor[Functor]
    Functor --> NT[natürliche Transformation]
    NT --> Diagram[kommutierendes Diagramm]
    Diagram --> Witness[Witness / Probe / Test]
    Witness --> Law[Refactor-Gesetz]
```
"""


def bootstrap_architecture_traces(*, repo_root: Path, category_theory: CategoryTheoryBundle, architecture_map: ArchitectureMapBundle, architecture_contracts: ArchitectureContractsBundle, architecture_witnesses: ArchitectureWitnessBundle, architecture_coherence: ArchitectureCoherenceBundle) -> ArchitectureTraceBundle:
    component_traces = _component_traces(architecture_map, architecture_contracts, architecture_witnesses)
    capsule_traces = _capsule_traces(architecture_map, architecture_coherence, architecture_witnesses)
    stage_traces = _stage_traces(architecture_map)
    validation = _validate(Path(repo_root), architecture_map, component_traces, capsule_traces, stage_traces, category_theory)
    return ArchitectureTraceBundle(
        component_traces=component_traces,
        capsule_traces=capsule_traces,
        stage_traces=stage_traces,
        validation=validation,
        text_diagram=_text_diagram(),
        mermaid_diagram=_mermaid_diagram(),
        plan=Stage32ArchitecturePlan(
            planned_after_stage_31=("Kohärenzmatrix als navigierbare Trace-Routen ausgeben", "alte reta-Teile stufenweise und kapselweise abbilden"),
            implemented_in_stage_32=("reta_architecture/architecture_traces.py", "architecture-traces-json", "architecture-traces-md"),
            inherited_from_previous_stages=("CategoryTheoryBundle", "ArchitectureMapBundle", "ArchitectureContractsBundle", "ArchitectureWitnessBundle", "ArchitectureCoherenceBundle"),
            behaviour_change="keine beabsichtigte CLI-/Prompt-/Tabellen-/Output-Verhaltensänderung; Stage 32 ist eine Trace-Metaschicht",
        ),
    )
