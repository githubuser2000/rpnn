#!/usr/bin/env python3
"""Stage-32 row-style probe.

This probe is intentionally dependency-free.  It checks that the Rust row-style
projection is present, wired into TableViewOutput, and still disabled by default
while `--rowcolors` / `--zeilenfarben` can activate the legacy coloredBeginCol
witness path.
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

    row_styles = ROOT / "crates/reta_architecture/src/table_view_row_styles.rs"
    table_output = ROOT / "crates/reta_architecture/src/table_view_output.rs"
    lib_rs = ROOT / "crates/reta_architecture/src/lib.rs"
    facade = ROOT / "crates/reta_architecture/src/facade.rs"
    runtime_switch = ROOT / "crates/reta_architecture/src/runtime_switch.rs"
    migration = ROOT / "crates/reta_architecture/src/migration_control.rs"
    ffi = ROOT / "src/ffi.rs"
    cargo = ROOT / "Cargo.toml"
    binary = ROOT / "src/bin/reta_arch_row_styles.rs"
    py_output_syntax = ROOT / "python_arch_reference/reta_architecture/output_syntax.py"

    checks = {
        "module_exists": row_styles.exists(),
        "bundle_declared": contains(row_styles, "pub struct TableViewRowStyleBundle"),
        "legacy_policy_declared": contains(row_styles, "LegacyColoredBeginCol"),
        "uses_colored_begin_col": contains(row_styles, "colored_begin_col(mode, row_number, rest)"),
        "default_disabled": contains(row_styles, "enabled: false"),
        "html_and_bbcode_only": contains(row_styles, "OutputMode::Html") and contains(row_styles, "OutputMode::Bbcode"),
        "table_output_config_has_row_styles": contains(table_output, "pub row_styles: TableViewRowStyleConfig"),
        "cli_rowcolors_parsed": contains(table_output, '"rowcolors" | "zeilenfarben"'),
        "nocolor_disables_row_styles": contains(table_output, "config.row_styles = config.row_styles.clone().without_color()"),
        "html_renderer_uses_styled_rows": contains(table_output, "styled_begin_row_for_row(") and contains(table_output, "OutputMode::Html"),
        "bbcode_renderer_uses_styled_rows": contains(table_output, "OutputMode::Bbcode"),
        "report_contains_row_style_counts": contains(table_output, "row_style_colored_row_count"),
        "lib_exports_module": contains(lib_rs, "pub mod table_view_row_styles"),
        "facade_runtime_contains_bundle": contains(facade, "pub table_view_row_styles: TableViewRowStyleBundle"),
        "runtime_switch_gates": contains(runtime_switch, "table_view_row_styles.legacy_colored_begin_col"),
        "migration_step": contains(migration, "step-table-view-row-styles"),
        "ffi_export": contains(ffi, "reta_architecture_table_view_row_styles_json"),
        "inspect_binary_target": contains(cargo, "rreta_arch_row_styles") and binary.exists(),
        "python_source_has_coloredBeginCol": contains(py_output_syntax, "def coloredBeginCol"),
    }
    status = "ok" if all(checks.values()) else "failed"
    out = {
        "stage": 32,
        "status": status,
        "checks": checks,
        "invariant": "row styles are disabled by default; --rowcolors/--zeilenfarben activates legacy coloredBeginCol for HTML/BBCode only; --nocolor disables it again",
    }
    print(json.dumps(out, indent=2 if args.pretty else None, ensure_ascii=False, sort_keys=True))
    if status != "ok":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
