#!/usr/bin/env python3
"""Probe Stage 62 language-sync witness wiring and the religion 744 backlog."""
from __future__ import annotations

import argparse
import csv
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read_csv(path: Path):
    with path.open("r", encoding="utf-8", newline="") as handle:
        return list(csv.reader(handle, delimiter=";"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    base = read_csv(ROOT / "csv" / "religion.csv")
    variants = {
        "en": ROOT / "csv" / "en-religion.csv",
        "cn": ROOT / "csv" / "cn-religion.csv",
        "vn": ROOT / "csv" / "vn-religion.csv",
        "kr": ROOT / "csv" / "kr-religion.csv",
    }
    variant_widths = {lang: len(read_csv(path)[0]) for lang, path in variants.items()}
    base_width = len(base[0])
    base_744_header = base[0][744] if base_width > 744 else ""
    base_744_first_value = base[1][744] if len(base) > 1 and len(base[1]) > 744 else ""

    sync_rs = (ROOT / "crates/reta_architecture/src/table_view_language_sync.rs").read_text(encoding="utf-8")
    lib_rs = (ROOT / "crates/reta_architecture/src/lib.rs").read_text(encoding="utf-8")
    shadow_rs = (ROOT / "crates/reta_architecture/src/shadow_pipeline.rs").read_text(encoding="utf-8")
    workflow_rs = (ROOT / "src/reta_workflow_py.rs").read_text(encoding="utf-8")
    ffi_rs = (ROOT / "src/ffi.rs").read_text(encoding="utf-8")
    cargo_toml = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    migration_rs = (ROOT / "crates/reta_architecture/src/migration_control.rs").read_text(encoding="utf-8")
    runtime_switch_rs = (ROOT / "crates/reta_architecture/src/runtime_switch.rs").read_text(encoding="utf-8")

    expected_pending = [lang for lang, width in variant_widths.items() if width <= 744]
    checks = {
        "base_religion_has_745_columns": base_width == 745,
        "base_744_header_is_neues_m": "Neues M" in base_744_header,
        "base_744_first_value_is_identity_wave": "Identität" in base_744_first_value and "Welle" in base_744_first_value,
        "variants_synced_for_744": expected_pending == [],
        "sync_module_exists": "TableViewLanguageSyncReport" in sync_rs,
        "sync_module_projects_base_payload": "project_base_column_payload" in sync_rs and "preview_csv_cell" in sync_rs,
        "sync_module_has_strict_policy": "require_no_pending_actions_for_ready" in sync_rs,
        "lib_exports_sync": "pub mod table_view_language_sync" in lib_rs and "language_sync_for_cli_args" in lib_rs,
        "shadow_cli_plan_carries_sync": "pub language_sync: TableViewLanguageSyncReport" in shadow_rs,
        "workflow_diagnostic_present": "ARCH_TABLE_VIEW_LANGUAGE_SYNC" in workflow_rs,
        "ffi_export_present": "reta_architecture_table_view_language_sync_json" in ffi_rs,
        "inspect_binary_registered": "rreta_arch_language_sync" in cargo_toml,
        "migration_step_present": "step-table-view-language-sync" in migration_rs,
        "runtime_switch_morphisms_present": "table_view_language_sync.translation_backlog_report" in runtime_switch_rs,
    }
    status = "ok" if all(checks.values()) else "failed"
    result = {
        "status": status,
        "checks": checks,
        "base_width": base_width,
        "variant_widths": variant_widths,
        "expected_pending_languages_for_744": expected_pending,
        "base_744_header_preview": base_744_header[:96],
        "base_744_first_value_preview": base_744_first_value[:96],
    }
    print(json.dumps(result, indent=2 if args.pretty else None, ensure_ascii=False))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
