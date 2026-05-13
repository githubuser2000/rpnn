from __future__ import annotations

"""Stage-35 rehearsal/readiness layer for guarded Reta migrations.

Stage 34 orders impact candidates into migration waves, steps and gate
bindings.  Stage 35 still does not move runtime behaviour.  It rehearses each
planned move as an explicit dry-run object: topological open sets for the
migration waves, refactor morphisms for the steps, local gate suites and a
readiness cover that glues the local checks into one global readiness section.
"""

from dataclasses import dataclass
from typing import Mapping, Sequence

from .architecture_contracts import ArchitectureContractsBundle
from .architecture_impact import ArchitectureImpactBundle
from .architecture_migration import ArchitectureMigrationBundle
from .category_theory import CategoryTheoryBundle


@dataclass(frozen=True)
class RehearsalOpenSetSpec:
    open_set_id: str
    wave_id: str
    order: int
    name: str
    owner_capsules: Sequence[str]
    candidates: Sequence[str]
    basis: Sequence[str]
    topology_role: str
    status: str

    def snapshot(self) -> dict:
        return {
            "open_set_id": self.open_set_id,
            "wave_id": self.wave_id,
            "order": self.order,
            "name": self.name,
            "owner_capsules": list(self.owner_capsules),
            "candidates": list(self.candidates),
            "basis": list(self.basis),
            "topology_role": self.topology_role,
            "status": self.status,
        }


@dataclass(frozen=True)
class RehearsalMoveSpec:
    move_id: str
    step_id: str
    wave_id: str
    legacy_owner: str
    target_owner: str
    current_capsule: str
    target_capsule: str
    action_type: str
    category: str
    functors: Sequence[str]
    natural_transformations: Sequence[str]
    diagrams: Sequence[str]
    laws: Sequence[str]
    gates: Sequence[str]
    preconditions: Sequence[str]
    postconditions: Sequence[str]
    rollback_anchor: str
    status: str

    def snapshot(self) -> dict:
        return {
            "move_id": self.move_id,
            "step_id": self.step_id,
            "wave_id": self.wave_id,
            "legacy_owner": self.legacy_owner,
            "target_owner": self.target_owner,
            "current_capsule": self.current_capsule,
            "target_capsule": self.target_capsule,
            "action_type": self.action_type,
            "category": self.category,
            "functors": list(self.functors),
            "natural_transformations": list(self.natural_transformations),
            "diagrams": list(self.diagrams),
            "laws": list(self.laws),
            "gates": list(self.gates),
            "preconditions": list(self.preconditions),
            "postconditions": list(self.postconditions),
            "rollback_anchor": self.rollback_anchor,
            "status": self.status,
        }


@dataclass(frozen=True)
class GateRehearsalSpec:
    gate_suite_id: str
    step_id: str
    candidate: str
    gates: Sequence[str]
    preflight_commands: Mapping[str, str]
    postflight_commands: Mapping[str, str]
    rollback_anchor: str
    command_parity_required: bool
    bound_diagrams: Sequence[str]
    status: str

    def snapshot(self) -> dict:
        return {
            "gate_suite_id": self.gate_suite_id,
            "step_id": self.step_id,
            "candidate": self.candidate,
            "gates": list(self.gates),
            "preflight_commands": dict(self.preflight_commands),
            "postflight_commands": dict(self.postflight_commands),
            "rollback_anchor": self.rollback_anchor,
            "command_parity_required": self.command_parity_required,
            "bound_diagrams": list(self.bound_diagrams),
            "status": self.status,
        }


@dataclass(frozen=True)
class RehearsalCoverSpec:
    cover_id: str
    open_set_id: str
    wave_id: str
    moves: Sequence[str]
    gate_suites: Sequence[str]
    gluing_operation: str
    universal_property: str
    status: str

    def snapshot(self) -> dict:
        return {
            "cover_id": self.cover_id,
            "open_set_id": self.open_set_id,
            "wave_id": self.wave_id,
            "moves": list(self.moves),
            "gate_suites": list(self.gate_suites),
            "gluing_operation": self.gluing_operation,
            "universal_property": self.universal_property,
            "status": self.status,
        }


@dataclass(frozen=True)
class RehearsalCheckSpec:
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
class RehearsalValidationSpec:
    status: str
    missing_migration_steps: Sequence[str]
    rehearsals_without_gate_suite: Sequence[str]
    open_sets_without_cover: Sequence[str]
    unknown_diagrams: Sequence[str]
    unknown_laws: Sequence[str]
    unknown_natural_transformations: Sequence[str]
    gate_suites_with_missing_commands: Sequence[str]
    checks: Sequence[RehearsalCheckSpec]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "missing_migration_steps": list(self.missing_migration_steps),
            "rehearsals_without_gate_suite": list(self.rehearsals_without_gate_suite),
            "open_sets_without_cover": list(self.open_sets_without_cover),
            "unknown_diagrams": list(self.unknown_diagrams),
            "unknown_laws": list(self.unknown_laws),
            "unknown_natural_transformations": list(self.unknown_natural_transformations),
            "gate_suites_with_missing_commands": list(self.gate_suites_with_missing_commands),
            "moves_without_gate_rehearsal": list(self.rehearsals_without_gate_suite),
            "moves_without_open_set": list(self.open_sets_without_cover),
            "unknown_transformations": list(self.unknown_natural_transformations),
            "checks": [item.snapshot() for item in self.checks],
        }


@dataclass(frozen=True)
class Stage35ArchitecturePlan:
    planned_after_stage_34: Sequence[str]
    implemented_in_stage_35: Sequence[str]
    inherited_from_previous_stages: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_after_stage_34": list(self.planned_after_stage_34),
            "implemented_in_stage_35": list(self.implemented_in_stage_35),
            "inherited_from_previous_stages": list(self.inherited_from_previous_stages),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class ArchitectureRehearsalBundle:
    open_sets: Sequence[RehearsalOpenSetSpec]
    moves: Sequence[RehearsalMoveSpec]
    gate_rehearsals: Sequence[GateRehearsalSpec]
    covers: Sequence[RehearsalCoverSpec]
    validation: RehearsalValidationSpec
    plan: Stage35ArchitecturePlan
    text_diagram: str
    mermaid_diagram: str

    def snapshot(self) -> dict:
        return {
            "class": self.__class__.__name__,
            "stage": 35,
            "purpose": "Stage-35 Trockenlauf-/Readiness-Schicht über dem Stage-34-Migrationsplan.",
            "paradigm": [
                "topology",
                "morphism",
                "universal_property",
                "presheaf",
                "sheaf",
                "category",
                "functor",
                "natural_transformation",
                "rehearsal",
                "readiness_gate",
            ],
            "counts": {
                "open_sets": len(self.open_sets),
                "moves": len(self.moves),
                "gate_rehearsals": len(self.gate_rehearsals),
                "covers": len(self.covers),
                "checks": len(self.validation.checks),
            },
            "open_sets": [item.snapshot() for item in self.open_sets],
            "moves": [item.snapshot() for item in self.moves],
            "gate_rehearsals": [item.snapshot() for item in self.gate_rehearsals],
            "covers": [item.snapshot() for item in self.covers],
            "validation": self.validation.snapshot(),
            "plan": self.plan.snapshot(),
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
        }


def _open_sets(migration: ArchitectureMigrationBundle) -> tuple[RehearsalOpenSetSpec, ...]:
    return tuple(
        RehearsalOpenSetSpec(
            open_set_id=f"REH35-OPEN-{wave.wave_id}",
            wave_id=wave.wave_id,
            order=wave.order,
            name=f"Rehearsal open set for {wave.name}",
            owner_capsules=tuple(wave.owner_capsules),
            candidates=tuple(wave.candidates),
            basis=("migration wave", "capsule owner", "candidate set", "gate basis"),
            topology_role="Basisöffnung der geplanten Migration; Einschränkungen laufen über Welle, Kapsel und Legacy-Owner.",
            status="ready",
        )
        for wave in migration.waves
    )


def _with_rehearsal_gate(gates: Sequence[str]) -> tuple[str, ...]:
    return tuple(dict.fromkeys(tuple(gates) + ("ArchitectureRehearsalSelfGate",)))


def _moves(migration: ArchitectureMigrationBundle) -> tuple[RehearsalMoveSpec, ...]:
    return tuple(
        RehearsalMoveSpec(
            move_id=f"REH35-MOVE-{step.step_id}",
            step_id=step.step_id,
            wave_id=step.wave_id,
            legacy_owner=step.legacy_owner,
            target_owner=step.target_owner,
            current_capsule=step.current_capsule,
            target_capsule=step.target_capsule,
            action_type=step.action_type,
            category="ArchitectureRehearsalCategory",
            functors=tuple(dict.fromkeys(tuple(step.functors) + ("MigrationStepRehearsalFunctor",))),
            natural_transformations=tuple(dict.fromkeys(tuple(step.natural_transformations) + ("MigrationRehearsalNaturalityTransformation", "RehearsalReadinessValidationTransformation"))),
            diagrams=tuple(dict.fromkeys(tuple(step.diagrams) + ("MigrationRehearsalSquare", "RehearsalReadinessValidationSquare"))),
            laws=tuple(dict.fromkeys(tuple(step.laws) + ("ArchitectureRehearsalReadinessLaw",))),
            gates=_with_rehearsal_gate(step.gates),
            preconditions=tuple(step.prerequisites) + ("Stage35 rehearsal open set exists", "Stage35 gate suite exists"),
            postconditions=("no runtime behaviour changed", "readiness cover validates", step.observable_invariant),
            rollback_anchor=step.legacy_owner,
            status="rehearsed",
        )
        for step in migration.steps
    )


def _gate_rehearsals(migration: ArchitectureMigrationBundle) -> tuple[GateRehearsalSpec, ...]:
    result: list[GateRehearsalSpec] = []
    for binding in migration.gate_bindings:
        gates = _with_rehearsal_gate(binding.gates)
        preflight = dict(binding.gate_commands)
        preflight.setdefault("ArchitectureRehearsalSelfGate", "python -B -S reta_architecture_probe_py.py architecture-rehearsal-json")
        postflight = dict(preflight)
        postflight.setdefault("ArchitectureValidationGate", "python -B -S reta_architecture_probe_py.py architecture-validation-json")
        result.append(
            GateRehearsalSpec(
                gate_suite_id=f"REH35-GATE-{binding.step_id}",
                step_id=binding.step_id,
                candidate=binding.candidate,
                gates=gates,
                preflight_commands=preflight,
                postflight_commands=postflight,
                rollback_anchor=binding.candidate,
                command_parity_required=binding.command_parity_required,
                bound_diagrams=tuple(dict.fromkeys(tuple(binding.bound_diagrams) + ("MigrationRehearsalSquare", "RehearsalReadinessValidationSquare"))),
                status="ready",
            )
        )
    return tuple(result)


def _covers(open_sets: Sequence[RehearsalOpenSetSpec], moves: Sequence[RehearsalMoveSpec], gates: Sequence[GateRehearsalSpec]) -> tuple[RehearsalCoverSpec, ...]:
    gates_by_step = {gate.step_id: gate.gate_suite_id for gate in gates}
    covers = []
    for open_set in open_sets:
        wave_moves = tuple(move.move_id for move in moves if move.wave_id == open_set.wave_id)
        wave_gate_suites = tuple(gates_by_step.get(move.step_id, "") for move in moves if move.wave_id == open_set.wave_id and move.step_id in gates_by_step)
        covers.append(
            RehearsalCoverSpec(
                cover_id=f"REH35-COVER-{open_set.wave_id}",
                open_set_id=open_set.open_set_id,
                wave_id=open_set.wave_id,
                moves=wave_moves,
                gate_suites=wave_gate_suites,
                gluing_operation="glue_local_gate_rehearsals_to_wave_readiness",
                universal_property="Local rehearsal sections have one canonical readiness cover for the migration wave.",
                status="covered" if wave_moves and wave_gate_suites else "needs_attention",
            )
        )
    return tuple(covers)


def _validate(category_theory: CategoryTheoryBundle, contracts: ArchitectureContractsBundle, migration: ArchitectureMigrationBundle, open_sets, moves, gates, covers) -> RehearsalValidationSpec:
    move_steps = {move.step_id for move in moves}
    migration_steps = {step.step_id for step in migration.steps}
    gate_steps = {gate.step_id for gate in gates}
    open_wave_ids = {item.wave_id for item in open_sets}
    cover_wave_ids = {item.wave_id for item in covers if item.status == "covered"}
    known_diagrams = {diagram.name for diagram in contracts.diagrams}
    known_laws = {law.name for law in contracts.laws}
    known_transformations = {nt.name for nt in category_theory.natural_transformations}

    missing_migration_steps = sorted(migration_steps - move_steps)
    rehearsals_without_gate_suite = sorted(move.step_id for move in moves if move.step_id not in gate_steps)
    open_sets_without_cover = sorted(open_wave_ids - cover_wave_ids)
    unknown_diagrams = sorted({diagram for move in moves for diagram in move.diagrams if diagram not in known_diagrams})
    unknown_laws = sorted({law for move in moves for law in move.laws if law not in known_laws})
    unknown_transformations = sorted({nt for move in moves for nt in move.natural_transformations if nt not in known_transformations})
    gate_suites_with_missing_commands = sorted(gate.gate_suite_id for gate in gates if not gate.preflight_commands or not gate.postflight_commands)
    failures = tuple(missing_migration_steps + rehearsals_without_gate_suite + open_sets_without_cover + unknown_diagrams + unknown_laws + unknown_transformations + gate_suites_with_missing_commands)
    checks = (
        RehearsalCheckSpec("RehearsalStepCoverageCheck", "passed" if not missing_migration_steps else "failed", missing_migration_steps, len(migration.steps), "Every Stage-34 migration step has a Stage-35 rehearsal move."),
        RehearsalCheckSpec("RehearsalGateSuiteCheck", "passed" if not rehearsals_without_gate_suite and not gate_suites_with_missing_commands else "failed", tuple(rehearsals_without_gate_suite + gate_suites_with_missing_commands), len(gates), "Every rehearsal move has preflight and postflight gate commands."),
        RehearsalCheckSpec("RehearsalCoverCheck", "passed" if not open_sets_without_cover else "failed", open_sets_without_cover, len(covers), "Every topological rehearsal open set is glued into a wave cover."),
        RehearsalCheckSpec("RehearsalContractReferenceCheck", "passed" if not unknown_diagrams and not unknown_laws else "failed", tuple(unknown_diagrams + unknown_laws), sum(len(move.diagrams) + len(move.laws) for move in moves), "Rehearsal moves reference known diagrams and refactor laws."),
        RehearsalCheckSpec("RehearsalNaturalityReferenceCheck", "passed" if not unknown_transformations else "failed", unknown_transformations, sum(len(move.natural_transformations) for move in moves), "Rehearsal moves reference known natural transformations."),
    )
    return RehearsalValidationSpec(
        status="passed" if not failures else "failed",
        missing_migration_steps=missing_migration_steps,
        rehearsals_without_gate_suite=rehearsals_without_gate_suite,
        open_sets_without_cover=open_sets_without_cover,
        unknown_diagrams=unknown_diagrams,
        unknown_laws=unknown_laws,
        unknown_natural_transformations=unknown_transformations,
        gate_suites_with_missing_commands=gate_suites_with_missing_commands,
        checks=checks,
    )


def _plan() -> Stage35ArchitecturePlan:
    return Stage35ArchitecturePlan(
        planned_after_stage_34=("turn ordered migration steps into dry-run/readiness objects", "make migration gates rehearsable before runtime movement"),
        implemented_in_stage_35=("reta_architecture/architecture_rehearsal.py", "architecture-rehearsal-json", "architecture-rehearsal-md", "MigrationRehearsalSquare", "RehearsalReadinessValidationSquare"),
        inherited_from_previous_stages=("Stage 27 category/functor/naturality layer", "Stage 28 map", "Stage 29 contracts", "Stage 30 witnesses", "Stage 31 validation/coherence", "Stage 32 traces/boundaries", "Stage 33 impact", "Stage 34 migration"),
        behaviour_change="keine beabsichtigte Laufzeit-/CLI-Verhaltensänderung; Stage 35 ist metadata-only und readiness-orientiert",
    )


_TEXT_DIAGRAM = """ArchitectureRehearsalBundle
├─ RehearsalOpenSetSpec
│  └─ migration waves as topological rehearsal regions
├─ RehearsalMoveSpec
│  └─ migration steps as dry-run refactor morphisms
├─ GateRehearsalSpec
│  └─ gate bindings as preflight/postflight local sections
├─ RehearsalCoverSpec
│  └─ universal gluing of local gate suites into wave readiness
└─ RehearsalValidationSpec
   └─ step coverage, gate coverage, cover coverage and naturality references
"""

_MERMAID_DIAGRAM = """flowchart TD
    Migration[ArchitectureMigrationBundle] -->|MigrationStepRehearsalFunctor| Move[RehearsalMoveSpec]
    Migration -->|MigrationGateRehearsalFunctor| Gate[GateRehearsalSpec]
    Move -->|RehearsalCoverFunctor| Cover[RehearsalCoverSpec]
    Gate --> Cover
    Cover -->|RehearsalGateValidationFunctor| Validation[ArchitectureValidationBundle]
"""


def bootstrap_architecture_rehearsal(
    *,
    category_theory: CategoryTheoryBundle,
    architecture_contracts: ArchitectureContractsBundle,
    architecture_impact: ArchitectureImpactBundle,
    architecture_migration: ArchitectureMigrationBundle,
) -> ArchitectureRehearsalBundle:
    """Return the Stage-35 dry-run/readiness layer."""

    del architecture_impact  # retained as explicit dependency from the Stage-33/34 pipeline
    open_sets = _open_sets(architecture_migration)
    moves = _moves(architecture_migration)
    gate_rehearsals = _gate_rehearsals(architecture_migration)
    covers = _covers(open_sets, moves, gate_rehearsals)
    validation = _validate(category_theory, architecture_contracts, architecture_migration, open_sets, moves, gate_rehearsals, covers)
    return ArchitectureRehearsalBundle(
        open_sets=open_sets,
        moves=moves,
        gate_rehearsals=gate_rehearsals,
        covers=covers,
        validation=validation,
        plan=_plan(),
        text_diagram=_TEXT_DIAGRAM,
        mermaid_diagram=_MERMAID_DIAGRAM,
    )
