#!/usr/bin/env python3
"""Validate the generated Rust CSV catalog against python_arch_reference/csv.

This probe is intentionally dependency-free and can run before Cargo is
available.  It checks that every Python CSV file is represented in
`csv_catalog.rs` and that the generated row/max-column metadata matches a
fresh Python CSV parse.
"""
from __future__ import annotations

import argparse
import csv
import json
import re
from pathlib import Path


def sniff_delimiter(text: str) -> str:
    try:
        return csv.Sniffer().sniff(text[:8192], delimiters=";,\t|,").delimiter
    except Exception:
        first = text.splitlines()[0] if text.splitlines() else ""
        return max([";", ",", "\t", "|"], key=first.count)


def parse_rust_catalog(path: Path) -> dict[str, dict[str, int]]:
    text = path.read_text(encoding="utf-8")
    entries: dict[str, dict[str, int]] = {}
    pattern = re.compile(
        r'CsvAsset \{\s*name: "(?P<name>(?:\\.|[^"])*)",.*?row_count: (?P<rows>\d+),\s*max_columns: (?P<cols>\d+),.*?nonempty_cell_count: (?P<nonempty>\d+),',
        re.DOTALL,
    )
    for match in pattern.finditer(text):
        name = match.group("name").encode("utf-8").decode("unicode_escape")
        entries[name] = {
            "row_count": int(match.group("rows")),
            "max_columns": int(match.group("cols")),
            "nonempty_cell_count": int(match.group("nonempty")),
        }
    return entries


def fresh_csv_stats(csv_root: Path) -> dict[str, dict[str, int]]:
    out: dict[str, dict[str, int]] = {}
    for path in sorted(csv_root.glob("*.csv")):
        text = path.read_text(encoding="utf-8", errors="replace")
        delimiter = sniff_delimiter(text)
        rows = list(csv.reader(text.splitlines(), delimiter=delimiter))
        out[path.name] = {
            "row_count": len(rows),
            "max_columns": max((len(row) for row in rows), default=0),
            "nonempty_cell_count": sum(1 for row in rows for cell in row if cell.strip()),
        }
    return out


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=Path(__file__).resolve().parents[1], type=Path)
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    root = args.root
    rust_catalog = parse_rust_catalog(root / "crates" / "reta_architecture" / "src" / "csv_catalog.rs")
    fresh = fresh_csv_stats(root / "python_arch_reference" / "csv")

    missing_in_rust = sorted(set(fresh) - set(rust_catalog))
    extra_in_rust = sorted(set(rust_catalog) - set(fresh))
    mismatches = []
    for name in sorted(set(fresh) & set(rust_catalog)):
        if fresh[name] != rust_catalog[name]:
            mismatches.append({"name": name, "fresh": fresh[name], "rust": rust_catalog[name]})

    result = {
        "fresh_asset_count": len(fresh),
        "rust_asset_count": len(rust_catalog),
        "missing_in_rust": missing_in_rust,
        "extra_in_rust": extra_in_rust,
        "mismatches": mismatches,
        "status": "ok" if not missing_in_rust and not extra_in_rust and not mismatches else "mismatch",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None))
    return 0 if result["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
