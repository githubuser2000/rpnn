#!/usr/bin/env python3
"""Audit py-reta-arch symbol representation in Rust.

This is stricter than architecture_module_coverage.py.  It distinguishes:

* declared: the symbol is visible as a Rust item name (fn/struct/enum/trait/type/const/static/module)
* marker_only: the symbol is only listed in a PY_ARCH_STAGE15_SURFACE marker
* missing: no declaration and no marker

The Stage 15 marker layer is useful during migration, but marker_only symbols still
need a real semantic Rust implementation before a module can be called fully ported.
"""
from __future__ import annotations

import argparse
import ast
import json
import re
from pathlib import Path
from typing import Dict, Iterable, List, Tuple

NAME_MAP = {
    "category_theory": "category",
    "morphisms": "morphism",
    "presheaves": "presheaf",
    "sheaves": "sheaf",
}

ITEM_RE = re.compile(
    r"\b(?:pub\s+)?(?:async\s+)?(?:fn|struct|enum|trait|type|const|static|mod)\s+([A-Za-z_][A-Za-z0-9_]*)\b"
)
IMPL_FN_RE = re.compile(r"\b(?:pub\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\b")
MARKER_RE = re.compile(r"PY_ARCH_STAGE15_SURFACE\s*:\s*&\[&str\]\s*=\s*&\[(.*?)\];", re.S)
STRING_RE = re.compile(r'"([^"\\]*(?:\\.[^"\\]*)*)"')


def rust_module_name(py_stem: str) -> str:
    return NAME_MAP.get(py_stem, py_stem)


def collect_python_symbols(path: Path) -> Tuple[List[str], List[str]]:
    tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
    functions: List[str] = []
    classes: List[str] = []
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            functions.append(node.name)
        elif isinstance(node, ast.ClassDef):
            classes.append(node.name)
    return sorted(set(functions)), sorted(set(classes))


def candidates(symbol: str) -> List[str]:
    camel_as_snake = re.sub(r"(?<!^)(?=[A-Z])", "_", symbol).lower()
    return sorted({symbol, symbol.replace("_", ""), camel_as_snake})


def collect_marker_names(rust_text: str) -> set[str]:
    names: set[str] = set()
    for match in MARKER_RE.finditer(rust_text):
        names.update(STRING_RE.findall(match.group(1)))
    return names


def collect_declared_names(rust_text: str) -> set[str]:
    names = set(ITEM_RE.findall(rust_text))
    # ITEM_RE catches free items; keep this explicit to survive future regex edits.
    names.update(IMPL_FN_RE.findall(rust_text))
    return names


def classify_symbol(symbol: str, declared: set[str], markers: set[str]) -> str:
    cand = candidates(symbol)
    if any(name in declared for name in cand):
        return "declared"
    if symbol in markers or any(name in markers for name in cand):
        return "marker_only"
    return "missing"


def audit(py_dir: Path, rust_dir: Path) -> Dict[str, object]:
    modules = []
    totals = {
        "functions": 0,
        "functions_declared": 0,
        "functions_marker_only": 0,
        "functions_missing": 0,
        "classes": 0,
        "classes_declared": 0,
        "classes_marker_only": 0,
        "classes_missing": 0,
    }
    for py_path in sorted(py_dir.glob("*.py")):
        if py_path.name.startswith("__"):
            continue
        rust_path = rust_dir / f"{rust_module_name(py_path.stem)}.rs"
        functions, classes = collect_python_symbols(py_path)
        rust_text = rust_path.read_text(encoding="utf-8") if rust_path.exists() else ""
        declared = collect_declared_names(rust_text)
        markers = collect_marker_names(rust_text)

        module = {
            "python_module": py_path.name,
            "rust_module": rust_path.name,
            "rust_module_exists": rust_path.exists(),
            "functions": len(functions),
            "classes": len(classes),
            "functions_declared": [],
            "functions_marker_only": [],
            "functions_missing": [],
            "classes_declared": [],
            "classes_marker_only": [],
            "classes_missing": [],
        }
        for kind, symbols in (("functions", functions), ("classes", classes)):
            for symbol in symbols:
                bucket = classify_symbol(symbol, declared, markers)
                module[f"{kind}_{bucket}"].append(symbol)
                totals[kind] += 1
                totals[f"{kind}_{bucket}"] += 1
        modules.append(module)
    return {**totals, "module_count": len(modules), "modules": modules}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--py-dir", default="python_arch_reference/reta_architecture")
    parser.add_argument("--rust-dir", default="crates/reta_architecture/src")
    parser.add_argument("--pretty", action="store_true")
    parser.add_argument("--only-marker-or-missing", action="store_true")
    ns = parser.parse_args()
    result = audit(Path(ns.py_dir), Path(ns.rust_dir))
    if ns.only_marker_or_missing:
        result = {
            **{k: v for k, v in result.items() if k != "modules"},
            "modules": [
                m for m in result["modules"]
                if m["functions_marker_only"] or m["functions_missing"] or m["classes_marker_only"] or m["classes_missing"]
            ],
        }
    print(json.dumps(result, ensure_ascii=False, indent=2 if ns.pretty else None, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
