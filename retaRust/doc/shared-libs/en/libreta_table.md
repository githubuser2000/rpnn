# libreta_table.so — tables, view, sheaf

## Purpose

Boundary for table materialization, table state, view construction, adapters, and merging local semantic sections.

## Direct integration

Direct user of this library: `libreta.so`.

Direct target dependencies: no direct private mandatory dependency inside this layer.

## Architecture boundary

This `.so` is an intentional ABI boundary. Rust-internal types should not cross it. The outside surface exports stable C symbols, simple integer values, and null-terminated strings. That keeps the topology stable while more Rust code can later move out of `libreta.so` and behind this library.

## Mathematical role

Sheaf: compatible local column, row, and parameter sections are glued into a global table.

## Important ABI symbols

- `reta_table_abi_version`
- `reta_table_abi_anchor`
- `reta_table_abi_manifest_json`
- `reta_table_abi_role_de`
- `reta_table_abi_role_en`


## Real code extraction

This library now contains concrete table helper logic for natural width calculation and budget shrinking. Full table materialization can keep moving behind the same boundary.

Additional symbols:

- `reta_table_natural_widths_json`
- `reta_table_shrink_widths_json`
- `reta_table_free_string`

## Build rule

`build.sh` builds the private core libraries first and then builds `libreta.so` with `RETA_LINK_CORE_SPLIT_LIBS=1`. As a result, `rreta` links directly only to `libreta.so`; the private core libraries appear as `DT_NEEDED` entries of `libreta.so`.

## Non-goal

This library should not become a second public program interface beside `libreta.so`. Public program execution remains stable through the facade.
