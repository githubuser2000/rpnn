# Transcompilation Architecture Stage 43

Stage 43 adds a guarded activation replay layer above the Stage-42 activation journal.

## New Rust module

```text
crates/reta_architecture/src/table_view_activation_replay.rs
```

The new module introduces:

```text
TableViewActivationReplayPolicy
TableViewActivationReplayReport
TableViewActivationReplaySnapshot
TableViewActivationReplayBundle
activation_replay_for_cli_args
activation_replay_from_journal
continuum_m_activation_replay_smoke
```

## Purpose

Stage 42 made the activation decision replayable as a journal. Stage 43 makes replay guarded against drift.

A journal replay may use Rust `TableViewOutput` lines only if these guards pass:

```text
journal is replayable
latest transaction id matches the current transaction id
latest legacy checksum matches the current legacy checksum
embedded selected-lines checksum still matches the embedded selected lines
```

If any guard fails, the replay report rolls back to the current legacy lines.

## Integration

Updated:

```text
crates/reta_architecture/src/lib.rs
crates/reta_architecture/src/facade.rs
crates/reta_architecture/src/runtime_switch.rs
crates/reta_architecture/src/migration_control.rs
src/reta_arch_shadow.rs
src/reta_workflow_py.rs
src/ffi.rs
Cargo.toml
```

New binary:

```text
src/bin/reta_arch_activation_replay.rs
```

New probe:

```text
tools/architecture_activation_replay_probe.py
```

## New diagnostics

`rreta` can now emit:

```text
ARCH_TABLE_VIEW_ACTIVATION_REPLAY
```

This reports whether the activation replay selected Rust view-output lines or rolled back to legacy output.

## New FFI function

```text
reta_architecture_table_view_activation_replay_json
```

## New runtime gates

```text
table_view_activation_replay.guard_journal_replay
table_view_activation_replay.match_transaction_id
table_view_activation_replay.match_legacy_checksum
table_view_activation_replay.rollback_to_legacy_lines
```

## New migration step

```text
step-table-view-activation-replay
```

## Safety invariant

The `493` / `744` path remains protected:

```text
493 -> direct CSV-backed M-Kontinuum cell
744 -> virtual / non-direct witness cell
```

Stage 43 does not change materialization, virtual-column rendering, or commit rules. It only adds a stricter replay guard after journaling.
