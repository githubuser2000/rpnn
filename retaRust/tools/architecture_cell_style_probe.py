#!/usr/bin/env python3
"""Stage-33 cell-style probe.

The probe is dependency-free and static by design.  It verifies that the Rust
cell-style projection is present, policy-controlled, wired into the HTML/BBCode
TableViewOutput renderers, and disabled by default unless explicit cell-style
flags are used.
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

    cell_styles = ROOT / "crates/reta_architecture/src/table_view_cell_styles.rs"
    table_output = ROOT / "crates/reta_architecture/src/table_view_output.rs"
    output_syntax = ROOT / "crates/reta_architecture/src/output_syntax.rs"
    lib_rs = ROOT / "crates/reta_architecture/src/lib.rs"
    facade = ROOT / "crates/reta_architecture/src/facade.rs"
    runtime_switch = ROOT / "crates/reta_architecture/src/runtime_switch.rs"
    migration = ROOT / "crates/reta_architecture/src/migration_control.rs"
    ffi = ROOT / "src/ffi.rs"
    cargo = ROOT / "Cargo.toml"
    binary = ROOT / "src/bin/reta_arch_cell_styles.rs"
    py_output_syntax = ROOT / "python_arch_reference/reta_architecture/output_syntax.py"

    checks = {
        "module_exists": cell_styles.exists(),
        "bundle_declared": contains(cell_styles, "pub struct TableViewCellStyleBundle"),
        "legacy_policy_declared": contains(cell_styles, "LegacyGenerateCell"),
        "witness_policy_declared": contains(cell_styles, "LegacyGenerateCellWitness"),
        "uses_generate_cell_begin": contains(cell_styles, "generate_cell_begin("),
        "default_disabled": contains(cell_styles, "enabled: false"),
        "html_and_bbcode_only": contains(cell_styles, "OutputMode::Html") and contains(cell_styles, "OutputMode::Bbcode"),
        "numbering_negative_columns": contains(cell_styles, "pseudo_column") and contains(cell_styles, "-2"),
        "tag_schema_witnesses": contains(cell_styles, "ordinary_tags_for_column"),
        "table_output_config_has_cell_styles": contains(table_output, "pub cell_styles: TableViewCellStyleConfig"),
        "cli_cellstyles_parsed": contains(table_output, '"cellstyles" | "cellstyle"'),
        "nocolor_disables_cell_styles": contains(table_output, "config.cell_styles = config.cell_styles.clone().without_color()"),
        "html_renderer_uses_styled_cells": contains(table_output, "styled_begin_cell_for_output_value(") and contains(table_output, "OutputMode::Html"),
        "bbcode_renderer_uses_styled_cells": contains(table_output, "OutputMode::Bbcode") and contains(table_output, "styled_end_cell_for_mode(OutputMode::Bbcode)"),
        "report_contains_cell_style_counts": contains(table_output, "cell_style_styled_cell_count"),
        "output_syntax_has_generate_cell": contains(output_syntax, "pub fn generate_cell_begin"),
        "lib_exports_module": contains(lib_rs, "pub mod table_view_cell_styles"),
        "facade_runtime_contains_bundle": contains(facade, "pub table_view_cell_styles: TableViewCellStyleBundle"),
        "runtime_switch_gates": contains(runtime_switch, "table_view_cell_styles.legacy_generate_cell"),
        "migration_step": contains(migration, "step-table-view-cell-styles"),
        "ffi_export": contains(ffi, "reta_architecture_table_view_cell_styles_json"),
        "inspect_binary_target": contains(cargo, "rreta_arch_cell_styles") and binary.exists(),
        "python_source_has_generateCell": contains(py_output_syntax, "def generateCell"),
    }
    status = "ok" if all(checks.values()) else "failed"
    out = {
        "stage": 33,
        "status": status,
        "checks": checks,
        "invariant": "cell styles are disabled by default; --cellstyles/--cellstylewitness activates legacy generateCell wrappers for HTML/BBCode only; --nocolor disables them again",
    }
    print(json.dumps(out, indent=2 if args.pretty else None, ensure_ascii=False, sort_keys=True))
    if status != "ok":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
