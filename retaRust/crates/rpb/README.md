# rpb

Dedicated additive static library package for the retaPrompt `rpb` frontend.

This package builds a dedicated `librpb.a` artifact and forwards into the
shared `retaprompt` layer, which itself delegates to the shared `reta::prompt`
implementation.

## Exported Rust API

- `run_rpb(argv)`
- `run_rpb_from_env()`

## Exported C ABI symbol

- `rpb_run_from_env`

## Build

```bash
cargo build -p rpb --lib
```
