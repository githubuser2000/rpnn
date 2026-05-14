#!/usr/bin/env python3
"""Stage-28 output-flag probe.

This probe is intentionally build-free. It checks that the Rust table-view
output path owns the visible Ausgabe flags before any guarded commit can use
that path: header suppression, empty-row filtering, width wrapping and the
legacy no-color/justtext/onetable/endless flags.
"""
from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read_text(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def religion_cell(row_index: int, column: int) -> str:
    with (ROOT / "python_arch_reference" / "csv" / "religion.csv").open(
        "r", encoding="utf-8", newline=""
    ) as handle:
        rows = list(csv.reader(handle, delimiter=";"))
    if 0 <= row_index < len(rows) and column < len(rows[row_index]):
        return rows[row_index][column]
    return ""


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    table_view_output = read_text("crates/reta_architecture/src/table_view_output.rs")
    runtime_switch = read_text("crates/reta_architecture/src/runtime_switch.rs")
    migration = read_text("crates/reta_architecture/src/migration_control.rs")
    shadow = read_text("crates/reta_architecture/src/shadow_pipeline.rs")
    ffi = read_text("src/ffi.rs")
    cargo = read_text("Cargo.toml")
    materialization = read_text("crates/reta_architecture/src/table_materialization.rs")

    required_symbols = [
        "TableViewOutputCliOptions",
        "parse_table_view_output_cli_options",
        "filtered_output_rows",
        "rendered_row_value_lines",
        "wrap_output_cell",
        "row_values_with_options",
        "output_flags_smoke",
    ]
    symbol_checks = {symbol: symbol in table_view_output for symbol in required_symbols}
    option_keys = [
        "nocolor",
        "justtext",
        "onetable",
        "endlessscreen",
        "endless",
        "dontwrap",
        "breite",
        "breiten",
        "keineleereninhalte",
        "keinenummerierung",
        "keineueberschriften",
    ]
    option_checks = {key: key in table_view_output for key in option_keys}
    checks = {
        "all_output_symbols_present": all(symbol_checks.values()),
        "all_output_options_present": all(option_checks.values()),
        "runtime_gate_output_flags": "table_view_output.output_flags" in runtime_switch,
        "runtime_gate_width": "table_view_output.width_wrapping" in runtime_switch,
        "runtime_gate_header_filter": "table_view_output.header_filter" in runtime_switch,
        "migration_step": "step-table-view-output-flags" in migration,
        "shadow_cli_uses_render_cli_args": "render_cli_args" in shadow and "TableViewOutputConfig::default().with_mode(output_mode)" in shadow,
        "ffi_export": "reta_architecture_table_view_output_options_json" in ffi,
        "inspect_binary": "rreta_arch_output_flags" in cargo and (ROOT / "src/bin/reta_arch_output_flags.rs").exists(),
        "limit_set_warning_removed": "fn limit_set" not in materialization,
    }

    header = religion_cell(0, 493)
    first_data = religion_cell(1, 493)
    report = {
        "stage": 28,
        "case": "ausgabe-output-flags",
        "args": [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
            "-ausgabe",
            "--keineueberschriften",
            "--keineleereninhalte",
            "--breite=8",
            "--nocolor",
            "--justtext",
        ],
        "religion_header_493": header,
        "religion_first_data_493_preview": first_data[:120],
        "expected_header_suppressed_text": "M Kontinuum",
        "expected_data_still_visible_text": "Wege-Gabelung",
        "symbol_checks": symbol_checks,
        "option_checks": option_checks,
        "checks": checks,
        "status": "ok" if all(checks.values()) else "failed",
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if report["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
