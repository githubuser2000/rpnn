# Transcompilation Architecture Stage 5

## Scope

Stage 5 moves the next concrete `py reta arch` modules into the shared Rust
architecture crate.  This stage focuses on the table rendering pipeline and the
first prompt word-completion morphism.

## New Rust modules

- `crates/reta_architecture/src/table_wrapping.rs`
  - transcompiled from `python_arch_reference/reta_architecture/table_wrapping.py`
  - owns `WrapType`, `TextWrapRuntime`, `TableWidthContext`, `alxwrap`, `wrap_cell_text`, and width selection.
- `crates/reta_architecture/src/row_filtering.rs`
  - transcompiled from `python_arch_reference/reta_architecture/row_filtering.py`
  - owns row-filter sets, range command morphisms, duplicate deletion, moon/sun, powers, prime multiples, ordinary multiples, neighbour inversion, and z/y position filters.
- `crates/reta_architecture/src/table_preparation.rs`
  - transcompiled from `python_arch_reference/reta_architecture/table_preparation.py`
  - owns display-line selection, cell wrapping, old/new column maps, generated tag gluing and main table preparation results.
- `crates/reta_architecture/src/table_output.rs`
  - transcompiled from `python_arch_reference/reta_architecture/table_output.py`
  - owns one-based column selection, max cell width detection, row width decisions, ANSI colorization, and deterministic prepared-table rendering for shell/csv/html/bbcode/emacs/markdown-like modes.
- `crates/reta_architecture/src/completion_word.rs`
  - transcompiled from `python_arch_reference/reta_architecture/completion_word.py`
  - owns prompt document prefix extraction, legacy word matching, completion candidate creation and word-completion snapshots.

## Runtime integration

`ArchitectureRuntime` now contains:

- `row_filtering`
- `table_wrapping`
- `table_preparation`
- `table_output`
- `completion_word`

`architecture_terms()` now exposes these names, and `ArchitectureSnapshotRef`
adds counts for row-filter families, table-preparation morphisms, table-output morphisms, and word-completion morphisms.

`PromptArchitectureContext` now records the word-completion sample count.  That
keeps `rretaPrompt` attached to the newly transcompiled prompt-completion layer
without changing visible prompt behaviour yet.

## Parity posture

The byte-exact legacy renderer is not removed in this stage.  Stage 5 creates
typed Rust sections for the same morphisms so the next stages can replace the
legacy path piece by piece while still comparing against Python output.

## Checks

- Real workspace cargo check is still blocked in this container because crates.io dependencies are not cached/resolvable here.
- `cargo 1.95.0` and `rustc 1.95.0` are installed.
- The isolated `reta_architecture` crate was checked with a local serde stub and all 49 architecture tests passed.
