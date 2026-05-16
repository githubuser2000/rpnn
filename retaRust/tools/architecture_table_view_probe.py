#!/usr/bin/env python3
"""Stage-22 probe for the renderable table-view layer.

This is intentionally dependency-free and does not require a Rust build.  It
checks the same data witnesses used by `table_view.rs`: the M-continuum direct
CSV column and, after the Stage-55 religion.csv update, the now-direct 744
`Neues M` column. Virtual-column policy remains available for other non-direct columns.
"""
from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def load_religion() -> list[list[str]]:
    with (ROOT / "python_arch_reference" / "csv" / "religion.csv").open(
        encoding="utf-8", newline=""
    ) as handle:
        return list(csv.reader(handle, delimiter=";"))


def load_html_744() -> dict[str, object] | None:
    path = ROOT / "python_arch_reference" / "htmlclassesPy.jsonl"
    with path.open(encoding="utf-8") as handle:
        for line in handle:
            if not line.strip():
                continue
            record = json.loads(line)
            if record.get("column_number") == 744:
                return record
    return None


def rust_source_contains_table_view_hooks() -> dict[str, bool]:
    files = {
        "lib": ROOT / "crates" / "reta_architecture" / "src" / "lib.rs",
        "facade": ROOT / "crates" / "reta_architecture" / "src" / "facade.rs",
        "shadow": ROOT / "crates" / "reta_architecture" / "src" / "shadow_pipeline.rs",
        "ffi": ROOT / "src" / "ffi.rs",
        "cargo": ROOT / "Cargo.toml",
    }
    text = {name: path.read_text(encoding="utf-8") for name, path in files.items()}
    return {
        "module_declared": "pub mod table_view;" in text["lib"],
        "exports_declared": "pub use table_view" in text["lib"],
        "facade_runtime_field": "pub table_view: TableViewBundle" in text["facade"],
        "snapshot_counts": "rust_table_view_morphism_count" in text["facade"],
        "shadow_cli_plan_contains_view": "pub table_view: MaterializedTableView" in text["shadow"],
        "ffi_export": "reta_architecture_table_view_json" in text["ffi"],
        "binary_declared": "rreta_arch_view" in text["cargo"],
    }


def build_probe() -> dict[str, object]:
    rows = load_religion()
    html_744 = load_html_744()
    header_493 = rows[0][493]
    data_493 = rows[1][493]
    header_744 = rows[0][744] if len(rows[0]) > 744 else ""
    data_744 = rows[1][744] if len(rows[1]) > 744 else ""
    source_max_columns = max(len(row) for row in rows)
    direct_744 = 744 < source_max_columns
    direct_default_rendered = [header_493, header_744, data_493, data_744]
    tag_summary_virtual = "999:untagged"
    hooks = rust_source_contains_table_view_hooks()
    status = "ok" if (
        "M Kontinuum" in header_493
        and "Wege-Gabelung" in data_493
        and "Neues M" in header_744
        and "Identität" in data_744
        and html_744 is not None
        and direct_744
        and all(hooks.values())
    ) else "mismatch"
    return {
        "status": status,
        "source_max_columns": source_max_columns,
        "direct_column_493_header": header_493,
        "direct_column_493_first_data_contains_wege_gabelung": "Wege-Gabelung" in data_493,
        "direct_column_744_header": header_744,
        "direct_column_744_first_data_contains_identitaet": "Identität" in data_744,
        "column_744_directly_addressable": direct_744,
        "html_column_744_witness_text": None if html_744 is None else html_744.get("text"),
        "default_policy": "suppress",
        "default_rendered_cell_count": len(direct_default_rendered),
        "default_rendered_virtual_cell_count": 0,
        "tag_summary_policy_non_direct_cell": tag_summary_virtual,
        "rust_hooks": hooks,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()
    probe = build_probe()
    print(json.dumps(probe, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    if probe["status"] != "ok":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
