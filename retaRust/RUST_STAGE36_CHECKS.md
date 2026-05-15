# Rust Stage 36 Checks

## Environment

```text
cargo 1.95.0
rustc 1.95.0
```

## Full workspace check

A full workspace check was attempted from the delivered project root:

```bash
cargo check -p reta_architecture
```

It did not reach project-code compilation because this container cannot resolve crates.io:

```text
Could not resolve host: index.crates.io
failed to get `hypher`
```

## Isolated architecture-crate check

To still type-check the changed `reta_architecture` code, I copied the architecture crate and `python_arch_reference` into a temporary workspace and used local minimal `serde` / `serde_derive` stubs. That avoids the root workspace's unavailable external dependencies while still checking Rust syntax, module wiring and tests for `reta_architecture`.

```bash
cargo check -p reta_architecture --offline
cargo test  -p reta_architecture --offline --lib
```

Result:

```text
cargo check: passed
cargo test: 193 passed, 0 failed
```

## Python probes

Passed:

```text
architecture_shell_style_probe.py
architecture_style_parity_probe.py
architecture_style_composition_probe.py
architecture_cell_style_probe.py
architecture_row_style_probe.py
architecture_html_output_probe.py
architecture_layout_probe.py
architecture_numbering_probe.py
architecture_output_flags_probe.py
architecture_column_order_probe.py
architecture_row_order_probe.py
architecture_table_view_output_parity_probe.py
architecture_table_view_output_commit_probe.py
architecture_module_coverage.py
architecture_semantic_surface_audit.py
```

## Audits

```text
Coverage: 1096 / 1096 functions, 239 / 239 classes, 0 missing
Strict semantic surface: 1096 declared functions, 0 marker-only, 0 missing; 239 declared classes, 0 marker-only, 0 missing
```
