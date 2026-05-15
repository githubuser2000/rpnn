#!/usr/bin/env python3
"""Static Stage-44 probe for hash-chained activation ledgers."""
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

    ledger = read("crates/reta_architecture/src/table_view_activation_ledger.rs")
    replay = read("crates/reta_architecture/src/table_view_activation_replay.rs")
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
    binary = read("src/bin/reta_arch_activation_ledger.rs")

    checks = {
        "module_file_present": "TableViewActivationLedger" in ledger,
        "policy_present": "TableViewActivationLedgerPolicy" in ledger,
        "entry_present": "TableViewActivationLedgerEntry" in ledger,
        "validation_present": "TableViewActivationLedgerValidation" in ledger,
        "hash_chain_functions_present": "activation_ledger_entry_record_hash" in ledger and "activation_ledger_entry_chain_hash" in ledger,
        "chain_validates_previous_hash": "entry.previous_chain_hash != previous_chain_hash" in ledger,
        "chain_validates_record_hash": "entry.record_hash != expected_record_hash" in ledger,
        "chain_validates_chain_hash": "entry.chain_hash != expected_chain_hash" in ledger,
        "sequence_guard_present": "sequence_numbers_not_contiguous" in ledger,
        "latest_replay_guard_present": "latest_replay_not_safe" in ledger,
        "continuum_smoke_present": "continuum_m_activation_ledger_smoke" in ledger,
        "tamper_tests_present": "activation_ledger_detects_hash_chain_tampering" in ledger and "activation_ledger_detects_sequence_drift" in ledger,
        "depends_on_journal": "activation_journal_for_cli_args" in ledger and "TableViewActivationJournal" in ledger,
        "depends_on_replay": "activation_replay_from_journal" in ledger and "TableViewActivationReplayReport" in ledger,
        "journal_still_has_replay_checksum": "stable_line_checksum(&record.selected_lines) == record.selected_lines_checksum" in journal,
        "replay_still_guards_current_transaction": "latest_transaction_id_does_not_match_current_transaction" in replay,
        "lib_exports_ledger": "pub mod table_view_activation_ledger" in lib and "activation_ledger_for_cli_args" in lib,
        "facade_runtime_contains_ledger": "table_view_activation_ledger" in facade and "rust_table_view_activation_ledger_morphism_count" in facade,
        "root_bridge_carries_ledger": "view_output_ledger" in shadow_bridge and "activation_ledger_from_journal" in shadow_bridge,
        "workflow_reports_ledger": "ARCH_TABLE_VIEW_ACTIVATION_LEDGER" in workflow,
        "workflow_uses_ledger_before_replay": "ledger.validation.is_ready() && ledger.replay_visible_output" in workflow,
        "shadow_pipeline_mentions_ledger": "table_view_activation_ledger.hash_chain" in shadow_pipeline,
        "runtime_gates_present": "table_view_activation_ledger.hash_chain" in runtime_switch and "table_view_activation_ledger.validate_chain" in runtime_switch,
        "migration_step_present": "step-table-view-activation-ledger" in migration,
        "ffi_export_present": "reta_architecture_table_view_activation_ledger_json" in ffi,
        "binary_registered": "rreta_arch_activation_ledger" in cargo and "reta_arch_activation_ledger.rs" in cargo,
        "binary_reads_legacy_lines": "--legacy-lines-file" in binary and "activation_ledger_for_cli_args" in binary,
    }
    status = "ok" if all(checks.values()) else "failed"
    report = {
        "stage": 44,
        "status": status,
        "checks": checks,
        "required_ledger_guards": [
            "contiguous_sequence_numbers",
            "previous_hash_points_to_prior_entry",
            "entry_hash_matches_record_fields",
            "latest_replay_report_is_safe",
        ],
        "universal_property": "activation journal records glue into one replayable ledger only when every local record hash composes with the previous chain hash and the latest replay remains safe.",
    }
    print(json.dumps(report, indent=2 if args.pretty else None, ensure_ascii=False, sort_keys=True))
    return 0 if status == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
