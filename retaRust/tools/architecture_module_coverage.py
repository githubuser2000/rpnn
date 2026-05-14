#!/usr/bin/env python3
"""Compare py-reta-arch module surfaces with reta_architecture Rust modules.

This is intentionally dependency-free.  It is a migration/audit helper, not a
proof of semantic equivalence.  It answers the practical question: which Python
classes/functions are at least represented by name in the Rust architecture
crate, and which still need explicit porting or a deliberate rename note?
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


def symbol_is_present(symbol: str, rust_text: str) -> bool:
    snake = symbol
    # Accept direct name, Rust snake_case, and Python camelCase remnants.
    candidates = {snake, symbol.replace("_", "")}
    # Very small camelCase -> snake_case helper for legacy Python names.
    camel_as_snake = re.sub(r"(?<!^)(?=[A-Z])", "_", symbol).lower()
    candidates.add(camel_as_snake)
    return any(re.search(r"\b" + re.escape(candidate) + r"\b", rust_text) for candidate in candidates)


def audit(py_dir: Path, rust_dir: Path) -> Dict[str, object]:
    modules = []
    total_functions = total_function_hits = 0
    total_classes = total_class_hits = 0
    for py_path in sorted(py_dir.glob("*.py")):
        if py_path.name.startswith("__"):
            continue
        rust_path = rust_dir / f"{rust_module_name(py_path.stem)}.rs"
        functions, classes = collect_python_symbols(py_path)
        rust_text = rust_path.read_text(encoding="utf-8") if rust_path.exists() else ""
        missing_functions = [name for name in functions if not symbol_is_present(name, rust_text)]
        missing_classes = [name for name in classes if not symbol_is_present(name, rust_text)]
        function_hits = len(functions) - len(missing_functions)
        class_hits = len(classes) - len(missing_classes)
        total_functions += len(functions)
        total_function_hits += function_hits
        total_classes += len(classes)
        total_class_hits += class_hits
        modules.append(
            {
                "python_module": py_path.name,
                "rust_module": rust_path.name,
                "rust_module_exists": rust_path.exists(),
                "functions": len(functions),
                "functions_represented_by_name": function_hits,
                "missing_functions": missing_functions,
                "classes": len(classes),
                "classes_represented_by_name": class_hits,
                "missing_classes": missing_classes,
            }
        )
    return {
        "python_directory": str(py_dir),
        "rust_directory": str(rust_dir),
        "module_count": len(modules),
        "functions": total_functions,
        "functions_represented_by_name": total_function_hits,
        "classes": total_classes,
        "classes_represented_by_name": total_class_hits,
        "modules": modules,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--py-dir", default="python_arch_reference/reta_architecture")
    parser.add_argument("--rust-dir", default="crates/reta_architecture/src")
    parser.add_argument("--pretty", action="store_true")
    parser.add_argument("--only-missing", action="store_true")
    ns = parser.parse_args()
    result = audit(Path(ns.py_dir), Path(ns.rust_dir))
    if ns.only_missing:
        result = {
            **{k: v for k, v in result.items() if k != "modules"},
            "modules": [
                item
                for item in result["modules"]
                if item["missing_functions"] or item["missing_classes"] or not item["rust_module_exists"]
            ],
        }
    print(json.dumps(result, ensure_ascii=False, indent=2 if ns.pretty else None, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
