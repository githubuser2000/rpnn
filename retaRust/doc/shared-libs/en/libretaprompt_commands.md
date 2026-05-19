# libretaprompt_commands.so — prompt commands

## Purpose

Command library for `rrpb` and the command side of `rrp`, `rrpl`, and `rrpe`. `rrpb` uses only this prompt library.

## Direct integration

Direct user of this library: `rrpb, rrp, rrpl, rrpe`.

Direct target dependencies: no direct private mandatory dependency inside this layer.

## Architecture boundary

This `.so` is an intentional ABI boundary. Rust-internal types should not cross it. The outside surface exports stable C symbols, simple integer values, and null-terminated strings. That keeps the topology stable while more Rust code can later move out of `libreta.so` and behind this library.

## Mathematical role

Morphisms from prompt text into executable Reta commands; no line-input UI, no autocomplete, no autosuggest.

## Important ABI symbols

- `retaprompt_commands_run_kind_from_env`
- `retaprompt_commands_run_current_executable_from_env`
- `retaprompt_commands_run_rp_from_env`
- `retaprompt_commands_run_rpl_from_env`
- `retaprompt_commands_run_rpb_from_env`
- `retaprompt_commands_run_rpe_from_env`

## Build rule

`build.sh` builds the private core libraries first and then builds `libreta.so` with `RETA_LINK_CORE_SPLIT_LIBS=1`. As a result, `rreta` links directly only to `libreta.so`; the private core libraries appear as `DT_NEEDED` entries of `libreta.so`.

## Non-goal

This library should not become a second public program interface beside `libreta.so`. Public program execution remains stable through the facade.
