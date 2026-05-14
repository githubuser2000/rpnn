#!/usr/bin/env python3
"""Probe the generated Rust html-class catalog against htmlclassesPy.jsonl.

This is dependency-free and intentionally works before a full Rust build.  It
checks the generator output and the important column-744 witness.
"""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "python_arch_reference" / "htmlclassesPy.jsonl"
RUST = ROOT / "crates" / "reta_architecture" / "src" / "html_class_catalog.rs"


def source_stats() -> dict[str, object]:
    records = []
    with SOURCE.open("r", encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if line:
                records.append(json.loads(line))
    col744 = [record for record in records if record.get("column_number") == 744]
    return {
        "record_count": len(records),
        "unique_column_count": len({record.get("column_number") for record in records}),
        "text_record_count": sum(1 for record in records if record.get("text")),
        "class_record_count": sum(1 for record in records if record.get("class_string")),
        "column_744_record_count": len(col744),
        "column_744_texts": [record.get("text") for record in col744],
    }


def rust_stats() -> dict[str, object]:
    text = RUST.read_text(encoding="utf-8")
    return {
        "record_count": len(re.findall(r"^    HtmlClassRecord \{", text, flags=re.MULTILINE)),
        "has_bootstrap": "bootstrap_html_class_catalog" in text,
        "has_owned_record": "OwnedHtmlClassRecord" in text,
        "has_column_744": "column_number: 744" in text,
        "has_column_744_text_question": 'text: "?"' in text,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    src = source_stats()
    rust = rust_stats()
    mismatches = []
    if src["record_count"] != rust["record_count"]:
        mismatches.append({"field": "record_count", "source": src["record_count"], "rust": rust["record_count"]})
    if not rust["has_bootstrap"]:
        mismatches.append({"field": "bootstrap", "rust": False})
    if not rust["has_owned_record"]:
        mismatches.append({"field": "owned_record", "rust": False})
    if not rust["has_column_744"] or not rust["has_column_744_text_question"]:
        mismatches.append({"field": "column_744_witness", "rust": rust})

    result = {
        "source": src,
        "rust": rust,
        "mismatches": mismatches,
        "status": "ok" if not mismatches else "mismatch",
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None))


if __name__ == "__main__":
    main()
