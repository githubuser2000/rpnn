# Transcompilation Architecture Stage 62

Stage 62 continues from Stage 61.

## Summary

- Added `crates/reta_architecture/src/table_view_language_sync.rs`.
- Synchronized `python_arch_reference/csv/en-religion.csv`, `cn-religion.csv`, `vn-religion.csv`, and `kr-religion.csv` to the 745-column religion data already present in the root CSV mirror.
- Regenerated `crates/reta_architecture/src/csv_catalog.rs`, so translated religion assets now report direct column `744`.
- Updated Rust tests and Python probes that previously expected stale 744-column language variants.
- Added `rreta_arch_language_sync` and `reta_architecture_table_view_language_sync_json`.
- Added the runtime diagnostic `ARCH_TABLE_VIEW_LANGUAGE_SYNC`.

## Current 493/744 state

`-spalten --kontinuum=m` still resolves to `493, 744`.

- `493` remains direct CSV-backed: `M Kontinuum (dreizehn)`.
- `744` is now direct CSV-backed in base and language religion mirrors: `Neues M (13) Kontinuum`.
- Virtual-column diagnostics continue to use non-direct column `999`.

## Checks

Probe status: `17/17` probes reported `ok`.

Coverage remains:

- Functions represented by name: `1096/1096`
- Classes represented by name: `239/239`
- Strict semantic surface marker-only: `0 functions, 0 classes`
- Missing strict semantic surface: `0 functions, 0 classes`

## Cargo

A full workspace Cargo build was not run in this container because `cargo`/`rustc` were unavailable here. Run locally:

```bash
cargo check -p reta_architecture
cargo test -p reta_architecture
cargo check --bin rreta_arch_language_sync
cargo check -p reta --lib
cargo check -p retaprompt_commands --lib
cargo check -p retaprompt_input --lib
cargo check -p retaprompt_frontends
python3 tools/architecture_language_sync_probe.py --pretty
```
