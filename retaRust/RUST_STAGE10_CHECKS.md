# RUST_STAGE10_CHECKS

## Toolchain
- cargo: missing
- rustc: missing

## Static checks
- TOML parse: `Cargo.toml` ok
- TOML parse: `crates/reta_architecture/Cargo.toml` ok
- TOML parse: `crates/retaprompt_commands/Cargo.toml` ok
- TOML parse: `crates/retaprompt_input/Cargo.toml` ok
- shadow_pipeline module file: ok
- shadow_pipeline lib.rs export: ok
- ShadowPipeline in facade: ok
- root shadow bridge module: ok
- root lib export: ok
- run_reta shadow diagnostic hook: ok
- FFI shadow cli export: ok
- FFI prompt shadow export: ok
- inspect binary target: ok
- probe tool: ok
- Python probe py_compile: ok

## Cargo build
- not run: cargo/rustc are unavailable in this container shell.

## Result
- static check status: ok
