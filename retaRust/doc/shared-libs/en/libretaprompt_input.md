# libretaprompt_input.so — prompt input, autocomplete, autosuggest

## Purpose

Interactive input library for `rrp`, `rrpl`, and `rrpe`. It encapsulates line input, autocomplete, autosuggest, history, and interactive profiles.

## Direct integration

Direct user of this library: `rrp, rrpl, rrpe`.

Direct target dependencies: `libretaprompt_commands.so`.

## Architecture boundary

This `.so` is an intentional ABI boundary. Rust-internal types should not cross it. The outside surface exports stable C symbols, simple integer values, and null-terminated strings. That keeps the topology stable while more Rust code can later move out of `libreta.so` and behind this library.

## Mathematical role

Bidirectional channel between user state and prompt state; completion is a local selection over the current token context.

## Important ABI symbols

- `retaprompt_input_run_kind_from_env`
- `retaprompt_input_run_current_executable_from_env`
- `retaprompt_input_run_any_current_executable_from_env`
- `retaprompt_input_run_launcher_kind_from_env`
- `retaprompt_input_run_rp_from_env`
- `retaprompt_input_run_rpl_from_env`
- `retaprompt_input_run_rpe_from_env`

## Build rule

`build.sh` builds the private core libraries first and then builds `libreta.so` with `RETA_LINK_CORE_SPLIT_LIBS=1`. As a result, `rreta` links directly only to `libreta.so`; the private core libraries appear as `DT_NEEDED` entries of `libreta.so`.

## Non-goal

This library should not become a second public program interface beside `libreta.so`. Public program execution remains stable through the facade.
