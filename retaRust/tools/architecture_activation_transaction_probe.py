#!/usr/bin/env python3
"""Static Stage-41 probe for guarded table-view activation transactions."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    module = read("crates/reta_architecture/src/table_view_activation_transaction.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    shadow_bridge = read("src/reta_arch_shadow.rs")
    workflow = read("src/reta_workflow_py.rs")
    runtime_switch = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    binary = read("src/bin/reta_arch_activation_transaction.rs")

    checks = {
        "module_file_present": "TableViewActivationTransactionReport" in module,
        "policy_requires_audit_and_commit": "require_audit_safe" in module and "require_commit_decision" in module,
        "selected_source_enum_present": "TableViewActivationLineSource" in module and "LegacyOutput" in module and "TableViewOutput" in module,
        "stable_checksum_present": "stable_line_checksum" in module and "FNV-1a" in module,
        "transaction_uses_audit_report": "audit_table_view_output_commit" in module and "audit.safe_to_commit" in module,
        "transaction_keeps_legacy_on_reject": "keep_legacy_lines_on_reject" in module and "legacy_lines.to_vec()" in module,
        "continuum_smoke_present": "continuum_m_activation_transaction_smoke" in module,
        "lib_exports_activation_transaction": "pub mod table_view_activation_transaction" in lib and "table_view_activation_transaction_for_cli_args" in lib,
        "facade_runtime_contains_transaction": "table_view_activation_transaction" in facade and "rust_table_view_activation_transaction_morphism_count" in facade,
        "root_bridge_carries_transaction": "view_output_transaction" in shadow_bridge and "TableViewActivationTransactionPolicy::default" in shadow_bridge,
        "workflow_uses_transaction_for_visible_selection": "ARCH_TABLE_VIEW_ACTIVATION_TRANSACTION" in workflow and "transaction.selected_lines.clone()" in workflow,
        "runtime_gates_present": "table_view_activation_transaction.select_visible_source" in runtime_switch and "table_view_activation_transaction.rollback_witness" in runtime_switch,
        "migration_step_present": "step-table-view-activation-transaction" in migration,
        "ffi_export_present": "reta_architecture_table_view_activation_transaction_json" in ffi,
        "binary_registered": "rreta_arch_activation_transaction" in cargo and "reta_arch_activation_transaction.rs" in cargo,
        "binary_reads_legacy_lines": "--legacy-lines-file" in binary and "table_view_activation_transaction" in binary,
    }
    status = "ok" if all(checks.values()) else "failed"
    report = {
        "stage": 41,
        "status": status,
        "checks": checks,
        "selected_sources": ["legacy_output", "table_view_output"],
        "required_transaction_inputs": [
            "ShadowTableViewOutputReport",
            "ShadowTableViewOutputCommitDecision",
            "TableViewCommitAuditReport",
            "legacy_visible_lines",
        ],
        "universal_property": "visible output is selected by a unique safe activation transaction that factors through the commit audit and preserves a rollback witness.",
    }
    print(json.dumps(report, indent=2 if args.pretty else None, ensure_ascii=False, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
