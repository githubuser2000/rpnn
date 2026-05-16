#!/usr/bin/env python3
"""Run useful architecture/activation/prompt diagnostics and collect outputs.

The script is designed for the current migration workflow. It runs the new
rreta_arch_* and rretaprompt_arch_* executables on a small set of stable cases
and stores their stdout/stderr/metadata in a single diagnostics directory.

Typical use from the repo root:

    python3 tools/run_architecture_diagnostics.py --build-selected --pretty

Outputs are written to target/reta_arch_diagnostics/<timestamp>/ by default.
"""
from __future__ import annotations

import argparse
import datetime as _dt
import json
import os
import shlex
import subprocess
import sys
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Sequence, Tuple

try:
    from architecture_binary_catalog import CATALOG, BinaryInfo
except Exception:  # pragma: no cover - when executed from unusual cwd
    sys.path.insert(0, str(Path(__file__).resolve().parent))
    from architecture_binary_catalog import CATALOG, BinaryInfo  # type: ignore


@dataclass(frozen=True)
class Case:
    name: str
    description: str
    reta_args: List[str]
    prompt_args: List[str]
    wants_legacy: bool = True


@dataclass
class RunResult:
    label: str
    command: List[str]
    cwd: str
    returncode: int
    stdout_file: str
    stderr_file: str
    meta_file: str
    elapsed_ms: int
    skipped: bool = False
    reason: str = ""


CASES: Dict[str, Case] = {
    "continuum_m": Case(
        name="continuum_m",
        description="Core 493/744 continuum regression: --kontinuum=m should select 493 and 744.",
        reta_args=["-zeilen", "--vorhervonausschnitt=1-1", "-spalten", "--kontinuum=m", "--breite=0"],
        prompt_args=["reta", "-spalten", "--kontinuum=m"],
    ),
    "continuum_m_en": Case(
        name="continuum_m_en",
        description="Language-aware 493/744 regression with English language selection.",
        reta_args=["-language=english", "-zeilen", "--vorhervonausschnitt=1-1", "-spalten", "--kontinuum=m", "--breite=0"],
        prompt_args=["reta", "-language=english", "-spalten", "--kontinuum=m"],
    ),
    "ordered_744_493": Case(
        name="ordered_744_493",
        description="Explicit column order should preserve 744 before 493 when requested.",
        reta_args=[
            "-zeilen", "--vorhervonausschnitt=1-1",
            "-spalten", "--kontinuum=m",
            "-ausgabe", "--spaltenreihenfolgeundnurdiese=744,493",
            "--breite=0",
        ],
        prompt_args=["reta", "-spalten", "--kontinuum=m", "-ausgabe", "--spaltenreihenfolgeundnurdiese=744,493"],
    ),
    "row_order": Case(
        name="row_order",
        description="Explicit row order should preserve 3,1,2 after the header row.",
        reta_args=["-zeilen", "--vorhervonausschnitt=3,1-2", "-spalten", "--religion=493", "--breite=0"],
        prompt_args=["reta", "-zeilen", "--vorhervonausschnitt=3,1-2", "-spalten", "--religion=493"],
    ),
    "prompt_p1234": Case(
        name="prompt_p1234",
        description="Prompt short form p1234 must stay math-only (mulpri 1234), not synthesize default absicht/thomas table argv.",
        reta_args=[],
        prompt_args=["p1234"],
        wants_legacy=False,
    ),
    "prompt_p12345": Case(
        name="prompt_p12345",
        description="Prompt short form p12345 must produce direct math output above the table row limit, not the known-command/no-output fallback.",
        reta_args=[],
        prompt_args=["p12345"],
        wants_legacy=False,
    ),
}

# Binaries that do not need legacy output but produce directly useful JSON.
NO_LEGACY_BINS: List[Tuple[str, List[str]]] = [
    ("rreta_arch_inspect", ["--reta-arch=dry-run"]),
    ("rreta_arch_materialize", []),
    ("rreta_arch_view", []),
    ("rreta_arch_view_output", []),
    ("rreta_arch_column_order", []),
    ("rreta_arch_row_order", []),
    ("rreta_arch_output_flags", []),
    ("rreta_arch_numbering", []),
    ("rreta_arch_layout", []),
    ("rreta_arch_virtual_columns", []),
    ("rreta_arch_virtual_parity", []),
    ("rreta_arch_language_parity", []),
    ("rreta_arch_language_coverage", []),
    ("rreta_arch_language_sync", []),
]

# Binaries that need legacy lines via --legacy-lines-file.
LEGACY_BINS: List[Tuple[str, List[str]]] = [
    ("rreta_arch_view_output_parity", []),
    ("rreta_arch_view_output_shadow", ["--reta-arch=commit"]),
    ("rreta_arch_commit_audit", ["--reta-arch=commit"]),
    ("rreta_arch_activation_transaction", ["--reta-arch=commit"]),
    ("rreta_arch_activation_journal", ["--reta-arch=commit"]),
    ("rreta_arch_activation_replay", ["--reta-arch=commit"]),
    ("rreta_arch_activation_ledger", ["--reta-arch=commit"]),
    ("rreta_arch_activation_store", ["--reta-arch=commit"]),
    ("rreta_arch_activation_persistence", ["--reta-arch=commit"]),
    ("rreta_arch_activation_readiness", ["--reta-arch=commit"]),
    ("rreta_arch_activation_promotion", ["--reta-arch=commit"]),
]

STYLE_BINS: List[Tuple[str, List[str]]] = [
    ("rreta_arch_html_output", ["-ausgabe", "--art=html", "--htmlclasses"]),
    ("rreta_arch_row_styles", ["-ausgabe", "--art=html", "--rowcolors"]),
    ("rreta_arch_cell_styles", ["-ausgabe", "--art=html", "--cellstyles"]),
    ("rreta_arch_style_composition", ["-ausgabe", "--art=html", "--htmlclasses", "--cellstyles"]),
    ("rreta_arch_style_parity", ["-ausgabe", "--art=html", "--htmlclasses", "--cellstyles", "--rowcolors"]),
    ("rreta_arch_shell_styles", ["-ausgabe", "--shellcolors"]),
]

PROMPT_BINS: List[Tuple[str, List[str], bool]] = [
    ("rreta_arch_prompt_language_completion", [], False),
    ("rreta_arch_prompt_language_guard", [], False),
    ("rreta_arch_prompt_language_commit", ["--reta-arch=commit"], False),
    ("rreta_arch_prompt_activation_readiness", ["--reta-arch=commit"], False),
    ("rretaprompt_arch_inspect", ["--reta-arch=commit"], True),
]

PYTHON_PROBES: List[str] = [
    "architecture_religion_csv_update_probe.py",
    "architecture_csv_catalog_probe.py",
    "architecture_language_sync_probe.py",
    "architecture_language_coverage_probe.py",
    "architecture_language_cli_probe.py",
    "architecture_prompt_language_completion_probe.py",
    "architecture_prompt_language_guard_probe.py",
    "architecture_prompt_language_commit_guard_probe.py",
    "architecture_prompt_activation_readiness_probe.py",
    "architecture_prompt_p1234_probe.py",
    "architecture_prompt_p12345_probe.py",
    "run_prompt_regression_tests.py",
    "architecture_table_view_output_parity_probe.py",
    "architecture_table_view_output_commit_probe.py",
    "architecture_activation_promotion_probe.py",
    "architecture_migration_step_arity_probe.py",
    "architecture_module_coverage.py",
    "architecture_semantic_surface_audit.py",
]


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def cargo_bin_package(name: str) -> str:
    for item in CATALOG:
        if item.name == name:
            return item.package
    if name == "rretaprompt_arch_inspect":
        return "retaprompt_frontends"
    return "reta"


def target_binary_path(root: Path, name: str, profile: str) -> Path:
    exe = name + (".exe" if os.name == "nt" else "")
    sub = "release" if profile == "release" else "debug"
    return root / "target" / sub / exe


def run_command(
    label: str,
    command: Sequence[str],
    cwd: Path,
    out_dir: Path,
    timeout: int,
    env: Optional[Dict[str, str]] = None,
) -> RunResult:
    safe_label = label.replace("/", "__").replace(" ", "_")
    stdout_file = out_dir / f"{safe_label}.stdout.txt"
    stderr_file = out_dir / f"{safe_label}.stderr.txt"
    meta_file = out_dir / f"{safe_label}.meta.json"
    started = _dt.datetime.now(_dt.timezone.utc)
    started_perf = _dt.datetime.now()
    try:
        proc = subprocess.run(
            list(command),
            cwd=str(cwd),
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
            env=env,
        )
        returncode = proc.returncode
        stdout = proc.stdout
        stderr = proc.stderr
    except FileNotFoundError as exc:
        returncode = 127
        stdout = ""
        stderr = f"COMMAND NOT FOUND: {exc.filename}\nFull command: {shlex.join(command)}\n"
    except subprocess.TimeoutExpired as exc:
        returncode = 124
        stdout = exc.stdout if isinstance(exc.stdout, str) else ""
        stderr = (exc.stderr if isinstance(exc.stderr, str) else "") + f"\nTIMEOUT after {timeout}s\n"
    elapsed = int((_dt.datetime.now() - started_perf).total_seconds() * 1000)
    stdout_file.write_text(stdout, encoding="utf-8", errors="replace")
    stderr_file.write_text(stderr, encoding="utf-8", errors="replace")
    meta = {
        "label": label,
        "command": list(command),
        "cwd": str(cwd),
        "returncode": returncode,
        "started_utc": started.isoformat(),
        "elapsed_ms": elapsed,
        "stdout_file": str(stdout_file.relative_to(out_dir.parent)),
        "stderr_file": str(stderr_file.relative_to(out_dir.parent)),
    }
    meta_file.write_text(json.dumps(meta, ensure_ascii=False, indent=2), encoding="utf-8")
    return RunResult(label, list(command), str(cwd), returncode, str(stdout_file), str(stderr_file), str(meta_file), elapsed)


def build_command_for_bin(root: Path, name: str, profile: str, args: List[str], prefer_target: bool, use_cargo_run: bool) -> List[str]:
    bin_path = target_binary_path(root, name, profile)
    if prefer_target and bin_path.exists() and not use_cargo_run:
        return [str(bin_path), *args]
    package = cargo_bin_package(name)
    cmd = ["cargo", "run", "--quiet"]
    if profile == "release":
        cmd.append("--release")
    if package != "reta":
        cmd.extend(["-p", package])
    cmd.extend(["--bin", name, "--", *args])
    return cmd


def build_selected(root: Path, names: Iterable[str], profile: str, out_dir: Path, timeout: int) -> List[RunResult]:
    results: List[RunResult] = []
    for name in sorted(set(names)):
        package = cargo_bin_package(name)
        cmd = ["cargo", "build"]
        if profile == "release":
            cmd.append("--release")
        if package != "reta":
            cmd.extend(["-p", package])
        cmd.extend(["--bin", name])
        results.append(run_command(f"build__{name}", cmd, root, out_dir, timeout))
    return results


def ensure_json_or_text_summary(path: Path) -> Dict[str, object]:
    text = path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""
    stripped = text.strip()
    if not stripped:
        return {"kind": "empty", "bytes": 0}
    try:
        data = json.loads(stripped)
    except Exception:
        return {"kind": "text", "bytes": len(text), "first_lines": stripped.splitlines()[:5]}
    summary: Dict[str, object] = {"kind": "json"}
    if isinstance(data, dict):
        summary["keys"] = sorted(data.keys())[:40]
        for key in [
            "status", "ready", "safe", "raw_equal", "semantic_equal", "language_ready",
            "language_coverage_ready", "language_sync_ready", "promotion_ready",
            "ready_for_visible_activation", "should_replace_visible_output",
        ]:
            if key in data:
                summary[key] = data[key]
    elif isinstance(data, list):
        summary["list_len"] = len(data)
    return summary


def write_legacy_lines(root: Path, case: Case, case_dir: Path, profile: str, timeout: int, prefer_target: bool, use_cargo_run: bool) -> Tuple[Path, RunResult]:
    cmd = build_command_for_bin(root, "rreta", profile, case.reta_args, prefer_target, use_cargo_run)
    result = run_command(f"legacy__{case.name}", cmd, root, case_dir, timeout)
    legacy_file = case_dir / "legacy_lines.txt"
    stdout_path = Path(result.stdout_file)
    legacy_file.write_text(stdout_path.read_text(encoding="utf-8", errors="replace"), encoding="utf-8")
    return legacy_file, result


def run_case(root: Path, case: Case, base_out: Path, args: argparse.Namespace) -> List[RunResult]:
    case_dir = base_out / "cases" / case.name
    case_dir.mkdir(parents=True, exist_ok=True)
    (case_dir / "case.json").write_text(json.dumps(asdict(case), ensure_ascii=False, indent=2), encoding="utf-8")
    results: List[RunResult] = []
    legacy_file: Optional[Path] = None
    if case.wants_legacy:
        legacy_file, legacy_result = write_legacy_lines(root, case, case_dir, args.profile, args.timeout, not args.force_cargo_run, args.force_cargo_run)
        results.append(legacy_result)

    bins: List[Tuple[str, List[str]]] = []
    if case.reta_args:
        if args.include_table:
            bins.extend(NO_LEGACY_BINS)
        if args.include_style:
            bins.extend(STYLE_BINS)
    for name, prefix in bins:
        cmd_args = prefix + case.reta_args
        cmd = build_command_for_bin(root, name, args.profile, cmd_args, not args.force_cargo_run, args.force_cargo_run)
        results.append(run_command(f"{case.name}__{name}", cmd, root, case_dir, args.timeout))

    if args.include_activation and legacy_file is not None:
        store_path = case_dir / "activation-store.txt"
        for name, prefix in LEGACY_BINS:
            extra = []
            if name == "rreta_arch_activation_file":
                extra = ["--activation-store-file", str(store_path)]
            cmd_args = ["--legacy-lines-file", str(legacy_file)] + extra + prefix + case.reta_args
            cmd = build_command_for_bin(root, name, args.profile, cmd_args, not args.force_cargo_run, args.force_cargo_run)
            results.append(run_command(f"{case.name}__{name}", cmd, root, case_dir, args.timeout))

    if args.include_prompt:
        prompt_dir = case_dir / "prompt"
        prompt_dir.mkdir(exist_ok=True)
        for name, prefix, prompt_frontend in PROMPT_BINS:
            cmd_args = prefix + case.prompt_args
            cmd = build_command_for_bin(root, name, args.profile, cmd_args, not args.force_cargo_run, args.force_cargo_run)
            results.append(run_command(f"{case.name}__{name}", cmd, root, prompt_dir, args.timeout))

    return results


def run_python_probes(root: Path, out_dir: Path, args: argparse.Namespace) -> List[RunResult]:
    results: List[RunResult] = []
    probes_dir = out_dir / "python_probes"
    probes_dir.mkdir(parents=True, exist_ok=True)
    for probe in PYTHON_PROBES:
        path = root / "tools" / probe
        if not path.exists():
            meta_file = probes_dir / f"{probe}.missing.json"
            meta_file.write_text(json.dumps({"probe": probe, "missing": True}, indent=2), encoding="utf-8")
            continue
        cmd = [sys.executable, str(path), "--pretty"]
        results.append(run_command(f"probe__{probe}", cmd, root, probes_dir, args.timeout))
    return results


def selected_bin_names(include_table: bool, include_style: bool, include_activation: bool, include_prompt: bool) -> List[str]:
    names = ["rreta"]
    if include_table:
        names.extend(name for name, _ in NO_LEGACY_BINS)
    if include_style:
        names.extend(name for name, _ in STYLE_BINS)
    if include_activation:
        names.extend(name for name, _ in LEGACY_BINS)
    if include_prompt:
        names.extend(name for name, _, _ in PROMPT_BINS)
    return sorted(set(names))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument("--out-dir", default="", help="Output directory. Default: target/reta_arch_diagnostics/<timestamp>")
    parser.add_argument("--profile", choices=["debug", "release"], default="debug")
    parser.add_argument("--timeout", type=int, default=120)
    parser.add_argument("--build-selected", action="store_true", help="Build selected binaries before running diagnostics")
    parser.add_argument("--force-cargo-run", action="store_true", help="Always use cargo run instead of target/debug binaries")
    parser.add_argument("--case", action="append", choices=sorted(CASES), help="Run only selected case(s)")
    parser.add_argument("--no-table", dest="include_table", action="store_false", default=True)
    parser.add_argument("--no-style", dest="include_style", action="store_false", default=True)
    parser.add_argument("--no-activation", dest="include_activation", action="store_false", default=True)
    parser.add_argument("--no-prompt", dest="include_prompt", action="store_false", default=True)
    parser.add_argument("--python-probes", action="store_true", help="Also run Python architecture probe scripts")
    parser.add_argument("--pretty", action="store_true", help="Print a compact human-readable summary")
    args = parser.parse_args()

    root = repo_root()
    timestamp = _dt.datetime.now().strftime("%Y%m%d_%H%M%S")
    out_dir = Path(args.out_dir) if args.out_dir else root / "target" / "reta_arch_diagnostics" / timestamp
    out_dir.mkdir(parents=True, exist_ok=True)

    catalog_json = out_dir / "binary_catalog.json"
    catalog_json.write_text(json.dumps([asdict(item) for item in CATALOG], ensure_ascii=False, indent=2), encoding="utf-8")

    results: List[RunResult] = []
    names = selected_bin_names(args.include_table, args.include_style, args.include_activation, args.include_prompt)
    if args.build_selected:
        build_dir = out_dir / "build"
        build_dir.mkdir(parents=True, exist_ok=True)
        results.extend(build_selected(root, names, args.profile, build_dir, args.timeout * 3))

    case_names = args.case or ["continuum_m", "continuum_m_en", "ordered_744_493", "row_order", "prompt_p1234", "prompt_p12345"]
    for case_name in case_names:
        results.extend(run_case(root, CASES[case_name], out_dir, args))

    if args.python_probes:
        results.extend(run_python_probes(root, out_dir, args))

    failures = [result for result in results if result.returncode != 0]
    summary = {
        "generated_at": _dt.datetime.now(_dt.timezone.utc).isoformat(),
        "repo_root": str(root),
        "out_dir": str(out_dir),
        "profile": args.profile,
        "case_names": case_names,
        "selected_bins": names,
        "result_count": len(results),
        "failure_count": len(failures),
        "failures": [asdict(result) for result in failures],
        "results": [asdict(result) for result in results],
    }

    # Add small JSON/text summaries for fast triage.
    triage = []
    for result in results:
        triage.append({
            "label": result.label,
            "returncode": result.returncode,
            "stdout_summary": ensure_json_or_text_summary(Path(result.stdout_file)),
            "stderr_summary": ensure_json_or_text_summary(Path(result.stderr_file)),
        })
    summary["triage"] = triage

    summary_path = out_dir / "diagnostics_summary.json"
    summary_path.write_text(json.dumps(summary, ensure_ascii=False, indent=2), encoding="utf-8")

    if args.pretty:
        print(f"diagnostics_dir={out_dir}")
        print(f"results={len(results)} failures={len(failures)}")
        for result in failures[:20]:
            print(f"FAIL {result.label}: returncode={result.returncode} stderr={result.stderr_file}")
        if len(failures) > 20:
            print(f"... {len(failures) - 20} more failures")
        print(f"summary={summary_path}")
    else:
        print(summary_path)
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
