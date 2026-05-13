from __future__ import annotations

import hashlib
import os
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Mapping, Sequence


IGNORED_DIR_NAMES = {"__pycache__", ".git", ".pytest_cache", ".mypy_cache"}
IGNORED_SUFFIXES = {".pyc", ".pyo"}

REQUIRED_SOURCE_PATHS = (
    "reta.py",
    "retaPrompt.py",
    "reta_architecture/__init__.py",
    "reta_architecture/facade.py",
    "reta_architecture/topology.py",
    "reta_architecture/presheaves.py",
    "reta_architecture/sheaves.py",
    "reta_architecture/morphisms.py",
    "reta_architecture/universal.py",
    "reta_architecture/category_theory.py",
    "reta_architecture/architecture_map.py",
    "reta_architecture/architecture_contracts.py",
    "reta_architecture/architecture_witnesses.py",
    "reta_architecture/architecture_validation.py",
    "reta_architecture/architecture_coherence.py",
    "reta_architecture/architecture_traces.py",
    "reta_architecture/architecture_boundaries.py",
    "reta_architecture/architecture_impact.py",
    "reta_architecture/architecture_migration.py",
    "reta_architecture/architecture_rehearsal.py",
    "reta_architecture/architecture_activation.py",
    "reta_architecture/architecture_progress.py",
    "reta_architecture/parallel_execution.py",
    "reta_architecture/execution_network.py",
    "reta_architecture/persistence.py",
    "reta_architecture/schema.py",
    "reta_architecture/tag_schema.py",
    "reta_architecture/semantics_builder.py",
    "reta_architecture/input_semantics.py",
    "reta_architecture/row_ranges.py",
    "reta_architecture/arithmetic.py",
    "reta_architecture/console_io.py",
    "reta_architecture/completion_word.py",
    "reta_architecture/completion_nested.py",
    "reta_architecture/column_selection.py",
    "reta_architecture/parameter_runtime.py",
    "reta_architecture/program_workflow.py",
    "reta_architecture/output_syntax.py",
    "reta_architecture/output_semantics.py",
    "reta_architecture/table_generation.py",
    "reta_architecture/table_preparation.py",
    "reta_architecture/row_filtering.py",
    "reta_architecture/table_wrapping.py",
    "reta_architecture/table_state.py",
    "reta_architecture/number_theory.py",
    "reta_architecture/table_output.py",
    "reta_architecture/table_runtime.py",
    "reta_architecture/generated_columns.py",
    "reta_architecture/meta_columns.py",
    "reta_architecture/concat_csv.py",
    "reta_architecture/combi_join.py",
    "reta_architecture/prompt_runtime.py",
    "reta_architecture/completion_runtime.py",
    "reta_architecture/prompt_language.py",
    "reta_architecture/prompt_session.py",
    "reta_architecture/prompt_execution.py",
    "reta_architecture/prompt_preparation.py",
    "reta_architecture/prompt_interaction.py",
    "i18n/words.py",
    "i18n/words_context.py",
    "i18n/words_matrix.py",
    "i18n/words_runtime.py",
    "libs/center.py",
    "libs/LibRetaPrompt.py",
    "libs/nestedAlx.py",
    "libs/word_completerAlx.py",
    "libs/tableHandling.py",
    "libs/lib4tables.py",
    "libs/lib4tables_concat.py",
    "libs/lib4tables_prepare.py",
    "csv/religion.csv",
    "csv/vn-religion.csv",
    "tests/test_architecture_refactor.py",
    "tests/test_command_parity.py",
)


def _normalise_path(path: str | Path) -> str:
    return str(path).replace("\\", "/").lstrip("./")


def is_runtime_artifact(path: str | Path) -> bool:
    path = Path(path)
    if any(part in IGNORED_DIR_NAMES for part in path.parts):
        return True
    return path.suffix in IGNORED_SUFFIXES


def _iter_all_regular_files(root: Path) -> Iterable[Path]:
    root = Path(root)
    for dirpath, dirnames, filenames in os.walk(root, followlinks=False):
        dirnames[:] = sorted(dirnames)
        for filename in sorted(filenames):
            path = Path(dirpath) / filename
            if path.is_file():
                yield path


def iter_manifest_files(root: Path) -> Iterable[Path]:
    root = Path(root)
    for path in _iter_all_regular_files(root):
        relative = path.relative_to(root)
        if is_runtime_artifact(relative):
            continue
        yield path



def _manifest_file_entry(root: Path, relative: str) -> tuple[str, int, bytes, int | None]:
    path = root / relative
    if not path.exists() and not relative.startswith("."):
        dotted = root / ("." + relative)
        if dotted.exists():
            path = dotted
    data = path.read_bytes()
    line_count = None
    if relative.startswith("csv/") and relative.endswith(".csv"):
        line_count = len(data.decode("utf-8", errors="replace").splitlines())
    return relative, len(data), hashlib.sha256(data).digest(), line_count


def _manifest_file_worker(payload):
    root_string, relatives = payload
    root = Path(root_string)
    return [_manifest_file_entry(root, str(relative)) for relative in relatives]


def _manifest_entries_parallel(root: Path, files: Sequence[str]):
    try:
        from .parallel_execution import ParallelExecutionConfig, _chunks, _pool_map_ordered

        config = ParallelExecutionConfig.from_environment()
        if not config.should_use_processes(len(files)):
            return None
        chunks = list(_chunks(list(files), config.chunk_size))
        if len(chunks) <= 1:
            return None
        payloads = [(str(root), chunk) for chunk in chunks]
        chunk_results, workers = _pool_map_ordered(_manifest_file_worker, payloads, config)
        entries = []
        for chunk in chunk_results:
            entries.extend(chunk)
        return sorted(entries, key=lambda item: item[0])
    except Exception:
        return None

@dataclass(frozen=True)
class RepoManifest:
    root: str
    file_count: int
    total_bytes: int
    digest: str
    files: tuple[str, ...]
    missing_required: tuple[str, ...]
    runtime_artifact_count: int
    csv_line_counts: Mapping[str, int]
    suspicious_csvs: tuple[str, ...]

    @classmethod
    def from_tree(cls, root: Path, required_paths: Sequence[str] = REQUIRED_SOURCE_PATHS) -> "RepoManifest":
        root = Path(root).resolve()
        digest = hashlib.sha256()
        files: list[str] = []
        runtime_artifact_count = 0
        for path in _iter_all_regular_files(root):
            relative_path = path.relative_to(root)
            if is_runtime_artifact(relative_path):
                runtime_artifact_count += 1
                continue
            files.append(_normalise_path(relative_path))

        entries = _manifest_entries_parallel(root, files)
        if entries is None:
            entries = [_manifest_file_entry(root, relative) for relative in files]
        entries = sorted(entries, key=lambda item: item[0])

        total_bytes = 0
        csv_line_counts = {}
        for relative, byte_count, file_digest, line_count in entries:
            total_bytes += byte_count
            digest.update(relative.encode("utf-8"))
            digest.update(b"\0")
            digest.update(file_digest)
            if line_count is not None:
                csv_line_counts[relative] = int(line_count)

        file_set = set(files)
        suspicious_csvs = tuple(
            relative
            for relative, line_count in csv_line_counts.items()
            if relative.endswith("religion.csv") and line_count < 500
        )
        missing_required = tuple(path for path in required_paths if _normalise_path(path) not in file_set)
        return cls(
            root=str(root),
            file_count=len(files),
            total_bytes=total_bytes,
            digest=digest.hexdigest(),
            files=tuple(sorted(files)),
            missing_required=missing_required,
            runtime_artifact_count=runtime_artifact_count,
            csv_line_counts=csv_line_counts,
            suspicious_csvs=suspicious_csvs,
        )

    def snapshot(self, include_files: bool = False) -> dict:
        data = {
            "root": self.root,
            "file_count": self.file_count,
            "total_bytes": self.total_bytes,
            "digest": self.digest,
            "missing_required": list(self.missing_required),
            "runtime_artifact_count": self.runtime_artifact_count,
            "suspicious_csvs": list(self.suspicious_csvs),
            "csv_line_counts": dict(self.csv_line_counts),
        }
        if include_files:
            data["files"] = list(self.files)
        return data
