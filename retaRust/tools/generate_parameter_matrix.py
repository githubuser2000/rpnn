#!/usr/bin/env python3
"""Regenerate crates/reta_architecture/src/parameter_matrix.rs from py reta arch.

The generated Rust file stores the integer column projection of
`i18n.words_matrix.paraNdataMatrix`.  It deliberately keeps complex bucket
semantics in their dedicated Rust modules; this matrix is the alias/pair index
needed by schema, sheaves, parameter runtime and prompt completion.
"""
from __future__ import annotations

import importlib
import json
import pathlib
import sys
from typing import Any, Iterable

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


def collect_ints(obj: Any, out: set[int]) -> None:
    if isinstance(obj, bool):
        return
    if isinstance(obj, int):
        out.add(obj)
    elif isinstance(obj, (set, list, tuple)):
        for item in obj:
            collect_ints(item, out)


def rust_str(value: str) -> str:
    return json.dumps(str(value), ensure_ascii=False)


def main() -> int:
    sys.path.insert(0, str(PY_ARCH))
    matrix_module = importlib.import_module("i18n.words_matrix")
    entries: list[tuple[int, list[str], list[str], list[int], int]] = []
    for index, entry in enumerate(matrix_module.paraNdataMatrix):
        if len(entry) < 2:
            continue
        main_aliases = as_strings(entry[0])
        parameter_aliases = as_strings(entry[1])
        columns: set[int] = set()
        for bucket in entry[2:]:
            collect_ints(bucket, columns)
        if main_aliases and parameter_aliases:
            entries.append((index, main_aliases, parameter_aliases, sorted(columns), max(0, len(entry) - 2)))

    lines: list[str] = []
    lines.append("//! Generated parameter matrix seed extracted from `python_arch_reference/i18n/words_matrix.py`.")
    lines.append("//!")
    lines.append("//! Stage 17 makes the Rust schema/sheaf/parameter runtime depend on the")
    lines.append("//! same alias-to-column matrix that `py reta arch` exposes.  The seed keeps")
    lines.append("//! only the integer column projection of the legacy bucket tuple; complex")
    lines.append("//! bucket semantics stay in their dedicated table/meta/generated modules.")
    lines.append("")
    lines.append("use serde::Serialize;")
    lines.append("")
    lines.append("use crate::schema::ParameterMatrixEntry;")
    lines.append("")
    lines.append("#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]")
    lines.append("pub struct ParameterMatrixSeed {")
    lines.append("    pub source_index: usize,")
    lines.append("    pub main_aliases: &'static [&'static str],")
    lines.append("    pub parameter_aliases: &'static [&'static str],")
    lines.append("    pub columns: &'static [i64],")
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
    for index, main_aliases, parameter_aliases, columns, legacy_bucket_count in entries:
        main_slice = "&[" + ", ".join(rust_str(item) for item in main_aliases) + "]"
        parameter_slice = "&[" + ", ".join(rust_str(item) for item in parameter_aliases) + "]"
        column_slice = "&[" + ", ".join(str(column) for column in columns) + "]"
        lines.append(
            "    ParameterMatrixSeed { "
            f"source_index: {index}, main_aliases: {main_slice}, "
            f"parameter_aliases: {parameter_slice}, columns: {column_slice}, "
            f"legacy_bucket_count: {legacy_bucket_count} }},"
        )
    lines.append("];")
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
    lines.append("        assert_eq!(canonical_pair_for_aliases(\"kontinuum\", \"m\"), Some((\"Kontinuum\".to_string(), \"M\".to_string())));")
    lines.append("    }")
    lines.append("")
    lines.append("    #[test]")
    lines.append("    fn generated_matrix_is_not_empty() {")
    lines.append("        assert!(parameter_matrix_seed_count() >= 400);")
    lines.append("        assert!(integer_column_projection_count() >= 600);")
    lines.append("    }")
    lines.append("}")
    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
