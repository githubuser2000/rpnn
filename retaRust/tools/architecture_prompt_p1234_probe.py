#!/usr/bin/env python3
"""Probe the p1234 prompt regression.

`p1234` is Python-like short syntax for the math command `mulpri 1234`.
It must not synthesize a bare-number default table command such as
`reta ... -spalten --absicht --thomas`, because `--absicht` and `--thomas`
are parameter values, not standalone `-spalten` parameters.
"""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def contains(text: str, needle: str) -> bool:
    return needle in text


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    root = repo_root()
    python_like_path = root / "src" / "prompt" / "python_like.rs"
    commands_path = root / "src" / "prompt" / "commands.rs"
    diagnostics_path = root / "tools" / "run_architecture_diagnostics.py"

    python_like = python_like_path.read_text(encoding="utf-8")
    commands = commands_path.read_text(encoding="utf-8")
    diagnostics = diagnostics_path.read_text(encoding="utf-8") if diagnostics_path.exists() else ""

    checks = {
        "has_math_only_helper": contains(python_like, "fn is_math_only_prompt_output_token"),
        "has_math_only_guard": contains(python_like, "contains_math_only_prompt_output(&normalized)"),
        "math_only_guard_returns_none": contains(python_like, "return None;"),
        "has_semantic_spalten_mapper": contains(python_like, "spalten_parameter_for_prompt_output_command"),
        "uses_menschliches_motivation": contains(python_like, "--menschliches=motivation"),
        "uses_galaxie_thomas": contains(python_like, "--galaxie=thomas"),
        "has_p1234_python_like_test": contains(python_like, "p_prefixed_number_does_not_synthesize_invalid_default_table_argv"),
        "has_p1234_command_test": contains(commands, "p_prefixed_number_is_math_output_not_invalid_reta_table"),
        "diagnostics_contains_prompt_p1234": contains(diagnostics, "prompt_p1234"),
    }

    # Ensure the previous bad assertion pattern is gone from the new bare-number test.
    bare_number_test_match = re.search(
        r"fn bare_number_still_synthesizes_default_table_argv\(\)[\s\S]*?\n    }",
        python_like,
    )
    bare_number_test = bare_number_test_match.group(0) if bare_number_test_match else ""
    checks["bare_number_test_no_longer_expects_invalid_standalone_subparams"] = (
        "!argv.contains(&\"--absicht\"" in bare_number_test
        and "!argv.contains(&\"--thomas\"" in bare_number_test
        and "--menschliches=motivation" in bare_number_test
        and "--galaxie=thomas" in bare_number_test
    )

    status = "ok" if all(checks.values()) else "failed"
    report = {
        "status": status,
        "probe": "architecture_prompt_p1234_probe",
        "regression": "p1234 must be math-only mulpri output, not a default absicht/thomas table argv",
        "previous_bad_command": "reta -zeilen --vorhervonausschnitt=1234 --oberesmaximum=1025 -spalten --absicht --thomas",
        "expected_prompt_semantics": {
            "input": "p1234",
            "short_expansion": ["mulpri", "1234"],
            "table_argv": None,
            "reason": "p is a math prompt command; default absicht/thomas table views are only for bare numeric prompts",
        },
        "valid_table_parameter_examples": [
            "--menschliches=motivation",
            "--galaxie=thomas",
        ],
        "checks": checks,
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
