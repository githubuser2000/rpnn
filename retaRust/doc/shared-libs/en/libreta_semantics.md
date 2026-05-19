# libreta_semantics.so — semantics, topology, presheaf

## Purpose

Boundary for column selection, row filters, generator choice, number logic, tags, and semantic condensation of parameters.

## Direct integration

Direct user of this library: `libreta.so`.

Direct target dependencies: no direct private mandatory dependency inside this layer.

## Architecture boundary

This `.so` is an intentional ABI boundary. Rust-internal types should not cross it. The outside surface exports stable C symbols, simple integer values, and null-terminated strings. That keeps the topology stable while more Rust code can later move out of `libreta.so` and behind this library.

## Mathematical role

Topology and presheaf: local parameter information is treated as sections whose neighborhood, closure, and compatibility are determined here.

## Important ABI symbols

- `reta_semantics_abi_version`
- `reta_semantics_abi_anchor`
- `reta_semantics_abi_manifest_json`
- `reta_semantics_abi_role_de`
- `reta_semantics_abi_role_en`


## Real code extraction

This library now contains the semantic choice and prompt-selection inventories (`WAHL15`, `WAHL16`, main switches, section switches).

Additional symbols:

- `reta_semantics_choice_counts_json`
- `reta_semantics_wahl15_value`
- `reta_semantics_wahl16_value`
- `reta_semantics_free_string`

## Build rule

`build.sh` builds the private core libraries first and then builds `libreta.so` with `RETA_LINK_CORE_SPLIT_LIBS=1`. As a result, `rreta` links directly only to `libreta.so`; the private core libraries appear as `DT_NEEDED` entries of `libreta.so`.

## Non-goal

This library should not become a second public program interface beside `libreta.so`. Public program execution remains stable through the facade.
