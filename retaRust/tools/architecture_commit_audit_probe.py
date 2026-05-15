#!/usr/bin/env python3
"""Static Stage-40 probe for table-view output commit audit witnesses."""
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

    audit = read("crates/reta_architecture/src/table_view_commit_audit.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    shadow = read("crates/reta_architecture/src/shadow_pipeline.rs")
    runtime_switch = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    binary = read("src/bin/reta_arch_commit_audit.rs")

    checks = {
        "module_file_present": "TableViewCommitAuditReport" in audit,
        "required_checks_named": all(name in audit for name in [
            "commit_gate_allowed",
            "raw_line_diff_equal_or_force",
            "virtual_direct_cells_equal",
            "decision_uses_view_output",
        ]),
        "diagnostic_checks_named": all(name in audit for name in [
            "semantic_rows_equal",
            "virtual_added_columns_are_witnesses",
            "rollback_anchor_recorded",
        ]),
        "audit_uses_shadow_report_and_decision": "ShadowTableViewOutputReport" in audit and "ShadowTableViewOutputCommitDecision" in audit,
        "safe_to_commit_requires_failed_list_empty_and_decision": "failed_required_checks.is_empty() && decision.use_view_output" in audit,
        "continuum_smoke_present": "continuum_m_commit_audit_smoke" in audit and "--virtualcolumns" in audit,
        "lib_exports_commit_audit": "pub mod table_view_commit_audit" in lib and "audit_table_view_output_commit" in lib,
        "facade_runtime_contains_audit": "table_view_commit_audit" in facade and "rust_table_view_commit_audit_morphism_count" in facade,
        "shadow_snapshot_mentions_audit": "table_view_commit_audit.audit_report" in shadow,
        "runtime_switch_gates_present": "table_view_commit_audit.audit_report" in runtime_switch and "table_view_commit_audit.required_guards" in runtime_switch,
        "migration_step_present": "step-table-view-commit-audit" in migration,
        "ffi_export_present": "reta_architecture_table_view_commit_audit_json" in ffi,
        "binary_registered": "rreta_arch_commit_audit" in cargo and "reta_arch_commit_audit.rs" in cargo,
        "binary_reads_legacy_lines": "--legacy-lines-file" in binary and "audit_table_view_output_commit" in binary,
    }
    status = "ok" if all(checks.values()) else "failed"
    report = {
        "stage": 40,
        "status": status,
        "checks": checks,
        "required_guard_count": 4,
        "diagnostic_guard_count": 3,
        "universal_property": "raw diff, semantic diff, runtime gate and virtual direct-cell identity are glued into one auditable commit witness before visible output can switch.",
    }
    print(json.dumps(report, indent=2 if args.pretty else None, ensure_ascii=False, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
