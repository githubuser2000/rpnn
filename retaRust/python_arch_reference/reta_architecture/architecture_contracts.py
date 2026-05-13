from __future__ import annotations

"""Stage-29 commutative architecture contracts for the Reta refactor.

Stage 27 names categories, functors and natural transformations.  Stage 28 draws
Reta as a capsule map.  Stage 29 makes the important naturality/parity claims
explicit as commutative diagrams, capsule boundary contracts and refactor laws.

This module is deliberately metadata-only.  It validates references against the
existing category-theory and architecture-map bundles, but it does not execute or
change the CLI/table runtime.
"""

from dataclasses import dataclass
from typing import Mapping, Sequence


@dataclass(frozen=True)
class DiagramArrowSpec:
    """One symbolic arrow in a commutative architecture diagram."""

    source: str
    target: str
    label: str
    code_owner: str
    paradigm_terms: Sequence[str]

    def snapshot(self) -> dict:
        return {
            "source": self.source,
            "target": self.target,
            "label": self.label,
            "code_owner": self.code_owner,
            "paradigm_terms": list(self.paradigm_terms),
        }


@dataclass(frozen=True)
class CommutativeDiagramSpec:
    """A named diagram whose top and bottom paths must agree semantically."""

    name: str
    diagram_type: str
    nodes: Mapping[str, str]
    top_path: Sequence[DiagramArrowSpec]
    bottom_path: Sequence[DiagramArrowSpec]
    equality: str
    capsules: Sequence[str]
    categories: Sequence[str]
    functors: Sequence[str]
    natural_transformations: Sequence[str]
    verification: Sequence[str]
    stage_origin: str
    description: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "diagram_type": self.diagram_type,
            "nodes": dict(self.nodes),
            "top_path": [item.snapshot() for item in self.top_path],
            "bottom_path": [item.snapshot() for item in self.bottom_path],
            "equality": self.equality,
            "capsules": list(self.capsules),
            "categories": list(self.categories),
            "functors": list(self.functors),
            "natural_transformations": list(self.natural_transformations),
            "verification": list(self.verification),
            "stage_origin": self.stage_origin,
            "description": self.description,
        }


@dataclass(frozen=True)
class CapsuleContractSpec:
    """Boundary contract for one Stage-28/29 architecture capsule."""

    capsule: str
    owns: Sequence[str]
    accepts: Sequence[str]
    produces: Sequence[str]
    must_not_own: Sequence[str]
    primary_category: str
    primary_functor_or_transformation: str
    protected_by: Sequence[str]
    implementation_anchors: Sequence[str]
    stage_span: str
    description: str

    def snapshot(self) -> dict:
        return {
            "capsule": self.capsule,
            "owns": list(self.owns),
            "accepts": list(self.accepts),
            "produces": list(self.produces),
            "must_not_own": list(self.must_not_own),
            "primary_category": self.primary_category,
            "primary_functor_or_transformation": self.primary_functor_or_transformation,
            "protected_by": list(self.protected_by),
            "implementation_anchors": list(self.implementation_anchors),
            "stage_span": self.stage_span,
            "description": self.description,
        }


@dataclass(frozen=True)
class RefactorLawSpec:
    """A human-readable invariant that future stages should preserve."""

    name: str
    law_type: str
    applies_to: Sequence[str]
    mathematical_reading: str
    reta_reading: str
    protected_paths: Sequence[str]
    evidence: Sequence[str]
    status: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "law_type": self.law_type,
            "applies_to": list(self.applies_to),
            "mathematical_reading": self.mathematical_reading,
            "reta_reading": self.reta_reading,
            "protected_paths": list(self.protected_paths),
            "evidence": list(self.evidence),
            "status": self.status,
        }


@dataclass(frozen=True)
class ContractValidationSpec:
    """Reference validation against CategoryTheoryBundle and ArchitectureMapBundle."""

    status: str
    known_capsules: Sequence[str]
    known_categories: Sequence[str]
    known_functors: Sequence[str]
    known_natural_transformations: Sequence[str]
    missing_capsules: Sequence[str]
    missing_categories: Sequence[str]
    missing_functors: Sequence[str]
    missing_natural_transformations: Sequence[str]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "known_capsules": list(self.known_capsules),
            "known_categories": list(self.known_categories),
            "known_functors": list(self.known_functors),
            "known_natural_transformations": list(self.known_natural_transformations),
            "missing_capsules": list(self.missing_capsules),
            "missing_categories": list(self.missing_categories),
            "missing_functors": list(self.missing_functors),
            "missing_natural_transformations": list(self.missing_natural_transformations),
        }


@dataclass(frozen=True)
class Stage29ArchitecturePlan:
    """Bridge from Stage 28's map to Stage 29's contracts."""

    planned_after_stage_28: Sequence[str]
    implemented_in_stage_29: Sequence[str]
    inherited_from_previous_stages: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_after_stage_28": list(self.planned_after_stage_28),
            "implemented_in_stage_29": list(self.implemented_in_stage_29),
            "inherited_from_previous_stages": list(self.inherited_from_previous_stages),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class ArchitectureContractsBundle:
    """Inspectable Stage-29 contracts over the categorical architecture."""

    diagrams: Sequence[CommutativeDiagramSpec]
    capsule_contracts: Sequence[CapsuleContractSpec]
    laws: Sequence[RefactorLawSpec]
    mermaid_diagram: str
    text_diagram: str
    validation: ContractValidationSpec
    plan: Stage29ArchitecturePlan

    def diagram_named(self, name: str) -> CommutativeDiagramSpec:
        for diagram in self.diagrams:
            if diagram.name == name:
                return diagram
        raise KeyError(f"Unknown commutative diagram: {name}")

    def capsule_contract_named(self, capsule: str) -> CapsuleContractSpec:
        for contract in self.capsule_contracts:
            if contract.capsule == capsule:
                return contract
        raise KeyError(f"Unknown capsule contract: {capsule}")

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "stage": 41,
            "purpose": "Kommutierende Architekturverträge, Kapselgrenzen, Refactor-Gesetze, Validierungs-/Trace-/Boundary-/Impact- und Stage-34-Migration-Plan-, Stage-35-Rehearsal- und Stage-36-Aktivierungsdiagramme für das Topologie-/Garben-/Kategorien-Paradigma.",
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
                "capsule_contract",
                "impact",
                "migration_gate",
                "migration_plan",
                "readiness_gate",
                "activation_commit",
                "rollback_section",
                "activated_runtime_morphism",
                "activated_word_completion",
            ],
            "counts": {
                "commutative_diagrams": len(self.diagrams),
                "capsule_contracts": len(self.capsule_contracts),
                "laws": len(self.laws),
            },
            "validation": self.validation.snapshot(),
            "commutative_diagrams": [item.snapshot() for item in self.diagrams],
            "capsule_contracts": [item.snapshot() for item in self.capsule_contracts],
            "laws": [item.snapshot() for item in self.laws],
            "diagrams": {
                "text": self.text_diagram,
                "mermaid": self.mermaid_diagram,
            },
            "plan": self.plan.snapshot(),
        }


def _arrow(source: str, target: str, label: str, code_owner: str, paradigm_terms: Sequence[str]) -> DiagramArrowSpec:
    return DiagramArrowSpec(source, target, label, code_owner, tuple(paradigm_terms))


def _diagram(
    name: str,
    diagram_type: str,
    nodes: Mapping[str, str],
    top_path: Sequence[DiagramArrowSpec],
    bottom_path: Sequence[DiagramArrowSpec],
    equality: str,
    capsules: Sequence[str],
    categories: Sequence[str],
    functors: Sequence[str],
    natural_transformations: Sequence[str],
    verification: Sequence[str],
    description: str,
) -> CommutativeDiagramSpec:
    return CommutativeDiagramSpec(
        name=name,
        diagram_type=diagram_type,
        nodes=dict(nodes),
        top_path=tuple(top_path),
        bottom_path=tuple(bottom_path),
        equality=equality,
        capsules=tuple(capsules),
        categories=tuple(categories),
        functors=tuple(functors),
        natural_transformations=tuple(natural_transformations),
        verification=tuple(verification),
        stage_origin="Stage 29",
        description=description,
    )


def _diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "RawCommandNaturalitySquare",
            "naturality square",
            {"A": "Raw command over U", "B": "Raw command over V", "C": "Canonical parameters over U", "D": "Canonical parameters over V"},
            (
                _arrow("A", "B", "restrict U→V", "topology.py + presheaves.py", ("topology", "presheaf", "morphism")),
                _arrow("B", "D", "canonicalize raw tokens", "prompt_language.py + sheaves.py", ("morphism", "sheaf")),
            ),
            (
                _arrow("A", "C", "canonicalize raw tokens", "input_semantics.py + prompt_runtime.py", ("functor", "natural_transformation")),
                _arrow("C", "D", "restrict canonical section", "topology.py + sheaves.py", ("topology", "sheaf")),
            ),
            "canonicalize(restrict(raw,U→V)) = restrict(canonicalize(raw),U→V)",
            ("InputPromptCapsule", "SemanticSheafCapsule"),
            ("OpenRetaContextCategory", "CanonicalSemanticSheafCategory"),
            ("RawCommandPresheafFunctor", "CanonicalParameterSheafFunctor"),
            ("RawToCanonicalParameterTransformation",),
            ("known alias lookup", "prompt-language regression tests"),
            "Raw CLI/prompt text remains compatible with context restriction and canonical parameter semantics.",
        ),
        _diagram(
            "PresheafSheafGluingSquare",
            "sheafification square",
            {"A": "Local sections over cover(U)", "B": "Local sections over cover(V)", "C": "Sheaf section over U", "D": "Sheaf section over V"},
            (
                _arrow("A", "B", "restrict local sections", "presheaves.py", ("presheaf", "morphism")),
                _arrow("B", "D", "glue compatible sections", "sheaves.py + universal.py", ("sheaf", "universal_property")),
            ),
            (
                _arrow("A", "C", "glue compatible sections", "sheaves.py + semantics_builder.py", ("sheaf", "universal_property")),
                _arrow("C", "D", "restrict glued section", "topology.py + sheaves.py", ("topology", "morphism")),
            ),
            "glue(restrict(local_sections,V)) = restrict(glue(local_sections),V)",
            ("LocalSectionCapsule", "SemanticSheafCapsule"),
            ("LocalSectionCategory", "CanonicalSemanticSheafCategory"),
            ("LocalDataPresheafFunctor", "GluedSemanticSheafFunctor"),
            ("PresheafToSheafGluingTransformation",),
            ("presheaves-json", "sheaves-json", "semantic count regressions"),
            "CSV, translation and prompt sections are local data that glue into one semantic sheaf without changing meaning under restriction.",
        ),
        _diagram(
            "UniversalWorkflowTableSquare",
            "universal construction square",
            {"A": "Canonical semantics", "B": "Workflow gluing input", "C": "Legacy Program table view", "D": "Global table section"},
            (
                _arrow("A", "B", "parameter runtime + column selection", "parameter_runtime.py + column_selection.py", ("functor", "morphism")),
                _arrow("B", "D", "universal table generation", "table_generation.py + universal.py", ("universal_property", "sheaf")),
            ),
            (
                _arrow("A", "C", "legacy Program facade", "reta.py", ("morphism", "legacy")),
                _arrow("C", "D", "architecture sync", "facade.py + program_workflow.py", ("natural_transformation", "morphism")),
            ),
            "architecture_table(canonical_semantics) = sync(legacy_program(canonical_semantics))",
            ("WorkflowGluingCapsule", "TableCoreCapsule", "CompatibilityCapsule"),
            ("CanonicalSemanticSheafCategory", "UniversalConstructionCategory", "TableSectionCategory"),
            ("TableGenerationGluingFunctor", "ArchitectureRuntimeFunctor"),
            ("TableGenerationGluingTransformation", "LegacyToArchitectureTransformation"),
            ("program workflow tests", "command parity tests"),
            "The global table is constructed at the universal workflow node, not by hidden ownership in reta.py.",
        ),
        _diagram(
            "GeneratedColumnStateSyncSquare",
            "endomorphism/state square",
            {"A": "Table section", "B": "Generated/enriched table", "C": "Explicit TableStateSections", "D": "Generated-column state"},
            (
                _arrow("A", "B", "generated-column endofunctor", "generated_columns.py", ("functor", "morphism")),
                _arrow("B", "D", "project generated state", "table_runtime.py + table_state.py", ("natural_transformation", "sheaf")),
            ),
            (
                _arrow("A", "C", "project explicit state", "table_state.py", ("morphism", "sheaf")),
                _arrow("C", "D", "sync generated metadata", "sheaves.py + universal.py", ("natural_transformation", "universal_property")),
            ),
            "project_state(Gᵢ(table)) = sync_generated(project_state(table))",
            ("GeneratedRelationCapsule", "TableCoreCapsule"),
            ("GeneratedColumnEndomorphismCategory", "TableSectionCategory"),
            ("GeneratedColumnEndofunctorFamily", "ExplicitTableStateFunctor"),
            ("GeneratedColumnsSheafSyncTransformation", "TableRuntimeToStateSectionsTransformation"),
            ("table_state tests", "generated_columns tests"),
            "Generated-column effects must be visible in table content, sheaf metadata and explicit table state together.",
        ),
        _diagram(
            "RuntimeStateProjectionSquare",
            "projection square",
            {"A": "Mutable Tables runtime", "B": "Mutated Tables runtime", "C": "TableStateSections", "D": "tableStateSnapshot"},
            (
                _arrow("A", "B", "legacy attribute mutation", "table_runtime.py", ("morphism", "legacy")),
                _arrow("B", "D", "snapshot", "table_runtime.py + table_state.py", ("natural_transformation", "sheaf")),
            ),
            (
                _arrow("A", "C", "route through state sections", "table_state.py", ("morphism", "sheaf")),
                _arrow("C", "D", "snapshot", "table_state.py", ("natural_transformation",)),
            ),
            "snapshot(mutate_legacy(Tables)) = snapshot(update_state_sections(Tables))",
            ("TableCoreCapsule",),
            ("TableSectionCategory",),
            ("MutableTableRuntimeFunctor", "ExplicitTableStateFunctor"),
            ("TableRuntimeToStateSectionsTransformation",),
            ("table_state snapshot tests",),
            "The mutable legacy-compatible Tables object and explicit state sections must show the same state.",
        ),
        _diagram(
            "RenderedOutputParitySquare",
            "output parity square",
            {"A": "Prepared table", "B": "Architecture rendered output", "C": "Legacy rendered output", "D": "Normalized comparable output"},
            (
                _arrow("A", "B", "architecture renderer", "table_output.py + output_syntax.py", ("functor", "morphism")),
                _arrow("B", "D", "normalize", "tests/test_command_parity.py", ("natural_transformation",)),
            ),
            (
                _arrow("A", "C", "legacy renderer", "libs/tableHandling.py + libs/lib4tables.py", ("legacy", "morphism")),
                _arrow("C", "D", "normalize", "tests/test_command_parity.py", ("natural_transformation",)),
            ),
            "normalize(render_arch(table)) = normalize(render_legacy(table))",
            ("OutputRenderingCapsule", "CompatibilityCapsule"),
            ("TableSectionCategory", "OutputFormatCategory", "LegacyFacadeCategory"),
            ("OutputRenderingFunctorFamily", "NormalizedOutputFunctor", "LegacyRuntimeFunctor"),
            ("RenderedOutputNormalizationTransformation", "LegacyToArchitectureTransformation"),
            ("Shell/Markdown/HTML parity", "gebrochenuniversum parity"),
            "Renderer-internal paths may differ, but normalized observable output must remain compatible.",
        ),
        _diagram(
            "LegacyArchitectureCompatibilitySquare",
            "compatibility square",
            {"A": "Legacy command/import", "B": "Legacy observable result", "C": "Architecture observable result", "D": "Same observable result"},
            (
                _arrow("A", "B", "old facade path", "reta.py + retaPrompt.py + libs", ("legacy", "morphism")),
                _arrow("B", "D", "parity comparison", "tests/test_command_parity.py", ("natural_transformation",)),
            ),
            (
                _arrow("A", "C", "architecture facade path", "facade.py", ("functor", "natural_transformation")),
                _arrow("C", "D", "parity comparison", "tests/test_command_parity.py", ("natural_transformation",)),
            ),
            "observe(legacy(command)) = observe(architecture(command))",
            ("CompatibilityCapsule", "RetaArchitectureRoot"),
            ("LegacyFacadeCategory", "OutputFormatCategory"),
            ("LegacyRuntimeFunctor", "ArchitectureRuntimeFunctor", "NormalizedOutputFunctor"),
            ("LegacyToArchitectureTransformation",),
            ("package integrity", "command parity"),
            "Legacy surfaces are façades into the architecture, not an independent second owner of semantics.",
        ),
        _diagram(
            "ArchitectureMapContractReflectionTriangle",
            "reflection triangle",
            {"A": "CategoryTheoryBundle", "B": "ArchitectureMapBundle", "C": "ArchitectureContractsBundle", "D": "Validated contracts"},
            (
                _arrow("A", "C", "CategoryTheoryToContractFunctor", "category_theory.py + architecture_contracts.py", ("category", "functor")),
                _arrow("C", "D", "ContractReferenceValidation", "architecture_contracts.py", ("morphism", "universal_property")),
            ),
            (
                _arrow("B", "C", "ArchitectureMapToContractFunctor", "architecture_map.py + architecture_contracts.py", ("functor", "natural_transformation")),
                _arrow("C", "D", "ContractReferenceValidation", "architecture_contracts.py", ("morphism", "universal_property")),
            ),
            "validate(contracts(category_theory)) = validate(contracts(architecture_map))",
            ("CategoricalMetaCapsule",),
            ("CommutativeArchitectureContractCategory",),
            ("CategoryTheoryToContractFunctor", "ArchitectureMapToContractFunctor"),
            ("ContractedNaturalityTransformation",),
            ("architecture-contracts-json validation",),
            "The Stage-29 contract layer itself is a reflected view of the category bundle and the capsule map.",
        ),
        _diagram(
            "ValidationWitnessCommutationSquare",
            "validation square",
            {"A": "Architecture contracts", "B": "Repository witnesses", "C": "Direct contract checks", "D": "Stage-31 validation report"},
            (
                _arrow("A", "B", "resolve concrete witnesses", "architecture_witnesses.py", ("witness", "natural_transformation")),
                _arrow("B", "D", "validate witnessed coverage", "architecture_validation.py", ("morphism", "universal_property")),
            ),
            (
                _arrow("A", "C", "validate symbolic references", "architecture_contracts.py + architecture_validation.py", ("functor", "morphism")),
                _arrow("C", "D", "compose validation summary", "architecture_validation.py", ("natural_transformation", "universal_property")),
            ),
            "validate_via_witnesses(contracts) = compose_validation(validate_contracts(contracts))",
            ("CategoricalMetaCapsule", "CompatibilityCapsule"),
            ("CommutativeArchitectureContractCategory", "ArchitectureValidationCategory"),
            ("ContractToValidationFunctor", "WitnessToValidationFunctor"),
            ("ContractWitnessValidationTransformation",),
            ("architecture-validation-json", "architecture-witnesses-json", "architecture-contracts-json"),
            "Stage 31 turns contracts and witnesses into one executable validation report before later stages move more code.",
        ),
    )


def _contract(
    capsule: str,
    owns: Sequence[str],
    accepts: Sequence[str],
    produces: Sequence[str],
    must_not_own: Sequence[str],
    primary_category: str,
    primary_functor_or_transformation: str,
    protected_by: Sequence[str],
    implementation_anchors: Sequence[str],
    stage_span: str,
    description: str,
) -> CapsuleContractSpec:
    return CapsuleContractSpec(
        capsule=capsule,
        owns=tuple(owns),
        accepts=tuple(accepts),
        produces=tuple(produces),
        must_not_own=tuple(must_not_own),
        primary_category=primary_category,
        primary_functor_or_transformation=primary_functor_or_transformation,
        protected_by=tuple(protected_by),
        implementation_anchors=tuple(implementation_anchors),
        stage_span=stage_span,
        description=description,
    )


def _capsule_contracts() -> tuple[CapsuleContractSpec, ...]:
    return (
        _contract("RetaArchitectureRoot", ("bootstrap order", "architecture facade"), ("legacy entry points",), ("snapshots", "bundle access"), ("domain-specific generated-column semantics",), "UniversalConstructionCategory", "ArchitectureRuntimeFunctor", ("snapshot-json",), ("facade.py", "__init__.py"), "Stages 1-29", "Root coordinates bundles without owning their inner semantics."),
        _contract("SchemaTopologyCapsule", ("schema", "open contexts", "basis covers"), ("i18n split modules", "tags"), ("ContextSelection", "RetaContextTopology"), ("table output rendering",), "OpenRetaContextCategory", "SchemaToTopologyFunctor", ("topology-json",), ("schema.py", "topology.py", "split_i18n.py"), "Stages 1-4", "Owns the topology base over reta contexts."),
        _contract("LocalSectionCapsule", ("local CSV/doc/prompt sections", "restriction maps"), ("filesystem", "open contexts"), ("restricted local sections",), ("canonical global semantics",), "LocalSectionCategory", "LocalDataPresheafFunctor", ("presheaves-json",), ("presheaves.py",), "Stage 1 onward", "Owns presheaf-like local data, not the glued sheaf meaning."),
        _contract("SemanticSheafCapsule", ("canonical parameter semantics", "generated/output/html sheaves"), ("local sections", "schema"), ("canonical pairs", "column numbers"), ("CLI parsing", "renderer syntax"), "CanonicalSemanticSheafCategory", "PresheafToSheafGluingTransformation", ("sheaves-json", "known pair lookup"), ("sheaves.py", "semantics_builder.py"), "Stages 1-3, 27-29", "Owns glued global semantic meaning."),
        _contract("InputPromptCapsule", ("prompt/runtime/completion/session/execution/preparation/interaction"), ("raw CLI/prompt text",), ("canonical command tokens", "prompt calls"), ("table rendering", "generated-column algorithms"), "OpenRetaContextCategory", "RawCommandPresheafFunctor", ("prompt-* tests",), ("input_semantics.py", "prompt_*.py"), "Stages 4, 6-12", "Owns raw input and prompt morphisms."),
        _contract("WorkflowGluingCapsule", ("parameter runtime", "column selection", "program workflow", "table generation gluing"), ("canonical semantics", "CLI args"), ("workflow result", "table-generation input"), ("mutable table internals",), "UniversalConstructionCategory", "TableGenerationGluingTransformation", ("program-workflow-json",), ("parameter_runtime.py", "column_selection.py", "program_workflow.py", "table_generation.py"), "Stages 13-15", "Owns the universal construction from semantics to table-generation input."),
        _contract("TableCoreCapsule", ("global table section", "explicit table state", "prepare/row/wrapping/number morphisms"), ("workflow result", "generated effects"), ("prepared table", "TableStateSections"), ("output syntax ownership", "CSV source ownership"), "TableSectionCategory", "TableRuntimeToStateSectionsTransformation", ("table-state-json", "table-runtime-json"), ("table_runtime.py", "table_state.py", "table_preparation.py", "row_filtering.py", "table_wrapping.py", "number_theory.py"), "Stages 16, 22-26", "Owns table state and preparation as explicit sections and morphisms."),
        _contract("GeneratedRelationCapsule", ("generated columns", "meta columns", "concat CSV", "combi join"), ("table section", "local CSV sections"), ("enriched table", "generated state"), ("renderer normalization",), "GeneratedColumnEndomorphismCategory", "GeneratedColumnEndofunctorFamily", ("generated-columns-json", "concat-csv-json", "combi-join-json"), ("generated_columns.py", "meta_columns.py", "concat_csv.py", "combi_join.py"), "Stages 17-19, 21", "Owns relation/enrichment morphisms on table sections."),
        _contract("OutputRenderingCapsule", ("output syntax", "output semantics", "table output renderers"), ("prepared table", "output mode"), ("rendered output",), ("legacy parity comparison",), "OutputFormatCategory", "OutputRenderingFunctorFamily", ("output-syntax-json", "table-output-json"), ("output_syntax.py", "output_semantics.py", "table_output.py"), "Stages 5, 20, 24", "Owns renderer functors from table sections to output formats."),
        _contract("CompatibilityCapsule", ("legacy facades", "package integrity", "command parity"), ("old command/import paths", "original archive"), ("same observable output", "manifest"), ("new canonical semantic ownership",), "LegacyFacadeCategory", "LegacyToArchitectureTransformation", ("package-integrity-json", "tests/test_command_parity.py"), ("reta.py", "retaPrompt.py", "libs/*", "package_integrity.py"), "Stages 3-29", "Owns compatibility surfaces and parity checks, not domain semantics."),
        _contract("CategoricalMetaCapsule", ("category theory", "architecture map", "architecture contracts", "architecture witnesses", "architecture validation"), ("all bundle metadata",), ("category-theory-json", "architecture-map-json", "architecture-contracts-json", "architecture-witnesses-json", "architecture-validation-json"), ("runtime domain mutation",), "CommutativeArchitectureContractCategory", "ContractWitnessValidationTransformation", ("architecture-contracts-json validation", "architecture-validation-json summary"), ("category_theory.py", "architecture_map.py", "architecture_contracts.py", "architecture_witnesses.py", "architecture_validation.py"), "Stages 27-31", "Owns the symbolic categorical map, diagrams, contracts, witnesses and executable validation."),
    )


def _law(name: str, law_type: str, applies_to: Sequence[str], mathematical_reading: str, reta_reading: str, protected_paths: Sequence[str], evidence: Sequence[str]) -> RefactorLawSpec:
    return RefactorLawSpec(name, law_type, tuple(applies_to), mathematical_reading, reta_reading, tuple(protected_paths), tuple(evidence), "documented and reference-validated in Stage 29")


def _laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ContextRefinementCompositionLaw", "topology/category", ("SchemaTopologyCapsule", "OpenRetaContextCategory"), "Context refinements compose.", "Sprache/Parameter/Zeilen/Ausgabe-Kontexte dürfen bei kompatibler Verfeinerung nicht ordnungsabhängig werden.", ("ContextSelection.refine",), ("topology-json",)),
        _law("PresheafRestrictionLaw", "presheaf", ("LocalSectionCapsule", "LocalSectionCategory"), "Restrictions compose along W⊆V⊆U.", "Lokale CSV-/Prompt-Sektionen behalten bei mehrfacher Einschränkung dieselbe Bedeutung wie bei direkter Einschränkung.", ("Presheaf.restrict",), ("presheaves-json",)),
        _law("SheafGluingUniquenessLaw", "sheaf/universal", ("SemanticSheafCapsule", "UniversalConstructionCategory"), "Compatible local sections glue to a unique represented global section.", "Parametersemantik soll nur über den kanonischen Builder/Gluing-Knoten global werden.", ("SheafBundle", "UniversalBundle"), ("known pair lookup", "semantic regression counts")),
        _law("RawCanonicalNaturalityLaw", "natural transformation", ("InputPromptCapsule", "SemanticSheafCapsule"), "Raw-command functor to canonical-parameter functor is natural in context restriction.", "Aliasauflösung und Kontextverfeinerung dürfen sich nicht widersprechen.", ("RawToCanonicalParameterTransformation",), ("prompt language tests",)),
        _law("WorkflowUniversalConstructionLaw", "universal property", ("WorkflowGluingCapsule", "TableCoreCapsule"), "The workflow construction mediates from canonical semantics to table sections.", "Tabellenbau gehört in den Workflow-Gluing-Knoten, nicht zurück in reta.py als Monolith.", ("TableGenerationGluingTransformation",), ("program-workflow-json",)),
        _law("GeneratedColumnStateSyncLaw", "functor/natural transformation", ("GeneratedRelationCapsule", "TableCoreCapsule"), "Generated-column endofunctors commute with explicit state projection.", "Generierte Spalten müssen TableStateSections und Sheaf-Metadaten synchron halten.", ("GeneratedColumnsSheafSyncTransformation",), ("table_state tests",)),
        _law("RuntimeStateProjectionLaw", "projection", ("TableCoreCapsule",), "Mutable runtime projection and explicit state section update have the same snapshot.", "Legacy-Attribute an Tables bleiben kompatibel, aber der Zustand ist explizit gekapselt.", ("TableRuntimeToStateSectionsTransformation",), ("table-state-json",)),
        _law("OutputNormalizationNaturalityLaw", "natural transformation", ("OutputRenderingCapsule", "CompatibilityCapsule"), "Output normalization is natural over supported renderer paths.", "HTML/Markdown/Shell dürfen intern anders laufen, müssen aber normalisiert paritätsfähig bleiben.", ("RenderedOutputNormalizationTransformation",), ("command parity tests",)),
        _law("LegacyCompatibilityNaturalityLaw", "compatibility natural transformation", ("CompatibilityCapsule", "RetaArchitectureRoot"), "LegacyRuntimeFunctor naturally transforms into ArchitectureRuntimeFunctor on supported command/import contexts.", "Alte Startdateien und libs sind Fassaden; neue Semantik gehört den Architektur-Kapseln.", ("LegacyToArchitectureTransformation",), ("package integrity", "command parity tests")),
        _law("ArchitectureValidationCompletenessLaw", "validation natural transformation", ("CategoricalMetaCapsule", "CompatibilityCapsule"), "Contract and witness validation commute into one Stage-31 report.", "Ein weiterer Umbau ist nur sauber, wenn Kategorie-, Kapsel-, Vertrags-, Witness-, Paket- und Markdown-Checks zusammen bestehen.", ("ContractWitnessValidationTransformation",), ("architecture-validation-json", "architecture-witnesses-json", "package-integrity-json")),
    )


def _stage32_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "CoherenceTraceNavigationSquare",
            "trace naturality square",
            {"A": "ArchitectureCoherenceBundle", "B": "ArchitectureTraceBundle", "C": "Legacy owner", "D": "RetaComponentTraceSpec"},
            (_arrow("A", "B", "CoherenceToTraceFunctor", "architecture_traces.py", ("functor", "trace")), _arrow("B", "D", "select component trace", "architecture_traces.py", ("morphism",))),
            (_arrow("A", "C", "read legacy mapping", "architecture_map.py", ("morphism",)), _arrow("C", "D", "LegacyOwnershipTraceFunctor", "architecture_traces.py", ("functor", "natural_transformation"))),
            "Tracing through coherence equals tracing from the old owner mapping.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule"),
            ("ArchitectureCoherenceCategory", "ArchitectureTraceCategory", "LegacyFacadeCategory"),
            ("CoherenceToTraceFunctor", "LegacyOwnershipTraceFunctor"),
            ("CoherenceToTraceTransformation",),
            ("architecture-traces-json", "architecture-coherence-json"),
            "Stage 32 macht die Kohärenzmatrix als alte-reta-Komponente→Kapsel→Kategorie/Funktor/Transformation-Spur navigierbar.",
        ),
        _diagram(
            "BoundaryImportGraphCommutationSquare",
            "boundary naturality square",
            {"A": "ArchitectureCoherenceBundle", "B": "ArchitectureBoundariesBundle", "C": "Python imports", "D": "CapsuleImportEdgeSpec"},
            (_arrow("A", "B", "CoherenceToBoundaryFunctor", "architecture_boundaries.py", ("functor", "category")), _arrow("B", "D", "collect capsule edges", "architecture_boundaries.py", ("morphism",))),
            (_arrow("A", "C", "read module ownership", "architecture_boundaries.py", ("morphism",)), _arrow("C", "D", "LegacyImportBoundaryFunctor", "architecture_boundaries.py", ("functor", "natural_transformation"))),
            "Boundary classification from coherence equals classification from real Python imports.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule"),
            ("ArchitectureCoherenceCategory", "ArchitectureBoundaryCategory", "LegacyFacadeCategory"),
            ("CoherenceToBoundaryFunctor", "LegacyImportBoundaryFunctor"),
            ("CoherenceBoundaryValidationTransformation",),
            ("architecture-boundaries-json",),
            "Stage 32 macht reale Python-Importe als Kapselgrenzen sichtbar; Stage 33 nutzt diese Grenzen als Impact-Eingabe.",
        ),
    )


def _stage32_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ArchitectureTraceNavigationLaw", "trace naturality", ("CategoricalMetaCapsule", "CompatibilityCapsule"), "Trace routes preserve the same categorical reading as coherence rows.", "Jede alte reta-Komponente muss über Kapsel, Kategorie, Funktor/Transformation, Diagramm und Witness verfolgbar bleiben.", ("CoherenceToTraceTransformation",), ("architecture-traces-json",)),
        _law("ArchitectureBoundaryImportLaw", "boundary morphism", ("CategoricalMetaCapsule", "CompatibilityCapsule"), "Concrete Python imports are explicit boundary morphisms between capsule owners.", "Spätere Umbauten dürfen Kapselgrenzen nicht verstecken; Importkanten müssen klassifizierbar bleiben.", ("CoherenceBoundaryValidationTransformation",), ("architecture-boundaries-json",)),
    )



def _stage33_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "TraceBoundaryImpactSquare",
            "impact naturality square",
            {"A": "ArchitectureTraceBundle", "B": "ArchitectureImpactBundle", "C": "ArchitectureBoundariesBundle", "D": "ImpactSourceSpec"},
            (_arrow("A", "B", "TraceBoundaryImpactFunctor", "architecture_impact.py", ("functor", "trace", "impact")), _arrow("B", "D", "select impact source", "architecture_impact.py", ("morphism", "impact"))),
            (_arrow("A", "C", "read boundary imports", "architecture_boundaries.py", ("morphism", "boundary")), _arrow("C", "D", "BoundaryImpactFunctor", "architecture_impact.py", ("functor", "natural_transformation"))),
            "Impact calculated from trace routes equals impact calculated from boundary-import evidence.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"),
            ("ArchitectureTraceCategory", "ArchitectureBoundaryCategory", "ArchitectureImpactCategory"),
            ("TraceBoundaryImpactFunctor", "BoundaryImpactFunctor"),
            ("TraceBoundaryImpactTransformation",),
            ("architecture-impact-json", "architecture-traces-json", "architecture-boundaries-json"),
            "Stage 33 verbindet Trace-Navigation und Import-Boundaries zu einer Impact-Route mit betroffenen Kapseln, Diagrammen, Gesetzen und Gates.",
        ),
        _diagram(
            "ImpactGateValidationSquare",
            "migration gate validation square",
            {"A": "Legacy owner", "B": "MigrationCandidateSpec", "C": "ArchitectureImpactBundle", "D": "RegressionGateSpec"},
            (_arrow("A", "B", "MigrationCandidateFunctor", "architecture_impact.py", ("functor", "migration_gate")), _arrow("B", "D", "gate candidate", "architecture_impact.py", ("morphism", "validation"))),
            (_arrow("A", "C", "TraceBoundaryImpactFunctor", "architecture_impact.py", ("functor", "impact")), _arrow("C", "D", "ImpactGateValidationFunctor", "architecture_impact.py", ("functor", "natural_transformation"))),
            "A guarded migration candidate and its impact-derived regression gates describe the same allowed future move.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"),
            ("LegacyFacadeCategory", "ArchitectureImpactCategory", "ArchitectureCoherenceCategory"),
            ("MigrationCandidateFunctor", "TraceBoundaryImpactFunctor", "ImpactGateValidationFunctor"),
            ("ImpactGateValidationTransformation",),
            ("architecture-impact-json", "architecture-validation-json", "tests/test_architecture_refactor.py"),
            "Stage 33 hält fest: weitere Extraktion ist erst sauber, wenn die Impact-Gates des Kandidaten bestehen.",
        ),
    )


def _stage33_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ArchitectureImpactGateLaw", "impact naturality", ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"), "Trace impact and boundary impact factor through the same gated migration route.", "Spätere Umbauten dürfen eine alte reta-Komponente nur bewegen, wenn ihre betroffenen Kapseln, Diagramme, Gesetze, Witnesses und Regression-Gates sichtbar bleiben.", ("TraceBoundaryImpactSquare", "ImpactGateValidationSquare", "TraceBoundaryImpactTransformation", "ImpactGateValidationTransformation"), ("architecture-impact-json", "architecture-traces-json", "architecture-boundaries-json", "architecture-validation-json")),
    )


def _stage34_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "ImpactMigrationPlanningSquare",
            "migration planning naturality square",
            {"A": "ArchitectureImpactBundle", "B": "ArchitectureMigrationBundle", "C": "MigrationCandidateSpec", "D": "MigrationStepSpec"},
            (_arrow("A", "B", "ImpactToMigrationPlanFunctor", "architecture_migration.py", ("functor", "migration_plan")), _arrow("B", "D", "plan", "architecture_migration.py", ("morphism", "migration_plan"))),
            (_arrow("A", "C", "select impact candidate", "architecture_impact.py", ("morphism", "impact")), _arrow("C", "D", "ImpactGateBindingFunctor", "architecture_migration.py", ("functor", "natural_transformation"))),
            "Planning directly from impact and planning through the candidate/gate-binding path yield the same guarded migration step.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"),
            ("ArchitectureImpactCategory", "ArchitectureMigrationCategory"),
            ("ImpactToMigrationPlanFunctor", "ImpactGateBindingFunctor"),
            ("ImpactGateMigrationTransformation",),
            ("architecture-migration-json", "architecture-impact-json", "architecture-validation-json"),
            "Stage 34 übersetzt Impact-Kandidaten in geordnete, gate-geschützte Migrationsschritte ohne Laufzeitverhalten zu bewegen.",
        ),
        _diagram(
            "MigrationGateCoherenceSquare",
            "migration gate coherence square",
            {"A": "MigrationStepSpec", "B": "MigrationWaveSpec", "C": "MigrationGateBindingSpec", "D": "MigrationInvariantSpec"},
            (_arrow("A", "B", "MigrationWaveOrderingFunctor", "architecture_migration.py", ("functor", "migration_wave")), _arrow("B", "D", "preserve_invariant", "architecture_migration.py", ("morphism", "universal_property"))),
            (_arrow("A", "C", "bind_gate", "architecture_migration.py", ("morphism", "migration_gate")), _arrow("C", "D", "MigrationGateCoherenceFunctor", "architecture_migration.py", ("functor", "coherence"))),
            "A migration step's wave ordering and its gate binding produce the same wave invariant.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"),
            ("ArchitectureMigrationCategory", "ArchitectureCoherenceCategory"),
            ("MigrationWaveOrderingFunctor", "MigrationGateCoherenceFunctor", "MigrationOrderingCoherenceFunctor"),
            ("MigrationPlanCoherenceTransformation",),
            ("architecture-migration-json", "architecture-coherence-json", "architecture-validation-json"),
            "Stage 34 hält fest: Eine spätere Extraktion ist erst planbar, wenn Wellenordnung, Gate-Binding und Invariante kommutieren.",
        ),
    )


def _stage34_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ArchitectureMigrationOrderingLaw", "migration naturality", ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"), "Impact candidates, migration steps, wave ordering and gate bindings must factor through the same naturality-preserving migration plan.", "Spätere Umbauten dürfen eine alte reta-Komponente erst bewegen, wenn ihr Stage-34-Migrationsschritt, seine Welle, seine Diagramme, natürlichen Transformationen und Gates sichtbar validiert sind.", ("ImpactMigrationPlanningSquare", "MigrationGateCoherenceSquare", "ImpactGateMigrationTransformation", "MigrationPlanCoherenceTransformation"), ("architecture-migration-json", "architecture-impact-json", "architecture-validation-json")),
    )


def _stage35_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "MigrationRehearsalSquare",
            "migration rehearsal naturality square",
            {"A": "MigrationStepSpec", "B": "RehearsalMoveSpec", "C": "MigrationGateBindingSpec", "D": "GateRehearsalSpec"},
            (_arrow("A", "B", "MigrationStepRehearsalFunctor", "architecture_rehearsal.py", ("functor", "morphism", "rehearsal")), _arrow("B", "D", "derive gate suite", "architecture_rehearsal.py", ("morphism", "readiness_gate"))),
            (_arrow("A", "C", "bind_gate", "architecture_migration.py", ("morphism", "migration_gate")), _arrow("C", "D", "MigrationGateRehearsalFunctor", "architecture_rehearsal.py", ("functor", "natural_transformation"))),
            "Rehearsing a migration step directly and rehearsing it through its gate binding produce the same gate-protected dry-run move.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"),
            ("ArchitectureMigrationCategory", "ArchitectureRehearsalCategory"),
            ("MigrationStepRehearsalFunctor", "MigrationGateRehearsalFunctor"),
            ("MigrationRehearsalNaturalityTransformation",),
            ("architecture-rehearsal-json", "architecture-migration-json", "architecture-validation-json"),
            "Stage 35 übersetzt Migrationsschritte in trockenlaufgeschützte Refactor-Morphismen.",
        ),
        _diagram(
            "RehearsalReadinessValidationSquare",
            "rehearsal readiness validation square",
            {"A": "MigrationWaveSpec", "B": "RehearsalCoverSpec", "C": "GateRehearsalSpec", "D": "ArchitectureValidationBundle"},
            (_arrow("A", "B", "RehearsalCoverFunctor", "architecture_rehearsal.py", ("functor", "topology", "universal_property")), _arrow("B", "D", "validate readiness cover", "architecture_validation.py", ("morphism", "validation"))),
            (_arrow("A", "C", "collect gate rehearsals", "architecture_rehearsal.py", ("morphism", "presheaf")), _arrow("C", "D", "RehearsalGateValidationFunctor", "architecture_validation.py", ("functor", "natural_transformation"))),
            "Wave cover validation and gate-suite validation produce the same Stage-35 readiness status.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"),
            ("ArchitectureMigrationCategory", "ArchitectureRehearsalCategory", "ArchitectureValidationCategory"),
            ("RehearsalCoverFunctor", "RehearsalGateValidationFunctor"),
            ("RehearsalReadinessValidationTransformation",),
            ("architecture-rehearsal-json", "architecture-validation-json", "architecture-coherence-json"),
            "Stage 35 hält fest: lokale Rehearsal-Sektionen müssen zu einer globalen Readiness-Garbe kleben.",
        ),
    )


def _stage35_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ArchitectureRehearsalReadinessLaw", "rehearsal naturality", ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"), "Migration steps, gate bindings, rehearsal covers and validation checks must factor through the same readiness-preserving dry-run diagram.", "Spätere Runtime-Umbauten dürfen eine alte reta-Komponente erst bewegen, wenn ihr Stage-35-Rehearsal-Open-Set, Refactor-Morphismus, Gate-Suite, Rollback-Anker und Readiness-Cover validiert sind.", ("MigrationRehearsalSquare", "RehearsalReadinessValidationSquare", "MigrationRehearsalNaturalityTransformation", "RehearsalReadinessValidationTransformation"), ("architecture-rehearsal-json", "architecture-migration-json", "architecture-validation-json")),
    )



def _stage36_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "RehearsalActivationSquare",
            "rehearsal activation naturality square",
            {"A": "RehearsalMoveSpec", "B": "ActivationUnitSpec", "C": "GateRehearsalSpec", "D": "ActivationGateSpec"},
            (_arrow("A", "B", "RehearsalActivationFunctor", "architecture_activation.py", ("functor", "morphism", "activation")), _arrow("B", "D", "derive activation gate", "architecture_activation.py", ("morphism", "commit_gate"))),
            (_arrow("A", "C", "derive gate rehearsal", "architecture_rehearsal.py", ("morphism", "readiness_gate")), _arrow("C", "D", "GateActivationFunctor", "architecture_activation.py", ("functor", "natural_transformation"))),
            "Activating a rehearsed move directly and activating it through its gate rehearsal produce the same commit-gated activation unit.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"),
            ("ArchitectureRehearsalCategory", "ArchitectureActivationCategory"),
            ("RehearsalActivationFunctor", "GateActivationFunctor"),
            ("RehearsalActivationNaturalityTransformation",),
            ("architecture-activation-json", "architecture-rehearsal-json", "architecture-validation-json"),
            "Stage 36 übersetzt Rehearsal-Moves in commit-geschützte Aktivierungsumschläge, ohne Laufzeitverhalten zu bewegen.",
        ),
        _diagram(
            "ActivationRollbackValidationSquare",
            "activation rollback validation square",
            {"A": "ActivationWindowSpec", "B": "ActivationTransactionSpec", "C": "ActivationGateSpec", "D": "ArchitectureValidationBundle"},
            (_arrow("A", "B", "ActivationTransactionFunctor", "architecture_activation.py", ("functor", "universal_property")), _arrow("B", "D", "ActivationValidationFunctor", "architecture_validation.py", ("functor", "validation"))),
            (_arrow("A", "C", "collect activation gates", "architecture_activation.py", ("morphism", "presheaf")), _arrow("C", "D", "ActivationRollbackFunctor", "architecture_activation.py", ("functor", "natural_transformation"))),
            "Transaction validation and rollback-gate validation produce the same Stage-36 activation safety status.",
            ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"),
            ("ArchitectureActivationCategory", "ArchitectureValidationCategory", "ArchitectureCoherenceCategory"),
            ("ActivationTransactionFunctor", "ActivationValidationFunctor", "ActivationRollbackFunctor"),
            ("ActivationRollbackValidationTransformation",),
            ("architecture-activation-json", "architecture-validation-json", "architecture-coherence-json"),
            "Stage 36 hält fest: Aktivierungsfenster dürfen erst als späterer Commit gelten, wenn Rollback-Sektionen und Validierung dieselbe globale Sicherheitsgarbe bilden.",
        ),
    )


def _stage36_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ArchitectureActivationCommitLaw", "activation naturality", ("CategoricalMetaCapsule", "CompatibilityCapsule", "RetaArchitectureRoot"), "Rehearsal moves, activation units, commit gates, rollback sections and validation checks must factor through the same activation transaction.", "Spätere Runtime-Umbauten dürfen einen Stage-35-Rehearsal-Move erst aktivieren, wenn Stage-36-Aktivierungsfenster, Commit-Gate, Rollback-Sektion, Transaktion und Validierung kommutieren.", ("RehearsalActivationSquare", "ActivationRollbackValidationSquare", "RehearsalActivationNaturalityTransformation", "ActivationRollbackValidationTransformation"), ("architecture-activation-json", "architecture-rehearsal-json", "architecture-validation-json")),
    )



def _stage37_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "CenterRowRangeCompatibilitySquare",
            "activated row-range compatibility square",
            {"A": "libs.center legacy row-range API", "B": "RowRangeMorphismBundle", "C": "RowRangeExpression", "D": "RowIndexSet"},
            (_arrow("A", "B", "CenterRowRangeCompatibilityFunctor", "libs/center.py", ("functor", "morphism", "compatibility")), _arrow("B", "D", "expand_row_range", "reta_architecture/row_ranges.py", ("morphism", "local_section"))),
            (_arrow("A", "C", "legacy wrapper preserves raw expression", "libs/center.py", ("morphism", "presheaf")), _arrow("C", "D", "RowRangeActivationFunctor", "reta_architecture/row_ranges.py", ("functor", "natural_transformation"))),
            "Calling BereichToNumbers2/isZeilenAngabe through center.py and calling RowRangeMorphismBundle directly produce the same row-range section.",
            ("InputPromptCapsule", "CompatibilityCapsule"),
            ("ActivatedRowRangeCategory", "LegacyFacadeCategory", "LocalSectionCategory"),
            ("CenterRowRangeCompatibilityFunctor", "RowRangeActivationFunctor", "RowRangeInputFunctor"),
            ("CenterRowRangeToArchitectureTransformation",),
            ("row-ranges-json", "tests.test_architecture_refactor", "tests.test_command_parity"),
            "Stage 37 ist die erste echte Aktivierung: center.py behält alte Namen, delegiert aber an den Architektur-Parser.",
        ),
        _diagram(
            "RowRangeValidationSquare",
            "activated row-range validation square",
            {"A": "RowRangeMorphismBundle", "B": "RowIndexSet", "C": "ArchitectureValidationBundle", "D": "CompatibilityCapsule"},
            (_arrow("A", "B", "RowRangeInputFunctor", "reta_architecture/row_ranges.py", ("functor", "local_section")), _arrow("B", "C", "RowRangeValidationFunctor", "reta_architecture/architecture_validation.py", ("functor", "validation"))),
            (_arrow("A", "D", "CenterRowRangeCompatibilityFunctor", "libs/center.py", ("functor", "compatibility")), _arrow("D", "C", "compatibility validation", "tests/test_architecture_refactor.py", ("morphism", "natural_transformation"))),
            "Row-range activation and row-range validation commute with the compatibility facade and the architecture validation report.",
            ("InputPromptCapsule", "CompatibilityCapsule", "CategoricalMetaCapsule"),
            ("ActivatedRowRangeCategory", "ArchitectureValidationCategory", "LegacyFacadeCategory"),
            ("RowRangeInputFunctor", "RowRangeValidationFunctor", "CenterRowRangeCompatibilityFunctor"),
            ("RowRangeValidationTransformation", "CenterRowRangeToArchitectureTransformation"),
            ("row-ranges-json", "architecture-validation-json", "tests.test_architecture_refactor"),
            "Stage 37 hält den aktivierten Parser in der Validierung und im Kompatibilitätsvertrag sichtbar.",
        ),
    )


def _stage37_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ActivatedRowRangeLaw", "activated runtime naturality", ("InputPromptCapsule", "CompatibilityCapsule"), "The legacy center row-range facade and the architecture row-range morphism bundle must factor through the same RowIndexSet section.", "center.py darf die Zeilenbereichslogik nicht wieder selbst besitzen; alte Funktionen bleiben Wrapper über RowRangeMorphismBundle und müssen dieselben Mengen liefern.", ("CenterRowRangeCompatibilitySquare", "RowRangeValidationSquare", "CenterRowRangeToArchitectureTransformation", "RowRangeValidationTransformation"), ("row-ranges-json", "tests.test_architecture_refactor", "tests.test_command_parity")),
    )


def _stage38_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "CenterArithmeticCompatibilitySquare",
            "activated arithmetic compatibility square",
            {"A": "libs.center legacy arithmetic API", "B": "ArithmeticMorphismBundle", "C": "ArithmeticExpression", "D": "ArithmeticSection"},
            (_arrow("A", "B", "CenterArithmeticCompatibilityFunctor", "libs/center.py", ("functor", "morphism", "compatibility")), _arrow("B", "D", "factor_pairs / prime_factors / divisor_range", "reta_architecture/arithmetic.py", ("morphism", "local_section"))),
            (_arrow("A", "C", "legacy wrapper preserves arithmetic expression", "libs/center.py", ("morphism", "presheaf")), _arrow("C", "D", "ArithmeticActivationFunctor", "reta_architecture/arithmetic.py", ("functor", "natural_transformation"))),
            "Calling multiples/teiler/primfaktoren/primRepeat through center.py and calling ArithmeticMorphismBundle directly produce the same arithmetic section.",
            ("InputPromptCapsule", "CompatibilityCapsule"),
            ("ActivatedArithmeticCategory", "LegacyFacadeCategory", "ActivatedRowRangeCategory"),
            ("CenterArithmeticCompatibilityFunctor", "ArithmeticActivationFunctor", "ArithmeticRowRangeGluingFunctor"),
            ("CenterArithmeticToArchitectureTransformation",),
            ("arithmetic-json", "tests.test_architecture_refactor", "tests.test_command_parity"),
            "Stage 38 setzt die zweite echte Aktivierung: center.py behält alte Arithmetiknamen, delegiert aber an ArithmeticMorphismBundle.",
        ),
        _diagram(
            "ArithmeticRowRangeGluingSquare",
            "activated arithmetic row-range gluing square",
            {"A": "RowRangeMorphismBundle", "B": "RowIndexSet", "C": "ArithmeticMorphismBundle", "D": "ArchitectureValidationBundle"},
            (_arrow("A", "B", "RowRangeInputFunctor", "reta_architecture/row_ranges.py", ("functor", "topology")), _arrow("B", "C", "ArithmeticRowRangeGluingFunctor", "reta_architecture/arithmetic.py", ("functor", "universal_property"))),
            (_arrow("A", "C", "compose row-range syntax with divisor gluing", "reta_architecture/arithmetic.py", ("morphism", "sheaf")), _arrow("C", "D", "ArithmeticValidationFunctor", "reta_architecture/architecture_validation.py", ("functor", "validation"))),
            "Row-range expansion and arithmetic divisor gluing commute with direct architecture validation.",
            ("InputPromptCapsule", "CategoricalMetaCapsule"),
            ("ActivatedRowRangeCategory", "ActivatedArithmeticCategory", "ArchitectureValidationCategory"),
            ("RowRangeInputFunctor", "ArithmeticRowRangeGluingFunctor", "ArithmeticValidationFunctor"),
            ("ArithmeticRowRangeGluingTransformation",),
            ("row-ranges-json", "arithmetic-json", "architecture-validation-json"),
            "Stage 38 hält die Abhängigkeit der Arithmetik von Stage 37 als kommutierendes Gluing-Diagramm sichtbar.",
        ),
    )


def _stage38_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ActivatedArithmeticLaw", "activated runtime naturality", ("InputPromptCapsule", "CompatibilityCapsule"), "The legacy center arithmetic facade and the architecture arithmetic morphism bundle must factor through the same arithmetic sections.", "center.py darf die Faktor-/Teiler-/Primfaktorlogik nicht wieder selbst besitzen; alte Funktionen bleiben Wrapper über ArithmeticMorphismBundle und müssen dieselben Ergebnisse liefern.", ("CenterArithmeticCompatibilitySquare", "ArithmeticRowRangeGluingSquare", "CenterArithmeticToArchitectureTransformation", "ArithmeticRowRangeGluingTransformation"), ("arithmetic-json", "row-ranges-json", "tests.test_architecture_refactor", "tests.test_command_parity")),
    )


def _stage39_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "CenterConsoleIOCompatibilitySquare",
            "activated console/io compatibility square",
            {"A": "libs.center legacy console/help API", "B": "ConsoleIOMorphismBundle", "C": "ConsoleIOSection", "D": "ConsoleOutputSection"},
            (_arrow("A", "B", "CenterConsoleIOCompatibilityFunctor", "libs/center.py", ("functor", "morphism", "compatibility")), _arrow("B", "D", "cli_output / get_text_wrap_things / help_text", "reta_architecture/console_io.py", ("morphism", "output"))),
            (_arrow("A", "C", "legacy wrapper preserves help/output request", "libs/center.py", ("morphism", "presheaf")), _arrow("C", "D", "ConsoleIOActivationFunctor", "reta_architecture/console_io.py", ("functor", "natural_transformation"))),
            "Calling cliout/getTextWrapThings/retaHilfe/unique_everseen through center.py and calling ConsoleIOMorphismBundle directly produce the same visible output or finite helper section.",
            ("OutputRenderingCapsule", "InputPromptCapsule", "CompatibilityCapsule"),
            ("ActivatedConsoleIOCategory", "LegacyFacadeCategory", "OutputFormatCategory"),
            ("CenterConsoleIOCompatibilityFunctor", "ConsoleIOActivationFunctor", "ConsoleIOOutputRenderingFunctor"),
            ("CenterConsoleIOToArchitectureTransformation",),
            ("console-io-json", "tests.test_architecture_refactor", "tests.test_command_parity"),
            "Stage 39 setzt die dritte echte Aktivierung: center.py behält alte Hilfe-/Output-/Utilitynamen, delegiert aber an ConsoleIOMorphismBundle.",
        ),
        _diagram(
            "ConsoleIOOutputValidationSquare",
            "activated console/io output validation square",
            {"A": "ConsoleIOMorphismBundle", "B": "ConsoleOutputSection", "C": "ArchitectureValidationBundle", "D": "OutputRenderingCapsule"},
            (_arrow("A", "B", "ConsoleIOOutputRenderingFunctor", "reta_architecture/console_io.py", ("functor", "output")), _arrow("B", "C", "ConsoleIOValidationFunctor", "reta_architecture/architecture_validation.py", ("functor", "validation"))),
            (_arrow("A", "D", "OutputRenderingFunctorFamily", "reta_architecture/table_output.py", ("functor", "rendering")), _arrow("D", "C", "output compatibility validation", "tests/test_architecture_refactor.py", ("morphism", "natural_transformation"))),
            "Console-IO activation and output validation commute with the existing output-rendering capsule.",
            ("OutputRenderingCapsule", "CompatibilityCapsule", "CategoricalMetaCapsule"),
            ("ActivatedConsoleIOCategory", "ArchitectureValidationCategory", "OutputFormatCategory"),
            ("ConsoleIOOutputRenderingFunctor", "ConsoleIOValidationFunctor", "OutputRenderingFunctorFamily"),
            ("ConsoleIOOutputValidationTransformation", "CenterConsoleIOToArchitectureTransformation"),
            ("console-io-json", "architecture-validation-json", "tests.test_architecture_refactor"),
            "Stage 39 hält die aktivierte Console-/Help-/Utility-Schicht in Validierung und Output-Kapsel sichtbar.",
        ),
    )


def _stage39_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ActivatedConsoleIOLaw", "activated runtime naturality", ("OutputRenderingCapsule", "InputPromptCapsule", "CompatibilityCapsule"), "The legacy center console/help/utility facade and the architecture console-IO morphism bundle must factor through the same output and finite helper sections.", "center.py darf Hilfe-/Output-/Wrapping-/Utilitylogik nicht wieder selbst besitzen; alte Funktionen bleiben Wrapper über ConsoleIOMorphismBundle und müssen dieselben sichtbaren Ausgaben bzw. Hilfssektionen liefern.", ("CenterConsoleIOCompatibilitySquare", "ConsoleIOOutputValidationSquare", "CenterConsoleIOToArchitectureTransformation", "ConsoleIOOutputValidationTransformation"), ("console-io-json", "architecture-validation-json", "tests.test_architecture_refactor", "tests.test_command_parity")),
    )


def _stage40_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "WordCompleterCompatibilitySquare",
            "activated word-completion compatibility square",
            {"A": "libs.word_completerAlx.WordCompleter", "B": "WordCompletionMorphismBundle", "C": "CursorPrefixOpenSet", "D": "CompletionCandidateSection"},
            (_arrow("A", "B", "LegacyWordCompleterCompatibilityFunctor", "libs/word_completerAlx.py", ("functor", "morphism", "compatibility")), _arrow("B", "D", "iter_word_completions", "reta_architecture/completion_word.py", ("morphism", "prompt"))),
            (_arrow("A", "C", "legacy get_completions restricts Document to cursor prefix", "libs/word_completerAlx.py", ("morphism", "topology")), _arrow("C", "D", "WordCompletionActivationFunctor", "reta_architecture/completion_word.py", ("functor", "natural_transformation"))),
            "Calling WordCompleter from word_completerAlx and calling WordCompletionMorphismBundle directly produce the same completion candidate section.",
            ("InputPromptCapsule", "CompatibilityCapsule"),
            ("ActivatedWordCompletionCategory", "LegacyFacadeCategory", "LocalSectionCategory"),
            ("LegacyWordCompleterCompatibilityFunctor", "WordCompletionActivationFunctor", "WordCompletionPromptFunctor"),
            ("WordCompleterToArchitectureTransformation",),
            ("word-completion-json", "tests.test_architecture_refactor", "tests.test_command_parity"),
            "Stage 40 setzt die vierte echte Aktivierung: word_completerAlx.py behält den alten WordCompleter-Namen, delegiert aber an die Architekturklasse.",
        ),
        _diagram(
            "WordCompletionValidationSquare",
            "activated word-completion validation square",
            {"A": "WordCompletionMorphismBundle", "B": "CompletionCandidateSection", "C": "ArchitectureValidationBundle", "D": "PromptCompletionSection"},
            (_arrow("A", "B", "WordCompletionPromptFunctor", "reta_architecture/completion_word.py", ("functor", "prompt")), _arrow("B", "C", "WordCompletionValidationFunctor", "reta_architecture/architecture_validation.py", ("functor", "validation"))),
            (_arrow("A", "D", "CompletionRuntimeBuilder", "reta_architecture/completion_runtime.py", ("morphism", "completion")), _arrow("D", "C", "prompt completion validation", "tests/test_architecture_refactor.py", ("morphism", "natural_transformation"))),
            "Word-completion activation and validation commute with the existing prompt completion runtime.",
            ("InputPromptCapsule", "CompatibilityCapsule", "CategoricalMetaCapsule"),
            ("ActivatedWordCompletionCategory", "ArchitectureValidationCategory", "LocalSectionCategory"),
            ("WordCompletionPromptFunctor", "WordCompletionValidationFunctor", "RawCommandPresheafFunctor"),
            ("WordCompletionValidationTransformation", "WordCompleterToArchitectureTransformation"),
            ("word-completion-json", "architecture-validation-json", "tests.test_architecture_refactor"),
            "Stage 40 hält die aktivierte Word-Completion-Schicht in Validierung und Prompt-Kapsel sichtbar.",
        ),
    )


def _stage40_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ActivatedWordCompletionLaw", "activated runtime naturality", ("InputPromptCapsule", "CompatibilityCapsule"), "The legacy word_completerAlx facade and the architecture word-completion morphism bundle must factor through the same completion candidate sections.", "word_completerAlx.py darf Matching- und Candidate-Erzeugung nicht wieder selbst besitzen; der alte WordCompleter bleibt Fassade über ArchitectureWordCompleter und muss dieselben Completion-Objekte liefern.", ("WordCompleterCompatibilitySquare", "WordCompletionValidationSquare", "WordCompleterToArchitectureTransformation", "WordCompletionValidationTransformation"), ("word-completion-json", "architecture-validation-json", "tests.test_architecture_refactor", "tests.test_command_parity")),
    )



def _stage41_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "NestedCompleterCompatibilitySquare",
            "activated nested-completion compatibility square",
            {"A": "libs.nestedAlx.NestedCompleter", "B": "NestedCompletionMorphismBundle", "C": "NestedCompletionOpenSet", "D": "NestedCompletionCandidateSection"},
            (_arrow("A", "B", "LegacyNestedCompleterCompatibilityFunctor", "libs/nestedAlx.py", ("functor", "morphism", "compatibility")), _arrow("B", "D", "yield_nested_candidates", "reta_architecture/completion_nested.py", ("morphism", "prompt"))),
            (_arrow("A", "C", "legacy matchTextAlx restricts prompt text to completion situation", "libs/nestedAlx.py", ("morphism", "topology")), _arrow("C", "D", "NestedCompletionActivationFunctor", "reta_architecture/completion_nested.py", ("functor", "natural_transformation"))),
            "Calling NestedCompleter from nestedAlx and calling NestedCompletionMorphismBundle directly produce the same nested completion candidate section.",
            ("InputPromptCapsule", "CompatibilityCapsule"),
            ("ActivatedNestedCompletionCategory", "LegacyFacadeCategory", "LocalSectionCategory"),
            ("LegacyNestedCompleterCompatibilityFunctor", "NestedCompletionActivationFunctor", "NestedCompletionPromptFunctor"),
            ("NestedCompleterToArchitectureTransformation",),
            ("nested-completion-json", "tests.test_architecture_refactor", "tests.test_command_parity"),
            "Stage 41 setzt die fünfte echte Aktivierung: nestedAlx.py behält NestedCompleter und ComplSitua, delegiert aber an die Architekturklasse.",
        ),
        _diagram(
            "NestedCompletionValidationSquare",
            "activated nested-completion validation square",
            {"A": "NestedCompletionMorphismBundle", "B": "NestedCompletionCandidateSection", "C": "ArchitectureValidationBundle", "D": "PromptCompletionSection"},
            (_arrow("A", "B", "NestedCompletionPromptFunctor", "reta_architecture/completion_nested.py", ("functor", "prompt")), _arrow("B", "C", "NestedCompletionValidationFunctor", "reta_architecture/architecture_validation.py", ("functor", "validation"))),
            (_arrow("A", "D", "CompletionRuntimeBuilder", "reta_architecture/completion_runtime.py", ("morphism", "completion")), _arrow("D", "C", "nested prompt completion validation", "tests/test_architecture_refactor.py", ("morphism", "natural_transformation"))),
            "Nested-completion activation and validation commute with the existing prompt completion runtime.",
            ("InputPromptCapsule", "CompatibilityCapsule", "CategoricalMetaCapsule"),
            ("ActivatedNestedCompletionCategory", "ArchitectureValidationCategory", "LocalSectionCategory"),
            ("NestedCompletionPromptFunctor", "NestedCompletionValidationFunctor", "RawCommandPresheafFunctor"),
            ("NestedCompletionValidationTransformation", "NestedCompleterToArchitectureTransformation"),
            ("nested-completion-json", "architecture-validation-json", "tests.test_architecture_refactor"),
            "Stage 41 hält die aktivierte Nested-Completion-Schicht in Validierung und Prompt-Kapsel sichtbar.",
        ),
    )


def _stage41_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ActivatedNestedCompletionLaw", "activated runtime naturality", ("InputPromptCapsule", "CompatibilityCapsule"), "The legacy nestedAlx facade and the architecture nested-completion morphism bundle must factor through the same hierarchical completion candidate sections.", "nestedAlx.py darf Situation-/Subcompleter-/Gleichheits-/Kommawertlogik nicht wieder selbst besitzen; NestedCompleter und ComplSitua bleiben Fassaden über ArchitectureNestedCompleter und müssen dieselben Completion-Pfade liefern.", ("NestedCompleterCompatibilitySquare", "NestedCompletionValidationSquare", "NestedCompleterToArchitectureTransformation", "NestedCompletionValidationTransformation"), ("nested-completion-json", "architecture-validation-json", "tests.test_architecture_refactor", "tests.test_command_parity")),
    )




def _stage43_diagrams() -> tuple[CommutativeDiagramSpec, ...]:
    return (
        _diagram(
            "ExecutionProcessParallelNaturalitySquare",
            "execution network process-parallel naturality square",
            {"A": "Table/row chunk inputs", "B": "Process execution tasks", "C": "Serial reducer result", "D": "Deterministic global result"},
            (_arrow("A", "B", "TableChunkExecutionFunctor / RowFilterProcessFunctor", "reta_architecture/parallel_execution.py + reta_architecture/execution_network.py", ("functor", "process", "scheduler")), _arrow("B", "D", "ExecutionResultGluingFunctor", "reta_architecture/execution_network.py", ("functor", "universal_property"))),
            (_arrow("A", "C", "serial row/table/arithmetic path", "reta_architecture/row_filtering.py + reta_architecture/arithmetic.py", ("morphism", "table", "arithmetic")), _arrow("C", "D", "deterministic reducer equality", "reta_architecture/execution_network.py", ("natural_transformation", "validation"))),
            "process_chunk_execution(input) reduced by original index equals the serial row/table/arithmetic result",
            ("TableCoreCapsule", "InputPromptCapsule", "CompatibilityCapsule"),
            ("ExecutionNetworkCategory", "SchedulerCategory", "TableSectionCategory", "ActivatedRowRangeCategory", "ActivatedArithmeticCategory"),
            ("TableChunkExecutionFunctor", "ExecutionResultGluingFunctor", "SchedulerResourceFunctor", "RowFilterProcessFunctor", "ArithmeticBatchExecutionFunctor", "ArithmeticRowRangeGluingFunctor"),
            ("ParallelExecutionNaturalityTransformation", "SchedulerExecutionNaturalityTransformation", "RowFilterProcessNaturalityTransformation", "ArithmeticBatchProcessNaturalityTransformation"),
            ("parallel-execution-json", "execution-network-json", "tests.test_architecture_refactor", "tests.test_command_parity"),
            "Stage 43 erweitert PyPy3-taugliche Prozessausführung: Zeilenfilter, Arithmetik-Batches und Chunk-Resultate müssen deterministisch zur seriellen Semantik kleben.",
        ),
        _diagram(
            "ChannelPromptExecutionNaturalitySquare",
            "channel/prompt execution naturality square",
            {"A": "Prompt command section", "B": "Half/Full duplex channel", "C": "Raw command presheaf section", "D": "Prompt execution result"},
            (_arrow("A", "B", "ChannelPromptFunctor", "reta_architecture/execution_network.py + reta_architecture/prompt_interaction.py", ("functor", "channel", "prompt")), _arrow("B", "D", "send/receive request-response", "reta_architecture/execution_network.py", ("morphism", "scheduler"))),
            (_arrow("A", "C", "RawCommandPresheafFunctor", "reta_architecture/presheaves.py + reta_architecture/prompt_language.py", ("functor", "presheaf")), _arrow("C", "D", "prompt execution abstraction", "reta_architecture/prompt_execution.py", ("morphism", "natural_transformation"))),
            "channelized prompt requests preserve the same raw-command section and executable prompt result",
            ("InputPromptCapsule", "CompatibilityCapsule"),
            ("ChannelCategory", "LocalSectionCategory", "OpenRetaContextCategory"),
            ("ChannelPromptFunctor", "RawCommandPresheafFunctor"),
            ("ChannelPromptNaturalityTransformation",),
            ("execution-network-json", "prompt-runtime-json", "tests.test_architecture_refactor"),
            "Stage 43 hält Halfduplex/Fullduplex-Kommunikation außerhalb der Topologien, aber natürlich zur Prompt-Prägarbe.",
        ),
        _diagram(
            "PersistenceRoundTripNaturalitySquare",
            "persistence roundtrip naturality square",
            {"A": "Local/sheaf/table sections", "B": "Persistent sections", "C": "Runtime sections", "D": "Loaded snapshots"},
            (_arrow("A", "B", "PresheafPersistenceFunctor / SheafPersistenceFunctor / TableStatePersistenceFunctor", "reta_architecture/persistence.py", ("functor", "database", "presheaf", "sheaf")), _arrow("B", "D", "load persisted snapshot", "reta_architecture/persistence.py", ("morphism", "database"))),
            (_arrow("A", "C", "runtime section construction", "reta_architecture/presheaves.py + reta_architecture/sheaves.py + reta_architecture/table_state.py", ("morphism", "runtime")), _arrow("C", "D", "compare loaded snapshot with runtime digest", "reta_architecture/persistence.py", ("natural_transformation", "validation"))),
            "load(persist(section)) preserves stable digest and semantic context for local, sheaf and table sections",
            ("LocalSectionCapsule", "SemanticSheafCapsule", "TableCoreCapsule", "CategoricalMetaCapsule"),
            ("PersistenceCategory", "LocalSectionCategory", "CanonicalSemanticSheafCategory", "TableSectionCategory", "ArchitectureValidationCategory"),
            ("PresheafPersistenceFunctor", "SheafPersistenceFunctor", "TableStatePersistenceFunctor", "PersistenceBatchPreparationFunctor", "PackageIntegrityExecutionFunctor", "WitnessToValidationFunctor"),
            ("PresheafPersistenceRoundTripTransformation", "SheafPersistenceRoundTripTransformation", "TableStatePersistenceTransformation", "PersistenceBatchPreparationNaturalityTransformation", "PackageIntegrityProcessNaturalityTransformation"),
            ("persistence-json", "package-integrity-json", "architecture-validation-json", "tests.test_architecture_refactor"),
            "Stage 43 speichert Prägarben-, Garben- und Tabellenzustände als Auditmaterialisierung, ohne die mathematischen Kernschichten zu verunreinigen.",
        ),
        _diagram(
            "CacheAuditPersistenceNaturalitySquare",
            "cache/audit persistence naturality square",
            {"A": "Execution or validation event", "B": "Audit persistence", "C": "Cached materialization", "D": "Validation/coherence result"},
            (_arrow("A", "B", "PersistenceAuditFunctor / ProcessExecutionAuditFunctor", "reta_architecture/persistence.py + reta_architecture/execution_network.py", ("functor", "audit")), _arrow("B", "D", "audit validation", "reta_architecture/architecture_validation.py", ("functor", "validation"))),
            (_arrow("A", "C", "CacheMaterializationFunctor", "reta_architecture/persistence.py", ("functor", "database", "cache")), _arrow("C", "D", "cache coherence validation", "reta_architecture/persistence.py + tests/test_architecture_refactor.py", ("natural_transformation", "coherence"))),
            "valid cache and audit materializations preserve the same validation/coherence statement as direct execution",
            ("WorkflowGluingCapsule", "TableCoreCapsule", "CategoricalMetaCapsule"),
            ("PersistenceCategory", "ExecutionNetworkCategory", "UniversalConstructionCategory", "TableSectionCategory", "ArchitectureValidationCategory", "ArchitectureCoherenceCategory"),
            ("CacheMaterializationFunctor", "AuditValidationPersistenceFunctor", "PersistenceAuditFunctor", "ProcessExecutionAuditFunctor", "TableGenerationGluingFunctor"),
            ("CacheCoherenceTransformation", "AuditPersistenceValidationTransformation", "ProcessExecutionAuditNaturalityTransformation"),
            ("persistence-json", "execution-network-json", "architecture-validation-json", "tests.test_architecture_refactor"),
            "Stage 43 macht Cache und Audit zu kontrollierten Materialisierungen: sie beschleunigen oder protokollieren, ersetzen aber nicht die Semantik.",
        ),
    )


def _stage43_laws() -> tuple[RefactorLawSpec, ...]:
    return (
        _law("ExecutionNetworkPersistenceLaw", "execution/persistence naturality", ("TableCoreCapsule", "LocalSectionCapsule", "SemanticSheafCapsule", "CategoricalMetaCapsule"), "Process execution, channel communication, persistence, cache and audit must factor through deterministic reducers and stable digests without mutating topology, presheaves, sheaves, morphisms or universal constructions.", "Queues, Stacks, Semaphoren, Kanäle und SQLite-Persistenz bleiben eigene Runtime-/Audit-Kapseln; mathematische Kernschichten bleiben rein und alle Materialisierungen müssen roundtrip-, cache- und parallelitätskohärent sein.", ("ExecutionProcessParallelNaturalitySquare", "ChannelPromptExecutionNaturalitySquare", "PersistenceRoundTripNaturalitySquare", "CacheAuditPersistenceNaturalitySquare", "ParallelExecutionNaturalityTransformation", "PresheafPersistenceRoundTripTransformation", "CacheCoherenceTransformation"), ("parallel-execution-json", "execution-network-json", "persistence-json", "tests.test_architecture_refactor", "tests.test_command_parity")),
    )
_TEXT_DIAGRAM = """\
ArchitectureContractsBundle
├─ Commutative diagrams
│  ├─ RawCommandNaturalitySquare
│  ├─ PresheafSheafGluingSquare
│  ├─ UniversalWorkflowTableSquare
│  ├─ GeneratedColumnStateSyncSquare
│  ├─ RuntimeStateProjectionSquare
│  ├─ RenderedOutputParitySquare
│  ├─ LegacyArchitectureCompatibilitySquare
│  ├─ ArchitectureMapContractReflectionTriangle
│  ├─ ValidationWitnessCommutationSquare
│  ├─ CoherenceTraceNavigationSquare
│  ├─ BoundaryImportGraphCommutationSquare
│  ├─ TraceBoundaryImpactSquare
│  ├─ ImpactGateValidationSquare
│  ├─ ImpactMigrationPlanningSquare
│  ├─ MigrationGateCoherenceSquare
│  ├─ MigrationRehearsalSquare
│  ├─ RehearsalReadinessValidationSquare
│  ├─ RehearsalActivationSquare
│  ├─ ActivationRollbackValidationSquare
│  ├─ CenterRowRangeCompatibilitySquare
│  ├─ RowRangeValidationSquare
│  ├─ CenterArithmeticCompatibilitySquare
│  ├─ ArithmeticRowRangeGluingSquare
│  ├─ CenterConsoleIOCompatibilitySquare
│  ├─ ConsoleIOOutputValidationSquare
│  ├─ WordCompleterCompatibilitySquare
│  ├─ WordCompletionValidationSquare
│  └─ NestedCompletionValidationSquare
├─ Capsule contracts
│  ├─ RetaArchitectureRoot
│  ├─ SchemaTopologyCapsule
│  ├─ LocalSectionCapsule
│  ├─ SemanticSheafCapsule
│  ├─ InputPromptCapsule
│  ├─ WorkflowGluingCapsule
│  ├─ TableCoreCapsule
│  ├─ GeneratedRelationCapsule
│  ├─ OutputRenderingCapsule
│  ├─ CompatibilityCapsule
│  └─ CategoricalMetaCapsule
└─ Refactor laws
   ├─ topology / presheaf / sheaf laws
   ├─ universal workflow law
   ├─ generated/state sync law
   ├─ output-normalization law
   ├─ legacy-compatibility law
   ├─ architecture-validation-completeness law
   ├─ architecture-trace / boundary laws
   ├─ architecture-impact-gate law
   ├─ activated-row-range law
   ├─ activated-arithmetic law
   ├─ activated-console-io law
   └─ activated-word-completion law
"""


_MERMAID_DIAGRAM = """\
```mermaid
flowchart TD
    Raw[Raw CLI/Prompt] -->|RawCommandNaturalitySquare| Canonical[Canonical semantic sheaf]
    Local[Local CSV/doc/prompt sections] -->|PresheafSheafGluingSquare| Canonical
    Canonical -->|UniversalWorkflowTableSquare| Table[Global table section]
    Table -->|GeneratedColumnStateSyncSquare| Generated[Generated/enriched state]
    Table -->|RuntimeStateProjectionSquare| State[Explicit TableStateSections]
    Table -->|RenderedOutputParitySquare| Output[Normalized output]
    Legacy[Legacy reta.py/retaPrompt.py/libs] -->|LegacyArchitectureCompatibilitySquare| Output
    Meta[CategoryTheoryBundle + ArchitectureMapBundle] -->|ArchitectureMapContractReflectionTriangle| Contracts[ArchitectureContractsBundle]
    Contracts -->|ContractReferenceValidation| Validated[Validated contracts]
    Witnesses[ArchitectureWitnessBundle] -->|ValidationWitnessCommutationSquare| Validation[ArchitectureValidationBundle]
    Validated -->|ContractWitnessValidationTransformation| Validation
    Migration -->|MigrationRehearsalSquare| Rehearsal[ArchitectureRehearsalBundle]
    Rehearsal -->|RehearsalReadinessValidationSquare| Validation
    Rehearsal -->|RehearsalActivationSquare| Activation[ArchitectureActivationBundle]
    Activation -->|ActivationRollbackValidationSquare| Validation
    Center[libs.center row-range facade] -->|CenterRowRangeCompatibilitySquare| RowRanges[RowRangeMorphismBundle]
    RowRanges -->|RowRangeValidationSquare| Validation
    RowRanges -->|ArithmeticRowRangeGluingSquare| Arithmetic[ArithmeticMorphismBundle]
    Center -->|CenterArithmeticCompatibilitySquare| Arithmetic
    Arithmetic -->|ArithmeticValidationFunctor| Validation
    Center -->|CenterConsoleIOCompatibilitySquare| ConsoleIO[ConsoleIOMorphismBundle]
    ConsoleIO -->|ConsoleIOOutputValidationSquare| Validation
    WordCompletion[WordCompletionMorphismBundle] -->|WordCompletionValidationSquare| Validation
    NestedCompletion[NestedCompletionMorphismBundle] -->|NestedCompletionValidationSquare| Validation
    Center -->|WordCompleterCompatibilitySquare| WordCompletion
    Trace[ArchitectureTraceBundle] -->|TraceBoundaryImpactSquare| Impact[ArchitectureImpactBundle]
    Boundary[ArchitectureBoundariesBundle] -->|TraceBoundaryImpactTransformation| Impact
    Impact -->|ImpactGateValidationSquare| Gates[Regression gates]
    Impact -->|ImpactMigrationPlanningSquare| Migration[ArchitectureMigrationBundle]
    Migration -->|MigrationGateCoherenceSquare| MigrationValidation[Migration validation]
```
"""


def _plan() -> Stage29ArchitecturePlan:
    return Stage29ArchitecturePlan(
        planned_after_stage_28=(
            "Die Stage-28-Kapselkarte nicht nur zeichnen, sondern als kommutierende Pfade und prüfbare Architekturverträge festhalten.",
            "Natürliche Transformationen mit konkreten Top-/Bottom-Pfaden, Kapseln, Kategorien und Prüfankern verbinden.",
            "Für jede Kapsel definieren, was sie besitzt, was sie annimmt, was sie produziert und was sie ausdrücklich nicht besitzen soll.",
        ),
        implemented_in_stage_29=(
            "Neue Datei reta_architecture/architecture_contracts.py mit CommutativeDiagramSpec, CapsuleContractSpec, RefactorLawSpec und ArchitectureContractsBundle.",
            "Neun kommutierende Diagramme verbinden Topologie, Prägarben, Garben, universelles Gluing, Tabellenzustand, Renderer, Legacy-Parität, Meta-Validierung und Stage-31-Witness-Validierung.",
            "Elf Kapselverträge legen stufenweise fest, welche reta-Teile in welcher Kapsel liegen und welche Grenzen nicht verletzt werden sollen.",
            "Zehn Refactor-Gesetze formulieren die Architektur-Invarianten, die spätere Stages schützen müssen.",
            "Die Vertragsreferenzen werden gegen CategoryTheoryBundle und ArchitectureMapBundle validiert und von Stage 31 ausführbar zusammengeführt.",
        ),
        inherited_from_previous_stages=(
            "Stage 1-3: Topologie, Prägarben, Garben und i18n-/Schema-Split.",
            "Stage 4-12: Input-/Prompt-Kapseln.",
            "Stage 13-26: Workflow-Gluing, Tabellenkern, Generated/Output/State-Schichten.",
            "Stage 27: Kategorien, Funktoren und natürliche Transformationen.",
            "Stage 28: Gesamtarchitekturkarte und Kapselbaum.",
            "Stage 30: Witness-Matrix über Repository-Ankern.",
            "Stage 31: ausführbare Architekturvalidierung.",
        ),
        behaviour_change="keine beabsichtigte Laufzeit-/CLI-Verhaltensänderung; Stage 29/31 ist eine Architekturvertrags-, Witness- und Validierungs-Metadaten-Schicht",
    )


def _names_from_attr(bundle, attr: str) -> tuple[str, ...]:
    if bundle is None:
        return ()
    return tuple(sorted(getattr(item, "name", "") for item in getattr(bundle, attr, ()) if getattr(item, "name", "")))


def _capsule_names(architecture_map) -> tuple[str, ...]:
    if architecture_map is None:
        return ()
    return tuple(sorted(getattr(item, "name", "") for item in getattr(architecture_map, "capsules", ()) if getattr(item, "name", "")))


def _validation(diagrams: Sequence[CommutativeDiagramSpec], contracts: Sequence[CapsuleContractSpec], category_theory=None, architecture_map=None) -> ContractValidationSpec:
    known_capsules = _capsule_names(architecture_map)
    known_categories = _names_from_attr(category_theory, "categories")
    known_functors = _names_from_attr(category_theory, "functors")
    known_natural_transformations = _names_from_attr(category_theory, "natural_transformations")

    referenced_capsules = set()
    referenced_categories = set()
    referenced_functors = set()
    referenced_transformations = set()
    for diagram in diagrams:
        referenced_capsules.update(diagram.capsules)
        referenced_categories.update(diagram.categories)
        referenced_functors.update(diagram.functors)
        referenced_transformations.update(diagram.natural_transformations)
    for contract in contracts:
        referenced_capsules.add(contract.capsule)
        referenced_categories.add(contract.primary_category)
        primary = contract.primary_functor_or_transformation
        if primary.endswith("Transformation"):
            referenced_transformations.add(primary)
        else:
            referenced_functors.add(primary)

    if not (known_capsules or known_categories or known_functors or known_natural_transformations):
        return ContractValidationSpec("not_checked", known_capsules, known_categories, known_functors, known_natural_transformations, (), (), (), ())

    missing_capsules = tuple(sorted(referenced_capsules - set(known_capsules)))
    missing_categories = tuple(sorted(referenced_categories - set(known_categories)))
    missing_functors = tuple(sorted(referenced_functors - set(known_functors)))
    missing_transformations = tuple(sorted(referenced_transformations - set(known_natural_transformations)))
    status = "passed" if not (missing_capsules or missing_categories or missing_functors or missing_transformations) else "failed"
    return ContractValidationSpec(
        status=status,
        known_capsules=known_capsules,
        known_categories=known_categories,
        known_functors=known_functors,
        known_natural_transformations=known_natural_transformations,
        missing_capsules=missing_capsules,
        missing_categories=missing_categories,
        missing_functors=missing_functors,
        missing_natural_transformations=missing_transformations,
    )


def bootstrap_architecture_contracts(category_theory=None, architecture_map=None) -> ArchitectureContractsBundle:
    """Return the Stage-29 architecture contracts bundle."""

    if category_theory is None:
        from .category_theory import bootstrap_category_theory

        category_theory = bootstrap_category_theory()
    if architecture_map is None:
        from .architecture_map import bootstrap_architecture_map

        architecture_map = bootstrap_architecture_map()
    diagrams = _diagrams() + _stage32_diagrams() + _stage33_diagrams() + _stage34_diagrams() + _stage35_diagrams() + _stage36_diagrams() + _stage37_diagrams() + _stage38_diagrams() + _stage39_diagrams() + _stage40_diagrams() + _stage41_diagrams() + _stage43_diagrams()
    contracts = _capsule_contracts()
    return ArchitectureContractsBundle(
        diagrams=diagrams,
        capsule_contracts=contracts,
        laws=_laws() + _stage32_laws() + _stage33_laws() + _stage34_laws() + _stage35_laws() + _stage36_laws() + _stage37_laws() + _stage38_laws() + _stage39_laws() + _stage40_laws() + _stage41_laws() + _stage43_laws(),
        mermaid_diagram=_MERMAID_DIAGRAM,
        text_diagram=_TEXT_DIAGRAM,
        validation=_validation(diagrams, contracts, category_theory=category_theory, architecture_map=architecture_map),
        plan=_plan(),
    )
