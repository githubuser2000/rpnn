# Transcompilation Architecture Stage 32

## Scope

Stage 32 adds a policy-controlled row-style projection for materialized table-view output.
It connects the legacy `coloredBeginCol` semantics from `python_arch_reference/reta_architecture/output_syntax.py` to the Rust `TableViewOutput` path.

The projection remains disabled by default.  It can be activated for shadow/diagnostic output with:

- `--rowcolors`
- `--zeilenfarben`
- `--rowcolorwitness`
- `--zeilenfarbenwitness`

and is disabled again by `--nocolor`.

## New Rust module

- `crates/reta_architecture/src/table_view_row_styles.rs`

New typed surfaces:

- `TableViewRowStylePolicy`
- `TableViewRowStyleConfig`
- `TableViewRowStyle`
- `TableViewRowStyleReport`
- `TableViewRowStyleSnapshot`
- `TableViewRowStyleBundle`

Main morphisms:

- `bootstrap_table_view_row_styles`
- `row_style_for_row`
- `row_style_for_source_row`
- `row_style_report_for_rows`
- `styled_begin_row_for_row`
- `continuum_m_row_style_smoke`

## Integration

Stage 32 wires row styles into:

- `TableViewOutputConfig`
- `TableViewOutputReport`
- HTML rendering
- BBCode rendering
- CLI option parsing
- `ArchitectureRuntime`
- `RuntimeSwitchBundle`
- `MigrationControlBundle`
- FFI
- a new inspect binary

New FFI:

- `reta_architecture_table_view_row_styles_json`

New binary:

- `rreta_arch_row_styles`

Example:

```bash
cargo run --bin rreta_arch_row_styles -- \
  reta \
  -zeilen --vorhervonausschnitt=1-1 \
  -spalten --kontinuum=m \
  -ausgabe --art=html --rowcolors
```

## Safety invariant

The default is still plain output.  Row colouring changes row wrappers only; it does not change table cells, materialized CSV sections, row order, column order, virtual column witnesses, or parity/commit rules.

## Stage-specific regression

The known `kontinuum=m` path remains protected:

- `493` remains directly materialized from `religion.csv`.
- `744` remains a virtual/non-direct witness unless an explicit virtual-column display policy renders it.
- Row styles apply only to the row wrapper for HTML/BBCode.

