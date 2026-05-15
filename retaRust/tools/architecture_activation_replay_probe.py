#!/usr/bin/env python3
"""Static Stage-43 probe for guarded activation-journal replay."""
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

    replay = read("crates/reta_architecture/src/table_view_activation_replay.rs")
    journal = read("crates/reta_architecture/src/table_view_activation_journal.rs")
    lib = read("crates/reta_architecture/src/lib.rs")
    facade = read("crates/reta_architecture/src/facade.rs")
    shadow_bridge = read("src/reta_arch_shadow.rs")
    workflow = read("src/reta_workflow_py.rs")
    runtime_switch = read("crates/reta_architecture/src/runtime_switch.rs")
    migration = read("crates/reta_architecture/src/migration_control.rs")
    ffi = read("src/ffi.rs")
    cargo = read("Cargo.toml")
    binary = read("src/bin/reta_arch_activation_replay.rs")

    checks = {
        "module_file_present": "TableViewActivationReplayReport" in replay,
        "policy_present": "TableViewActivationReplayPolicy" in replay,
        "guards_current_transaction_id": "latest_transaction_id_does_not_match_current_transaction" in replay,
        "guards_current_legacy_checksum": "latest_legacy_checksum_does_not_match_current_legacy" in replay,
        "rolls_back_to_legacy": "rollback_to_legacy" in replay and "fallback_legacy_lines.to_vec()" in replay,
        "uses_journal_replay_report": "replay_activation_journal" in replay and "TableViewActivationJournalReplayReport" in replay,
        "continuum_smoke_present": "continuum_m_activation_replay_smoke" in replay,
        "journal_still_has_checksum_guard": "stable_line_checksum(&record.selected_lines) == record.selected_lines_checksum" in journal,
        "lib_exports_replay": "pub mod table_view_activation_replay" in lib and "activation_replay_for_cli_args" in lib,
        "facade_runtime_contains_replay": "table_view_activation_replay" in facade and "rust_table_view_activation_replay_morphism_count" in facade,
        "root_bridge_carries_replay": "view_output_replay" in shadow_bridge and "activation_replay_from_journal" in shadow_bridge,
        "workflow_reports_replay": "ARCH_TABLE_VIEW_ACTIVATION_REPLAY" in workflow,
        "workflow_uses_replay_before_transaction": "replay.replay_visible_output" in workflow and "committed_shadow_lines = Some(replay.selected_lines.clone())" in workflow,
        "runtime_gates_present": "table_view_activation_replay.guard_journal_replay" in runtime_switch and "table_view_activation_replay.match_legacy_checksum" in runtime_switch,
        "migration_step_present": "step-table-view-activation-replay" in migration,
        "ffi_export_present": "reta_architecture_table_view_activation_replay_json" in ffi,
        "binary_registered": "rreta_arch_activation_replay" in cargo and "reta_arch_activation_replay.rs" in cargo,
        "binary_reads_legacy_lines": "--legacy-lines-file" in binary and "activation_replay_for_cli_args" in binary,
    }
    status = "ok" if all(checks.values()) else "failed"
    report = {
        "stage": 43,
        "status": status,
        "checks": checks,
        "required_replay_guards": [
            "journal_is_replayable",
            "latest_transaction_id_matches_current_transaction",
            "latest_legacy_checksum_matches_current_legacy",
            "selected_lines_checksum_matches_embedded_lines",
        ],
        "rollback_condition": "on any journal, transaction-id, selected-lines checksum or current-legacy checksum drift, the replay report returns current legacy output instead of Rust view-output lines.",
        "universal_property": "journal replay is identity on safe activation records and rollback on transaction or checksum drift.",
    }
    print(json.dumps(report, indent=2 if args.pretty else None, ensure_ascii=False, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
