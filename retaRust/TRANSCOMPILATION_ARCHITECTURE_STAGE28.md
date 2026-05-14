# Transcompilation Architecture Stage 28

Stage 28 builds on the successful Stage 27 workspace build reported from Termux.

## Focus

`table_view_output` now owns the next visible-output knobs from `py reta arch`:

- `--keineueberschriften`
- `--keineleereninhalte`
- `--breite=...`
- `--breiten=...`
- `--dontwrap`
- `--nocolor`
- `--justtext`
- `--onetable`
- `--endlessscreen`
- `--endless`
- explicit `--keinenummerierung` target

The old visible renderer remains the behaviour oracle. These options are now represented in the Rust shadow/diagnostic path so future commits can compare them against legacy output before switching.

## Key Rust additions

- `TableViewOutputCliOptions`
- `parse_table_view_output_cli_options`
- `filtered_output_rows`
- `rendered_row_value_lines`
- `expand_row_to_value_lines`
- `row_values_with_options`
- `wrap_output_cell`
- `output_flags_smoke`

## Integration

- `ShadowCliPlan` now uses `render_cli_args` for `table_view_output` so CLI output flags are applied.
- `RetaRunArchitecture::from_cli_args` now routes its architecture output report through the CLI-aware renderer.
- Runtime gates added:
  - `table_view_output.output_flags`
  - `table_view_output.width_wrapping`
  - `table_view_output.header_filter`
- Migration step added:
  - `step-table-view-output-flags`
- New FFI export:
  - `reta_architecture_table_view_output_options_json`
- New inspect binary:
  - `rreta_arch_output_flags`

## Warning fix

The unused `limit_set` helper in `table_materialization.rs` was removed. This addresses the warning you saw after the Stage 27 build.

## Checks

- isolated `cargo check -p reta_architecture --offline` with local serde stubs: passed
- isolated `cargo test -p reta_architecture --offline --lib` with local serde stubs: 158 passed, 0 failed
- `architecture_output_flags_probe.py`: passed
- column-order, row-order, parity and commit probes: passed
- coverage: 1096 / 1096 functions and 239 / 239 classes
- strict semantic surface: 0 marker-only, 0 missing

I did not claim a full workspace build in this container because crates.io dependencies are still not cached/resolvable here. Your Termux build already proved the real dependency path for Stage 27.
