#!/usr/bin/env python3
"""Static Stage-42 probe for replayable table-view activation journals."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    journal = read("crates/reta_architecture/src/table_view_activation_journal.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    shadow_bridge = read("src/reta_arch_shadow.rs")
    workflow = read("src/reta_workflow_py.rs")
    shadow_pipeline = read("crates/reta_architecture/src/shadow_pipeline.rs")
    runtime_switch = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    binary = read("src/bin/reta_arch_activation_journal.rs")

    checks = {
        "module_file_present": "TableViewActivationJournal" in journal,
        "journal_record_present": "TableViewActivationJournalRecord" in journal,
        "replay_report_present": "TableViewActivationJournalReplayReport" in journal,
        "checksum_guard_present": "stable_line_checksum(&record.selected_lines) == record.selected_lines_checksum" in journal,
        "fallback_legacy_on_reject": "latest_record_rejected" in journal and "fallback_legacy_lines.to_vec()" in journal,
        "continuum_smoke_present": "continuum_m_activation_journal_smoke" in journal,
        "lib_exports_journal": "pub mod table_view_activation_journal" in lib and "activation_journal_for_cli_args" in lib,
        "facade_runtime_contains_journal": "table_view_activation_journal" in facade and "rust_table_view_activation_journal_morphism_count" in facade,
        "root_bridge_carries_journal": "view_output_journal" in shadow_bridge and "activation_journal_from_transactions" in shadow_bridge,
        "workflow_reports_journal": "ARCH_TABLE_VIEW_ACTIVATION_JOURNAL" in workflow,
        "shadow_pipeline_mentions_journal": "table_view_activation_journal.record_transaction" in shadow_pipeline,
        "runtime_gates_present": "table_view_activation_journal.record_transaction" in runtime_switch and "table_view_activation_journal.replay_selected_lines" in runtime_switch,
        "migration_step_present": "step-table-view-activation-journal" in migration,
        "ffi_export_present": "reta_architecture_table_view_activation_journal_json" in ffi,
        "binary_registered": "rreta_arch_activation_journal" in cargo and "reta_arch_activation_journal.rs" in cargo,
        "binary_replays_journal": "replay_activation_journal" in binary and "--legacy-lines-file" in binary,
    }
    status = "ok" if all(checks.values()) else "failed"
    report = {
        "stage": 42,
        "status": status,
        "checks": checks,
        "required_replay_guards": [
            "latest_record_is_safe",
            "selected_lines_are_present",
            "selected_lines_checksum_matches",
            "fallback_legacy_lines_are_available_on_reject",
        ],
        "universal_property": "activation records can replay visible Rust output only through a safe transaction whose embedded selected-lines checksum still matches; otherwise replay falls back to legacy output.",
    }
    print(json.dumps(report, indent=2 if args.pretty else None, ensure_ascii=False, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
