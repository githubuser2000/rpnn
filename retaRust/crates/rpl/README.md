# rpl

Dedicated additive static library package for the retaPrompt `rpl` frontend.

This package builds a dedicated `librpl.a` artifact and forwards into the
shared `retaprompt` layer, which itself delegates to the shared `reta::prompt`
implementation.

## Exported Rust API

- `run_rpl(argv)`
- `run_rpl_from_env()`

## Exported C ABI symbol

- `rpl_run_from_env`

## Build

```bash
cargo build -p rpl --lib
```
