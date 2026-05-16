#!/usr/bin/env python3
"""Static Stage 52 probe for activation-readiness policy CLI wiring."""
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

    readiness = read("crates/reta_architecture/src/table_view_activation_readiness.rs")
    runtime_switch = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    binary = read("src/bin/reta_arch_activation_readiness.rs")
    ffi = read("src/ffi.rs")
    shadow = read("src/reta_arch_shadow.rs")

    checks = {
        "unused_shadow_report_import_removed": "ShadowTableViewOutputReport" not in readiness,
        "policy_strict_declared": "pub fn strict() -> Self" in readiness,
        "policy_diagnostic_declared": "pub fn diagnostic() -> Self" in readiness,
        "policy_from_cli_declared": "pub fn from_cli_args(args: &[String], base: &Self)" in readiness,
        "policy_required_guard_names_declared": "pub fn required_guard_names(&self)" in readiness,
        "policy_tests_present": "readiness_policy_cli_can_switch_to_diagnostic_mode" in readiness
        and "readiness_policy_cli_can_require_recovery" in readiness,
        "runtime_switch_strips_diagnostic_flag": "--activation-readiness-diagnostic" in runtime_switch,
        "runtime_switch_strips_preview_flag": "--activation-readiness-preview=" in runtime_switch,
        "runtime_switch_test_present": "readiness_policy_flags_are_stripped_before_legacy_execution" in runtime_switch,
        "runtime_switch_gate_present": "table_view_activation_readiness.policy_from_cli" in runtime_switch,
        "migration_gate_present": "table_view_activation_readiness.policy_from_cli" in migration,
        "inspect_binary_uses_cli_policy": "TableViewActivationReadinessPolicy::from_cli_args" in binary
        and '"policy_from_cli"' in binary,
        "ffi_uses_cli_policy": "TableViewActivationReadinessPolicy::from_cli_args" in ffi
        and '"policy_from_cli"' in ffi,
        "shadow_bridge_uses_cli_policy": "TableViewActivationReadinessPolicy::from_cli_args" in shadow
        and "&readiness_policy" in shadow,
    }
    failed = [name for name, ok in checks.items() if not ok]
    out = {
        "stage": 52,
        "status": "ok" if not failed else "failed",
        "checks": checks,
        "failed": failed,
        "fixed_warning": "removed unused ShadowTableViewOutputReport import from table_view_activation_readiness.rs",
        "new_policy_flags": [
            "--activation-readiness-strict",
            "--activation-readiness-diagnostic",
            "--activation-readiness-no-selected-lines",
            "--activation-readiness-include-selected-lines",
            "--activation-readiness-require-recovery",
            "--activation-readiness-ignore-recovery",
            "--activation-readiness-require-persistence",
            "--activation-readiness-ignore-persistence",
            "--activation-readiness-preview=N",
        ],
    }
    print(json.dumps(out, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
