from __future__ import annotations

"""Stage-31 coherence matrix for the Reta architecture.

Stage 27 named categories, functors and natural transformations.  Stage 28
arranged them as capsules and architecture flows.  Stage 29 stated contracts as
commutative diagrams and laws.  Stage 30 connected those contracts to concrete
repository witnesses.

Stage 31 closes that stack into a coherence matrix: every capsule, route and
natural transformation can be traced through the categorical layer, the map, the
contracts and the witness layer.  This module is metadata-only; it does not
change CLI, prompt, table generation or rendering behaviour.
"""

from dataclasses import dataclass
from typing import Mapping, Sequence

from .architecture_contracts import ArchitectureContractsBundle
from .architecture_map import ArchitectureMapBundle
from .architecture_witnesses import ArchitectureWitnessBundle
from .category_theory import CategoryTheoryBundle


@dataclass(frozen=True)
class CapsuleCoherenceSpec:
    """One capsule traced through category, functor, contract and witness layers."""

    capsule: str
    category: str
    functors: Sequence[str]
    natural_transformations: Sequence[str]
    diagrams: Sequence[str]
    laws: Sequence[str]
    witness_slice: str
    code_owners: Sequence[str]
    stage_span: str
    coherence_reading: str

    def snapshot(self) -> dict:
        return {
            "capsule": self.capsule,
            "category": self.category,
            "functors": list(self.functors),
            "natural_transformations": list(self.natural_transformations),
            "diagrams": list(self.diagrams),
            "laws": list(self.laws),
            "witness_slice": self.witness_slice,
            "code_owners": list(self.code_owners),
            "stage_span": self.stage_span,
            "coherence_reading": self.coherence_reading,
        }


@dataclass(frozen=True)
class FunctorialRouteSpec:
    """One architecture-map flow classified as a functorial or naturality route."""

    source_capsule: str
    target_capsule: str
    morphism: str
    functor_or_transformation: str
    categorical_kind: str
    contract_diagrams: Sequence[str]
    witness_diagrams: Sequence[str]
    code_owner: str
    status: str
    reading: str

    def snapshot(self) -> dict:
        return {
            "source_capsule": self.source_capsule,
            "target_capsule": self.target_capsule,
            "morphism": self.morphism,
            "functor_or_transformation": self.functor_or_transformation,
            "categorical_kind": self.categorical_kind,
            "contract_diagrams": list(self.contract_diagrams),
            "witness_diagrams": list(self.witness_diagrams),
            "code_owner": self.code_owner,
            "status": self.status,
            "reading": self.reading,
        }


@dataclass(frozen=True)
class NaturalityCoherenceSpec:
    """Coherence status for one named natural transformation."""

    transformation: str
    source_functor: str
    target_functor: str
    component_count: int
    diagrams: Sequence[str]
    capsules: Sequence[str]
    witness_status: str
    status: str
    naturality_condition: str

    def snapshot(self) -> dict:
        return {
            "transformation": self.transformation,
            "source_functor": self.source_functor,
            "target_functor": self.target_functor,
            "component_count": self.component_count,
            "diagrams": list(self.diagrams),
            "capsules": list(self.capsules),
            "witness_status": self.witness_status,
            "status": self.status,
            "naturality_condition": self.naturality_condition,
        }


@dataclass(frozen=True)
class LawCoherenceSpec:
    """Coherence status for one refactor law."""

    law: str
    protected_capsules: Sequence[str]
    diagrams: Sequence[str]
    obligation_present: bool
    status: str
    reading: str

    def snapshot(self) -> dict:
        return {
            "law": self.law,
            "protected_capsules": list(self.protected_capsules),
            "diagrams": list(self.diagrams),
            "obligation_present": self.obligation_present,
            "status": self.status,
            "reading": self.reading,
        }


@dataclass(frozen=True)
class CoherenceValidationSpec:
    """Cross-layer validation result for Stage 31."""

    status: str
    missing_capsule_contracts: Sequence[str]
    missing_capsule_witnesses: Sequence[str]
    unresolved_categories: Sequence[str]
    unresolved_functors: Sequence[str]
    unresolved_natural_transformations: Sequence[str]
    routes_without_known_functor_or_transformation: Sequence[str]
    routes_without_contract: Sequence[str]
    routes_without_witness: Sequence[str]
    transformations_without_witness: Sequence[str]
    laws_without_obligation: Sequence[str]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "missing_capsule_contracts": list(self.missing_capsule_contracts),
            "missing_capsule_witnesses": list(self.missing_capsule_witnesses),
            "unresolved_categories": list(self.unresolved_categories),
            "unresolved_functors": list(self.unresolved_functors),
            "unresolved_natural_transformations": list(self.unresolved_natural_transformations),
            "routes_without_known_functor_or_transformation": list(self.routes_without_known_functor_or_transformation),
            "routes_without_contract": list(self.routes_without_contract),
            "routes_without_witness": list(self.routes_without_witness),
            "transformations_without_witness": list(self.transformations_without_witness),
            "laws_without_obligation": list(self.laws_without_obligation),
        }


@dataclass(frozen=True)
class Stage31CoherencePlan:
    """Bridge from Stage-30 witnesses to Stage-31 coherence."""

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
class ArchitectureCoherenceBundle:
    """Inspectable Stage-31 coherence matrix over the categorical architecture stack."""

    capsule_coherence: Sequence[CapsuleCoherenceSpec]
    functorial_routes: Sequence[FunctorialRouteSpec]
    naturality_coherence: Sequence[NaturalityCoherenceSpec]
    law_coherence: Sequence[LawCoherenceSpec]
    validation: CoherenceValidationSpec
    text_diagram: str
    mermaid_diagram: str
    plan: Stage31CoherencePlan

    def capsule_named(self, capsule: str) -> CapsuleCoherenceSpec:
        for item in self.capsule_coherence:
            if item.capsule == capsule:
                return item
        raise KeyError(f"Unknown coherent capsule: {capsule}")

    def route_for(self, source: str, target: str, name: str | None = None) -> FunctorialRouteSpec:
        for item in self.functorial_routes:
            if item.source_capsule == source and item.target_capsule == target:
                if name is None or item.functor_or_transformation == name:
                    return item
        raise KeyError(f"Unknown functorial route: {source} -> {target} ({name or '*'})")

    def naturality_named(self, transformation: str) -> NaturalityCoherenceSpec:
        for item in self.naturality_coherence:
            if item.transformation == transformation:
                return item
        raise KeyError(f"Unknown coherent natural transformation: {transformation}")

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "stage": 41,
            "purpose": "Kohärenzmatrix über Kategorien, Funktoren, natürliche Transformationen, Kapseln, Verträge, Witnesses, Traces, Boundaries, Impact-Gates und Stage-34-Migrationsplanung, Stage-35-Rehearsal und Stage-36-Aktivierung.",
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
                "coherence",
                "trace",
                "boundary",
                "impact",
                "migration_gate",
                "migration_plan",
                "readiness_gate",
                "activation_commit",
                "rollback_section",
                "activated_runtime_morphism",
                "activated_word_completion",
                "activated_console_io",
                "activated_nested_completion",
            ],
            "counts": {
                "capsule_coherence": len(self.capsule_coherence),
                "functorial_routes": len(self.functorial_routes),
                "naturality_coherence": len(self.naturality_coherence),
                "law_coherence": len(self.law_coherence),
            },
            "validation": self.validation.snapshot(),
            "capsule_coherence": [item.snapshot() for item in self.capsule_coherence],
            "functorial_routes": [item.snapshot() for item in self.functorial_routes],
            "naturality_coherence": [item.snapshot() for item in self.naturality_coherence],
            "law_coherence": [item.snapshot() for item in self.law_coherence],
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
            "plan": self.plan.snapshot(),
        }


def _cap(
    capsule: str,
    category: str,
    functors: Sequence[str],
    natural_transformations: Sequence[str],
    diagrams: Sequence[str],
    laws: Sequence[str],
    witness_slice: str,
    code_owners: Sequence[str],
    stage_span: str,
    coherence_reading: str,
) -> CapsuleCoherenceSpec:
    return CapsuleCoherenceSpec(
        capsule=capsule,
        category=category,
        functors=tuple(functors),
        natural_transformations=tuple(natural_transformations),
        diagrams=tuple(diagrams),
        laws=tuple(laws),
        witness_slice=witness_slice,
        code_owners=tuple(code_owners),
        stage_span=stage_span,
        coherence_reading=coherence_reading,
    )


def _capsule_coherence() -> tuple[CapsuleCoherenceSpec, ...]:
    return (
        _cap(
            "RetaArchitectureRoot",
            "CommutativeArchitectureContractCategory",
            ("ArchitectureRuntimeFunctor", "ArchitectureMapToContractFunctor"),
            ("ContractedNaturalityTransformation", "LegacyToArchitectureTransformation"),
            ("ArchitectureMapContractReflectionTriangle", "LegacyArchitectureCompatibilitySquare"),
            ("LegacyCompatibilityNaturalityLaw",),
            "RetaArchitectureRoot",
            ("reta_architecture/facade.py", "reta_architecture/architecture_map.py"),
            "Stages 1-31",
            "Die Root-Fassade ist kohärent, wenn Snapshot, Karte, Verträge und Witness-Matrix denselben Kapselbaum beschreiben.",
        ),
        _cap(
            "SchemaTopologyCapsule",
            "OpenRetaContextCategory",
            ("SchemaToTopologyFunctor", "ArchitectureMapToContractFunctor"),
            ("ContractedNaturalityTransformation",),
            ("ArchitectureMapContractReflectionTriangle",),
            ("ContextRefinementCompositionLaw",),
            "SchemaTopologyCapsule",
            ("reta_architecture/schema.py", "reta_architecture/topology.py", "i18n/words_context.py"),
            "Stages 1-4",
            "Sprache, Parameter und Scopes werden als offene Kontexte gelesen und müssen bei Verfeinerung funktoriell stabil bleiben.",
        ),
        _cap(
            "LocalSectionCapsule",
            "LocalSectionCategory",
            ("LocalDataPresheafFunctor", "RawCommandPresheafFunctor"),
            ("PresheafToSheafGluingTransformation",),
            ("PresheafSheafGluingSquare",),
            ("PresheafRestrictionLaw",),
            "LocalSectionCapsule",
            ("reta_architecture/presheaves.py", "csv/*.csv", "doc/*.md"),
            "Stages 1, 13, 19, 28",
            "CSV-, Dokument- und Prompt-Rohstücke sind lokale Sektionen; Restriktion vor/nach Gluing muss denselben Kontext respektieren.",
        ),
        _cap(
            "SemanticSheafCapsule",
            "CanonicalSemanticSheafCategory",
            ("CanonicalParameterSheafFunctor", "GluedSemanticSheafFunctor"),
            ("RawToCanonicalParameterTransformation", "PresheafToSheafGluingTransformation"),
            ("RawCommandNaturalitySquare", "PresheafSheafGluingSquare"),
            ("SheafGluingUniquenessLaw", "RawCanonicalNaturalityLaw"),
            "SemanticSheafCapsule",
            ("reta_architecture/sheaves.py", "reta_architecture/semantics_builder.py"),
            "Stages 1-5, 27-31",
            "Kanonische Parametersemantik ist die geklebte Garbensemantik; Alias- und Prägarbenpfade müssen kommutieren.",
        ),
        _cap(
            "InputPromptCapsule",
            "ActivatedArithmeticCategory",
            ("RawCommandPresheafFunctor", "RowRangeActivationFunctor", "CenterRowRangeCompatibilityFunctor", "RowRangeInputFunctor", "RowRangeValidationFunctor", "ArithmeticActivationFunctor", "CenterArithmeticCompatibilityFunctor", "ArithmeticRowRangeGluingFunctor", "ArithmeticValidationFunctor", "ConsoleIOActivationFunctor", "CenterConsoleIOCompatibilityFunctor", "WordCompletionActivationFunctor", "LegacyWordCompleterCompatibilityFunctor", "WordCompletionPromptFunctor", "WordCompletionValidationFunctor", "NestedCompletionActivationFunctor", "LegacyNestedCompleterCompatibilityFunctor", "NestedCompletionPromptFunctor", "NestedCompletionValidationFunctor"),
            ("RawToCanonicalParameterTransformation", "CenterRowRangeToArchitectureTransformation", "RowRangeValidationTransformation", "CenterArithmeticToArchitectureTransformation", "ArithmeticRowRangeGluingTransformation", "CenterConsoleIOToArchitectureTransformation", "WordCompleterToArchitectureTransformation", "WordCompletionValidationTransformation", "NestedCompleterToArchitectureTransformation", "NestedCompletionValidationTransformation"),
            ("RawCommandNaturalitySquare", "CenterRowRangeCompatibilitySquare", "RowRangeValidationSquare", "CenterArithmeticCompatibilitySquare", "ArithmeticRowRangeGluingSquare", "CenterConsoleIOCompatibilitySquare", "WordCompleterCompatibilitySquare", "WordCompletionValidationSquare", "NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare"),
            ("RawCanonicalNaturalityLaw", "ActivatedRowRangeLaw", "ActivatedArithmeticLaw", "ActivatedConsoleIOLaw", "ActivatedWordCompletionLaw", "ActivatedNestedCompletionLaw"),
            "InputPromptCapsule",
            ("retaPrompt.py", "libs/center.py", "reta_architecture/row_ranges.py", "reta_architecture/arithmetic.py", "reta_architecture/console_io.py", "reta_architecture/prompt_language.py", "reta_architecture/prompt_runtime.py", "reta_architecture/completion_word.py", "libs/word_completerAlx.py", "reta_architecture/completion_nested.py", "libs/nestedAlx.py"),
            "Stages 4, 6-12, 37-41",
            "Raw CLI/Prompt-Text bleibt lokal; Stage 37 aktiviert Zeilenbereiche, Stage 38 center-Arithmetik, Stage 39 center-Console-/Help-/Utilityfunktionen und Stage 40 word-completerAlx und Stage 41 nestedAlx als Architektur-Besitz.",
        ),
        _cap(
            "WorkflowGluingCapsule",
            "UniversalConstructionCategory",
            ("TableGenerationGluingFunctor",),
            ("TableGenerationGluingTransformation",),
            ("UniversalWorkflowTableSquare",),
            ("WorkflowUniversalConstructionLaw",),
            "WorkflowGluingCapsule",
            ("reta_architecture/column_selection.py", "reta_architecture/parameter_runtime.py", "reta_architecture/program_workflow.py", "reta_architecture/table_generation.py"),
            "Stages 13-15",
            "Parameter-Runtime, Spaltenauswahl und Tabellenbau sind die universelle Konstruktion zwischen Semantikgarbe und globaler Tabellensektion.",
        ),
        _cap(
            "TableCoreCapsule",
            "TableSectionCategory",
            ("MutableTableRuntimeFunctor", "ExplicitTableStateFunctor", "TableGenerationGluingFunctor"),
            ("TableRuntimeToStateSectionsTransformation", "TableGenerationGluingTransformation"),
            ("RuntimeStateProjectionSquare", "UniversalWorkflowTableSquare"),
            ("RuntimeStateProjectionLaw", "WorkflowUniversalConstructionLaw"),
            "TableCoreCapsule",
            ("reta_architecture/table_runtime.py", "reta_architecture/table_state.py", "reta_architecture/table_preparation.py"),
            "Stages 16, 22-26",
            "Tables ist die globale Tabellensektion; Stage 31 prüft, dass mutable Runtime und explizite StateSections dieselbe Semantik tragen.",
        ),
        _cap(
            "GeneratedRelationCapsule",
            "GeneratedColumnEndomorphismCategory",
            ("GeneratedColumnEndofunctorFamily",),
            ("GeneratedColumnsSheafSyncTransformation",),
            ("GeneratedColumnStateSyncSquare",),
            ("GeneratedColumnStateSyncLaw",),
            "GeneratedRelationCapsule",
            ("reta_architecture/generated_columns.py", "reta_architecture/meta_columns.py", "reta_architecture/concat_csv.py", "reta_architecture/combi_join.py"),
            "Stages 17-21",
            "Generated-, Meta-, CSV- und Kombi-Operationen sind Endomorphismen auf Tabellen-/Relationssektionen und müssen ihren Zustand synchronisieren.",
        ),
        _cap(
            "OutputRenderingCapsule",
            "OutputFormatCategory",
            ("OutputRenderingFunctorFamily", "NormalizedOutputFunctor", "ConsoleIOOutputRenderingFunctor", "ConsoleIOValidationFunctor"),
            ("RenderedOutputNormalizationTransformation", "ConsoleIOOutputValidationTransformation", "CenterConsoleIOToArchitectureTransformation", "WordCompleterToArchitectureTransformation", "WordCompletionValidationTransformation", "NestedCompleterToArchitectureTransformation", "NestedCompletionValidationTransformation"),
            ("RenderedOutputParitySquare", "ConsoleIOOutputValidationSquare", "CenterConsoleIOCompatibilitySquare", "WordCompleterCompatibilitySquare", "WordCompletionValidationSquare", "NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare"),
            ("OutputNormalizationNaturalityLaw", "ActivatedConsoleIOLaw", "ActivatedWordCompletionLaw", "ActivatedNestedCompletionLaw"),
            "OutputRenderingCapsule",
            ("reta_architecture/output_semantics.py", "reta_architecture/output_syntax.py", "reta_architecture/table_output.py", "reta_architecture/console_io.py", "reta_architecture/completion_word.py", "reta_architecture/completion_nested.py"),
            "Stages 5, 20, 24, 39",
            "Renderer sind Funktoren aus Tabellen-Sektionen in Ausgabeformate; Stage 39 ergänzt Console-/Help-/Wrapping-Morphismen, während Normalisierung semantische Vergleichbarkeit schützt.",
        ),
        _cap(
            "CompatibilityCapsule",
            "LegacyFacadeCategory",
            ("LegacyRuntimeFunctor", "ArchitectureRuntimeFunctor", "NormalizedOutputFunctor"),
            ("LegacyToArchitectureTransformation", "RenderedOutputNormalizationTransformation"),
            ("LegacyArchitectureCompatibilitySquare", "RenderedOutputParitySquare"),
            ("LegacyCompatibilityNaturalityLaw", "OutputNormalizationNaturalityLaw"),
            "CompatibilityCapsule",
            ("reta.py", "retaPrompt.py", "libs/tableHandling.py", "tests/test_command_parity.py"),
            "Stages 3-31",
            "Legacy-Importe und alte Kommandos bleiben Fassaden; Parität ist die natürlichkeitserhaltende Kompatibilitätsbedingung.",
        ),
        _cap(
            "CategoricalMetaCapsule",
            "CommutativeArchitectureContractCategory",
            ("CategoryTheoryToContractFunctor", "ArchitectureMapToContractFunctor", "TraceBoundaryImpactFunctor", "ImpactGateValidationFunctor", "ImpactToMigrationPlanFunctor", "ImpactGateBindingFunctor", "MigrationWaveOrderingFunctor", "MigrationGateCoherenceFunctor", "MigrationOrderingCoherenceFunctor", "MigrationStepRehearsalFunctor", "MigrationGateRehearsalFunctor", "RehearsalCoverFunctor", "RehearsalGateValidationFunctor", "RehearsalReadinessCoherenceFunctor", "RehearsalActivationFunctor", "GateActivationFunctor", "ActivationTransactionFunctor", "ActivationRollbackFunctor", "ActivationValidationFunctor", "ActivationCoherenceFunctor", "RowRangeActivationFunctor", "CenterRowRangeCompatibilityFunctor", "RowRangeInputFunctor", "RowRangeValidationFunctor", "ArithmeticActivationFunctor", "CenterArithmeticCompatibilityFunctor", "ArithmeticRowRangeGluingFunctor", "ArithmeticValidationFunctor", "WordCompletionActivationFunctor", "LegacyWordCompleterCompatibilityFunctor", "WordCompletionPromptFunctor", "WordCompletionValidationFunctor", "NestedCompletionActivationFunctor", "LegacyNestedCompleterCompatibilityFunctor", "NestedCompletionPromptFunctor", "NestedCompletionValidationFunctor"),
            ("ContractedNaturalityTransformation", "TraceBoundaryImpactTransformation", "ImpactGateValidationTransformation", "ImpactGateMigrationTransformation", "MigrationPlanCoherenceTransformation", "MigrationRehearsalNaturalityTransformation", "RehearsalReadinessValidationTransformation", "RehearsalActivationNaturalityTransformation", "ActivationRollbackValidationTransformation", "CenterRowRangeToArchitectureTransformation", "RowRangeValidationTransformation", "CenterArithmeticToArchitectureTransformation", "ArithmeticRowRangeGluingTransformation", "CenterConsoleIOToArchitectureTransformation", "ConsoleIOOutputValidationTransformation", "WordCompleterToArchitectureTransformation", "WordCompletionValidationTransformation", "NestedCompleterToArchitectureTransformation", "NestedCompletionValidationTransformation"),
            ("ArchitectureMapContractReflectionTriangle", "TraceBoundaryImpactSquare", "ImpactGateValidationSquare", "ImpactMigrationPlanningSquare", "MigrationGateCoherenceSquare", "MigrationRehearsalSquare", "RehearsalReadinessValidationSquare", "RehearsalActivationSquare", "ActivationRollbackValidationSquare", "CenterRowRangeCompatibilitySquare", "RowRangeValidationSquare", "CenterArithmeticCompatibilitySquare", "ArithmeticRowRangeGluingSquare", "CenterConsoleIOCompatibilitySquare", "ConsoleIOOutputValidationSquare", "WordCompleterCompatibilitySquare", "WordCompletionValidationSquare", "NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare"),
            ("ContextRefinementCompositionLaw", "LegacyCompatibilityNaturalityLaw", "ArchitectureImpactGateLaw", "ArchitectureMigrationOrderingLaw", "ArchitectureRehearsalReadinessLaw", "ArchitectureActivationCommitLaw", "ActivatedRowRangeLaw", "ActivatedArithmeticLaw", "ActivatedConsoleIOLaw", "ActivatedWordCompletionLaw", "ActivatedNestedCompletionLaw"),
            "CategoricalMetaCapsule",
            ("reta_architecture/category_theory.py", "reta_architecture/architecture_map.py", "reta_architecture/architecture_contracts.py", "reta_architecture/architecture_witnesses.py", "reta_architecture/architecture_coherence.py", "reta_architecture/architecture_impact.py", "reta_architecture/architecture_migration.py", "reta_architecture/architecture_rehearsal.py", "reta_architecture/architecture_activation.py", "reta_architecture/row_ranges.py", "reta_architecture/arithmetic.py", "reta_architecture/console_io.py", "reta_architecture/completion_word.py", "reta_architecture/completion_nested.py"),
            "Stages 27-41",
            "Die Meta-Kapsel beschreibt und prüft bis Stage 37 auch die erste echte Aktivierung: Row-Range-, Arithmetik- und Console-/Help-/Utility-Morphismen aus center.py sowie Word- und Nested-Completion aus den Legacy-Fassaden werden Architektur-Besitz.",
        ),
    )


_FLOW_TO_DIAGRAMS: Mapping[str, tuple[str, ...]] = {
    "SchemaToTopologyFunctor": ("ArchitectureMapContractReflectionTriangle",),
    "LocalDataPresheafFunctor": ("PresheafSheafGluingSquare",),
    "PresheafToSheafGluingTransformation": ("PresheafSheafGluingSquare",),
    "RawToCanonicalParameterTransformation": ("RawCommandNaturalitySquare",),
    "TableGenerationGluingTransformation": ("UniversalWorkflowTableSquare",),
    "TableGenerationGluingFunctor": ("UniversalWorkflowTableSquare",),
    "GeneratedColumnEndofunctorFamily": ("GeneratedColumnStateSyncSquare",),
    "GeneratedColumnsSheafSyncTransformation": ("GeneratedColumnStateSyncSquare",),
    "OutputRenderingFunctorFamily": ("RenderedOutputParitySquare",),
    "RenderedOutputNormalizationTransformation": ("RenderedOutputParitySquare",),
    "LegacyToArchitectureTransformation": ("LegacyArchitectureCompatibilitySquare",),
    "TableRuntimeToStateSectionsTransformation": ("RuntimeStateProjectionSquare",),
    "ContractedNaturalityTransformation": ("ArchitectureMapContractReflectionTriangle",),
    "ContractWitnessValidationTransformation": ("ValidationWitnessCommutationSquare",),
    "CoherenceMatrixFunctor": ("CoherenceTraceNavigationSquare", "BoundaryImportGraphCommutationSquare"),
    "CoherenceToTraceFunctor": ("CoherenceTraceNavigationSquare",),
    "LegacyOwnershipTraceFunctor": ("CoherenceTraceNavigationSquare",),
    "CoherenceToTraceTransformation": ("CoherenceTraceNavigationSquare",),
    "CoherenceToBoundaryFunctor": ("BoundaryImportGraphCommutationSquare",),
    "LegacyImportBoundaryFunctor": ("BoundaryImportGraphCommutationSquare",),
    "CoherenceBoundaryValidationTransformation": ("BoundaryImportGraphCommutationSquare",),
    "TraceBoundaryImpactFunctor": ("TraceBoundaryImpactSquare", "ImpactGateValidationSquare"),
    "BoundaryImpactFunctor": ("TraceBoundaryImpactSquare",),
    "ImpactGateValidationFunctor": ("ImpactGateValidationSquare",),
    "MigrationCandidateFunctor": ("ImpactGateValidationSquare",),
    "TraceBoundaryImpactTransformation": ("TraceBoundaryImpactSquare",),
    "ImpactGateValidationTransformation": ("ImpactGateValidationSquare",),
    "ImpactToMigrationPlanFunctor": ("ImpactMigrationPlanningSquare",),
    "ImpactGateBindingFunctor": ("ImpactMigrationPlanningSquare",),
    "MigrationWaveOrderingFunctor": ("MigrationGateCoherenceSquare",),
    "MigrationOrderingCoherenceFunctor": ("MigrationGateCoherenceSquare",),
    "MigrationGateCoherenceFunctor": ("MigrationGateCoherenceSquare",),
    "ImpactGateMigrationTransformation": ("ImpactMigrationPlanningSquare",),
    "MigrationPlanCoherenceTransformation": ("MigrationGateCoherenceSquare",),
    "MigrationStepRehearsalFunctor": ("MigrationRehearsalSquare",),
    "MigrationGateRehearsalFunctor": ("MigrationRehearsalSquare",),
    "RehearsalCoverFunctor": ("RehearsalReadinessValidationSquare",),
    "RehearsalGateValidationFunctor": ("RehearsalReadinessValidationSquare",),
    "RehearsalReadinessCoherenceFunctor": ("RehearsalReadinessValidationSquare",),
    "MigrationRehearsalNaturalityTransformation": ("MigrationRehearsalSquare",),
    "RehearsalReadinessValidationTransformation": ("RehearsalReadinessValidationSquare",),
    "RehearsalActivationFunctor": ("RehearsalActivationSquare",),
    "GateActivationFunctor": ("RehearsalActivationSquare",),
    "ActivationTransactionFunctor": ("ActivationRollbackValidationSquare",),
    "ActivationRollbackFunctor": ("ActivationRollbackValidationSquare",),
    "ActivationValidationFunctor": ("ActivationRollbackValidationSquare",),
    "ActivationCoherenceFunctor": ("ActivationRollbackValidationSquare",),
    "RehearsalActivationNaturalityTransformation": ("RehearsalActivationSquare",),
    "ActivationRollbackValidationTransformation": ("ActivationRollbackValidationSquare",),
    "RowRangeActivationFunctor": ("CenterRowRangeCompatibilitySquare",),
    "CenterRowRangeCompatibilityFunctor": ("CenterRowRangeCompatibilitySquare", "RowRangeValidationSquare"),
    "RowRangeInputFunctor": ("RowRangeValidationSquare",),
    "RowRangeValidationFunctor": ("RowRangeValidationSquare",),
    "CenterRowRangeToArchitectureTransformation": ("CenterRowRangeCompatibilitySquare", "RowRangeValidationSquare"),
    "RowRangeValidationTransformation": ("RowRangeValidationSquare",),
    "ArithmeticActivationFunctor": ("CenterArithmeticCompatibilitySquare",),
    "CenterArithmeticCompatibilityFunctor": ("CenterArithmeticCompatibilitySquare",),
    "ArithmeticRowRangeGluingFunctor": ("ArithmeticRowRangeGluingSquare",),
    "ArithmeticValidationFunctor": ("ArithmeticRowRangeGluingSquare",),
    "CenterArithmeticToArchitectureTransformation": ("CenterArithmeticCompatibilitySquare",),
    "ArithmeticRowRangeGluingTransformation": ("ArithmeticRowRangeGluingSquare",),
    "ConsoleIOActivationFunctor": ("CenterConsoleIOCompatibilitySquare",),
    "CenterConsoleIOCompatibilityFunctor": ("CenterConsoleIOCompatibilitySquare", "ConsoleIOOutputValidationSquare", "WordCompleterCompatibilitySquare", "WordCompletionValidationSquare", "NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare"),
    "ConsoleIOOutputRenderingFunctor": ("ConsoleIOOutputValidationSquare",),
    "ConsoleIOValidationFunctor": ("ConsoleIOOutputValidationSquare",),
    "CenterConsoleIOToArchitectureTransformation": ("CenterConsoleIOCompatibilitySquare", "ConsoleIOOutputValidationSquare", "WordCompleterCompatibilitySquare", "WordCompletionValidationSquare", "NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare"),
    "ConsoleIOOutputValidationTransformation": ("ConsoleIOOutputValidationSquare",),
    "WordCompletionActivationFunctor": ("WordCompleterCompatibilitySquare",),
    "LegacyWordCompleterCompatibilityFunctor": ("WordCompleterCompatibilitySquare", "WordCompletionValidationSquare", "NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare"),
    "WordCompletionPromptFunctor": ("WordCompletionValidationSquare",),
    "WordCompletionValidationFunctor": ("WordCompletionValidationSquare",),
    "WordCompleterToArchitectureTransformation": ("WordCompleterCompatibilitySquare", "WordCompletionValidationSquare", "NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare"),
    "WordCompletionValidationTransformation": ("WordCompletionValidationSquare",),
    "NestedCompletionActivationFunctor": ("NestedCompleterCompatibilitySquare",),
    "LegacyNestedCompleterCompatibilityFunctor": ("NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare"),
    "NestedCompletionPromptFunctor": ("NestedCompletionValidationSquare",),
    "NestedCompletionValidationFunctor": ("NestedCompletionValidationSquare",),
    "NestedCompleterToArchitectureTransformation": ("NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare"),
    "NestedCompletionValidationTransformation": ("NestedCompletionValidationSquare",),
}


def _route_kind(name: str, functor_names: set[str], transformation_names: set[str]) -> str:
    if name in functor_names:
        return "functor"
    if name in transformation_names:
        return "natural_transformation"
    return "unknown"


def _functorial_routes(
    architecture_map: ArchitectureMapBundle,
    contracts: ArchitectureContractsBundle,
    witnesses: ArchitectureWitnessBundle,
    functor_names: set[str],
    transformation_names: set[str],
) -> tuple[FunctorialRouteSpec, ...]:
    contract_diagrams = {diagram.name for diagram in contracts.diagrams}
    witness_diagrams = {item.diagram for item in witnesses.diagram_witnesses}
    routes: list[FunctorialRouteSpec] = []
    for flow in architecture_map.flows:
        name = flow.functor_or_transformation
        kind = _route_kind(name, functor_names, transformation_names)
        diagrams = tuple(diagram for diagram in _FLOW_TO_DIAGRAMS.get(name, ()) if diagram in contract_diagrams)
        witnessed = tuple(diagram for diagram in diagrams if diagram in witness_diagrams)
        status = "coherent" if kind != "unknown" and diagrams and witnessed == diagrams else "needs_attention"
        routes.append(
            FunctorialRouteSpec(
                source_capsule=flow.source,
                target_capsule=flow.target,
                morphism=flow.morphism,
                functor_or_transformation=name,
                categorical_kind=kind,
                contract_diagrams=diagrams,
                witness_diagrams=witnessed,
                code_owner=flow.code_owner,
                status=status,
                reading=f"{flow.source} → {flow.target} is treated as a {kind} route protected by {', '.join(diagrams) or 'no diagram'}.",
            )
        )
    return tuple(routes)


def _naturality_coherence(
    category_theory: CategoryTheoryBundle,
    witnesses: ArchitectureWitnessBundle,
) -> tuple[NaturalityCoherenceSpec, ...]:
    witness_by_name = {item.transformation: item for item in witnesses.naturality_witnesses}
    items: list[NaturalityCoherenceSpec] = []
    for transformation in category_theory.natural_transformations:
        witness = witness_by_name.get(transformation.name)
        diagrams = tuple(witness.diagrams) if witness is not None else ()
        capsules = tuple(witness.capsules) if witness is not None else ()
        witness_status = witness.witness_status if witness is not None else "missing"
        status = "coherent" if witness is not None and witness_status == "witnessed" else "needs_attention"
        items.append(
            NaturalityCoherenceSpec(
                transformation=transformation.name,
                source_functor=transformation.source_functor,
                target_functor=transformation.target_functor,
                component_count=len(transformation.components),
                diagrams=diagrams,
                capsules=capsules,
                witness_status=witness_status,
                status=status,
                naturality_condition=transformation.naturality_condition,
            )
        )
    return tuple(items)


def _law_coherence(
    contracts: ArchitectureContractsBundle,
    witnesses: ArchitectureWitnessBundle,
) -> tuple[LawCoherenceSpec, ...]:
    obligations = {item.name: item for item in witnesses.obligations}
    contract_capsules = {item.capsule for item in contracts.capsule_contracts}
    items: list[LawCoherenceSpec] = []
    for law in contracts.laws:
        obligation = obligations.get(law.name)
        protected_capsules = tuple(item for item in law.applies_to if item in contract_capsules)
        diagrams = tuple(
            name
            for name in law.protected_paths
            if name.endswith("Square") or name.endswith("Triangle")
        )
        if not diagrams:
            diagrams = tuple(
                obligation.witness_diagrams
                if obligation is not None
                else ()
            )
        present = obligation is not None
        items.append(
            LawCoherenceSpec(
                law=law.name,
                protected_capsules=protected_capsules,
                diagrams=diagrams,
                obligation_present=present,
                status="coherent" if present else "needs_attention",
                reading=f"{law.name} protects {', '.join(protected_capsules) or 'no capsule'} through {', '.join(diagrams) or 'its recorded protected paths'}.",
            )
        )
    return tuple(items)


def _validate(
    capsule_coherence: Sequence[CapsuleCoherenceSpec],
    routes: Sequence[FunctorialRouteSpec],
    naturality: Sequence[NaturalityCoherenceSpec],
    law_coherence: Sequence[LawCoherenceSpec],
    category_theory: CategoryTheoryBundle,
    architecture_map: ArchitectureMapBundle,
    contracts: ArchitectureContractsBundle,
    witnesses: ArchitectureWitnessBundle,
) -> CoherenceValidationSpec:
    capsule_names = {item.name for item in architecture_map.capsules}
    contract_capsules = {item.capsule for item in contracts.capsule_contracts}
    witness_capsules = {item.capsule for item in witnesses.capsule_slices}
    category_names = {item.name for item in category_theory.categories}
    functor_names = {item.name for item in category_theory.functors}
    transformation_names = {item.name for item in category_theory.natural_transformations}
    obligation_names = {item.name for item in witnesses.obligations}

    missing_capsule_contracts = tuple(sorted(capsule_names - contract_capsules))
    missing_capsule_witnesses = tuple(sorted(capsule_names - witness_capsules))
    unresolved_categories = tuple(sorted({item.category for item in capsule_coherence} - category_names))
    unresolved_functors = tuple(sorted({name for item in capsule_coherence for name in item.functors} - functor_names))
    unresolved_natural_transformations = tuple(
        sorted({name for item in capsule_coherence for name in item.natural_transformations} - transformation_names)
    )
    routes_without_known = tuple(
        f"{item.source_capsule}->{item.target_capsule}:{item.functor_or_transformation}"
        for item in routes
        if item.categorical_kind == "unknown"
    )
    routes_without_contract = tuple(
        f"{item.source_capsule}->{item.target_capsule}:{item.functor_or_transformation}"
        for item in routes
        if not item.contract_diagrams
    )
    routes_without_witness = tuple(
        f"{item.source_capsule}->{item.target_capsule}:{item.functor_or_transformation}"
        for item in routes
        if item.contract_diagrams and tuple(item.contract_diagrams) != tuple(item.witness_diagrams)
    )
    transformations_without_witness = tuple(sorted(item.transformation for item in naturality if item.witness_status == "missing"))
    laws_without_obligation = tuple(sorted(item.law for item in law_coherence if item.law not in obligation_names))

    failed = any(
        (
            missing_capsule_contracts,
            missing_capsule_witnesses,
            unresolved_categories,
            unresolved_functors,
            unresolved_natural_transformations,
            routes_without_known,
            routes_without_contract,
            routes_without_witness,
            transformations_without_witness,
            laws_without_obligation,
        )
    )
    return CoherenceValidationSpec(
        status="passed" if not failed else "needs_attention",
        missing_capsule_contracts=missing_capsule_contracts,
        missing_capsule_witnesses=missing_capsule_witnesses,
        unresolved_categories=unresolved_categories,
        unresolved_functors=unresolved_functors,
        unresolved_natural_transformations=unresolved_natural_transformations,
        routes_without_known_functor_or_transformation=routes_without_known,
        routes_without_contract=routes_without_contract,
        routes_without_witness=routes_without_witness,
        transformations_without_witness=transformations_without_witness,
        laws_without_obligation=laws_without_obligation,
    )


_TEXT_DIAGRAM = """\
ArchitectureCoherenceBundle
├─ capsule_coherence_matrix
│  ├─ every capsule → category → functor/natural transformation → contract → witness
│  └─ old reta surfaces remain compatibility entrances, not semantic owners
├─ functorial_route_matrix
│  ├─ architecture-map flows classified as functors or natural transformations
│  └─ each route points to a Stage-29 diagram and Stage-30 witness
├─ naturality_coherence_matrix
│  └─ every natural transformation is tied to diagrams, capsules and witness anchors
├─ law_coherence_matrix
│  └─ every refactor law has a witness obligation
├─ impact_gate_coherence_hint
│  └─ Stage-33 impact sources and migration candidates are checked by ArchitectureImpactBundle
├─ migration_plan_coherence_hint
│  └─ Stage-34 migration waves, Stage-35 rehearsals, Stage-36 activation transactions, Stage-37 row-range activation and Stage-38 arithmetic activation and Stage-39 console/io activation and Stage-40 word-completion activation and Stage-41 nested-completion activation are checked by ArchitectureMigrationBundle / ArchitectureRehearsalBundle / ArchitectureActivationBundle / RowRangeMorphismBundle / ArithmeticMorphismBundle
└─ validation
   └─ cross-layer gaps are reported before a later extraction is accepted
"""


_MERMAID_DIAGRAM = """\
```mermaid
flowchart TD
    Cat[CategoryTheoryBundle<br/>categories / functors / natural transformations]
    Map[ArchitectureMapBundle<br/>capsules / flows / stage map]
    Contracts[ArchitectureContractsBundle<br/>diagrams / laws]
    Witness[ArchitectureWitnessBundle<br/>anchors / slices / obligations]
    Coherence[ArchitectureCoherenceBundle<br/>Stage 31 coherence matrix]
    Cat --> Coherence
    Map --> Coherence
    Contracts --> Coherence
    Witness --> Coherence
    Coherence --> Capsule[Capsule coherence<br/>what owns what]
    Coherence --> Routes[Functorial routes<br/>how data moves]
    Coherence --> Natural[Naturality coherence<br/>which diagrams commute]
    Coherence --> Laws[Law coherence<br/>what future stages must keep true]
    Coherence --> Impact[Impact coherence<br/>trace + boundary routes expose gates]
    Coherence --> Migration[Migration coherence<br/>impact candidates become gated waves]
    Coherence --> Activation[Activation coherence<br/>rehearsed moves become commit/rollback transactions]
    Coherence --> Arithmetic[Arithmetic coherence<br/>center arithmetic delegates to activated morphisms]
    Coherence --> ConsoleIO[Console-IO coherence<br/>center help/output utilities delegate to activated morphisms]
    Coherence --> WordCompletion[Word-completion coherence<br/>word_completerAlx delegates to activated morphisms]
    Coherence --> NestedCompletion[Nested-completion coherence<br/>nestedAlx delegates to activated morphisms]
```
"""


def _plan() -> Stage31CoherencePlan:
    return Stage31CoherencePlan(
        planned_after_stage_30=(
            "Nicht bei einzelnen Witnesses stehen bleiben, sondern die ganze Kategorie/Karte/Vertrag/Witness-Schichtung zusammenziehen.",
            "Jede spätere Extraktion soll vorher sehen können, welche Kapsel, welcher Funktor, welches Diagramm und welcher Witness betroffen sind.",
            "Die natürliche-Transformationen-Schicht soll als echte Kommutativitätsmatrix nutzbar werden.",
        ),
        implemented_in_stage_31=(
            "reta_architecture/architecture_coherence.py",
            "architecture-coherence-json probe",
            "architecture-coherence-md probe",
            "Stage-31 tests for cross-layer validation",
        ),
        inherited_from_previous_stages=(
            "Stage 27: categories, functors and natural transformations",
            "Stage 28: capsule map, containment and architecture flows",
            "Stage 29: commutative diagrams, capsule contracts and refactor laws",
            "Stage 30: repository witnesses, slices and proof obligations",
        ),
        behaviour_change="keine beabsichtigte Laufzeit-/CLI-Verhaltensänderung; Stage 31 ist eine Kohärenz-, Trace- und Validierungsschicht",
    )


def bootstrap_architecture_coherence(
    *,
    category_theory: CategoryTheoryBundle,
    architecture_map: ArchitectureMapBundle,
    architecture_contracts: ArchitectureContractsBundle,
    architecture_witnesses: ArchitectureWitnessBundle,
) -> ArchitectureCoherenceBundle:
    """Return the Stage-31 coherence matrix over all previous architecture layers."""

    functor_names = {item.name for item in category_theory.functors}
    transformation_names = {item.name for item in category_theory.natural_transformations}
    capsule_coherence = _capsule_coherence()
    routes = _functorial_routes(
        architecture_map=architecture_map,
        contracts=architecture_contracts,
        witnesses=architecture_witnesses,
        functor_names=functor_names,
        transformation_names=transformation_names,
    )
    naturality = _naturality_coherence(category_theory=category_theory, witnesses=architecture_witnesses)
    laws = _law_coherence(contracts=architecture_contracts, witnesses=architecture_witnesses)
    validation = _validate(
        capsule_coherence=capsule_coherence,
        routes=routes,
        naturality=naturality,
        law_coherence=laws,
        category_theory=category_theory,
        architecture_map=architecture_map,
        contracts=architecture_contracts,
        witnesses=architecture_witnesses,
    )
    return ArchitectureCoherenceBundle(
        capsule_coherence=capsule_coherence,
        functorial_routes=routes,
        naturality_coherence=naturality,
        law_coherence=laws,
        validation=validation,
        text_diagram=_TEXT_DIAGRAM,
        mermaid_diagram=_MERMAID_DIAGRAM,
        plan=_plan(),
    )
