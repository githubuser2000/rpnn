#!/usr/bin/env python3
"""Smoke-probe guarded rretaPrompt architecture activation.

This runner intentionally does not need Python package dependencies.  It runs a
legacy prompt command, a dry-run architecture command, and a guarded commit
command, then checks that guarded prompt commit does not change visible output.
"""

from __future__ import annotations

import argparse
import dataclasses
import shlex
import subprocess
import sys
from pathlib import Path


@dataclasses.dataclass(frozen=True)
class PromptProbeCase:
    name: str
    command: str


CASES = [
    PromptProbeCase(
        "prompt-reta-kontinuum-m-744-regression",
        "reta -zeilen --vorhervonausschnitt=1-1 -spalten --kontinuum=m --breite=0",
    ),
    PromptProbeCase("prompt-simple-row-range", "reta -zeilen --vorhervonausschnitt=1-3 --breite=0"),
    PromptProbeCase("prompt-short-number-command", "12-15"),
]


@dataclasses.dataclass(frozen=True)
class RunResult:
    argv: list[str]
    returncode: int
    stdout: str
    stderr: str

    def visible_tuple(self) -> tuple[int, str, str]:
        return (self.returncode, self.stdout, self.stderr)


def run(argv: list[str], timeout: int) -> RunResult:
    completed = subprocess.run(
        argv,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=timeout,
        check=False,
    )
    return RunResult(argv=argv, returncode=completed.returncode, stdout=completed.stdout, stderr=completed.stderr)


def case_args(prompt_bin: Path, case: PromptProbeCase) -> list[str]:
    return [str(prompt_bin), *shlex.split(case.command)]


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prompt", type=Path, default=Path("target/debug/rrpb"))
    parser.add_argument("--inspect", type=Path, default=Path("target/debug/rretaprompt_arch_inspect"))
    parser.add_argument("--timeout", type=int, default=30)
    args = parser.parse_args(argv)

    failures: list[str] = []
    allow = "shadow_pipeline.prompt_commit,shadow_pipeline.prompt_adapter,prompt_interaction.plan,prompt_execution.argv"

    for case in CASES:
        legacy = run(case_args(args.prompt, case), args.timeout)
        dry_run = run(
            [str(args.prompt), "--reta-arch=dry-run", *shlex.split(case.command)],
            args.timeout,
        )
        commit = run(
            [
                str(args.prompt),
                "--reta-arch=commit",
                f"--reta-arch-allow={allow}",
                *shlex.split(case.command),
            ],
            args.timeout,
        )
        inspect = run(
            [
                str(args.inspect),
                "--reta-arch=commit",
                f"--reta-arch-allow={allow}",
                *shlex.split(case.command),
            ],
            args.timeout,
        )

        print(f"== {case.name} ==")
        print(f"legacy rc={legacy.returncode} stdout={len(legacy.stdout)} stderr={len(legacy.stderr)}")
        print(f"dry-run rc={dry_run.returncode} stdout={len(dry_run.stdout)} stderr={len(dry_run.stderr)}")
        print(f"commit rc={commit.returncode} stdout={len(commit.stdout)} stderr={len(commit.stderr)}")
        print(f"inspect rc={inspect.returncode} stdout={len(inspect.stdout)} stderr={len(inspect.stderr)}")

        if dry_run.visible_tuple() != legacy.visible_tuple():
            failures.append(f"{case.name}: dry-run changed visible output")
        if commit.visible_tuple() != legacy.visible_tuple():
            failures.append(f"{case.name}: guarded commit changed visible output")
        if inspect.returncode != 0:
            failures.append(f"{case.name}: inspect failed: {inspect.stderr.strip()}")
        if "prompt_commit" not in inspect.stdout:
            failures.append(f"{case.name}: inspect output did not mention prompt_commit")

    if failures:
        print("\nFAILURES:", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print("\nall prompt commit probes passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
