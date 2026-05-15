#!/usr/bin/env python3
"""Static smoke probe for Stage 47 activation file backend."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED_SYMBOLS = {
    "crates/reta_architecture/src/table_view_activation_file.rs": [
        "TableViewActivationFilePolicy",
        "TableViewActivationFileReport",
        "TableViewActivationFileSnapshot",
        "activation_file_policy_from_cli_args",
        "activation_file_for_cli_args",
        "write_activation_store_file",
        "read_activation_store_file",
        "atomic_write_text",
        "read_digest_mismatch",
        "activation_file_roundtrips_ready_store",
    ],
    "crates/reta_architecture/src/lib.rs": [
        "pub mod table_view_activation_file;",
        "TableViewActivationFileReport",
        "activation_file_for_cli_args",
        "write_activation_store_file",
    ],
    "crates/reta_architecture/src/facade.rs": [
        "table_view_activation_file",
        "rust_table_view_activation_file_morphism_count",
        "rust_table_view_activation_file_validation_status",
    ],
    "crates/reta_architecture/src/runtime_switch.rs": [
        "table_view_activation_file.atomic_write_store",
        "table_view_activation_file.read_store_file",
        "--activation-store-file",
    ],
    "crates/reta_architecture/src/migration_control.rs": [
        "step-table-view-activation-file",
        "table_view_activation_file.atomic_write_store",
    ],
    "src/reta_arch_shadow.rs": [
        "view_output_file",
        "activation_file_policy_from_cli_args",
        "write_activation_store_file",
    ],
    "src/reta_workflow_py.rs": [
        "ARCH_TABLE_VIEW_ACTIVATION_FILE",
        "file.read_matches_source",
    ],
    "src/ffi.rs": [
        "reta_architecture_table_view_activation_file_json",
        "activation_file_for_cli_args",
    ],
    "Cargo.toml": [
        "rreta_arch_activation_file",
        "src/bin/reta_arch_activation_file.rs",
    ],
    "src/bin/reta_arch_activation_file.rs": [
        "--activation-store-file",
        "--read-existing-file",
        "activation_file_for_cli_args",
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

    file_rs = read("crates/reta_architecture/src/table_view_activation_file.rs")
    runtime_switch_rs = read("crates/reta_architecture/src/runtime_switch.rs")
    workflow_rs = read("src/reta_workflow_py.rs")

    checks = {
        "module_exists": (ROOT / "crates/reta_architecture/src/table_view_activation_file.rs").exists(),
        "atomic_write_present": "atomic_write_text" in file_rs,
        "readback_digest_guard": "read_digest_mismatch" in file_rs,
        "parse_read_store_guard": "parsed_read_store_not_ready" in file_rs,
        "cli_flags_stripped_before_legacy": "--activation-store-file" in runtime_switch_rs,
        "root_file_diagnostic": "ARCH_TABLE_VIEW_ACTIVATION_FILE" in workflow_rs,
        "optional_root_write": "file_enabled" in read("src/reta_arch_shadow.rs"),
        "ffi_export": "reta_architecture_table_view_activation_file_json" in read("src/ffi.rs"),
        "binary_target": "rreta_arch_activation_file" in read("Cargo.toml"),
    }

    status = "ok" if not missing and all(checks.values()) else "blocked"
    result = {
        "stage": 47,
        "status": status,
        "missing": missing,
        "checks": checks,
        "universal_property": "file_backed_activation_store_is_ready_only_when_write_read_parse_and_digest_are_identity",
    }
    print(json.dumps(result, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
