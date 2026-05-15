# Stage 35 checks

## Environment

```text
cargo: not available in this container shell
rustc: not available in this container shell
```

A full workspace build was therefore not executed here. The previous user-side Termux build showed that the Stage 27+ workspace can build with real dependencies; Stage 35 should be checked there with the commands listed in the manifest.

## Executed checks

```text
Cargo.toml parse: passed
crates/reta_architecture/Cargo.toml parse: passed
crates/retaprompt_frontends/Cargo.toml parse: passed
Python probe tools py_compile: passed
Style parity probe: passed
Style composition regression probe: passed
Cell style regression probe: passed
Row style regression probe: passed
HTML output regression probe: passed
Layout regression probe: passed
Numbering regression probe: passed
Output flags regression probe: passed
Column order regression probe: passed
Row order regression probe: passed
Table view output parity probe: passed
Table view output commit probe: passed
Coverage audit: passed
Semantic surface audit: passed
Static lexer balance on changed Rust files: passed
Archive generated and test-read: passed
```

## Important safety invariant

```text
Style-aware normalization is diagnostic only.
raw_equal remains the commit-safe condition.
semantic_equal may explain a style-only mismatch, but it does not by itself permit visible output replacement.
```
