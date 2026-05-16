#!/usr/bin/env python3
"""Probe the p12345 prompt regression.

`p12345` is Python-like short syntax for the math command `mulpri 12345`.
It must produce immediate math output even though 12345 is above the current
reta table row universe.  The table-row parser is intentionally capped; the
math prompt parser must not be capped by that table limit.
"""
from __future__ import annotations

import argparse
import json
import shutil
import subprocess
from pathlib import Path


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def contains(text: str, needle: str) -> bool:
    return needle in text


def run_optional_cargo_test(root: Path, enabled: bool) -> dict:
    if not enabled:
        return {"ran": False, "reason": "not requested"}
    cargo = shutil.which("cargo")
    if not cargo:
        return {"ran": False, "reason": "cargo not found"}
    cmd = [cargo, "test", "-p", "reta", "--lib", "p_prefixed_large_number", "--", "--nocapture"]
    proc = subprocess.run(cmd, cwd=root, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return {
        "ran": True,
        "command": cmd,
        "returncode": proc.returncode,
        "stdout_tail": proc.stdout[-4000:],
        "stderr_tail": proc.stderr[-4000:],
        "passed": proc.returncode == 0,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pretty", action="store_true")
    parser.add_argument("--run-cargo", action="store_true", help="also run the focused cargo unit test when cargo is available")
    args = parser.parse_args()

    root = repo_root()
    python_like_path = root / "src" / "prompt" / "python_like.rs"
    commands_path = root / "src" / "prompt" / "commands.rs"
    diagnostics_path = root / "tools" / "run_architecture_diagnostics.py"
    regression_runner_path = root / "tools" / "run_prompt_regression_tests.py"

    python_like = python_like_path.read_text(encoding="utf-8")
    commands = commands_path.read_text(encoding="utf-8")
    diagnostics = diagnostics_path.read_text(encoding="utf-8") if diagnostics_path.exists() else ""
    regression_runner = regression_runner_path.read_text(encoding="utf-8") if regression_runner_path.exists() else ""

    checks = {
        "has_unbounded_direct_math_parser": contains(commands, "parse_unbounded_direct_math_numbers"),
        "has_direct_math_range_cap": contains(commands, "MAX_DIRECT_MATH_RANGE_ITEMS"),
        "table_parser_empty_result_falls_back_to_direct_math": contains(commands, "if !numbers.is_empty()")
        and contains(commands, "parse_unbounded_direct_math_numbers(token)"),
        "has_p12345_python_like_test": contains(python_like, "p_prefixed_large_number_stays_math_only_before_direct_output"),
        "has_p12345_command_test": contains(commands, "p_prefixed_large_number_outputs_math_instead_of_no_output_message"),
        "has_mulpri_large_number_test": contains(commands, "mulpri_large_number_outputs_math_above_table_limit"),
        "has_large_integer_list_test": contains(commands, "direct_math_integer_list_above_table_limit_outputs_each_number"),
        "diagnostics_contains_prompt_p12345": contains(diagnostics, "prompt_p12345"),
        "regression_runner_contains_p12345": contains(regression_runner, "p12345"),
        "no_expected_no_output_message_for_p12345": not contains(commands, "p12345")
        or contains(commands, "!output.text.contains(\"nichts auszugeben\")"),
    }

    cargo_result = run_optional_cargo_test(root, args.run_cargo)
    if cargo_result.get("ran"):
        checks["optional_cargo_test_passed"] = bool(cargo_result.get("passed"))

    status = "ok" if all(checks.values()) else "failed"
    report = {
        "status": status,
        "probe": "architecture_prompt_p12345_probe",
        "regression": "p12345 must produce direct mulpri math output above the table row limit",
        "previous_bad_message": "Dies ('mulpri 12345 multis prim primfaktorenvergleich') ist tatsächlich ein Befehl (oder es sind mehrere), aber es gibt nichts auszugeben.",
        "expected_prompt_semantics": {
            "input": "p12345",
            "short_expansion": ["mulpri", "12345"],
            "macro_expansion": ["mulpri", "12345", "multis", "prim", "primfaktorenvergleich"],
            "output_kind": "PromptCommand::Immediate",
            "must_contain": ["12345:"],
            "must_not_contain": ["nichts auszugeben", "--absicht", "--thomas"],
        },
        "checks": checks,
        "cargo": cargo_result,
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
