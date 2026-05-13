from __future__ import annotations

"""Stage-36 activation/commit layer for guarded Reta migrations.

Stage 35 turned the Stage-34 migration plan into rehearsal open sets, dry-run
refactor morphisms, gate suites and readiness covers.  Stage 36 still does not
move runtime behaviour.  It adds the next missing architectural object: a
controlled activation envelope.  A rehearsed move becomes an activation unit;
its gate suite becomes a preflight/commit/postflight/rollback gate; local
activation units glue into a transaction window with an explicit rollback
section.

Mathematical reading:

* rehearsal open sets are topological domains for possible activation;
* activation units are morphisms from a rehearsed move to a controlled commit;
* activation gates are local presheaf sections of commands and rollback anchors;
* transactions glue local activation sections into one global readiness section;
* rollback and validation are naturality requirements before future code moves.
"""

from dataclasses import dataclass
from typing import Mapping, Sequence

from .architecture_contracts import ArchitectureContractsBundle
from .architecture_migration import ArchitectureMigrationBundle
from .architecture_rehearsal import ArchitectureRehearsalBundle, GateRehearsalSpec, RehearsalMoveSpec
from .category_theory import CategoryTheoryBundle


@dataclass(frozen=True)
class ActivationWindowSpec:
    """Topological window in which a rehearsed migration wave could activate."""

    window_id: str
    wave_id: str
    order: int
    open_set_id: str
    owner_capsules: Sequence[str]
    activation_units: Sequence[str]
    basis: Sequence[str]
    topology_role: str
    status: str

    def snapshot(self) -> dict:
        return {
            "window_id": self.window_id,
            "wave_id": self.wave_id,
            "order": self.order,
            "open_set_id": self.open_set_id,
            "owner_capsules": list(self.owner_capsules),
            "activation_units": list(self.activation_units),
            "basis": list(self.basis),
            "topology_role": self.topology_role,
            "status": self.status,
        }


@dataclass(frozen=True)
class ActivationUnitSpec:
    """One rehearsed move lifted to a controlled, not-yet-executed activation unit."""

    activation_id: str
    move_id: str
    step_id: str
    wave_id: str
    legacy_owner: str
    target_owner: str
    current_capsule: str
    target_capsule: str
    activation_kind: str
    category: str
    functors: Sequence[str]
    natural_transformations: Sequence[str]
    diagrams: Sequence[str]
    laws: Sequence[str]
    required_gates: Sequence[str]
    rehearsal_gate_suite: str
    commit_strategy: str
    rollback_strategy: str
    observable_invariant: str
    status: str

    def snapshot(self) -> dict:
        return {
            "activation_id": self.activation_id,
            "move_id": self.move_id,
            "step_id": self.step_id,
            "wave_id": self.wave_id,
            "legacy_owner": self.legacy_owner,
            "target_owner": self.target_owner,
            "current_capsule": self.current_capsule,
            "target_capsule": self.target_capsule,
            "activation_kind": self.activation_kind,
            "category": self.category,
            "functors": list(self.functors),
            "natural_transformations": list(self.natural_transformations),
            "diagrams": list(self.diagrams),
            "laws": list(self.laws),
            "required_gates": list(self.required_gates),
            "rehearsal_gate_suite": self.rehearsal_gate_suite,
            "commit_strategy": self.commit_strategy,
            "rollback_strategy": self.rollback_strategy,
            "observable_invariant": self.observable_invariant,
            "status": self.status,
        }


@dataclass(frozen=True)
class ActivationGateSpec:
    """Gate suite for an activation unit: preflight, commit, postflight and rollback."""

    activation_id: str
    gate_suite_id: str
    gates: Sequence[str]
    preflight_commands: Mapping[str, str]
    commit_commands: Mapping[str, str]
    postflight_commands: Mapping[str, str]
    rollback_commands: Mapping[str, str]
    command_parity_required: bool
    bound_diagrams: Sequence[str]
    status: str

    def snapshot(self) -> dict:
        return {
            "activation_id": self.activation_id,
            "gate_suite_id": self.gate_suite_id,
            "gates": list(self.gates),
            "preflight_commands": dict(self.preflight_commands),
            "commit_commands": dict(self.commit_commands),
            "postflight_commands": dict(self.postflight_commands),
            "rollback_commands": dict(self.rollback_commands),
            "command_parity_required": self.command_parity_required,
            "bound_diagrams": list(self.bound_diagrams),
            "status": self.status,
        }


@dataclass(frozen=True)
class ActivationRollbackSpec:
    """Rollback section that keeps a future activation reversible."""

    activation_id: str
    rollback_anchor: str
    rollback_commands: Mapping[str, str]
    protected_diagrams: Sequence[str]
    protected_laws: Sequence[str]
    status: str

    def snapshot(self) -> dict:
        return {
            "activation_id": self.activation_id,
            "rollback_anchor": self.rollback_anchor,
            "rollback_commands": dict(self.rollback_commands),
            "protected_diagrams": list(self.protected_diagrams),
            "protected_laws": list(self.protected_laws),
            "status": self.status,
        }


@dataclass(frozen=True)
class ActivationTransactionSpec:
    """Universal gluing of activation units inside one wave/window."""

    transaction_id: str
    window_id: str
    wave_id: str
    activation_units: Sequence[str]
    gate_suites: Sequence[str]
    rollback_sections: Sequence[str]
    commit_order: Sequence[str]
    gluing_operation: str
    universal_property: str
    status: str

    def snapshot(self) -> dict:
        return {
            "transaction_id": self.transaction_id,
            "window_id": self.window_id,
            "wave_id": self.wave_id,
            "activation_units": list(self.activation_units),
            "gate_suites": list(self.gate_suites),
            "rollback_sections": list(self.rollback_sections),
            "commit_order": list(self.commit_order),
            "gluing_operation": self.gluing_operation,
            "universal_property": self.universal_property,
            "status": self.status,
        }


@dataclass(frozen=True)
class ActivationCheckSpec:
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
class ActivationValidationSpec:
    status: str
    missing_rehearsal_moves: Sequence[str]
    activations_without_gate: Sequence[str]
    activations_without_rollback: Sequence[str]
    windows_without_transaction: Sequence[str]
    unknown_diagrams: Sequence[str]
    unknown_laws: Sequence[str]
    unknown_natural_transformations: Sequence[str]
    transactions_not_ready: Sequence[str]
    checks: Sequence[ActivationCheckSpec]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "missing_rehearsal_moves": list(self.missing_rehearsal_moves),
            "activations_without_gate": list(self.activations_without_gate),
            "activations_without_rollback": list(self.activations_without_rollback),
            "windows_without_transaction": list(self.windows_without_transaction),
            "unknown_diagrams": list(self.unknown_diagrams),
            "unknown_laws": list(self.unknown_laws),
            "unknown_natural_transformations": list(self.unknown_natural_transformations),
            "transactions_not_ready": list(self.transactions_not_ready),
            "checks": [item.snapshot() for item in self.checks],
        }


@dataclass(frozen=True)
class Stage36ArchitecturePlan:
    planned_after_stage_35: Sequence[str]
    implemented_in_stage_36: Sequence[str]
    inherited_from_previous_stages: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_after_stage_35": list(self.planned_after_stage_35),
            "implemented_in_stage_36": list(self.implemented_in_stage_36),
            "inherited_from_previous_stages": list(self.inherited_from_previous_stages),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class ArchitectureActivationBundle:
    windows: Sequence[ActivationWindowSpec]
    units: Sequence[ActivationUnitSpec]
    gates: Sequence[ActivationGateSpec]
    rollbacks: Sequence[ActivationRollbackSpec]
    transactions: Sequence[ActivationTransactionSpec]
    validation: ActivationValidationSpec
    plan: Stage36ArchitecturePlan
    text_diagram: str
    mermaid_diagram: str

    def snapshot(self) -> dict:
        return {
            "class": self.__class__.__name__,
            "stage": 36,
            "purpose": "Stage-36 Aktivierungs-/Commit-Schicht über Stage-35-Rehearsal-Moves, ohne Runtime-Logik zu verschieben.",
            "paradigm": [
                "topology",
                "morphism",
                "universal_property",
                "presheaf",
                "sheaf",
                "category",
                "functor",
                "natural_transformation",
                "activation",
                "commit_gate",
                "rollback_section",
            ],
            "counts": {
                "windows": len(self.windows),
                "units": len(self.units),
                "gates": len(self.gates),
                "rollbacks": len(self.rollbacks),
                "transactions": len(self.transactions),
                "checks": len(self.validation.checks),
            },
            "windows": [item.snapshot() for item in self.windows],
            "units": [item.snapshot() for item in self.units],
            "gates": [item.snapshot() for item in self.gates],
            "rollbacks": [item.snapshot() for item in self.rollbacks],
            "transactions": [item.snapshot() for item in self.transactions],
            "validation": self.validation.snapshot(),
            "plan": self.plan.snapshot(),
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
        }


def _dedupe(items: Sequence[str]) -> tuple[str, ...]:
    return tuple(dict.fromkeys(item for item in items if item))


def _gate_by_step(rehearsal: ArchitectureRehearsalBundle) -> dict[str, GateRehearsalSpec]:
    return {gate.step_id: gate for gate in rehearsal.gate_rehearsals}


def _windows(rehearsal: ArchitectureRehearsalBundle) -> tuple[ActivationWindowSpec, ...]:
    move_ids_by_wave: dict[str, list[str]] = {}
    for move in rehearsal.moves:
        move_ids_by_wave.setdefault(move.wave_id, []).append(f"ACT36-{move.move_id}")
    return tuple(
        ActivationWindowSpec(
            window_id=f"ACT36-WINDOW-{open_set.wave_id}",
            wave_id=open_set.wave_id,
            order=open_set.order,
            open_set_id=open_set.open_set_id,
            owner_capsules=tuple(open_set.owner_capsules),
            activation_units=tuple(move_ids_by_wave.get(open_set.wave_id, ())),
            basis=("rehearsal open set", "gate suite", "commit order", "rollback section"),
            topology_role="activation window is an open neighbourhood in which rehearsed moves may be committed later",
            status="ready_window" if move_ids_by_wave.get(open_set.wave_id) else "empty_window",
        )
        for open_set in rehearsal.open_sets
    )


def _unit_from_move(move: RehearsalMoveSpec, gate: GateRehearsalSpec | None) -> ActivationUnitSpec:
    activation_id = f"ACT36-{move.move_id}"
    has_gate = gate is not None
    return ActivationUnitSpec(
        activation_id=activation_id,
        move_id=move.move_id,
        step_id=move.step_id,
        wave_id=move.wave_id,
        legacy_owner=move.legacy_owner,
        target_owner=move.target_owner,
        current_capsule=move.current_capsule,
        target_capsule=move.target_capsule,
        activation_kind="commit_ready_envelope" if has_gate else "commit_blocked_envelope",
        category="ArchitectureActivationCategory",
        functors=_dedupe(tuple(move.functors) + ("RehearsalActivationFunctor", "ActivationTransactionFunctor")),
        natural_transformations=_dedupe(tuple(move.natural_transformations) + ("RehearsalActivationNaturalityTransformation", "ActivationRollbackValidationTransformation")),
        diagrams=_dedupe(tuple(move.diagrams) + ("RehearsalActivationSquare", "ActivationRollbackValidationSquare")),
        laws=_dedupe(tuple(move.laws) + ("ArchitectureActivationCommitLaw",)),
        required_gates=_dedupe(tuple(move.gates) + ("ArchitectureActivationSelfGate",)),
        rehearsal_gate_suite=gate.gate_suite_id if has_gate else "",
        commit_strategy="metadata-only activation envelope; future code move must be applied by a later stage after all gates pass",
        rollback_strategy=gate.rollback_anchor if has_gate else move.rollback_anchor,
        observable_invariant="legacy CLI/API output and architecture snapshots must remain equivalent before and after a future activation",
        status="activation_ready" if has_gate else "missing_gate_suite",
    )


def _units(rehearsal: ArchitectureRehearsalBundle) -> tuple[ActivationUnitSpec, ...]:
    gates = _gate_by_step(rehearsal)
    return tuple(_unit_from_move(move, gates.get(move.step_id)) for move in rehearsal.moves)


def _activation_gates(units: Sequence[ActivationUnitSpec], rehearsal: ArchitectureRehearsalBundle) -> tuple[ActivationGateSpec, ...]:
    gates = {gate.gate_suite_id: gate for gate in rehearsal.gate_rehearsals}
    result: list[ActivationGateSpec] = []
    for unit in units:
        gate = gates.get(unit.rehearsal_gate_suite)
        if gate is None:
            result.append(
                ActivationGateSpec(
                    activation_id=unit.activation_id,
                    gate_suite_id=f"ACT36-GATE-{unit.step_id}",
                    gates=tuple(unit.required_gates),
                    preflight_commands={},
                    commit_commands={},
                    postflight_commands={},
                    rollback_commands={},
                    command_parity_required=False,
                    bound_diagrams=tuple(unit.diagrams),
                    status="missing_rehearsal_gate",
                )
            )
            continue
        preflight = dict(gate.preflight_commands)
        preflight.setdefault("ArchitectureActivationSelfGate", "python -B -S reta_architecture_probe_py.py architecture-activation-json")
        postflight = dict(gate.postflight_commands)
        postflight.setdefault("ArchitectureActivationPostflight", "python -B -S reta_architecture_probe_py.py architecture-validation-json")
        commit_commands = {
            "ActivationCommitDryRun": "python -B -S reta_architecture_probe_py.py architecture-activation-json",
            "ActivationCoherenceDryRun": "python -B -S reta_architecture_probe_py.py architecture-coherence-json",
        }
        rollback_commands = {
            "ActivationRollbackAnchor": gate.rollback_anchor,
            "ActivationRollbackValidation": "python -B -S reta_architecture_probe_py.py architecture-activation-json",
        }
        result.append(
            ActivationGateSpec(
                activation_id=unit.activation_id,
                gate_suite_id=f"ACT36-GATE-{unit.step_id}",
                gates=_dedupe(tuple(gate.gates) + ("ArchitectureActivationSelfGate",)),
                preflight_commands=preflight,
                commit_commands=commit_commands,
                postflight_commands=postflight,
                rollback_commands=rollback_commands,
                command_parity_required=gate.command_parity_required,
                bound_diagrams=_dedupe(tuple(gate.bound_diagrams) + tuple(unit.diagrams)),
                status="commit_gated",
            )
        )
    return tuple(result)


def _rollbacks(units: Sequence[ActivationUnitSpec], gates: Sequence[ActivationGateSpec]) -> tuple[ActivationRollbackSpec, ...]:
    gates_by_activation = {gate.activation_id: gate for gate in gates}
    result: list[ActivationRollbackSpec] = []
    for unit in units:
        gate = gates_by_activation.get(unit.activation_id)
        rollback_commands = dict(gate.rollback_commands) if gate else {}
        result.append(
            ActivationRollbackSpec(
                activation_id=unit.activation_id,
                rollback_anchor=unit.rollback_strategy,
                rollback_commands=rollback_commands,
                protected_diagrams=tuple(unit.diagrams),
                protected_laws=tuple(unit.laws),
                status="rollback_ready" if rollback_commands and unit.rollback_strategy else "rollback_missing",
            )
        )
    return tuple(result)


def _transactions(windows: Sequence[ActivationWindowSpec], units: Sequence[ActivationUnitSpec], gates: Sequence[ActivationGateSpec], rollbacks: Sequence[ActivationRollbackSpec]) -> tuple[ActivationTransactionSpec, ...]:
    unit_by_id = {unit.activation_id: unit for unit in units}
    gate_by_activation = {gate.activation_id: gate for gate in gates}
    rollback_by_activation = {rollback.activation_id: rollback for rollback in rollbacks}
    result: list[ActivationTransactionSpec] = []
    for window in windows:
        window_units = tuple(unit_id for unit_id in window.activation_units if unit_id in unit_by_id)
        gate_ids = tuple(gate_by_activation[unit_id].gate_suite_id for unit_id in window_units if unit_id in gate_by_activation)
        rollback_ids = tuple(unit_id for unit_id in window_units if unit_id in rollback_by_activation and rollback_by_activation[unit_id].status == "rollback_ready")
        ready = bool(window_units) and len(gate_ids) == len(window_units) and len(rollback_ids) == len(window_units)
        result.append(
            ActivationTransactionSpec(
                transaction_id=f"ACT36-TX-{window.wave_id}",
                window_id=window.window_id,
                wave_id=window.wave_id,
                activation_units=window_units,
                gate_suites=gate_ids,
                rollback_sections=rollback_ids,
                commit_order=window_units,
                gluing_operation="glue_activation_units_and_gate_sections_to_transaction_window",
                universal_property="Local activation units have one canonical guarded transaction with rollback before any future runtime move.",
                status="transaction_ready" if ready else "transaction_blocked",
            )
        )
    return tuple(result)


def _validate(
    category_theory: CategoryTheoryBundle,
    contracts: ArchitectureContractsBundle,
    migration: ArchitectureMigrationBundle,
    rehearsal: ArchitectureRehearsalBundle,
    windows: Sequence[ActivationWindowSpec],
    units: Sequence[ActivationUnitSpec],
    gates: Sequence[ActivationGateSpec],
    rollbacks: Sequence[ActivationRollbackSpec],
    transactions: Sequence[ActivationTransactionSpec],
) -> ActivationValidationSpec:
    del migration  # kept as explicit Stage-34 dependency for the activation pipeline
    move_ids = {move.move_id for move in rehearsal.moves}
    unit_moves = {unit.move_id for unit in units}
    gate_units = {gate.activation_id for gate in gates if gate.status == "commit_gated"}
    rollback_units = {rollback.activation_id for rollback in rollbacks if rollback.status == "rollback_ready"}
    window_ids = {window.window_id for window in windows}
    transaction_windows = {tx.window_id for tx in transactions if tx.status == "transaction_ready"}
    known_diagrams = {diagram.name for diagram in contracts.diagrams}
    known_laws = {law.name for law in contracts.laws}
    known_transformations = {nt.name for nt in category_theory.natural_transformations}

    missing_rehearsal_moves = tuple(sorted(move_ids - unit_moves))
    activations_without_gate = tuple(sorted(unit.activation_id for unit in units if unit.activation_id not in gate_units))
    activations_without_rollback = tuple(sorted(unit.activation_id for unit in units if unit.activation_id not in rollback_units))
    windows_without_transaction = tuple(sorted(window_ids - transaction_windows))
    unknown_diagrams = tuple(sorted({diagram for unit in units for diagram in unit.diagrams if diagram not in known_diagrams}))
    unknown_laws = tuple(sorted({law for unit in units for law in unit.laws if law not in known_laws}))
    unknown_transformations = tuple(sorted({nt for unit in units for nt in unit.natural_transformations if nt not in known_transformations}))
    transactions_not_ready = tuple(sorted(tx.transaction_id for tx in transactions if tx.status != "transaction_ready"))
    failures = missing_rehearsal_moves + activations_without_gate + activations_without_rollback + windows_without_transaction + unknown_diagrams + unknown_laws + unknown_transformations + transactions_not_ready
    checks = (
        ActivationCheckSpec("ActivationRehearsalCoverageCheck", "passed" if not missing_rehearsal_moves else "failed", missing_rehearsal_moves, len(rehearsal.moves), "Every Stage-35 rehearsal move has a Stage-36 activation unit."),
        ActivationCheckSpec("ActivationGateCoverageCheck", "passed" if not activations_without_gate else "failed", activations_without_gate, len(gates), "Every activation unit is backed by a commit gate suite."),
        ActivationCheckSpec("ActivationRollbackCoverageCheck", "passed" if not activations_without_rollback else "failed", activations_without_rollback, len(rollbacks), "Every activation unit has a rollback section."),
        ActivationCheckSpec("ActivationTransactionCoverCheck", "passed" if not windows_without_transaction and not transactions_not_ready else "failed", windows_without_transaction + transactions_not_ready, len(transactions), "Every activation window glues to a ready transaction."),
        ActivationCheckSpec("ActivationContractReferenceCheck", "passed" if not unknown_diagrams and not unknown_laws else "failed", unknown_diagrams + unknown_laws, sum(len(unit.diagrams) + len(unit.laws) for unit in units), "Activation units reference known diagrams and laws."),
        ActivationCheckSpec("ActivationNaturalityReferenceCheck", "passed" if not unknown_transformations else "failed", unknown_transformations, sum(len(unit.natural_transformations) for unit in units), "Activation units reference known natural transformations."),
    )
    return ActivationValidationSpec(
        status="passed" if not failures else "failed",
        missing_rehearsal_moves=missing_rehearsal_moves,
        activations_without_gate=activations_without_gate,
        activations_without_rollback=activations_without_rollback,
        windows_without_transaction=windows_without_transaction,
        unknown_diagrams=unknown_diagrams,
        unknown_laws=unknown_laws,
        unknown_natural_transformations=unknown_transformations,
        transactions_not_ready=transactions_not_ready,
        checks=checks,
    )


def _plan() -> Stage36ArchitecturePlan:
    return Stage36ArchitecturePlan(
        planned_after_stage_35=(
            "turn rehearsal/readiness moves into controlled activation envelopes",
            "make future commits explicit as preflight/commit/postflight/rollback transactions before moving runtime code",
        ),
        implemented_in_stage_36=(
            "reta_architecture/architecture_activation.py",
            "architecture-activation-json",
            "architecture-activation-md",
            "RehearsalActivationSquare",
            "ActivationRollbackValidationSquare",
            "ArchitectureActivationCommitLaw",
        ),
        inherited_from_previous_stages=(
            "Stage 27 category/functor/naturality layer",
            "Stage 28 architecture map",
            "Stage 29 contracts",
            "Stage 30 witnesses",
            "Stage 31 validation/coherence",
            "Stage 32 traces/boundaries",
            "Stage 33 impact",
            "Stage 34 migration",
            "Stage 35 rehearsal/readiness",
        ),
        behaviour_change="keine beabsichtigte Laufzeit-/CLI-Verhaltensänderung; Stage 36 ist metadata-only und beschreibt spätere Aktivierung/Commit/Rollback",
    )


_TEXT_DIAGRAM = """ArchitectureActivationBundle
├─ ActivationWindowSpec
│  └─ rehearsal open sets as controlled activation windows
├─ ActivationUnitSpec
│  └─ rehearsal moves as not-yet-executed commit envelopes
├─ ActivationGateSpec
│  └─ preflight / commit / postflight / rollback command sections
├─ ActivationRollbackSpec
│  └─ rollback anchors protecting diagrams and laws
├─ ActivationTransactionSpec
│  └─ universal gluing of local activation units into wave transactions
└─ ActivationValidationSpec
   └─ rehearsal coverage, gate coverage, rollback coverage, transaction coverage and naturality references
"""

_MERMAID_DIAGRAM = """flowchart TD
    Rehearsal[ArchitectureRehearsalBundle] -->|RehearsalActivationFunctor| Unit[ActivationUnitSpec]
    Rehearsal -->|GateActivationFunctor| Gate[ActivationGateSpec]
    Unit -->|ActivationTransactionFunctor| Tx[ActivationTransactionSpec]
    Gate --> Tx
    Gate -->|ActivationRollbackFunctor| Rollback[ActivationRollbackSpec]
    Tx -->|ActivationValidationFunctor| Validation[ArchitectureValidationBundle]
"""


def bootstrap_architecture_activation(
    *,
    category_theory: CategoryTheoryBundle,
    architecture_contracts: ArchitectureContractsBundle,
    architecture_migration: ArchitectureMigrationBundle,
    architecture_rehearsal: ArchitectureRehearsalBundle,
) -> ArchitectureActivationBundle:
    """Return the Stage-36 activation/commit/rollback envelope."""

    windows = _windows(architecture_rehearsal)
    units = _units(architecture_rehearsal)
    gates = _activation_gates(units, architecture_rehearsal)
    rollbacks = _rollbacks(units, gates)
    transactions = _transactions(windows, units, gates, rollbacks)
    validation = _validate(
        category_theory,
        architecture_contracts,
        architecture_migration,
        architecture_rehearsal,
        windows,
        units,
        gates,
        rollbacks,
        transactions,
    )
    return ArchitectureActivationBundle(
        windows=windows,
        units=units,
        gates=gates,
        rollbacks=rollbacks,
        transactions=transactions,
        validation=validation,
        plan=_plan(),
        text_diagram=_TEXT_DIAGRAM,
        mermaid_diagram=_MERMAID_DIAGRAM,
    )
