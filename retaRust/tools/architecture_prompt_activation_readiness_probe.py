#!/usr/bin/env python3
"""Static probe for Stage 67 prompt activation readiness integration."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    module = read("crates/reta_architecture/src/prompt_activation_readiness.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    runtime = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    binary = read("src/bin/reta_arch_prompt_activation_readiness.rs")
    inspect = read("crates/retaprompt_frontends/src/bin/retaprompt_arch_inspect.rs")
    prompt_app = read("src/prompt/app.rs")

    checks = {
        "module_declares_policy": "pub struct PromptActivationReadinessPolicy" in module,
        "module_declares_report": "pub struct PromptActivationReadinessReport" in module,
        "module_folds_reports": "prompt_activation_readiness_from_reports" in module,
        "module_checks_commit": "prompt_shadow_commit_uses_shadow_plan" in module,
        "module_checks_language_guard": "prompt_language_guard_ready" in module and "prompt_language_guard_has_no_failed_guards" in module,
        "module_has_smoke_test": "continuum_m_prompt_activation_readiness_smoke" in module,
        "lib_exports_module": "pub mod prompt_activation_readiness" in lib,
        "lib_reexports_symbols": "PromptActivationReadinessPolicy" in lib and "prompt_activation_readiness_from_reports" in lib,
        "facade_runtime_field": "pub prompt_activation_readiness: PromptActivationReadinessBundle" in facade,
        "facade_snapshot_fields": "rust_prompt_activation_readiness_morphism_count" in facade,
        "prompt_context_fields": "prompt_activation_readiness_ready" in facade,
        "runtime_strips_readiness_flags": "--prompt-activation-readiness-diagnostic" in runtime and "--prompt-readiness-preview=" in runtime,
        "runtime_adapter_gate": "morphism.starts_with(\"prompt_activation_readiness\")" in runtime,
        "migration_step_present": "step-prompt-activation-readiness" in migration,
        "ffi_export_present": "reta_architecture_prompt_activation_readiness_json" in ffi,
        "binary_registered": "rreta_arch_prompt_activation_readiness" in cargo,
        "binary_present": "PromptActivationReadinessInspect" in binary and "prompt_activation_readiness_from_reports" in binary,
        "retaprompt_inspect_reports_readiness": "prompt_activation_readiness" in inspect and "PromptActivationReadinessPolicy::from_cli_args" in inspect,
        "prompt_app_logs_readiness": "readiness_ready" in prompt_app and "prompt_activation_readiness_from_reports" in prompt_app,
    }
    result = {
        "stage": 67,
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "failed": [name for name, ok in checks.items() if not ok],
    }
    print(json.dumps(result, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if result["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
