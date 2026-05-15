#!/usr/bin/env python3
"""Stage-35 style-aware output parity probe.

Checks that HTML/BBCode style wrappers are normalized as diagnostic semantic
noise while raw line equality remains the only commit-safe criterion.
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

    parity = read("crates/reta_architecture/src/table_view_output_parity.rs")
    runtime = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    lib = read("crates/reta_architecture/src/lib.rs")
    binary = ROOT / "src/bin/reta_arch_style_parity.rs"

    checks = {
        "style_aware_config_fields": "style_aware_markup" in parity and "ignore_style_wrappers" in parity,
        "normalized_report_tracks_wrappers": "style_wrapper_line_count" in parity and "document_normalized" in parity,
        "document_parser_present": "pub fn parse_markup_document_rows" in parity and "parse_html_document_rows" in parity and "parse_bbcode_document_rows" in parity,
        "styled_bbcode_td_parser_present": "fn find_bbcode_open_tag" in parity and "[td=\"\"]A[/td]" in parity,
        "multiline_html_test_present": "multiline_html_cells_normalize_to_same_semantic_rows_as_compact_html" in parity,
        "raw_commit_guard_test_present": "style_aware_normalization_does_not_make_raw_commit_safe" in parity and "!report.is_commit_safe_raw()" in parity,
        "duplicate_cell_count_increment_removed": parity.count("cell_count += cells.len()") == 0,
        "lib_exports_document_parser": "parse_markup_document_rows" in lib,
        "runtime_gates_present": all(name in runtime for name in [
            "table_view_style_parity.markup_document_normalize",
            "table_view_style_parity.bbcode_styled_td",
            "table_view_style_parity.raw_commit_guard",
        ]),
        "migration_step_present": "step-table-view-style-parity" in migration,
        "ffi_export_present": "reta_architecture_table_view_style_parity_json" in ffi,
        "inspect_binary_present": "rreta_arch_style_parity" in cargo and binary.exists(),
    }

    result = {
        "stage": 35,
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "invariant": (
            "Styled HTML/BBCode wrappers, catalog classes, generateCell wrappers and multiline "
            "cell markup are semantically normalized for shadow diagnostics, but raw line equality "
            "is still required before any visible output commit."
        ),
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    if result["status"] != "ok":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
