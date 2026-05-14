# Rust Architecture Stage 20 Checks

## Performed

- `python3 -m py_compile` for the new and existing architecture probe/generator tools.
- `tools/architecture_table_materialization_probe.py --pretty`.
- `tools/architecture_module_coverage.py --pretty` and `--only-missing`.
- `tools/architecture_semantic_surface_audit.py --pretty` and `--only-marker-or-missing`.
- Isolated `reta_architecture` cargo check with local serde/serde_derive stubs:
  - `cargo check --manifest-path crates/reta_architecture/Cargo.toml --offline`
  - `cargo test --manifest-path crates/reta_architecture/Cargo.toml --offline --lib`

## Results

- Isolated `reta_architecture` check: passed.
- Isolated `reta_architecture` tests: 132 passed, 0 failed.
- Stage-20 materialization probe: ok.
- Coverage audit: 1096/1096 functions and 239/239 classes represented.
- Strict semantic-surface audit: 1096/1096 functions and 239/239 classes declared; 0 marker-only; 0 missing.

## Workspace check

A full workspace build was attempted with the real dependency graph, but crates.io name resolution is still blocked in this container:

```text
Could not resolve host: index.crates.io
failed to get `hypher`
```

So the full workspace build should still be run locally with normal network/cache access.
