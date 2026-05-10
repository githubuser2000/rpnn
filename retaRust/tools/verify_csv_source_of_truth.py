#!/usr/bin/env python3
"""Verify that retaRust CSV files come from canonical reta.py `reta/csv`.

This guard intentionally treats `reta/csv` as the source of truth and reports
when checked-in Rust CSV data silently matches the known-drifting `reta.arch/csv`
instead.  It can be run in two modes:

  python tools/verify_csv_source_of_truth.py
      verifies csv/ and python_reference/csv against the checked-in manifest

  python tools/verify_csv_source_of_truth.py --canonical ../reta/csv --arch ../reta.arch/csv
      additionally compares checked-in CSV files against external source trees
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "tools" / "reta_csv_source_manifest.tsv"
CHECKED_DIRS = (ROOT / "csv", ROOT / "python_reference" / "csv")


def fnv1a64(data: bytes) -> int:
    value = 0xCBF29CE484222325
    for byte in data:
        value ^= byte
        value = (value * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return value


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load_manifest(path: Path = MANIFEST) -> dict[str, tuple[int, str, str]]:
    entries: dict[str, tuple[int, str, str]] = {}
    with path.open("r", encoding="utf-8", newline="") as handle:
        rows = (line for line in handle if line.strip() and not line.startswith("#"))
        reader = csv.DictReader(rows, delimiter="\t")
        for row in reader:
            entries[row["file"]] = (int(row["bytes"]), row["fnv1a64"], row["sha256"])
    return entries


def verify_against_manifest(csv_dir: Path, manifest: dict[str, tuple[int, str, str]]) -> list[str]:
    errors: list[str] = []
    actual_names = {path.name for path in csv_dir.glob("*.csv")}
    expected_names = set(manifest)
    for name in sorted(expected_names - actual_names):
        errors.append(f"{csv_dir}: missing {name}")
    for name in sorted(actual_names - expected_names):
        errors.append(f"{csv_dir}: unexpected {name}")
    for name in sorted(expected_names & actual_names):
        path = csv_dir / name
        data = path.read_bytes()
        expected_len, expected_fnv, expected_sha = manifest[name]
        actual_fnv = f"{fnv1a64(data):016x}"
        actual_sha = sha256_hex(data)
        if len(data) != expected_len or actual_fnv != expected_fnv or actual_sha != expected_sha:
            errors.append(
                f"{path}: differs from reta/csv manifest "
                f"(bytes {len(data)} != {expected_len}, "
                f"fnv {actual_fnv} != {expected_fnv}, sha256 {actual_sha} != {expected_sha})"
            )
    return errors


def compare_external_source(
    csv_dir: Path,
    canonical: Path | None,
    arch: Path | None,
    manifest: dict[str, tuple[int, str, str]],
) -> list[str]:
    errors: list[str] = []
    if canonical is not None:
        for name in sorted(manifest):
            checked = csv_dir / name
            reference = canonical / name
            if not reference.is_file():
                errors.append(f"canonical source missing {reference}")
                continue
            if checked.read_bytes() != reference.read_bytes():
                errors.append(f"{checked}: does not match canonical reta/csv/{name}")
    if arch is not None:
        arch_matches: list[str] = []
        for name in sorted(manifest):
            checked = csv_dir / name
            arch_file = arch / name
            canonical_file = canonical / name if canonical is not None else None
            if not arch_file.is_file():
                continue
            if checked.read_bytes() == arch_file.read_bytes() and (
                canonical_file is None or checked.read_bytes() != canonical_file.read_bytes()
            ):
                arch_matches.append(name)
        if arch_matches:
            errors.append(
                f"{csv_dir}: {len(arch_matches)} files match reta.arch/csv while differing from reta/csv: "
                + ", ".join(arch_matches)
            )
    return errors


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--canonical", type=Path, help="path to canonical reta/csv")
    parser.add_argument("--arch", type=Path, help="optional path to reta.arch/csv for drift detection")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    manifest = load_manifest()
    errors: list[str] = []
    for csv_dir in CHECKED_DIRS:
        errors.extend(verify_against_manifest(csv_dir, manifest))
        errors.extend(compare_external_source(csv_dir, args.canonical, args.arch, manifest))
    if errors:
        print("CSV source-of-truth check failed:", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1
    print("CSV source-of-truth check passed: csv/ and python_reference/csv match canonical reta/csv manifest.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
