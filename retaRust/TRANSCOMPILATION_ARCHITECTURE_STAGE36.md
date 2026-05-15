# Transcompilation Architecture Stage 36

Stage 36 adds a policy-controlled shell/ANSI styling projection to the Rust architecture path.

## Main addition

New module:

```text
crates/reta_architecture/src/table_view_shell_styles.rs
```

This module connects the old shell `table_output.colorize` semantics to the materialized `TableView` output path without enabling it by default.

## New concepts

```text
TableViewShellStylePolicy
TableViewShellStyleConfig
TableViewShellStyle
TableViewShellStyleReport
TableViewShellStyleSnapshot
TableViewShellStyleBundle
```

Policies:

```text
Plain
LegacyColorize
LegacyColorizeWitness
```

## CLI / shadow flags

New Rust architecture flags recognized by `table_view_output`:

```text
--shellcolors
--shellcolor
--ansicolors
--ansicolor
--shellcolorwitness
--ansicolorwitness
```

`--nocolor` disables this shell projection again.

## Integration

Changed areas:

```text
TableViewOutputConfig.shell_styles
TableViewOutputReport.shell_style_* fields
render_shell_rows(...)
ArchitectureRuntime.table_view_shell_styles
ArchitectureSnapshotRef shell-style counters
RuntimeSwitch gates
MigrationControl step
FFI export
Inspect binary
Static probe tool
```

New runtime gates:

```text
table_view_shell_styles.legacy_colorize
table_view_shell_styles.ansi_cell_wrapper
table_view_shell_styles.strip_ansi_parity
```

New migration step:

```text
step-table-view-shell-styles
```

New FFI export:

```text
reta_architecture_table_view_shell_styles_json
```

New inspect binary:

```text
rreta_arch_shell_styles
```

## Safety invariant

Shell ANSI bytes are a local output projection. Cell values remain recoverable by stripping ANSI. The semantic parity layer already strips ANSI for diagnostics, but raw line equality remains the commit guard.

## Checks

The full workspace check was blocked in this container by crates.io DNS resolution. The architecture crate was checked in an isolated temporary workspace with local `serde` / `serde_derive` stubs:

```text
cargo check: passed
cargo test: 193 passed, 0 failed
```

Python probes and audits passed:

```text
coverage: 1096/1096 functions, 239/239 classes
strict semantic surface: 0 marker-only, 0 missing
```
