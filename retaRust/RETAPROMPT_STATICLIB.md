# retaPrompt static library layer

This project now contains an additive shared retaPrompt library layer inside the existing `reta`
crate static library. It does **not** introduce a second `reta` library and leaves the existing
`reta` runtime library untouched.

## Central shared layer

Rust modules:

- `src/prompt/frontend_profile.rs`
- `src/prompt/retapromptlib.rs`

The shared retaPrompt layer centralizes the maximal common frontend behavior for:

- `rp`
- `rpl`
- `rpb`
- `rpe`

## Frontend defaults

- `rp`: vi mode, no implicit logging, interactive
- `rpl`: vi mode, implicit logging, interactive
- `rpb`: vi mode, no implicit logging, one-shot
- `rpe`: emacs mode, no implicit logging, interactive

## Stable Rust entry points

- `reta::prompt::run_retaprompt_rp_from_env()`
- `reta::prompt::run_retaprompt_rpl_from_env()`
- `reta::prompt::run_retaprompt_rpb_from_env()`
- `reta::prompt::run_retaprompt_rpe_from_env()`
- `reta::prompt::run_retaprompt_with_kind(argv, kind)`
- `reta::prompt::run_retaprompt_with_profile(argv, profile)`

## Exported C ABI symbols inside the existing static library

- `retaprompt_run_rp_from_env`
- `retaprompt_run_rpl_from_env`
- `retaprompt_run_rpb_from_env`
- `retaprompt_run_rpe_from_env`

These symbols are exported from the package static library artifact produced by the existing
`[lib] crate-type = ["rlib", "staticlib"]` configuration.

## Intent

This is additive. Old code paths are preserved and delegate forward where useful. The change
creates a real shared retaPrompt frontend library layer without creating a second `reta` library
crate and without deleting existing runtime code.
