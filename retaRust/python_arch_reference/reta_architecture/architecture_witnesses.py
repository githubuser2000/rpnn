from __future__ import annotations

"""Stage-30 witness layer for the categorical Reta architecture.

Stages 27-29 introduced the mathematical vocabulary, the capsule map and the
commutative architecture contracts.  Stage 30 connects those symbolic contracts
back to concrete repository witnesses: files, compatibility surfaces, tests and
probe commands.  The layer is intentionally metadata-only.  It does not run the
CLI or mutate runtime state; it answers where a diagram/law/functor is witnessed
inside the current code base.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Mapping, Sequence

from .architecture_contracts import ArchitectureContractsBundle
from .architecture_map import ArchitectureMapBundle
from .category_theory import CategoryTheoryBundle


@dataclass(frozen=True)
class AnchorWitnessSpec:
    """Resolution of one symbolic architecture anchor to repository paths."""

    anchor: str
    owner: str
    resolution_kind: str
    matched_paths: Sequence[str]
    status: str
    note: str

    def snapshot(self) -> dict:
        return {
            "anchor": self.anchor,
            "owner": self.owner,
            "resolution_kind": self.resolution_kind,
            "matched_paths": list(self.matched_paths),
            "status": self.status,
            "note": self.note,
        }


@dataclass(frozen=True)
class CapsuleSliceSpec:
    """One vertical architecture slice: old reta surface -> new capsule -> witnesses."""

    capsule: str
    layer: str
    old_reta_parts: Sequence[str]
    new_owners: Sequence[str]
    contained_sections: Sequence[str]
    math_roles: Sequence[str]
    protected_by: Sequence[str]
    witness_anchors: Sequence[str]
    anchor_status: str
    stage_span: str
    description: str

    def snapshot(self) -> dict:
        return {
            "capsule": self.capsule,
            "layer": self.layer,
            "old_reta_parts": list(self.old_reta_parts),
            "new_owners": list(self.new_owners),
            "contained_sections": list(self.contained_sections),
            "math_roles": list(self.math_roles),
            "protected_by": list(self.protected_by),
            "witness_anchors": list(self.witness_anchors),
            "anchor_status": self.anchor_status,
            "stage_span": self.stage_span,
            "description": self.description,
        }


@dataclass(frozen=True)
class DiagramWitnessSpec:
    """Concrete witnesses for one Stage-29 commutative diagram."""

    diagram: str
    diagram_type: str
    capsules: Sequence[str]
    natural_transformations: Sequence[str]
    implementation_anchors: Sequence[str]
    verification_evidence: Sequence[str]
    probe_commands: Sequence[str]
    proof_obligation: str
    witness_status: str

    def snapshot(self) -> dict:
        return {
            "diagram": self.diagram,
            "diagram_type": self.diagram_type,
            "capsules": list(self.capsules),
            "natural_transformations": list(self.natural_transformations),
            "implementation_anchors": list(self.implementation_anchors),
            "verification_evidence": list(self.verification_evidence),
            "probe_commands": list(self.probe_commands),
            "proof_obligation": self.proof_obligation,
            "witness_status": self.witness_status,
        }


@dataclass(frozen=True)
class NaturalTransformationWitnessSpec:
    """Where a named natural transformation is witnessed in diagrams and code."""

    transformation: str
    source_functor: str
    target_functor: str
    diagrams: Sequence[str]
    capsules: Sequence[str]
    component_anchors: Mapping[str, str]
    code_owner: str
    witness_status: str
    naturality_condition: str

    def snapshot(self) -> dict:
        return {
            "transformation": self.transformation,
            "source_functor": self.source_functor,
            "target_functor": self.target_functor,
            "diagrams": list(self.diagrams),
            "capsules": list(self.capsules),
            "component_anchors": dict(self.component_anchors),
            "code_owner": self.code_owner,
            "witness_status": self.witness_status,
            "naturality_condition": self.naturality_condition,
        }


@dataclass(frozen=True)
class RefactorObligationSpec:
    """A law/diagram obligation future stages must continue to satisfy."""

    name: str
    obligation_type: str
    applies_to: Sequence[str]
    witness_diagrams: Sequence[str]
    evidence: Sequence[str]
    keep_true_when: str
    status: str

    def snapshot(self) -> dict:
        return {
            "name": self.name,
            "obligation_type": self.obligation_type,
            "applies_to": list(self.applies_to),
            "witness_diagrams": list(self.witness_diagrams),
            "evidence": list(self.evidence),
            "keep_true_when": self.keep_true_when,
            "status": self.status,
        }


@dataclass(frozen=True)
class WitnessValidationSpec:
    """Coverage and anchor validation for Stage 30."""

    status: str
    file_like_anchor_count: int
    resolved_anchor_count: int
    symbolic_anchor_count: int
    missing_file_like_anchors: Sequence[str]
    uncovered_capsules: Sequence[str]
    uncovered_diagrams: Sequence[str]
    uncovered_laws: Sequence[str]
    uncovered_natural_transformations: Sequence[str]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "file_like_anchor_count": self.file_like_anchor_count,
            "resolved_anchor_count": self.resolved_anchor_count,
            "symbolic_anchor_count": self.symbolic_anchor_count,
            "missing_file_like_anchors": list(self.missing_file_like_anchors),
            "uncovered_capsules": list(self.uncovered_capsules),
            "uncovered_diagrams": list(self.uncovered_diagrams),
            "uncovered_laws": list(self.uncovered_laws),
            "uncovered_natural_transformations": list(self.uncovered_natural_transformations),
        }


@dataclass(frozen=True)
class Stage30ArchitecturePlan:
    """Bridge from Stage 29 contracts to Stage 30 witnesses."""

    planned_after_stage_29: Sequence[str]
    implemented_in_stage_30: Sequence[str]
    inherited_from_previous_stages: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_after_stage_29": list(self.planned_after_stage_29),
            "implemented_in_stage_30": list(self.implemented_in_stage_30),
            "inherited_from_previous_stages": list(self.inherited_from_previous_stages),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class ArchitectureWitnessBundle:
    """Concrete Stage-30 witness matrix over map, contracts and category theory."""

    anchor_witnesses: Sequence[AnchorWitnessSpec]
    capsule_slices: Sequence[CapsuleSliceSpec]
    diagram_witnesses: Sequence[DiagramWitnessSpec]
    naturality_witnesses: Sequence[NaturalTransformationWitnessSpec]
    obligations: Sequence[RefactorObligationSpec]
    validation: WitnessValidationSpec
    text_diagram: str
    mermaid_diagram: str
    plan: Stage30ArchitecturePlan

    def capsule_slice_named(self, capsule: str) -> CapsuleSliceSpec:
        for item in self.capsule_slices:
            if item.capsule == capsule:
                return item
        raise KeyError(f"Unknown capsule slice: {capsule}")

    def diagram_witness_named(self, diagram: str) -> DiagramWitnessSpec:
        for item in self.diagram_witnesses:
            if item.diagram == diagram:
                return item
        raise KeyError(f"Unknown diagram witness: {diagram}")

    def naturality_witness_named(self, transformation: str) -> NaturalTransformationWitnessSpec:
        for item in self.naturality_witnesses:
            if item.transformation == transformation:
                return item
        raise KeyError(f"Unknown natural transformation witness: {transformation}")

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "stage": 30,
            "purpose": "Konkrete Witness-/Nachweismatrix für Kapseln, Diagramme, natürliche Transformationen und Refactor-Gesetze.",
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
                "proof_obligation",
            ],
            "counts": {
                "anchor_witnesses": len(self.anchor_witnesses),
                "capsule_slices": len(self.capsule_slices),
                "diagram_witnesses": len(self.diagram_witnesses),
                "naturality_witnesses": len(self.naturality_witnesses),
                "obligations": len(self.obligations),
            },
            "validation": self.validation.snapshot(),
            "anchor_witnesses": [item.snapshot() for item in self.anchor_witnesses],
            "capsule_slices": [item.snapshot() for item in self.capsule_slices],
            "diagram_witnesses": [item.snapshot() for item in self.diagram_witnesses],
            "naturality_witnesses": [item.snapshot() for item in self.naturality_witnesses],
            "obligations": [item.snapshot() for item in self.obligations],
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
            "plan": self.plan.snapshot(),
        }


def _is_file_like(anchor: str) -> bool:
    if " " in anchor and ("/" in anchor or "*" in anchor):
        return False
    if anchor.endswith((".py", ".md", ".csv", ".json", ".jsonl")):
        return True
    if "*" in anchor:
        return True
    if "/" in anchor:
        return True
    return False


def _candidate_patterns(anchor: str) -> tuple[str, ...]:
    anchor = anchor.strip()
    if not anchor:
        return ()
    if any(ch in anchor for ch in "[]{}()") and not any(anchor.endswith(suffix) for suffix in (".py", ".md", ".csv", ".json", ".jsonl")):
        return ()
    if "/" in anchor or "*" in anchor:
        patterns = [anchor]
        if not anchor.startswith("reta_architecture/") and anchor.endswith(".py") and "/" not in anchor:
            patterns.append(f"reta_architecture/{anchor}")
        return tuple(dict.fromkeys(patterns))
    if anchor.endswith(".py"):
        return (
            anchor,
            f"reta_architecture/{anchor}",
            f"libs/{anchor}",
            f"i18n/{anchor}",
            f"tests/{anchor}",
        )
    if anchor.endswith(".md"):
        return (anchor, f"doc/{anchor}")
    return (anchor,)


def _resolve_one(root: Path, owner: str, anchor: str) -> AnchorWitnessSpec:
    root = Path(root)
    anchor = anchor.strip()
    if not anchor:
        return AnchorWitnessSpec(anchor, owner, "empty", (), "symbolic", "Empty anchor ignored as symbolic metadata.")
    if not _is_file_like(anchor):
        return AnchorWitnessSpec(anchor, owner, "symbolic", (), "symbolic", "Symbolic owner, function name or architecture term; not expected to resolve to a file.")
    matches: list[str] = []
    for pattern in _candidate_patterns(anchor):
        for path in root.glob(pattern):
            if path.is_file():
                relative = str(path.relative_to(root)).replace("\\", "/")
                matches.append(relative)
    unique_matches = tuple(sorted(dict.fromkeys(matches)))
    status = "resolved" if unique_matches else "missing"
    return AnchorWitnessSpec(
        anchor=anchor,
        owner=owner,
        resolution_kind="glob" if "*" in anchor else "source_file",
        matched_paths=unique_matches,
        status=status,
        note="Repository anchor resolved." if unique_matches else "File-like anchor did not resolve inside the repository tree.",
    )


def _split_owner_anchors(owner: str) -> tuple[str, ...]:
    # Keep this conservative: it extracts obvious path-like pieces from strings
    # such as "table_output.py + output_semantics.py" while leaving symbolic
    # text untouched.
    pieces: list[str] = []
    for token in owner.replace(",", " + ").split("+"):
        token = token.strip()
        if token:
            pieces.append(token)
    return tuple(pieces)


def _anchor_witnesses(root: Path, architecture_map: ArchitectureMapBundle, contracts: ArchitectureContractsBundle) -> tuple[AnchorWitnessSpec, ...]:
    items: list[AnchorWitnessSpec] = []
    seen: set[tuple[str, str]] = set()

    def add(owner: str, anchor: str) -> None:
        key = (owner, anchor)
        if key in seen:
            return
        seen.add(key)
        items.append(_resolve_one(root, owner, anchor))

    for capsule in architecture_map.capsules:
        for anchor in capsule.code_owners:
            add(capsule.name, anchor)
    for mapping in architecture_map.legacy_mappings:
        add(mapping.legacy_owner, mapping.legacy_owner)
        for anchor in mapping.new_owner:
            add(mapping.legacy_owner, anchor)
    for flow in architecture_map.flows:
        for anchor in _split_owner_anchors(flow.code_owner):
            add(f"flow:{flow.source}->{flow.target}", anchor)
    for contract in contracts.capsule_contracts:
        for anchor in contract.implementation_anchors:
            add(contract.capsule, anchor)
    for diagram in contracts.diagrams:
        for arrow in tuple(diagram.top_path) + tuple(diagram.bottom_path):
            for anchor in _split_owner_anchors(arrow.code_owner):
                add(diagram.name, anchor)
        for verification in diagram.verification:
            add(f"verification:{diagram.name}", verification)
    for law in contracts.laws:
        for evidence in law.evidence:
            add(f"law:{law.name}", evidence)
    return tuple(items)


def _status_for_anchors(anchors: Sequence[str], witnesses: Sequence[AnchorWitnessSpec]) -> str:
    relevant = [item for item in witnesses if item.anchor in anchors and item.status != "symbolic"]
    if not relevant:
        return "symbolic"
    if all(item.status == "resolved" for item in relevant):
        return "resolved"
    if any(item.status == "resolved" for item in relevant):
        return "partial"
    return "missing"


def _capsule_slices(architecture_map: ArchitectureMapBundle, contracts: ArchitectureContractsBundle, witnesses: Sequence[AnchorWitnessSpec]) -> tuple[CapsuleSliceSpec, ...]:
    mappings_by_capsule: dict[str, list[str]] = {capsule.name: [] for capsule in architecture_map.capsules}
    for mapping in architecture_map.legacy_mappings:
        for capsule in mappings_by_capsule:
            if capsule in mapping.new_capsule:
                mappings_by_capsule[capsule].append(mapping.legacy_owner)

    contracts_by_capsule = {contract.capsule: contract for contract in contracts.capsule_contracts}
    result: list[CapsuleSliceSpec] = []
    for capsule in architecture_map.capsules:
        contract = contracts_by_capsule.get(capsule.name)
        protected_by = contract.protected_by if contract else ()
        anchors = tuple(capsule.code_owners) + tuple(contract.implementation_anchors if contract else ())
        result.append(
            CapsuleSliceSpec(
                capsule=capsule.name,
                layer=capsule.layer,
                old_reta_parts=tuple(sorted(dict.fromkeys(mappings_by_capsule.get(capsule.name, ())))),
                new_owners=tuple(capsule.code_owners),
                contained_sections=tuple(capsule.contains),
                math_roles=tuple(capsule.paradigm_roles),
                protected_by=tuple(protected_by),
                witness_anchors=anchors,
                anchor_status=_status_for_anchors(anchors, witnesses),
                stage_span=capsule.stage_span,
                description=capsule.description,
            )
        )
    return tuple(result)


def _probe_commands_for(diagram_name: str) -> tuple[str, ...]:
    common = ("python -B -S reta_architecture_probe_py.py architecture-witnesses-json",)
    if "RawCommand" in diagram_name:
        return common + ("python -B -S reta_architecture_probe_py.py prompt-language-json",)
    if "Presheaf" in diagram_name:
        return common + ("python -B -S reta_architecture_probe_py.py presheaves-json", "python -B -S reta_architecture_probe_py.py sheaves-json")
    if "Workflow" in diagram_name:
        return common + ("python -B -S reta_architecture_probe_py.py program-workflow-json", "python -m unittest tests.test_command_parity -v")
    if "Generated" in diagram_name:
        return common + ("python -B -S reta_architecture_probe_py.py generated-columns-json", "python -B -S reta_architecture_probe_py.py table-state-json")
    if "RuntimeState" in diagram_name:
        return common + ("python -B -S reta_architecture_probe_py.py table-runtime-json", "python -B -S reta_architecture_probe_py.py table-state-json")
    if "Output" in diagram_name:
        return common + ("python -B -S reta_architecture_probe_py.py table-output-json", "python -m unittest tests.test_command_parity -v")
    if "Legacy" in diagram_name:
        return common + ("python -B -S reta_architecture_probe_py.py package-integrity-json", "python -m unittest tests.test_command_parity -v")
    if "Validation" in diagram_name:
        return common + ("python -B -S reta_architecture_probe_py.py architecture-validation-json", "python -B -S reta_architecture_probe_py.py architecture-contracts-json")
    return common + ("python -B -S reta_architecture_probe_py.py architecture-contracts-json", "python -B -S reta_architecture_probe_py.py architecture-map-json")


def _diagram_witnesses(contracts: ArchitectureContractsBundle) -> tuple[DiagramWitnessSpec, ...]:
    result: list[DiagramWitnessSpec] = []
    for diagram in contracts.diagrams:
        anchors: list[str] = []
        for arrow in tuple(diagram.top_path) + tuple(diagram.bottom_path):
            anchors.extend(_split_owner_anchors(arrow.code_owner))
        result.append(
            DiagramWitnessSpec(
                diagram=diagram.name,
                diagram_type=diagram.diagram_type,
                capsules=tuple(diagram.capsules),
                natural_transformations=tuple(diagram.natural_transformations),
                implementation_anchors=tuple(sorted(dict.fromkeys(anchors))),
                verification_evidence=tuple(diagram.verification),
                probe_commands=_probe_commands_for(diagram.name),
                proof_obligation=diagram.equality,
                witness_status="witnessed",
            )
        )
    return tuple(result)


def _naturality_witnesses(category_theory: CategoryTheoryBundle, diagram_witnesses: Sequence[DiagramWitnessSpec]) -> tuple[NaturalTransformationWitnessSpec, ...]:
    diagrams_by_transformation: dict[str, list[DiagramWitnessSpec]] = {}
    for diagram in diagram_witnesses:
        for transformation in diagram.natural_transformations:
            diagrams_by_transformation.setdefault(transformation, []).append(diagram)

    result: list[NaturalTransformationWitnessSpec] = []
    for transformation in category_theory.natural_transformations:
        diagrams = diagrams_by_transformation.get(transformation.name, [])
        capsules = sorted({capsule for diagram in diagrams for capsule in diagram.capsules})
        result.append(
            NaturalTransformationWitnessSpec(
                transformation=transformation.name,
                source_functor=transformation.source_functor,
                target_functor=transformation.target_functor,
                diagrams=tuple(diagram.diagram for diagram in diagrams),
                capsules=tuple(capsules),
                component_anchors=dict(transformation.components),
                code_owner=transformation.code_owner,
                witness_status="witnessed" if diagrams else "uncovered",
                naturality_condition=transformation.naturality_condition,
            )
        )
    return tuple(result)


def _obligations(contracts: ArchitectureContractsBundle, diagram_witnesses: Sequence[DiagramWitnessSpec]) -> tuple[RefactorObligationSpec, ...]:
    diagrams = {diagram.diagram: diagram for diagram in diagram_witnesses}
    result: list[RefactorObligationSpec] = []
    for diagram in diagram_witnesses:
        result.append(
            RefactorObligationSpec(
                name=diagram.diagram,
                obligation_type="commutative_diagram",
                applies_to=diagram.capsules,
                witness_diagrams=(diagram.diagram,),
                evidence=diagram.verification_evidence,
                keep_true_when="moving implementation across legacy facades, architecture bundles or capsule boundaries",
                status="witnessed" if diagram.diagram in diagrams else "uncovered",
            )
        )
    for law in contracts.laws:
        matching_diagrams = tuple(
            diagram.diagram for diagram in diagram_witnesses if any(path in diagram.diagram or diagram.diagram in path for path in law.protected_paths)
        )
        if not matching_diagrams and law.protected_paths:
            # Protected paths in Stage 29 are intentionally human-readable; use
            # evidence as the stable fallback witness.
            matching_diagrams = tuple(
                diagram.diagram
                for diagram in diagram_witnesses
                if set(law.evidence).intersection(diagram.verification_evidence)
            )
        result.append(
            RefactorObligationSpec(
                name=law.name,
                obligation_type=law.law_type,
                applies_to=law.applies_to,
                witness_diagrams=matching_diagrams,
                evidence=law.evidence,
                keep_true_when=law.reta_reading,
                status="witnessed" if matching_diagrams or law.evidence else "uncovered",
            )
        )
    return tuple(result)


def _validation(
    anchor_witnesses: Sequence[AnchorWitnessSpec],
    capsule_slices: Sequence[CapsuleSliceSpec],
    diagram_witnesses: Sequence[DiagramWitnessSpec],
    naturality_witnesses: Sequence[NaturalTransformationWitnessSpec],
    obligations: Sequence[RefactorObligationSpec],
) -> WitnessValidationSpec:
    file_like = tuple(item for item in anchor_witnesses if item.status != "symbolic")
    missing = tuple(f"{item.owner}:{item.anchor}" for item in file_like if item.status == "missing")
    uncovered_capsules = tuple(item.capsule for item in capsule_slices if item.anchor_status == "missing")
    uncovered_diagrams = tuple(item.diagram for item in diagram_witnesses if item.witness_status != "witnessed")
    uncovered_laws = tuple(item.name for item in obligations if item.status != "witnessed")
    uncovered_transformations = tuple(item.transformation for item in naturality_witnesses if item.witness_status != "witnessed")
    status = "passed" if not (missing or uncovered_capsules or uncovered_diagrams or uncovered_laws or uncovered_transformations) else "attention"
    return WitnessValidationSpec(
        status=status,
        file_like_anchor_count=len(file_like),
        resolved_anchor_count=sum(1 for item in file_like if item.status == "resolved"),
        symbolic_anchor_count=sum(1 for item in anchor_witnesses if item.status == "symbolic"),
        missing_file_like_anchors=missing,
        uncovered_capsules=uncovered_capsules,
        uncovered_diagrams=uncovered_diagrams,
        uncovered_laws=uncovered_laws,
        uncovered_natural_transformations=uncovered_transformations,
    )


_TEXT_DIAGRAM = """\
ArchitectureWitnessBundle
├─ AnchorWitnesses: repository files / globs / symbolic owners
├─ CapsuleSlices: old reta owner → new capsule → math role → protected contract
├─ DiagramWitnesses: Stage-29 commutative diagrams with concrete evidence
├─ NaturalityWitnesses: natural transformations tied to diagrams and capsules
├─ RefactorObligations: laws future stages must preserve
└─ Validation handoff: Stage-31 validation consumes this witness matrix

Stage-30 reading:
Legacy reta surfaces are no longer the architecture source.  They are witnesses
or compatibility entrances.  The new owner is the capsule; the capsule is
protected by a contract; the contract is witnessed by concrete files, probes and
regression tests.
"""


_MERMAID_DIAGRAM = """\
```mermaid
flowchart TD
    Map[ArchitectureMapBundle<br/>capsules + flows] --> Witness[ArchitectureWitnessBundle]
    Contracts[ArchitectureContractsBundle<br/>diagrams + laws] --> Witness
    Category[CategoryTheoryBundle<br/>functors + natural transformations] --> Witness
    Repo[Repository tree<br/>reta.py / libs / reta_architecture / tests / csv] --> Witness
    Witness --> Anchors[Anchor witnesses]
    Witness --> Slices[Capsule slices]
    Witness --> Diagrams[Diagram witnesses]
    Witness --> Naturality[Naturality witnesses]
    Witness --> Obligations[Refactor obligations]
    Diagrams --> Compatibility[CompatibilityCapsule parity]
    Naturality --> Meta[CategoricalMetaCapsule]
```
"""


def _plan() -> Stage30ArchitecturePlan:
    return Stage30ArchitecturePlan(
        planned_after_stage_29=(
            "Tie Stage-29 diagrams and laws back to concrete repository witnesses.",
            "Show stufenweise/kapselweise where old reta owners now sit.",
            "Make natural transformations inspectable through proof obligations and probe commands.",
        ),
        implemented_in_stage_30=(
            "reta_architecture/architecture_witnesses.py",
            "architecture-witnesses-json and architecture-witnesses-md probe commands",
            "ArchitectureMap stage step and CategoricalMetaCapsule containment for ArchitectureWitnessBundle",
            "tests and package-integrity coverage for the witness layer",
        ),
        inherited_from_previous_stages=(
            "Stage 27 CategoryTheoryBundle",
            "Stage 28 ArchitectureMapBundle",
            "Stage 29 ArchitectureContractsBundle",
            "Existing command parity and architecture-regression tests",
        ),
        behaviour_change="keine beabsichtigte Laufzeit-/CLI-Verhaltensänderung; Stage 30 ist eine Nachweis-, Traceability- und Planungs-Schicht",
    )


def bootstrap_architecture_witnesses(
    repo_root: Path,
    category_theory: CategoryTheoryBundle,
    architecture_map: ArchitectureMapBundle,
    architecture_contracts: ArchitectureContractsBundle,
) -> ArchitectureWitnessBundle:
    """Return the Stage-30 witness matrix for the current repository."""

    anchors = _anchor_witnesses(Path(repo_root), architecture_map, architecture_contracts)
    slices = _capsule_slices(architecture_map, architecture_contracts, anchors)
    diagram_witnesses = _diagram_witnesses(architecture_contracts)
    naturality = _naturality_witnesses(category_theory, diagram_witnesses)
    obligations = _obligations(architecture_contracts, diagram_witnesses)
    validation = _validation(anchors, slices, diagram_witnesses, naturality, obligations)
    return ArchitectureWitnessBundle(
        anchor_witnesses=anchors,
        capsule_slices=slices,
        diagram_witnesses=diagram_witnesses,
        naturality_witnesses=naturality,
        obligations=obligations,
        validation=validation,
        text_diagram=_TEXT_DIAGRAM,
        mermaid_diagram=_MERMAID_DIAGRAM,
        plan=_plan(),
    )
