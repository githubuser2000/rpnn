#!/usr/bin/env python3
"""Static smoke probe for Stage 51 activation-readiness wiring."""
from __future__ import annotations
import argparse, json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

def contains(path: str, needle: str) -> bool:
    return needle in (ROOT / path).read_text(encoding="utf-8")

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()
    checks = {
        "module_exists": (ROOT / "crates/reta_architecture/src/table_view_activation_readiness.rs").exists(),
        "lib_pub_mod": contains("crates/reta_architecture/src/lib.rs", "pub mod table_view_activation_readiness;"),
        "lib_root_exports": contains("crates/reta_architecture/src/lib.rs", "activation_readiness_for_cli_args"),
        "facade_runtime_field": contains("crates/reta_architecture/src/facade.rs", "table_view_activation_readiness: TableViewActivationReadinessBundle"),
        "facade_snapshot_count": contains("crates/reta_architecture/src/facade.rs", "rust_table_view_activation_readiness_morphism_count"),
        "runtime_switch_gates": contains("crates/reta_architecture/src/runtime_switch.rs", "table_view_activation_readiness.default_promotion_gate"),
        "migration_step": contains("crates/reta_architecture/src/migration_control.rs", "step-table-view-activation-readiness"),
        "shadow_bridge_report_field": contains("src/reta_arch_shadow.rs", "view_output_readiness: Option<reta_architecture::TableViewActivationReadinessReport>"),
        "root_diagnostic": contains("src/reta_workflow_py.rs", "ARCH_TABLE_VIEW_ACTIVATION_READINESS"),
        "ffi_export": contains("src/ffi.rs", "reta_architecture_table_view_activation_readiness_json"),
        "inspect_binary_declared": contains("Cargo.toml", "rreta_arch_activation_readiness"),
        "inspect_binary_exists": (ROOT / "src/bin/reta_arch_activation_readiness.rs").exists(),
    }
    status = "ok" if all(checks.values()) else "blocked"
    result = {"status": status, "checks": checks, "failed": [k for k,v in checks.items() if not v]}
    print(json.dumps(result, indent=2 if args.pretty else None, ensure_ascii=False))
    return 0 if status == "ok" else 1

if __name__ == "__main__":
    raise SystemExit(main())
