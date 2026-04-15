# retaPrompt static library layer

This project keeps the existing `reta` crate and adds an additive retaPrompt layer on top.

The important native-linking distinction is now:

- `libreta.a` = heavy implementation base
- `libretaprompt.a` = small retaPrompt ABI forwarding archive

That means `libretaprompt.a` must not be produced by Rust `crate-type = ["staticlib"]`, because that would bundle `reta` again and duplicate `libreta.a`.
Instead, the retaPrompt ABI implementation is exported from `libreta.a` under prefixed C symbols, and `tools/build_retaprompt_staticlib.sh` builds a tiny forwarding `libretaprompt.a` on top.

## Shared Rust prompt code

The real prompt behavior remains centralized in the existing Rust prompt code:

- `src/prompt/frontend_profile.rs`
- `src/prompt/retapromptlib.rs`
- `src/prompt/mod.rs`

The additive Rust package layer lives here:

- `crates/retaprompt/Cargo.toml`
- `crates/retaprompt/src/lib.rs`
- `crates/retaprompt/include/retaprompt.h`
- `crates/retaprompt/src/retaprompt_shim.c`
- `crates/retaprompt_frontends/Cargo.toml`
- `crates/retaprompt_frontends/src/bin/rp.rs`
- `crates/retaprompt_frontends/src/bin/rpl.rs`
- `crates/retaprompt_frontends/src/bin/rpb.rs`
- `crates/retaprompt_frontends/src/bin/rpe.rs`

## Frontend defaults

- `rp`: vi mode, no implicit logging, interactive
- `rpl`: vi mode, implicit logging, interactive
- `rpb`: vi mode, no implicit logging, one-shot
- `rpe`: emacs mode, no implicit logging, interactive

## Stable Rust entry points

From the main crate public API:

- `reta::prompt::PromptFrontendKind`
- `reta::prompt::PromptFrontendProfile`
- `reta::prompt::run_retaprompt_rp_from_env()`
- `reta::prompt::run_retaprompt_rpl_from_env()`
- `reta::prompt::run_retaprompt_rpb_from_env()`
- `reta::prompt::run_retaprompt_rpe_from_env()`
- `reta::prompt::run_retaprompt_auto_from_env()`
- `reta::prompt::run_retaprompt_with_kind(argv, kind)`
- `reta::prompt::run_retaprompt_with_profile(argv, profile)`

From the additive `retaprompt` Rust crate:

- `retaprompt::run_rp_from_env()`
- `retaprompt::run_rpl_from_env()`
- `retaprompt::run_rpb_from_env()`
- `retaprompt::run_rpe_from_env()`
- `retaprompt::run_auto_from_env()`

## Native ABI split

`libreta.a` exports the implementation symbols:

- `reta_retaprompt_run_kind_from_env`
- `reta_retaprompt_run_auto_from_env`
- `reta_retaprompt_run_rp_from_env`
- `reta_retaprompt_run_rpl_from_env`
- `reta_retaprompt_run_rpb_from_env`
- `reta_retaprompt_run_rpe_from_env`

`libretaprompt.a` exports the public retaPrompt ABI symbols and forwards to the symbols above:

- `retaprompt_run_kind_from_env`
- `retaprompt_run_auto_from_env`
- `retaprompt_run_rp_from_env`
- `retaprompt_run_rpl_from_env`
- `retaprompt_run_rpb_from_env`
- `retaprompt_run_rpe_from_env`

## Build

Build both archives with the dedicated helper:

```bash
./tools/build_retaprompt_staticlib.sh debug
./tools/build_retaprompt_staticlib.sh release
```

Correct native link model:

```text
... libretaprompt.a libreta.a ...
```
