# RUST_STAGE43_CHECKS

Stage 43 checks performed in the container.

## Environment

- `cargo`: missing in this container shell
- `rustc`: missing in this container shell
- Full workspace build: not performed here

## Static checks

- `Cargo.toml` parse: passed
- `crates/reta_architecture/Cargo.toml` parse: passed
- `crates/retaprompt_frontends/Cargo.toml` parse: passed
- Static delimiter-balance check on changed Rust files: passed
- `tools/architecture_activation_replay_probe.py` py_compile: passed
- Regression probe scripts py_compile: passed
- No `__pycache__` / `.pyc` packaged into the project tree: passed

## Probe checks

- Activation replay probe: passed
- Activation journal regression probe: passed
- Activation transaction regression probe: passed
- Commit audit regression probe: passed
- Virtual commit guard regression probe: passed
- Virtual parity regression probe: passed
- Table-view-output parity regression probe: passed
- Table-view-output commit regression probe: passed

## Surface audits

- Architecture coverage: `1096 / 1096` functions, `239 / 239` classes
- Strict semantic surface: `0` marker-only, `0` missing

## Not performed here

- `cargo check -p reta_architecture`
- `cargo test -p reta_architecture`
- full workspace build

Reason: this container shell does not currently expose `cargo` or `rustc`.
