#!/usr/bin/env python3
"""Static smoke probe for Stage 65 prompt-language guard wiring."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

CHECKS = {
    "module_exists": "crates/reta_architecture/src/prompt_language_guard.rs",
    "inspect_binary_exists": "src/bin/reta_arch_prompt_language_guard.rs",
}

TEXT_CHECKS = {
    "lib_pub_mod": ("crates/reta_architecture/src/lib.rs", "pub mod prompt_language_guard;"),
    "lib_reexports_policy": ("crates/reta_architecture/src/lib.rs", "PromptLanguageGuardPolicy"),
    "facade_runtime_field": ("crates/reta_architecture/src/facade.rs", "pub prompt_language_guard: PromptLanguageGuardBundle"),
    "facade_prompt_context_ready": ("crates/reta_architecture/src/facade.rs", "prompt_language_guard_ready"),
    "runtime_switch_gate": ("crates/reta_architecture/src/runtime_switch.rs", "prompt_language_guard.direct_744_prompt_guard"),
    "runtime_switch_strips_flag": ("crates/reta_architecture/src/runtime_switch.rs", "--prompt-language-guard-strict"),
    "migration_step": ("crates/reta_architecture/src/migration_control.rs", "step-prompt-language-guard"),
    "ffi_export": ("src/ffi.rs", "reta_architecture_prompt_language_guard_json"),
    "cargo_bin": ("Cargo.toml", "rreta_arch_prompt_language_guard"),
    "module_policy_from_cli": ("crates/reta_architecture/src/prompt_language_guard.rs", "PromptLanguageGuardPolicy::from_cli_args"),
    "module_uses_prompt_execution": ("crates/reta_architecture/src/prompt_language_guard.rs", "plan_prompt_execution"),
    "module_smoke": ("crates/reta_architecture/src/prompt_language_guard.rs", "continuum_m_prompt_language_guard_smoke"),
}


def status_for() -> dict:
    checks = {}
    for name, rel in CHECKS.items():
        checks[name] = (ROOT / rel).exists()
    for name, (rel, needle) in TEXT_CHECKS.items():
        path = ROOT / rel
        checks[name] = path.exists() and needle in path.read_text(encoding="utf-8")
    failed = [name for name, ok in checks.items() if not ok]
    module = (ROOT / "crates/reta_architecture/src/prompt_language_guard.rs").read_text(encoding="utf-8")
    checks["guard_blocks_non_reta_test"] = "prompt_language_guard_blocks_non_reta_prompt_by_default" in module
    checks["guard_accepts_744_test"] = "prompt_language_guard_accepts_synced_continuum_m" in module
    failed = [name for name, ok in checks.items() if not ok]
    return {
        "stage": 65,
        "status": "ok" if not failed else "failed",
        "failed_checks": failed,
        "checks": checks,
        "prompt_guard_morphisms": [
            "prompt_language_guard.prompt_to_reta_argv",
            "prompt_language_guard.language_completion_ready",
            "prompt_language_guard.language_coverage_ready",
            "prompt_language_guard.language_sync_ready",
            "prompt_language_guard.direct_744_prompt_guard",
        ],
        "universal_property": "prompt_language_guard_must_commute_with_table_view_language_cover_before_prompt_activation",
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()
    result = status_for()
    print(json.dumps(result, indent=2 if args.pretty else None, sort_keys=True))
    raise SystemExit(0 if result["status"] == "ok" else 1)


if __name__ == "__main__":
    main()
