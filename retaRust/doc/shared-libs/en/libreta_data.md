# libreta_data.so — data and catalogs

## Purpose

Boundary for words, aliases, CSV/HTML catalogs, static tables, language values, and data sources. This library is where data access and immutable catalog logic move behind the facade.

## Direct integration

Direct user of this library: `libreta.so`.

Direct target dependencies: no direct private mandatory dependency inside this layer.

## Architecture boundary

This `.so` is an intentional ABI boundary. Rust-internal types should not cross it. The outside surface exports stable C symbols, simple integer values, and null-terminated strings. That keeps the topology stable while more Rust code can later move out of `libreta.so` and behind this library.

## Mathematical role

Relational basis: objects are words, aliases, CSV rows, columns, and catalog records; morphisms are lookup, normalization, and projection.

## Important ABI symbols

- `reta_data_abi_version`
- `reta_data_abi_anchor`
- `reta_data_abi_manifest_json`
- `reta_data_abi_role_de`
- `reta_data_abi_role_en`


## Real code extraction

This library is no longer only an ABI anchor. It now contains the generated `Words` data structure and alias projections from the Python-source-of-truth logic. Therefore it should not have the exact same size as the small stub-like components anymore.

Additional symbols:

- `reta_data_words_entry_count`
- `reta_data_shared_words_json`
- `reta_data_all_main_alias_groups_json`
- `reta_data_parameter_alias_groups_for_main_json`
- `reta_data_resolve_parameter_main_alias`
- `reta_data_free_string`

## Build rule

`build.sh` builds the private core libraries first and then builds `libreta.so` with `RETA_LINK_CORE_SPLIT_LIBS=1`. As a result, `rreta` links directly only to `libreta.so`; the private core libraries appear as `DT_NEEDED` entries of `libreta.so`.

## Non-goal

This library should not become a second public program interface beside `libreta.so`. Public program execution remains stable through the facade.
