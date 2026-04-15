# retaPrompt static library layer

This project now contains one **dedicated** additive `retaprompt` package for the
shared retaPrompt frontend layer and one separate `retaprompt_frontends`
package for the executable wrappers.

It does **not** introduce a second `reta` runtime implementation and does not
remove the existing `reta` crate. Instead, it adds a thin library package on top
of the existing prompt/runtime code so `rp`, `rpl`, `rpb`, and `rpe` can share a
single dedicated static library artifact.

## Central shared layer

The real prompt behavior remains centralized in the existing Rust prompt code:

- `src/prompt/frontend_profile.rs`
- `src/prompt/retapromptlib.rs`
- `src/prompt/mod.rs`

The additive package layers now live here:

- `crates/retaprompt/Cargo.toml`
- `crates/retaprompt/src/lib.rs`
- `crates/retaprompt/include/retaprompt.h`
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

These symbols are exported only from the dedicated `crates/retaprompt` static
library artifact. The main `reta` crate keeps the shared Rust prompt logic but no
longer exports the retaPrompt C ABI itself, so the native linkage target stays
centered on one dedicated `libretaprompt.a`.

The package artifact is produced by `crates/retaprompt` with:

```toml
[lib]
crate-type = ["rlib", "staticlib"]
```

## Build commands

Build only the single shared retaPrompt static library:

```bash
cargo build -p retaprompt --lib
```

Build the thin frontend executables that sit on top of it:

```bash
cargo build -p retaprompt_frontends --bin rp
cargo build -p retaprompt_frontends --bin rpl
cargo build -p retaprompt_frontends --bin rpb
cargo build -p retaprompt_frontends --bin rpe
```

Or use the helper script:

```bash
./tools/build_retaprompt_staticlib.sh debug lib
./tools/build_retaprompt_staticlib.sh debug all
```

## Intent

This is additive. Old code paths are preserved and delegate forward where
useful. The structural completion here is now explicit:

- `retaprompt` = one shared dedicated static library package
- `retaprompt_frontends` = the four thin executable wrappers

That means the project no longer models the four prompt executables as four
separate static library targets. Instead, they all converge on the same shared
`libretaprompt.a`.
