#!/usr/bin/env python3
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[1]
samples = [
    [],
    ["-spalten", "--religion"],
    ["-spalten", "--religion=sternpolygon"],
    ["-kombi", "--galaxie=tiere"],
]
for sample in samples:
    out = subprocess.check_output(
        ["cargo", "run", "--quiet", "--bin", "reta", "--", *sample],
        cwd=root,
        text=True,
    )
    print("ARGS:", sample)
    print(out)
