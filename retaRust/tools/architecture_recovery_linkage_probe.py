#!/usr/bin/env python3
"""Probe Stage 50 recovery linkage repair.

Stage 48 introduced activation recovery. Stage 49 tried to call the public
module path from the root crate, but some build trees exposed only root-level
reexports and not the public module path. Stage 50 makes the architecture crate
explicitly declare the recovery module and root-level reexports, and switches
runtime callers back to those stable root-level exports.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

FILES = {
    "module_file_exists": Path("crates/reta_architecture/src/table_view_activation_recovery.rs"),
    "inspect_binary_exists": Path("src/bin/reta_arch_activation_recovery.rs"),
    "ffi_file_exists": Path("src/ffi.rs"),
    "shadow_bridge_exists": Path("src/reta_arch_shadow.rs"),
}

ROOT_EXPORT_TOKENS = [
    "reta_architecture::TableViewActivationRecoveryPolicy",
    "reta_architecture::TableViewActivationRecoveryReport",
    "reta_architecture::activation_recovery_policy_from_cli_args",
    "reta_architecture::activation_recovery_for_cli_args",
]

MODULE_RUNTIME_PREFIX = "reta_architecture::table_view_activation_recovery::"


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    files_present = {name: (ROOT / rel).exists() for name, rel in FILES.items()}
    binary = read("src/bin/reta_arch_activation_recovery.rs")
    ffi = read("src/ffi.rs")
    shadow = read("src/reta_arch_shadow.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    module = read("crates/reta_architecture/src/table_view_activation_recovery.rs")
    combined_runtime = "\n".join([binary, ffi, shadow])

    root_export_occurrences = {
        token: combined_runtime.count(token) for token in ROOT_EXPORT_TOKENS
    }
    checks = {
        **files_present,
        "lib_declares_public_recovery_module": "pub mod table_view_activation_recovery;" in lib,
        "lib_reexports_recovery_symbols_with_self_path": "pub use self::table_view_activation_recovery" in lib
        and "TableViewActivationRecoveryPolicy" in lib
        and "activation_recovery_for_cli_args" in lib,
        "module_declares_policy": "pub struct TableViewActivationRecoveryPolicy" in module,
        "module_declares_policy_parser": "pub fn activation_recovery_policy_from_cli_args" in module,
        "module_declares_runner": "pub fn activation_recovery_for_cli_args" in module,
        "runtime_uses_root_policy_export": root_export_occurrences[ROOT_EXPORT_TOKENS[0]] >= 2,
        "runtime_uses_root_report_export": root_export_occurrences[ROOT_EXPORT_TOKENS[1]] >= 1,
        "runtime_uses_root_policy_parser_export": root_export_occurrences[ROOT_EXPORT_TOKENS[2]] >= 3,
        "runtime_uses_root_runner_export": root_export_occurrences[ROOT_EXPORT_TOKENS[3]] >= 3,
        "runtime_does_not_call_private_module_path": MODULE_RUNTIME_PREFIX not in combined_runtime,
    }
    missing = {key: value for key, value in checks.items() if not value}
    out = {
        "stage": 50,
        "status": "ok" if not missing else "failed",
        "checks": checks,
        "missing": missing,
        "root_export_occurrences": root_export_occurrences,
        "module_runtime_prefix_remaining": combined_runtime.count(MODULE_RUNTIME_PREFIX),
        "fixed_error": "reta binaries use stable reta_architecture root-level recovery exports; lib.rs explicitly declares and reexports the recovery module",
    }
    print(json.dumps(out, indent=2 if args.pretty else None, sort_keys=True))


if __name__ == "__main__":
    main()
