# RUST_STAGE18_CHECKS

Date: 2026-05-14

## Environment

```text
cargo 1.95.0
rustc 1.95.0
python 3.11.8
```

## Full workspace check

Command attempted from `retaRust`:

```bash
cargo check -p reta_architecture
```

Result: not completed because this container cannot resolve crates.io.

Observed blocker:

```text
Could not resolve host: index.crates.io
failed to get `hypher` as a dependency of package `reta v0.6.0`
```

## Isolated architecture-crate check

Because external crates were unavailable, `crates/reta_architecture` was copied outside the workspace and checked with local `serde`/`serde_derive` stubs. This avoids the root workspace dependencies (`hypher`, `indexmap`, `libloading`, `serde_json`) and verifies the architecture crate syntax/types/tests.

Commands:

```bash
cargo check --offline
cargo test --offline --lib -- --test-threads=1
```

Results:

```text
cargo check --offline: passed
cargo test --offline --lib: 122 passed, 0 failed
```

New Stage-18 targeted tests passed:

```text
parameter_matrix::generated_matrix_contains_current_744_regression
parameter_matrix::generated_matrix_preserves_symbolic_bucket_payloads
parameter_runtime::column_alias_matrix_preserves_legacy_bucket_coordinates
parameter_runtime::symbolic_bucket_negation_removes_matching_local_sections
table_generation::generation_plan_carries_symbolic_parameter_buckets
```

## Python and generator checks

```bash
python3 tools/generate_parameter_matrix.py
python3 -m py_compile \
  tools/generate_parameter_matrix.py \
  tools/architecture_module_coverage.py \
  tools/architecture_semantic_surface_audit.py \
  tools/architecture_shadow_probe.py \
  tools/architecture_commit_probe.py \
  tools/architecture_prompt_commit_probe.py
```

Results:

```text
parameter_matrix generator deterministic: passed
Python tool py_compile: passed
```

## Coverage audits

```text
architecture_module_coverage: functions 1096/1096, classes 239/239, modules 59
architecture_semantic_surface_audit: functions declared 1096/1096, marker-only 0, missing 0
architecture_semantic_surface_audit: classes declared 239/239, marker-only 0, missing 0
```
