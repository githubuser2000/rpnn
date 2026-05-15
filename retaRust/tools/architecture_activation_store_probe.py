#!/usr/bin/env python3
"""Static smoke probe for the Stage-45 activation-store layer.

The probe is intentionally dependency-free. It verifies that the Rust tree has a
line-oriented activation store module, that root diagnostics/FFI/bin hooks are
wired, and that the migration/runtime gates know the new store morphisms.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

REQUIRED_SYMBOLS = {
    "crates/reta_architecture/src/table_view_activation_store.rs": [
        "TableViewActivationStorePolicy",
        "TableViewActivationStoreValidation",
        "TableViewActivationStore",
        "TableViewActivationStoreParseReport",
        "activation_store_for_cli_args",
        "activation_store_from_journal_and_ledger",
        "activation_store_text_from_journal_and_ledger",
        "parse_activation_store_text",
        "continuum_m_activation_store_smoke",
        "escape_field",
        "unescape_field",
    ],
    "crates/reta_architecture/src/lib.rs": [
        "pub mod table_view_activation_store;",
        "pub use table_view_activation_store",
        "TableViewActivationStorePolicy",
    ],
    "crates/reta_architecture/src/facade.rs": [
        "table_view_activation_store",
        "rust_table_view_activation_store_morphism_count",
        "rust_table_view_activation_store_validation_status",
    ],
    "crates/reta_architecture/src/runtime_switch.rs": [
        "table_view_activation_store.encode_line_store",
        "table_view_activation_store.parse_line_store",
        "table_view_activation_store.validate_stored_hash_chain",
        "table_view_activation_store.rollback_on_store_drift",
    ],
    "crates/reta_architecture/src/migration_control.rs": [
        "step-table-view-activation-store",
        "table_view_activation_store.encode_line_store",
        "table_view_activation_store.validate_stored_hash_chain",
    ],
    "src/reta_arch_shadow.rs": [
        "view_output_store",
        "activation_store_from_journal_and_ledger",
    ],
    "src/reta_workflow_py.rs": [
        "ARCH_TABLE_VIEW_ACTIVATION_STORE",
        "store.validation.status",
    ],
    "src/ffi.rs": [
        "reta_architecture_table_view_activation_store_json",
        "parse_activation_store_text",
    ],
    "Cargo.toml": [
        "rreta_arch_activation_store",
        "src/bin/reta_arch_activation_store.rs",
    ],
    "src/bin/reta_arch_activation_store.rs": [
        "--store-file",
        "activation_store_for_cli_args",
        "parse_activation_store_text",
    ],
}


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def main() -> None:
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

    store_rs = read("crates/reta_architecture/src/table_view_activation_store.rs")
    line_store_has_journal_lines = '"J".to_string()' in store_rs
    line_store_has_selected_lines = '"L".to_string()' in store_rs
    line_store_has_ledger_lines = '"G".to_string()' in store_rs
    detects_tampering_test = "activation_store_detects_tampered_record" in store_rs
    roundtrip_test = "activation_store_roundtrips_continuum_smoke" in store_rs

    result = {
        "stage": 45,
        "status": "ok" if not missing else "missing-symbols",
        "missing": missing,
        "module_present": (ROOT / "crates/reta_architecture/src/table_view_activation_store.rs").exists(),
        "line_store_record_kinds": {
            "header": '"H".to_string()' in store_rs,
            "journal_record": line_store_has_journal_lines,
            "selected_line": line_store_has_selected_lines,
            "ledger_hash": line_store_has_ledger_lines,
        },
        "tests_present": {
            "roundtrip": roundtrip_test,
            "tamper_detection": detects_tampering_test,
            "escaping": "activation_store_escapes_selected_lines" in store_rs,
        },
        "root_diagnostic": "ARCH_TABLE_VIEW_ACTIVATION_STORE" in read("src/reta_workflow_py.rs"),
        "ffi_export": "reta_architecture_table_view_activation_store_json" in read("src/ffi.rs"),
        "binary_target": "rreta_arch_activation_store" in read("Cargo.toml"),
    }
    print(json.dumps(result, indent=2 if args.pretty else None, sort_keys=True))


if __name__ == "__main__":
    main()
