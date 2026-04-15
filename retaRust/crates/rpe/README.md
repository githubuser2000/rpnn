# rpe

Dedicated additive static library package for the retaPrompt `rpe` frontend.

This package builds a dedicated `librpe.a` artifact and forwards into the
shared `retaprompt` layer, which itself delegates to the shared `reta::prompt`
implementation.

## Exported Rust API

- `run_rpe(argv)`
- `run_rpe_from_env()`

## Exported C ABI symbol

- `rpe_run_from_env`

## Build

```bash
cargo build -p rpe --lib
```
