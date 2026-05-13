from __future__ import annotations

"""Stage-42 execution/progress layer for guarded Reta migrations.

Stage 34 ordered all impact candidates into migration waves, but later stages
already activated several planned moves (most visibly the Stage-37 to Stage-41
input/runtime extractions).  The repository therefore needs one more explicit
meta object: a progress overlay that compares the *planned* migration bundle
with the *currently observed* repository surfaces.

Mathematical reading:

* Stage-34 migration steps are objects in the migration category.
* concrete Python/data/doc surfaces are local sections over the current tree.
* progress classification is a functor from planned steps to observed facade /
  active-owner / retained-local-section states.
* wave summaries glue the local progress sections into one global status sheaf.
* outstanding work items are the still-nontrivial complements of the already
  commuting compatibility squares.
"""

import ast
import os
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Mapping, Sequence

from .architecture_activation import ArchitectureActivationBundle
from .architecture_migration import ArchitectureMigrationBundle, MigrationStepSpec, MigrationWaveSpec


KNOWN_COMPATIBILITY_FACADES = {
    "i18n/words.py",
    "reta.py",
    "retaPrompt.py",
    "libs/center.py",
    "libs/LibRetaPrompt.py",
    "libs/nestedAlx.py",
    "libs/word_completerAlx.py",
    "libs/lib4tables.py",
    "libs/tableHandling.py",
    "libs/lib4tables_prepare.py",
    "libs/lib4tables_concat.py",
}
KNOWN_MIXED_DATA_OWNERS = {
    "libs/lib4tables_Enum.py",
}
TAG_SCHEMA_OWNER = "reta_architecture/tag_schema.py"
OPTIONAL_FACADE_THINNING = {
    "libs/center.py",
    "libs/lib4tables_prepare.py",
    "libs/lib4tables_concat.py",
    "reta.py",
    "retaPrompt.py",
    "libs/LibRetaPrompt.py",
}
ACTIVE_OWNER_STATUSES = {
    "active_architecture_owner",
    "extracted_to_compatibility_facade",
    "retained_local_section",
    "status_document_present",
}
MIXED_STATUSES = {"still_mixed", "still_mixed_data_owner", "docs_distributed"}


@dataclass(frozen=True)
class LegacySurfaceProgressSpec:
    owner: str
    owner_kind: str
    path: str
    exists: bool
    line_count: int
    function_count: int
    wrapper_like_count: int
    architecture_imports: Sequence[str]
    execution_status: str
    evidence: Sequence[str]
    remaining_work: Sequence[str]
    reading: str

    def snapshot(self) -> dict:
        return {
            "owner": self.owner,
            "owner_kind": self.owner_kind,
            "path": self.path,
            "exists": self.exists,
            "line_count": self.line_count,
            "function_count": self.function_count,
            "wrapper_like_count": self.wrapper_like_count,
            "architecture_imports": list(self.architecture_imports),
            "execution_status": self.execution_status,
            "evidence": list(self.evidence),
            "remaining_work": list(self.remaining_work),
            "reading": self.reading,
        }


@dataclass(frozen=True)
class MigrationExecutionSpec:
    step_id: str
    wave_id: str
    candidate: str
    legacy_owner: str
    target_owner: str
    planned_status: str
    execution_status: str
    owner_kind: str
    evidence: Sequence[str]
    remaining_work: Sequence[str]
    reading: str

    def snapshot(self) -> dict:
        return {
            "step_id": self.step_id,
            "wave_id": self.wave_id,
            "candidate": self.candidate,
            "legacy_owner": self.legacy_owner,
            "target_owner": self.target_owner,
            "planned_status": self.planned_status,
            "execution_status": self.execution_status,
            "owner_kind": self.owner_kind,
            "evidence": list(self.evidence),
            "remaining_work": list(self.remaining_work),
            "reading": self.reading,
        }


@dataclass(frozen=True)
class WaveExecutionSpec:
    wave_id: str
    name: str
    total_steps: int
    completed_steps: int
    mixed_steps: int
    outstanding_steps: int
    step_statuses: Mapping[str, int]
    remaining_owners: Sequence[str]
    status: str

    def snapshot(self) -> dict:
        return {
            "wave_id": self.wave_id,
            "name": self.name,
            "total_steps": self.total_steps,
            "completed_steps": self.completed_steps,
            "mixed_steps": self.mixed_steps,
            "outstanding_steps": self.outstanding_steps,
            "step_statuses": dict(self.step_statuses),
            "remaining_owners": list(self.remaining_owners),
            "status": self.status,
        }


@dataclass(frozen=True)
class OutstandingWorkItemSpec:
    item_id: str
    priority: str
    title: str
    owners: Sequence[str]
    reason: str
    recommended_next_step: str
    status: str

    def snapshot(self) -> dict:
        return {
            "item_id": self.item_id,
            "priority": self.priority,
            "title": self.title,
            "owners": list(self.owners),
            "reason": self.reason,
            "recommended_next_step": self.recommended_next_step,
            "status": self.status,
        }


@dataclass(frozen=True)
class ProgressCheckSpec:
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
class ProgressValidationSpec:
    status: str
    steps_without_surface: Sequence[str]
    inconsistent_wave_counts: Sequence[str]
    mixed_owners: Sequence[str]
    outstanding_items: Sequence[str]
    checks: Sequence[ProgressCheckSpec]

    def snapshot(self) -> dict:
        return {
            "status": self.status,
            "steps_without_surface": list(self.steps_without_surface),
            "inconsistent_wave_counts": list(self.inconsistent_wave_counts),
            "mixed_owners": list(self.mixed_owners),
            "outstanding_items": list(self.outstanding_items),
            "checks": [item.snapshot() for item in self.checks],
        }


@dataclass(frozen=True)
class Stage42ArchitecturePlan:
    planned_after_stage_41: Sequence[str]
    implemented_in_stage_42: Sequence[str]
    inherited_from_previous_stages: Sequence[str]
    behaviour_change: str

    def snapshot(self) -> dict:
        return {
            "planned_after_stage_41": list(self.planned_after_stage_41),
            "implemented_in_stage_42": list(self.implemented_in_stage_42),
            "inherited_from_previous_stages": list(self.inherited_from_previous_stages),
            "behaviour_change": self.behaviour_change,
        }


@dataclass(frozen=True)
class ArchitectureProgressBundle:
    surfaces: Sequence[LegacySurfaceProgressSpec]
    step_progress: Sequence[MigrationExecutionSpec]
    wave_progress: Sequence[WaveExecutionSpec]
    outstanding_work: Sequence[OutstandingWorkItemSpec]
    validation: ProgressValidationSpec
    text_diagram: str
    mermaid_diagram: str
    plan: Stage42ArchitecturePlan

    def surface_named(self, owner: str) -> LegacySurfaceProgressSpec:
        for item in self.surfaces:
            if item.owner == owner:
                return item
        raise KeyError(f"Unknown progress owner: {owner}")

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "stage": 42,
            "purpose": "Stage-42 execution overlay: planned migration steps are compared with the actually observed facade/data/doc ownership of the repository.",
            "paradigm": [
                "topology",
                "morphism",
                "universal_property",
                "presheaf",
                "sheaf",
                "category",
                "functor",
                "natural_transformation",
                "progress_overlay",
                "compatibility_facade",
                "outstanding_work",
            ],
            "counts": {
                "surfaces": len(self.surfaces),
                "step_progress": len(self.step_progress),
                "wave_progress": len(self.wave_progress),
                "outstanding_work": len(self.outstanding_work),
                "checks": len(self.validation.checks),
            },
            "surfaces": [item.snapshot() for item in self.surfaces],
            "step_progress": [item.snapshot() for item in self.step_progress],
            "wave_progress": [item.snapshot() for item in self.wave_progress],
            "outstanding_work": [item.snapshot() for item in self.outstanding_work],
            "validation": self.validation.snapshot(),
            "diagrams": {"text": self.text_diagram, "mermaid": self.mermaid_diagram},
            "plan": self.plan.snapshot(),
        }


def _norm(path: str | Path) -> str:
    return str(path).replace("\\", "/").lstrip("./")


def _iter_function_nodes(tree: ast.AST) -> Iterable[ast.FunctionDef | ast.AsyncFunctionDef]:
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            yield node


def _strip_docstring(body: Sequence[ast.stmt]) -> list[ast.stmt]:
    items = list(body)
    if items and isinstance(items[0], ast.Expr) and isinstance(getattr(items[0], "value", None), ast.Constant) and isinstance(items[0].value.value, str):
        return items[1:]
    return items


def _architecture_import_names(tree: ast.AST) -> tuple[tuple[str, ...], set[str]]:
    modules: list[str] = []
    names: set[str] = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            for alias in node.names:
                if alias.name.startswith("reta_architecture"):
                    modules.append(alias.name)
                    names.add(alias.asname or alias.name.split(".")[-1])
        elif isinstance(node, ast.ImportFrom):
            if (node.module or "").startswith("reta_architecture"):
                modules.append(node.module or "")
                for alias in node.names:
                    names.add(alias.asname or alias.name)
    return tuple(dict.fromkeys(modules)), names


def _mentions_architecture(expr: ast.AST, imported_names: set[str]) -> bool:
    for node in ast.walk(expr):
        if isinstance(node, ast.Name):
            if node.id in imported_names:
                return True
            if node.id.endswith("_MORPHISMS") or node.id.endswith("_morphisms"):
                return True
        elif isinstance(node, ast.Attribute):
            if node.attr == "architecture":
                return True
            if isinstance(node.value, ast.Name) and node.value.id in imported_names:
                return True
    return False


def _is_wrapper_like(function_node: ast.FunctionDef | ast.AsyncFunctionDef, imported_names: set[str]) -> bool:
    body = _strip_docstring(function_node.body)
    if len(body) != 1:
        return False
    stmt = body[0]
    if isinstance(stmt, ast.Return) and stmt.value is not None:
        return _mentions_architecture(stmt.value, imported_names)
    if isinstance(stmt, ast.Expr):
        return _mentions_architecture(stmt.value, imported_names)
    return False


@dataclass(frozen=True)
class _PythonFileAnalysis:
    exists: bool
    line_count: int
    function_count: int
    wrapper_like_count: int
    architecture_imports: tuple[str, ...]
    doc_has_facade_marker: bool


def _analyse_python_file(path: Path) -> _PythonFileAnalysis:
    if not path.exists():
        return _PythonFileAnalysis(False, 0, 0, 0, (), False)
    source = path.read_text(encoding="utf-8")
    tree = ast.parse(source)
    modules, names = _architecture_import_names(tree)
    function_nodes = tuple(_iter_function_nodes(tree))
    wrapper_like_count = sum(1 for node in function_nodes if _is_wrapper_like(node, names))
    facade_marker = any(
        marker in source.lower()
        for marker in (
            "compatibility facade",
            "legacy compatibility facade",
            "legacy wrapper",
            "legacy-kompatible",
            "programmfassade",
            "compatibility bundles",
        )
    )
    return _PythonFileAnalysis(
        exists=True,
        line_count=len(source.splitlines()),
        function_count=len(function_nodes),
        wrapper_like_count=wrapper_like_count,
        architecture_imports=modules,
        doc_has_facade_marker=facade_marker,
    )




def _git_progress_command(repo_root: Path, *args: str) -> subprocess.CompletedProcess | None:
    repo_root = Path(repo_root).resolve()
    try:
        return subprocess.run(
            [
                'git',
                '-c',
                f'safe.directory={repo_root}',
                '-C',
                str(repo_root),
                *args,
            ],
            capture_output=True,
            text=True,
            timeout=10,
        )
    except (FileNotFoundError, subprocess.TimeoutExpired):
        return None


def _parity_reference_state(repo_root: Path) -> tuple[bool, str]:
    reference = Path('/mnt/data/reta.todel.zip')
    if reference.exists():
        return True, f'external archive available: {reference}'
    requested_commit = os.environ.get('RETA_PARITY_BASELINE_COMMIT')
    if requested_commit:
        proc = _git_progress_command(repo_root, 'rev-parse', '--verify', requested_commit)
        if proc is not None and proc.returncode == 0:
            return True, f'git baseline commit available: {requested_commit}'
    proc = _git_progress_command(repo_root, 'rev-list', '--max-parents=0', 'HEAD')
    if proc is None or proc.returncode != 0:
        return False, 'no external archive and no readable git baseline available'
    for line in proc.stdout.splitlines():
        commit = line.strip()
        if commit:
            return True, f'git baseline commit available: {commit}'
    return False, 'no external archive and no readable git baseline available'


def _surface_for_owner(repo_root: Path, owner: str) -> LegacySurfaceProgressSpec:
    if owner == "csv/*.csv":
        csv_files = tuple(sorted(_norm(path.relative_to(repo_root)) for path in (repo_root / "csv").glob("*.csv")))
        return LegacySurfaceProgressSpec(
            owner=owner,
            owner_kind="local_section_data",
            path="csv/*.csv",
            exists=bool(csv_files),
            line_count=0,
            function_count=0,
            wrapper_like_count=0,
            architecture_imports=(),
            execution_status="retained_local_section",
            evidence=(f"{len(csv_files)} CSV local sections discovered", "CSV files remain intentional presheaf/local-section data"),
            remaining_work=(),
            reading="CSV tables remain deliberately local sections of the data presheaf; they are not a code-smell owner that still needs extraction.",
        )

    if owner == "readme*.md / doc/*.md":
        root_docs = tuple(sorted(_norm(path.relative_to(repo_root)) for path in repo_root.glob("readme*.md")))
        doc_docs = tuple(sorted(_norm(path.relative_to(repo_root)) for path in (repo_root / "doc").glob("*.md")))
        current_status_doc = repo_root / "ARCHITECTURE_STATUS.md"
        stage_status_doc = repo_root / "ARCHITECTURE_STATUS_STAGE42.md"
        has_status_doc = current_status_doc.exists() or stage_status_doc.exists()
        remaining = () if has_status_doc else ("collapse the distributed stage history into one current architecture status document",)
        return LegacySurfaceProgressSpec(
            owner=owner,
            owner_kind="documentation_surface",
            path="readme*.md + doc/*.md",
            exists=bool(root_docs or doc_docs),
            line_count=0,
            function_count=0,
            wrapper_like_count=0,
            architecture_imports=(),
            execution_status="status_document_present" if has_status_doc else "docs_distributed",
            evidence=(
                f"{len(root_docs)} root markdown files discovered",
                f"{len(doc_docs)} doc markdown files discovered",
                "consolidated current-status document present" if has_status_doc else "stage history still distributed across many markdown files",
            ),
            remaining_work=remaining,
            reading="Documentation is intentionally a local repository section, but a consolidated current-status document should condense the scattered stage history into one current view.",
        )

    path = repo_root / owner
    analysis = _analyse_python_file(path) if owner.endswith(".py") else _PythonFileAnalysis(path.exists(), 0, 0, 0, (), False)

    if owner.startswith("reta_architecture/"):
        evidence = ["architecture owner module present"]
        if analysis.line_count:
            evidence.append(f"{analysis.line_count} lines")
        if analysis.architecture_imports:
            evidence.append(f"imports {len(analysis.architecture_imports)} architecture modules")
        return LegacySurfaceProgressSpec(
            owner=owner,
            owner_kind="architecture_owner",
            path=owner,
            exists=analysis.exists,
            line_count=analysis.line_count,
            function_count=analysis.function_count,
            wrapper_like_count=analysis.wrapper_like_count,
            architecture_imports=analysis.architecture_imports,
            execution_status="active_architecture_owner",
            evidence=tuple(evidence),
            remaining_work=(),
            reading="This owner already lives directly inside reta_architecture and therefore counts as an active architecture owner rather than pending migration work.",
        )

    if owner in KNOWN_MIXED_DATA_OWNERS:
        tag_schema_owner = repo_root / TAG_SCHEMA_OWNER
        if tag_schema_owner.exists():
            evidence = [f"{analysis.line_count} lines", f"dedicated owner {TAG_SCHEMA_OWNER} present"]
            if analysis.architecture_imports:
                evidence.append(f"imports {len(analysis.architecture_imports)} architecture modules")
            if analysis.doc_has_facade_marker:
                evidence.append("module describes itself as a compatibility facade")
            return LegacySurfaceProgressSpec(
                owner=owner,
                owner_kind="legacy_compatibility_surface",
                path=owner,
                exists=analysis.exists,
                line_count=analysis.line_count,
                function_count=analysis.function_count,
                wrapper_like_count=analysis.wrapper_like_count,
                architecture_imports=analysis.architecture_imports,
                execution_status="extracted_to_compatibility_facade",
                evidence=tuple(evidence),
                remaining_work=(),
                reading="The historical lib4tables_Enum surface now re-exports the dedicated Stage-42 tag-schema owner from reta_architecture.",
            )
        return LegacySurfaceProgressSpec(
            owner=owner,
            owner_kind="legacy_data_owner",
            path=owner,
            exists=analysis.exists,
            line_count=analysis.line_count,
            function_count=analysis.function_count,
            wrapper_like_count=analysis.wrapper_like_count,
            architecture_imports=analysis.architecture_imports,
            execution_status="still_mixed_data_owner",
            evidence=(
                f"{analysis.line_count} lines",
                "no dedicated architecture tag-schema owner exists yet",
            ),
            remaining_work=(
                "extract tag tables / enum data into a dedicated architecture or local-section owner",
            ),
            reading="The file still owns large tag/data tables directly and is the clearest remaining mixed owner in the migration plan.",
        )

    is_facade = owner in KNOWN_COMPATIBILITY_FACADES or (
        analysis.exists
        and (
            analysis.doc_has_facade_marker
            or (analysis.architecture_imports and analysis.wrapper_like_count >= max(1, analysis.function_count // 3))
            or (owner == "i18n/words.py" and analysis.line_count <= 80)
        )
    )

    if is_facade:
        remaining: tuple[str, ...] = ()
        if owner in OPTIONAL_FACADE_THINNING:
            remaining = ("optional: thin the remaining compatibility shell even further if a future stage wants a near-zero wrapper surface",)
        evidence = [f"{analysis.line_count} lines"]
        if analysis.architecture_imports:
            evidence.append(f"imports {len(analysis.architecture_imports)} architecture modules")
        if analysis.wrapper_like_count:
            evidence.append(f"{analysis.wrapper_like_count}/{analysis.function_count} functions look wrapper-like")
        if analysis.doc_has_facade_marker:
            evidence.append("module describes itself as a compatibility facade")
        return LegacySurfaceProgressSpec(
            owner=owner,
            owner_kind="legacy_compatibility_surface",
            path=owner,
            exists=analysis.exists,
            line_count=analysis.line_count,
            function_count=analysis.function_count,
            wrapper_like_count=analysis.wrapper_like_count,
            architecture_imports=analysis.architecture_imports,
            execution_status="extracted_to_compatibility_facade",
            evidence=tuple(evidence),
            remaining_work=remaining,
            reading="The old import/API surface still exists, but the owning behaviour has already moved into reta_architecture and the file now acts as a compatibility shell.",
        )

    return LegacySurfaceProgressSpec(
        owner=owner,
        owner_kind="legacy_mixed_surface",
        path=owner,
        exists=analysis.exists,
        line_count=analysis.line_count,
        function_count=analysis.function_count,
        wrapper_like_count=analysis.wrapper_like_count,
        architecture_imports=analysis.architecture_imports,
        execution_status="still_mixed",
        evidence=(f"{analysis.line_count} lines", f"{analysis.wrapper_like_count}/{analysis.function_count} functions look wrapper-like"),
        remaining_work=("reduce remaining local logic until the file is only a compatibility facade or explicit local-section owner",),
        reading="The file still contains enough local ownership that the migration should not yet be considered finished.",
    )


def _execution_status_for_surface(surface: LegacySurfaceProgressSpec) -> str:
    if surface.execution_status == "active_architecture_owner":
        return "active_architecture_owner"
    if surface.execution_status == "extracted_to_compatibility_facade":
        return "extracted_to_compatibility_facade"
    if surface.execution_status in {"retained_local_section", "status_document_present"}:
        return "retained_local_section"
    if surface.execution_status == "docs_distributed":
        return "documentation_pending_consolidation"
    if surface.execution_status == "still_mixed_data_owner":
        return "mixed_data_owner"
    return "still_mixed"


def _step_progress(steps: Sequence[MigrationStepSpec], surfaces: Mapping[str, LegacySurfaceProgressSpec]) -> tuple[MigrationExecutionSpec, ...]:
    result: list[MigrationExecutionSpec] = []
    for step in steps:
        surface = surfaces[step.legacy_owner]
        status = _execution_status_for_surface(surface)
        result.append(
            MigrationExecutionSpec(
                step_id=step.step_id,
                wave_id=step.wave_id,
                candidate=step.candidate,
                legacy_owner=step.legacy_owner,
                target_owner=step.target_owner,
                planned_status=step.status,
                execution_status=status,
                owner_kind=surface.owner_kind,
                evidence=tuple(surface.evidence),
                remaining_work=tuple(surface.remaining_work),
                reading=f"{step.step_id} was planned as {step.status}; the observed repository surface now classifies {step.legacy_owner} as {status}.",
            )
        )
    return tuple(result)


def _wave_progress(waves: Sequence[MigrationWaveSpec], steps: Sequence[MigrationExecutionSpec]) -> tuple[WaveExecutionSpec, ...]:
    by_wave: dict[str, list[MigrationExecutionSpec]] = {}
    for step in steps:
        by_wave.setdefault(step.wave_id, []).append(step)
    items: list[WaveExecutionSpec] = []
    for wave in waves:
        members = by_wave.get(wave.wave_id, [])
        status_counts: dict[str, int] = {}
        for member in members:
            status_counts[member.execution_status] = status_counts.get(member.execution_status, 0) + 1
        completed = sum(1 for member in members if member.execution_status in ACTIVE_OWNER_STATUSES)
        mixed = sum(1 for member in members if member.execution_status in {"mixed_data_owner", "still_mixed", "documentation_pending_consolidation"})
        outstanding = len(members) - completed
        remaining_owners = tuple(sorted({member.legacy_owner for member in members if member.execution_status not in ACTIVE_OWNER_STATUSES}))
        if not members:
            status = "empty_wave"
        elif outstanding == 0:
            status = "implemented_or_retained"
        elif mixed > 0:
            status = "partially_remaining"
        else:
            status = "in_transition"
        items.append(
            WaveExecutionSpec(
                wave_id=wave.wave_id,
                name=wave.name,
                total_steps=len(members),
                completed_steps=completed,
                mixed_steps=mixed,
                outstanding_steps=outstanding,
                step_statuses=status_counts,
                remaining_owners=remaining_owners,
                status=status,
            )
        )
    return tuple(items)


def _outstanding_work(surfaces: Sequence[LegacySurfaceProgressSpec], repo_root: Path) -> tuple[OutstandingWorkItemSpec, ...]:
    items: list[OutstandingWorkItemSpec] = []
    next_id = 1
    for surface in surfaces:
        if surface.execution_status == "still_mixed_data_owner":
            items.append(
                OutstandingWorkItemSpec(
                    item_id=f"WIP42-{next_id:02d}",
                    priority="high",
                    title="Extract remaining tag/data owner from lib4tables_Enum",
                    owners=(surface.owner,),
                    reason="The file still owns large tag/data tables directly and is not yet reduced to a compatibility facade or local-section-only declaration.",
                    recommended_next_step="Create a dedicated architecture/local-section owner for tag schema data and turn lib4tables_Enum.py into a thin re-export module.",
                    status="open",
                )
            )
            next_id += 1
        elif surface.execution_status == "docs_distributed":
            items.append(
                OutstandingWorkItemSpec(
                    item_id=f"WIP42-{next_id:02d}",
                    priority="medium",
                    title="Condense scattered stage documentation into one current status file",
                    owners=(surface.owner,),
                    reason="The repository history is rich but spread across many stage and audit markdown files.",
                    recommended_next_step="Add a current architecture status document that names what is truly still open after the Stage-37 to Stage-41 activations.",
                    status="open",
                )
            )
            next_id += 1
    parity_available, parity_reading = _parity_reference_state(repo_root)
    if not parity_available:
        items.append(
            OutstandingWorkItemSpec(
                item_id=f"WIP42-{next_id:02d}",
                priority="medium",
                title="Restore original reference archive for command parity",
                owners=("tests/test_command_parity.py",),
                reason="The parity suite cannot currently resolve any baseline archive, neither from /mnt/data/reta.todel.zip nor from the repository history.",
                recommended_next_step="Provide /mnt/data/reta.todel.zip or make the Git baseline readable so tests/test_command_parity.py can synthesize its original comparison archive.",
                status="environment-blocked",
            )
        )
    return tuple(items)


def _validation(
    migration: ArchitectureMigrationBundle,
    surfaces: Sequence[LegacySurfaceProgressSpec],
    step_progress: Sequence[MigrationExecutionSpec],
    wave_progress: Sequence[WaveExecutionSpec],
    outstanding_work: Sequence[OutstandingWorkItemSpec],
) -> ProgressValidationSpec:
    surface_names = {item.owner for item in surfaces}
    steps_without_surface = tuple(sorted(step.legacy_owner for step in migration.steps if step.legacy_owner not in surface_names))
    inconsistent_wave_counts: list[str] = []
    step_counts_by_wave: dict[str, int] = {}
    for step in step_progress:
        step_counts_by_wave[step.wave_id] = step_counts_by_wave.get(step.wave_id, 0) + 1
    for wave in wave_progress:
        if wave.total_steps != step_counts_by_wave.get(wave.wave_id, 0):
            inconsistent_wave_counts.append(wave.wave_id)
    mixed_owners = tuple(sorted(item.owner for item in surfaces if item.execution_status in MIXED_STATUSES))
    outstanding_ids = tuple(item.item_id for item in outstanding_work)
    checks = (
        ProgressCheckSpec(
            "ProgressSurfaceCoverageCheck",
            "passed" if not steps_without_surface else "failed",
            steps_without_surface,
            len(surfaces),
            "Every Stage-34 migration owner should have one observed Stage-42 surface classification.",
        ),
        ProgressCheckSpec(
            "ProgressWaveCountCheck",
            "passed" if not inconsistent_wave_counts else "failed",
            tuple(inconsistent_wave_counts),
            len(wave_progress),
            "Wave step counts derived from the progress overlay should match the underlying migration bundle.",
        ),
        ProgressCheckSpec(
            "ProgressOutstandingWorkCheck",
            "attention" if outstanding_ids else "passed",
            outstanding_ids,
            len(outstanding_work),
            "Outstanding work items are informational: they show what still blocks a fully closed progress overlay.",
        ),
    )
    failures = steps_without_surface + tuple(inconsistent_wave_counts)
    status = "passed" if not failures and not mixed_owners and not outstanding_ids else "attention"
    return ProgressValidationSpec(
        status=status,
        steps_without_surface=steps_without_surface,
        inconsistent_wave_counts=tuple(inconsistent_wave_counts),
        mixed_owners=mixed_owners,
        outstanding_items=outstanding_ids,
        checks=checks,
    )


def _text_diagram(waves: Sequence[WaveExecutionSpec], outstanding_work: Sequence[OutstandingWorkItemSpec]) -> str:
    lines = [
        "ArchitectureProgressBundle",
        "├─ surfaces: observed owners / facades / local sections",
        "├─ step_progress: Stage-34 planned steps → Stage-42 execution overlay",
        "├─ wave_progress:",
    ]
    for index, wave in enumerate(waves):
        connector = "├" if index < len(waves) - 1 else "└"
        lines.append(
            f"│  {connector}─ {wave.wave_id}: {wave.status} ({wave.completed_steps}/{wave.total_steps} completed, {wave.outstanding_steps} outstanding)"
        )
    if outstanding_work:
        lines.append("└─ outstanding_work:")
        for index, item in enumerate(outstanding_work):
            connector = "├" if index < len(outstanding_work) - 1 else "└"
            lines.append(f"   {connector}─ {item.item_id}: {item.title} [{item.status}]")
    return "\n".join(lines) + "\n"


def _mermaid_diagram() -> str:
    return """```mermaid
flowchart TD
    Plan[Stage-34 migration steps] --> Surface[Observed repository surfaces]
    Surface --> Step[MigrationExecutionSpec]
    Step --> Wave[WaveExecutionSpec]
    Wave --> Out[OutstandingWorkItemSpec]
    Out --> Validation[ProgressValidationSpec]
```
"""


def bootstrap_architecture_progress(
    *,
    repo_root: Path,
    architecture_migration: ArchitectureMigrationBundle,
    architecture_activation: ArchitectureActivationBundle,
) -> ArchitectureProgressBundle:
    del architecture_activation  # kept as explicit dependency anchor for the Stage-36→42 route
    repo_root = Path(repo_root).resolve()
    owners = tuple(dict.fromkeys(step.legacy_owner for step in architecture_migration.steps))
    surfaces = tuple(_surface_for_owner(repo_root, owner) for owner in owners)
    surface_map = {item.owner: item for item in surfaces}
    step_progress = _step_progress(architecture_migration.steps, surface_map)
    wave_progress = _wave_progress(architecture_migration.waves, step_progress)
    outstanding_work = _outstanding_work(surfaces, repo_root)
    validation = _validation(architecture_migration, surfaces, step_progress, wave_progress, outstanding_work)
    return ArchitectureProgressBundle(
        surfaces=surfaces,
        step_progress=step_progress,
        wave_progress=wave_progress,
        outstanding_work=outstanding_work,
        validation=validation,
        text_diagram=_text_diagram(wave_progress, outstanding_work),
        mermaid_diagram=_mermaid_diagram(),
        plan=Stage42ArchitecturePlan(
            planned_after_stage_41=(
                "turn the outdated Stage-34 'planned' view into an explicit observed progress overlay",
                "distinguish real compatibility facades from still-mixed legacy/data owners",
                "name the truly remaining work after Stage-37 to Stage-41 runtime activations",
            ),
            implemented_in_stage_42=(
                "reta_architecture/architecture_progress.py",
                "reta_architecture/tag_schema.py",
                "libs/lib4tables_Enum.py facade reduction",
                "architecture-progress-json",
                "architecture-progress-md",
                "ARCHITECTURE_STATUS_STAGE42.md",
            ),
            inherited_from_previous_stages=(
                "ArchitectureMigrationBundle",
                "ArchitectureActivationBundle",
                "Stage-37 row-range activation",
                "Stage-38 arithmetic activation",
                "Stage-39 console/help activation",
                "Stage-40 word-completion activation",
                "Stage-41 nested-completion activation",
            ),
            behaviour_change="No runtime behaviour change. Stage 42 only adds an explicit repository-status overlay over the already existing migration plan.",
        ),
    )
