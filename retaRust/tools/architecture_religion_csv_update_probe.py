#!/usr/bin/env python3
"""Stage-55 probe for the updated base religion.csv.

The uploaded religion.csv adds the concrete `Neues M (13) Kontinuum` column at
legacy/source index 744.  Earlier stages treated 744 as a virtual/non-direct
witness because the base CSV had only 744 columns (0..743).  This probe checks
that the three Rust-project CSV mirrors and the generated Rust CSV catalog now
agree on the new direct column.
"""
from __future__ import annotations

import argparse
import csv
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CSV_MIRRORS = [
    ROOT / "csv" / "religion.csv",
    ROOT / "python_reference" / "csv" / "religion.csv",
    ROOT / "python_arch_reference" / "csv" / "religion.csv",
]

def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()

def read_rows(path: Path) -> list[list[str]]:
    with path.open(encoding="utf-8", newline="") as handle:
        return list(csv.reader(handle, delimiter=";"))

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    shas = {str(path.relative_to(ROOT)): sha256(path) for path in CSV_MIRRORS}
    rows = read_rows(CSV_MIRRORS[-1])
    max_columns = max(len(row) for row in rows)
    header_493 = rows[0][493] if len(rows[0]) > 493 else ""
    data_493 = rows[1][493] if len(rows) > 1 and len(rows[1]) > 493 else ""
    header_744 = rows[0][744] if len(rows[0]) > 744 else ""
    data_744 = rows[1][744] if len(rows) > 1 and len(rows[1]) > 744 else ""

    catalog = (ROOT / "crates/reta_architecture/src/csv_catalog.rs").read_text(encoding="utf-8")
    materialization = (ROOT / "crates/reta_architecture/src/table_materialization.rs").read_text(encoding="utf-8")
    table_view = (ROOT / "crates/reta_architecture/src/table_view.rs").read_text(encoding="utf-8")
    virtual_columns = (ROOT / "crates/reta_architecture/src/table_view_virtual_columns.rs").read_text(encoding="utf-8")
    virtual_parity = (ROOT / "crates/reta_architecture/src/table_view_virtual_parity.rs").read_text(encoding="utf-8")

    checks = {
        "three_csv_mirrors_exist": all(path.exists() for path in CSV_MIRRORS),
        "three_csv_mirrors_have_same_hash": len(set(shas.values())) == 1,
        "religion_rows_1025": len(rows) == 1025,
        "religion_max_columns_745": max_columns == 745,
        "column_493_still_m_kontinuum": "M Kontinuum" in header_493 and "Wege-Gabelung" in data_493,
        "column_744_now_direct_neues_m": "Neues M" in header_744 and "Identität" in data_744,
        "catalog_base_religion_max_columns_745": bool(re.search(r'name: "religion\.csv",.*?max_columns: 745,.*?header_columns: 745,', catalog, re.S)),
        "materialization_test_expects_direct_744": "continuum_m_materializes_493_and_direct_744_after_religion_csv_update" in materialization,
        "table_view_test_expects_direct_744": "explicit_spaltenreihenfolge_places_direct_744_before_493_after_csv_update" in table_view,
        "virtual_policy_has_generic_non_direct_999": "non_direct_999_virtual_column_policy_smoke" in virtual_columns and "999:untagged" in virtual_columns,
        "virtual_parity_tracks_direct_744": "continuum_m_direct_744_preserved" in virtual_parity,
    }
    result = {
        "stage": 55,
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "csv_hashes": shas,
        "row_count": len(rows),
        "max_columns": max_columns,
        "header_493": header_493,
        "header_744": header_744,
        "data_744_preview": data_744[:160],
        "invariant": "The uploaded religion.csv makes legacy/source column 744 direct; virtual-column policies remain available for genuinely non-direct columns such as 999.",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if result["status"] == "ok" else 1

if __name__ == "__main__":
    raise SystemExit(main())
