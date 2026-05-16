#!/usr/bin/env python3
"""Probe CSV table materialization invariants.

The tool is intentionally dependency-free.  Without a built Rust binary it
checks the source-of-truth CSV, the generated Rust parameter matrix and the
Stage-55 direct-744 CSV materialization path.  With a `--materialize-bin` argument it
can also compare the runtime JSON emitted by `rreta_arch_materialize`.
"""
from __future__ import annotations

import argparse
import csv
import json
import pathlib
import re
import subprocess
import sys
from typing import Any

SMOKE_ARGS = [
    "reta",
    "-zeilen",
    "--vorhervonausschnitt=1-1",
    "-spalten",
    "--kontinuum=m",
    "--breite=0",
]


def read_religion(root: pathlib.Path) -> list[list[str]]:
    path = root / "python_arch_reference" / "csv" / "religion.csv"
    with path.open("r", encoding="utf-8", newline="") as handle:
        return list(csv.reader(handle, delimiter=";"))


def matrix_contains_744(root: pathlib.Path) -> bool:
    matrix = root / "crates" / "reta_architecture" / "src" / "parameter_matrix.rs"
    text = matrix.read_text(encoding="utf-8")
    return bool(
        re.search(r'main_aliases:\s*&\["Kontinuum", "kontinuum"\].*?parameter_aliases:\s*&\["M", "m", "Dreizehn"\].*?columns:\s*&\[493, 744\]', text, re.S)
    )




def virtual_column_path_present(root: pathlib.Path) -> bool:
    table_materialization = root / "crates" / "reta_architecture" / "src" / "table_materialization.rs"
    text = table_materialization.read_text(encoding="utf-8")
    return all(
        needle in text
        for needle in [
            "VirtualColumnMaterialization",
            "materialize_virtual_columns",
            "html_class_text",
            "ordinary_tags_for_column",
            "continuum_m_virtual_column_present",
        ]
    )

def source_probe(root: pathlib.Path) -> dict[str, Any]:
    rows = read_religion(root)
    header = rows[0]
    first_data = rows[1]
    selected_columns = [493, 744]
    header_preview = [header[column] for column in selected_columns if column < len(header)]
    first_data_preview = [first_data[column] for column in selected_columns if column < len(first_data)]
    return {
        "selected_columns_legacy": selected_columns,
        "religion_row_count": len(rows),
        "religion_max_columns": max(len(row) for row in rows),
        "matrix_contains_kontinuum_m_744": matrix_contains_744(root),
        "header_preview": header_preview,
        "first_data_preview": first_data_preview,
        "missing_legacy_columns": [column for column in selected_columns if column >= len(header)],
        "header_contains_m_kontinuum": any("M Kontinuum" in cell for cell in header_preview),
        "header_contains_neues_m": any("Neues M" in cell for cell in header_preview),
        "first_data_contains_weges_gabelung": any("Wege-Gabelung" in cell for cell in first_data_preview),
        "first_data_contains_identitaet": any("Identität" in cell for cell in first_data_preview),
        "column_744_directly_addressable": 744 < len(header),
        "virtual_column_path_present": virtual_column_path_present(root),
    }


def binary_probe(root: pathlib.Path, materialize_bin: pathlib.Path | None) -> dict[str, Any] | None:
    if materialize_bin is None:
        return None
    command = [str(materialize_bin), *SMOKE_ARGS]
    completed = subprocess.run(command, cwd=root, text=True, capture_output=True, check=False)
    result: dict[str, Any] = {
        "command": command,
        "returncode": completed.returncode,
        "stderr": completed.stderr,
    }
    if completed.stdout.strip():
        try:
            result["json"] = json.loads(completed.stdout)
        except json.JSONDecodeError as exc:
            result["json_error"] = str(exc)
            result["stdout_prefix"] = completed.stdout[:1000]
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=pathlib.Path, default=pathlib.Path.cwd())
    parser.add_argument("--materialize-bin", type=pathlib.Path)
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    root = args.root.resolve()
    source = source_probe(root)
    binary = binary_probe(root, args.materialize_bin)

    status = "ok"
    problems: list[str] = []
    if not source["matrix_contains_kontinuum_m_744"]:
        problems.append("parameter_matrix_missing_kontinuum_m_744")
    if not source["header_contains_m_kontinuum"]:
        problems.append("religion_header_missing_m_kontinuum")
    if not source["first_data_contains_weges_gabelung"]:
        problems.append("religion_first_data_missing_wege_gabelung")
    if not source["header_contains_neues_m"]:
        problems.append("religion_header_missing_neues_m")
    if not source["first_data_contains_identitaet"]:
        problems.append("religion_first_data_missing_identitaet")
    if not source["column_744_directly_addressable"]:
        problems.append("religion_744_not_direct_after_update")
    if not source["virtual_column_path_present"]:
        problems.append("virtual_column_path_missing")
    if binary is not None:
        if binary.get("returncode") != 0:
            problems.append("materialize_binary_nonzero")
        report = binary.get("json") or {}
        if report and not report.get("continuum_m_columns_present"):
            problems.append("materialize_binary_missing_continuum_m")
        if report and report.get("continuum_m_virtual_column_present"):
            problems.append("materialize_binary_still_treats_744_as_virtual")
        if report and 744 in report.get("continuum_m_missing_columns", []):
            problems.append("materialize_binary_still_missing_direct_744")
    if problems:
        status = "failed"

    output = {
        "status": status,
        "problems": problems,
        "smoke_args": SMOKE_ARGS,
        "source": source,
        "binary": binary,
    }
    print(json.dumps(output, ensure_ascii=False, indent=2 if args.pretty else None))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
