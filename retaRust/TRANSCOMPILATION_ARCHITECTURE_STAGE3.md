# Transcompilation Architecture Stage 3

## Goal

Continue the migration from the monolithic Python reference toward the modular
`py reta arch` source by moving two high-value architecture owners into Rust:

- `reta_architecture/tag_schema.py`
- `reta_architecture/row_ranges.py` together with `input_semantics.RowRangeSyntax`

The visible Reta output path remains compatibility-first.  The new Rust modules
are used as architecture anchors and are safe to call from both `rreta` and
`rretaPrompt`.

## Implemented

### `crates/reta_architecture/src/tag_schema.rs`

Adds a typed Rust tag-schema owner for the Python `ST` enum and tag tables.
The generated tables preserve Python `dictViceversa` behavior: when duplicate
columns occur in several tag groups, the later Python dictionary group wins in
the reverse lookup.

Important parity checks included in Rust tests:

- Python enum values `sternPolygon = 0` through `gebrRat = 6`
- ordinary column `14` keeps the effective Python reverse tags
  `{sternPolygon, galaxie}`
- ordinary column `744` maps to `{keinParaOdMetaP, sternPolygon}`
- ordinary column `370` maps to `{keinParaOdMetaP, sternPolygon, galaxie}`
- table-count snapshot matches Python `TagSchemaBundle.snapshot()`

### `crates/reta_architecture/src/row_ranges.rs`

Adds a Rust row-range morphism bundle for the Stage-37 Python architecture
module.  It includes:

- `RowRangeSyntax`
- comma splitting outside `[]`, `{}`, `()`
- integer and fraction token recognition
- explicit set/list/tuple integer literals
- subtractive ranges
- `v`-prefixed multiples
- plus-offset expansion
- `RowRangeMorphismBundle`

### Shared enum facade now uses the architecture table

`src/shared/lib4tables_enum_py.rs` now delegates ordinary/kombi tag reverse
lookups to `reta_architecture::tag_schema` instead of the older generated
`reta_exact_tags_py` match table.  The old generated file stays in the tree as
a compatibility fallback/reference and is marked `allow(dead_code)`.

## Checks performed in this environment

Rust/Cargo are present here now:

- `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`
- `rustc 1.95.0 (59807616e 2026-04-14)`

A real Cargo build still cannot complete here because the environment cannot
resolve `index.crates.io` and therefore cannot download missing dependencies.
The failure is currently dependency-fetching only, not a compiler error in this
stage.

Local offline checks with temporary dependency stubs passed:

- `cargo test --offline` for isolated `reta_architecture`: 13 tests passed
- `cargo check -p reta --lib --offline` with stubs: passed
- `cargo check -p retaprompt_commands --lib --offline` with stubs: passed

These stubs are not included in the delivered source archive.  They were only
used to catch syntax/type-integration errors while the network is unavailable.

## Changed files

- `crates/reta_architecture/src/facade.rs`
- `crates/reta_architecture/src/lib.rs`
- `crates/reta_architecture/src/row_ranges.rs`
- `crates/reta_architecture/src/tag_schema.rs`
- `src/shared/lib4tables_enum_py.rs`
- `src/shared/reta_exact_tags_py.rs`

## Next stage

The next useful Rust transcompilation targets are:

1. `parameter_runtime.py` -> typed CLI parameter resolution
2. `column_selection.py` and `table_state.py` -> typed table-state capsule
3. `table_generation.py` / `table_preparation.py` -> Rust generation pipeline
4. `prompt_language.py` / `completion_runtime.py` -> deeper `rretaPrompt` parity
