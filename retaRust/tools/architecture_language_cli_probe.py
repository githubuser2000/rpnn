#!/usr/bin/env python3
"""Stage 57 probe: Python -language=... reaches Rust table materialization.

Stage 56 made language fallback column-aware.  Stage 57 makes the language
choice itself CLI-aware in the Rust architecture path, so `-language=english`
can select localized CSV sections when they are complete.  After the Stage-55 CSV sync, English also contains 744 directly.
"""
from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "crates" / "reta_architecture" / "src"
BIN = ROOT / "src" / "bin"
FFI = ROOT / "src" / "ffi.rs"


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace")


def csv_max_columns(path: Path) -> int:
    rows = list(csv.reader(path.read_text(encoding="utf-8", errors="replace").splitlines(), delimiter=";"))
    return max((len(row) for row in rows), default=0)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    csv_catalog = read(SRC / "csv_catalog.rs")
    generator = read(ROOT / "tools" / "generate_csv_catalog.py")
    mat = read(SRC / "table_materialization.rs")
    param = read(SRC / "parameter_runtime.rs")
    shadow = read(SRC / "shadow_pipeline.rs")
    facade = read(SRC / "facade.rs")
    ffi = read(FFI)
    view_bin = read(BIN / "reta_arch_view_output.rs")
    parity_bin = read(BIN / "reta_arch_view_output_parity.rs")

    csv_dir = ROOT / "python_arch_reference" / "csv"
    base_cols = csv_max_columns(csv_dir / "religion.csv")
    en_cols = csv_max_columns(csv_dir / "en-religion.csv")

    checks = {
        "csv_language_alias_parser_present": "from_language_value" in csv_catalog
        and "csv_language_from_cli_args" in csv_catalog
        and "language_value_from_cli_arg" in csv_catalog,
        "csv_generator_preserves_language_parser": "from_language_value" in generator
        and "csv_language_from_cli_args" in generator,
        "materialization_config_from_cli_present": "TableMaterializationConfig::from_cli_args" in mat
        and "table_materialization_config_from_cli_args" in mat,
        "materialization_config_uses_csv_language": "config.language = csv_language_from_cli_args(args)" in mat,
        "materialization_language_tests_present": "materialization_config_reads_python_language_parameter" in mat
        and "materialization_config_language_variant_keeps_744_direct_after_sync" in mat,
        "parameter_runtime_tracks_language": "selected_language: Option<CsvLanguage>" in param
        and "language_value_from_cli_arg(raw)" in param,
        "shadow_pipeline_uses_cli_materialization_config": "TableMaterializationConfig::from_cli_args(&cleaned_args)" in shadow,
        "facade_reports_materialized_language": "materialized_language" in facade
        and "TableMaterializationConfig::from_cli_args(&clean_args)" in facade,
        "ffi_uses_cli_materialization_config": "TableMaterializationConfig::from_cli_args(&args)" in ffi,
        "inspect_bins_use_cli_materialization_config": "TableMaterializationConfig::from_cli_args(&args)" in view_bin
        and "TableMaterializationConfig::from_cli_args(&reta_args)" in parity_bin,
        "base_religion_has_745_columns": base_cols >= 745,
        "en_religion_synced_745_columns": en_cols == 745,
    }
    report = {
        "status": "ok" if all(checks.values()) else "failed",
        "stage": 57,
        "base_religion_max_columns": base_cols,
        "en_religion_max_columns": en_cols,
        "checks": checks,
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None))
    raise SystemExit(0 if report["status"] == "ok" else 1)


if __name__ == "__main__":
    main()
