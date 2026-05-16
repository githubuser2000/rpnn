#!/usr/bin/env python3
"""Static Stage 54 probe for activation-promotion wiring."""
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

    module = read("crates/reta_architecture/src/table_view_activation_promotion.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    runtime_switch = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    binary = read("src/bin/reta_arch_activation_promotion.rs")
    cargo = read("Cargo.toml")
    ffi = read("src/ffi.rs")
    shadow = read("src/reta_arch_shadow.rs")
    workflow = read("src/reta_workflow_py.rs")

    checks = {
        "promotion_module_present": "pub struct TableViewActivationPromotionReport" in module,
        "promotion_policy_from_cli_present": "pub fn from_cli_args(args: &[String], base: &Self)" in module,
        "promotion_from_readiness_present": "pub fn activation_promotion_from_readiness" in module,
        "promotion_for_cli_args_present": "pub fn activation_promotion_for_cli_args" in module,
        "promotion_smoke_present": "continuum_m_activation_promotion_smoke" in module,
        "strict_raw_guard_present": "raw_line_diff_equal" in module,
        "virtual_direct_guard_present": "virtual_direct_cells_equal" in module,
        "lib_module_exported": "pub mod table_view_activation_promotion" in lib,
        "lib_symbols_reexported": "TableViewActivationPromotionPolicy" in lib
        and "activation_promotion_for_cli_args" in lib,
        "facade_runtime_field_present": "table_view_activation_promotion: TableViewActivationPromotionBundle" in facade,
        "facade_snapshot_present": "rust_table_view_activation_promotion_morphism_count" in facade,
        "runtime_switch_known_morphism_present": "table_view_activation_promotion.default_visible_source" in runtime_switch,
        "runtime_switch_strips_cli_flags": "--activation-promotion-diagnostic" in runtime_switch
        and "--activation-promotion-preview=" in runtime_switch,
        "migration_step_present": "step-table-view-activation-promotion" in migration,
        "inspect_binary_registered": 'name = "rreta_arch_activation_promotion"' in cargo,
        "inspect_binary_uses_policy": "TableViewActivationPromotionPolicy::from_cli_args" in binary,
        "ffi_export_present": "reta_architecture_table_view_activation_promotion_json" in ffi,
        "shadow_bridge_builds_promotion": "view_output_promotion" in shadow
        and "activation_promotion_from_readiness" in shadow,
        "workflow_diagnostic_present": "ARCH_TABLE_VIEW_ACTIVATION_PROMOTION" in workflow,
        "duplicate_switch_config_removed": shadow.count("let (_, switch_config) = reta_architecture::extract_architecture_switch_from_argv(argv, None);") == 1,
        "duplicate_view_output_replay_literal_removed": shadow.count("        view_output_replay,\n") == 1,
    }
    failed = [name for name, ok in checks.items() if not ok]
    out = {
        "stage": 54,
        "status": "ok" if not failed else "failed",
        "checks": checks,
        "failed": failed,
        "new_module": "crates/reta_architecture/src/table_view_activation_promotion.rs",
        "new_binary": "rreta_arch_activation_promotion",
        "new_ffi": "reta_architecture_table_view_activation_promotion_json",
        "fixed_shadow_bridge_duplicates": checks["duplicate_switch_config_removed"]
        and checks["duplicate_view_output_replay_literal_removed"],
        "policy_flags": [
            "--activation-promotion-strict",
            "--activation-promotion-diagnostic",
            "--activation-promotion-allow-force",
            "--activation-promotion-require-commit-mode",
            "--activation-promotion-ignore-commit-mode",
            "--activation-promotion-require-readiness",
            "--activation-promotion-ignore-readiness",
            "--activation-promotion-include-selected-lines",
            "--activation-promotion-preview=N",
        ],
    }
    print(json.dumps(out, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if not failed else 1


if __name__ == "__main__":
    raise SystemExit(main())
