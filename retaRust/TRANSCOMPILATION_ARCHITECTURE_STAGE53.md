# Transcompilation Architecture Stage 53

Stage 53 fixes the `migration_control.rs` compile error reported from Termux:

```text
error[E0061]: this function takes 7 arguments but 8 arguments were supplied
crates/reta_architecture/src/migration_control.rs:400:9
```

## Fix

The migration entry `step-table-view-activation-readiness` now matches the seven-argument `step(...)` helper signature again.

The new activation-readiness CLI-policy morphism is preserved:

```text
table_view_activation_readiness.policy_from_cli
```

but the accidental extra string argument was removed from the `step(...)` call.

## New guard probe

Added:

```text
tools/architecture_migration_step_arity_probe.py
```

It verifies that every `step(...)` call in:

```text
crates/reta_architecture/src/migration_control.rs
```

has exactly seven arguments. It also checks that the activation-readiness migration step still contains:

```text
table_view_activation_readiness.policy_from_cli
table_view_activation_readiness.default_promotion_gate
all_local_activation_witnesses_must_glue_before_default_visible_promotion
```

## Checks run in this environment

Passed:

```text
python3 tools/architecture_migration_step_arity_probe.py --pretty
python3 tools/architecture_activation_readiness_policy_probe.py --pretty
python3 tools/architecture_activation_readiness_probe.py --pretty
python3 tools/architecture_recovery_linkage_probe.py --pretty
python3 tools/architecture_commit_audit_probe.py --pretty
python3 tools/architecture_table_view_output_parity_probe.py --pretty
python3 tools/architecture_table_view_output_commit_probe.py --pretty
python3 tools/architecture_module_coverage.py --pretty
python3 tools/architecture_semantic_surface_audit.py --pretty
```

Cargo was not available in this container, so the full workspace build was not run here.

## Local recommended checks

```bash
cargo check -p reta_architecture
cargo test -p reta_architecture
cargo check --bin rreta_arch_activation_readiness
python3 tools/architecture_migration_step_arity_probe.py --pretty
python3 tools/architecture_activation_readiness_policy_probe.py --pretty
```
