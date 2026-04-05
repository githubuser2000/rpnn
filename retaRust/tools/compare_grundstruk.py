#!/usr/bin/env python3
from pathlib import Path
import subprocess
import sys
import difflib

root = Path(__file__).resolve().parents[1]
py_ref = root / "python_reference" / "grundStrukHtml.py"

def run_python(blank: bool) -> str:
    args = [sys.executable, str(py_ref)]
    if blank:
        args.append("blank")
    return subprocess.check_output(args, text=True)

def run_rust(blank: bool) -> str:
    args = ["cargo", "run", "--quiet", "--bin", "grundStrukHtml"]
    if blank:
        args.extend(["--", "blank"])
    return subprocess.check_output(args, cwd=root, text=True)

for blank in (False, True):
    py_out = run_python(blank)
    rs_out = run_rust(blank)
    label = "blank" if blank else "normal"
    if py_out == rs_out:
        print(f"{label}: exact match")
    else:
        print(f"{label}: DIFFERENT")
        diff = difflib.unified_diff(
            py_out.splitlines(True),
            rs_out.splitlines(True),
            fromfile="python",
            tofile="rust",
        )
        print("".join(list(diff)[:200]))
