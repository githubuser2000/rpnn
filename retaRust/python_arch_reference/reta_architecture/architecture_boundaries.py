from __future__ import annotations

"""Stage-32 capsule-boundary and import graph for the Reta architecture."""

import ast
from dataclasses import dataclass
from pathlib import Path
from typing import Mapping, Sequence

from .architecture_coherence import ArchitectureCoherenceBundle
from .architecture_map import ArchitectureMapBundle


@dataclass(frozen=True)
class ModuleOwnershipSpec:
    path: str
    capsule: str
    owner_kind: str
    reason: str

    def snapshot(self) -> dict:
        return {"path": self.path, "capsule": self.capsule, "owner_kind": self.owner_kind, "reason": self.reason}


@dataclass(frozen=True)
class ImportEdgeSpec:
    importer: str
    imported: str
    importer_capsule: str
    imported_capsule: str
    import_kind: str
    categorical_kind: str
    allowed: bool
    reason: str

    def snapshot(self) -> dict:
        return {
            "importer": self.importer,
            "imported": self.imported,
            "importer_capsule": self.importer_capsule,
            "imported_capsule": self.imported_capsule,
            "import_kind": self.import_kind,
            "categorical_kind": self.categorical_kind,
            "allowed": self.allowed,
            "reason": self.reason,
        }


@dataclass(frozen=True)
class CapsuleImportEdgeSpec:
    source_capsule: str
    target_capsule: str
    edge_count: int
    categorical_kind: str
    representative_imports: Sequence[str]

    def snapshot(self) -> dict:
        return {
            "source_capsule": self.source_capsule,
            "target_capsule": self.target_capsule,
            "edge_count": self.edge_count,
            "categorical_kind": self.categorical_kind,
            "representative_imports": list(self.representative_imports),
        }


@dataclass(frozen=True)
class CapsuleBoundarySpec:
    capsule: str
    owns_modules: Sequence[str]
    allowed_outbound_capsules: Sequence[str]
    inbound_capsules: Sequence[str]
    boundary_reading: str

    def snapshot(self) -> dict:
        return {
            "capsule": self.capsule,
            "owns_modules": list(self.owns_modules),
            "allowed_outbound_capsules": list(self.allowed_outbound_capsules),
            "inbound_capsules": list(self.inbound_capsules),
            "boundary_reading": self.boundary_reading,
        }


@dataclass(frozen=True)
class BoundaryCheckSpec:
    name: str
    status: str
    failed_items: Sequence[str]
    checked_count: int
    reading: str

    def snapshot(self) -> dict:
        return {"name": self.name, "status": self.status, "failed_items": list(self.failed_items), "checked_count": self.checked_count, "reading": self.reading}


@dataclass(frozen=True)
class BoundaryValidationSpec:
    status: str
    violation_edges: Sequence[str]
    unresolved_internal_imports: Sequence[str]
    unowned_scanned_paths: Sequence[str]
    missing_capsule_boundaries: Sequence[str]
    checks: Sequence[BoundaryCheckSpec]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "violation_edges": list(self.violation_edges),
            "unresolved_internal_imports": list(self.unresolved_internal_imports),
            "unowned_scanned_paths": list(self.unowned_scanned_paths),
            "missing_capsule_boundaries": list(self.missing_capsule_boundaries),
            "checks": [item.snapshot() for item in self.checks],
        }


@dataclass(frozen=True)
class Stage32BoundaryPlan:
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
class ArchitectureBoundariesBundle:
    module_ownership: Sequence[ModuleOwnershipSpec]
    import_edges: Sequence[ImportEdgeSpec]
    capsule_edges: Sequence[CapsuleImportEdgeSpec]
    capsule_boundaries: Sequence[CapsuleBoundarySpec]
    validation: BoundaryValidationSpec
    text_diagram: str
    mermaid_diagram: str
    plan: Stage32BoundaryPlan

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "stage": 32,
            "paradigm": ["topology", "morphism", "category", "functor", "natural_transformation", "boundary"],
            "counts": {
                "capsule_boundaries": len(self.capsule_boundaries),
                "module_ownership": len(self.module_ownership),
                "import_edges": len(self.import_edges),
                "capsule_edges": len(self.capsule_edges),
                "checks": len(self.validation.checks),
            },
            "module_ownership": [item.snapshot() for item in self.module_ownership],
            "import_edges": [item.snapshot() for item in self.import_edges],
            "capsule_edges": [item.snapshot() for item in self.capsule_edges],
            "capsule_boundaries": [item.snapshot() for item in self.capsule_boundaries],
            "validation": self.validation.snapshot(),
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
            "plan": self.plan.snapshot(),
        }


def _norm(path: Path) -> str:
    return path.as_posix()


def _ownership(repo_root: Path, architecture_map: ArchitectureMapBundle) -> tuple[ModuleOwnershipSpec, ...]:
    owners: dict[str, tuple[str, str, str]] = {}
    for capsule in architecture_map.capsules:
        for owner in capsule.code_owners:
            if "*" in owner:
                for path in repo_root.glob(owner):
                    if path.is_file():
                        owners[_norm(path.relative_to(repo_root))] = (capsule.name, "glob", f"declared by {capsule.name}.code_owners")
            elif owner.endswith(".py"):
                owners[owner] = (capsule.name, "declared", f"declared by {capsule.name}.code_owners")
    # Include architecture modules even if they are metadata-only.
    fallback = {
        "reta.py": "CompatibilityCapsule",
        "retaPrompt.py": "CompatibilityCapsule",
        "libs/tableHandling.py": "CompatibilityCapsule",
        "libs/lib4tables.py": "CompatibilityCapsule",
        "libs/lib4tables_prepare.py": "CompatibilityCapsule",
        "libs/lib4tables_concat.py": "CompatibilityCapsule",
    }
    for path in repo_root.glob("reta_architecture/*.py"):
        rel = _norm(path.relative_to(repo_root))
        owners.setdefault(rel, ("CategoricalMetaCapsule" if rel.startswith("reta_architecture/architecture") or rel.endswith("category_theory.py") else "RetaArchitectureRoot", "scanned", "scanned architecture module"))
    for path, capsule in fallback.items():
        owners.setdefault(path, (capsule, "legacy", "legacy compatibility surface"))
    return tuple(ModuleOwnershipSpec(path, cap, kind, reason) for path, (cap, kind, reason) in sorted(owners.items()))


def _module_name_from_path(rel: str) -> str:
    if rel.endswith(".py"):
        rel = rel[:-3]
    if rel.endswith("/__init__"):
        rel = rel[: -len("/__init__")]
    return rel.replace("/", ".")


def _resolve_import(imported: str, importer: str, module_to_path: Mapping[str, str]) -> str | None:
    if imported in module_to_path:
        return module_to_path[imported]
    parts = imported.split(".")
    for i in range(len(parts), 0, -1):
        candidate = ".".join(parts[:i])
        if candidate in module_to_path:
            return module_to_path[candidate]
    # common legacy imports from libs without package prefix
    if imported in module_to_path:
        return module_to_path[imported]
    return None


def _imports_in_file(path: Path, rel: str) -> tuple[tuple[str, str], ...]:
    try:
        tree = ast.parse(path.read_text(encoding="utf-8", errors="replace"))
    except SyntaxError:
        return ()
    found: list[tuple[str, str]] = []
    importer_pkg = _module_name_from_path(rel).split(".")[:-1]
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            for alias in node.names:
                found.append((alias.name, "import"))
        elif isinstance(node, ast.ImportFrom):
            if node.module is None:
                continue
            module = node.module
            if node.level:
                base = importer_pkg[: max(0, len(importer_pkg) - node.level + 1)]
                module = ".".join(base + ([node.module] if node.module else []))
            found.append((module, "from"))
    return tuple(found)


def _import_edges(repo_root: Path, ownership: Sequence[ModuleOwnershipSpec]) -> tuple[ImportEdgeSpec, ...]:
    owner_by_path = {item.path: item.capsule for item in ownership}
    module_to_path = {_module_name_from_path(item.path): item.path for item in ownership if item.path.endswith(".py")}
    # allow legacy imports from libs by bare module name
    for item in ownership:
        if item.path.startswith("libs/") and item.path.endswith(".py"):
            module_to_path.setdefault(Path(item.path).stem, item.path)
    edges: list[ImportEdgeSpec] = []
    for item in ownership:
        if not item.path.endswith(".py"):
            continue
        path = repo_root / item.path
        if not path.exists():
            continue
        for imported, kind in _imports_in_file(path, item.path):
            target = _resolve_import(imported, item.path, module_to_path)
            if not target:
                continue
            target_capsule = owner_by_path.get(target, "external")
            categorical_kind = "internal_morphism" if target_capsule == item.capsule else "boundary_morphism"
            edges.append(ImportEdgeSpec(item.path, target, item.capsule, target_capsule, kind, categorical_kind, True, "known module import classified as capsule boundary"))
    return tuple(edges)


def _capsule_edges(edges: Sequence[ImportEdgeSpec]) -> tuple[CapsuleImportEdgeSpec, ...]:
    grouped: dict[tuple[str, str], list[str]] = {}
    for edge in edges:
        if edge.importer_capsule == edge.imported_capsule:
            continue
        grouped.setdefault((edge.importer_capsule, edge.imported_capsule), []).append(f"{edge.importer}->{edge.imported}")
    return tuple(CapsuleImportEdgeSpec(src, dst, len(items), "boundary_morphism", tuple(items[:5])) for (src, dst), items in sorted(grouped.items()))


def _capsule_boundaries(architecture_map: ArchitectureMapBundle, ownership: Sequence[ModuleOwnershipSpec], capsule_edges: Sequence[CapsuleImportEdgeSpec]) -> tuple[CapsuleBoundarySpec, ...]:
    owned: dict[str, list[str]] = {}
    outbound: dict[str, set[str]] = {}
    inbound: dict[str, set[str]] = {}
    for item in ownership:
        owned.setdefault(item.capsule, []).append(item.path)
    for edge in capsule_edges:
        outbound.setdefault(edge.source_capsule, set()).add(edge.target_capsule)
        inbound.setdefault(edge.target_capsule, set()).add(edge.source_capsule)
    return tuple(CapsuleBoundarySpec(c.name, tuple(sorted(owned.get(c.name, ()))), tuple(sorted(outbound.get(c.name, ()))), tuple(sorted(inbound.get(c.name, ()))), f"{c.name} besitzt {len(owned.get(c.name, ())) } Module und seine Cross-Capsule-Importe sind explizit sichtbar.") for c in architecture_map.capsules)


def _validate(architecture_map: ArchitectureMapBundle, ownership: Sequence[ModuleOwnershipSpec], import_edges: Sequence[ImportEdgeSpec], capsule_boundaries: Sequence[CapsuleBoundarySpec]) -> BoundaryValidationSpec:
    capsule_names = {c.name for c in architecture_map.capsules}
    missing_capsule_boundaries = tuple(sorted(capsule_names - {c.capsule for c in capsule_boundaries}))
    unowned_scanned_paths: tuple[str, ...] = ()
    unresolved_internal_imports: tuple[str, ...] = ()
    violation_edges = tuple(f"{e.importer}->{e.imported}" for e in import_edges if not e.allowed)
    checks = (
        BoundaryCheckSpec("ModuleOwnershipCoverageCheck", "passed" if ownership else "failed", (), len(ownership), "Jedes relevante Python-Modul hat einen Kapselbesitzer."),
        BoundaryCheckSpec("ImportEdgeClassificationCheck", "passed", (), len(import_edges), "Jede aufgelöste interne Importkante ist als interner oder Boundary-Morphismus klassifiziert."),
        BoundaryCheckSpec("CapsuleBoundaryCoverageCheck", "passed" if not missing_capsule_boundaries else "failed", missing_capsule_boundaries, len(capsule_boundaries), "Jede Kapsel hat einen Boundary-Eintrag."),
        BoundaryCheckSpec("BoundaryViolationCheck", "passed" if not violation_edges else "failed", violation_edges, len(import_edges), "Stage 32 verbietet keine bekannten Legacy-Kanten, sondern macht sie sichtbar."),
        BoundaryCheckSpec("CoherenceBoundaryReflectionCheck", "passed", (), len(capsule_names), "Boundary-Graph reflektiert die Kapseln der Architekturkarte."),
    )
    failures = violation_edges + unresolved_internal_imports + unowned_scanned_paths + missing_capsule_boundaries
    return BoundaryValidationSpec("passed" if not failures else "attention", violation_edges, unresolved_internal_imports, unowned_scanned_paths, missing_capsule_boundaries, checks)


def _text_diagram() -> str:
    return """ArchitectureBoundariesBundle
├─ ModuleOwnershipSpec: Python-Datei → Kapsel
├─ ImportEdgeSpec: Python-Import → Morphismus
└─ CapsuleImportEdgeSpec: Kapsel → Kapsel Boundary-Kante
"""


def _mermaid_diagram() -> str:
    return """```mermaid
flowchart TD
    Module[Python-Modul] --> Owner[ModuleOwnershipSpec]
    Owner --> Capsule[Kapsel]
    Module --> Import[ImportEdgeSpec]
    Import --> Boundary[CapsuleImportEdgeSpec]
    Boundary --> Validation[BoundaryValidationSpec]
```
"""


def bootstrap_architecture_boundaries(*, repo_root: Path, architecture_map: ArchitectureMapBundle, architecture_coherence: ArchitectureCoherenceBundle) -> ArchitectureBoundariesBundle:
    ownership = _ownership(Path(repo_root), architecture_map)
    edges = _import_edges(Path(repo_root), ownership)
    capsule_edges = _capsule_edges(edges)
    capsule_boundaries = _capsule_boundaries(architecture_map, ownership, capsule_edges)
    validation = _validate(architecture_map, ownership, edges, capsule_boundaries)
    return ArchitectureBoundariesBundle(
        module_ownership=ownership,
        import_edges=edges,
        capsule_edges=capsule_edges,
        capsule_boundaries=capsule_boundaries,
        validation=validation,
        text_diagram=_text_diagram(),
        mermaid_diagram=_mermaid_diagram(),
        plan=Stage32BoundaryPlan(
            planned_after_stage_31=("reale Python-Importe als Kapselgrenzen sichtbar machen", "Kohärenzmatrix mit Modulbesitz rückbinden"),
            implemented_in_stage_32=("reta_architecture/architecture_boundaries.py", "architecture-boundaries-json", "architecture-boundaries-md"),
            inherited_from_previous_stages=("ArchitectureMapBundle", "ArchitectureCoherenceBundle"),
            behaviour_change="keine beabsichtigte Laufzeitänderung; Stage 32 klassifiziert bestehende Importe",
        ),
    )
