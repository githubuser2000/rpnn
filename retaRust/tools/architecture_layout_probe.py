#!/usr/bin/env python3
"""Stage-30 static probe for the Rust table-view shell layout projection."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def column_pages(widths: list[int], sep: int, max_width: int | None, onetable: bool) -> list[dict[str, int]]:
    if not widths:
        return []
    if onetable or not max_width:
        return [{"start": 0, "end": len(widths), "width": sum(widths) + sep * (len(widths) - 1)}]
    pages: list[dict[str, int]] = []
    start = 0
    current = 0
    for index, width in enumerate(widths):
        proposed = width if index == start else current + sep + width
        if index > start and proposed > max_width:
            pages.append({"start": start, "end": index, "width": sum(widths[start:index]) + sep * (index - start - 1)})
            start = index
            current = width
        else:
            current = proposed
    pages.append({"start": start, "end": len(widths), "width": sum(widths[start:]) + sep * (len(widths) - start - 1)})
    return pages


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    layout_rs = read("crates/reta_architecture/src/table_view_layout.rs")
    output_rs = read("crates/reta_architecture/src/table_view_output.rs")
    lib_rs = read("crates/reta_architecture/src/lib.rs")
    facade_rs = read("crates/reta_architecture/src/facade.rs")
    runtime_switch = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")

    rows = [["A", "Breit", "C"], ["eins", "zwei", "drei"]]
    measured = [max(len(row[col]) for row in rows) for col in range(3)]
    effective = [measured[0], min(4, measured[1]), measured[2]]
    pages = column_pages(effective, len(" | "), 11, False)

    checks = {
        "module_present": (ROOT / "crates/reta_architecture/src/table_view_layout.rs").exists(),
        "config_type": "TableViewLayoutConfig" in layout_rs,
        "report_type": "TableViewLayoutReport" in layout_rs,
        "measure_function": "measure_column_widths" in layout_rs,
        "page_function": "column_pages_for_widths" in layout_rs,
        "layout_function": "layout_value_rows" in layout_rs,
        "output_config_embeds_layout": "pub layout: TableViewLayoutConfig" in output_rs,
        "shell_renderer_uses_layout": "layout_value_rows(&value_lines" in output_rs,
        "output_report_has_layout_fields": "layout_page_count" in output_rs and "layout_column_widths" in output_rs,
        "lib_exports_layout": "pub mod table_view_layout" in lib_rs and "TableViewLayoutBundle" in lib_rs,
        "facade_tracks_layout": "table_view_layout" in facade_rs and "rust_table_view_layout_morphism_count" in facade_rs,
        "runtime_gates": all(token in runtime_switch for token in ["table_view_layout.column_widths", "table_view_layout.horizontal_pages", "table_view_layout.shell_padding"]),
        "migration_step": "step-table-view-layout" in migration,
        "ffi_export": "reta_architecture_table_view_layout_json" in ffi,
        "inspect_binary": "rreta_arch_layout" in cargo and (ROOT / "src/bin/reta_arch_layout.rs").exists(),
    }
    report = {
        "stage": 30,
        "case": "table-view-shell-layout",
        "sample_rows": rows,
        "measured_widths": measured,
        "effective_widths_with_override_0_4_0": effective,
        "sample_pages_max_width_11": pages,
        "expected_first_padded_line": "A    | Breit | C   ",
        "checks": checks,
        "status": "ok" if all(checks.values()) and pages[0]["start"] == 0 and pages[0]["end"] == 2 else "failed",
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if report["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
