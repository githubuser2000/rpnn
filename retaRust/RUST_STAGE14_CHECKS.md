# Rust Stage 14 checks

## Environment

`cargo` and `rustc` are not available in this container shell, so a full Cargo build/test run was not possible here.

## Performed checks

```text
Cargo.toml parse ok
crates/reta_architecture/Cargo.toml parse ok
crates/retaprompt_frontends/Cargo.toml parse ok
Python probe tools py_compile ok
stage14_coverage_full.json parse ok
stage14_coverage_missing.json parse ok
```

## Coverage audit

```text
Stage 13 functions: 486 / 1096
Stage 14 functions: 772 / 1096
Stage 13 classes:   180 / 239
Stage 14 classes:   200 / 239
```

## Important limitation

The coverage audit checks whether Python architecture function/class names are represented in the matching Rust module. It is useful for steering the transcompilation, but it is not a semantic or byte-exact parity proof.

The next local checks on a machine with Rust installed should be:

```bash
cargo check -p reta_architecture
cargo test -p reta_architecture
cargo check -p reta --lib
cargo check -p retaprompt_commands --lib
cargo check -p retaprompt_input --lib
cargo check -p retaprompt_frontends
```
