# Rust Stage 16 Checks

## Tool availability

- cargo: available in this container
- rustc: available in this container

## Full workspace build

A real Cargo build/check using the project dependencies could not be completed because crates.io DNS resolution failed:

```text
Could not resolve host: index.crates.io
failed to get `hypher` as a dependency of package `reta v0.6.0`
```

## Isolated architecture crate check

To verify the changed architecture code despite the offline dependency blocker, the `reta_architecture` crate was copied into a temporary test workspace with local minimal `serde` / `serde_derive` stubs.

Commands conceptually run:

```bash
cargo check --offline
cargo test --offline
```

Result:

```text
cargo check --offline: passed
cargo test --offline: 108 passed, 0 failed
```

## Audits

Strict semantic surface audit:

```text
functions declared:     1096 / 1096
functions marker-only:     0 / 1096
functions missing:         0 / 1096
classes declared:       239 / 239
classes marker-only:      0 / 239
classes missing:          0 / 239
```

Normal module coverage audit:

```text
functions represented by name: 1096 / 1096
classes represented by name:    239 / 239
```

## Python tool checks

```text
python3 -m py_compile tools/architecture_module_coverage.py tools/architecture_semantic_surface_audit.py tools/architecture_shadow_probe.py tools/architecture_commit_probe.py tools/architecture_prompt_commit_probe.py
```

passed.
