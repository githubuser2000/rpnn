#!/usr/bin/env python3
from __future__ import annotations

import csv
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def fail(message: str) -> None:
    print(message, file=sys.stderr)
    raise SystemExit(1)


def check_csv() -> None:
    for sub in ["csv", "python_reference/csv", "python_arch_reference/csv"]:
        d = ROOT / sub
        if not d.exists():
            continue
        for name in ["religion.csv", "cn-religion.csv", "en-religion.csv", "kr-religion.csv", "vn-religion.csv"]:
            path = d / name
            if not path.exists():
                continue
            with path.open("r", encoding="utf-8-sig", newline="") as handle:
                header = next(csv.reader(handle, delimiter=";"))
            if len(header) != 746:
                fail(f"{path}: expected 746 columns, got {len(header)}")
            if header[744] != "Neues M (13) Kontinuum" or header[745] != "alternative Größenordnungen":
                fail(f"{path}: wrong truth tail columns {header[744:746]!r}")


def check_matrices() -> None:
    files = [
        "src/shared/exact_i18n.rs",
        "src/shared/words_py.rs",
        "src/shared/words_python_like.rs",
        "crates/reta_architecture/src/parameter_matrix.rs",
    ]
    for rel in files:
        text = (ROOT / rel).read_text(encoding="utf-8")
        for old in ["{4, 21, 54, 197, 425}", "{30, 82, 425}", "columns: &[4, 21, 54, 197, 425]", "columns: &[30, 82, 425]"]:
            if old in text:
                fail(f"{rel}: stale matrix pattern {old}")
        if "745" not in text:
            fail(f"{rel}: missing column 745 mapping")


def check_renderer() -> None:
    text = (ROOT / "src/shared/reta_output_py.rs").read_text(encoding="utf-8")
    required = [
        "fn shell_width_zero_raw_mode_py(&self) -> bool",
        "fn shell_cell_measure_width_from_mode_py(cell: &str, raw_width_zero_mode: bool) -> usize",
        "fn shell_cell_parts_for_output_py(&self, cell: &str, width: usize) -> Vec<String>",
        "if self.breiteHasBeenOnceZero || self.shellRowsAmount == 0",
        "return vec![cell.replace('\\n', \"\")];",
        "Self::shell_cell_measure_width_from_mode_py(cell, raw_width_zero_mode)",
        "self.shell_cell_parts_for_output_py(cell, width)",
        "self.shell_cell_parts_for_output_py(cell, widths[col_idx])",
    ]
    for needle in required:
        if needle not in text:
            fail(f"renderer missing invariant: {needle}")
    if "cell.split('\\\\n')" in text:
        fail("renderer has a double-escaped newline char literal")


def check_combi_join() -> None:
    text = (ROOT / "crates/reta_architecture/src/combi_join.rs").read_text(encoding="utf-8")
    if ".filter(|cell| !cell.trim().is_empty())" in text:
        fail("combi_join remove_one_number still drops empty cells")
    if "remove_one_number_keeps_empty_cells_like_python" not in text:
        fail("combi_join empty-cell regression test missing")


def main() -> int:
    check_csv()
    check_matrices()
    check_renderer()
    check_combi_join()
    print("py reta truth output invariants passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
