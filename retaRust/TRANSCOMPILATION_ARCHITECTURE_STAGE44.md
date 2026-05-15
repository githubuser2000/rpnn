# Transcompilation Architecture Stage 44

Stage 44 extends the guarded table-view activation path with a replayable hash-chained activation ledger.

## Main addition

New module:

```text
crates/reta_architecture/src/table_view_activation_ledger.rs
```

It adds:

```text
TableViewActivationLedgerPolicy
TableViewActivationLedgerEntry
TableViewActivationLedgerValidation
TableViewActivationLedger
TableViewActivationLedgerSnapshot
TableViewActivationLedgerBundle
activation_ledger_for_cli_args
activation_ledger_from_journal
activation_ledger_entries_from_records
activation_ledger_entry_record_hash
activation_ledger_entry_chain_hash
validate_activation_ledger_entries
continuum_m_activation_ledger_smoke
```

## Purpose

Stage 42 introduced activation journals. Stage 43 added guarded replay and rollback. Stage 44 turns those journal records into a deterministic ledger chain:

```text
ActivationTransaction
  -> ActivationJournalRecord
  -> ActivationLedgerEntry
  -> hash chain validation
  -> guarded replay witness
```

The ledger validates:

```text
contiguous sequence numbers
previous hash points to the prior entry
record hash matches the entry fields
chain hash matches previous-chain + record hash
latest replay remains safe
```

## Root integration

`src/reta_arch_shadow.rs` now carries:

```text
view_output_ledger: Option<TableViewActivationLedger>
```

`src/reta_workflow_py.rs` now emits:

```text
ARCH_TABLE_VIEW_ACTIVATION_LEDGER
```

The visible output remains guarded. If the ledger is valid and its replay is safe, the replay-selected lines can be used; otherwise the existing replay/transaction/shadow/legacy fallback order remains.

## New interfaces

New binary:

```text
rreta_arch_activation_ledger
```

New FFI export:

```text
reta_architecture_table_view_activation_ledger_json
```

New probe:

```text
tools/architecture_activation_ledger_probe.py
```

## Migration gates

New runtime-switch morphisms:

```text
table_view_activation_ledger.hash_chain
table_view_activation_ledger.validate_chain
table_view_activation_ledger.replay_latest_safe_record
table_view_activation_ledger.rollback_on_chain_drift
```

New migration step:

```text
step-table-view-activation-ledger
```

## Safety invariant

The ledger does not loosen commit rules. It adds a stronger witness:

```text
A table-view activation may be replayed only when the journal records form one valid hash chain and the latest replay remains safe.
```

The known `493`/`744` case remains unchanged:

```text
493 -> direct CSV-backed M-Kontinuum cell
744 -> virtual / non-direct witness cell
```

## Checks performed in this environment

Passed:

```text
architecture_activation_ledger_probe.py
architecture_activation_replay_probe.py
architecture_activation_journal_probe.py
architecture_activation_transaction_probe.py
architecture_commit_audit_probe.py
architecture_virtual_commit_guard_probe.py
architecture_virtual_parity_probe.py
architecture_table_view_output_parity_probe.py
architecture_table_view_output_commit_probe.py
architecture_module_coverage.py
architecture_semantic_surface_audit.py
TOML parse checks
archive creation and read check
```

Not performed here:

```text
cargo check
cargo test
full workspace build
```

Reason: `cargo` / `rustc` are not available in this container shell.
