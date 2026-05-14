# Transcompilation Architecture Stage 25

Stage 25 adds a mode-aware semantic parity layer for the materialized table-view output path.

## New Rust module

- `crates/reta_architecture/src/table_view_output_parity.rs`

The module normalizes output lines into semantic row/cell presheaves for:

- shell
- csv
- html
- bbcode
- emacs
- markdown
- nichts

It keeps raw line equality as the only normal commit-safe rule, but adds diagnostic semantic equality so renderer mismatches can be classified as markup/spacing syntax noise versus actual cell mismatches.

## New public structures

- `TableViewOutputParityConfig`
- `NormalizedOutputLine`
- `NormalizedOutputReport`
- `TableViewOutputParityReport`
- `TableViewOutputParitySnapshot`
- `TableViewOutputParityBundle`

## New public functions

- `bootstrap_table_view_output_parity`
- `normalize_output_lines`
- `semantic_rows_from_lines`
- `compare_output_lines`
- `compare_table_view_output_to_legacy`
- `parse_line_as_cells`
- `canonicalize_cell`
- `strip_ansi_escape_sequences`

## Runtime integration

`ShadowTableViewOutputReport` now carries:

- raw strict diff
- semantic normalized diff

`ShadowTableViewOutputCommitDecision` now carries:

- `semantic_equal`

The visible output is still not switched by semantic equality. Commit remains guarded by raw equality unless force mode explicitly overrides.

## New gates / migration step

Runtime switch recognizes:

- `table_view_output.parity_normalize`
- `table_view_output.semantic_diff`
- `shadow_pipeline.table_view_output_semantic_diff`

Migration control includes:

- `step-table-view-output-parity`

## New FFI export

- `reta_architecture_table_view_output_parity_json(argc, argv, legacy_text)`

## New binary

- `rreta_arch_view_output_parity`

Example:

```bash
cargo run --bin rreta_arch_view_output_parity -- \
  --legacy-lines-file legacy.txt \
  -zeilen --vorhervonausschnitt=1-1 \
  -spalten --kontinuum=m \
  --breite=0
```

## Checks

Static and Python probes passed. A full Cargo build was not run in this container because `cargo` and `rustc` are not installed here.
