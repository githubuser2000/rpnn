#!/usr/bin/env python3
"""Static smoke probe for Stage 46 activation persistence bridge."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def contains(path: str, needle: str) -> bool:
    return needle in (ROOT / path).read_text(encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    checks = {
        "module_exists": (ROOT / "crates/reta_architecture/src/table_view_activation_persistence.rs").exists(),
        "lib_module_export": contains("crates/reta_architecture/src/lib.rs", "pub mod table_view_activation_persistence;"),
        "lib_public_api": contains("crates/reta_architecture/src/lib.rs", "TableViewActivationPersistenceReport"),
        "runtime_field": contains("crates/reta_architecture/src/facade.rs", "table_view_activation_persistence"),
        "runtime_switch_gates": contains("crates/reta_architecture/src/runtime_switch.rs", "table_view_activation_persistence.persist_store_text"),
        "migration_step": contains("crates/reta_architecture/src/migration_control.rs", "step-table-view-activation-persistence"),
        "root_shadow_bridge": contains("src/reta_arch_shadow.rs", "view_output_persistence"),
        "root_diagnostic": contains("src/reta_workflow_py.rs", "ARCH_TABLE_VIEW_ACTIVATION_PERSISTENCE"),
        "ffi_export": contains("src/ffi.rs", "reta_architecture_table_view_activation_persistence_json"),
        "inspect_binary": contains("Cargo.toml", "rreta_arch_activation_persistence"),
        "persistence_snapshot_mentions_activation": contains("crates/reta_architecture/src/persistence.rs", "persist_activation_store_text"),
        "roundtrip_test_present": contains("crates/reta_architecture/src/table_view_activation_persistence.rs", "activation_persistence_roundtrips_ready_store"),
    }
    status = "ok" if all(checks.values()) else "blocked"
    result = {
        "stage": 46,
        "status": status,
        "checks": checks,
        "failed": [name for name, ok in checks.items() if not ok],
        "universal_property": "persisted_activation_store_is_ready_only_when_load_parse_and_hash_match_the_source_store",
    }
    print(json.dumps(result, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
