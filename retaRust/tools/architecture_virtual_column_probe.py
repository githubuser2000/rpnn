#!/usr/bin/env python3
"""Stage-37 probe for virtual/non-direct column render policies.

Checks that virtual-column policy remains available after the Stage-55 CSV update, where continuum `744` is now direct and generic non-direct columns such as 999 remain virtual.
suppressed by default, but can be rendered in Rust architecture inspect/shadow
paths through explicit virtual-column policies.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    virtuals = read("crates/reta_architecture/src/table_view_virtual_columns.rs")
    view = read("crates/reta_architecture/src/table_view.rs")
    output = read("crates/reta_architecture/src/table_view_output.rs")
    runtime = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    binary = ROOT / "src/bin/reta_arch_virtual_columns.rs"

    checks = {
        "module_exists": (ROOT / "crates/reta_architecture/src/table_view_virtual_columns.rs").exists(),
        "bundle_declared": "pub struct TableViewVirtualColumnBundle" in virtuals,
        "cli_parser_declared": "parse_table_view_virtual_column_cli_options" in virtuals,
        "policies_declared": all(name in view for name in ["Suppress", "Placeholder", "TagSummary", "Witness"]),
        "direct_744_test_present": "continuum_m_744_is_direct_after_religion_csv_update" in virtuals,
        "tag_summary_test_present": "tag_summary_policy_renders_non_direct_999" in virtuals,
        "placeholder_test_present": "placeholder_policy_can_emit_question_mark_witnesses_for_non_direct_columns" in virtuals,
        "output_config_carries_suppress_question_marks": "pub suppress_question_mark_virtuals: bool" in output,
        "output_cli_flags_parsed": all(flag in output for flag in [
            '"virtualcolumns"', '"virtualplaceholder"', '"virtualwitness"', '"suppressvirtualcolumns"'
        ]),
        "output_uses_virtual_config": "suppress_question_mark_virtuals: mode_config.suppress_question_mark_virtuals" in output,
        "output_report_contains_virtual_policy": "pub virtual_column_policy: String" in output,
        "runtime_gates_present": all(name in runtime for name in [
            "table_view_virtual_columns.policy",
            "table_view_virtual_columns.tag_summary",
            "table_view_virtual_columns.placeholder",
            "table_view_virtual_columns.witness",
        ]),
        "migration_step_present": "step-table-view-virtual-columns" in migration,
        "facade_runtime_contains_bundle": "pub table_view_virtual_columns: TableViewVirtualColumnBundle" in facade,
        "lib_exports_module": "pub mod table_view_virtual_columns" in lib and "TableViewVirtualColumnReport" in lib,
        "ffi_export_present": "reta_architecture_table_view_virtual_columns_json" in ffi,
        "inspect_binary_present": "rreta_arch_virtual_columns" in cargo and binary.exists(),
        "direct_744_transition_preserved": "Neues M" in virtuals and "continuum_m_744_is_direct_after_religion_csv_update" in virtuals,
        "non_direct_999_virtual_policy_preserved": "999:untagged" in virtuals and "non_direct_999_virtual_column_policy_smoke" in virtuals,
    }
    result = {
        "stage": 37,
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "default_policy": "suppress",
        "explicit_policies": ["tag-summary", "placeholder", "witness"],
        "known_regression_case": "-spalten --kontinuum=m --spaltenreihenfolgeundnurdiese=744,493",
        "invariant": (
            "Virtual/non-direct columns remain witnesses by default; explicit CLI/shadow "
            "policies can render tag summaries, placeholders or full witnesses without "
            "weakening legacy-visible output parity."
        ),
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if result["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
