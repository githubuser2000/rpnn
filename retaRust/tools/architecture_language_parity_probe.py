#!/usr/bin/env python3
"""Stage 58 probe: language-aware TableView materialization parity.

The Stage 55 religion.csv update made base column 744 direct.  Stage 56/57 made
language fallback CLI-aware.  This probe checks the new Rust language-parity
witness: an English section may remain localized for column 493, but must use the synchronized English religion asset for `--kontinuum=m` because English religion now contains direct column 744.
"""
from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "crates" / "reta_architecture" / "src"


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace")


def csv_rows(path: Path) -> list[list[str]]:
    return list(csv.reader(path.read_text(encoding="utf-8", errors="replace").splitlines(), delimiter=";"))


def csv_max_columns(path: Path) -> int:
    rows = csv_rows(path)
    return max((len(row) for row in rows), default=0)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    module = read(SRC / "table_view_language_parity.rs")
    lib = read(SRC / "lib.rs")
    shadow = read(SRC / "shadow_pipeline.rs")
    runtime = read(SRC / "runtime_switch.rs")
    migration = read(SRC / "migration_control.rs")
    facade = read(SRC / "facade.rs")
    ffi = read(ROOT / "src" / "ffi.rs")
    workflow = read(ROOT / "src" / "reta_workflow_py.rs")
    cargo = read(ROOT / "Cargo.toml")
    bin_rs = read(ROOT / "src" / "bin" / "reta_arch_language_parity.rs")

    base = ROOT / "python_arch_reference" / "csv" / "religion.csv"
    en = ROOT / "python_arch_reference" / "csv" / "en-religion.csv"
    base_rows = csv_rows(base)
    en_rows = csv_rows(en)
    base_cols = csv_max_columns(base)
    en_cols = csv_max_columns(en)

    checks = {
        "module_declares_policy_and_report": "TableViewLanguageParityPolicy" in module
        and "TableViewLanguageParityReport" in module,
        "module_has_cli_runner": "language_parity_for_cli_args" in module
        and "TableMaterializationConfig::from_cli_args" in module,
        "module_checks_english_493_variant": "english_493_can_use_language_variant" in module,
        "module_checks_english_744_synced_variant": "english_kontinuum_m_can_use_synced_language_asset_for_direct_744" in module,
        "module_allows_no_fallback_after_sync": "disabling_language_fallback_is_ready_after_language_asset_sync" in module,
        "lib_reexports_language_parity": "pub mod table_view_language_parity" in lib
        and "TableViewLanguageParityReport" in lib
        and "language_parity_for_cli_args" in lib,
        "facade_exposes_runtime_bundle": "table_view_language_parity" in facade
        and "bootstrap_table_view_language_parity_impl" in facade,
        "shadow_cli_plan_carries_language_parity": "pub language_parity: TableViewLanguageParityReport" in shadow
        and "language_parity_for_cli_args" in shadow,
        "runtime_gates_present": "table_view_language_parity.base_fallback_guard" in runtime
        and "table_view_language_parity.direct_744_guard" in runtime,
        "migration_step_present": "step-table-view-language-parity" in migration,
        "ffi_export_present": "reta_architecture_table_view_language_parity_json" in ffi,
        "inspect_binary_present": "rreta_arch_language_parity" in cargo
        and "language_parity_for_cli_args" in bin_rs,
        "workflow_diagnostic_present": "ARCH_TABLE_VIEW_LANGUAGE_PARITY" in workflow
        and "report.language_parity.ready()" in workflow,
        "base_religion_has_direct_744": base_cols >= 746
        and "Neues M" in (base_rows[0][744] if base_cols > 744 else ""),
        "english_religion_has_direct_744": en_cols >= 745,
        "english_religion_can_direct_493": en_cols > 493
        and bool(en_rows[0][493]),
    }
    report = {
        "status": "ok" if all(checks.values()) else "failed",
        "stage": 58,
        "base_religion_max_columns": base_cols,
        "english_religion_max_columns": en_cols,
        "base_744_header": base_rows[0][744] if base_cols > 744 else None,
        "english_493_header_preview": en_rows[0][493][:80] if en_cols > 493 else None,
        "expected_english_493_effective_asset": "en-religion.csv",
        "expected_english_kontinuum_m_effective_asset": "en-religion.csv",
        "checks": checks,
    }
    print(json.dumps(report, ensure_ascii=False, indent=2 if args.pretty else None))
    raise SystemExit(0 if report["status"] == "ok" else 1)


if __name__ == "__main__":
    main()
