from __future__ import annotations

"""Stage-34 guarded migration plan for the categorical Reta architecture.

Stage 33 turns trace and boundary information into impact sources, affected
contracts and regression gates.  Stage 34 takes the next step: it does not move
runtime behaviour yet, but it orders those guarded candidates into migration
waves and concrete steps.  A future extraction is only acceptable when its
candidate, gates, diagrams, laws and natural transformations still commute.

Mathematical reading:

* impact sources are objects in the ArchitectureImpactCategory;
* guarded migration steps are objects in the ArchitectureMigrationCategory;
* the plan is a functorial projection from impact to ordered migration waves;
* gate binding and wave ordering are naturality requirements for later code
  movement across compatibility facades.
"""

from dataclasses import dataclass
from typing import Mapping, Sequence

from .architecture_contracts import ArchitectureContractsBundle
from .architecture_impact import ArchitectureImpactBundle, MigrationCandidateSpec, RegressionGateSpec
from .architecture_map import ArchitectureMapBundle
from .category_theory import CategoryTheoryBundle


@dataclass(frozen=True)
class MigrationWaveSpec:
    """One ordered capsule wave for later guarded runtime extraction."""

    wave_id: str
    order: int
    name: str
    focus: str
    owner_capsules: Sequence[str]
    candidates: Sequence[str]
    universal_property: str
    functorial_route: Sequence[str]
    naturality_requirement: str
    required_gates: Sequence[str]
    status: str

    def snapshot(self) -> dict:
        return {
            "wave_id": self.wave_id,
            "order": self.order,
            "name": self.name,
            "focus": self.focus,
            "owner_capsules": list(self.owner_capsules),
            "candidates": list(self.candidates),
            "universal_property": self.universal_property,
            "functorial_route": list(self.functorial_route),
            "naturality_requirement": self.naturality_requirement,
            "required_gates": list(self.required_gates),
            "status": self.status,
        }


@dataclass(frozen=True)
class MigrationStepSpec:
    """Concrete migration step derived from one Stage-33 guarded candidate."""

    step_id: str
    wave_id: str
    candidate: str
    legacy_owner: str
    current_capsule: str
    target_capsule: str
    action_type: str
    target_owner: str
    category: str
    functors: Sequence[str]
    natural_transformations: Sequence[str]
    diagrams: Sequence[str]
    laws: Sequence[str]
    gates: Sequence[str]
    prerequisites: Sequence[str]
    observable_invariant: str
    status: str

    def snapshot(self) -> dict:
        return {
            "step_id": self.step_id,
            "wave_id": self.wave_id,
            "candidate": self.candidate,
            "legacy_owner": self.legacy_owner,
            "current_capsule": self.current_capsule,
            "target_capsule": self.target_capsule,
            "action_type": self.action_type,
            "target_owner": self.target_owner,
            "category": self.category,
            "functors": list(self.functors),
            "natural_transformations": list(self.natural_transformations),
            "diagrams": list(self.diagrams),
            "laws": list(self.laws),
            "gates": list(self.gates),
            "prerequisites": list(self.prerequisites),
            "observable_invariant": self.observable_invariant,
            "status": self.status,
        }


@dataclass(frozen=True)
class MigrationGateBindingSpec:
    """Binding between one migration step and the concrete gates that must hold."""

    step_id: str
    candidate: str
    gates: Sequence[str]
    gate_commands: Mapping[str, str]
    command_parity_required: bool
    bound_diagrams: Sequence[str]
    missing_gates: Sequence[str]
    status: str
    reading: str

    def snapshot(self) -> dict:
        return {
            "step_id": self.step_id,
            "candidate": self.candidate,
            "gates": list(self.gates),
            "gate_commands": dict(self.gate_commands),
            "command_parity_required": self.command_parity_required,
            "bound_diagrams": list(self.bound_diagrams),
            "missing_gates": list(self.missing_gates),
            "status": self.status,
            "reading": self.reading,
        }


@dataclass(frozen=True)
class MigrationInvariantSpec:
    """Law-like invariant that a whole migration wave must keep true."""

    name: str
    wave_id: str
    applies_to: Sequence[str]
    diagrams: Sequence[str]
    laws: Sequence[str]
    natural_transformations: Sequence[str]
    required_gates: Sequence[str]
    proof_obligation: str
    status: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "wave_id": self.wave_id,
            "applies_to": list(self.applies_to),
            "diagrams": list(self.diagrams),
            "laws": list(self.laws),
            "natural_transformations": list(self.natural_transformations),
            "required_gates": list(self.required_gates),
            "proof_obligation": self.proof_obligation,
            "status": self.status,
        }


@dataclass(frozen=True)
class MigrationCheckSpec:
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
class MigrationValidationSpec:
    status: str
    missing_candidates: Sequence[str]
    steps_without_gate_binding: Sequence[str]
    unknown_gates: Sequence[str]
    unknown_diagrams: Sequence[str]
    unknown_natural_transformations: Sequence[str]
    unordered_waves: Sequence[str]
    empty_waves: Sequence[str]
    checks: Sequence[MigrationCheckSpec]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "missing_candidates": list(self.missing_candidates),
            "steps_without_gate_binding": list(self.steps_without_gate_binding),
            "unknown_gates": list(self.unknown_gates),
            "unknown_diagrams": list(self.unknown_diagrams),
            "unknown_natural_transformations": list(self.unknown_natural_transformations),
            "unordered_waves": list(self.unordered_waves),
            "empty_waves": list(self.empty_waves),
            "checks": [item.snapshot() for item in self.checks],
        }


@dataclass(frozen=True)
class Stage34ArchitecturePlan:
    planned_after_stage_33: Sequence[str]
    implemented_in_stage_34: Sequence[str]
    inherited_from_previous_stages: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_after_stage_33": list(self.planned_after_stage_33),
            "implemented_in_stage_34": list(self.implemented_in_stage_34),
            "inherited_from_previous_stages": list(self.inherited_from_previous_stages),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class ArchitectureMigrationBundle:
    """Stage-34 ordered migration plan over Stage-33 impact candidates."""

    waves: Sequence[MigrationWaveSpec]
    steps: Sequence[MigrationStepSpec]
    gate_bindings: Sequence[MigrationGateBindingSpec]
    invariants: Sequence[MigrationInvariantSpec]
    validation: MigrationValidationSpec
    text_diagram: str
    mermaid_diagram: str
    plan: Stage34ArchitecturePlan

    def wave_named(self, wave_id: str) -> MigrationWaveSpec:
        for wave in self.waves:
            if wave.wave_id == wave_id:
                return wave
        raise KeyError(f"Unknown migration wave: {wave_id}")

    def steps_for_owner(self, legacy_owner: str) -> tuple[MigrationStepSpec, ...]:
        return tuple(step for step in self.steps if step.legacy_owner == legacy_owner)

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "stage": 34,
            "purpose": "Guarded migration plan: Stage-33 impact candidates are ordered into capsule waves, steps, gate bindings and invariants before any future runtime move.",
            "paradigm": [
                "topology",
                "morphism",
                "universal_property",
                "presheaf",
                "sheaf",
                "category",
                "functor",
                "natural_transformation",
                "migration_plan",
                "migration_wave",
                "gate_binding",
            ],
            "counts": {
                "waves": len(self.waves),
                "steps": len(self.steps),
                "gate_bindings": len(self.gate_bindings),
                "invariants": len(self.invariants),
                "checks": len(self.validation.checks),
            },
            "waves": [item.snapshot() for item in self.waves],
            "steps": [item.snapshot() for item in self.steps],
            "gate_bindings": [item.snapshot() for item in self.gate_bindings],
            "invariants": [item.snapshot() for item in self.invariants],
            "validation": self.validation.snapshot(),
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
            "plan": self.plan.snapshot(),
        }


def _dedupe(items: Sequence[str]) -> tuple[str, ...]:
    return tuple(dict.fromkeys(item for item in items if item))


def _wave_id_for(candidate: MigrationCandidateSpec) -> str:
    owner = candidate.legacy_owner
    target = candidate.target_capsule
    if owner.startswith("reta_architecture/"):
        return "M0"
    if owner.startswith("i18n/") or owner.startswith("csv/") or "*.csv" in owner or "readme" in owner or owner.endswith("lib4tables_Enum.py"):
        return "M1"
    if owner in {"retaPrompt.py", "libs/center.py", "libs/LibRetaPrompt.py", "libs/nestedAlx.py"} or "InputPromptCapsule" in target:
        return "M2"
    if owner == "reta.py" or "WorkflowGluingCapsule" in target:
        return "M3"
    if owner.endswith("lib4tables.py") or "OutputRenderingCapsule" in target:
        return "M6"
    if owner.endswith("lib4tables_prepare.py") or owner.endswith("tableHandling.py") or "TableCoreCapsule" in target:
        return "M4"
    if owner.endswith("lib4tables_concat.py") or "GeneratedRelationCapsule" in target:
        return "M5"
    return "M0"


def _wave_catalog() -> Mapping[str, tuple[int, str, str, tuple[str, ...], str, tuple[str, ...], str, tuple[str, ...]]]:
    return {
        "M0": (0, "Meta-Kohärenz und Planungswelle", "Stage-27 bis Stage-34 Metaebenen selbst stabil halten", ("CategoricalMetaCapsule",), "The meta stack is the universal audit object for later moves.", ("ImpactToMigrationPlanFunctor", "MigrationWaveOrderingFunctor"), "Impact-derived and gate-derived planning must name the same allowed move.", ("CategoryTheoryProbeGate", "ArchitectureMapProbeGate", "ArchitectureContractsProbeGate", "ArchitectureMigrationSelfGate")),
        "M1": (1, "Topologie-/Prägarben-Datenwelle", "i18n, CSV und Dokumentsektionen nur entlang Kontext- und Restriktionsgesetzen verändern", ("SchemaTopologyCapsule", "LocalSectionCapsule", "SemanticSheafCapsule"), "Local sections glue to canonical semantics after context restriction.", ("SchemaToTopologyFunctor", "LocalDataPresheafFunctor", "ImpactToMigrationPlanFunctor"), "Restrict-then-glue and glue-then-restrict remain equivalent.", ("CategoryTheoryProbeGate", "ArchitectureMapProbeGate", "ArchitectureMigrationSelfGate")),
        "M2": (2, "Prompt-/Input-Morphismuswelle", "Prompt- und CLI-Rohtext weiter aus Legacy-Fassaden lösen", ("InputPromptCapsule", "SemanticSheafCapsule"), "Raw command sections canonically map into semantic sheaves.", ("RawCommandPresheafFunctor", "ImpactToMigrationPlanFunctor"), "Raw-to-canonical naturality must commute for each prompt context.", ("ArchitectureContractsProbeGate", "ArchitectureTraceProbeGate", "CommandParityGate", "ArchitectureMigrationSelfGate")),
        "M3": (3, "Workflow-/Universal-Gluing-Welle", "reta.py als Workflow-Fassade auf universelle Konstruktionen reduzieren", ("WorkflowGluingCapsule", "CompatibilityCapsule"), "The table workflow is the universal glued object from semantic sections and column selection.", ("TableGenerationGluingFunctor", "ImpactToMigrationPlanFunctor"), "Workflow gluing before/after compatibility projection yields the same table section.", ("ArchitectureContractsProbeGate", "ArchitectureImpactSelfGate", "CommandParityGate", "ArchitectureMigrationSelfGate")),
        "M4": (4, "Table-Core-State-Welle", "Table runtime/state/prepare ownership enger in TableCoreCapsule führen", ("TableCoreCapsule",), "The global table section and explicit state sections are two projections of the same object.", ("MutableTableRuntimeFunctor", "ExplicitTableStateFunctor", "ImpactToMigrationPlanFunctor"), "Runtime-state projection remains natural under prepare/filter/wrap morphisms.", ("ArchitectureWitnessProbeGate", "ArchitectureImpactSelfGate", "CommandParityGate", "ArchitectureMigrationSelfGate")),
        "M5": (5, "Generated-Relation-Welle", "Generated/meta/concat/combi Operationen als Endofunktoren konsolidieren", ("GeneratedRelationCapsule",), "Generated relations are endomorphisms of table sections with sheaf-state sync.", ("GeneratedColumnEndofunctorFamily", "ImpactToMigrationPlanFunctor"), "Generated-column state sync remains independent of legacy entry path.", ("ArchitectureContractsProbeGate", "ArchitectureImpactSelfGate", "CommandParityGate", "ArchitectureMigrationSelfGate")),
        "M6": (6, "Output-Rendering- und Paritätswelle", "Renderer-Funktoren und Normalisierung weiter von Legacy-Ausgabe entkoppeln", ("OutputRenderingCapsule", "CompatibilityCapsule"), "Rendered outputs are functorial images of table sections, compared through normalization.", ("OutputRenderingFunctorFamily", "NormalizedOutputFunctor", "ImpactToMigrationPlanFunctor"), "Render-then-normalize and legacy-normalize paths stay observationally equivalent.", ("ArchitectureContractsProbeGate", "ArchitectureImpactSelfGate", "CommandParityGate", "ArchitectureMigrationSelfGate")),
    }


def _category_for_target(target_capsule: str) -> str:
    if "InputPromptCapsule" in target_capsule:
        return "LocalSectionCategory"
    if "WorkflowGluingCapsule" in target_capsule:
        return "UniversalConstructionCategory"
    if "TableCoreCapsule" in target_capsule:
        return "TableSectionCategory"
    if "GeneratedRelationCapsule" in target_capsule:
        return "GeneratedColumnEndomorphismCategory"
    if "OutputRenderingCapsule" in target_capsule:
        return "OutputFormatCategory"
    if "SchemaTopologyCapsule" in target_capsule:
        return "OpenRetaContextCategory"
    if "LocalSectionCapsule" in target_capsule:
        return "LocalSectionCategory"
    if "SemanticSheafCapsule" in target_capsule:
        return "CanonicalSemanticSheafCategory"
    if "CompatibilityCapsule" in target_capsule:
        return "LegacyFacadeCategory"
    return "ArchitectureMigrationCategory"


def _target_owner_for(candidate: MigrationCandidateSpec) -> str:
    target = candidate.target_capsule
    if "InputPromptCapsule" in target:
        return "reta_architecture/prompt_runtime.py + prompt_language/session/execution/preparation/interaction"
    if "WorkflowGluingCapsule" in target:
        return "reta_architecture/program_workflow.py + table_generation.py + column_selection.py"
    if "TableCoreCapsule" in target:
        return "reta_architecture/table_runtime.py + table_state.py + table_preparation.py"
    if "GeneratedRelationCapsule" in target:
        return "reta_architecture/generated_columns.py + meta_columns.py + concat_csv.py + combi_join.py"
    if "OutputRenderingCapsule" in target:
        return "reta_architecture/output_syntax.py + output_semantics.py + table_output.py"
    if "SchemaTopologyCapsule" in target:
        return "reta_architecture/schema.py + topology.py + i18n split modules"
    if "LocalSectionCapsule" in target:
        return "reta_architecture/presheaves.py + csv/doc local sections"
    if "CategoricalMetaCapsule" in target:
        return "reta_architecture/category_theory.py + architecture_* meta bundles"
    return target


def _functors_for(candidate: MigrationCandidateSpec) -> tuple[str, ...]:
    target = candidate.target_capsule
    functors = ["ImpactToMigrationPlanFunctor", "ImpactGateBindingFunctor"]
    if "InputPromptCapsule" in target:
        functors.insert(0, "RawCommandPresheafFunctor")
    if "WorkflowGluingCapsule" in target:
        functors.insert(0, "TableGenerationGluingFunctor")
    if "TableCoreCapsule" in target:
        functors[:0] = ["MutableTableRuntimeFunctor", "ExplicitTableStateFunctor"]
    if "GeneratedRelationCapsule" in target:
        functors.insert(0, "GeneratedColumnEndofunctorFamily")
    if "OutputRenderingCapsule" in target:
        functors[:0] = ["OutputRenderingFunctorFamily", "NormalizedOutputFunctor"]
    if "SchemaTopologyCapsule" in target:
        functors.insert(0, "SchemaToTopologyFunctor")
    if "LocalSectionCapsule" in target:
        functors.insert(0, "LocalDataPresheafFunctor")
    if "CategoricalMetaCapsule" in target:
        functors.append("MigrationWaveOrderingFunctor")
    return _dedupe(functors)


def _transformations_for(candidate: MigrationCandidateSpec) -> tuple[str, ...]:
    target = candidate.target_capsule
    transformations = ["ImpactGateMigrationTransformation", "MigrationPlanCoherenceTransformation"]
    if candidate.current_capsule == "CompatibilityCapsule" or candidate.legacy_owner.startswith(("reta.py", "retaPrompt.py", "libs/")):
        transformations.append("LegacyToArchitectureTransformation")
    if "InputPromptCapsule" in target:
        transformations.append("RawToCanonicalParameterTransformation")
    if "WorkflowGluingCapsule" in target:
        transformations.append("TableGenerationGluingTransformation")
    if "TableCoreCapsule" in target:
        transformations.append("TableRuntimeToStateSectionsTransformation")
    if "GeneratedRelationCapsule" in target:
        transformations.append("GeneratedColumnsSheafSyncTransformation")
    if "OutputRenderingCapsule" in target:
        transformations.append("RenderedOutputNormalizationTransformation")
    if "LocalSectionCapsule" in target or "SemanticSheafCapsule" in target:
        transformations.append("PresheafToSheafGluingTransformation")
    if "CategoricalMetaCapsule" in target:
        transformations.append("TraceBoundaryImpactTransformation")
    return _dedupe(transformations)


def _steps(candidates: Sequence[MigrationCandidateSpec]) -> tuple[MigrationStepSpec, ...]:
    result: list[MigrationStepSpec] = []
    for index, candidate in enumerate(candidates, start=1):
        wave_id = _wave_id_for(candidate)
        action_type = "extract" if "extraction" in candidate.move_kind else "maintain"
        preserved_diagrams = tuple(diagram for diagram in candidate.affected_diagrams if diagram.endswith(("Square", "Triangle")))
        diagrams = _dedupe(preserved_diagrams + ("ImpactMigrationPlanningSquare", "MigrationGateCoherenceSquare"))
        gates = _dedupe(tuple(candidate.gates) + ("ArchitectureMigrationSelfGate",))
        result.append(
            MigrationStepSpec(
                step_id=f"MIG34-{index:02d}",
                wave_id=wave_id,
                candidate=candidate.candidate,
                legacy_owner=candidate.legacy_owner,
                current_capsule=candidate.current_capsule,
                target_capsule=candidate.target_capsule,
                action_type=action_type,
                target_owner=_target_owner_for(candidate),
                category=_category_for_target(candidate.target_capsule),
                functors=_functors_for(candidate),
                natural_transformations=_transformations_for(candidate),
                diagrams=diagrams,
                laws=("ArchitectureImpactGateLaw", "ArchitectureMigrationOrderingLaw"),
                gates=gates,
                prerequisites=("Stage33 impact candidate exists", "Stage34 migration gate binding exists", "all listed probes stay green"),
                observable_invariant="legacy command/API output and architecture snapshot remain observationally equivalent after this move",
                status="planned_not_executed",
            )
        )
    return tuple(result)


def _stage34_gate() -> RegressionGateSpec:
    return RegressionGateSpec(
        "ArchitectureMigrationSelfGate",
        "probe",
        ("migration waves", "migration steps", "gate bindings"),
        "python -B -S reta_architecture_probe_py.py architecture-migration-json",
        ("Stage 34 guarded migration plan",),
        "Stage 34",
        "required",
        "Die Stage-34-Migrationsplanung muss alle Impact-Kandidaten geordnet und gegated halten.",
    )


def _gate_catalog(impact: ArchitectureImpactBundle) -> Mapping[str, RegressionGateSpec]:
    gates = {item.name: item for item in impact.regression_gates}
    stage34 = _stage34_gate()
    gates[stage34.name] = stage34
    return gates


def _gate_bindings(steps: Sequence[MigrationStepSpec], gate_catalog: Mapping[str, RegressionGateSpec]) -> tuple[MigrationGateBindingSpec, ...]:
    result: list[MigrationGateBindingSpec] = []
    for step in steps:
        missing = tuple(gate for gate in step.gates if gate not in gate_catalog)
        commands = {gate: gate_catalog[gate].command for gate in step.gates if gate in gate_catalog}
        result.append(
            MigrationGateBindingSpec(
                step_id=step.step_id,
                candidate=step.candidate,
                gates=tuple(step.gates),
                gate_commands=commands,
                command_parity_required="CommandParityGate" in step.gates,
                bound_diagrams=tuple(step.diagrams),
                missing_gates=missing,
                status="bound" if not missing else "missing_gate",
                reading=f"{step.step_id} may move only when {len(commands)} gate commands, including Stage-34 self validation, remain available.",
            )
        )
    return tuple(result)


def _waves(steps: Sequence[MigrationStepSpec]) -> tuple[MigrationWaveSpec, ...]:
    catalog = _wave_catalog()
    by_wave: dict[str, list[MigrationStepSpec]] = {wave_id: [] for wave_id in catalog}
    for step in steps:
        by_wave.setdefault(step.wave_id, []).append(step)
    waves: list[MigrationWaveSpec] = []
    for wave_id, (order, name, focus, capsules, universal_property, functorial_route, naturality_requirement, default_gates) in sorted(catalog.items(), key=lambda item: item[1][0]):
        members = by_wave.get(wave_id, [])
        member_gates = _dedupe([gate for step in members for gate in step.gates] + list(default_gates))
        waves.append(
            MigrationWaveSpec(
                wave_id=wave_id,
                order=order,
                name=name,
                focus=focus,
                owner_capsules=capsules,
                candidates=tuple(step.candidate for step in members),
                universal_property=universal_property,
                functorial_route=functorial_route,
                naturality_requirement=naturality_requirement,
                required_gates=member_gates,
                status="ready_planned" if members else "empty_planned_wave",
            )
        )
    return tuple(waves)


def _invariants(waves: Sequence[MigrationWaveSpec], steps: Sequence[MigrationStepSpec]) -> tuple[MigrationInvariantSpec, ...]:
    by_wave: dict[str, list[MigrationStepSpec]] = {}
    for step in steps:
        by_wave.setdefault(step.wave_id, []).append(step)
    result: list[MigrationInvariantSpec] = []
    for wave in waves:
        members = by_wave.get(wave.wave_id, [])
        result.append(
            MigrationInvariantSpec(
                name=f"{wave.wave_id}::{wave.name}::NaturalityInvariant",
                wave_id=wave.wave_id,
                applies_to=tuple(step.legacy_owner for step in members),
                diagrams=_dedupe([diagram for step in members for diagram in step.diagrams]),
                laws=_dedupe([law for step in members for law in step.laws]),
                natural_transformations=_dedupe([nt for step in members for nt in step.natural_transformations]),
                required_gates=tuple(wave.required_gates),
                proof_obligation=wave.naturality_requirement,
                status="planned" if members else "empty_wave",
            )
        )
    return tuple(result)


def _validate(
    *,
    candidates: Sequence[MigrationCandidateSpec],
    waves: Sequence[MigrationWaveSpec],
    steps: Sequence[MigrationStepSpec],
    gate_bindings: Sequence[MigrationGateBindingSpec],
    category_theory: CategoryTheoryBundle,
    contracts: ArchitectureContractsBundle,
    gate_catalog: Mapping[str, RegressionGateSpec],
) -> MigrationValidationSpec:
    candidate_names = {item.candidate for item in candidates}
    step_candidates = {item.candidate for item in steps}
    binding_steps = {item.step_id for item in gate_bindings}
    gate_names = set(gate_catalog)
    diagram_names = {item.name for item in contracts.diagrams}
    transformation_names = {item.name for item in category_theory.natural_transformations}
    wave_orders = [wave.order for wave in waves]

    missing_candidates = tuple(sorted(candidate_names - step_candidates))
    steps_without_binding = tuple(sorted(step.step_id for step in steps if step.step_id not in binding_steps))
    unknown_gates = tuple(sorted({gate for step in steps for gate in step.gates if gate not in gate_names}))
    unknown_diagrams = tuple(sorted({diagram for step in steps for diagram in step.diagrams if diagram not in diagram_names}))
    unknown_transformations = tuple(sorted({nt for step in steps for nt in step.natural_transformations if nt not in transformation_names}))
    unordered_waves = tuple(str(order) for order in wave_orders if wave_orders.count(order) > 1)
    empty_waves = tuple(wave.wave_id for wave in waves if not wave.candidates)

    checks = (
        MigrationCheckSpec("MigrationCandidateCoverageCheck", "passed" if not missing_candidates else "failed", missing_candidates, len(candidate_names), "Jeder Stage-33-Impact-Kandidat wird in Stage 34 zu mindestens einem Migrationsschritt."),
        MigrationCheckSpec("MigrationGateBindingCheck", "passed" if not steps_without_binding and not unknown_gates else "failed", steps_without_binding + unknown_gates, len(steps), "Jeder Migrationsschritt besitzt konkrete Gate-Kommandos."),
        MigrationCheckSpec("MigrationDiagramReferenceCheck", "passed" if not unknown_diagrams else "failed", unknown_diagrams, sum(len(step.diagrams) for step in steps), "Migration steps reference known commutative diagrams."),
        MigrationCheckSpec("MigrationNaturalityReferenceCheck", "passed" if not unknown_transformations else "failed", unknown_transformations, sum(len(step.natural_transformations) for step in steps), "Migration steps reference known natural transformations."),
        MigrationCheckSpec("MigrationWaveOrderingCheck", "passed" if not unordered_waves and not empty_waves else "failed", unordered_waves + empty_waves, len(waves), "Migration waves are ordered and non-empty."),
    )
    failures = missing_candidates + steps_without_binding + unknown_gates + unknown_diagrams + unknown_transformations + unordered_waves + empty_waves
    return MigrationValidationSpec(
        status="passed" if not failures else "failed",
        missing_candidates=missing_candidates,
        steps_without_gate_binding=steps_without_binding,
        unknown_gates=unknown_gates,
        unknown_diagrams=unknown_diagrams,
        unknown_natural_transformations=unknown_transformations,
        unordered_waves=unordered_waves,
        empty_waves=empty_waves,
        checks=checks,
    )


def _text_diagram() -> str:
    return """ArchitectureMigrationBundle
├─ MigrationWaveSpec
│  └─ ordered capsule waves M0..M6
├─ MigrationStepSpec
│  └─ Stage-33 candidate → action → target owner → category/functor/naturality
├─ MigrationGateBindingSpec
│  └─ step → regression gates → concrete commands
├─ MigrationInvariantSpec
│  └─ wave → diagrams/laws/natural transformations that must keep commuting
└─ MigrationValidationSpec
   └─ candidate coverage, gate binding, diagram/naturality references and wave order
"""


def _mermaid_diagram() -> str:
    return """```mermaid
flowchart TD
    Impact[ArchitectureImpactBundle<br/>sources + candidates + gates] --> Plan[ArchitectureMigrationBundle]
    Plan --> Waves[ordered migration waves]
    Plan --> Steps[MigrationStepSpec]
    Steps --> Gates[MigrationGateBindingSpec]
    Steps --> Invariants[MigrationInvariantSpec]
    Gates --> Validation[MigrationValidationSpec]
    Invariants --> Validation
    Validation --> Future[future runtime extraction<br/>only after gates commute]
```
"""


def _plan() -> Stage34ArchitecturePlan:
    return Stage34ArchitecturePlan(
        planned_after_stage_33=(
            "Turn Stage-33 impact candidates into a concrete, ordered migration plan instead of leaving them as a flat risk list.",
            "Make each future move stufenweise and kapselweise: wave, step, target owner, functor route, natural transformation, diagrams and gates.",
            "Preserve the adjusted architecture paradigm by treating later code movement as a naturality-preserving migration, not a direct rewrite.",
        ),
        implemented_in_stage_34=(
            "reta_architecture/architecture_migration.py",
            "architecture-migration-json and architecture-migration-md probe commands",
            "ArchitectureMap containment/flow/stage step for ArchitectureMigrationBundle",
            "CategoryTheory ArchitectureMigrationCategory plus migration functors and natural transformations",
            "ArchitectureContracts migration diagrams and ArchitectureMigrationOrderingLaw",
        ),
        inherited_from_previous_stages=(
            "Stage 27 CategoryTheoryBundle",
            "Stage 28 ArchitectureMapBundle",
            "Stage 29 ArchitectureContractsBundle",
            "Stage 30 ArchitectureWitnessBundle",
            "Stage 31 ArchitectureValidation/Coherence bundles",
            "Stage 32 ArchitectureTrace/Boundary bundles",
            "Stage 33 ArchitectureImpactBundle",
        ),
        behaviour_change="keine beabsichtigte Laufzeit-/CLI-/Prompt-/Tabellen-/Output-Verhaltensänderung; Stage 34 ist ein geordneter Migrationsplan über den Stage-33 Impact-Gates",
    )


def bootstrap_architecture_migration(
    *,
    category_theory: CategoryTheoryBundle,
    architecture_map: ArchitectureMapBundle,
    architecture_contracts: ArchitectureContractsBundle,
    architecture_impact: ArchitectureImpactBundle,
) -> ArchitectureMigrationBundle:
    # architecture_map is part of the signature because the migration plan is a
    # stage-map extension even when the current derivation uses impact candidates.
    del architecture_map
    gate_catalog = _gate_catalog(architecture_impact)
    steps = _steps(architecture_impact.migration_candidates)
    waves = _waves(steps)
    gate_bindings = _gate_bindings(steps, gate_catalog)
    invariants = _invariants(waves, steps)
    validation = _validate(
        candidates=architecture_impact.migration_candidates,
        waves=waves,
        steps=steps,
        gate_bindings=gate_bindings,
        category_theory=category_theory,
        contracts=architecture_contracts,
        gate_catalog=gate_catalog,
    )
    return ArchitectureMigrationBundle(
        waves=waves,
        steps=steps,
        gate_bindings=gate_bindings,
        invariants=invariants,
        validation=validation,
        text_diagram=_text_diagram(),
        mermaid_diagram=_mermaid_diagram(),
        plan=_plan(),
    )
