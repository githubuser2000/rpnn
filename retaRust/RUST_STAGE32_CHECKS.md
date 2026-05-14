# RUST_STAGE32_CHECKS

## Scope

Stage 32 adds the `table_view_row_styles` Rust module and wires it into the materialized `TableViewOutput` path.  It ports the legacy `coloredBeginCol` row-colour semantics as a disabled-by-default, policy-controlled output wrapper projection.

## Checks performed

- `python3 -m py_compile` for the new and regression probe tools: passed
- `tools/architecture_row_style_probe.py --pretty`: passed
- `tools/architecture_html_output_probe.py --pretty`: passed
- `tools/architecture_layout_probe.py --pretty`: passed
- `tools/architecture_numbering_probe.py --pretty`: passed
- `tools/architecture_output_flags_probe.py --pretty`: passed
- `tools/architecture_column_order_probe.py --pretty`: passed
- `tools/architecture_row_order_probe.py --pretty`: passed
- `tools/architecture_table_view_output_parity_probe.py --pretty`: passed
- `tools/architecture_table_view_output_commit_probe.py --pretty`: passed
- Architecture coverage audit: 1096 / 1096 functions, 239 / 239 classes, 0 missing
- Strict semantic-surface audit: 0 marker-only, 0 missing
- Isolated `reta_architecture` cargo check with local serde/serde_derive stubs: passed
- Isolated `reta_architecture` cargo test with local serde/serde_derive stubs: 178 passed, 0 failed
- Patch whitespace check: no trailing whitespace in added lines
- Archive generated and test-read: passed

## Not fully performed here

A real full-workspace cargo check could not be completed in this container because crates.io dependency resolution is unavailable offline:

```text
error: no matching package named `indexmap` found
location searched: crates.io index
required by package `reta v0.6.0`
```

The user-side Termux build has previously compiled the workspace; Stage 32 should still be tested there with the real dependency cache/toolchain.

## Suggested local commands

```bash
tar -xjf reta_rust_arch_stage32.tar.bz2
cd retaRust
cargo check -p reta_architecture
cargo test -p reta_architecture
cargo check -p reta --lib
cargo check -p retaprompt_commands --lib
cargo check -p retaprompt_input --lib
cargo check -p retaprompt_frontends
python3 tools/architecture_row_style_probe.py --pretty
cargo run --bin rreta_arch_row_styles -- \
  reta -zeilen --vorhervonausschnitt=1-1 \
  -spalten --kontinuum=m \
  -ausgabe --art=html --rowcolors
```
