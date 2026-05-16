#!/usr/bin/env python3
"""Probe migration_control.rs step(...) calls for the Rust reta architecture migration plan.

The helper function step(...) currently accepts exactly seven arguments. This probe
keeps new migration entries from accidentally adding an eighth argument, which is
easy to do when adding a new morphism/gate/witness string.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


def repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def rust_file() -> Path:
    return repo_root() / "crates" / "reta_architecture" / "src" / "migration_control.rs"


def extract_calls(source: str, name: str = "step") -> list[dict[str, Any]]:
    calls: list[dict[str, Any]] = []
    needle = f"{name}("
    idx = 0
    while True:
        start = source.find(needle, idx)
        if start < 0:
            break
        # Ignore the helper function definition: fn step(...)
        prefix = source[max(0, start - 4):start]
        if prefix.endswith("fn "):
            idx = start + len(needle)
            continue
        line = source[:start].count("\n") + 1
        pos = start + len(needle)
        depth = 1
        in_string = False
        escaped = False
        while pos < len(source) and depth:
            ch = source[pos]
            if in_string:
                if escaped:
                    escaped = False
                elif ch == "\\":
                    escaped = True
                elif ch == '"':
                    in_string = False
            else:
                if ch == '"':
                    in_string = True
                elif ch == '(':
                    depth += 1
                elif ch == ')':
                    depth -= 1
            pos += 1
        body = source[start + len(needle):pos - 1]
        calls.append({"line": line, "body": body})
        idx = pos
    return calls


def count_top_level_args(body: str) -> int:
    body_stripped = body.strip()
    if not body_stripped:
        return 0
    depth = 0
    in_string = False
    escaped = False
    commas = 0
    for ch in body:
        if in_string:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                in_string = False
        else:
            if ch == '"':
                in_string = True
            elif ch in "([{":
                depth += 1
            elif ch in ")]}":
                depth -= 1
            elif ch == "," and depth == 0:
                commas += 1
    return commas if body_stripped.endswith(",") else commas + 1


def first_string_args(body: str) -> list[str]:
    values: list[str] = []
    current: list[str] = []
    in_string = False
    escaped = False
    for ch in body:
        if in_string:
            if escaped:
                current.append(ch)
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == '"':
                values.append("".join(current))
                current = []
                in_string = False
            else:
                current.append(ch)
        else:
            if ch == '"':
                in_string = True
    return values


def build_report() -> dict[str, Any]:
    path = rust_file()
    text = path.read_text(encoding="utf-8")
    calls = extract_calls(text)
    entries = []
    bad = []
    readiness = None
    for call in calls:
        count = count_top_level_args(call["body"])
        string_args = first_string_args(call["body"])
        step_id = string_args[0] if string_args else None
        entry = {
            "line": call["line"],
            "arg_count": count,
            "step_id": step_id,
            "string_arg_count": len(string_args),
        }
        entries.append(entry)
        if count != 7:
            bad.append(entry)
        if step_id == "step-table-view-activation-readiness":
            readiness = {
                **entry,
                "string_args": string_args,
                "contains_policy_from_cli": "table_view_activation_readiness.policy_from_cli" in string_args,
                "contains_default_promotion_gate": "table_view_activation_readiness.default_promotion_gate" in string_args,
                "contains_universal_oracle": "all_local_activation_witnesses_must_glue_before_default_visible_promotion" in string_args,
            }
    report = {
        "status": "ok" if not bad and readiness and readiness["contains_policy_from_cli"] else "failed",
        "file": str(path.relative_to(repo_root())),
        "step_call_count": len(entries),
        "expected_arg_count": 7,
        "bad_step_calls": bad,
        "unique_arg_counts": sorted({entry["arg_count"] for entry in entries}),
        "activation_readiness_step": readiness,
    }
    return report


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()
    report = build_report()
    print(json.dumps(report, indent=2 if args.pretty else None, ensure_ascii=False, sort_keys=True))
    return 0 if report["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
