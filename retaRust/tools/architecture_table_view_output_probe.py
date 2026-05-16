#!/usr/bin/env python3
"""Stage-23 smoke probe for the materialized table-view output bridge.

The probe does not require a Rust build.  It checks the static Rust hooks and
uses the generated Python reference assets to verify the concrete continuum/m
case that has driven the porting work: 493 is direct CSV-backed, and after the
Stage-55 religion.csv update 744 is also directly CSV-backed as `Neues M`.
"""

from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def parse_religion_row(index: int) -> list[str]:
    with (ROOT / "python_arch_reference" / "csv" / "religion.csv").open(
        encoding="utf-8", newline=""
    ) as handle:
        reader = csv.reader(handle, delimiter=";")
        for row_index, row in enumerate(reader):
            if row_index == index:
                return row
    return []


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    lib = read(ROOT / "crates" / "reta_architecture" / "src" / "lib.rs")
    facade = read(ROOT / "crates" / "reta_architecture" / "src" / "facade.rs")
    shadow = read(ROOT / "crates" / "reta_architecture" / "src" / "shadow_pipeline.rs")
    runtime_switch = read(ROOT / "crates" / "reta_architecture" / "src" / "runtime_switch.rs")
    migration = read(ROOT / "crates" / "reta_architecture" / "src" / "migration_control.rs")
    ffi = read(ROOT / "src" / "ffi.rs")
    cargo = read(ROOT / "Cargo.toml")
    module = read(ROOT / "crates" / "reta_architecture" / "src" / "table_view_output.rs")

    header = parse_religion_row(0)
    first = parse_religion_row(1)
    direct_493_ok = len(header) > 493 and "M Kontinuum" in header[493]
    first_493_ok = len(first) > 493 and "Wege-Gabelung" in first[493]
    column_744_directly_addressable = len(header) > 744
    direct_744_ok = column_744_directly_addressable and "Neues M" in header[744]

    required_symbols = [
        "TableViewOutputBundle",
        "TableViewOutputConfig",
        "render_materialized_table_view",
        "render_table_view_for_cli_args",
        "render_html_rows",
        "render_markdown_rows",
        "render_csv_rows",
        "continuum_m_table_view_output_smoke",
    ]
    module_symbols_present = {symbol: symbol in module for symbol in required_symbols}

    result = {
        "status": "ok",
        "module_present": "pub mod table_view_output" in lib,
        "public_exports_present": "bootstrap_table_view_output" in lib
        and "render_table_view_for_cli_args" in lib,
        "facade_runtime_field_present": "table_view_output: TableViewOutputBundle" in facade,
        "facade_snapshot_counts_present": "rust_table_view_output_morphism_count" in facade,
        "shadow_plan_contains_output_report": "table_view_output: TableViewOutputReport" in shadow,
        "runtime_gate_present": "table_view_output.render" in runtime_switch,
        "migration_step_present": "step-table-view-output" in migration,
        "ffi_export_present": "reta_architecture_table_view_output_json" in ffi,
        "binary_present": "rreta_arch_view_output" in cargo,
        "module_symbols_present": module_symbols_present,
        "all_module_symbols_present": all(module_symbols_present.values()),
        "religion_header_493": header[493] if len(header) > 493 else None,
        "religion_first_row_493_contains_wege_gabelung": first_493_ok,
        "direct_493_ok": direct_493_ok,
        "column_744_directly_addressable": column_744_directly_addressable,
        "direct_744_ok": direct_744_ok,
        "virtual_policy_default_suppress": "VirtualColumnDisplayPolicy::Suppress" in module,
        "markdown_separator_supported": "include_markdown_header_separator" in module,
        "html_escape_supported": "html_escape_cell" in module,
        "csv_escape_supported": "csv_escape_cell" in module,
    }
    if not all(
        value
        for key, value in result.items()
        if key
        not in {
            "religion_header_493",
            "module_symbols_present",
        }
    ):
        result["status"] = "failed"
    # Stage 55 expects 744 to be directly addressable in the updated religion.csv.
    if not result["direct_744_ok"]:
        result["status"] = "failed"

    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None))
    return 0 if result["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
