#!/usr/bin/env python3
"""Vergleicht python_reference/generate4readme.py gegen das Rust-Binary rgenerate4readme."""
from __future__ import annotations

import difflib
import os
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
PY_REF = ROOT / "python_reference" / "generate4readme.py"
PY_REF_DIR = ROOT / "python_reference"
PY_REF_LIBS = PY_REF_DIR / "libs"


def python_env() -> dict[str, str]:
    env = os.environ.copy()
    parts = [str(PY_REF_DIR), str(PY_REF_LIBS), str(ROOT)]
    if env.get("PYTHONPATH"):
        parts.append(env["PYTHONPATH"])
    env["PYTHONPATH"] = os.pathsep.join(parts)
    return env


def run(command: list[str], *, timeout: float, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str] | subprocess.TimeoutExpired[str]:
    try:
        return subprocess.run(
            command,
            cwd=ROOT,
            env=env,
            text=True,
            capture_output=True,
            timeout=timeout,
        )
    except subprocess.TimeoutExpired as exc:
        return exc


def stdout_of(result: subprocess.CompletedProcess[str] | subprocess.TimeoutExpired[str]) -> str:
    return result.stdout or ""


def stderr_of(result: subprocess.CompletedProcess[str] | subprocess.TimeoutExpired[str]) -> str:
    return result.stderr or ""


def returncode_of(result: subprocess.CompletedProcess[str] | subprocess.TimeoutExpired[str]) -> int:
    return 124 if isinstance(result, subprocess.TimeoutExpired) else result.returncode


def parse_args(argv: list[str]) -> tuple[list[str], float, int, bool]:
    args = list(argv)
    timeout = float(os.environ.get("RETA_COMPARE_TIMEOUT", "120"))
    diff_limit = int(os.environ.get("RETA_COMPARE_DIFF_LINES", "200"))
    compare_stderr = False
    sample: list[str] = []
    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "--compare-stderr":
            compare_stderr = True
            i += 1
        elif arg == "--timeout":
            if i + 1 >= len(args):
                raise SystemExit("--timeout braucht eine Sekundenangabe")
            timeout = float(args[i + 1])
            i += 2
        elif arg == "--diff-lines":
            if i + 1 >= len(args):
                raise SystemExit("--diff-lines braucht eine Zeilenanzahl")
            diff_limit = int(args[i + 1])
            i += 2
        elif arg == "--":
            sample.extend(args[i + 1 :])
            break
        else:
            sample.append(arg)
            i += 1
    return sample, timeout, diff_limit, compare_stderr


def main(argv: list[str]) -> int:
    if not PY_REF.exists():
        print(f"Python-Referenz fehlt: {PY_REF}", file=sys.stderr)
        return 2
    if shutil.which("cargo") is None:
        print("cargo nicht gefunden; Rust-Vergleich kann hier nicht laufen.", file=sys.stderr)
        return 2

    sample, timeout, diff_limit, compare_stderr = parse_args(argv[1:])
    py_cmd = [sys.executable, "-S", str(PY_REF), *sample]
    rs_cmd = ["cargo", "run", "--quiet", "--features", "rust-tool-bins", "--bin", "rgenerate4readme", "--", *sample]
    py = run(py_cmd, timeout=timeout, env=python_env())
    rs = run(rs_cmd, timeout=timeout)

    all_ok = True
    py_rc = returncode_of(py)
    rs_rc = returncode_of(rs)
    if py_rc != rs_rc:
        all_ok = False
        print(f"RETURN-CODE DIFF python={py_rc} rust={rs_rc}")

    if stdout_of(py) == stdout_of(rs):
        print(f"stdout exact match ({len(stdout_of(py))} bytes)")
    else:
        all_ok = False
        print("STDOUT DIFF:")
        diff = list(
            difflib.unified_diff(
                stdout_of(py).splitlines(True),
                stdout_of(rs).splitlines(True),
                fromfile="python.stdout",
                tofile="rust.stdout",
            )
        )
        print("".join(diff[:diff_limit]), end="")
        if len(diff) > diff_limit:
            print(f"\n... diff abgeschnitten nach {diff_limit} Zeilen ...")

    if compare_stderr:
        if stderr_of(py) == stderr_of(rs):
            print("stderr exact match")
        else:
            all_ok = False
            print("STDERR DIFF:")
            diff = list(
                difflib.unified_diff(
                    stderr_of(py).splitlines(True),
                    stderr_of(rs).splitlines(True),
                    fromfile="python.stderr",
                    tofile="rust.stderr",
                )
            )
            print("".join(diff[:diff_limit]), end="")
            if len(diff) > diff_limit:
                print(f"\n... stderr-diff abgeschnitten nach {diff_limit} Zeilen ...")
    elif stderr_of(py) or stderr_of(rs):
        print(
            "stderr: nicht bitverglichen "
            f"(python={len(stderr_of(py))} bytes, rust={len(stderr_of(rs))} bytes; nutze --compare-stderr für harten Diff)"
        )

    return 0 if all_ok else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
