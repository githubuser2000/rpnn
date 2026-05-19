#!/usr/bin/env python3
"""Vergleicht grundStrukHtml.py gegen den schlanken Rust/C-Launcher rgrundStrukHtml."""
from __future__ import annotations

import difflib
import os
from pathlib import Path
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
PY_REF = ROOT / "python_reference" / "grundStrukHtml.py"
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


def run_python(blank: bool, *, timeout: float):
    args = [sys.executable, "-S", str(PY_REF)]
    if blank:
        args.append("blank")
    return run(args, timeout=timeout, env=python_env())


def rust_launcher_name() -> str:
    return "rgrundStrukHtml.exe" if os.name == "nt" else "rgrundStrukHtml"


def rust_runtime_env() -> dict[str, str]:
    env = os.environ.copy()
    target_dirs = [ROOT / "target" / "debug", ROOT / "target" / "release"]
    for var in ("LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH", "PATH"):
        parts = [str(path) for path in target_dirs if path.exists()]
        if env.get(var):
            parts.append(env[var])
        if parts:
            env[var] = os.pathsep.join(parts)
    return env


def find_rust_launcher() -> Path | None:
    profile = os.environ.get("RETA_COMPARE_PROFILE")
    profiles = [profile] if profile else ["debug", "release"]
    for candidate_profile in profiles:
        if not candidate_profile:
            continue
        candidate = ROOT / "target" / candidate_profile / rust_launcher_name()
        if candidate.exists():
            return candidate
    return None


def prepare_rust_launcher() -> int:
    if find_rust_launcher() is not None:
        return 0
    build_script = ROOT / "build.sh"
    bash = shutil.which("bash")
    if not build_script.exists() or bash is None:
        print("build.sh oder bash fehlt; rgrundStrukHtml kann nicht gebaut werden.", file=sys.stderr)
        return 2
    completed = subprocess.run(
        [bash, str(build_script), os.environ.get("RETA_COMPARE_PROFILE", "debug")],
        cwd=ROOT,
        text=True,
        capture_output=True,
    )
    if completed.returncode != 0:
        print("rgrundStrukHtml konnte nicht über build.sh gebaut werden:", file=sys.stderr)
        if completed.stdout:
            print(completed.stdout, file=sys.stderr, end="")
        if completed.stderr:
            print(completed.stderr, file=sys.stderr, end="")
    return completed.returncode


def run_rust(blank: bool, *, timeout: float):
    launcher = find_rust_launcher()
    if launcher is None:
        raise RuntimeError("rgrundStrukHtml wurde nicht gebaut")
    args = [str(launcher)]
    if blank:
        args.append("blank")
    return run(args, timeout=timeout, env=rust_runtime_env())


def stdout_of(result) -> str:
    return result.stdout or ""


def stderr_of(result) -> str:
    return result.stderr or ""


def returncode_of(result) -> int:
    return 124 if isinstance(result, subprocess.TimeoutExpired) else result.returncode


def main() -> int:
    if not PY_REF.exists():
        print(f"Python-Referenz fehlt: {PY_REF}", file=sys.stderr)
        return 2
    if not (PY_REF_DIR / "csv" / "religion.csv").exists():
        print("python_reference/csv/religion.csv fehlt", file=sys.stderr)
        return 2
    build_rc = prepare_rust_launcher()
    if build_rc != 0:
        return build_rc

    timeout = float(os.environ.get("RETA_COMPARE_TIMEOUT", "120"))
    diff_limit = int(os.environ.get("RETA_COMPARE_DIFF_LINES", "200"))
    all_ok = True
    for blank in (False, True):
        py = run_python(blank, timeout=timeout)
        rs = run_rust(blank, timeout=timeout)
        label = "blank" if blank else "normal"
        py_rc = returncode_of(py)
        rs_rc = returncode_of(rs)
        if py_rc != rs_rc:
            all_ok = False
            print(f"{label}: RETURN-CODE DIFF python={py_rc} rust={rs_rc}")
        if stdout_of(py) == stdout_of(rs):
            print(f"{label}: stdout exact match")
        else:
            all_ok = False
            print(f"{label}: STDOUT DIFF")
            diff = list(
                difflib.unified_diff(
                    stdout_of(py).splitlines(True),
                    stdout_of(rs).splitlines(True),
                    fromfile="python.stdout",
                    tofile="rust.stdout",
                )
            )
            print("".join(diff[:diff_limit]))
            if len(diff) > diff_limit:
                print(f"... diff abgeschnitten nach {diff_limit} Zeilen ...")
        if stderr_of(py) or stderr_of(rs):
            print(
                f"{label}: stderr nicht bitverglichen "
                f"(python={len(stderr_of(py))} bytes, rust={len(stderr_of(rs))} bytes)"
            )
    return 0 if all_ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
