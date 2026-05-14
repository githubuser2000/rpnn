#!/usr/bin/env python3
"""Smoke-test the guarded Rust-architecture commit path.

The probe is intentionally small and dependency-free.  It compares the visible
output of the normal legacy-compatible `rreta` path with dry-run and guarded
commit modes.  The commit mode is only expected to change the internal renderer
when the Rust shadow output already matches legacy, so stdout/stderr/exit-code
must stay equal for all cases here.
"""

from __future__ import annotations

import argparse
import difflib
import json
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Sequence


@dataclass(frozen=True)
class ProbeCase:
    case_id: str
    args: tuple[str, ...]


CASES: tuple[ProbeCase, ...] = (
    ProbeCase("row-basic-vorhervonausschnitt", ("-zeilen", "--vorhervonausschnitt=1-3", "-spalten", "--breite=0")),
    ProbeCase("kontinuum-m-744-regression", ("-zeilen", "--vorhervonausschnitt=1-1", "-spalten", "--kontinuum=m", "--breite=0")),
    ProbeCase("markdown-output-guard", ("-zeilen", "--vorhervonausschnitt=1-2", "-ausgabe", "--art=markdown")),
)


@dataclass
class RunResult:
    label: str
    returncode: int
    stdout: str
    stderr: str


def run(label: str, command: Sequence[str], cwd: Path) -> RunResult:
    try:
        proc = subprocess.run(
            list(command),
            cwd=str(cwd),
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
        )
    except FileNotFoundError as exc:
        return RunResult(label, 127, "", f"missing executable: {exc}\n")
    return RunResult(label, proc.returncode, proc.stdout, proc.stderr)


def compare(left: RunResult, right: RunResult) -> dict[str, object]:
    stdout_equal = left.stdout == right.stdout
    stderr_equal = left.stderr == right.stderr
    exit_equal = left.returncode == right.returncode
    diff_preview: list[str] = []
    if not stdout_equal:
        diff_preview = list(
            difflib.unified_diff(
                left.stdout.splitlines(),
                right.stdout.splitlines(),
                fromfile=left.label,
                tofile=right.label,
                lineterm="",
                n=3,
            )
        )[:40]
    return {
        "left": left.label,
        "right": right.label,
        "stdout_equal": stdout_equal,
        "stderr_equal": stderr_equal,
        "exit_equal": exit_equal,
        "left_exit": left.returncode,
        "right_exit": right.returncode,
        "diff_preview": diff_preview,
    }


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--rust", default="target/debug/rreta")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args(argv)

    repo = args.repo.resolve()
    rust = Path(args.rust)
    if not rust.is_absolute():
        rust = repo / rust

    allow = "shadow_pipeline.table_commit,shadow_pipeline.table_adapter,table_adapters.render"
    report: dict[str, object] = {"rust": str(rust), "cases": []}
    failures = 0

    for case in CASES:
        base = run("legacy", [str(rust), *case.args], repo)
        dry = run("dry-run", [str(rust), "--reta-arch=dry-run", *case.args], repo)
        commit = run(
            "commit-guard",
            [str(rust), "--reta-arch=commit", f"--reta-arch-allow={allow}", *case.args],
            repo,
        )
        comparisons = [compare(base, dry), compare(base, commit)]
        for item in comparisons:
            if not (item["stdout_equal"] and item["stderr_equal"] and item["exit_equal"]):
                failures += 1
        report["cases"].append(
            {
                "case_id": case.case_id,
                "args": list(case.args),
                "results": [base.__dict__, dry.__dict__, commit.__dict__],
                "comparisons": comparisons,
            }
        )

    if args.json:
        print(json.dumps(report, indent=2, ensure_ascii=False))
    else:
        for case in report["cases"]:  # type: ignore[index]
            print(case["case_id"])
            for result in case["results"]:
                print(f"  {result['label']}: exit={result['returncode']} stdout={len(result['stdout'])} stderr={len(result['stderr'])}")
            for item in case["comparisons"]:
                status = "OK" if item["stdout_equal"] and item["stderr_equal"] and item["exit_equal"] else "DIFF"
                print(f"  {status}: {item['left']} ↔ {item['right']}")
                for line in item.get("diff_preview", []):
                    print(f"    {line}")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
