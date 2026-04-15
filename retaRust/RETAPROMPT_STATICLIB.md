# retaPrompt static library layer

This project now contains a **dedicated** additive `retaprompt` package for the
shared retaPrompt frontend layer.

It does **not** introduce a second `reta` runtime implementation and does not
remove the existing `reta` crate. Instead, it adds a thin package on top of the
existing prompt/runtime code so `rp`, `rpl`, `rpb`, and `rpe` can be built as a
separate static library artifact.

## Central shared layer

The real prompt behavior remains centralized in the existing Rust prompt code:

- `src/prompt/frontend_profile.rs`
- `src/prompt/retapromptlib.rs`
- `src/prompt/mod.rs`

The additive package layer lives here:

- `crates/retaprompt/Cargo.toml`
- `crates/retaprompt/src/lib.rs`
- `crates/retaprompt/src/bin/rp.rs`
- `crates/retaprompt/src/bin/rpl.rs`
- `crates/retaprompt/src/bin/rpb.rs`
- `crates/retaprompt/src/bin/rpe.rs`
- `crates/retaprompt/include/retaprompt.h`

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

From the dedicated package:

- `retaprompt::run_rp_from_env()`
- `retaprompt::run_rpl_from_env()`
- `retaprompt::run_rpb_from_env()`
- `retaprompt::run_rpe_from_env()`
- `retaprompt::run_auto_from_env()`

## Exported C ABI symbols inside the dedicated static library

- `retaprompt_run_kind_from_env`
- `retaprompt_run_auto_from_env`
- `retaprompt_run_rp_from_env`
- `retaprompt_run_rpl_from_env`
- `retaprompt_run_rpb_from_env`
- `retaprompt_run_rpe_from_env`

These symbols are exported from the package static library artifact produced by
`crates/retaprompt` with:

```toml
[lib]
crate-type = ["rlib", "staticlib"]
```

## Build commands

Dedicated package library:

```bash
cargo build -p retaprompt --lib
```

Dedicated package binaries:

```bash
cargo build -p retaprompt --bin rp
cargo build -p retaprompt --bin rpl
cargo build -p retaprompt --bin rpb
cargo build -p retaprompt --bin rpe
```

## Intent

This is additive. Old code paths are preserved and delegate forward where
useful. The main structural completion here is that the dedicated `retaprompt`
package now depends on the **public** `reta::prompt` API rather than reaching
into deeper internal module paths, so the separate static library becomes a thin
consumer of the same shared prompt layer instead of another ad-hoc integration.

## Cargo bin discovery

The root package and the dedicated `retaprompt` package both set `autobins = false`.
That keeps Cargo restricted to the explicit `[[bin]]` entries so the legacy
`src/bin/reta_min.rs` path is no longer picked up accidentally.


## Single static library outcome

The workspace is intentionally reduced to one dedicated retaPrompt package:

- `crates/retaprompt` -> `libretaprompt.a`

`rp`, `rpl`, `rpb`, and `rpe` stay available as explicit binaries, but there is
no longer a separate static library package per frontend. The single shared
static library is the additive packaging layer that unifies the maximal common
behavior of all four frontends.
