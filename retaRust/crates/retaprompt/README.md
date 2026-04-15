# retaprompt

Additive static library package for the shared retaPrompt frontend layer.

This package exists so `rp`, `rpl`, `rpb`, and `rpe` can be built and linked
through a dedicated `retaprompt` static library artifact instead of only through
the main `reta` static library.

It does **not** delete or replace the existing `reta` crate. The current project
keeps the old code paths intact and adds a dedicated package on top.

## Exposed entry points

Rust API:

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

The package now imports the stable public `reta::prompt::*` re-exports instead
of reaching into `reta::prompt::frontend_profile::*` directly. That keeps the
shared prompt API centered in one public module and makes the separate package a
thin additive layer.

C ABI symbols exported from the static library:

- `retaprompt_run_kind_from_env`
- `retaprompt_run_auto_from_env`
- `retaprompt_run_rp_from_env`
- `retaprompt_run_rpl_from_env`
- `retaprompt_run_rpb_from_env`
- `retaprompt_run_rpe_from_env`

ABI kind values for `retaprompt_run_kind_from_env`:

- `0` = auto
- `1` = rp
- `2` = rpl
- `3` = rpb
- `4` = rpe

## Frontend defaults in this shared layer

- `rp`: vi mode, no implicit logging, interactive
- `rpl`: vi mode, implicit logging, interactive
- `rpb`: vi mode, no implicit logging, one-shot
- `rpe`: emacs mode, no implicit logging, interactive

## Build

Build the dedicated static library:

```bash
cargo build -p retaprompt --lib
```

Build one of the dedicated prompt binaries from the package:

```bash
cargo build -p retaprompt --bin rp
cargo build -p retaprompt --bin rpl
cargo build -p retaprompt --bin rpb
cargo build -p retaprompt --bin rpe
```

Run the dedicated package binaries directly:

```bash
cargo run -p retaprompt --bin rp
cargo run -p retaprompt --bin rpl
cargo run -p retaprompt --bin rpb -- av12-15
cargo run -p retaprompt --bin rpe
```

The dedicated static library artifact is emitted as `libretaprompt.a`
inside the usual Cargo target directory for the selected profile.

## Header for native linkage

A minimal C header is included at:

- `crates/retaprompt/include/retaprompt.h`

This header matches the exported no-mangle symbols from the static library.

## Cargo bin discovery

The root package and the dedicated `retaprompt` package both set `autobins = false`.
That keeps Cargo restricted to the explicit `[[bin]]` entries so the legacy
`src/bin/reta_min.rs` path is no longer picked up accidentally.
