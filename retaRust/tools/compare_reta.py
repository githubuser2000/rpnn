#!/usr/bin/env python3
"""Bitgenauer Vergleich zwischen gebündelter Python-Referenz und Rust-reta.

Python bleibt die Wahrheit. Dieses Werkzeug läuft dieselben argv-Samples gegen
`python_reference/reta.py` und `cargo run --bin reta` und diffed die Ausgaben.
"""
from __future__ import annotations

from dataclasses import dataclass
import difflib
import os
from pathlib import Path
import shutil
import subprocess
import sys
from typing import Iterable

ROOT = Path(__file__).resolve().parents[1]
PY_REF = ROOT / "python_reference" / "reta.py"
PY_REF_DIR = ROOT / "python_reference"
PY_REF_LIBS = PY_REF_DIR / "libs"

DEFAULT_SAMPLES: list[list[str]] = [
    ["-nichts"],
    ["-spalten", "--religion"],
    ["-spalten", "--religion=sternpolygon"],
    ["-kombi", "--galaxie=tiere"],
]


@dataclass(frozen=True)
class RunResult:
    label: str
    argv: list[str]
    command: list[str]
    returncode: int
    stdout: str
    stderr: str
    timed_out: bool = False


def _python_env() -> dict[str, str]:
    env = os.environ.copy()
    path_parts = [str(PY_REF_DIR), str(PY_REF_LIBS), str(ROOT)]
    if env.get("PYTHONPATH"):
        path_parts.append(env["PYTHONPATH"])
    env["PYTHONPATH"] = os.pathsep.join(path_parts)
    return env


def _run(label: str, command: list[str], argv: list[str], *, timeout: float) -> RunResult:
    try:
        completed = subprocess.run(
            command,
            cwd=ROOT,
            env=_python_env() if label == "python" else os.environ.copy(),
            text=True,
            capture_output=True,
            timeout=timeout,
        )
        return RunResult(
            label=label,
            argv=argv,
            command=command,
            returncode=completed.returncode,
            stdout=completed.stdout,
            stderr=completed.stderr,
        )
    except subprocess.TimeoutExpired as exc:
        return RunResult(
            label=label,
            argv=argv,
            command=command,
            returncode=124,
            stdout=exc.stdout or "",
            stderr=exc.stderr or "",
            timed_out=True,
        )


def run_python(sample: list[str], *, timeout: float) -> RunResult:
    # `-S` hält externe sitecustomize-/venv-Nebeneffekte aus dem Referenzlauf.
    # Die gebündelte Referenz bringt die benötigten Stubs unter python_reference/ mit.
    return _run("python", [sys.executable, "-S", str(PY_REF), *sample], sample, timeout=timeout)


def run_rust(sample: list[str], *, timeout: float) -> RunResult:
    return _run(
        "rust",
        ["cargo", "run", "--quiet", "--bin", "reta", "--", *sample],
        sample,
        timeout=timeout,
    )


def unified_diff(left: str, right: str, *, fromfile: str, tofile: str, limit: int) -> str:
    lines = list(
        difflib.unified_diff(
            left.splitlines(True),
            right.splitlines(True),
            fromfile=fromfile,
            tofile=tofile,
        )
    )
    if len(lines) > limit:
        return "".join(lines[:limit]) + f"\n... diff abgeschnitten nach {limit} Zeilen ...\n"
    return "".join(lines)


def print_command(prefix: str, result: RunResult) -> None:
    cmd = " ".join(subprocess.list2cmdline([part]) for part in result.command)
    print(f"{prefix}: {cmd}")


def compare_one(sample: list[str], *, timeout: float, compare_stderr: bool, diff_limit: int) -> bool:
    print(f"\nARGS: {sample!r}")
    py = run_python(sample, timeout=timeout)
    rs = run_rust(sample, timeout=timeout)
    print_command("PY", py)
    print_command("RS", rs)

    ok = True
    if py.timed_out or rs.timed_out:
        ok = False
        if py.timed_out:
            print(f"PYTHON TIMEOUT nach {timeout:g}s")
        if rs.timed_out:
            print(f"RUST TIMEOUT nach {timeout:g}s")

    if py.returncode != rs.returncode:
        ok = False
        print(f"RETURN-CODE DIFF: python={py.returncode} rust={rs.returncode}")

    if py.stdout != rs.stdout:
        ok = False
        print("STDOUT DIFF:")
        print(unified_diff(py.stdout, rs.stdout, fromfile="python.stdout", tofile="rust.stdout", limit=diff_limit))
    else:
        print("stdout: exact match")

    if compare_stderr:
        if py.stderr != rs.stderr:
            ok = False
            print("STDERR DIFF:")
            print(unified_diff(py.stderr, rs.stderr, fromfile="python.stderr", tofile="rust.stderr", limit=diff_limit))
        else:
            print("stderr: exact match")
    elif py.stderr or rs.stderr:
        print(
            "stderr: nicht in den Bitvergleich einbezogen "
            f"(python={len(py.stderr)} bytes, rust={len(rs.stderr)} bytes; nutze --compare-stderr für harten Diff)"
        )

    return ok


def parse_args(argv: list[str]) -> tuple[list[list[str]], float, bool, int]:
    args = list(argv)
    compare_stderr = False
    timeout = float(os.environ.get("RETA_COMPARE_TIMEOUT", "120"))
    diff_limit = int(os.environ.get("RETA_COMPARE_DIFF_LINES", "200"))

    sample_args: list[str] = []
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
            sample_args.extend(args[i + 1 :])
            break
        else:
            sample_args.append(arg)
            i += 1

    samples = [sample_args] if sample_args else DEFAULT_SAMPLES
    return samples, timeout, compare_stderr, diff_limit


def main(argv: list[str]) -> int:
    if not PY_REF.exists():
        print(f"Python-Referenz fehlt: {PY_REF}", file=sys.stderr)
        return 2
    if not (PY_REF_DIR / "csv" / "religion.csv").exists():
        print(
            "Python-Referenz ist nicht lauffähig: python_reference/csv/religion.csv fehlt",
            file=sys.stderr,
        )
        return 2
    if shutil.which("cargo") is None:
        print("cargo nicht gefunden; Rust-Vergleich kann hier nicht laufen.", file=sys.stderr)
        return 2

    samples, timeout, compare_stderr, diff_limit = parse_args(argv[1:])
    all_ok = True
    for sample in samples:
        all_ok = compare_one(
            sample,
            timeout=timeout,
            compare_stderr=compare_stderr,
            diff_limit=diff_limit,
        ) and all_ok
    return 0 if all_ok else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
