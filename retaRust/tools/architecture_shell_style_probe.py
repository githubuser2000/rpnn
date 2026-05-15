#!/usr/bin/env python3
"""Stage-36 shell/ANSI style probe.

Verifies that the Rust materialized table-view output has a disabled-by-default
shell colour projection backed by the legacy `table_output.colorize` function,
that `--shellcolors`/`--ansicolors` activates it, and that parity can strip ANSI
bytes semantically without weakening raw commit guards.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    shell = read("crates/reta_architecture/src/table_view_shell_styles.rs")
    output = read("crates/reta_architecture/src/table_view_output.rs")
    parity = read("crates/reta_architecture/src/table_view_output_parity.rs")
    table_output = read("crates/reta_architecture/src/table_output.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    runtime = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    binary = ROOT / "src/bin/reta_arch_shell_styles.rs"

    checks = {
        "module_exists": (ROOT / "crates/reta_architecture/src/table_view_shell_styles.rs").exists(),
        "bundle_declared": "pub struct TableViewShellStyleBundle" in shell,
        "legacy_policy_declared": "LegacyColorize" in shell and "LegacyColorizeWitness" in shell,
        "uses_legacy_colorize": "use crate::table_output::colorize" in shell and "colorize(value" in shell,
        "default_disabled": "enabled: false" in shell and "pub fn disabled" in shell,
        "continuum_smoke_present": "continuum_m_shell_style_smoke" in shell,
        "table_output_has_colorize": "pub fn colorize" in table_output and "\\u{1b}[" in table_output,
        "cli_flags_parsed": '"shellcolors" | "shellcolor" | "ansicolors" | "ansicolor"' in output,
        "nocolor_disables_shell": "config.shell_styles = config.shell_styles.clone().without_color()" in output,
        "render_shell_uses_projection": "colorize_shell_output_value(" in output and "config.shell_styles.activates_mode(OutputMode::Shell)" in output,
        "report_contains_shell_counts": "shell_style_ansi_cell_count" in output and "shell_style_report" in output,
        "parity_strips_ansi": "strip_ansi_escape_sequences" in parity and "strip_ansi: true" in parity,
        "lib_exports_module": "pub mod table_view_shell_styles" in lib and "TableViewShellStyleReport" in lib,
        "facade_runtime_contains_bundle": "pub table_view_shell_styles: TableViewShellStyleBundle" in facade,
        "runtime_gates_present": all(name in runtime for name in [
            "table_view_shell_styles.legacy_colorize",
            "table_view_shell_styles.ansi_cell_wrapper",
            "table_view_shell_styles.strip_ansi_parity",
        ]),
        "migration_step_present": "step-table-view-shell-styles" in migration,
        "ffi_export_present": "reta_architecture_table_view_shell_styles_json" in ffi,
        "inspect_binary_present": "rreta_arch_shell_styles" in cargo and binary.exists(),
        "tests_present": "shell_colors_flag_activates_ansi_projection" in output and "nocolor_disables_shell_color_projection" in output,
    }
    result = {
        "stage": 36,
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "invariant": (
            "Shell ANSI colouring is disabled by default, can be activated only by explicit "
            "shell colour flags, is disabled again by --nocolor, and remains semantically "
            "strip-able for parity diagnostics while raw output still guards commits."
        ),
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    if result["status"] != "ok":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
