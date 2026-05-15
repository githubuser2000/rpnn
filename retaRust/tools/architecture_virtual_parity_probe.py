#!/usr/bin/env python3
"""Stage-38 probe for virtual-column parity diagnostics.

The Rust architecture may render virtual/non-direct columns such as the
continuum `744` witness in shadow/inspect modes.  This probe checks that the new
parity layer is present and that its contract is explicit: rendering virtual
columns may add witness cells, but must not mutate direct CSV-backed cells such
as the `493` M-Kontinuum column.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    parity_path = ROOT / "crates/reta_architecture/src/table_view_virtual_parity.rs"
    parity = parity_path.read_text(encoding="utf-8") if parity_path.exists() else ""
    lib = read("crates/reta_architecture/src/lib.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    runtime = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    binary = ROOT / "src/bin/reta_arch_virtual_parity.rs"

    checks = {
        "module_exists": parity_path.exists(),
        "bundle_declared": "pub struct TableViewVirtualParityBundle" in parity,
        "config_declared": "pub struct TableViewVirtualParityConfig" in parity,
        "report_declared": "pub struct TableViewVirtualParityReport" in parity,
        "direct_signature_declared": "pub struct DirectCellSignature" in parity,
        "virtual_signature_declared": "pub struct VirtualCellSignature" in parity,
        "direct_cells_compare_present": "direct_cells_equal" in parity and "direct_cell_signatures" in parity,
        "raw_and_semantic_diff_present": "raw_lines_equal" in parity and "semantic_diff" in parity,
        "744_added_only_invariant_present": "continuum_m_virtual_744_added_only" in parity,
        "493_preserved_invariant_present": "continuum_m_direct_493_preserved" in parity,
        "smoke_test_present": "continuum_m_virtual_policy_adds_744_without_touching_493" in parity,
        "suppress_vs_suppress_test_present": "suppress_vs_suppress_is_raw_equal" in parity,
        "lib_exports_module": "pub mod table_view_virtual_parity" in lib and "TableViewVirtualParityReport" in lib,
        "facade_runtime_contains_bundle": "pub table_view_virtual_parity: TableViewVirtualParityBundle" in facade,
        "facade_snapshot_counts_present": "rust_table_view_virtual_parity_morphism_count" in facade,
        "runtime_gates_present": all(name in runtime for name in [
            "table_view_virtual_parity.direct_cell_identity",
            "table_view_virtual_parity.added_virtual_only",
            "table_view_virtual_parity.raw_commit_guard",
        ]),
        "migration_step_present": "step-table-view-virtual-parity" in migration,
        "ffi_export_present": "reta_architecture_table_view_virtual_parity_json" in ffi,
        "inspect_binary_present": "rreta_arch_virtual_parity" in cargo and binary.exists(),
    }
    result = {
        "stage": 38,
        "status": "ok" if all(checks.values()) else "failed",
        "checks": checks,
        "reference_policy": "suppress",
        "rendered_policy": "tag-summary",
        "known_regression_case": "-spalten --kontinuum=m -ausgabe --spaltenreihenfolgeundnurdiese=744,493",
        "expected_direct_column": 493,
        "expected_virtual_column": 744,
        "invariant": (
            "Virtual-column rendering is a local policy morphism: it may add virtual witness "
            "cells, but it must be identity on direct CSV-backed cells before any commit gate."
        ),
    }
    print(json.dumps(result, ensure_ascii=False, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if result["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
