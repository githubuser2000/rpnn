#!/usr/bin/env python3
"""Stage-27 row-order probe.

This probe is intentionally dependency-free.  It verifies that the Rust
architecture source carries explicit `-zeilen` order through parameter runtime,
generation, materialization and inspect/FFI surfaces.  It also reads the local
CSV presheaf to provide a data witness for the requested row order.
"""
from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read_text(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def csv_witness(row_order: list[int], column: int = 493) -> list[dict[str, object]]:
    path = ROOT / "python_arch_reference" / "csv" / "religion.csv"
    with path.open("r", encoding="utf-8", newline="") as handle:
        rows = list(csv.reader(handle, delimiter=";"))
    out = []
    for row_index in row_order:
        value = ""
        if 0 <= row_index < len(rows) and column < len(rows[row_index]):
            value = rows[row_index][column]
        out.append(
            {
                "source_row_zero_based": row_index,
                "legacy_column": column,
                "preview": value[:96],
                "contains_text": bool(value),
            }
        )
    return out


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    checks = {
        "parameter_runtime_rows_as_ordered": "rows_as_ordered" in read_text("crates/reta_architecture/src/parameter_runtime.rs"),
        "parameter_runtime_no_column_row_leak_test": "numeric_spalten_range_does_not_select_rows" in read_text("crates/reta_architecture/src/parameter_runtime.rs"),
        "table_generation_row_order_override": "row_order_override" in read_text("crates/reta_architecture/src/table_generation.rs"),
        "table_generation_ordered_selected_rows": "ordered_selected_rows" in read_text("crates/reta_architecture/src/table_generation.rs"),
        "materialization_ordered_rows": "ordered_rows_for_projection" in read_text("crates/reta_architecture/src/table_materialization.rs"),
        "materialization_report_fields": "materialized_row_order_zero_based" in read_text("crates/reta_architecture/src/table_materialization.rs"),
        "runtime_gate": "table_materialization.row_order_override" in read_text("crates/reta_architecture/src/runtime_switch.rs"),
        "migration_step": "step-row-order-override" in read_text("crates/reta_architecture/src/migration_control.rs"),
        "ffi_export": "reta_architecture_row_order_json" in read_text("src/ffi.rs"),
        "inspect_binary": "rreta_arch_row_order" in read_text("Cargo.toml") and (ROOT / "src/bin/reta_arch_row_order.rs").exists(),
    }

    expected_order = [3, 1, 2]
    expected_materialized_order = [0, 3, 1, 2]
    report = {
        "stage": 27,
        "case": "vorhervonausschnitt-order-3-1-2",
        "args": [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=3,1-2",
            "-spalten",
            "--religion=493",
            "--breite=0",
        ],
        "expected_ordered_selected_rows": expected_order,
        "expected_materialized_row_order_zero_based": expected_materialized_order,
        "source_column_493_witnesses": csv_witness(expected_materialized_order),
        "checks": checks,
        "status": "ok" if all(checks.values()) else "failed",
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if report["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
