# libreta_arch.so — architecture, category, morphism

## Purpose

Boundary for architecture metadata, category, morphism graph, topology, universal property, shadow pipeline, and parity model.

## Direct integration

Direct user of this library: `libreta.so`.

Direct target dependencies: no direct private mandatory dependency inside this layer.

## Architecture boundary

This `.so` is an intentional ABI boundary. Rust-internal types should not cross it. The outside surface exports stable C symbols, simple integer values, and null-terminated strings. That keeps the topology stable while more Rust code can later move out of `libreta.so` and behind this library.

## Mathematical role

Mathematical category: objects are states and requests, morphisms are transformations, functors connect semantics and output, natural transformations preserve parity.

## Important ABI symbols

- `reta_arch_abi_version`
- `reta_arch_abi_anchor`
- `reta_arch_abi_manifest_json`
- `reta_arch_architecture_morphism_count`
- `reta_arch_architecture_csv_asset_count`

## Build rule

`build.sh` builds the private core libraries first and then builds `libreta.so` with `RETA_LINK_CORE_SPLIT_LIBS=1`. As a result, `rreta` links directly only to `libreta.so`; the private core libraries appear as `DT_NEEDED` entries of `libreta.so`.

## Non-goal

This library should not become a second public program interface beside `libreta.so`. Public program execution remains stable through the facade.
