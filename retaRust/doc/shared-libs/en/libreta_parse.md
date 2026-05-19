# libreta_parse.so — parsing and input morphisms

## Purpose

Boundary for command-line parsing, text decomposition, alias resolution, parameter preparation, and prompt token translation.

## Direct integration

Direct user of this library: `libreta.so`.

Direct target dependencies: no direct private mandatory dependency inside this layer.

## Architecture boundary

This `.so` is an intentional ABI boundary. Rust-internal types should not cross it. The outside surface exports stable C symbols, simple integer values, and null-terminated strings. That keeps the topology stable while more Rust code can later move out of `libreta.so` and behind this library.

## Mathematical role

Morphism family from raw text or `argv` into a canonical request. The library should not render tables or mutate global data.

## Important ABI symbols

- `reta_parse_abi_version`
- `reta_parse_abi_anchor`
- `reta_parse_abi_manifest_json`
- `reta_parse_abi_role_de`
- `reta_parse_abi_role_en`


## Real code extraction

This library now contains the shell/prompt tokenizer `split_shell_like`. This is the first real parsing function behind this ABI boundary.

Additional symbols:

- `reta_parse_shell_token_count`
- `reta_parse_shell_tokens_json`
- `reta_parse_free_string`

## Build rule

`build.sh` builds the private core libraries first and then builds `libreta.so` with `RETA_LINK_CORE_SPLIT_LIBS=1`. As a result, `rreta` links directly only to `libreta.so`; the private core libraries appear as `DT_NEEDED` entries of `libreta.so`.

## Non-goal

This library should not become a second public program interface beside `libreta.so`. Public program execution remains stable through the facade.
