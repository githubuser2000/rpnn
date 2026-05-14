#!/usr/bin/env python3
"""Run architecture shadow/parity smoke probes after the Rust workspace is built.

The script is intentionally dependency-free.  It compares the currently built
Rust binary against optional Python legacy / Python architecture commands and
also exercises the new `rreta_arch_inspect` plan binary when it exists.
"""

from __future__ import annotations

import argparse
import difflib
import json
import os
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Sequence


@dataclass(frozen=True)
class ProbeCase:
    case_id: str
    args: tuple[str, ...]
    category: str


CASES: tuple[ProbeCase, ...] = (
    ProbeCase(
        "row-basic-vorhervonausschnitt",
        ("-zeilen", "--vorhervonausschnitt=1-3", "-spalten", "--breite=0"),
        "rows",
    ),
    ProbeCase(
        "kontinuum-m-744-regression",
        ("-zeilen", "--vorhervonausschnitt=1-1", "-spalten", "--kontinuum=m", "--breite=0"),
        "tag_schema",
    ),
    ProbeCase(
        "output-mode-markdown",
        ("-zeilen", "--vorhervonausschnitt=1-2", "-ausgabe", "--art=markdown"),
        "output",
    ),
)


@dataclass
class CommandResult:
    name: str
    returncode: int
    stdout: str
    stderr: str


def run_command(name: str, command: Sequence[str], cwd: Path, env: dict[str, str]) -> CommandResult:
    try:
        proc = subprocess.run(
            list(command),
            cwd=str(cwd),
            env=env,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
    except FileNotFoundError as exc:
        return CommandResult(name, 127, "", f"missing executable: {exc}\n")
    return CommandResult(name, proc.returncode, proc.stdout, proc.stderr)


def compare(left: CommandResult, right: CommandResult) -> dict[str, object]:
    stdout_equal = left.stdout == right.stdout
    stderr_equal = left.stderr == right.stderr
    exit_equal = left.returncode == right.returncode
    diff_preview: list[str] = []
    if not stdout_equal:
        diff_preview = list(
            difflib.unified_diff(
                left.stdout.splitlines(),
                right.stdout.splitlines(),
                fromfile=left.name,
                tofile=right.name,
                lineterm="",
                n=3,
            )
        )[:40]
    return {
        "left": left.name,
        "right": right.name,
        "stdout_equal": stdout_equal,
        "stderr_equal": stderr_equal,
        "exit_equal": exit_equal,
        "left_exit": left.returncode,
        "right_exit": right.returncode,
        "diff_preview": diff_preview,
    }


def existing_executable(path: str | None) -> Path | None:
    if not path:
        return None
    p = Path(path)
    return p if p.exists() else None


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", default=Path(__file__).resolve().parents[1], type=Path)
    parser.add_argument("--rust", default="target/debug/rreta")
    parser.add_argument("--inspect", default="target/debug/rreta_arch_inspect")
    parser.add_argument("--py-reta", default=None, help="optional path to legacy py reta.py")
    parser.add_argument("--py-reta-arch", default=None, help="optional path to py reta arch reta.py")
    parser.add_argument("--arch-mode", default="dry-run")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args(argv)

    repo = args.repo.resolve()
    env = os.environ.copy()
    env.setdefault("RETA_ARCH", args.arch_mode)

    rust = existing_executable(str(repo / args.rust)) or existing_executable(args.rust)
    inspect = existing_executable(str(repo / args.inspect)) or existing_executable(args.inspect)
    py_reta = existing_executable(args.py_reta)
    py_reta_arch = existing_executable(args.py_reta_arch)

    report: dict[str, object] = {"arch_mode": args.arch_mode, "cases": []}
    failures = 0

    for case in CASES:
        commands: list[tuple[str, list[str]]] = []
        if py_reta:
            commands.append(("py_reta", [sys.executable, str(py_reta), *case.args]))
        if py_reta_arch:
            commands.append(("py_reta_arch", [sys.executable, str(py_reta_arch), *case.args]))
        if rust:
            commands.append(("rust_rreta", [str(rust), f"--reta-arch={args.arch_mode}", *case.args]))
        if inspect:
            commands.append(("rust_arch_plan", [str(inspect), f"--reta-arch={args.arch_mode}", *case.args]))

        results = [run_command(name, command, repo, env) for name, command in commands]
        comparisons = []
        for left_index in range(len(results)):
            for right_index in range(left_index + 1, len(results)):
                left = results[left_index]
                right = results[right_index]
                if left.name == "rust_arch_plan" or right.name == "rust_arch_plan":
                    continue
                item = compare(left, right)
                comparisons.append(item)
                if not (item["stdout_equal"] and item["stderr_equal"] and item["exit_equal"]):
                    failures += 1
        plan_summary = None
        for result in results:
            if result.name == "rust_arch_plan" and result.stdout.strip().startswith("{"):
                try:
                    plan = json.loads(result.stdout)
                    plan_summary = {
                        "switch_mode": plan.get("switch_mode"),
                        "gates": len(plan.get("gates", [])),
                        "activation_units": len(plan.get("activation_units", [])),
                        "cleaned_args": plan.get("cleaned_args", []),
                    }
                except json.JSONDecodeError:
                    plan_summary = {"parse_error": result.stdout[:200]}
        report["cases"].append(
            {
                "case_id": case.case_id,
                "category": case.category,
                "results": [r.__dict__ for r in results],
                "comparisons": comparisons,
                "plan_summary": plan_summary,
            }
        )

    if args.json:
        print(json.dumps(report, indent=2, ensure_ascii=False))
    else:
        for case in report["cases"]:  # type: ignore[index]
            print(f"{case['case_id']} [{case['category']}]")
            for result in case["results"]:
                print(f"  {result['name']}: exit={result['returncode']} stdout={len(result['stdout'])} stderr={len(result['stderr'])}")
            if case.get("plan_summary"):
                print(f"  plan: {case['plan_summary']}")
            for cmp in case["comparisons"]:
                status = "OK" if cmp["stdout_equal"] and cmp["stderr_equal"] and cmp["exit_equal"] else "DIFF"
                print(f"  {status}: {cmp['left']} ↔ {cmp['right']}")
                for line in cmp.get("diff_preview", []):
                    print(f"    {line}")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
