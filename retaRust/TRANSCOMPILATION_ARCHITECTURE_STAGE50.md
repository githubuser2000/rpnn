# Transcompilation Architecture Stage 50

Stage 50 fixes the Termux build error:

```text
error[E0433]: cannot find `table_view_activation_recovery` in `reta_architecture`
```

## Fix

Runtime callers no longer use:

```rust
reta_architecture::table_view_activation_recovery::...
```

They now use the stable root-level exports:

```rust
reta_architecture::TableViewActivationRecoveryPolicy
reta_architecture::TableViewActivationRecoveryReport
reta_architecture::activation_recovery_policy_from_cli_args
reta_architecture::activation_recovery_for_cli_args
```

`crates/reta_architecture/src/lib.rs` now explicitly carries the boundary in this form:

```rust
pub mod table_view_activation_recovery;
pub use self::table_view_activation_recovery::{ ... };
```

## Changed files

- TRANSCOMPILATION_ARCHITECTURE_STAGE50.json
- TRANSCOMPILATION_ARCHITECTURE_STAGE50.md
- crates/reta_architecture/src/lib.rs
- src/bin/reta_arch_activation_recovery.rs
- src/ffi.rs
- src/reta_arch_shadow.rs
- tools/architecture_recovery_linkage_probe.py

## Checks

- Recovery linkage probe: passed.
- Activation recovery/file/persistence/store/ledger/replay regression probes: passed.
- Commit audit and virtual parity regression probes: passed.
- Coverage audit: `1096 / 1096` functions, `239 / 239` classes.
- Strict semantic surface: `0` marker-only functions, `0` missing functions, `0` marker-only classes, `0` missing classes.
- Isolated `reta_architecture` cargo check with local serde stubs: passed.
- Isolated `reta_architecture` cargo test with local serde stubs: 227 passed, 0 failed.

## Full build note

The full workspace build was not completed in this container because external crates.io dependencies are not resolvable here. The reported recovery-symbol error is directly addressed.
