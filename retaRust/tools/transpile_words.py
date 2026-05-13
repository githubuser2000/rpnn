
#!/usr/bin/env python3
from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from pathlib import Path


def rust_str_lit(s: str) -> str:
    return json.dumps(s, ensure_ascii=False)


def rs_string_expr(s: str) -> str:
    return f"{rust_str_lit(s)}.to_string()"


def rust_atom(v, enum_name: str) -> str:
    if v is None:
        return f"{enum_name}::NoneValue"
    if isinstance(v, bool):
        return f"{enum_name}::Bool({str(v).lower()})"
    if isinstance(v, int):
        return f"{enum_name}::Int({v})"
    if isinstance(v, str):
        return f"{enum_name}::Str({rust_str_lit(v)}.to_string())"
    if isinstance(v, tuple):
        inner = ", ".join(rust_atom(x, enum_name) for x in v)
        return f"{enum_name}::Tuple(vec![{inner}])"
    raise TypeError(f"unsupported atom: {type(v)!r} {v!r}")


def rust_raw_string(text: str) -> str:
    for n in range(1, 16):
        end = '"' + ('#' * n)
        if end not in text:
            hashes = '#' * n
            return f'r{hashes}"{text}"{hashes}'
    raise ValueError("no safe raw string delimiter found")


def seq_to_vec_str(seq) -> str:
    return ", ".join(rs_string_expr(str(x)) for x in seq)


def normalized_name_items(value):
    if isinstance(value, (tuple, list)):
        return list(value)
    if isinstance(value, set):
        return sorted(list(value), key=py_sort_key)
    return [value]


def field_for_entry(module, main_value):
    for field in module.ParametersMain._fields:
        if getattr(module.ParametersMain, field) == main_value:
            return field
    raise KeyError(repr(main_value))


def py_sort_key(v):
    if v is None:
        return (0,)
    if isinstance(v, bool):
        return (1, int(v))
    if isinstance(v, int):
        return (2, v)
    if isinstance(v, str):
        if v.lstrip('-').isdigit():
            return (3, 0, int(v), v)
        return (3, 1, v)
    if isinstance(v, tuple):
        return (4, tuple(py_sort_key(x) for x in v))
    raise TypeError(f"unsupported sort key atom: {type(v)!r} {v!r}")


def data_blocks(entry, enum_name: str, total_blocks: int = 12):
    blocks = []
    for idx in range(total_blocks):
        if idx < len(entry) - 2:
            items = sorted(list(entry[idx + 2]), key=py_sort_key)
            if items:
                blocks.append("vec![" + ", ".join(rust_atom(v, enum_name) for v in items) + "]")
            else:
                blocks.append("vec![]")
        else:
            blocks.append("vec![]")
    return blocks


def extract_block(source_text: str, start_marker: str, end_marker: str | None) -> str:
    start = source_text.index(start_marker) + len(start_marker)
    if end_marker is None:
        return source_text[start:].rstrip()
    end = source_text.index(end_marker, start)
    return source_text[start:end].rstrip()


def generate_words_py(module) -> str:
    lines = [
        "#![allow(non_snake_case)]",
        "",
        "use indexmap::IndexMap;",
        "use serde::{Deserialize, Serialize};",
        "",
        "#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]",
        "pub enum PyValue {",
        "    Int(i64),",
        "    Str(String),",
        "    Bool(bool),",
        "    Tuple(Vec<PyValue>),",
        "    NoneValue,",
        "}",
        "",
        "#[derive(Clone, Debug, Serialize, Deserialize)]",
        "pub struct StoreParameterEntry {",
        "    pub parameterMainNames: Vec<String>,",
        "    pub parameterNames: Vec<String>,",
        "    pub datas: Vec<Vec<PyValue>>,",
        "}",
        "",
        "#[derive(Clone, Debug, Serialize, Deserialize)]",
        "pub struct Words {",
        "    pub paraNdataMatrix: Vec<StoreParameterEntry>,",
        "    pub kombiParaNdataMatrix: IndexMap<i64, Vec<String>>,",
        "    pub kombiParaNdataMatrix2: IndexMap<i64, Vec<String>>,",
        "}",
        "",
        "impl Words {",
        "    pub fn new() -> Self {",
        "        let mut paraNdataMatrix: Vec<StoreParameterEntry> = vec![];",
    ]
    for entry in module.paraNdataMatrix:
        main_names = normalized_name_items(entry[0])
        param_names = normalized_name_items(entry[1])
        blocks = data_blocks(entry, "PyValue")
        lines.extend([
            "        paraNdataMatrix.push(StoreParameterEntry {",
            f"            parameterMainNames: vec![{seq_to_vec_str(main_names)}],",
            f"            parameterNames: vec![{seq_to_vec_str(param_names)}],",
            "            datas: vec![",
        ])
        for block in blocks:
            lines.append(f"                {block},")
        lines.extend([
            "            ],",
            "        });",
        ])
    lines.extend([
        "",
        "        let mut kombiParaNdataMatrix: IndexMap<i64, Vec<String>> = IndexMap::new();",
    ])
    for k, vals in module.kombiParaNdataMatrix.items():
        lines.append(f"        kombiParaNdataMatrix.insert({k}, vec![{seq_to_vec_str(vals)}]);")
    lines.extend([
        "",
        "        let mut kombiParaNdataMatrix2: IndexMap<i64, Vec<String>> = IndexMap::new();",
    ])
    for k, vals in module.kombiParaNdataMatrix2.items():
        lines.append(f"        kombiParaNdataMatrix2.insert({k}, vec![{seq_to_vec_str(vals)}]);")
    lines.extend([
        "",
        "        Self {",
        "            paraNdataMatrix,",
        "            kombiParaNdataMatrix,",
        "            kombiParaNdataMatrix2,",
        "        }",
        "    }",
        "}",
        "",
    ])
    return "\n".join(lines)


def generate_runtime_mod(module) -> str:
    lines = [
        "#![allow(non_snake_case)]",
        "use indexmap::IndexMap;",
        "",
        "#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]",
        "pub enum PyAtom {",
        "    Int(i64),",
        "    Str(String),",
        "    Bool(bool),",
        "    Tuple(Vec<PyAtom>),",
        "    NoneValue,",
        "}",
        "",
        "#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]",
        "pub struct PairStr(pub String, pub String);",
        "",
        "#[derive(Clone, Debug)]",
        "pub struct StoreParameterEntry {",
        "    pub parameterMainNames: Vec<String>,",
        "    pub parameterNames: Vec<String>,",
        "    pub datas: Vec<Vec<PyAtom>>,",
        "}",
        "",
        "#[derive(Clone, Debug)]",
        "pub struct I18nExact {",
        "    pub paraNdataMatrix: Vec<StoreParameterEntry>,",
        "    pub kombiParaNdataMatrix: IndexMap<i64, Vec<String>>,",
        "    pub kombiParaNdataMatrix2: IndexMap<i64, Vec<String>>,",
        "}",
        "",
        "impl I18nExact {",
        "    pub fn from_python_evaluated_shapes() -> Self {",
        "        let mut paraNdataMatrix: Vec<StoreParameterEntry> = vec![];",
    ]
    for entry in module.paraNdataMatrix:
        field = field_for_entry(module, entry[0])
        param_names = normalized_name_items(entry[1])
        blocks = data_blocks(entry, "PyAtom")
        lines.extend([
            "        paraNdataMatrix.push(StoreParameterEntry {",
            f"            parameterMainNames: vec![{rs_string_expr(field)}],",
            f"            parameterNames: vec![{seq_to_vec_str(param_names)}],",
            "            datas: vec![",
        ])
        for block in blocks:
            lines.append(f"                {block},")
        lines.extend([
            "            ],",
            "        });",
        ])
    lines.extend([
        "",
        "        let mut kombiParaNdataMatrix: IndexMap<i64, Vec<String>> = IndexMap::new();",
    ])
    for k, vals in module.kombiParaNdataMatrix.items():
        lines.append(f"        kombiParaNdataMatrix.insert({k}, vec![{seq_to_vec_str(vals)}]);")
    lines.extend([
        "",
        "        let mut kombiParaNdataMatrix2: IndexMap<i64, Vec<String>> = IndexMap::new();",
    ])
    for k, vals in module.kombiParaNdataMatrix2.items():
        lines.append(f"        kombiParaNdataMatrix2.insert({k}, vec![{seq_to_vec_str(vals)}]);")
    lines.extend([
        "",
        "        Self {",
        "            paraNdataMatrix,",
        "            kombiParaNdataMatrix,",
        "            kombiParaNdataMatrix2,",
        "        }",
        "    }",
        "}",
        "",
    ])
    return "\n".join(lines)


def generate_python_like_words(module, para_block: str, kombi1_block: str, kombi2_block: str) -> str:
    lines = [
        "#![allow(non_snake_case)]",
        "use indexmap::IndexMap;",
        "use serde::{Deserialize, Serialize};",
        "",
        "#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]",
        "pub enum PyValue {",
        "    Int(i64),",
        "    Str(String),",
        "    Bool(bool),",
        "    Tuple(Vec<PyValue>),",
        "    NoneValue,",
        "}",
        "",
        "#[derive(Clone, Debug, Serialize, Deserialize)]",
        "pub struct StoreParameterEntry {",
        "    pub parameterMainNames: Vec<String>,",
        "    pub parameterNames: Vec<String>,",
        "    pub datas: Vec<Vec<PyValue>>,",
        "}",
        "",
        "#[derive(Clone, Debug, Serialize, Deserialize)]",
        "pub struct Words {",
        "    pub paraNdataMatrix: Vec<StoreParameterEntry>,",
        "    pub kombiParaNdataMatrix: IndexMap<i64, Vec<String>>,",
        "    pub kombiParaNdataMatrix2: IndexMap<i64, Vec<String>>,",
        "}",
        "",
        "impl Words {",
        "    pub fn new() -> Self {",
        "        let mut paraNdataMatrix: Vec<StoreParameterEntry> = vec![];",
    ]
    for entry in module.paraNdataMatrix:
        field = field_for_entry(module, entry[0])
        field_repr = str([field])
        param_names = normalized_name_items(entry[1])
        blocks = data_blocks(entry, "PyValue")
        lines.extend([
            "        paraNdataMatrix.push(StoreParameterEntry {",
            f"            parameterMainNames: vec![{rs_string_expr(field_repr)}],",
            f"            parameterNames: vec![{seq_to_vec_str(param_names)}],",
            "            datas: vec![",
        ])
        for block in blocks:
            lines.append(f"                {block},")
        lines.extend([
            "            ],",
            "        });",
        ])
    lines.extend([
        "",
        "        let mut kombiParaNdataMatrix: IndexMap<i64, Vec<String>> = IndexMap::new();",
    ])
    for k, vals in module.kombiParaNdataMatrix.items():
        lines.append(f"        kombiParaNdataMatrix.insert({k}, vec![{seq_to_vec_str(vals)}]);")
    lines.extend([
        "",
        "        let mut kombiParaNdataMatrix2: IndexMap<i64, Vec<String>> = IndexMap::new();",
    ])
    for k, vals in module.kombiParaNdataMatrix2.items():
        lines.append(f"        kombiParaNdataMatrix2.insert({k}, vec![{seq_to_vec_str(vals)}]);")
    lines.extend([
        "",
        "        Self {",
        "            paraNdataMatrix,",
        "            kombiParaNdataMatrix,",
        "            kombiParaNdataMatrix2,",
        "        }",
        "    }",
        "}",
        "",
        f"pub const PYTHON_SOURCE__WORDS_PARA_NDATA_MATRIX: &str = {rust_raw_string(para_block)};",
        "",
        f"pub const PYTHON_SOURCE__WORDS_KOMBI_MATRIX_1: &str = {rust_raw_string(kombi1_block)};",
        "",
        f"pub const PYTHON_SOURCE__WORDS_KOMBI_MATRIX_2: &str = {rust_raw_string(kombi2_block)};",
        "",
    ])
    return "\n".join(lines)


def generate_exact_i18n(module, para_block: str, kombi1_block: str, kombi2_block: str) -> str:
    lines = [
        "use indexmap::IndexMap;",
        "",
        "#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]",
        "pub enum PyAtom {",
        "    Int(i64),",
        "    Str(String),",
        "    Bool(bool),",
        "    Tuple(Vec<PyAtom>),",
        "    NoneValue,",
        "}",
        "",
        "#[derive(Clone, Debug)]",
        "pub struct StoreParameterEntry {",
        "    pub parameterMainNames: Vec<String>,",
        "    pub parameterNames: Vec<String>,",
        "    pub datas: Vec<Vec<PyAtom>>,",
        "}",
        "",
        "#[derive(Clone, Debug)]",
        "pub struct I18nSubset {",
        "    pub paraNdataMatrix: Vec<StoreParameterEntry>,",
        "    pub kombiParaNdataMatrix: IndexMap<i64, Vec<String>>,",
        "    pub kombiParaNdataMatrix2: IndexMap<i64, Vec<String>>,",
        "}",
        "",
        "impl I18nSubset {",
        "    pub fn new() -> Self {",
        "        let mut paraNdataMatrix: Vec<StoreParameterEntry> = vec![];",
    ]
    for entry in module.paraNdataMatrix:
        field = field_for_entry(module, entry[0])
        field_repr = str([field])
        param_names = normalized_name_items(entry[1])
        blocks = data_blocks(entry, "PyAtom")
        lines.extend([
            "        paraNdataMatrix.push(StoreParameterEntry {",
            f"            parameterMainNames: vec![{rs_string_expr(field_repr)}],",
            f"            parameterNames: vec![{seq_to_vec_str(param_names)}],",
            "            datas: vec![",
        ])
        for block in blocks:
            lines.append(f"                {block},")
        lines.extend([
            "            ],",
            "        });",
        ])
    lines.extend([
        "",
        "        let mut kombiParaNdataMatrix: IndexMap<i64, Vec<String>> = IndexMap::new();",
    ])
    for k, vals in module.kombiParaNdataMatrix.items():
        lines.append(f"        kombiParaNdataMatrix.insert({k}, vec![{seq_to_vec_str(vals)}]);")
    lines.extend([
        "",
        "        let mut kombiParaNdataMatrix2: IndexMap<i64, Vec<String>> = IndexMap::new();",
    ])
    for k, vals in module.kombiParaNdataMatrix2.items():
        lines.append(f"        kombiParaNdataMatrix2.insert({k}, vec![{seq_to_vec_str(vals)}]);")
    lines.extend([
        "",
        "        Self {",
        "            paraNdataMatrix,",
        "            kombiParaNdataMatrix,",
        "            kombiParaNdataMatrix2,",
        "        }",
        "    }",
        "}",
        "",
        f"pub const PYTHON_SOURCE__WORDS_PARA_NDATA_MATRIX: &str = {rust_raw_string(para_block)};",
        "",
        f"pub const PYTHON_SOURCE__WORDS_KOMBI_MATRIX_1: &str = {rust_raw_string(kombi1_block)};",
        "",
        f"pub const PYTHON_SOURCE__WORDS_KOMBI_MATRIX_2: &str = {rust_raw_string(kombi2_block)};",
        "",
    ])
    return "\n".join(lines)


def generate_words_rs(source_text: str) -> str:
    return "\n".join([
        "#![allow(non_snake_case)]",
        "/*",
        "DIREKT-TRANSCOMPILATIONSFRONT FÜR i18n/words.py",
        "Python-Quelle eingefroren für 1:1-Übernahme.",
        "*/",
        "",
        f"pub const PYTHON_SOURCE__WORDS: &str = {rust_raw_string(source_text)};",
        "",
    ])


def load_module(source_path: Path):
    sys.argv = [str(source_path.name)]
    spec = importlib.util.spec_from_file_location("transpiled_words_source", str(source_path))
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("source_words_py", type=Path)
    parser.add_argument("repo_root", type=Path)
    args = parser.parse_args()

    source_text = args.source_words_py.read_text(encoding="utf-8")
    module = load_module(args.source_words_py)

    para_block = extract_block(
        source_text,
        "paraNdataMatrix: list = ",
        "\nparaNdataMatrix = paraNdataMatrix",
    )
    kombi1_block = extract_block(
        source_text,
        "kombiParaNdataMatrix: OrderedDict = ",
        "\n\nkombiParaNdataMatrix2: OrderedDict = ",
    )
    kombi2_block = extract_block(
        source_text,
        "kombiParaNdataMatrix2: OrderedDict = ",
        None,
    )

    outputs = {
        args.repo_root / "python_reference" / "i18n" / "words.py": source_text,
        args.repo_root / "python_reference" / "words.py": source_text,
        args.repo_root / "src" / "i18n" / "words.rs": generate_words_rs(source_text),
        args.repo_root / "src" / "shared" / "words_py.rs": generate_words_py(module),
        args.repo_root / "src" / "runtime" / "mod.rs": generate_runtime_mod(module),
        args.repo_root / "src" / "shared" / "words_python_like.rs": generate_python_like_words(
            module, para_block, kombi1_block, kombi2_block
        ),
        args.repo_root / "src" / "shared" / "exact_i18n.rs": generate_exact_i18n(
            module, para_block, kombi1_block, kombi2_block
        ),
    }

    for path, content in outputs.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")

    print(f"updated {len(outputs)} files from {args.source_words_py}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
