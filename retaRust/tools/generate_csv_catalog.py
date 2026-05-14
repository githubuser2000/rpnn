#!/usr/bin/env python3
"""Generate the Rust CSV asset catalog from python_arch_reference/csv.

The generated module intentionally does not depend on external crates.  It
captures the concrete CSV presheaf that py-reta-arch uses: filenames,
language variants, dialects, row/column statistics and static include_str!
accessors for the current CSV content.
"""
from __future__ import annotations

import csv
import json
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[1]
CSV_ROOT = ROOT / "python_arch_reference" / "csv"
OUT = ROOT / "crates" / "reta_architecture" / "src" / "csv_catalog.rs"

LANG_PREFIXES = {
    "en": "English",
    "cn": "Chinese",
    "vn": "Vietnamese",
    "kr": "Korean",
}

KIND_BY_BASE = {
    "2024-07-06-symbols-alt-ak-circle-sphere-etc.csv": "SymbolsAltCircleSphere",
    "dualism-trinities-etc.csv": "DualismTrinities",
    "gebrochen-rational-emotionen.csv": "GebrochenRationalEmotionen",
    "gebrochen-rational-galaxie.csv": "GebrochenRationalGalaxie",
    "gebrochen-rational-strukturgroesse.csv": "GebrochenRationalStrukturgroesse",
    "gebrochen-rational-universum.csv": "GebrochenRationalUniversum",
    "kombi-gedanken17-absichten13-bewusstsein15.csv": "KombiGedankenAbsichtenBewusstsein",
    "kombi-meta-systeme.csv": "KombiMetaSysteme",
    "kombi-meta.csv": "KombiMeta",
    "kombi-universelle-wirklichkeit.csv": "KombiUniverselleWirklichkeit",
    "kombi.csv": "Kombi",
    "kreisVomTyp18.csv": "KreisVomTyp18",
    "meaningOfLife.csv": "MeaningOfLife",
    "primenumbers.csv": "PrimeNumbers",
    "religion.csv": "Religion",
    "sunMoonEtc.csv": "SunMoonEtc",
    "thomas-decodedDekodiert-in-motives-purposesAbsichten.csv": "ThomasDecodedMotivesPurposes",
}

DELIM_VARIANT = {
    ";": "Semicolon",
    ",": "Comma",
    "\t": "Tab",
    "|": "Pipe",
}


def rust_str(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def detect_language_and_base(name: str) -> tuple[str, str]:
    for prefix, variant in LANG_PREFIXES.items():
        marker = f"{prefix}-"
        if name.startswith(marker):
            return variant, name[len(marker):]
    return "Base", name


def sniff_delimiter(text: str) -> str:
    sample = text[:8192]
    try:
        return csv.Sniffer().sniff(sample, delimiters=";,\t|,").delimiter
    except Exception:
        first = text.splitlines()[0] if text.splitlines() else ""
        return max([";", ",", "\t", "|"], key=first.count)


@dataclass(frozen=True)
class Asset:
    name: str
    base_name: str
    language: str
    kind: str
    delimiter: str
    rows: list[list[str]]
    byte_len: int

    @property
    def row_count(self) -> int:
        return len(self.rows)

    @property
    def max_columns(self) -> int:
        return max((len(row) for row in self.rows), default=0)

    @property
    def header_columns(self) -> int:
        return len(self.rows[0]) if self.rows else 0

    @property
    def nonempty_cell_count(self) -> int:
        return sum(1 for row in self.rows for cell in row if cell.strip())

    @property
    def header_preview(self) -> str:
        if not self.rows:
            return ""
        preview = " | ".join(self.rows[0][:6])
        return preview[:240]


def read_assets() -> list[Asset]:
    assets: list[Asset] = []
    for path in sorted(CSV_ROOT.glob("*.csv"), key=lambda p: p.name):
        text = path.read_text(encoding="utf-8", errors="replace")
        delim = sniff_delimiter(text)
        rows = list(csv.reader(text.splitlines(), delimiter=delim))
        language, base_name = detect_language_and_base(path.name)
        kind = KIND_BY_BASE.get(base_name, "Other")
        assets.append(
            Asset(
                name=path.name,
                base_name=base_name,
                language=language,
                kind=kind,
                delimiter=DELIM_VARIANT.get(delim, "Semicolon"),
                rows=rows,
                byte_len=len(text.encode("utf-8")),
            )
        )
    return assets


def enum_variants(values: Iterable[str]) -> str:
    return "\n".join(f"    {value}," for value in values)


def generate(assets: list[Asset]) -> str:
    languages = ["Base", "English", "Chinese", "Vietnamese", "Korean"]
    kinds = [
        "SymbolsAltCircleSphere",
        "DualismTrinities",
        "GebrochenRationalEmotionen",
        "GebrochenRationalGalaxie",
        "GebrochenRationalStrukturgroesse",
        "GebrochenRationalUniversum",
        "KombiGedankenAbsichtenBewusstsein",
        "KombiMetaSysteme",
        "KombiMeta",
        "KombiUniverselleWirklichkeit",
        "Kombi",
        "KreisVomTyp18",
        "MeaningOfLife",
        "PrimeNumbers",
        "Religion",
        "SunMoonEtc",
        "ThomasDecodedMotivesPurposes",
        "Other",
    ]
    delimiters = ["Semicolon", "Comma", "Tab", "Pipe"]

    asset_entries = []
    for asset in assets:
        asset_entries.append(
            "    CsvAsset {\n"
            f"        name: {rust_str(asset.name)},\n"
            f"        base_name: {rust_str(asset.base_name)},\n"
            f"        language: CsvLanguage::{asset.language},\n"
            f"        kind: CsvAssetKind::{asset.kind},\n"
            f"        delimiter: CsvDelimiter::{asset.delimiter},\n"
            f"        row_count: {asset.row_count},\n"
            f"        max_columns: {asset.max_columns},\n"
            f"        header_columns: {asset.header_columns},\n"
            f"        nonempty_cell_count: {asset.nonempty_cell_count},\n"
            f"        byte_len: {asset.byte_len},\n"
            f"        header_preview: {rust_str(asset.header_preview)},\n"
            "    },"
        )

    content_match = []
    for asset in assets:
        content_match.append(
            f"        {rust_str(asset.name)} => Some(include_str!(\"../../../python_arch_reference/csv/{asset.name}\")),"
        )

    lang_canonical = {
        "Base": "base",
        "English": "en",
        "Chinese": "cn",
        "Vietnamese": "vn",
        "Korean": "kr",
    }
    lang_match = "\n".join(
        f"            CsvLanguage::{variant} => \"{canonical}\"," for variant, canonical in lang_canonical.items()
    )
    kind_match = "\n".join(
        f"            CsvAssetKind::{variant} => \"{variant}\"," for variant in kinds
    )
    delim_match = "\n".join(
        [
            "            CsvDelimiter::Semicolon => ';',",
            "            CsvDelimiter::Comma => ',',",
            "            CsvDelimiter::Tab => '\\t',",
            "            CsvDelimiter::Pipe => '|',",
        ]
    )

    return f'''//! Generated CSV asset catalog extracted from `python_arch_reference/csv`.
//!
//! Stage 19 makes the concrete CSV presheaf visible to Rust.  This module
//! carries CSV metadata, static `include_str!` accessors and a small CSV parser
//! so concat/Kombi/religion tables can be inspected without falling back to the
//! old Python runtime.

use serde::{{Deserialize, Serialize}};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CsvLanguage {{
{enum_variants(languages)}
}}

impl CsvLanguage {{
    pub fn canonical(self) -> &'static str {{
        match self {{
{lang_match}
        }}
    }}
}}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CsvAssetKind {{
{enum_variants(kinds)}
}}

impl CsvAssetKind {{
    pub fn canonical(self) -> &'static str {{
        match self {{
{kind_match}
        }}
    }}
}}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CsvDelimiter {{
{enum_variants(delimiters)}
}}

impl CsvDelimiter {{
    pub fn as_char(self) -> char {{
        match self {{
{delim_match}
        }}
    }}
}}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct CsvAsset {{
    pub name: &'static str,
    pub base_name: &'static str,
    pub language: CsvLanguage,
    pub kind: CsvAssetKind,
    pub delimiter: CsvDelimiter,
    pub row_count: usize,
    pub max_columns: usize,
    pub header_columns: usize,
    pub nonempty_cell_count: usize,
    pub byte_len: usize,
    pub header_preview: &'static str,
}}

impl CsvAsset {{
    pub fn text(self) -> Option<&'static str> {{
        csv_text_by_name(self.name)
    }}

    pub fn rows(self) -> Vec<Vec<String>> {{
        self.text()
            .map(|text| parse_csv_text(text, self.delimiter))
            .unwrap_or_default()
    }}

    pub fn owned(self) -> OwnedCsvAsset {{
        OwnedCsvAsset {{
            name: self.name.to_string(),
            base_name: self.base_name.to_string(),
            language: self.language,
            kind: self.kind,
            delimiter: self.delimiter,
            row_count: self.row_count,
            max_columns: self.max_columns,
            header_columns: self.header_columns,
            nonempty_cell_count: self.nonempty_cell_count,
            byte_len: self.byte_len,
            header_preview: self.header_preview.to_string(),
        }}
    }}
}}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnedCsvAsset {{
    pub name: String,
    pub base_name: String,
    pub language: CsvLanguage,
    pub kind: CsvAssetKind,
    pub delimiter: CsvDelimiter,
    pub row_count: usize,
    pub max_columns: usize,
    pub header_columns: usize,
    pub nonempty_cell_count: usize,
    pub byte_len: usize,
    pub header_preview: String,
}}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnedCsvCatalogBundle {{
    pub assets: Vec<OwnedCsvAsset>,
}}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CsvCatalogBundle {{
    pub assets: Vec<CsvAsset>,
}}

impl CsvCatalogBundle {{
    pub fn snapshot(&self) -> CsvCatalogSnapshot {{
        CsvCatalogSnapshot {{
            class: "CsvCatalogBundle".to_string(),
            asset_count: self.assets.len(),
            base_asset_count: self.assets.iter().filter(|asset| asset.language == CsvLanguage::Base).count(),
            language_variant_count: self.assets.iter().filter(|asset| asset.language != CsvLanguage::Base).count(),
            total_row_count: self.assets.iter().map(|asset| asset.row_count).sum(),
            total_nonempty_cell_count: self.assets.iter().map(|asset| asset.nonempty_cell_count).sum(),
            semicolon_asset_count: self.assets.iter().filter(|asset| asset.delimiter == CsvDelimiter::Semicolon).count(),
            comma_asset_count: self.assets.iter().filter(|asset| asset.delimiter == CsvDelimiter::Comma).count(),
            religion_row_count: csv_asset_by_name("religion.csv").map(|asset| asset.row_count).unwrap_or(0),
            kombi_meta_row_count: csv_asset_by_name("kombi-meta.csv").map(|asset| asset.row_count).unwrap_or(0),
        }}
    }}

    pub fn owned(&self) -> OwnedCsvCatalogBundle {{
        OwnedCsvCatalogBundle {{
            assets: self.assets.iter().copied().map(CsvAsset::owned).collect(),
        }}
    }}

    pub fn by_name(&self, name: &str) -> Option<CsvAsset> {{
        self.assets.iter().copied().find(|asset| asset.name == name)
    }}

    pub fn by_kind(&self, kind: CsvAssetKind) -> Vec<CsvAsset> {{
        self.assets.iter().copied().filter(|asset| asset.kind == kind).collect()
    }}

    pub fn by_language(&self, language: CsvLanguage) -> Vec<CsvAsset> {{
        self.assets.iter().copied().filter(|asset| asset.language == language).collect()
    }}

    pub fn rows_by_name(&self, name: &str) -> Option<Vec<Vec<String>>> {{
        csv_rows_by_name(name)
    }}

    pub fn cell_by_name(&self, name: &str, row_one_based: usize, column_one_based: usize) -> Option<String> {{
        csv_cell_by_name(name, row_one_based, column_one_based)
    }}
}}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CsvCatalogSnapshot {{
    pub class: String,
    pub asset_count: usize,
    pub base_asset_count: usize,
    pub language_variant_count: usize,
    pub total_row_count: usize,
    pub total_nonempty_cell_count: usize,
    pub semicolon_asset_count: usize,
    pub comma_asset_count: usize,
    pub religion_row_count: usize,
    pub kombi_meta_row_count: usize,
}}

pub const CSV_ASSETS: &[CsvAsset] = &[
{chr(10).join(asset_entries)}
];

pub fn bootstrap_csv_catalog() -> CsvCatalogBundle {{
    CsvCatalogBundle {{ assets: CSV_ASSETS.to_vec() }}
}}

pub fn csv_catalog_owned() -> OwnedCsvCatalogBundle {{
    bootstrap_csv_catalog().owned()
}}

pub fn csv_asset_records() -> Vec<OwnedCsvAsset> {{
    CSV_ASSETS.iter().copied().map(CsvAsset::owned).collect()
}}

pub fn csv_asset_count() -> usize {{
    CSV_ASSETS.len()
}}

pub fn csv_total_row_count() -> usize {{
    CSV_ASSETS.iter().map(|asset| asset.row_count).sum()
}}

pub fn csv_language_variant_count() -> usize {{
    CSV_ASSETS.iter().filter(|asset| asset.language != CsvLanguage::Base).count()
}}

pub fn csv_asset_by_name(name: &str) -> Option<CsvAsset> {{
    CSV_ASSETS.iter().copied().find(|asset| asset.name == name)
}}

pub fn csv_assets_by_kind(kind: CsvAssetKind) -> Vec<CsvAsset> {{
    CSV_ASSETS.iter().copied().filter(|asset| asset.kind == kind).collect()
}}

pub fn csv_assets_by_language(language: CsvLanguage) -> Vec<CsvAsset> {{
    CSV_ASSETS.iter().copied().filter(|asset| asset.language == language).collect()
}}

pub fn csv_text_by_name(name: &str) -> Option<&'static str> {{
    match name {{
{chr(10).join(content_match)}
        _ => None,
    }}
}}

pub fn csv_rows_by_name(name: &str) -> Option<Vec<Vec<String>>> {{
    let asset = csv_asset_by_name(name)?;
    let text = csv_text_by_name(name)?;
    Some(parse_csv_text(text, asset.delimiter))
}}

pub fn csv_cell_by_name(name: &str, row_one_based: usize, column_one_based: usize) -> Option<String> {{
    if row_one_based == 0 || column_one_based == 0 {{
        return None;
    }}
    let rows = csv_rows_by_name(name)?;
    rows.get(row_one_based - 1)?.get(column_one_based - 1).cloned()
}}

pub fn parse_csv_text(text: &str, delimiter: CsvDelimiter) -> Vec<Vec<String>> {{
    parse_csv_text_with_delimiter(text, delimiter.as_char())
}}

pub fn parse_csv_text_with_delimiter(text: &str, delimiter: char) -> Vec<Vec<String>> {{
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut cell = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {{
        if ch == '"' {{
            if in_quotes && chars.peek() == Some(&'"') {{
                cell.push('"');
                chars.next();
            }} else {{
                in_quotes = !in_quotes;
            }}
        }} else if ch == delimiter && !in_quotes {{
            row.push(cell);
            cell = String::new();
        }} else if (ch == '\\n' || ch == '\\r') && !in_quotes {{
            if ch == '\\r' && chars.peek() == Some(&'\\n') {{
                chars.next();
            }}
            row.push(cell);
            cell = String::new();
            rows.push(row);
            row = Vec::new();
        }} else {{
            cell.push(ch);
        }}
    }}

    if !cell.is_empty() || !row.is_empty() || text.ends_with(delimiter) {{
        row.push(cell);
        rows.push(row);
    }}

    rows
}}

pub fn select_csv_rows_one_based(rows: &[Vec<String>], selected: &[usize]) -> Vec<Vec<String>> {{
    selected
        .iter()
        .filter_map(|line| line.checked_sub(1).and_then(|index| rows.get(index)).cloned())
        .collect()
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn catalog_contains_current_religion_csv() {{
        let asset = csv_asset_by_name("religion.csv").unwrap();
        assert_eq!(asset.kind, CsvAssetKind::Religion);
        assert_eq!(asset.row_count, {next(a.row_count for a in assets if a.name == 'religion.csv')});
        assert!(asset.max_columns > 700);
    }}

    #[test]
    fn catalog_knows_kombi_meta_csv() {{
        let asset = csv_asset_by_name("kombi-meta.csv").unwrap();
        assert_eq!(asset.kind, CsvAssetKind::KombiMeta);
        assert_eq!(asset.row_count, {next(a.row_count for a in assets if a.name == 'kombi-meta.csv')});
    }}

    #[test]
    fn parser_handles_quoted_delimiters_and_newlines() {{
        let parsed = parse_csv_text_with_delimiter("a;\\"b;c\\";d\\n1;\\"two\\nlines\\";3", ';');
        assert_eq!(parsed[0], vec!["a".to_string(), "b;c".to_string(), "d".to_string()]);
        assert_eq!(parsed[1][1], "two\\nlines");
    }}

    #[test]
    fn asset_parser_matches_static_metadata_for_smoke_files() {{
        for name in ["religion.csv", "kombi-meta.csv", "cn-thomas-decodedDekodiert-in-motives-purposesAbsichten.csv"] {{
            let asset = csv_asset_by_name(name).unwrap();
            let rows = csv_rows_by_name(name).unwrap();
            assert_eq!(rows.len(), asset.row_count, "{{name}}");
            assert_eq!(rows.iter().map(Vec::len).max().unwrap_or(0), asset.max_columns, "{{name}}");
        }}
    }}
}}
'''


def main() -> None:
    assets = read_assets()
    OUT.write_text(generate(assets), encoding="utf-8")
    print(json.dumps({
        "asset_count": len(assets),
        "total_rows": sum(a.row_count for a in assets),
        "total_nonempty_cells": sum(a.nonempty_cell_count for a in assets),
        "religion_rows": next(a.row_count for a in assets if a.name == "religion.csv"),
        "kombi_meta_rows": next(a.row_count for a in assets if a.name == "kombi-meta.csv"),
        "comma_assets": [a.name for a in assets if a.delimiter == "Comma"],
    }, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
