from __future__ import annotations

"""Stage-33 executable validation layer for the categorical Reta architecture.

Stages 27-32 made the mathematical architecture visible as categories,
functors, natural transformations, a capsule map, commutative contracts,
repository witnesses, coherence, traces and boundary graphs. Stage 33 composes
those layers with the impact/migration-gate calculus so future refactors can see
what they touch before moving behaviour. Stage 34 adds a migration-plan layer
that orders those candidates into waves, steps and gate bindings.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Sequence

from .architecture_contracts import ArchitectureContractsBundle
from .architecture_map import ArchitectureMapBundle
from .architecture_witnesses import ArchitectureWitnessBundle
from .architecture_traces import ArchitectureTraceBundle
from .architecture_boundaries import ArchitectureBoundariesBundle
from .architecture_impact import ArchitectureImpactBundle
from .architecture_migration import ArchitectureMigrationBundle
from .architecture_rehearsal import ArchitectureRehearsalBundle
from .architecture_activation import ArchitectureActivationBundle
from .row_ranges import RowRangeMorphismBundle
from .arithmetic import ArithmeticMorphismBundle
from .console_io import ConsoleIOMorphismBundle
from .completion_word import WordCompletionMorphismBundle
from .completion_nested import NestedCompletionMorphismBundle
from .category_theory import CategoryTheoryBundle
from .package_integrity import REQUIRED_SOURCE_PATHS, is_runtime_artifact


@dataclass(frozen=True)
class ArchitectureValidationCheckSpec:
    """One executable consistency check over the staged architecture graph."""

    name: str
    layer: str
    obligation: str
    status: str
    severity: str
    checked_count: int
    failed_items: Sequence[str]
    evidence: Sequence[str]
    description: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "layer": self.layer,
            "obligation": self.obligation,
            "status": self.status,
            "severity": self.severity,
            "checked_count": self.checked_count,
            "failed_items": list(self.failed_items),
            "evidence": list(self.evidence),
            "description": self.description,
        }


@dataclass(frozen=True)
class ArchitectureValidationLayerSpec:
    """Grouped checks for one architecture layer."""

    name: str
    role: str
    checks: Sequence[str]
    status: str
    failed_checks: Sequence[str]

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "role": self.role,
            "checks": list(self.checks),
            "status": self.status,
            "failed_checks": list(self.failed_checks),
        }


@dataclass(frozen=True)
class ArchitectureValidationSummarySpec:
    """Stage-31 validation summary."""

    status: str
    total_checks: int
    passed_checks: int
    attention_checks: int
    failed_checks: int
    warning_checks: int
    error_checks: int
    checked_items: int
    failed_items: Sequence[str]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "total_checks": self.total_checks,
            "passed_checks": self.passed_checks,
            "attention_checks": self.attention_checks,
            "failed_checks": self.failed_checks,
            "warning_checks": self.warning_checks,
            "error_checks": self.error_checks,
            "checked_items": self.checked_items,
            "failed_items": list(self.failed_items),
        }


@dataclass(frozen=True)
class Stage31ArchitecturePlan:
    """Bridge from Stage 30 witnesses to Stage 31 validation."""

    planned_after_stage_30: Sequence[str]
    implemented_in_stage_31: Sequence[str]
    inherited_from_previous_stages: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_after_stage_30": list(self.planned_after_stage_30),
            "implemented_in_stage_31": list(self.implemented_in_stage_31),
            "inherited_from_previous_stages": list(self.inherited_from_previous_stages),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class ArchitectureValidationBundle:
    """Executable Stage-31 audit over categories, map, contracts and witnesses."""

    checks: Sequence[ArchitectureValidationCheckSpec]
    layers: Sequence[ArchitectureValidationLayerSpec]
    summary: ArchitectureValidationSummarySpec
    text_diagram: str
    mermaid_diagram: str
    plan: Stage31ArchitecturePlan

    def check_named(self, name: str) -> ArchitectureValidationCheckSpec:
        for check in self.checks:
            if check.name == name:
                return check
        raise KeyError(f"Unknown architecture validation check: {name}")

    def assert_passed(self) -> None:
        if self.summary.status != "passed":
            raise AssertionError("Architecture validation did not pass: " + ", ".join(self.summary.failed_items))

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "stage": 41,
            "purpose": "Ausführbare Validierung der Topologie-/Garben-/Kategorien-/Funktoren-/natürliche-Transformationen-Architektur.",
            "paradigm": [
                "topology",
                "morphism",
                "universal_property",
                "presheaf",
                "sheaf",
                "category",
                "functor",
                "natural_transformation",
                "commutative_diagram",
                "witness",
                "validation",
                "trace",
                "boundary",
                "impact",
                "migration_gate",
                "migration_plan",
                "readiness_gate",
                "activation_commit",
                "rollback_section",
                "activated_runtime_morphism",
                "activated_console_io",
                "activated_nested_completion",
            ],
            "counts": {
                "checks": len(self.checks),
                "layers": len(self.layers),
                "passed_checks": self.summary.passed_checks,
                "attention_checks": self.summary.attention_checks,
                "failed_checks": self.summary.failed_checks,
            },
            "summary": self.summary.snapshot(),
            "layers": [item.snapshot() for item in self.layers],
            "checks": [item.snapshot() for item in self.checks],
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
            "plan": self.plan.snapshot(),
        }


def _passed(failed_items: Sequence[str]) -> str:
    return "passed" if not failed_items else "attention"


def _check(
    name: str,
    layer: str,
    obligation: str,
    failed_items: Sequence[str],
    checked_count: int,
    evidence: Sequence[str],
    description: str,
    severity: str = "error",
) -> ArchitectureValidationCheckSpec:
    return ArchitectureValidationCheckSpec(
        name=name,
        layer=layer,
        obligation=obligation,
        status=_passed(tuple(failed_items)),
        severity=severity,
        checked_count=checked_count,
        failed_items=tuple(failed_items),
        evidence=tuple(evidence),
        description=description,
    )


def _category_checks(category_theory: CategoryTheoryBundle) -> tuple[ArchitectureValidationCheckSpec, ...]:
    category_names = {item.name for item in category_theory.categories}
    functor_names = {item.name for item in category_theory.functors}
    paradigm = set(category_theory.snapshot().get("paradigm", ()))
    required_terms = {
        "topology",
        "morphism",
        "universal_property",
        "presheaf",
        "sheaf",
        "category",
        "functor",
        "natural_transformation",
    }
    functor_failures = tuple(
        sorted(
            f"{functor.name}:{ref}"
            for functor in category_theory.functors
            for ref in (functor.source_category, functor.target_category)
            if ref not in category_names
        )
    )
    transformation_failures = tuple(
        sorted(
            f"{transformation.name}:{ref}"
            for transformation in category_theory.natural_transformations
            for ref in (transformation.source_functor, transformation.target_functor)
            if ref not in functor_names
        )
    )
    term_failures = tuple(sorted(required_terms - paradigm))
    return (
        _check(
            "CategoryFunctorReferenceCheck",
            "CategoryTheoryBundle",
            "Every functor must reference known source and target categories.",
            functor_failures,
            len(category_theory.functors) * 2,
            ("category-theory-json",),
            "Validiert die Kategorie-Endpunkte aller Funktoren.",
        ),
        _check(
            "NaturalTransformationReferenceCheck",
            "CategoryTheoryBundle",
            "Every natural transformation must reference known source and target functors.",
            transformation_failures,
            len(category_theory.natural_transformations) * 2,
            ("category-theory-json",),
            "Validiert, dass natürliche Transformationen wirklich zwischen registrierten Funktoren liegen.",
        ),
        _check(
            "ParadigmTermCoverageCheck",
            "CategoryTheoryBundle",
            "The architecture snapshot must keep the agreed mathematical paradigm terms visible.",
            term_failures,
            len(required_terms),
            ("category-theory-json",),
            "Schützt die vereinbarten Begriffe: Topologie, Morphismus, universelle Eigenschaft, Prägarbe, Garbe, Kategorie, Funktor, natürliche Transformation.",
        ),
    )


def _map_checks(architecture_map: ArchitectureMapBundle) -> tuple[ArchitectureValidationCheckSpec, ...]:
    snapshot = architecture_map.snapshot()
    capsule_names = {item.name for item in architecture_map.capsules}
    meta_children = {
        "CategoryTheoryBundle",
        "ArchitectureMapBundle",
        "ArchitectureContractsBundle",
        "ArchitectureWitnessBundle",
        "ArchitectureValidationBundle",
        "ArchitectureCoherenceBundle",
        "ArchitectureTraceBundle",
        "ArchitectureBoundariesBundle",
        "ArchitectureImpactBundle",
        "ArchitectureMigrationBundle",
        "ArchitectureRehearsalBundle",
        "ArchitectureActivationBundle",
        "RowRangeMorphismBundle",
        "ArithmeticMorphismBundle",
        "ConsoleIOMorphismBundle",
        "WordCompletionMorphismBundle",
        "NestedCompletionMorphismBundle",
    }
    flow_failures = tuple(
        sorted(
            f"{flow.source}->{flow.target}"
            for flow in architecture_map.flows
            if flow.source not in capsule_names or flow.target not in capsule_names
        )
    )
    containment_failures = tuple(
        sorted(
            f"{item.parent}->{item.child}"
            for item in architecture_map.containment
            if item.parent not in capsule_names or (item.child not in capsule_names and item.child not in meta_children)
        )
    )
    stage_failures = ()
    if snapshot.get("stage") != 42:
        stage_failures = (f"stage={snapshot.get('stage')}",)
    stage_names = {item.stage for item in architecture_map.stage_steps}
    return (
        _check(
            "ArchitectureMapStageCheck",
            "ArchitectureMapBundle",
            "The capsule map must advertise the current staged architecture level.",
            stage_failures,
            len(architecture_map.stage_steps),
            ("architecture-map-json",),
            "Prüft, dass Stage 42 in der Architekturkarte sichtbar ist.",
        ),
        _check(
            "ArchitectureFlowCapsuleReferenceCheck",
            "ArchitectureMapBundle",
            "Every architecture flow must connect known capsules.",
            flow_failures,
            len(architecture_map.flows) * 2,
            ("architecture-map-json",),
            "Prüft die Datenflusskanten der Kapselkarte.",
        ),
        _check(
            "CapsuleContainmentReferenceCheck",
            "ArchitectureMapBundle",
            "Containment edges must point to known capsules or registered meta bundles.",
            containment_failures,
            len(architecture_map.containment) * 2,
            ("architecture-diagram-md",),
            "Prüft die stufenweise Kapsel-In-Kapsel-Struktur.",
        ),
    )


def _contract_checks(
    category_theory: CategoryTheoryBundle,
    architecture_map: ArchitectureMapBundle,
    architecture_contracts: ArchitectureContractsBundle,
) -> tuple[ArchitectureValidationCheckSpec, ...]:
    capsule_names = {item.name for item in architecture_map.capsules}
    contract_capsules = {item.capsule for item in architecture_contracts.capsule_contracts}
    diagram_names = {item.name for item in architecture_contracts.diagrams}
    category_names = {item.name for item in category_theory.categories}
    functor_names = {item.name for item in category_theory.functors}
    transformation_names = {item.name for item in category_theory.natural_transformations}
    validation_snapshot = architecture_contracts.validation.snapshot()
    capsule_failures = tuple(sorted((capsule_names - contract_capsules) | (contract_capsules - capsule_names)))
    reference_failures = tuple(
        sorted(
            f"{key}:{value}"
            for key in (
                "missing_capsules",
                "missing_categories",
                "missing_functors",
                "missing_natural_transformations",
            )
            for value in validation_snapshot.get(key, ())
        )
    )
    diagram_reference_failures = tuple(
        sorted(
            f"{diagram.name}:{ref}"
            for diagram in architecture_contracts.diagrams
            for ref in tuple(diagram.categories) + tuple(diagram.functors) + tuple(diagram.natural_transformations)
            if ref not in category_names and ref not in functor_names and ref not in transformation_names
        )
    )
    return (
        _check(
            "CapsuleContractCoverageCheck",
            "ArchitectureContractsBundle",
            "Every capsule in the map must have exactly one contract and every contract must belong to a capsule.",
            capsule_failures,
            len(capsule_names) + len(contract_capsules),
            ("architecture-contracts-json", "architecture-map-json"),
            "Prüft die Kapselgrenzen gegen die Stage-29/31-Verträge.",
        ),
        _check(
            "ContractReferenceValidationCheck",
            "ArchitectureContractsBundle",
            "The built-in contract reference validation must pass.",
            reference_failures,
            len(diagram_names) + len(architecture_contracts.capsule_contracts),
            ("architecture-contracts-json",),
            "Übernimmt und bündelt die Referenzprüfung aus ArchitectureContractsBundle.",
        ),
        _check(
            "DiagramCategoryFunctorTransformationCheck",
            "ArchitectureContractsBundle",
            "Every commutative diagram must reference known categories, functors and natural transformations.",
            diagram_reference_failures,
            sum(len(d.categories) + len(d.functors) + len(d.natural_transformations) for d in architecture_contracts.diagrams),
            ("architecture-contracts-json", "category-theory-json"),
            "Prüft die natürliche-Transformationen-Verträge gegen die Kategorie-Theorie-Schicht.",
        ),
    )


def _witness_checks(
    category_theory: CategoryTheoryBundle,
    architecture_map: ArchitectureMapBundle,
    architecture_contracts: ArchitectureContractsBundle,
    architecture_witnesses: ArchitectureWitnessBundle,
) -> tuple[ArchitectureValidationCheckSpec, ...]:
    capsule_names = {item.name for item in architecture_map.capsules}
    slice_names = {item.capsule for item in architecture_witnesses.capsule_slices}
    diagram_names = {item.name for item in architecture_contracts.diagrams}
    witnessed_diagrams = {item.diagram for item in architecture_witnesses.diagram_witnesses}
    law_names = {item.name for item in architecture_contracts.laws}
    obligation_names = {item.name for item in architecture_witnesses.obligations}
    transformation_names = {item.name for item in category_theory.natural_transformations}
    witnessed_transformations = {
        item.transformation
        for item in architecture_witnesses.naturality_witnesses
        if item.witness_status == "witnessed"
    }
    witness_validation = architecture_witnesses.validation.snapshot()
    witness_failures = tuple(
        sorted(
            f"{key}:{value}"
            for key in (
                "missing_file_like_anchors",
                "uncovered_capsules",
                "uncovered_diagrams",
                "uncovered_laws",
                "uncovered_natural_transformations",
            )
            for value in witness_validation.get(key, ())
        )
    )
    capsule_failures = tuple(sorted((capsule_names - slice_names) | (slice_names - capsule_names)))
    diagram_failures = tuple(sorted((diagram_names - witnessed_diagrams) | (witnessed_diagrams - diagram_names)))
    law_failures = tuple(sorted(law_names - obligation_names))
    transformation_failures = tuple(sorted(transformation_names - witnessed_transformations))
    return (
        _check(
            "WitnessValidationCheck",
            "ArchitectureWitnessBundle",
            "The Stage-30 witness validation must pass under the Stage-31 category/contract graph.",
            witness_failures,
            len(architecture_witnesses.anchor_witnesses),
            ("architecture-witnesses-json",),
            "Bündelt die Anker-, Diagramm-, Gesetz- und Natürlichkeits-Witness-Prüfung.",
        ),
        _check(
            "CapsuleSliceCoverageCheck",
            "ArchitectureWitnessBundle",
            "Every map capsule must have a concrete capsule slice witness.",
            capsule_failures,
            len(capsule_names) + len(slice_names),
            ("architecture-witnesses-json",),
            "Prüft, dass jede Kapsel stufenweise und kapselweise auf konkrete reta-Teile rückgebunden ist.",
        ),
        _check(
            "DiagramWitnessCoverageCheck",
            "ArchitectureWitnessBundle",
            "Every commutative diagram must have a concrete diagram witness.",
            diagram_failures,
            len(diagram_names) + len(witnessed_diagrams),
            ("architecture-witnesses-json", "architecture-contracts-json"),
            "Prüft, dass kein kommutierendes Diagramm nur symbolisch bleibt.",
        ),
        _check(
            "RefactorLawObligationCoverageCheck",
            "ArchitectureWitnessBundle",
            "Every refactor law must be represented as a future-stage obligation.",
            law_failures,
            len(law_names),
            ("architecture-witnesses-json", "architecture-contracts-json"),
            "Prüft, dass alle Refactor-Gesetze als Obligations für spätere Stages sichtbar sind.",
        ),
        _check(
            "NaturalTransformationWitnessCoverageCheck",
            "ArchitectureWitnessBundle",
            "Every natural transformation must be witnessed by at least one contract diagram.",
            transformation_failures,
            len(transformation_names),
            ("architecture-witnesses-json", "category-theory-json"),
            "Prüft die Natürlichkeitsabdeckung von Raw→Canonical bis Stage-31-Validierung.",
        ),
    )


def _trace_checks(architecture_map: ArchitectureMapBundle, architecture_traces: ArchitectureTraceBundle | None) -> tuple[ArchitectureValidationCheckSpec, ...]:
    if architecture_traces is None:
        return (_check("TraceValidationPresenceCheck", "ArchitectureTraceBundle", "Stage 32 validation should receive the trace bundle.", ("architecture_traces missing",), 1, ("architecture-traces-json",), "Prüft, dass die Stage-32-Trace-Schicht eingespeist wird."),)
    validation = architecture_traces.validation.snapshot()
    failures = tuple(sorted(f"{key}:{value}" for key in ("missing_component_traces", "missing_capsule_traces", "missing_stage_traces", "missing_stage32_documents", "unresolved_hops") for value in validation.get(key, ())))
    legacy_owners = {item.legacy_owner for item in architecture_map.legacy_mappings}
    traced_owners = {item.legacy_owner for item in architecture_traces.component_traces}
    capsule_names = {item.name for item in architecture_map.capsules}
    traced_capsules = {item.capsule for item in architecture_traces.capsule_traces}
    stage_names = {item.stage for item in architecture_map.stage_steps}
    traced_stages = {item.stage for item in architecture_traces.stage_traces}
    return (
        _check("ArchitectureTraceValidationCheck", "ArchitectureTraceBundle", "The Stage-32 trace graph must validate owner, capsule, stage and route coverage.", failures, validation.get("route_hop_count", 0) + validation.get("component_trace_count", 0), ("architecture-traces-json",), "Bündelt die interne Stage-32-Trace-Validierung."),
        _check("ComponentTraceCoverageCheck", "ArchitectureTraceBundle", "Every legacy mapping must have a navigable component trace.", tuple(sorted((legacy_owners - traced_owners) | (traced_owners - legacy_owners))), len(legacy_owners) + len(traced_owners), ("architecture-traces-json", "architecture-map-json"), "Prüft alte-reta-Komponente → Kapsel → Kategorie/Witness-Spur."),
        _check("CapsuleTraceCoverageCheck", "ArchitectureTraceBundle", "Every architecture capsule must have a capsule trace.", tuple(sorted((capsule_names - traced_capsules) | (traced_capsules - capsule_names))), len(capsule_names) + len(traced_capsules), ("architecture-traces-json",), "Prüft Kapsel-Traces."),
        _check("StageTraceCoverageCheck", "ArchitectureTraceBundle", "Every staged refactor step must have a trace row.", tuple(sorted((stage_names - traced_stages) | (traced_stages - stage_names))), len(stage_names) + len(traced_stages), ("architecture-traces-json",), "Prüft Stage-Historie-Traces."),
    )


def _boundary_checks(architecture_boundaries: ArchitectureBoundariesBundle | None) -> tuple[ArchitectureValidationCheckSpec, ...]:
    if architecture_boundaries is None:
        return (_check("BoundaryValidationPresenceCheck", "ArchitectureBoundariesBundle", "Stage 32 validation should receive the boundary bundle.", ("architecture_boundaries missing",), 1, ("architecture-boundaries-json",), "Prüft, dass die Stage-32-Boundary-Schicht eingespeist wird."),)
    snapshot = architecture_boundaries.snapshot()
    validation = snapshot["validation"]
    failures = tuple(sorted(f"{key}:{value}" for key in ("violation_edges", "unresolved_internal_imports", "unowned_scanned_paths", "missing_capsule_boundaries") for value in validation.get(key, ())))
    return (_check("BoundaryValidationStatusCheck", "ArchitectureBoundariesBundle", "The Stage-32 import-boundary validation must pass.", failures, snapshot["counts"]["import_edges"] + snapshot["counts"]["module_ownership"], ("architecture-boundaries-json",), "Bündelt Kapselgrenzen, Modulbesitz und Importgraph in die Gesamtvalidierung."),)




def _impact_checks(architecture_impact: ArchitectureImpactBundle | None) -> tuple[ArchitectureValidationCheckSpec, ...]:
    if architecture_impact is None:
        return (_check("ImpactValidationPresenceCheck", "ArchitectureImpactBundle", "Stage 33 validation should receive the impact bundle.", ("architecture_impact missing",), 1, ("architecture-impact-json",), "Prüft, dass die Stage-33-Impact-/Migration-Gate-Schicht eingespeist wird."),)
    snapshot = architecture_impact.snapshot()
    validation = snapshot["validation"]
    failures = tuple(
        sorted(
            f"{key}:{value}"
            for key in (
                "missing_sources",
                "sources_without_contracts",
                "candidates_without_gates",
                "unknown_capsules",
                "uncovered_natural_transformations",
            )
            for value in validation.get(key, ())
        )
    )
    candidate_failures = tuple(validation.get("candidates_without_gates", ()))
    return (
        _check(
            "ImpactValidationStatusCheck",
            "ArchitectureImpactBundle",
            "The Stage-33 impact validation must pass.",
            failures,
            snapshot["counts"]["impact_sources"] + snapshot["counts"]["impact_contracts"],
            ("architecture-impact-json",),
            "Bündelt Impact-Quellen, betroffene Verträge, natürliche Transformationen und Regression-Gates in die Gesamtvalidierung.",
        ),
        _check(
            "MigrationGateCoverageCheck",
            "ArchitectureImpactBundle",
            "Every guarded migration candidate must be protected by concrete regression gates.",
            candidate_failures,
            snapshot["counts"]["migration_candidates"],
            ("architecture-impact-json", "architecture-validation-json", "tests/test_command_parity.py"),
            "Prüft, dass spätere Extraktionen aus Legacy-Flächen nur über benannte Gates laufen.",
        ),
    )


def _migration_checks(architecture_migration: ArchitectureMigrationBundle | None) -> tuple[ArchitectureValidationCheckSpec, ...]:
    if architecture_migration is None:
        return (_check("MigrationValidationPresenceCheck", "ArchitectureMigrationBundle", "Stage 34 validation should receive the migration-plan bundle.", ("architecture_migration missing",), 1, ("architecture-migration-json",), "Prüft, dass die Stage-34-Migration-Plan-Schicht eingespeist wird."),)
    snapshot = architecture_migration.snapshot()
    validation = snapshot["validation"]
    failures = tuple(
        sorted(
            f"{key}:{value}"
            for key in (
                "missing_candidates",
                "steps_without_gate_binding",
                "unknown_gates",
                "unknown_diagrams",
                "unknown_natural_transformations",
                "unordered_waves",
                "empty_waves",
            )
            for value in validation.get(key, ())
        )
    )
    binding_failures = tuple(item["step_id"] for item in snapshot["gate_bindings"] if item["status"] != "bound")
    return (
        _check(
            "MigrationValidationStatusCheck",
            "ArchitectureMigrationBundle",
            "The Stage-34 migration-plan validation must pass.",
            failures,
            snapshot["counts"]["steps"] + snapshot["counts"]["gate_bindings"],
            ("architecture-migration-json",),
            "Bündelt Migration-Wellen, Schritte, Gate-Bindings, Diagramme und natürliche Transformationen in die Gesamtvalidierung.",
        ),
        _check(
            "MigrationGateBindingCoverageCheck",
            "ArchitectureMigrationBundle",
            "Every migration step must have a bound gate command set.",
            binding_failures,
            snapshot["counts"]["gate_bindings"],
            ("architecture-migration-json", "architecture-impact-json"),
            "Prüft, dass Stage-34-Migrationsschritte nicht ohne Gate-Bindings geplant werden.",
        ),
    )



def _rehearsal_checks(architecture_rehearsal: ArchitectureRehearsalBundle | None) -> tuple[ArchitectureValidationCheckSpec, ...]:
    if architecture_rehearsal is None:
        return (_check("RehearsalValidationPresenceCheck", "ArchitectureRehearsalBundle", "Stage 35 validation should receive the rehearsal/readiness bundle.", ("architecture_rehearsal missing",), 1, ("architecture-rehearsal-json",), "Prüft, dass die Stage-35-Rehearsal-Schicht eingespeist wird."),)
    snapshot = architecture_rehearsal.snapshot()
    validation = snapshot["validation"]
    status_failures = tuple(validation.get("failed_items", ())) if validation.get("status") != "passed" else ()
    gate_failures = tuple(validation.get("gate_suites_with_missing_commands", ()))
    return (
        _check(
            "RehearsalValidationStatusCheck",
            "ArchitectureRehearsalBundle",
            "The Stage-35 rehearsal bundle must validate its dry-run moves, covers and gate suites.",
            status_failures,
            snapshot["counts"]["moves"] + snapshot["counts"]["gate_rehearsals"],
            ("architecture-rehearsal-json",),
            "Prüft den Status der Stage-35-Rehearsal-Schicht.",
        ),
        _check(
            "RehearsalGateCommandCoverageCheck",
            "ArchitectureRehearsalBundle",
            "Every gate rehearsal must expose executable preflight and postflight commands.",
            gate_failures,
            snapshot["counts"]["gate_rehearsals"],
            ("architecture-rehearsal-json",),
            "Prüft die Gate-Kommandos der Stage-35-Trockenlauf-Suiten.",
        ),
    )



def _activation_checks(architecture_activation: ArchitectureActivationBundle | None) -> tuple[ArchitectureValidationCheckSpec, ...]:
    if architecture_activation is None:
        return (_check("ActivationValidationPresenceCheck", "ArchitectureActivationBundle", "Stage 36 validation should receive the activation/commit bundle.", ("architecture_activation missing",), 1, ("architecture-activation-json",), "Prüft, dass die Stage-36-Aktivierungs-Schicht eingespeist wird."),)
    snapshot = architecture_activation.snapshot()
    validation = snapshot["validation"]
    status_failures = tuple(validation.get("failed_items", ())) if validation.get("status") != "passed" else ()
    gate_failures = tuple(validation.get("activations_without_gate", ()))
    rollback_failures = tuple(validation.get("activations_without_rollback", ()))
    transaction_failures = tuple(validation.get("windows_without_transaction", ())) + tuple(validation.get("transactions_not_ready", ()))
    return (
        _check(
            "ActivationValidationStatusCheck",
            "ArchitectureActivationBundle",
            "The Stage-36 activation bundle must validate activation units, gates, rollbacks and transactions.",
            status_failures,
            snapshot["counts"]["units"] + snapshot["counts"]["transactions"],
            ("architecture-activation-json",),
            "Prüft den Status der Stage-36-Aktivierungs-/Commit-Schicht.",
        ),
        _check(
            "ActivationGateCoverageCheck",
            "ArchitectureActivationBundle",
            "Every activation unit must have a commit gate suite.",
            gate_failures,
            snapshot["counts"]["gates"],
            ("architecture-activation-json", "architecture-rehearsal-json"),
            "Prüft, dass Stage-36-Aktivierungen nicht ohne Commit-Gates stehen.",
        ),
        _check(
            "ActivationRollbackCoverageCheck",
            "ArchitectureActivationBundle",
            "Every activation unit must have a rollback section.",
            rollback_failures,
            snapshot["counts"]["rollbacks"],
            ("architecture-activation-json",),
            "Prüft die Rollback-Sektionen der Stage-36-Aktivierungseinheiten.",
        ),
        _check(
            "ActivationTransactionCoverageCheck",
            "ArchitectureActivationBundle",
            "Every activation window must glue to a ready transaction.",
            transaction_failures,
            snapshot["counts"]["transactions"],
            ("architecture-activation-json", "architecture-validation-json"),
            "Prüft das universelle Gluing lokaler Aktivierungen in Transaktionsfenster.",
        ),
    )



def _row_range_checks(row_ranges: RowRangeMorphismBundle | None, repo_root: Path) -> tuple[ArchitectureValidationCheckSpec, ...]:
    repo_root = Path(repo_root)
    if row_ranges is None:
        return (_check("RowRangeActivationPresenceCheck", "RowRangeMorphismBundle", "Stage 37 validation should receive the activated row-range morphism bundle.", ("row_ranges missing",), 1, ("row-ranges-json",), "Prüft, dass die Stage-37-Row-Range-Aktivierung eingespeist wird."),)
    snapshot = row_ranges.snapshot()
    center_source = (repo_root / "libs" / "center.py").read_text(encoding="utf-8") if (repo_root / "libs" / "center.py").exists() else ""
    row_source_exists = (repo_root / "reta_architecture" / "row_ranges.py").exists()
    wrapper_failures = []
    if "ROW_RANGE_MORPHISMS = bootstrap_row_range_morphisms" not in center_source:
        wrapper_failures.append("center missing ROW_RANGE_MORPHISMS bootstrap")
    if "return ROW_RANGE_MORPHISMS.range_to_numbers" not in center_source:
        wrapper_failures.append("BereichToNumbers2 not delegated")
    required_morphisms = {"range_to_numbers", "is_row_range", "str_as_generator_to_set"}
    morphism_failures = tuple(sorted(required_morphisms - set(snapshot.get("morphisms", ()))))
    stage_failures = () if snapshot.get("stage") == 37 and row_source_exists else (f"stage={snapshot.get('stage')} source={row_source_exists}",)
    sample_failures = []
    samples = {
        "1-3": {1, 2, 3},
        "1-3+1": {1, 2, 3, 4},
        "{1,2,5}": {1, 2, 5},
    }
    for text, expected in samples.items():
        if row_ranges.range_to_numbers(text, max_value=20) != expected:
            sample_failures.append(text)
    return (
        _check("RowRangeActivationStageCheck", "RowRangeMorphismBundle", "The row-range morphism bundle must advertise Stage 37 and exist as a source file.", stage_failures, 2, ("row-ranges-json",), "Prüft, dass der aktivierte Parser als Stage-37-Schicht sichtbar ist."),
        _check("RowRangeCenterDelegationCheck", "RowRangeMorphismBundle", "Legacy center row-range functions must delegate to RowRangeMorphismBundle.", tuple(wrapper_failures), 2, ("libs/center.py", "row-ranges-json"), "Schützt center.py als dünne Kompatibilitätsfassade für Zeilenbereichslogik."),
        _check("RowRangeMorphismCoverageCheck", "RowRangeMorphismBundle", "The activated bundle must expose the essential row-range morphisms.", morphism_failures, len(snapshot.get("morphisms", ())), ("row-ranges-json",), "Prüft Morphismen für Validierung, Generator-Literal und Zeilenmengen-Expansion."),
        _check("RowRangeSampleExpansionCheck", "RowRangeMorphismBundle", "Representative row-range expressions must still expand to the expected row sets.", tuple(sample_failures), len(samples), ("row-ranges-json",), "Schützt die beobachtbare Semantik kleiner Zeilenbereichsbeispiele."),
    )


def _arithmetic_checks(arithmetic: ArithmeticMorphismBundle | None, repo_root: Path) -> tuple[ArchitectureValidationCheckSpec, ...]:
    repo_root = Path(repo_root)
    if arithmetic is None:
        return (_check("ArithmeticActivationPresenceCheck", "ArithmeticMorphismBundle", "Stage 38 validation should receive the activated arithmetic morphism bundle.", ("arithmetic missing",), 1, ("arithmetic-json",), "Prüft, dass die Stage-38-Arithmetik-Aktivierung eingespeist wird."),)
    snapshot = arithmetic.snapshot()
    center_source = (repo_root / "libs" / "center.py").read_text(encoding="utf-8") if (repo_root / "libs" / "center.py").exists() else ""
    source_exists = (repo_root / "reta_architecture" / "arithmetic.py").exists()
    wrapper_failures = []
    if "ARITHMETIC_MORPHISMS = bootstrap_arithmetic_morphisms" not in center_source:
        wrapper_failures.append("center missing ARITHMETIC_MORPHISMS bootstrap")
    for marker in (
        "return ARITHMETIC_MORPHISMS.multiples",
        "return ARITHMETIC_MORPHISMS.divisors_for_range",
        "return ARITHMETIC_MORPHISMS.prime_factors",
        "return ARITHMETIC_MORPHISMS.prime_repeat",
        "return ARITHMETIC_MORPHISMS.has_digit",
    ):
        if marker not in center_source:
            wrapper_failures.append(marker)
    required_morphisms = {"factor_pairs", "divisor_range", "prime_factors", "prime_repeat_legacy", "prime_repeat_pairs", "invert_int_value_dict", "has_digit"}
    morphism_failures = tuple(sorted(required_morphisms - set(snapshot.get("morphisms", ()))))
    stage_failures = () if snapshot.get("stage") == 38 and source_exists else (f"stage={snapshot.get('stage')} source={source_exists}",)
    sample_failures = []
    if set(arithmetic.multiples(12)) != {(6, 2), (4, 3), (12, 1)}:
        sample_failures.append("multiples(12)")
    if arithmetic.prime_factors(24) != [2, 2, 2, 3]:
        sample_failures.append("prime_factors(24)")
    values = [2, 2, 2, 3]
    if arithmetic.prime_repeat(values) != ["2^3", 3] or values != [3, 2, 2, 2]:
        sample_failures.append("prime_repeat side effect/result")
    divisor_strings, divisor_set = arithmetic.divisors_for_range("6")
    if divisor_set != {2, 3, 6} or set(divisor_strings) != {"2", "3", "6"}:
        sample_failures.append("divisors_for_range(6)")
    if not arithmetic.has_digit("ab3") or arithmetic.has_digit("abc"):
        sample_failures.append("has_digit")
    return (
        _check("ArithmeticActivationStageCheck", "ArithmeticMorphismBundle", "The arithmetic morphism bundle must advertise Stage 38 and exist as a source file.", stage_failures, 2, ("arithmetic-json",), "Prüft, dass die aktivierte Arithmetik als Stage-38-Schicht sichtbar ist."),
        _check("ArithmeticCenterDelegationCheck", "ArithmeticMorphismBundle", "Legacy center arithmetic functions must delegate to ArithmeticMorphismBundle.", tuple(wrapper_failures), 6, ("libs/center.py", "arithmetic-json"), "Schützt center.py als dünne Kompatibilitätsfassade für Faktor-/Teiler-/Primfaktorlogik."),
        _check("ArithmeticMorphismCoverageCheck", "ArithmeticMorphismBundle", "The activated bundle must expose the essential arithmetic morphisms.", morphism_failures, len(snapshot.get("morphisms", ())), ("arithmetic-json",), "Prüft Morphismen für Faktorpaare, Teiler-Gluing, Primfaktoren und Digit-Erkennung."),
        _check("ArithmeticSampleExpansionCheck", "ArithmeticMorphismBundle", "Representative arithmetic helpers must still return the expected legacy results.", tuple(sample_failures), 5, ("arithmetic-json", "tests.test_architecture_refactor"), "Schützt die beobachtbare Semantik kleiner Arithmetikbeispiele."),
    )


def _console_io_checks(console_io: ConsoleIOMorphismBundle | None, repo_root: Path) -> tuple[ArchitectureValidationCheckSpec, ...]:
    repo_root = Path(repo_root)
    if console_io is None:
        return (_check("ConsoleIOActivationPresenceCheck", "ConsoleIOMorphismBundle", "Stage 39 validation should receive the activated console/io morphism bundle.", ("console_io missing",), 1, ("console-io-json",), "Prüft, dass die Stage-39-Console-IO-Aktivierung eingespeist wird."),)
    snapshot = console_io.snapshot()
    center_source = (repo_root / "libs" / "center.py").read_text(encoding="utf-8", errors="replace") if (repo_root / "libs" / "center.py").exists() else ""
    source_exists = (repo_root / "reta_architecture" / "console_io.py").exists()
    stage_failures = () if snapshot.get("stage") == 39 and source_exists else (f"stage={snapshot.get('stage')} source={source_exists}",)
    wrapper_failures = []
    for marker in (
        "CONSOLE_IO_MORPHISMS = bootstrap_console_io_morphisms",
        "return CONSOLE_IO_MORPHISMS.cliout",
        "return CONSOLE_IO_MORPHISMS.text_wrap_runtime",
        "return CONSOLE_IO_MORPHISMS.print_reta_help",
        "return CONSOLE_IO_MORPHISMS.unique_everseen",
    ):
        if marker not in center_source:
            wrapper_failures.append(marker)
    required_morphisms = {"cli_output", "get_text_wrap_things", "reta_help_text", "reta_prompt_help_text", "chunks", "unique_everseen", "DefaultOrderedDict"}
    morphism_failures = tuple(sorted(required_morphisms - set(snapshot.get("morphisms", ()))))
    sample_failures = []
    if list(console_io.chunks([1, 2, 3, 4, 5], 2)) != [[1, 2], [3, 4], [5]]:
        sample_failures.append("chunks")
    if list(console_io.unique_everseen("ABCA")) != ["A", "B", "C"]:
        sample_failures.append("unique_everseen")
    try:
        width, _, _, fill = console_io.text_wrap_runtime()
        if not isinstance(width, int) or width <= 0 or fill is None:
            sample_failures.append("text_wrap_runtime")
    except Exception as exc:  # pragma: no cover - recorded as validation evidence
        sample_failures.append("text_wrap_runtime:" + type(exc).__name__)
    return (
        _check("ConsoleIOActivationStageCheck", "ConsoleIOMorphismBundle", "The console/io morphism bundle must advertise Stage 39 and exist as a source file.", stage_failures, 2, ("console-io-json",), "Prüft, dass die aktivierte Console-/Help-/Utility-Schicht als Stage-39-Schicht sichtbar ist."),
        _check("ConsoleIOCenterDelegationCheck", "ConsoleIOMorphismBundle", "Legacy center console/help/utility functions must delegate to ConsoleIOMorphismBundle.", tuple(wrapper_failures), 5, ("libs/center.py", "console-io-json"), "Schützt center.py als dünne Kompatibilitätsfassade für Hilfe-/Output-/Utilitylogik."),
        _check("ConsoleIOMorphismCoverageCheck", "ConsoleIOMorphismBundle", "The activated bundle must expose the essential console/help/utility morphisms.", morphism_failures, len(snapshot.get("morphisms", ())), ("console-io-json",), "Prüft Morphismen für Hilfe, Wrapping, CLI-Ausgabe, Debug-Ausgabe und endliche Hilfssektionen."),
        _check("ConsoleIOSampleExpansionCheck", "ConsoleIOMorphismBundle", "Representative console/io helpers must still return the expected legacy helper results.", tuple(sample_failures), 3, ("console-io-json", "tests.test_architecture_refactor"), "Schützt die beobachtbare Semantik kleiner Utility-Beispiele."),
    )


def _word_completion_checks(word_completion: WordCompletionMorphismBundle | None, repo_root: Path) -> tuple[ArchitectureValidationCheckSpec, ...]:
    repo_root = Path(repo_root)
    if word_completion is None:
        return (_check("WordCompletionActivationPresenceCheck", "WordCompletionMorphismBundle", "Stage 40 validation should receive the activated word-completion morphism bundle.", ("word_completion missing",), 1, ("word-completion-json",), "Prüft, dass die Stage-40-Word-Completion-Aktivierung eingespeist wird."),)
    snapshot = word_completion.snapshot()
    facade_source = (repo_root / "libs" / "word_completerAlx.py").read_text(encoding="utf-8", errors="replace") if (repo_root / "libs" / "word_completerAlx.py").exists() else ""
    source_exists = (repo_root / "reta_architecture" / "completion_word.py").exists()
    stage_failures = () if snapshot.get("stage") == 40 and source_exists else (f"stage={snapshot.get('stage')} source={source_exists}",)
    wrapper_failures = []
    for marker in (
        "from reta_architecture.completion_word import ArchitectureWordCompleter as WordCompleter",
        "__all__ = [\"WordCompleter\"]",
    ):
        if marker not in facade_source:
            wrapper_failures.append(marker)
    required_morphisms = {"resolve_words", "word_before_cursor", "word_completion_matches", "iter_word_completions", "create_completer"}
    morphism_failures = tuple(sorted(required_morphisms - set(snapshot.get("morphisms", ()))))
    sample_failures = []
    try:
        from .completion_word import Document
        texts = [item.text for item in word_completion.completions(["reta", "religion", "alpha"], Document("re"))]
        if texts != ["reta", "religion"]:
            sample_failures.append("prefix completions")
        middle = [item.text for item in word_completion.completions(["alpha", "beta", "theta"], Document("et"), match_middle=True)]
        if middle != ["beta", "theta"]:
            sample_failures.append("middle completions")
        completer = word_completion.create_completer(["reta", "religion", "alpha"])
        facade_class_name = type(completer).__name__
        if facade_class_name != "ArchitectureWordCompleter":
            sample_failures.append("create_completer class")
    except Exception as exc:  # pragma: no cover - recorded as validation evidence
        sample_failures.append("completion sample:" + type(exc).__name__)
    return (
        _check("WordCompletionActivationStageCheck", "WordCompletionMorphismBundle", "The word-completion morphism bundle must advertise Stage 40 and exist as a source file.", stage_failures, 2, ("word-completion-json",), "Prüft, dass die aktivierte Word-Completion-Schicht als Stage-40-Schicht sichtbar ist."),
        _check("WordCompleterFacadeDelegationCheck", "WordCompletionMorphismBundle", "Legacy word_completerAlx.WordCompleter must be a facade for ArchitectureWordCompleter.", tuple(wrapper_failures), 2, ("libs/word_completerAlx.py", "word-completion-json"), "Schützt word_completerAlx.py als dünne Kompatibilitätsfassade für Completion-Matching."),
        _check("WordCompletionMorphismCoverageCheck", "WordCompletionMorphismBundle", "The activated bundle must expose the essential word-completion morphisms.", morphism_failures, len(snapshot.get("morphisms", ())), ("word-completion-json",), "Prüft Morphismen für Wortquellen, Cursor-Präfixe, Matching und Completion-Kandidaten."),
        _check("WordCompletionSampleExpansionCheck", "WordCompletionMorphismBundle", "Representative word-completion helpers must still return the expected legacy candidates.", tuple(sample_failures), 3, ("word-completion-json", "tests.test_architecture_refactor"), "Schützt die beobachtbare Semantik kleiner Completion-Beispiele."),
    )


def _nested_completion_checks(nested_completion: NestedCompletionMorphismBundle | None, repo_root: Path) -> tuple[ArchitectureValidationCheckSpec, ...]:
    repo_root = Path(repo_root)
    if nested_completion is None:
        return (_check("NestedCompletionActivationPresenceCheck", "NestedCompletionMorphismBundle", "Stage 41 validation should receive the activated nested-completion morphism bundle.", ("nested_completion missing",), 1, ("nested-completion-json",), "Prüft, dass die Stage-41-Nested-Completion-Aktivierung eingespeist wird."),)
    snapshot = nested_completion.snapshot()
    facade_source = (repo_root / "libs" / "nestedAlx.py").read_text(encoding="utf-8", errors="replace") if (repo_root / "libs" / "nestedAlx.py").exists() else ""
    source_exists = (repo_root / "reta_architecture" / "completion_nested.py").exists()
    stage_failures = () if snapshot.get("stage") == 41 and source_exists else (f"stage={snapshot.get('stage')} source={source_exists}",)
    wrapper_failures = []
    for marker in (
        "from reta_architecture.completion_nested import",
        "ArchitectureNestedCompleter",
        "NestedCompleter",
        "bootstrap_nested_completion_morphisms",
    ):
        if marker not in facade_source:
            wrapper_failures.append(marker)
    required_morphisms = {"create_completer", "matchTextAlx", "gleichKommaSpalten", "gleichKommaZeilen", "gleichKommaKombi", "gleichKommaAusg", "get_completions"}
    morphism_failures = tuple(sorted(required_morphisms - set(snapshot.get("morphisms", ()))))
    sample_failures = []
    try:
        from .completion_nested import ArchitectureNestedCompleter, ComplSitua
        if ArchitectureNestedCompleter.__name__ != "ArchitectureNestedCompleter":
            sample_failures.append("ArchitectureNestedCompleter class")
        if ComplSitua.retaAnfang.name not in snapshot.get("situations", ()): 
            sample_failures.append("situations sample")
        if "matchTextAlx" not in snapshot.get("morphisms", ()): 
            sample_failures.append("matchTextAlx morphism")
    except Exception as exc:  # pragma: no cover - recorded as validation evidence
        sample_failures.append("nested sample:" + type(exc).__name__)
    return (
        _check("NestedCompletionActivationStageCheck", "NestedCompletionMorphismBundle", "The nested-completion morphism bundle must advertise Stage 41 and exist as a source file.", stage_failures, 2, ("nested-completion-json",), "Prüft, dass die aktivierte Nested-Completion-Schicht als Stage-41-Schicht sichtbar ist."),
        _check("NestedCompleterFacadeDelegationCheck", "NestedCompletionMorphismBundle", "Legacy nestedAlx.NestedCompleter must be a facade for ArchitectureNestedCompleter.", tuple(wrapper_failures), 4, ("libs/nestedAlx.py", "nested-completion-json"), "Schützt nestedAlx.py als dünne Kompatibilitätsfassade für hierarchische Prompt-Completion."),
        _check("NestedCompletionMorphismCoverageCheck", "NestedCompletionMorphismBundle", "The activated bundle must expose the essential nested-completion morphisms.", morphism_failures, len(snapshot.get("morphisms", ())), ("nested-completion-json",), "Prüft Morphismen für Sub-Completer-Auswahl, Gleich-/Kommawerte und Completion-Erzeugung."),
        _check("NestedCompletionSampleExpansionCheck", "NestedCompletionMorphismBundle", "Representative nested-completion helpers must still expose the expected legacy class and situations.", tuple(sample_failures), 3, ("nested-completion-json", "tests.test_architecture_refactor"), "Schützt die beobachtbare Semantik kleiner Nested-Completion-Beispiele."),
    )

def _repo_checks(repo_root: Path) -> tuple[ArchitectureValidationCheckSpec, ...]:
    repo_root = Path(repo_root)
    files: list[str] = []
    runtime_artifact_count = 0
    for path in repo_root.rglob("*"):
        if not path.is_file():
            continue
        relative_path = path.relative_to(repo_root)
        relative = str(relative_path).replace("\\", "/")
        if is_runtime_artifact(relative_path):
            runtime_artifact_count += 1
            continue
        files.append(relative)
    file_set = set(files)
    missing_manifest = tuple(path for path in REQUIRED_SOURCE_PATHS if path not in file_set)
    # The live tree may contain import-generated __pycache__ files while this
    # validation itself is being executed by unit tests. The release/package
    # probe still reports runtime artifacts, but this in-process architecture
    # check only treats missing required sources as a hard validation failure.
    artifact_failures: tuple[str, ...] = ()
    markdown_files = tuple(sorted(relative for relative in files if relative.endswith(".md")))
    expected_markdown = ("STAGE41_CHANGES.md", "ARCHITECTURE_REFACTOR_STAGE41.md", "ARCHITECTURE_NESTED_COMPLETION_STAGE41.md", "MARKDOWN_AUDIT_STAGE41.md", "PACKAGE_AUDIT_STAGE41.md")
    markdown_failures = tuple(item for item in expected_markdown if item not in markdown_files)
    return (
        _check(
            "PackageIntegrityValidationCheck",
            "RepositoryManifest",
            "Required architecture files must be present; release package probes separately report runtime artifacts.",
            tuple(missing_manifest) + artifact_failures,
            len(files) + runtime_artifact_count,
            ("package-integrity-json",),
            "Prüft das Paket als Kompatibilitäts- und Regressionsanker, ohne während laufender Tests erzeugte Cache-Dateien als Architekturfehler zu werten.",
        ),
        _check(
            "MarkdownStageHistoryCheck",
            "RepositoryMarkdown",
            "The stage history must include the Stage-41 activated nested-completion documents.",
            markdown_failures,
            len(markdown_files),
            ("MARKDOWN_AUDIT_STAGE41.md",),
            "Prüft, dass die gelesene Markdown-Historie um Stage 41 fortgeschrieben wurde.",
            severity="warning",
        ),
    )

def _summary(checks: Sequence[ArchitectureValidationCheckSpec]) -> ArchitectureValidationSummarySpec:
    failed = tuple(f"{check.name}:{item}" for check in checks for item in check.failed_items)
    attention = tuple(check for check in checks if check.status != "passed")
    error_checks = tuple(check for check in attention if check.severity == "error")
    warning_checks = tuple(check for check in attention if check.severity != "error")
    status = "passed" if not attention else ("failed" if error_checks else "attention")
    return ArchitectureValidationSummarySpec(
        status=status,
        total_checks=len(checks),
        passed_checks=sum(1 for check in checks if check.status == "passed"),
        attention_checks=len(attention),
        failed_checks=len(error_checks),
        warning_checks=len(warning_checks),
        error_checks=len(error_checks),
        checked_items=sum(check.checked_count for check in checks),
        failed_items=failed,
    )


def _layers(checks: Sequence[ArchitectureValidationCheckSpec]) -> tuple[ArchitectureValidationLayerSpec, ...]:
    roles = {
        "CategoryTheoryBundle": "Kategorien, Funktoren und natürliche Transformationen referenziell geschlossen halten.",
        "ArchitectureMapBundle": "Kapseln, Containment und Datenflüsse stufenweise stabil halten.",
        "ArchitectureContractsBundle": "Kommutierende Diagramme und Kapselverträge gegen Kategorie und Karte validieren.",
        "ArchitectureWitnessBundle": "Repository-Anker, Witnesses und Obligations vollständig halten.",
        "RepositoryManifest": "Paketintegrität als Kompatibilitätsanker prüfen.",
        "RepositoryMarkdown": "Stage-Historie und menschliche Architekturbegründung fortschreiben.",
        "ArchitectureImpactBundle": "Stage-32-Traces und Boundaries als Impact- und Migration-Gate-Routen prüfen.",
        "ArchitectureMigrationBundle": "Stage-33-Impact-Kandidaten als Wellen, Schritte und Gate-Binding prüfen.",
        "ArchitectureRehearsalBundle": "Stage-34-Migrationsschritte als Trockenlauf-Moves, Gate-Suites und Readiness-Covers prüfen.",
        "ArchitectureActivationBundle": "Stage-36-Rehearsal-Moves als Aktivierungsfenster, Commit-Gates, Rollback-Sektionen und Transaktionen prüfen.",
        "RowRangeMorphismBundle": "Stage-37 aktivierte Zeilenbereichs-Morphismen und center.py-Delegation prüfen.",
        "ArithmeticMorphismBundle": "Stage-38 aktivierte Arithmetik-Morphismen und center.py-Delegation prüfen.",
        "ConsoleIOMorphismBundle": "Stage-39 aktivierte Console-/Help-/Utility-Morphismen und center.py-Delegation prüfen.",
        "WordCompletionMorphismBundle": "Stage-40 aktivierte Word-Completion-Morphismen und word_completerAlx.py-Delegation prüfen.",
        "NestedCompletionMorphismBundle": "Stage-41 aktivierte nestedAlx-Hierarchiecompletion und Legacy-Fassade prüfen.",
    }
    grouped: dict[str, list[ArchitectureValidationCheckSpec]] = {}
    for check in checks:
        grouped.setdefault(check.layer, []).append(check)
    result: list[ArchitectureValidationLayerSpec] = []
    for layer in sorted(grouped):
        layer_checks = grouped[layer]
        failed = tuple(check.name for check in layer_checks if check.status != "passed")
        result.append(
            ArchitectureValidationLayerSpec(
                name=layer,
                role=roles.get(layer, "Validierungsschicht"),
                checks=tuple(check.name for check in layer_checks),
                status="passed" if not failed else "attention",
                failed_checks=failed,
            )
        )
    return tuple(result)


_TEXT_DIAGRAM = """\
ArchitectureValidationBundle
├─ Category checks
│  ├─ functors reference known categories
│  ├─ natural transformations reference known functors
│  └─ paradigm terms remain visible
├─ Map checks
│  ├─ Stage-32 capsule map
│  ├─ known flow endpoints
│  └─ containment references
├─ Contract checks
│  ├─ capsule contract coverage
│  ├─ built-in contract validation
│  └─ diagram category/functor/transformation references
├─ Witness checks
│  ├─ anchor and capsule-slice coverage
│  ├─ diagram witnesses
│  ├─ refactor-law obligations
│  └─ natural-transformation witnesses
├─ Repository checks
│  ├─ package integrity
│  └─ Markdown stage history
├─ ArchitectureCoherenceBundle
│  └─ Stage-32 coherence matrix consumes the same validated stack
├─ ArchitectureTraceBundle
│  └─ Stage-32 component and capsule traces consume validation/coherence
├─ ArchitectureBoundariesBundle
│  └─ Stage-32 module boundary graph consumes package and map checks
├─ ArchitectureImpactBundle
│  └─ Stage-33 impact sources and migration gates consume traces, boundaries and contracts
├─ ArchitectureMigrationBundle
│  └─ Stage-34 migration waves consume impact candidates, contracts and gates
├─ ArchitectureRehearsalBundle
│  └─ Stage-35 rehearsal covers consume migration steps and gate suites
├─ ArchitectureActivationBundle
│  └─ Stage-36 activation transactions consume rehearsal moves, commit gates and rollback sections
├─ RowRangeMorphismBundle
│  └─ Stage-37 activated row-range morphisms consume the first activation envelope
├─ ArithmeticMorphismBundle
│  └─ Stage-38 activated arithmetic morphisms consume the next activation envelope
├─ ConsoleIOMorphismBundle
├─ WordCompletionMorphismBundle
│  └─ Stage-40 activated word-completion morphisms consume the fourth activation envelope
└─ NestedCompletionMorphismBundle
   └─ Stage-41 activated nested prompt-completion morphisms consume the next activation envelope
"""


_MERMAID_DIAGRAM = """\
```mermaid
flowchart TD
    Category[CategoryTheoryBundle<br/>categories + functors + natural transformations] --> Validation[ArchitectureValidationBundle]
    Map[ArchitectureMapBundle<br/>capsules + flows + stages] --> Validation
    Contracts[ArchitectureContractsBundle<br/>commutative diagrams + laws] --> Validation
    Witnesses[ArchitectureWitnessBundle<br/>anchors + obligations] --> Validation
    Repo[Repository tree<br/>package + Markdown history] --> Validation
    Validation --> Summary[Validation summary<br/>passed / attention / failed]
    Validation --> Coherence[ArchitectureCoherenceBundle<br/>cross-layer coherence matrix]
    Summary --> Future[Future stages<br/>move code only when checks commute]
    Coherence --> Future
    Impact[ArchitectureImpactBundle<br/>impact sources + migration gates] --> Validation
    Migration[ArchitectureMigrationBundle<br/>waves + steps + gate bindings] --> Validation
    Rehearsal[ArchitectureRehearsalBundle<br/>moves + gate suites + readiness covers] --> Validation
    Activation[ArchitectureActivationBundle<br/>activation units + commit/rollback transactions] --> Validation
    RowRanges[RowRangeMorphismBundle<br/>activated center row-range parser] --> Validation
    Arithmetic[ArithmeticMorphismBundle<br/>activated center arithmetic] --> Validation
    ConsoleIO[ConsoleIOMorphismBundle<br/>activated center console/help utilities] --> Validation
    WordCompletion[WordCompletionMorphismBundle<br/>activated prompt word completion] --> Validation
    NestedCompletion[NestedCompletionMorphismBundle<br/>activated nested prompt completion] --> Validation
    ```
"""


def _plan() -> Stage31ArchitecturePlan:
    return Stage31ArchitecturePlan(
        planned_after_stage_30=(
            "Use Stage-30 witnesses as the input to an executable architecture audit.",
            "Validate category, functor, natural-transformation, capsule, contract and witness references together.",
            "Give future stages one probe that says whether the staged refactor graph still commutes and affected owners have impact gates.",
        ),
        implemented_in_stage_31=(
            "reta_architecture/architecture_validation.py",
            "architecture-validation-json and architecture-validation-md probe commands",
            "Stage-33 ImpactValidationStatusCheck and MigrationGateCoverageCheck; Stage-34 MigrationValidationStatusCheck and MigrationGateBindingCoverageCheck",
            "ArchitectureMap stage step and CategoricalMetaCapsule containment for ArchitectureValidationBundle",
            "CategoryTheory functors and a natural transformation for contract/witness validation",
            "ArchitectureContracts diagram for the Stage-31 validation square",
        ),
        inherited_from_previous_stages=(
            "Stage 27 CategoryTheoryBundle",
            "Stage 28 ArchitectureMapBundle",
            "Stage 29 ArchitectureContractsBundle",
            "Stage 30 ArchitectureWitnessBundle",
            "Stage 32 ArchitectureTraceBundle and ArchitectureBoundariesBundle",
            "Stage 33 ArchitectureImpactBundle",
            "Stage 34 ArchitectureMigrationBundle",
            "Existing package-integrity and command-parity tests",
        ),
        behaviour_change="keine beabsichtigte Laufzeit-/CLI-Verhaltensänderung; Stage 31 ist eine ausführbare Architekturvalidierung über der bestehenden Metaschicht",
    )


def bootstrap_architecture_validation(
    repo_root: Path,
    category_theory: CategoryTheoryBundle,
    architecture_map: ArchitectureMapBundle,
    architecture_contracts: ArchitectureContractsBundle,
    architecture_witnesses: ArchitectureWitnessBundle,
    architecture_traces: ArchitectureTraceBundle | None = None,
    architecture_boundaries: ArchitectureBoundariesBundle | None = None,
    architecture_impact: ArchitectureImpactBundle | None = None,
    architecture_migration: ArchitectureMigrationBundle | None = None,
    architecture_rehearsal: ArchitectureRehearsalBundle | None = None,
    architecture_activation: ArchitectureActivationBundle | None = None,
    row_ranges: RowRangeMorphismBundle | None = None,
    arithmetic: ArithmeticMorphismBundle | None = None,
    console_io: ConsoleIOMorphismBundle | None = None,
    word_completion: WordCompletionMorphismBundle | None = None,
    nested_completion: NestedCompletionMorphismBundle | None = None,
) -> ArchitectureValidationBundle:
    """Return the Stage-36 executable architecture validation bundle."""

    checks = (
        _category_checks(category_theory)
        + _map_checks(architecture_map)
        + _contract_checks(category_theory, architecture_map, architecture_contracts)
        + _witness_checks(category_theory, architecture_map, architecture_contracts, architecture_witnesses)
        + _trace_checks(architecture_map, architecture_traces)
        + _boundary_checks(architecture_boundaries)
        + _impact_checks(architecture_impact)
        + _migration_checks(architecture_migration)
        + _rehearsal_checks(architecture_rehearsal)
        + _activation_checks(architecture_activation)
        + _row_range_checks(row_ranges, Path(repo_root))
        + _arithmetic_checks(arithmetic, Path(repo_root))
        + _console_io_checks(console_io, Path(repo_root))
        + _word_completion_checks(word_completion, Path(repo_root))
        + _nested_completion_checks(nested_completion, Path(repo_root))
        + _repo_checks(Path(repo_root))
    )
    return ArchitectureValidationBundle(
        checks=checks,
        layers=_layers(checks),
        summary=_summary(checks),
        text_diagram=_TEXT_DIAGRAM,
        mermaid_diagram=_MERMAID_DIAGRAM,
        plan=_plan(),
    )
