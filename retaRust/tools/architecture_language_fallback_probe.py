#!/usr/bin/env python3
"""Probe Stage 56 language fallback for direct religion.csv column 744.

The base religion.csv has been updated to 746 columns, so the matrix projection
`--kontinuum=m -> 493,744` can be directly CSV-backed in the base language.
Language variants are now expected to be synchronized at 746 columns.  The Rust materializer must not
turn 744 back into a virtual column just because English/Chinese/Vietnamese/
Korean assets are stale; it should fall back to base religion.csv for that
projection while keeping localized assets for projections that they can satisfy.
"""
from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CSV = ROOT / "python_arch_reference" / "csv"
MAT = ROOT / "crates" / "reta_architecture" / "src" / "table_materialization.rs"
CAT = ROOT / "crates" / "reta_architecture" / "src" / "csv_catalog.rs"
GEN = ROOT / "tools" / "generate_csv_catalog.py"


def rows(path: Path) -> list[list[str]]:
    text = path.read_text(encoding="utf-8", errors="replace")
    return list(csv.reader(text.splitlines(), delimiter=";"))


def max_columns(path: Path) -> int:
    r = rows(path)
    return max((len(row) for row in r), default=0)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    base = CSV / "religion.csv"
    variants = [CSV / f"{prefix}-religion.csv" for prefix in ["en", "cn", "vn", "kr"]]
    base_rows = rows(base)
    base_max = max((len(row) for row in base_rows), default=0)
    variant_max = {path.name: max_columns(path) for path in variants if path.exists()}

    table_materialization = MAT.read_text(encoding="utf-8")
    csv_catalog = CAT.read_text(encoding="utf-8")
    generator = GEN.read_text(encoding="utf-8")

    checks = {
        "base_religion_has_direct_744": base_max >= 746,
        "base_744_header_is_neues_m": len(base_rows[0]) > 744 and "Neues M" in base_rows[0][744],
        "language_variants_synced_at_746_columns": all(value == 746 for value in variant_max.values()),
        "csv_catalog_has_required_column_helper": "csv_asset_for_language_with_required_columns" in csv_catalog,
        "generator_preserves_required_column_helper": "csv_asset_for_language_with_required_columns" in generator,
        "materialization_config_has_fallback_flag": "fallback_to_base_for_missing_language_columns" in table_materialization,
        "ordinary_materialization_uses_column_aware_language_asset": "asset_name_for_language_with_columns" in table_materialization,
        "english_744_synced_variant_test_present": "language_materialization_uses_synced_variant_for_direct_744" in table_materialization,
        "english_493_variant_test_present": "language_materialization_keeps_variant_when_direct_columns_exist" in table_materialization,
        "old_plain_ordinary_asset_call_removed": 'CsvProjectionRequest::for_asset(asset_name_for_language(\n            "religion.csv"' not in table_materialization,
    }

    report = {
        "status": "ok" if all(checks.values()) else "failed",
        "base_religion_max_columns": base_max,
        "variant_max_columns": variant_max,
        "base_493_header": base_rows[0][493] if base_max > 493 else None,
        "base_744_header": base_rows[0][744] if base_max > 744 else None,
        "checks": checks,
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None))
    raise SystemExit(0 if report["status"] == "ok" else 1)


if __name__ == "__main__":
    main()
