#!/usr/bin/env python3
"""Stage-24 probe for guarded materialized table-view-output commits.

This probe does not require a Rust build.  It checks that the Rust sources now
wire the materialized `TableViewOutputReport` into the shadow pipeline, commit
policy, runtime diagnostics, migration gates and inspect binary.  It also keeps
the continuum/m witness after the Stage-55 CSV update: both 493 and 744 are directly rendered, while virtual-column policy is reserved for genuinely non-direct columns.
"""

from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def load_religion() -> list[list[str]]:
    with (ROOT / "python_arch_reference" / "csv" / "religion.csv").open(
        encoding="utf-8", newline=""
    ) as handle:
        return list(csv.reader(handle, delimiter=";"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    shadow = read(ROOT / "crates" / "reta_architecture" / "src" / "shadow_pipeline.rs")
    runtime_switch = read(ROOT / "crates" / "reta_architecture" / "src" / "runtime_switch.rs")
    migration = read(ROOT / "crates" / "reta_architecture" / "src" / "migration_control.rs")
    bridge = read(ROOT / "src" / "reta_arch_shadow.rs")
    workflow = read(ROOT / "src" / "reta_workflow_py.rs")
    ffi = read(ROOT / "src" / "ffi.rs")
    cargo = read(ROOT / "Cargo.toml")
    lib = read(ROOT / "crates" / "reta_architecture" / "src" / "lib.rs")
    binary = read(ROOT / "src" / "bin" / "reta_arch_view_output_shadow.rs")

    rows = load_religion()
    source_max_columns = max(len(row) for row in rows)
    header_493 = rows[0][493]
    data_493 = rows[1][493]

    checks = {
        "shadow_report_type": "ShadowTableViewOutputReport" in shadow,
        "shadow_commit_policy_type": "ShadowTableViewOutputCommitPolicy" in shadow,
        "shadow_commit_decision_type": "ShadowTableViewOutputCommitDecision" in shadow,
        "shadow_report_method": "shadow_table_view_output" in shadow,
        "shadow_commit_method": "table_view_output_commit_decision" in shadow,
        "shadow_eval_function": "evaluate_shadow_table_view_output_commit" in shadow,
        "runtime_render_gate": "table_view_output.render" in runtime_switch,
        "runtime_commit_gate": "table_view_output.commit" in runtime_switch,
        "runtime_shadow_adapter_gate": "shadow_pipeline.table_view_output_adapter" in runtime_switch,
        "runtime_shadow_commit_gate": "shadow_pipeline.table_view_output_commit" in runtime_switch,
        "migration_commit_step": "step-table-view-output-commit" in migration,
        "migration_shadow_output_step": "step-shadow-table-view-output" in migration,
        "bridge_carries_view_output": "view_output_report" in bridge and "view_output_commit" in bridge,
        "workflow_diagnostic_output": "ARCH_TABLE_VIEW_OUTPUT" in workflow,
        "workflow_diagnostic_commit": "ARCH_TABLE_VIEW_OUTPUT_COMMIT" in workflow,
        "workflow_prefers_view_output_when_committed": "use_view_output" in workflow,
        "ffi_policy_export": "reta_architecture_table_view_output_commit_policy_json" in ffi,
        "lib_exports_commit_policy": "ShadowTableViewOutputCommitPolicy" in lib,
        "binary_declared": "rreta_arch_view_output_shadow" in cargo,
        "binary_reads_legacy_file": "--legacy-lines-file" in binary,
        "binary_outputs_report_and_commit": '"report"' in binary and '"commit"' in binary,
        "continuum_493_direct_header": "M Kontinuum" in header_493,
        "continuum_493_first_data": "Wege-Gabelung" in data_493,
        "continuum_744_direct": 744 < source_max_columns and "Neues M" in rows[0][744],
    }

    result = {
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "source_max_columns": source_max_columns,
        "header_493": header_493,
        "data_493_contains_wege_gabelung": "Wege-Gabelung" in data_493,
        "expected_commit_rule": "table_view_output commits only when gate is allowed and legacy lines equal materialized view output lines; force may override mismatch",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if result["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
