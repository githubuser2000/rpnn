#!/usr/bin/env python3
"""Static smoke probe for Stage 48 activation-file recovery."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED_SYMBOLS = {
    "crates/reta_architecture/src/table_view_activation_recovery.rs": [
        "TableViewActivationRecoveryPolicy",
        "TableViewActivationRecoveryReport",
        "TableViewActivationRecoverySnapshot",
        "activation_recovery_policy_from_cli_args",
        "activation_recovery_for_cli_args",
        "read_activation_store_file_for_recovery",
        "continuum_m_activation_recovery_smoke",
        "recovery_file_path_missing",
        "recovered_store_replay_not_safe",
        "activation_recovery_replays_existing_safe_store_as_candidate",
    ],
    "crates/reta_architecture/src/lib.rs": [
        "pub mod table_view_activation_recovery;",
        "TableViewActivationRecoveryReport",
        "activation_recovery_for_cli_args",
        "read_activation_store_file_for_recovery",
    ],
    "crates/reta_architecture/src/facade.rs": [
        "table_view_activation_recovery",
        "rust_table_view_activation_recovery_morphism_count",
        "rust_table_view_activation_recovery_validation_status",
    ],
    "crates/reta_architecture/src/runtime_switch.rs": [
        "table_view_activation_recovery.read_existing_store_file",
        "table_view_activation_recovery.replay_or_rollback",
        "--activation-recovery-file",
    ],
    "crates/reta_architecture/src/migration_control.rs": [
        "step-table-view-activation-recovery",
        "table_view_activation_recovery.read_existing_store_file",
    ],
    "src/reta_arch_shadow.rs": [
        "view_output_recovery",
        "activation_recovery_policy_from_cli_args",
        "activation_recovery_for_cli_args",
    ],
    "src/reta_workflow_py.rs": [
        "ARCH_TABLE_VIEW_ACTIVATION_RECOVERY",
        "recovery.recover_visible_output",
    ],
    "src/ffi.rs": [
        "reta_architecture_table_view_activation_recovery_json",
        "activation_recovery_for_cli_args",
    ],
    "Cargo.toml": [
        "rreta_arch_activation_recovery",
        "src/bin/reta_arch_activation_recovery.rs",
    ],
    "src/bin/reta_arch_activation_recovery.rs": [
        "--activation-recovery-file",
        "--activation-recovery-allow-replay",
        "activation_recovery_for_cli_args",
    ],
}


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    missing: dict[str, list[str]] = {}
    for rel, symbols in REQUIRED_SYMBOLS.items():
        path = ROOT / rel
        if not path.exists():
            missing[rel] = ["<file missing>"]
            continue
        text = path.read_text(encoding="utf-8")
        absent = [symbol for symbol in symbols if symbol not in text]
        if absent:
            missing[rel] = absent

    recovery_rs = read("crates/reta_architecture/src/table_view_activation_recovery.rs")
    switch_rs = read("crates/reta_architecture/src/runtime_switch.rs")
    workflow_rs = read("src/reta_workflow_py.rs")

    checks = {
        "module_exists": (ROOT / "crates/reta_architecture/src/table_view_activation_recovery.rs").exists(),
        "diagnostic_default_not_visible": "allow_visible_recovery: false" in recovery_rs,
        "explicit_visible_recovery_flag": "--activation-recovery-allow-replay" in recovery_rs,
        "transaction_guard_present": "current_transaction_id" in recovery_rs and "activation_replay_from_journal" in recovery_rs,
        "legacy_checksum_guard_present": "current_legacy_checksum" in recovery_rs,
        "cli_flags_stripped_before_legacy": "--activation-recovery-file" in switch_rs,
        "root_recovery_diagnostic": "ARCH_TABLE_VIEW_ACTIVATION_RECOVERY" in workflow_rs,
        "root_recovery_optional_commit": "recover_visible_output" in workflow_rs,
        "ffi_export": "reta_architecture_table_view_activation_recovery_json" in read("src/ffi.rs"),
        "binary_target": "rreta_arch_activation_recovery" in read("Cargo.toml"),
    }

    status = "ok" if not missing and all(checks.values()) else "blocked"
    result = {
        "stage": 48,
        "status": status,
        "missing": missing,
        "checks": checks,
        "universal_property": "file_recovery_is_identity_on_current_safe_activation_store_and_rollback_on_file_or_checksum_drift",
    }
    print(json.dumps(result, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
