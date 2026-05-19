# libreta.so — thin stable Reta facade

## Purpose

`libreta.so` is the public, stable C ABI facade for `rreta` and for external users of the Reta ABI. In the split build, this library no longer carries the heavy Reta engine code. It exports the known public symbols and forwards actual execution to `libreta_runtime.so`.

## Direct integration

Direct user of this library: `rreta`.

Direct target dependencies: `libreta_data.so`, `libreta_parse.so`, `libreta_semantics.so`, `libreta_table.so`, `libreta_render.so`, `libreta_arch.so`, `libreta_runtime.so`.

## Architecture boundary

This `.so` is an intentional ABI facade. Rust-internal types do not cross this boundary. The outside surface keeps stable C symbols, simple integer values, C strings, and `RetaFfiResponse`. Internally it delegates to prefixed runtime symbols such as `reta_runtime_core_run_argv` in `libreta_runtime.so`.

## Mathematical role

Universal property: all non-interactive frontends factor through the same canonical path `argv/stdin -> RetaRequest -> RetaResponse -> Output`. The facade remains the universal object; the runtime engine is the factored morphism carrier.

## Important ABI symbols

- `reta_run_and_print_from_env_ffi`
- `reta_abi_version`
- `reta_run_argv`
- `reta_free_string`
- `reta_shared_words_json`
- `reta_all_main_alias_groups_json`
- `reta_parameter_alias_groups_for_main_json`
- `reta_core_split_abi_anchor`
- `reta_core_split_abi_manifest_json`
- `reta_core_split_abi_is_linked`

## Build rule

`build.sh` builds the private core libraries first and then builds `libreta.so` with `--features split-facade` and `RETA_LINK_CORE_SPLIT_LIBS=1`. As a result, `rreta` links directly only to `libreta.so`, while `libreta.so` carries the private core libraries as `DT_NEEDED` entries.

## Size rule

`libreta.so` must be smaller than `libreta_runtime.so`. The build script fails when this rule is violated. This prevents the heavy engine code from accidentally moving back into the facade.
