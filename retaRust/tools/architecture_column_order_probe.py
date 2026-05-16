#!/usr/bin/env python3
"""Stage-26 probe for explicit output-column order.

This probe does not require a Rust build. It checks that the Stage-26 Rust
sources carry `--spaltenreihenfolgeundnurdiese` through the architecture path:
parameter runtime -> generation plan -> materialization -> table view/output.
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

    files = {
        "parameter_runtime": ROOT / "crates" / "reta_architecture" / "src" / "parameter_runtime.rs",
        "table_generation": ROOT / "crates" / "reta_architecture" / "src" / "table_generation.rs",
        "table_materialization": ROOT / "crates" / "reta_architecture" / "src" / "table_materialization.rs",
        "table_view": ROOT / "crates" / "reta_architecture" / "src" / "table_view.rs",
        "runtime_switch": ROOT / "crates" / "reta_architecture" / "src" / "runtime_switch.rs",
        "migration_control": ROOT / "crates" / "reta_architecture" / "src" / "migration_control.rs",
        "ffi": ROOT / "src" / "ffi.rs",
        "cargo": ROOT / "Cargo.toml",
    }
    text = {name: read(path) for name, path in files.items()}
    religion = load_religion()
    max_columns = max(len(row) for row in religion)
    header_493 = religion[0][493]
    data_493 = religion[1][493]

    expected_requested_order = [744, 493]
    suppressed_visible_columns = [744, 493]
    tag_summary_visible_columns = [744, 493]

    checks = {
        "ordered_range_parser_present": "fn ordered_range_numbers" in text["parameter_runtime"],
        "parameter_runtime_order_test_present": "output_spaltenreihenfolgeundnurdiese_preserves_explicit_order" in text["parameter_runtime"],
        "generation_plan_carries_override": "pub column_order_override: Vec<i64>" in text["table_generation"],
        "generation_plan_order_method_present": "pub fn ordered_selected_columns" in text["table_generation"],
        "materialization_request_carries_order": "pub column_order_legacy: Vec<usize>" in text["table_materialization"],
        "materialization_report_carries_order": "pub materialized_column_order_legacy: Vec<usize>" in text["table_materialization"],
        "materialization_uses_ordered_projection": "ordered_columns_for_projection" in text["table_materialization"],
        "materialization_order_test_present": "spaltenreihenfolgeundnurdiese_preserves_requested_materialization_order" in text["table_materialization"],
        "table_view_order_test_present": "explicit_spaltenreihenfolge_places_direct_744_before_493" in text["table_view"],
        "runtime_gate_present": "table_materialization.column_order_override" in text["runtime_switch"],
        "migration_step_present": "step-column-order-override" in text["migration_control"],
        "ffi_export_present": "reta_architecture_column_order_json" in text["ffi"],
        "inspect_binary_declared": "rreta_arch_column_order" in text["cargo"],
        "religion_493_is_direct": "M Kontinuum" in header_493 and "Wege-Gabelung" in data_493,
        "religion_744_is_direct": 744 < max_columns and "Neues M" in religion[0][744],
    }

    result = {
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "expected_requested_order": expected_requested_order,
        "suppressed_policy_visible_columns": suppressed_visible_columns,
        "tag_summary_policy_visible_columns": tag_summary_visible_columns,
        "source_max_columns": max_columns,
        "direct_column_493_header": header_493,
        "direct_column_493_first_data_contains_wege_gabelung": "Wege-Gabelung" in data_493,
        "column_744_directly_addressable": 744 < max_columns,
        "stage55_invariant": "spaltenreihenfolgeundnurdiese order is preserved before rendering; 744 is now a direct CSV-backed column in the updated religion.csv",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if result["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
