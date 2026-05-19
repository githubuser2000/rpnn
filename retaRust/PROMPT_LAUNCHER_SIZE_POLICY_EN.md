# Prompt launcher size policy

This file is a hard maintenance rule for `rrp`, `rrpl`, `rrpe`, and `rrpb`.

## Goal

The four prompt executables are launchers only. They must not contain prompt algorithms,
completion logic, autosuggest logic, command parsers, or Reta execution logic.

Allowed topology:

```text
rrp  -> libretaprompt_input.so + libretaprompt_commands.so
rrpl -> libretaprompt_input.so + libretaprompt_commands.so
rrpe -> libretaprompt_input.so + libretaprompt_commands.so
rrpb -> libretaprompt_commands.so
```

`rrpb` remains command-only. `rrp`, `rrpl`, and `rrpe` keep both prompt libraries as
direct dependencies so input/autocomplete/autosuggest and command execution stay in
separate `.so` libraries while both ABI edges remain visible.

## Forbidden

In `crates/retaprompt_frontends/src/bin/rp.rs`, `rpl.rs`, `rpe.rs`, and `rpb.rs`, these
forms are forbidden:

```rust
retaprompt_input::...
retaprompt_commands::...
use retaprompt_input;
use retaprompt_commands;
```

Those calls embed Rust crate code into the executables and make them large again.

## Automatic guards

```bash
python3 tools/guard_prompt_frontend_sources.py
tools/guard_prompt_launcher_topology.sh target/release
```

The source guard checks the Rust sources of the Cargo launchers. The topology guard
checks the final linked files. The default limit for each prompt launcher is `262144`
bytes and can only be changed through `RETA_PROMPT_LAUNCHER_MAX_BYTES`.

Do not use that variable as an excuse to accept Rust payload in the launchers. If the
guard fails, the cause is almost always the wrong build path or a regression back to
Rust frontend binaries.

## Retired build path

`RETA_BUILD_RUST_FRONTEND_BINS=1` is retired and intentionally fails in the active
build. It was an entry point for oversized frontend binaries.
