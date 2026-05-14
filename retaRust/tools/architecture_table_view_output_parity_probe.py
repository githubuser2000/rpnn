#!/usr/bin/env python3
"""Stage-25 smoke probe for materialized table-view-output semantic parity.

This probe does not need a Rust build.  It checks the Rust source wiring for the
new normalization/parity module and mirrors the most important normalization
behaviour in Python: Markdown separator rows, HTML table wrappers and CSV quotes
should not look like semantic cell mismatches.
"""

from __future__ import annotations

import argparse
import csv
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def normalize_markdown(lines: list[str]) -> list[list[str]]:
    rows: list[list[str]] = []
    for line in lines:
        stripped = line.strip()
        if not stripped:
            continue
        if "|" not in stripped:
            cells = [stripped]
        else:
            cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if cells and all(cell.strip().strip(":").strip("-").strip() == "" and "-" in cell for cell in cells):
            continue
        rows.append([re.sub(r"\s+", " ", cell).strip() for cell in cells if cell.strip()])
    return rows


def normalize_html(lines: list[str]) -> list[list[str]]:
    rows: list[list[str]] = []
    for line in lines:
        cells = re.findall(r"<(?:td|th)[^>]*>(.*?)</(?:td|th)>", line, flags=re.I)
        if not cells:
            continue
        rows.append([
            re.sub(r"\s+", " ", cell.replace("&amp;", "&").replace("&lt;", "<").replace("&gt;", ">")).strip()
            for cell in cells
        ])
    return rows


def normalize_csv(lines: list[str]) -> list[list[str]]:
    return [[cell.strip() for cell in row] for row in csv.reader(lines, delimiter=";")]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    lib = read(ROOT / "crates" / "reta_architecture" / "src" / "lib.rs")
    facade = read(ROOT / "crates" / "reta_architecture" / "src" / "facade.rs")
    shadow = read(ROOT / "crates" / "reta_architecture" / "src" / "shadow_pipeline.rs")
    runtime_switch = read(ROOT / "crates" / "reta_architecture" / "src" / "runtime_switch.rs")
    migration = read(ROOT / "crates" / "reta_architecture" / "src" / "migration_control.rs")
    workflow = read(ROOT / "src" / "reta_workflow_py.rs")
    ffi = read(ROOT / "src" / "ffi.rs")
    cargo = read(ROOT / "Cargo.toml")
    module_path = ROOT / "crates" / "reta_architecture" / "src" / "table_view_output_parity.rs"
    module = read(module_path)
    binary = read(ROOT / "src" / "bin" / "reta_arch_view_output_parity.rs")

    markdown_left = ["| A |", "| x |"]
    markdown_right = ["| A |", "| --- |", "| x |"]
    html_left = ["<table border=0 id=\"bigtable\">", "<tr>", "<td>A &amp; B</td>", "</tr>", "</table>"]
    html_right = ["<td>A &amp; B</td>"]
    csv_rows = normalize_csv(['a;"b;c"'])

    checks = {
        "module_file_present": module_path.exists(),
        "module_declared": "pub mod table_view_output_parity" in lib,
        "public_exports_present": "TableViewOutputParityReport" in lib and "compare_table_view_output_to_legacy" in lib,
        "facade_runtime_field_present": "table_view_output_parity: TableViewOutputParityBundle" in facade,
        "facade_snapshot_count_present": "rust_table_view_output_parity_morphism_count" in facade,
        "shadow_report_carries_semantic_diff": "semantic_diff: TableViewOutputParityReport" in shadow,
        "shadow_commit_carries_semantic_equal": "semantic_equal: report.semantic_diff.semantic_equal" in shadow,
        "runtime_switch_gate_present": "table_view_output.semantic_diff" in runtime_switch,
        "migration_step_present": "step-table-view-output-parity" in migration,
        "workflow_diagnostic_mentions_semantic_equal": "semantic_equal" in workflow,
        "ffi_export_present": "reta_architecture_table_view_output_parity_json" in ffi,
        "binary_declared": "rreta_arch_view_output_parity" in cargo,
        "binary_reads_legacy_file": "--legacy-lines-file" in binary,
        "module_has_markdown_separator_logic": "is_markdown_separator_row" in module,
        "module_has_html_parser": "parse_html_cells" in module,
        "module_has_csv_quote_parser": "parse_csv_line" in module,
        "markdown_separator_ignored": normalize_markdown(markdown_left) == normalize_markdown(markdown_right),
        "html_wrappers_ignored": normalize_html(html_left) == normalize_html(html_right) == [["A & B"]],
        "csv_separator_inside_quotes_kept": csv_rows == [["a", "b;c"]],
    }

    result = {
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "markdown_semantic_rows": normalize_markdown(markdown_right),
        "html_semantic_rows": normalize_html(html_left),
        "csv_semantic_rows": csv_rows,
        "expected_commit_rule": "raw equality is still the only normal commit criterion; semantic equality is diagnostic until an explicit future gate changes policy",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if result["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
