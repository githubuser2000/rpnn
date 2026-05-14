#!/usr/bin/env python3
"""Stage-29 static probe for the Rust table-view numbering projection."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

def moon_number(num: int):
    if num <= 2:
        return [], []
    results, exponents = [], []
    for exponent in range(2, num):
        one = num ** (1.0 / exponent)
        if round(round(one) * 100000) == round(one * 100000):
            results.append(round(one))
            exponents.append(exponent - 2)
    return results, exponents

def legacy_zaehlung_map(max_row: int):
    out = {}
    is_moon = True
    zaehlung = 0
    for row in range(1, max_row + 1):
        was_moon = is_moon
        is_moon = bool(moon_number(row)[0])
        if was_moon and not is_moon:
            zaehlung += 1
        out[row] = zaehlung
    return out

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    numbering_rs = ROOT / "crates/reta_architecture/src/table_view_numbering.rs"
    output_rs = ROOT / "crates/reta_architecture/src/table_view_output.rs"
    lib_rs = ROOT / "crates/reta_architecture/src/lib.rs"
    facade_rs = ROOT / "crates/reta_architecture/src/facade.rs"
    ffi_rs = ROOT / "src/ffi.rs"
    cargo = ROOT / "Cargo.toml"
    probe = {
        "stage": 29,
        "numbering_module_present": numbering_rs.exists(),
        "numbering_mode_enum_present": "TableViewNumberingMode" in numbering_rs.read_text(encoding="utf-8"),
        "legacy_pair_present": "LegacyPair" in numbering_rs.read_text(encoding="utf-8"),
        "legacy_zaehlung_function_present": "legacy_zaehlung_map" in numbering_rs.read_text(encoding="utf-8"),
        "table_view_output_uses_numbering": "numbering_values_for_source_row" in output_rs.read_text(encoding="utf-8"),
        "lib_exports_numbering": "pub mod table_view_numbering" in lib_rs.read_text(encoding="utf-8"),
        "facade_tracks_numbering": "table_view_numbering" in facade_rs.read_text(encoding="utf-8"),
        "ffi_export_present": "reta_architecture_table_view_numbering_json" in ffi_rs.read_text(encoding="utf-8"),
        "inspect_binary_registered": "rreta_arch_numbering" in cargo.read_text(encoding="utf-8"),
    }
    zmap = legacy_zaehlung_map(12)
    probe["zaehlung_sample"] = [[row, zmap[row]] for row in range(1, 13)]
    probe["initial_intervals_match_python_docs"] = (
        zmap[1] == 1 and zmap[4] == 1 and zmap[5] == 2 and zmap[9] == 2 and zmap[10] == 3
    )
    probe["legacy_pair_headers"] = ["Zählung", "Nummerierung"]
    probe["status"] = "ok" if all(
        value for key, value in probe.items()
        if key not in {"stage", "zaehlung_sample", "legacy_pair_headers", "status"}
    ) else "failed"
    print(json.dumps(probe, ensure_ascii=False, indent=2 if args.pretty else None))

if __name__ == "__main__":
    main()
