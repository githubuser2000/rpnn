#!/usr/bin/env python3
"""Stage-34 HTML style-composition probe.

Checks that HTML catalog attributes and legacy generateCell wrappers are no
longer mutually exclusive in the Rust TableViewOutput path.  The probe is
static/dependency-free so it can run before a full Rust build.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def contains(path: Path, needle: str) -> bool:
    return needle in path.read_text(encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    table_output = ROOT / "crates/reta_architecture/src/table_view_output.rs"
    runtime = ROOT / "crates/reta_architecture/src/runtime_switch.rs"
    migration = ROOT / "crates/reta_architecture/src/migration_control.rs"
    lib = ROOT / "crates/reta_architecture/src/lib.rs"
    ffi = ROOT / "src/ffi.rs"
    cargo = ROOT / "Cargo.toml"
    binary = ROOT / "src/bin/reta_arch_style_composition.rs"

    text = table_output.read_text(encoding="utf-8")
    checks = {
        "composition_helper_present": "pub fn html_begin_cell_for_output_value" in text,
        "td_merge_present": "pub fn compose_html_td_open_tags" in text,
        "attribute_lookup_used_inside_renderer": "html_attribute_for_cell(cell, &config.html_attributes)" in text,
        "html_renderer_uses_composition": "html_begin_cell_for_output_value(" in text and "render_html_rows_with_config" in text,
        "composition_counts_in_report": all(name in text for name in ["html_cell_style_composition_enabled", "html_cell_style_composition_count", "html_attribute_only_cell_count"]),
        "duplicate_class_guard": "!merged.contains(\"class=\\\"z_0 r_493\\\" class=\")" in text,
        "class_and_style_are_merged": "dedup_words(" in text and "quoted_attr_values" in text and "style_values" in text,
        "lib_exports_helpers": all(name in lib.read_text(encoding="utf-8") for name in ["compose_html_td_open_tags", "html_begin_cell_for_output_value", "html_cell_style_composition_counts"]),
        "runtime_gate_present": "table_view_style_composition.html_cell_merge" in runtime.read_text(encoding="utf-8"),
        "migration_step_present": "step-table-view-style-composition" in migration.read_text(encoding="utf-8"),
        "ffi_export_present": "reta_architecture_table_view_style_composition_json" in ffi.read_text(encoding="utf-8"),
        "inspect_binary_present": "rreta_arch_style_composition" in cargo.read_text(encoding="utf-8") and binary.exists(),
    }
    result = {
        "stage": 34,
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "invariant": "HTML catalog attributes and legacy generateCell wrappers compose into one <td> opening tag without losing class/style witnesses; default plain output remains unchanged unless both policies are enabled.",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    if result["status"] != "ok":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
