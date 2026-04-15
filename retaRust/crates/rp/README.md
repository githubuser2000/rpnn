# rp

Dedicated additive static library package for the retaPrompt `rp` frontend.

This package builds a dedicated `librp.a` artifact and forwards into the
shared `retaprompt` layer, which itself delegates to the shared `reta::prompt`
implementation.

## Exported Rust API

- `run_rp(argv)`
- `run_rp_from_env()`

## Exported C ABI symbol

- `rp_run_from_env`

## Build

```bash
cargo build -p rp --lib
```
