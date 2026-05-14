#!/usr/bin/env python3
"""Generate the Rust html-class catalog from python_arch_reference/htmlclassesPy.jsonl.

The generated file deliberately keeps static borrowed records Serialize-only.
Owned records are exposed separately for JSON/FFI round-tripping.
"""
from __future__ import annotations

import json
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "python_arch_reference" / "htmlclassesPy.jsonl"
TARGET = ROOT / "crates" / "reta_architecture" / "src" / "html_class_catalog.rs"


def rust_string(value: Any) -> str:
    if value is None:
        value = ""
    text = str(value)
    # JSON escaping with UTF-8 text maps cleanly to Rust string escapes for the
    # data present in htmlclassesPy.jsonl while keeping umlauts readable.
    return json.dumps(text, ensure_ascii=False)


def rust_option_i64(value: Any) -> str:
    return "None" if value is None else f"Some({int(value)})"


def load_records() -> list[dict[str, Any]]:
    records: list[dict[str, Any]] = []
    with SOURCE.open("r", encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if not line:
                continue
            obj = json.loads(line)
            records.append(
                {
                    "column_number": int(obj.get("column_number", 0)),
                    "row_number": obj.get("row_number"),
                    "tag": obj.get("tag", ""),
                    "class_string": obj.get("class_string", ""),
                    "text": obj.get("text", ""),
                    "raw_open_tag": obj.get("raw_open_tag", ""),
                    "raw_html": obj.get("raw_html", ""),
                }
            )
    return records


def main() -> None:
    records = load_records()
    unique_columns = sorted({record["column_number"] for record in records})
    with_text = sum(1 for record in records if record["text"])
    with_classes = sum(1 for record in records if record["class_string"])

    lines: list[str] = []
    lines.append("//! Generated HTML-class catalog extracted from `python_arch_reference/htmlclassesPy.jsonl`.")
    lines.append("//!")
    lines.append("//! Stage 21 makes the HTML/class witness layer visible to Rust.  The records")
    lines.append("//! are static and Serialize-only; owned records are available for JSON/FFI")
    lines.append("//! round-tripping without impossible `'static` deserialization bounds.")
    lines.append("")
    lines.append("use serde::{Deserialize, Serialize};")
    lines.append("")
    lines.append("#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]")
    lines.append("pub struct HtmlClassRecord {")
    lines.append("    pub column_number: i64,")
    lines.append("    pub row_number: Option<i64>,")
    lines.append("    pub tag: &'static str,")
    lines.append("    pub class_string: &'static str,")
    lines.append("    pub text: &'static str,")
    lines.append("    pub raw_open_tag: &'static str,")
    lines.append("    pub raw_html: &'static str,")
    lines.append("}")
    lines.append("")
    lines.append("impl HtmlClassRecord {")
    lines.append("    pub fn owned(self) -> OwnedHtmlClassRecord {")
    lines.append("        OwnedHtmlClassRecord {")
    lines.append("            column_number: self.column_number,")
    lines.append("            row_number: self.row_number,")
    lines.append("            tag: self.tag.to_string(),")
    lines.append("            class_string: self.class_string.to_string(),")
    lines.append("            text: self.text.to_string(),")
    lines.append("            raw_open_tag: self.raw_open_tag.to_string(),")
    lines.append("            raw_html: self.raw_html.to_string(),")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]")
    lines.append("pub struct OwnedHtmlClassRecord {")
    lines.append("    pub column_number: i64,")
    lines.append("    pub row_number: Option<i64>,")
    lines.append("    pub tag: String,")
    lines.append("    pub class_string: String,")
    lines.append("    pub text: String,")
    lines.append("    pub raw_open_tag: String,")
    lines.append("    pub raw_html: String,")
    lines.append("}")
    lines.append("")
    lines.append("#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]")
    lines.append("pub struct HtmlClassCatalogSnapshot {")
    lines.append("    pub class: String,")
    lines.append("    pub record_count: usize,")
    lines.append("    pub unique_column_count: usize,")
    lines.append("    pub text_record_count: usize,")
    lines.append("    pub class_record_count: usize,")
    lines.append("    pub column_744_record_count: usize,")
    lines.append("    pub column_744_text: Option<String>,")
    lines.append("}")
    lines.append("")
    lines.append("#[derive(Clone, Debug, Eq, PartialEq, Serialize)]")
    lines.append("pub struct HtmlClassCatalogBundle {")
    lines.append("    pub records: Vec<HtmlClassRecord>,")
    lines.append("}")
    lines.append("")
    lines.append("impl HtmlClassCatalogBundle {")
    lines.append("    pub fn snapshot(&self) -> HtmlClassCatalogSnapshot {")
    lines.append("        html_class_catalog_snapshot()")
    lines.append("    }")
    lines.append("    pub fn owned_records(&self) -> Vec<OwnedHtmlClassRecord> {")
    lines.append("        self.records.iter().copied().map(HtmlClassRecord::owned).collect()")
    lines.append("    }")
    lines.append("    pub fn records_for_column(&self, column_number: i64) -> Vec<HtmlClassRecord> {")
    lines.append("        html_class_records_for_column(column_number)")
    lines.append("    }")
    lines.append("    pub fn record(&self, column_number: i64, row_number: Option<i64>) -> Option<HtmlClassRecord> {")
    lines.append("        html_class_record(column_number, row_number)")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("pub const HTML_CLASS_RECORDS: &[HtmlClassRecord] = &[")
    for record in records:
        lines.append("    HtmlClassRecord {")
        lines.append(f"        column_number: {record['column_number']},")
        lines.append(f"        row_number: {rust_option_i64(record['row_number'])},")
        lines.append(f"        tag: {rust_string(record['tag'])},")
        lines.append(f"        class_string: {rust_string(record['class_string'])},")
        lines.append(f"        text: {rust_string(record['text'])},")
        lines.append(f"        raw_open_tag: {rust_string(record['raw_open_tag'])},")
        lines.append(f"        raw_html: {rust_string(record['raw_html'])},")
        lines.append("    },")
    lines.append("];")
    lines.append("")
    lines.append("pub fn bootstrap_html_class_catalog() -> HtmlClassCatalogBundle {")
    lines.append("    HtmlClassCatalogBundle { records: HTML_CLASS_RECORDS.to_vec() }")
    lines.append("}")
    lines.append("")
    lines.append("pub fn html_class_record_count() -> usize { HTML_CLASS_RECORDS.len() }")
    lines.append(f"pub fn html_class_unique_column_count() -> usize {{ {len(unique_columns)} }}")
    lines.append(f"pub fn html_class_text_record_count() -> usize {{ {with_text} }}")
    lines.append(f"pub fn html_class_class_record_count() -> usize {{ {with_classes} }}")
    lines.append("")
    lines.append("pub fn html_class_records_for_column(column_number: i64) -> Vec<HtmlClassRecord> {")
    lines.append("    HTML_CLASS_RECORDS.iter().copied().filter(|record| record.column_number == column_number).collect()")
    lines.append("}")
    lines.append("")
    lines.append("pub fn html_class_record(column_number: i64, row_number: Option<i64>) -> Option<HtmlClassRecord> {")
    lines.append("    HTML_CLASS_RECORDS.iter().copied().find(|record| record.column_number == column_number && record.row_number == row_number)")
    lines.append("}")
    lines.append("")
    lines.append("pub fn html_class_text_for_column_row(column_number: i64, row_number: Option<i64>) -> Option<&'static str> {")
    lines.append("    html_class_record(column_number, row_number).map(|record| record.text)")
    lines.append("}")
    lines.append("")
    lines.append("pub fn html_class_owned_records() -> Vec<OwnedHtmlClassRecord> {")
    lines.append("    HTML_CLASS_RECORDS.iter().copied().map(HtmlClassRecord::owned).collect()")
    lines.append("}")
    lines.append("")
    lines.append("pub fn html_class_catalog_snapshot() -> HtmlClassCatalogSnapshot {")
    lines.append("    HtmlClassCatalogSnapshot {")
    lines.append("        class: \"HtmlClassCatalogSnapshot\".to_string(),")
    lines.append("        record_count: html_class_record_count(),")
    lines.append("        unique_column_count: html_class_unique_column_count(),")
    lines.append("        text_record_count: html_class_text_record_count(),")
    lines.append("        class_record_count: html_class_class_record_count(),")
    lines.append("        column_744_record_count: html_class_records_for_column(744).len(),")
    lines.append("        column_744_text: html_class_text_for_column_row(744, None).map(str::to_string),")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("#[cfg(test)]")
    lines.append("mod tests {")
    lines.append("    use super::*;")
    lines.append("    #[test]")
    lines.append("    fn catalog_contains_744_witness() {")
    lines.append("        let record = html_class_record(744, None).expect(\"column 744 witness\");")
    lines.append("        assert_eq!(record.text, \"?\");")
    lines.append("    }")
    lines.append("    #[test]")
    lines.append("    fn owned_records_are_available_for_json_round_trips() {")
    lines.append("        let owned = html_class_owned_records();")
    lines.append("        assert_eq!(owned.len(), HTML_CLASS_RECORDS.len());")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    TARGET.write_text("\n".join(lines), encoding="utf-8")

    stats = {
        "source": str(SOURCE.relative_to(ROOT)),
        "target": str(TARGET.relative_to(ROOT)),
        "record_count": len(records),
        "unique_column_count": len(unique_columns),
        "text_record_count": with_text,
        "class_record_count": with_classes,
        "column_744_records": sum(1 for record in records if record["column_number"] == 744),
    }
    print(json.dumps(stats, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
