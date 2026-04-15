# retaprompt

Additive shared Rust library for the retaPrompt frontend layer.

This crate is the shared Rust-facing layer for `rp`, `rpl`, `rpb`, and `rpe`, while the native `libretaprompt.a` artifact is intentionally built as a tiny forwarding archive by `tools/build_retaprompt_staticlib.sh`.
That avoids embedding a second copy of `libreta.a`.

## Exposed Rust entry points

- `run_rp(argv)`
- `run_rpl(argv)`
- `run_rpb(argv)`
- `run_rpe(argv)`
- `run_auto_from_env()`
- `run_rp_from_env()`
- `run_rpl_from_env()`
- `run_rpb_from_env()`
- `run_rpe_from_env()`
- `run_with_kind(argv, kind)`
- `run_with_profile(argv, profile)`

The crate imports the stable public `reta::prompt::*` re-exports instead of reaching into deeper prompt internals directly.

## Native ABI layering

The actual implementation is exported from `libreta.a` under prefixed C symbols:

- `reta_retaprompt_run_kind_from_env`
- `reta_retaprompt_run_auto_from_env`
- `reta_retaprompt_run_rp_from_env`
- `reta_retaprompt_run_rpl_from_env`
- `reta_retaprompt_run_rpb_from_env`
- `reta_retaprompt_run_rpe_from_env`

The public retaPrompt ABI stays:

- `retaprompt_run_kind_from_env`
- `retaprompt_run_auto_from_env`
- `retaprompt_run_rp_from_env`
- `retaprompt_run_rpl_from_env`
- `retaprompt_run_rpb_from_env`
- `retaprompt_run_rpe_from_env`

Those public symbols are emitted by the tiny forwarding `libretaprompt.a` wrapper archive.

ABI kind values for `retaprompt_run_kind_from_env`:

- `0` = auto
- `1` = rp
- `2` = rpl
- `3` = rpb
- `4` = rpe

## Build

Build the additive Rust crate:

```bash
cargo build -p retaprompt --lib
```

Build the tiny native forwarding archive:

```bash
./tools/build_retaprompt_staticlib.sh debug
```
