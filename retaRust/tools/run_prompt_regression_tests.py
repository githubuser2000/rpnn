#!/usr/bin/env python3
"""Run the prompt regression checks that matter for the current Rust port.

This runner is intentionally small and CI-friendly.  It combines static probes
with focused cargo tests when Cargo is available.  The most important current
regressions are:

* p1234 must stay math-only, not become `reta ... -spalten --absicht --thomas`.
* p12345 must still produce math output even though 12345 is above the table
  row universe.
* bare numeric prompts may still synthesize the default table view, but with
  valid `-spalten` parameters.
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
from typing import Iterable, List


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def run_command(label: str, command: List[str], cwd: Path) -> dict:
    started = time.monotonic()
    proc = subprocess.run(command, cwd=cwd, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    elapsed_ms = int((time.monotonic() - started) * 1000)
    return {
        "label": label,
        "command": command,
        "returncode": proc.returncode,
        "elapsed_ms": elapsed_ms,
        "stdout_tail": proc.stdout[-6000:],
        "stderr_tail": proc.stderr[-6000:],
        "passed": proc.returncode == 0,
    }


def run_probe(root: Path, script: str, extra: Iterable[str] = ()) -> dict:
    path = root / "tools" / script
    if not path.exists():
        return {"label": script, "passed": False, "skipped": False, "reason": "probe script missing"}
    return run_command(script, [sys.executable, str(path), "--pretty", *extra], root)


def run_cargo_tests(root: Path, enabled: bool) -> list[dict]:
    if not enabled:
        return [{"label": "cargo", "skipped": True, "passed": True, "reason": "not requested"}]
    cargo = shutil.which("cargo")
    if not cargo:
        return [{"label": "cargo", "skipped": True, "passed": True, "reason": "cargo not found"}]

    tests = [
        [cargo, "test", "-p", "reta", "--lib", "p_prefixed_number", "--", "--nocapture"],
        [cargo, "test", "-p", "reta", "--lib", "p_prefixed_large_number", "--", "--nocapture"],
        [cargo, "test", "-p", "reta", "--lib", "mulpri_large_number", "--", "--nocapture"],
        [cargo, "test", "-p", "reta", "--lib", "direct_math_integer_list", "--", "--nocapture"],
        [cargo, "test", "-p", "reta", "--lib", "bare_number_still_synthesizes_default_table_argv", "--", "--nocapture"],
    ]
    return [run_command(" ".join(cmd[1:]), cmd, root) for cmd in tests]


def run_binary_smoke(root: Path, enabled: bool, binary: str) -> list[dict]:
    if not enabled:
        return [{"label": "binary-smoke", "skipped": True, "passed": True, "reason": "not requested"}]
    bin_path = root / "target" / "debug" / binary
    if os.name == "nt":
        bin_path = bin_path.with_suffix(".exe")
    if not bin_path.exists():
        return [{"label": "binary-smoke", "skipped": True, "passed": True, "reason": f"{bin_path} not found"}]

    runs = [
        ("p1234", [str(bin_path), "p1234"]),
        ("p12345", [str(bin_path), "p12345"]),
        ("mulpri 12345", [str(bin_path), "mulpri", "12345"]),
    ]
    results = []
    for label, cmd in runs:
        result = run_command(f"{binary} {label}", cmd, root)
        text = (result.get("stdout_tail") or "") + "\n" + (result.get("stderr_tail") or "")
        result["semantic_passed"] = "nichts auszugeben" not in text and "--absicht" not in text and "--thomas" not in text
        result["passed"] = bool(result["passed"] and result["semantic_passed"])
        results.append(result)
    return results


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pretty", action="store_true")
    parser.add_argument("--cargo", action="store_true", help="run focused cargo tests when cargo is available")
    parser.add_argument("--binary-smoke", action="store_true", help="run target/debug/rrpb smoke tests when present")
    parser.add_argument("--binary", default="rrpb", help="prompt binary name for --binary-smoke, default: rrpb")
    args = parser.parse_args()

    root = repo_root()
    results = []
    results.append(run_probe(root, "architecture_prompt_p1234_probe.py"))
    results.append(run_probe(root, "architecture_prompt_p12345_probe.py"))
    results.extend(run_cargo_tests(root, args.cargo))
    results.extend(run_binary_smoke(root, args.binary_smoke, args.binary))

    failed = [r for r in results if not r.get("passed")]
    report = {
        "status": "ok" if not failed else "failed",
        "runner": "run_prompt_regression_tests",
        "failed_count": len(failed),
        "result_count": len(results),
        "results": results,
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
