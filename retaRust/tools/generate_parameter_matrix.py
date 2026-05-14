#!/usr/bin/env python3
"""Regenerate crates/reta_architecture/src/parameter_matrix.rs from py reta arch.

The generated Rust file stores the alias-to-bucket projection of
`i18n.words_matrix.paraNdataMatrix`.  Stage 17 kept only a flattened integer
column projection.  Stage 18 preserves the legacy bucket coordinates too,
including symbolic generated/fraction selectors such as `primMotivStern`,
`PrimCSV` and the gebrochen-rational string numerators.
"""
from __future__ import annotations

import importlib
import json
import pathlib
import sys
import subprocess
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
PY_ARCH = ROOT / "python_arch_reference"
OUT = ROOT / "crates" / "reta_architecture" / "src" / "parameter_matrix.rs"


def as_strings(obj: Any) -> list[str]:
    if obj is None:
        return []
    if isinstance(obj, (str, int, float, bool)):
        return [str(obj)]
    try:
        values = [str(x) for x in obj if x is not None]
    except TypeError:
        return [str(obj)]
    if isinstance(obj, (set, frozenset)):
        values.sort()
    return values


def collect_bucket_values(obj: Any, integers: set[int], symbols: set[str]) -> None:
    if isinstance(obj, bool):
        return
    if isinstance(obj, int):
        integers.add(obj)
    elif isinstance(obj, str):
        symbols.add(obj)
    elif isinstance(obj, (set, frozenset, list, tuple)):
        for item in obj:
            collect_bucket_values(item, integers, symbols)


def rust_str(value: str) -> str:
    return json.dumps(str(value), ensure_ascii=False)


def rust_int_slice(values: list[int]) -> str:
    return "&[" + ", ".join(str(value) for value in values) + "]"


def rust_str_slice(values: list[str]) -> str:
    return "&[" + ", ".join(rust_str(value) for value in values) + "]"


def main() -> int:
    sys.path.insert(0, str(PY_ARCH))
    matrix_module = importlib.import_module("i18n.words_matrix")
    entries: list[tuple[int, list[str], list[str], list[int], list[tuple[int, list[int], list[str]]], int]] = []
    all_symbols: set[str] = set()
    all_int_columns: set[int] = set()
    nonempty_bucket_projection_count = 0

    for index, entry in enumerate(matrix_module.paraNdataMatrix):
        if len(entry) < 2:
            continue
        main_aliases = as_strings(entry[0])
        parameter_aliases = as_strings(entry[1])
        columns: set[int] = set()
        buckets: list[tuple[int, list[int], list[str]]] = []
        for bucket_index, bucket in enumerate(entry[2:]):
            integers: set[int] = set()
            symbols: set[str] = set()
            collect_bucket_values(bucket, integers, symbols)
            columns.update(integers)
            all_int_columns.update(integers)
            all_symbols.update(symbols)
            if integers or symbols:
                nonempty_bucket_projection_count += 1
                buckets.append((bucket_index, sorted(integers), sorted(symbols)))
        if main_aliases and parameter_aliases:
            entries.append((index, main_aliases, parameter_aliases, sorted(columns), buckets, max(0, len(entry) - 2)))

    lines: list[str] = []
    lines.append("//! Generated parameter matrix seed extracted from `python_arch_reference/i18n/words_matrix.py`.")
    lines.append("//!")
    lines.append("//! Stage 18 stores the same alias-to-bucket projection that `py reta arch`")
    lines.append("//! exposes through `paraNdataMatrix`: flattened integer columns for the")
    lines.append("//! existing simple path plus exact legacy bucket coordinates for generated,")
    lines.append("//! concat, Kombi and gebrochen-rational sections.  Symbolic bucket payloads")
    lines.append("//! are preserved instead of being silently flattened away.")
    lines.append("")
    lines.append("use serde::Serialize;")
    lines.append("")
    lines.append("use crate::schema::ParameterMatrixEntry;")
    lines.append("")
    lines.append("#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]")
    lines.append("pub struct ParameterBucketProjection {")
    lines.append("    pub bucket: u8,")
    lines.append("    pub integers: &'static [i64],")
    lines.append("    pub symbols: &'static [&'static str],")
    lines.append("}")
    lines.append("")
    lines.append("#[derive(Clone, Debug, Eq, PartialEq, Serialize)]")
    lines.append("pub struct OwnedParameterBucketProjection {")
    lines.append("    pub bucket: u8,")
    lines.append("    pub integers: Vec<i64>,")
    lines.append("    pub symbols: Vec<String>,")
    lines.append("}")
    lines.append("")
    lines.append("#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]")
    lines.append("pub struct ParameterMatrixSeed {")
    lines.append("    pub source_index: usize,")
    lines.append("    pub main_aliases: &'static [&'static str],")
    lines.append("    pub parameter_aliases: &'static [&'static str],")
    lines.append("    pub columns: &'static [i64],")
    lines.append("    pub buckets: &'static [ParameterBucketProjection],")
    lines.append("    pub legacy_bucket_count: usize,")
    lines.append("}")
    lines.append("")
    lines.append("impl ParameterMatrixSeed {")
    lines.append("    pub fn to_entry(self) -> ParameterMatrixEntry {")
    lines.append("        ParameterMatrixEntry {")
    lines.append("            main_aliases: self.main_aliases.iter().map(|value| (*value).to_string()).collect(),")
    lines.append("            parameter_aliases: self.parameter_aliases.iter().map(|value| (*value).to_string()).collect(),")
    lines.append("            columns: self.columns.to_vec(),")
    lines.append("        }")
    lines.append("    }")
    lines.append("}")
    lines.append("")
    lines.append("pub const PARAMETER_MATRIX_SEEDS: &[ParameterMatrixSeed] = &[")
    for index, main_aliases, parameter_aliases, columns, buckets, legacy_bucket_count in entries:
        main_slice = rust_str_slice(main_aliases)
        parameter_slice = rust_str_slice(parameter_aliases)
        column_slice = rust_int_slice(columns)
        bucket_parts = []
        for bucket, integers, symbols in buckets:
            bucket_parts.append(
                "ParameterBucketProjection { "
                f"bucket: {bucket}, integers: {rust_int_slice(integers)}, symbols: {rust_str_slice(symbols)} "
                "}"
            )
        bucket_slice = "&[" + ", ".join(bucket_parts) + "]"
        lines.append(
            "    ParameterMatrixSeed { "
            f"source_index: {index}, main_aliases: {main_slice}, "
            f"parameter_aliases: {parameter_slice}, columns: {column_slice}, "
            f"buckets: {bucket_slice}, legacy_bucket_count: {legacy_bucket_count} }},"
        )
    lines.append("];" )
    lines.append("")
    lines.append(f"pub const PARAMETER_MATRIX_SYMBOL_COUNT: usize = {len(all_symbols)};")
    lines.append(f"pub const PARAMETER_MATRIX_NONEMPTY_BUCKET_PROJECTION_COUNT: usize = {nonempty_bucket_projection_count};")
    lines.append("")
    lines.append("pub fn parameter_matrix_entries() -> Vec<ParameterMatrixEntry> {")
    lines.append("    PARAMETER_MATRIX_SEEDS.iter().copied().map(ParameterMatrixSeed::to_entry).collect()")
    lines.append("}")
    lines.append("")
    lines.append("pub fn parameter_matrix_seed_count() -> usize {")
    lines.append("    PARAMETER_MATRIX_SEEDS.len()")
    lines.append("}")
    lines.append("")
    lines.append("pub fn integer_column_projection_count() -> usize {")
    lines.append("    let mut seen = std::collections::BTreeSet::new();")
    lines.append("    for seed in PARAMETER_MATRIX_SEEDS {")
    lines.append("        for column in seed.columns {")
    lines.append("            seen.insert(*column);")
    lines.append("        }")
    lines.append("    }")
    lines.append("    seen.len()")
    lines.append("}")
    lines.append("")
    lines.append("pub fn symbolic_bucket_projection_count() -> usize {")
    lines.append("    PARAMETER_MATRIX_SYMBOL_COUNT")
    lines.append("}")
    lines.append("")
    lines.append("pub fn nonempty_bucket_projection_count() -> usize {")
    lines.append("    PARAMETER_MATRIX_NONEMPTY_BUCKET_PROJECTION_COUNT")
    lines.append("}")
    lines.append("")
    lines.append("pub fn columns_for_alias_pair(main: &str, parameter: &str) -> Vec<i64> {")
    lines.append("    let mut out = std::collections::BTreeSet::new();")
    lines.append("    let main = main.trim();")
    lines.append("    let parameter = parameter.trim();")
    lines.append("    for seed in PARAMETER_MATRIX_SEEDS {")
    lines.append("        if seed.main_aliases.iter().any(|alias| *alias == main)")
    lines.append("            && seed.parameter_aliases.iter().any(|alias| *alias == parameter)")
    lines.append("        {")
    lines.append("            out.extend(seed.columns.iter().copied());")
    lines.append("        }")
    lines.append("    }")
    lines.append("    out.into_iter().collect()")
    lines.append("}")
    lines.append("")
    lines.append("pub fn bucket_projections_for_alias_pair(main: &str, parameter: &str) -> Vec<OwnedParameterBucketProjection> {")
    lines.append("    let mut integers_by_bucket: std::collections::BTreeMap<u8, std::collections::BTreeSet<i64>> = std::collections::BTreeMap::new();")
    lines.append("    let mut symbols_by_bucket: std::collections::BTreeMap<u8, std::collections::BTreeSet<String>> = std::collections::BTreeMap::new();")
    lines.append("    let main = main.trim();")
    lines.append("    let parameter = parameter.trim();")
    lines.append("    for seed in PARAMETER_MATRIX_SEEDS {")
    lines.append("        if seed.main_aliases.iter().any(|alias| *alias == main)")
    lines.append("            && seed.parameter_aliases.iter().any(|alias| *alias == parameter)")
    lines.append("        {")
    lines.append("            for projection in seed.buckets {")
    lines.append("                integers_by_bucket.entry(projection.bucket).or_default().extend(projection.integers.iter().copied());")
    lines.append("                symbols_by_bucket.entry(projection.bucket).or_default().extend(projection.symbols.iter().map(|value| (*value).to_string()));")
    lines.append("            }")
    lines.append("        }")
    lines.append("    }")
    lines.append("    let mut keys = integers_by_bucket.keys().copied().collect::<std::collections::BTreeSet<_>>();")
    lines.append("    keys.extend(symbols_by_bucket.keys().copied());")
    lines.append("    keys.into_iter().map(|bucket| OwnedParameterBucketProjection {")
    lines.append("        bucket,")
    lines.append("        integers: integers_by_bucket.remove(&bucket).unwrap_or_default().into_iter().collect(),")
    lines.append("        symbols: symbols_by_bucket.remove(&bucket).unwrap_or_default().into_iter().collect(),")
    lines.append("    }).collect()")
    lines.append("}")
    lines.append("")
    lines.append("pub fn canonical_pair_for_aliases(main: &str, parameter: &str) -> Option<(String, String)> {")
    lines.append("    let main = main.trim();")
    lines.append("    let parameter = parameter.trim();")
    lines.append("    PARAMETER_MATRIX_SEEDS.iter().find_map(|seed| {")
    lines.append("        if seed.main_aliases.iter().any(|alias| *alias == main)")
    lines.append("            && seed.parameter_aliases.iter().any(|alias| *alias == parameter)")
    lines.append("        {")
    lines.append("            Some((seed.main_aliases[0].to_string(), seed.parameter_aliases[0].to_string()))")
    lines.append("        } else {")
    lines.append("            None")
    lines.append("        }")
    lines.append("    })")
    lines.append("}")
    lines.append("")
    lines.append("#[cfg(test)]")
    lines.append("mod tests {")
    lines.append("    use super::*;")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn generated_matrix_contains_current_744_regression() {")
    lines.append("        let columns = columns_for_alias_pair(\"kontinuum\", \"m\");")
    lines.append("        assert!(columns.contains(&493));")
    lines.append("        assert!(columns.contains(&744));")
    lines.append("        let buckets = bucket_projections_for_alias_pair(\"kontinuum\", \"m\");")
    lines.append("        assert_eq!(buckets.iter().find(|item| item.bucket == 0).unwrap().integers, vec![493, 744]);")
    lines.append("        assert_eq!(canonical_pair_for_aliases(\"kontinuum\", \"m\"), Some((\"Kontinuum\".to_string(), \"M\".to_string())));")
    lines.append("    }")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn generated_matrix_preserves_symbolic_bucket_payloads() {")
    lines.append("        let buckets = bucket_projections_for_alias_pair(\"multiplikationen\", \"motivstern\");")
    lines.append("        let bucket = buckets.iter().find(|item| item.bucket == 7).unwrap();")
    lines.append("        assert_eq!(bucket.symbols, vec![\"primMotivStern\".to_string()]);")
    lines.append("        let gebrochen = bucket_projections_for_alias_pair(\"gebrochenuniversum\", \"2\");")
    lines.append("        assert!(gebrochen.iter().any(|item| item.bucket == 5 && item.symbols.contains(&\"2\".to_string())));")
    lines.append("    }")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn generated_matrix_is_not_empty() {")
    lines.append("        assert!(parameter_matrix_seed_count() >= 400);")
    lines.append("        assert!(integer_column_projection_count() >= 600);")
    lines.append("        assert!(symbolic_bucket_projection_count() >= 30);")
    lines.append("        assert!(nonempty_bucket_projection_count() >= parameter_matrix_seed_count());")
    lines.append("    }")
    lines.append("}")
    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    try:
        subprocess.run(["rustfmt", str(OUT)], check=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    except OSError:
        pass
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
