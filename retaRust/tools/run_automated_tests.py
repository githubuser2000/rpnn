#!/usr/bin/env python3
"""Run the automated regression suite for the Rust Reta architecture port.

This script is meant for local Termux/Linux CI.  It intentionally combines
three layers:

1. Static/Python probes that do not need Cargo.
2. Focused Cargo tests for the critical Rust modules.
3. Binary smoke tests for the inspect/diagnostic executables.

The output is a JSON report under target/reta_arch_tests/ and a concise summary
on stdout.  This directory is ignored by git.
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path
from typing import Iterable, List, Sequence


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def now_stamp() -> str:
    return time.strftime("%Y%m%d-%H%M%S")


def run_command(label: str, command: Sequence[str], cwd: Path, *, timeout: int | None = None) -> dict:
    started = time.monotonic()
    try:
        proc = subprocess.run(
            list(command),
            cwd=cwd,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
        )
        returncode = proc.returncode
        stdout = proc.stdout
        stderr = proc.stderr
        error = None
    except subprocess.TimeoutExpired as exc:
        returncode = 124
        stdout = exc.stdout if isinstance(exc.stdout, str) else ""
        stderr = exc.stderr if isinstance(exc.stderr, str) else ""
        error = f"timeout after {timeout}s"
    except FileNotFoundError as exc:
        returncode = 127
        stdout = ""
        stderr = str(exc)
        error = "command not found"
    elapsed_ms = int((time.monotonic() - started) * 1000)
    return {
        "label": label,
        "command": list(command),
        "returncode": returncode,
        "elapsed_ms": elapsed_ms,
        "passed": returncode == 0,
        "error": error,
        "stdout_tail": stdout[-8000:],
        "stderr_tail": stderr[-8000:],
    }


def run_probe(root: Path, script: str, extra: Iterable[str] = ()) -> dict:
    path = root / "tools" / script
    if not path.exists():
        return {
            "label": script,
            "command": [sys.executable, str(path)],
            "returncode": 127,
            "passed": False,
            "error": "probe script missing",
            "stdout_tail": "",
            "stderr_tail": "",
        }
    return run_command(script, [sys.executable, str(path), "--pretty", *extra], root)


def cargo_args_with_tool_feature(args: Sequence[str]) -> list[str]:
    normalized = list(args)
    if "--bin" not in normalized or "--features" in normalized:
        return normalized
    # Root-package diagnostic bins are feature-gated so normal shared-library
    # builds do not accidentally produce heavy Rust executables.  Bins from
    # other packages already carry their own package selector and do not need it.
    if "-p" in normalized or "--package" in normalized:
        return normalized
    insert_at = 1 if normalized else 0
    return normalized[:insert_at] + ["--features", "rust-tool-bins"] + normalized[insert_at:]


def cargo_command(root: Path, args: Sequence[str], *, timeout: int | None = None) -> dict:
    normalized_args = cargo_args_with_tool_feature(args)
    cargo = shutil.which("cargo")
    if not cargo:
        return {
            "label": "cargo " + " ".join(normalized_args),
            "command": ["cargo", *normalized_args],
            "returncode": 0,
            "passed": True,
            "skipped": True,
            "reason": "cargo not found",
            "stdout_tail": "",
            "stderr_tail": "",
            "elapsed_ms": 0,
        }
    return run_command("cargo " + " ".join(normalized_args), [cargo, *normalized_args], root, timeout=timeout)


def binary_path(root: Path, name: str) -> Path:
    suffix = ".exe" if os.name == "nt" else ""
    return root / "target" / "debug" / f"{name}{suffix}"


def run_binary_smoke(root: Path, name: str, args: Sequence[str], expected_substrings: Sequence[str]) -> dict:
    path = binary_path(root, name)
    if not path.exists():
        return {
            "label": f"binary {name}",
            "command": [str(path), *args],
            "returncode": 0,
            "passed": True,
            "skipped": True,
            "reason": f"{path} not found; build binaries first or run with --build-binaries",
            "stdout_tail": "",
            "stderr_tail": "",
            "elapsed_ms": 0,
        }
    result = run_command(f"binary {name}", [str(path), *args], root)
    text = (result.get("stdout_tail") or "") + "\n" + (result.get("stderr_tail") or "")
    missing = [needle for needle in expected_substrings if needle not in text]
    result["expected_substrings"] = list(expected_substrings)
    result["missing_substrings"] = missing
    result["passed"] = bool(result["passed"] and not missing)
    return result


PYTHON_PROBES_FULL = [
    "architecture_prompt_p1234_probe.py",
    "architecture_prompt_p12345_probe.py",
    "architecture_prompt_language_completion_probe.py",
    "architecture_prompt_language_guard_probe.py",
    "architecture_prompt_language_commit_guard_probe.py",
    "architecture_prompt_activation_readiness_probe.py",
    "architecture_language_sync_probe.py",
    "architecture_language_sync_guard_probe.py",
    "architecture_language_coverage_probe.py",
    "architecture_language_coverage_guard_probe.py",
    "architecture_language_parity_probe.py",
    "architecture_language_cli_probe.py",
    "architecture_language_fallback_probe.py",
    "architecture_religion_csv_update_probe.py",
    "architecture_csv_catalog_probe.py",
    "architecture_table_materialization_probe.py",
    "architecture_table_view_probe.py",
    "architecture_virtual_column_probe.py",
    "architecture_virtual_parity_probe.py",
    "architecture_table_view_output_parity_probe.py",
    "architecture_table_view_output_commit_probe.py",
    "architecture_commit_audit_probe.py",
    "architecture_activation_promotion_probe.py",
    "architecture_migration_step_arity_probe.py",
]

PYTHON_PROBES_QUICK = [
    "architecture_prompt_p1234_probe.py",
    "architecture_prompt_p12345_probe.py",
    "architecture_language_sync_probe.py",
    "architecture_language_coverage_probe.py",
    "architecture_prompt_language_guard_probe.py",
    "architecture_prompt_activation_readiness_probe.py",
    "architecture_migration_step_arity_probe.py",
]

CARGO_CHECKS_FULL = [
    ["check", "-p", "reta_architecture"],
    ["test", "-p", "reta_architecture"],
    ["test", "-p", "reta_architecture", "--test", "language_744_regressions"],
    ["test", "-p", "retaprompt_commands", "--test", "prompt_math_regressions"],
    ["test", "-p", "reta", "--test", "architecture_binary_smoke"],
    ["test", "-p", "reta", "--lib", "p_prefixed_large_number", "--", "--nocapture"],
    ["test", "-p", "reta", "--lib", "mulpri_large_number", "--", "--nocapture"],
    ["check", "--bin", "rreta_arch_language_sync"],
    ["check", "--bin", "rreta_arch_language_coverage"],
    ["check", "--bin", "rreta_arch_prompt_activation_readiness"],
    ["check", "--bin", "rreta_arch_prompt_language_commit"],
    ["check", "-p", "reta", "--lib"],
    ["check", "-p", "retaprompt_commands", "--lib"],
    ["check", "-p", "retaprompt_input", "--lib"],
    ["check", "-p", "retaprompt_frontends"],
]

CARGO_CHECKS_QUICK = [
    ["test", "-p", "retaprompt_commands", "--test", "prompt_math_regressions"],
    ["test", "-p", "reta_architecture", "--test", "language_744_regressions"],
    ["test", "-p", "reta", "--lib", "p_prefixed_large_number", "--", "--nocapture"],
    ["check", "--bin", "rreta_arch_prompt_activation_readiness"],
]

BINARY_SMOKES = [
    (
        "rreta_arch_language_sync",
        ["reta", "-language=english", "-zeilen", "--vorhervonausschnitt=1-1", "-spalten", "--kontinuum=m", "--breite=0"],
        ["\"pending_action_count\": 0", "\"status\": \"ready\""],
    ),
    (
        "rreta_arch_language_coverage",
        ["reta", "-language=english", "-zeilen", "--vorhervonausschnitt=1-1", "-spalten", "--kontinuum=m", "--breite=0"],
        ["\"stale_language_count\": 0", "\"languages_missing_744\": []"],
    ),
    (
        "rreta_arch_prompt_language_guard",
        ["reta", "-language=english", "-spalten", "--kontinuum=m"],
        ["\"status\": \"ready\"", "\"direct_744_available_for_prompt_language\": true"],
    ),
    (
        "rreta_arch_prompt_activation_readiness",
        ["--reta-arch=commit", "reta", "-language=english", "-spalten", "--kontinuum=m"],
        ["\"prompt_language_guard_ready\": true"],
    ),
]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pretty", action="store_true", help="pretty-print final JSON to stdout")
    parser.add_argument("--quick", action="store_true", help="run the smaller critical suite")
    parser.add_argument("--full", action="store_true", help="run the full suite; currently default unless --quick")
    parser.add_argument("--no-python-probes", action="store_true")
    parser.add_argument("--no-cargo", action="store_true")
    parser.add_argument("--binary-smoke", action="store_true", help="run target/debug binary smoke checks when binaries exist")
    parser.add_argument("--build-binaries", action="store_true", help="cargo build the selected diagnostic binaries before --binary-smoke")
    parser.add_argument("--fail-fast", action="store_true")
    parser.add_argument("--json-out", help="write report to this file instead of target/reta_arch_tests/<timestamp>/automated_tests_report.json")
    parser.add_argument("--timeout", type=int, default=None, help="optional timeout in seconds for cargo commands")
    args = parser.parse_args()

    root = repo_root()
    out_dir = root / "target" / "reta_arch_tests" / now_stamp()
    out_dir.mkdir(parents=True, exist_ok=True)
    results: list[dict] = []

    def add(result: dict) -> None:
        results.append(result)
        if args.fail_fast and not result.get("passed"):
            raise SystemExit(write_report(root, out_dir, args, results))

    probes = PYTHON_PROBES_QUICK if args.quick else PYTHON_PROBES_FULL
    if not args.no_python_probes:
        for probe in probes:
            add(run_probe(root, probe))

        add(run_probe(root, "architecture_module_coverage.py", ["--only-missing"]))
        add(run_probe(root, "architecture_semantic_surface_audit.py", ["--only-marker-or-missing"]))

    if not args.no_cargo:
        cargo_checks = CARGO_CHECKS_QUICK if args.quick else CARGO_CHECKS_FULL
        if args.build_binaries:
            for name, _, _ in BINARY_SMOKES:
                add(cargo_command(root, ["build", "--bin", name], timeout=args.timeout))
        for cargo_args in cargo_checks:
            add(cargo_command(root, cargo_args, timeout=args.timeout))

    if args.binary_smoke:
        for name, bin_args, expected in BINARY_SMOKES:
            add(run_binary_smoke(root, name, bin_args, expected))

    return write_report(root, out_dir, args, results)


def write_report(root: Path, out_dir: Path, args: argparse.Namespace, results: list[dict]) -> int:
    failed = [result for result in results if not result.get("passed")]
    skipped = [result for result in results if result.get("skipped")]
    report = {
        "status": "ok" if not failed else "failed",
        "runner": "run_automated_tests",
        "repo_root": str(root),
        "output_dir": str(out_dir),
        "quick": bool(args.quick),
        "result_count": len(results),
        "failed_count": len(failed),
        "skipped_count": len(skipped),
        "failed_labels": [result.get("label") for result in failed],
        "results": results,
    }
    report_path = Path(args.json_out) if args.json_out else out_dir / "automated_tests_report.json"
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None))
    print(f"\nAutomated test report written to: {report_path}", file=sys.stderr)
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
