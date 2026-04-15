# retaprompt

Additive static library package for the shared retaPrompt frontend layer.

This package exists so `rp`, `rpl`, `rpb`, and `rpe` can be linked through one
dedicated `retaprompt` static library artifact instead of being treated as
separate native library targets.

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

The package imports the stable public `reta::prompt::*` re-exports instead of
reaching into deeper prompt internals directly. That keeps the shared prompt API
centered in one public module and makes the separate package a thin additive
layer.

C ABI symbols exported from the static library:

- `retaprompt_run_kind_from_env`
- `retaprompt_run_auto_from_env`
- `retaprompt_run_rp_from_env`
- `retaprompt_run_rpl_from_env`
- `retaprompt_run_rpb_from_env`
- `retaprompt_run_rpe_from_env`

These C symbols are exported only by the dedicated `retaprompt` static
library. The main `reta` crate still owns the shared Rust prompt logic, but it
no longer exports the retaPrompt C ABI itself, so native linkage stays centered
on one `libretaprompt.a`.

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

Build the separate frontend wrappers from the dedicated frontend package:

```bash
cargo build -p retaprompt_frontends --bin rp
cargo build -p retaprompt_frontends --bin rpl
cargo build -p retaprompt_frontends --bin rpb
cargo build -p retaprompt_frontends --bin rpe
```

The dedicated static library artifact is emitted as `libretaprompt.a`
inside the usual Cargo target directory for the selected profile.

This is the only dedicated retaPrompt static library package in the workspace.
There are no extra `librp.a`, `librpl.a`, `librpb.a`, or `librpe.a` packages in
this layout.

## Header for native linkage

A minimal C header is included at:

- `crates/retaprompt/include/retaprompt.h`

This header matches the exported no-mangle symbols from the static library.
