# libreta_runtime.so — Reta engine, network, and scheduler

## Purpose

In the split build, `libreta_runtime.so` carries the heavy non-interactive Reta core. This includes program execution, workflow, table construction, output generation, architecture shadow paths, cache, and runtime bridges. This keeps `libreta.so` as a small facade.

## Direct integration

Direct user of this library: `libreta.so`.

Direct target dependencies: no private mandatory dependency inside this layer. The other private core libraries remain visible as topology anchors through `libreta.so`.

## Architecture boundary

This `.so` has two ABI layers:

1. public metadata symbols such as `reta_runtime_abi_anchor`,
2. private engine symbols with the `reta_runtime_core_*` prefix, intended to be used only by `libreta.so`.

External programs should continue to use `include/reta.h` and `libreta.so`, not this engine ABI directly. The runtime carrier compiles the old public `reta_*` engine symbols internally without `no_mangle`, preventing symbol interposition from sending `libreta.so` into recursion.

## Mathematical role

Network: tasks are nodes or edges, queues determine ordering, semaphores limit resources, and reduction keeps output deterministic. Categorically, this library is the morphism carrier through which the universal facade factors concrete program execution.

## Important ABI symbols

- `reta_runtime_abi_version`
- `reta_runtime_abi_anchor`
- `reta_runtime_abi_manifest_json`
- `reta_runtime_core_run_and_print_from_env_ffi`
- `reta_runtime_core_run_argv`
- `reta_runtime_core_free_string`
- `reta_runtime_core_shared_words_json`
- `reta_runtime_core_all_main_alias_groups_json`
- `reta_runtime_core_parameter_alias_groups_for_main_json`


## New component dependencies

`libreta_runtime.so` now also links against `libreta_data.so`, `libreta_parse.so`, `libreta_semantics.so`, `libreta_table.so`, `libreta_render.so`, and `libreta_arch.so`. The runtime anchor calls their ABI anchors so the `DT_NEEDED` topology does not collapse back into empty stubs.

## Build rule

`build.sh` builds this library before `libreta.so`. Then `libreta.so` is built with `--features split-facade` and linked against the prefixed runtime-core symbols.

## Size rule

This library should be larger than `libreta.so` because it carries the heavy engine code. Finer future distribution into `data`, `parse`, `semantics`, `table`, and `render` can continue behind the same topology.
