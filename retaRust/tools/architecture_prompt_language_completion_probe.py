#!/usr/bin/env python3
"""Stage 64 smoke probe for prompt language-completion wiring."""
from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def csv_shape(path: str) -> tuple[int, int, str, str]:
    with (ROOT / path).open("r", encoding="utf-8", newline="") as handle:
        rows = list(csv.reader(handle, delimiter=";"))
    width = max((len(row) for row in rows), default=0)
    header_493 = rows[0][493] if rows and len(rows[0]) > 493 else ""
    header_744 = rows[0][744] if rows and len(rows[0]) > 744 else ""
    return len(rows), width, header_493, header_744


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    module = read("crates/reta_architecture/src/prompt_language_completion.rs")
    completion_runtime = read("crates/reta_architecture/src/completion_runtime.rs")
    completion_nested = read("crates/reta_architecture/src/completion_nested.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    lib_rs = read("crates/reta_architecture/src/lib.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    runtime_switch = read("crates/reta_architecture/src/runtime_switch.rs")

    rows, width, header_493, header_744 = csv_shape("csv/en-religion.csv")

    checks = {
        "module_exists": (ROOT / "crates/reta_architecture/src/prompt_language_completion.rs").exists(),
        "language_parameter_completion_present": '"-language="' in module and '"--sprache="' in module,
        "language_value_completion_present": '"english"' in module and '"koreanisch"' in module,
        "prompt_sync_witness_present": "language_sync_for_cli_args" in module and "language_coverage_for_cli_args" in module,
        "continuum_smoke_present": "continuum_m_prompt_language_completion_smoke" in module,
        "completion_runtime_has_language_fields": "language_parameters" in completion_runtime and "language_values" in completion_runtime,
        "nested_completion_has_language_value_context": "LanguageValPara" in completion_nested and "runtime.language_values.clone()" in completion_nested,
        "facade_runtime_has_bundle": "prompt_language_completion: PromptLanguageCompletionBundle" in facade,
        "prompt_context_has_language_fields": "prompt_language_completion_candidate_count" in facade and "prompt_language_sync_ready" in facade,
        "lib_exports_prompt_language_completion": "pub mod prompt_language_completion" in lib_rs and "PromptLanguageCompletionReport" in lib_rs,
        "ffi_export_present": "reta_architecture_prompt_language_completion_json" in ffi,
        "inspect_binary_registered": "rreta_arch_prompt_language_completion" in cargo,
        "runtime_switch_has_prompt_gates": "prompt_language_completion.language_value_candidates" in runtime_switch,
        "migration_step_present": "step-prompt-language-completion" in migration,
        "synced_en_religion_has_745_columns": rows == 1025 and width == 745,
        "synced_en_religion_744_header": "Neues M" in header_744,
        "legacy_493_header_still_present": "M Kontinuum" in header_493,
    }
    report = {
        "stage": 64,
        "status": "ok" if all(checks.values()) else "blocked",
        "checks": checks,
        "en_religion": {
            "rows": rows,
            "max_columns": width,
            "header_493": header_493[:80],
            "header_744": header_744[:80],
        },
        "prompt_examples": {
            "parameter": "reta -la -> -language=",
            "value": "reta -language=e -> english/en/englisch",
            "continuum": "reta -language=english -spalten --kontinuum=m -> synced 493+744 language witness",
        },
    }
    print(json.dumps(report, indent=2 if args.pretty else None, ensure_ascii=False))
    return 0 if report["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
