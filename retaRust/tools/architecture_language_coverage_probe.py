#!/usr/bin/env python3
"""Stage-60 smoke probe for language CSV coverage witnesses.

This intentionally stays dependency-free.  It checks the generated Rust source
and the CSV assets so the coverage layer can be verified even when cargo is not
available in the execution environment.
"""
from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def csv_shape(path: Path) -> tuple[int, int]:
    with path.open("r", encoding="utf-8", newline="") as handle:
        rows = list(csv.reader(handle, delimiter=";"))
    return len(rows), max((len(row) for row in rows), default=0)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    module = read("crates/reta_architecture/src/table_view_language_coverage.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    shadow = read("crates/reta_architecture/src/shadow_pipeline.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    workflow = read("src/reta_workflow_py.rs")
    runtime_switch = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")

    base_rows, base_cols = csv_shape(ROOT / "python_arch_reference/csv/religion.csv")
    en_rows, en_cols = csv_shape(ROOT / "python_arch_reference/csv/en-religion.csv")
    cn_rows, cn_cols = csv_shape(ROOT / "python_arch_reference/csv/cn-religion.csv")
    vn_rows, vn_cols = csv_shape(ROOT / "python_arch_reference/csv/vn-religion.csv")
    kr_rows, kr_cols = csv_shape(ROOT / "python_arch_reference/csv/kr-religion.csv")

    checks = {
        "module_exists": (ROOT / "crates/reta_architecture/src/table_view_language_coverage.rs").exists(),
        "coverage_report_declared": "TableViewLanguageCoverageReport" in module,
        "coverage_bundle_declared": "TableViewLanguageCoverageBundle" in module,
        "coverage_for_cli_declared": "language_coverage_for_cli_args" in module,
        "asset_coverage_declared": "LanguageAssetCoverage" in module,
        "fallback_disabled_guard_declared": "fallback_disabled_for_incomplete_requested_language_asset" in module,
        "continuum_smoke_declared": "continuum_m_language_coverage_smoke" in module,
        "lib_pub_mod": "pub mod table_view_language_coverage;" in lib,
        "lib_reexports": "TableViewLanguageCoverageReport" in lib and "language_coverage_for_cli_args" in lib,
        "shadow_report_has_coverage": "pub language_coverage: TableViewLanguageCoverageReport" in shadow,
        "shadow_cli_plan_has_coverage": "language_coverage," in shadow,
        "workflow_diagnostic": "ARCH_TABLE_VIEW_LANGUAGE_COVERAGE" in workflow,
        "ffi_export": "reta_architecture_table_view_language_coverage_json" in ffi,
        "inspect_binary_registered": "rreta_arch_language_coverage" in cargo,
        "runtime_gates": "table_view_language_coverage.translation_gap_report" in runtime_switch,
        "migration_step": "step-table-view-language-coverage" in migration,
        "base_religion_has_745_columns": base_cols == 745,
        "en_religion_still_stale_744_columns": en_cols == 744,
        "cn_religion_still_stale_744_columns": cn_cols == 744,
        "vn_religion_still_stale_744_columns": vn_cols == 744,
        "kr_religion_still_stale_744_columns": kr_cols == 744,
        "base_has_expected_rows": base_rows == 1025,
    }
    status = "ok" if all(checks.values()) else "failed"
    payload = {
        "stage": 60,
        "status": status,
        "checks": checks,
        "csv_shapes": {
            "religion.csv": {"rows": base_rows, "columns": base_cols},
            "en-religion.csv": {"rows": en_rows, "columns": en_cols},
            "cn-religion.csv": {"rows": cn_rows, "columns": cn_cols},
            "vn-religion.csv": {"rows": vn_rows, "columns": vn_cols},
            "kr-religion.csv": {"rows": kr_rows, "columns": kr_cols},
        },
        "expected_language_gap": {
            "required_columns_for_kontinuum_m": [493, 744],
            "stale_languages_missing_744": ["en", "cn", "vn", "kr"],
            "safe_action": "fallback_to_base_religion_until_language_variants_are_extended_to_745_columns",
        },
    }
    print(json.dumps(payload, indent=2 if args.pretty else None, ensure_ascii=False))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
