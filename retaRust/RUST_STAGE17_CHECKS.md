# Rust Stage 17 Checks

Performed in this container:

```text
Cargo.toml parse: ok
crates/reta_architecture/Cargo.toml parse: ok
crates/retaprompt_frontends/Cargo.toml parse: ok
Python tools py_compile: ok
architecture_module_coverage.py: 1096/1096 functions, 239/239 classes, missing 0
architecture_semantic_surface_audit.py: marker-only 0, missing 0
parameter matrix generator determinism: ok
parameter matrix seed count: 429
unique integer column projections: 642
kontinuum/m -> 493,744: present
```

Not performed:

```text
cargo check
cargo test
full workspace build
```

Reason:

```text
cargo: missing
rustc: missing
```

The generated Stage 17 code adds Rust unit tests for the new parameter-matrix, schema, parameter runtime, input semantics, completion runtime, nested completion, sheaf, and semantics-builder paths. They should be run locally with real Cargo.
