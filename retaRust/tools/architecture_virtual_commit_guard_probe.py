#!/usr/bin/env python3
"""Static Stage-39 probe for CLI-aware virtual column parity commit guards."""
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

    virtual_parity = (ROOT / "crates/reta_architecture/src/table_view_virtual_parity.rs").read_text(encoding="utf-8")
    shadow = (ROOT / "crates/reta_architecture/src/shadow_pipeline.rs").read_text(encoding="utf-8")
    ffi = (ROOT / "src/ffi.rs").read_text(encoding="utf-8")
    runtime_switch = (ROOT / "crates/reta_architecture/src/runtime_switch.rs").read_text(encoding="utf-8")
    migration = (ROOT / "crates/reta_architecture/src/migration_control.rs").read_text(encoding="utf-8")

    checks = {
        "unused_table_view_output_report_import_removed": "TableViewOutputReport" not in virtual_parity.split("\n", 40)[0:40],
        "cli_virtual_policy_lift_function_present": "with_cli_virtual_options" in virtual_parity and "from_cli_args" in virtual_parity,
        "virtual_parity_report_records_cli_source": "rendered_policy_source" in virtual_parity and "cli_virtual_option_count" in virtual_parity,
        "shadow_report_carries_virtual_parity": "virtual_column_parity: TableViewVirtualParityReport" in shadow,
        "shadow_cli_plan_uses_cli_virtual_policy": "TableViewVirtualParityConfig::from_cli_args" in shadow,
        "commit_policy_requires_virtual_direct_identity": "require_virtual_direct_identity" in shadow,
        "commit_decision_exposes_virtual_direct_identity": "virtual_direct_cells_equal" in shadow and "virtual_added_column_count" in shadow,
        "commit_evaluator_rejects_direct_virtual_drift": "virtual_policy_changed_direct_csv_cells" in shadow,
        "ffi_virtual_parity_uses_cli_policy": "TableViewVirtualParityConfig::from_cli_args(&args, mode)" in ffi,
        "runtime_gate_present": "table_view_output.commit_virtual_guard" in runtime_switch,
        "migration_step_present": "step-table-view-virtual-commit-guard" in migration,
    }
    status = "ok" if all(checks.values()) else "failed"
    report = {
        "stage": 39,
        "status": status,
        "checks": checks,
        "universal_property": "CLI-selected virtual-column policies may be observed or rendered only if direct CSV cells stay identity before any guarded commit.",
    }
    print(json.dumps(report, indent=2 if args.pretty else None, ensure_ascii=False, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
