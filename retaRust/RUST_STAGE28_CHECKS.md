# Rust Stage 28 Checks

## Build context

The user reported that Stage 27 built successfully in Termux with only one warning:

```text
warning: function `limit_set` is never used
```

Stage 28 removes that unused function.

## Checks performed here

Because this container cannot resolve/download the full external crates.io dependency set used by the whole workspace, I checked the architecture crate in an isolated temporary workspace with local `serde` / `serde_derive` stubs.

```text
cargo check -p reta_architecture --offline: passed
cargo test  -p reta_architecture --offline --lib: 158 passed, 0 failed
```

## Probe checks

```text
architecture_output_flags_probe.py: passed
architecture_column_order_probe.py: passed
architecture_row_order_probe.py: passed
architecture_table_view_output_parity_probe.py: passed
architecture_table_view_output_commit_probe.py: passed
architecture_module_coverage.py: passed
architecture_semantic_surface_audit.py: passed
```

## Coverage

```text
functions represented: 1096 / 1096
classes represented:   239 / 239
missing:                 0
```

## Strict semantic surface

```text
functions declared:     1096 / 1096
functions marker-only:     0
functions missing:         0
classes declared:        239 / 239
classes marker-only:       0
classes missing:           0
```

## Important note

This is not a claim that every Rust output flag is already byte-exact with the legacy renderer. Stage 28 makes the flags explicit, typed, inspectable and shadow-comparable. Visible output still remains protected by the existing shadow/commit gates.
