# Reta/RetaPrompt shell variables — large programmer documentation

This file documents shell and environment variables relevant to the current `.so` build, runtime paths, retaPrompt, Termux scripts, and architecture/parity diagnostics. It is intentionally extensive: a programmer should be able to see whether a variable matters at build time, link time, run time, or only inside a script.

The key rule of this version is: **launchers stay small; logic belongs in `.so` libraries.** Therefore variables such as `RETA_LIB_PATH` and `RETA_RENDER_LIB_PATH` are only path/loader helpers. Autocomplete, autosuggest, command execution, rendering, semantics, and runtime topology are implemented in libraries.

## Quick start

```bash
# normal shared-library build
./build.sh release

# create package
./tools/package_prompt_split_sharedlibs.sh release

# if a launcher cannot find its library
RETA_LIB_PATH=target/release/libreta.so target/release/rrp
RETA_RENDER_LIB_PATH=target/release/libreta_render.so target/release/rgrundStrukHtml blank

# conservatively limit parallel execution
RETA_PARALLEL_WORKERS=2 RETA_PARALLEL_THRESHOLD=512 ./target/release/rreta -h
```

## Layer model

| Layer | Examples | Job | May contain algorithms? |
|---|---|---|---|
| C launchers | `rreta`, `rrp`, `rrpl`, `rrpe`, `rrpb`, `rgrundStrukHtml` | argv, paths, exit codes, ABI calls | No |
| Public facade | `libreta.so` | stable Reta ABI, delegation to core | Thin delegation only |
| Core components | `libreta_data.so`, `libreta_parse.so`, `libreta_semantics.so`, `libreta_table.so`, `libreta_render.so`, `libreta_arch.so`, `libreta_runtime.so` | data, parser, semantics, table, render, architecture, runtime | Yes |
| Prompt components | `libretaprompt_commands.so`, `libretaprompt_input.so` | command execution, input, autocomplete, autosuggest | Yes |
| Shell variables | `RETA_*`, `CARGO_*`, loader paths | configuration and diagnostics | No, control data only |

## Runtime, prompt, and loader variables

| Variable | Area | Meaning | Values | When to set | Caution |
|---|---|---|---|---|---|
| `RETA_LIB_PATH` | Runtime | Path to `libreta.so` when `retaprompt_commands` cannot find the Reta facade next to the executable or through the loader path. | Absolute or relative file path. | Set during package tests, Termux copies, or development outside RPATH/RUNPATH. | Must point to the facade, not to `libreta_runtime.so`. |
| `RETA_RENDER_LIB_PATH` | Runtime | Path to `libreta_render.so` for the dynamic `rgrundStrukHtml` launcher. | Absolute or relative file path. | Useful when `rgrundStrukHtml` is started from another directory. | Do not confuse it with `RETA_LIB_PATH`; HTML rendering enters through `libreta_render.so`. |
| `RETA_CSV_PATH` | Runtime | Path to the CSV data directory of the Reta data base. | Directory. | Set when CSV files are not in the expected package layout. | A wrong path makes data and alias functions incomplete. |
| `RETA_BIN` | Prompt | Optional path to an external Reta executable if prompt commands are intentionally not routed through `libreta.so`. | File path. | Diagnostics or old setups only. | Normally leave unset in the new `.so` architecture. |
| `RETA_PROMPT_SESSION_LOG` | Prompt | Path for the retaPrompt session log. | File path. | Set when prompt input/output should be traceable. | Do not log secrets if the file may be shared. |
| `COLUMNS` | Terminal | Terminal width for wrapping, tables, and prompt display. | Positive integer. | Override auto detection in CI or pipe environments. | Very small values degrade table layout. |
| `LINES` | Terminal | Terminal height for console/TUI paths. | Positive integer. | Rarely needed; useful for reproducible tests. | Not every path consumes `LINES`. |
| `HOME` | System | Base path for Termux target directories and some historical scripts. | Directory. | Normally supplied by the system. | Do not override during builds unless you know why. |
| `LD_LIBRARY_PATH` | Loader | Additional search path for the Linux/Android dynamic linker. | Colon-separated directory list. | Use only when RPATH/RUNPATH or package layout is not enough. | Can prefer the wrong `.so` version; RUNPATH is cleaner for production. |
| `DYLD_LIBRARY_PATH` | Loader | macOS equivalent of `LD_LIBRARY_PATH` for development ports. | Colon-separated directory list. | Relevant only for macOS experiments. | This project is primarily Linux/Termux-oriented. |

## Build, Cargo, and link variables

| Variable | Area | Meaning | Values | When to set | Caution |
|---|---|---|---|---|---|
| `CARGO_TARGET_DIR` | Cargo/build | Alternative Cargo target directory. | Directory. | Set when artifacts should live outside `target/`. | Scripts derive `target/debug` or `target/release` from it. |
| `RETA_LINK_CORE_SPLIT_LIBS` | Build | Enables the link edge from `libreta.so` to the private core `.so` files in `build.rs`. | `1` or unset. | Set by build scripts. | Do not force to `0`; otherwise `libreta.so` can become heavy again. |
| `RETA_RENDER_LINK_SEMANTICS` | Build | Enables the edge `libreta_render.so -> libreta_semantics.so`. | `1` or unset. | Set by build scripts. | Without it, `rgrundStrukHtml` loses the intended render/semantics topology. |
| `RETA_RUNTIME_LINK_CORE_COMPONENTS` | Build | Enables edges from `libreta_runtime.so` to data, parse, semantics, table, render, and arch. | `1` or unset. | Set by build scripts. | Without it, isolated stubs can reappear. |
| `RETA_BUILD_RUST_TOOL_BINS` | Build | Additionally builds heavy Rust diagnostic and tool binaries. | `1` or unset/`0`. | Set only for developer diagnostics. | Do not use for package size measurements; final public binaries are C launchers. |
| `RETA_BUILD_RUST_FRONTEND_BINS` | Build | Retired/blocked variable that used to build heavy Rust prompt frontends. | Must stay unset or `0`. | Do not set it anymore. | `1` intentionally fails the build so `rrp/rrpl/rrpe/rrpb` cannot become large again. |
| `RETA_PROMPT_LAUNCHER_MAX_BYTES` | Build/guard | Maximum allowed size of a prompt launcher. | Positive byte count, default `262144`. | Raise only when a platform legitimately makes C launchers larger. | Do not use as a workaround for Rust payload; inspect `tools/guard_prompt_launcher_topology.sh` first. |
| `PROFILE` | Cargo/script | Cargo profile; in scripts it is derived from the first argument, `debug` or `release`. | `debug` or `release`. | Prefer `./build.sh debug` or `./build.sh release` over manual export. | Cargo also sets `PROFILE` inside build scripts. |
| `OUT_DIR` | Cargo | Cargo-provided output directory for build scripts. | Directory. | Do not set manually. | Used by Rust `build.rs` files for generated linker shims. |
| `CARGO_MANIFEST_DIR` | Cargo | Cargo-provided path to the current crate manifest. | Directory. | Do not set manually. | Useful for build-script path resolution. |
| `RUSTFLAGS` | Cargo | Additional flags for rustc. | String. | Set only intentionally, for linker or symbol tests. | Can strongly change size and link topology. |

## Parallel execution variables

| Variable | Area | Meaning | Values | When to set | Caution |
|---|---|---|---|---|---|
| `RETA_PARALLEL` | Parallel | Compatible master switch for parallel execution; alias/source for the architecture parallel mode. | `auto`, `off`, `threads`, `processes`, or similar mode values. | Quickly test a global parallel mode. | More specific mode variables should win on conflict. |
| `RETA_PARALLEL_MODE` | Parallel | Explicit parallel mode for the architecture layer. | Mode string. | Use when the architecture path must be controlled unambiguously. | Clearer for new scripts than the alias `RETA_PARALLEL`. |
| `RETA_PARALLEL_WORKERS` | Parallel | Number of workers for parallel execution. | Positive integer. | Limit on CI, Termux, or small devices. | Too high can stress memory, scheduling, and output order. |
| `RETA_PARALLEL_CHUNK_SIZE` | Parallel | Chunk size for task batches. | Positive integer. | Tune scheduler overhead versus latency. | Small chunks increase overhead; large chunks reduce balancing. |
| `RETA_PARALLEL_THRESHOLD` | Parallel | Minimum size from which parallel execution starts. | Positive integer. | Increase when small jobs are slower in parallel. | Too low makes simple tables unnecessarily expensive. |
| `RETA_PARALLEL_START_METHOD` | Parallel | Start method for process/worker models in architecture context. | String. | Mostly useful for parity/Python reference paths. | Rust paths may not consume every Python start-method value. |
| `RETA_JOBS` | Parallel | Worker-count alias in the split runtime path. | Positive integer. | Short alias for batch/generator runs. | Merged with `RETA_THREADS`/`RETA_NUM_THREADS`. |
| `RETA_THREADS` | Parallel | Thread-count alias in the split runtime path. | Positive integer. | Limit on small devices. | Do not set inconsistently with `RETA_JOBS`. |
| `RETA_NUM_THREADS` | Parallel | Another thread-count alias. | Positive integer. | Compatibility with old scripts. | Prefer `RETA_PARALLEL_WORKERS` in new scripts. |
| `RETA_PARALLEL_MIN_ITEMS` | Parallel | Minimum item count for parallel execution in the shared runtime path. | Positive integer. | Increase if parallel execution starts too early. | Alias: `RETA_PARALLEL_MIN`. |
| `RETA_PARALLEL_MIN` | Parallel | Short alias for `RETA_PARALLEL_MIN_ITEMS`. | Positive integer. | Old script compatibility. | Prefer the longer name in new documentation. |
| `RETA_PARALLEL_ALLOW_NESTED` | Parallel | Allows nested parallel execution. | Boolean-like: `1`, `true`, `yes`. | Only for explicitly tested pipelines. | Can easily create oversubscription. |
| `RETA_GENERATORS` | Parallel lane | Controls the parallel strategy for lane `RETA_GENERATORS`. | Mode string or boolean-like value. | Set only when this single lane should differ from the global mode. | Lane values are more specific than global values. |
| `RETA_GENERATORS_PARALLEL` | Parallel lane | Enables parallel execution for lane `RETA_GENERATORS`. | Boolean-like. | Targeted performance experiments. | Can override conservative global settings. |
| `RETA_GENERATORS_SERIAL` | Parallel lane | Forces serial execution for lane `RETA_GENERATORS`. | Boolean-like. | Parity debugging or nondeterministic output. | Do not set together with `RETA_GENERATORS_PARALLEL`. |
| `RETA_GENERATORS_PARALLEL_MIN_ITEMS` | Parallel lane | Minimum item count for parallel execution of lane `RETA_GENERATORS`. | Positive integer. | Fine tune the lane. | Alias: `RETA_GENERATORS_PARALLEL_MIN`. |
| `RETA_GENERATORS_PARALLEL_MIN` | Parallel lane | Short alias for `RETA_GENERATORS_PARALLEL_MIN_ITEMS`. | Positive integer. | Compatibility. | Prefer the longer name in new scripts. |
| `RETA_OUTPUT` | Parallel lane | Controls the parallel strategy for lane `RETA_OUTPUT`. | Mode string or boolean-like value. | Set only when this single lane should differ from the global mode. | Lane values are more specific than global values. |
| `RETA_OUTPUT_PARALLEL` | Parallel lane | Enables parallel execution for lane `RETA_OUTPUT`. | Boolean-like. | Targeted performance experiments. | Can override conservative global settings. |
| `RETA_OUTPUT_SERIAL` | Parallel lane | Forces serial execution for lane `RETA_OUTPUT`. | Boolean-like. | Parity debugging or nondeterministic output. | Do not set together with `RETA_OUTPUT_PARALLEL`. |
| `RETA_OUTPUT_PARALLEL_MIN_ITEMS` | Parallel lane | Minimum item count for parallel execution of lane `RETA_OUTPUT`. | Positive integer. | Fine tune the lane. | Alias: `RETA_OUTPUT_PARALLEL_MIN`. |
| `RETA_OUTPUT_PARALLEL_MIN` | Parallel lane | Short alias for `RETA_OUTPUT_PARALLEL_MIN_ITEMS`. | Positive integer. | Compatibility. | Prefer the longer name in new scripts. |
| `RETA_WIDTH` | Parallel lane | Controls the parallel strategy for lane `RETA_WIDTH`. | Mode string or boolean-like value. | Set only when this single lane should differ from the global mode. | Lane values are more specific than global values. |
| `RETA_WIDTH_PARALLEL` | Parallel lane | Enables parallel execution for lane `RETA_WIDTH`. | Boolean-like. | Targeted performance experiments. | Can override conservative global settings. |
| `RETA_WIDTH_SERIAL` | Parallel lane | Forces serial execution for lane `RETA_WIDTH`. | Boolean-like. | Parity debugging or nondeterministic output. | Do not set together with `RETA_WIDTH_PARALLEL`. |
| `RETA_WIDTH_PARALLEL_MIN_ITEMS` | Parallel lane | Minimum item count for parallel execution of lane `RETA_WIDTH`. | Positive integer. | Fine tune the lane. | Alias: `RETA_WIDTH_PARALLEL_MIN`. |
| `RETA_WIDTH_PARALLEL_MIN` | Parallel lane | Short alias for `RETA_WIDTH_PARALLEL_MIN_ITEMS`. | Positive integer. | Compatibility. | Prefer the longer name in new scripts. |
| `RETA_WIDTHS` | Parallel lane | Controls the parallel strategy for lane `RETA_WIDTHS`. | Mode string or boolean-like value. | Set only when this single lane should differ from the global mode. | Lane values are more specific than global values. |
| `RETA_WIDTHS_PARALLEL` | Parallel lane | Enables parallel execution for lane `RETA_WIDTHS`. | Boolean-like. | Targeted performance experiments. | Can override conservative global settings. |
| `RETA_WIDTHS_SERIAL` | Parallel lane | Forces serial execution for lane `RETA_WIDTHS`. | Boolean-like. | Parity debugging or nondeterministic output. | Do not set together with `RETA_WIDTHS_PARALLEL`. |
| `RETA_WIDTHS_PARALLEL_MIN_ITEMS` | Parallel lane | Minimum item count for parallel execution of lane `RETA_WIDTHS`. | Positive integer. | Fine tune the lane. | Alias: `RETA_WIDTHS_PARALLEL_MIN`. |
| `RETA_WIDTHS_PARALLEL_MIN` | Parallel lane | Short alias for `RETA_WIDTHS_PARALLEL_MIN_ITEMS`. | Positive integer. | Compatibility. | Prefer the longer name in new scripts. |
| `RETA_PROMPT` | Parallel lane | Controls the parallel strategy for lane `RETA_PROMPT`. | Mode string or boolean-like value. | Set only when this single lane should differ from the global mode. | Lane values are more specific than global values. |
| `RETA_PROMPT_PARALLEL` | Parallel lane | Enables parallel execution for lane `RETA_PROMPT`. | Boolean-like. | Targeted performance experiments. | Can override conservative global settings. |
| `RETA_PROMPT_SERIAL` | Parallel lane | Forces serial execution for lane `RETA_PROMPT`. | Boolean-like. | Parity debugging or nondeterministic output. | Do not set together with `RETA_PROMPT_PARALLEL`. |
| `RETA_PROMPT_PARALLEL_MIN_ITEMS` | Parallel lane | Minimum item count for parallel execution of lane `RETA_PROMPT`. | Positive integer. | Fine tune the lane. | Alias: `RETA_PROMPT_PARALLEL_MIN`. |
| `RETA_PROMPT_PARALLEL_MIN` | Parallel lane | Short alias for `RETA_PROMPT_PARALLEL_MIN_ITEMS`. | Positive integer. | Compatibility. | Prefer the longer name in new scripts. |
| `RETA_PROMPT_BATCH` | Parallel lane | Controls the parallel strategy for lane `RETA_PROMPT_BATCH`. | Mode string or boolean-like value. | Set only when this single lane should differ from the global mode. | Lane values are more specific than global values. |
| `RETA_PROMPT_BATCH_PARALLEL` | Parallel lane | Enables parallel execution for lane `RETA_PROMPT_BATCH`. | Boolean-like. | Targeted performance experiments. | Can override conservative global settings. |
| `RETA_PROMPT_BATCH_SERIAL` | Parallel lane | Forces serial execution for lane `RETA_PROMPT_BATCH`. | Boolean-like. | Parity debugging or nondeterministic output. | Do not set together with `RETA_PROMPT_BATCH_PARALLEL`. |
| `RETA_PROMPT_BATCH_PARALLEL_MIN_ITEMS` | Parallel lane | Minimum item count for parallel execution of lane `RETA_PROMPT_BATCH`. | Positive integer. | Fine tune the lane. | Alias: `RETA_PROMPT_BATCH_PARALLEL_MIN`. |
| `RETA_PROMPT_BATCH_PARALLEL_MIN` | Parallel lane | Short alias for `RETA_PROMPT_BATCH_PARALLEL_MIN_ITEMS`. | Positive integer. | Compatibility. | Prefer the longer name in new scripts. |

## Architecture, parity, persistence, and recovery variables

| Variable | Area | Meaning | Values | When to set | Caution |
|---|---|---|---|---|---|
| `RETA_PERSISTENCE_DB` | Persistence | SQLite/file path for architecture persistence. | File path. | Set when audit/persistence data should be stored permanently. | Takes precedence over `RETA_AUDIT_DB`. |
| `RETA_AUDIT_DB` | Persistence | Compatible audit database path. | File path. | Old scripts and Python reference. | New scripts should use `RETA_PERSISTENCE_DB`. |
| `RETA_ARCHITECTURE_MODE` | Architecture | Main mode for architecture/topology paths. | Mode string. | When category/topology/activation logic should be explicitly enabled. | Aliases: `RETA_ARCH_MODE`, `RETA_ARCH`. |
| `RETA_ARCH_MODE` | Architecture | Short alias for `RETA_ARCHITECTURE_MODE`. | Mode string. | Compatibility. | Do not set inconsistently with the main variable. |
| `RETA_ARCH` | Architecture | Shortest alias for architecture mode. | Mode string. | Quick shell tests. | The long name is clearer for durable scripts. |
| `RETA_ARCH_TRACE` | Architecture | Enables architecture-layer trace output. | Boolean-like or trace level. | Morphisms/topology debugging. | Can produce a lot of output. |
| `RETA_ARCH_COMPARE_PY` | Parity | Enables comparison with the Python reference path. | Boolean-like. | Check Rust/Python commutativity. | Requires a reachable Python reference path. |
| `RETA_ARCH_COMPARE_PY_ARCH` | Parity | Also compares Python architecture paths. | Boolean-like. | Deep parity diagnostics. | Slower than a normal run. |
| `RETA_ARCH_ROLLBACK_ANCHOR` | Activation | Anchor/marker for rollback or recovery point. | String/ID. | Activation-file or recovery tests. | Set only with documentation for the concrete scenario. |
| `RETA_ARCH_ALLOW` | Architecture | Whitelist for architecture features. | Comma-separated list. | Feature slicing for tests. | Avoid unclear mixing with blocklists. |
| `RETA_ARCH_BLOCK` | Architecture | Blocklist for architecture features. | Comma-separated list. | Disable selected paths. | Can alter parity. |
| `RETA_ARCH_ACTIVATION_FILE` | Activation | Path to an activation file. | File path. | Load activations reproducibly. | File content must match the expected format. |
| `RETA_ARCH_ACTIVATION_DIR` | Activation | Directory for activation files. | Directory. | Manage multiple activation files. | An explicit file can take precedence. |
| `RETA_ARCH_ACTIVATION_RECOVERY_FILE` | Recovery | Path to a recovery file. | File path. | Recovery after activation/state tests. | Do not overwrite production data. |
| `RETA_ARCH_ACTIVATION_RECOVERY` | Recovery | Enables activation recovery behavior. | Boolean-like. | Only for tested recovery paths. | Can hide expected failures if permanently set. |

## Script-internal variables

These names appear in `build.sh`, `tools/*.sh`, or `termux_copy.sh`. They are documented so script changes remain reviewable. They are normally **not** environment variables that users should export.

| Name | Area | Explanation |
|---|---|---|
| `ROOT_DIR` | Script internal | Repository root computed from the script path. |
| `TARGET_DIR` | Script internal | Profile-specific target directory, usually `target/debug` or `target/release`. |
| `CARGO_FLAGS` | Script internal | Array of Cargo flags, for example `--release`. |
| `CORE_COMPONENT_BASE_PACKAGES` | Script internal | Crates built before `reta_render` and the runtime. |
| `CORE_COMPONENT_PACKAGES` | Script internal | Core components including the render library. |
| `CORE_SPLIT_PACKAGES` | Script internal | All core split crates including runtime. |
| `CORE_SPLIT_LIBRARIES` | Script internal | Expected `libreta_*.so` artifact names without prefix/suffix. |
| `PROMPT_SPLIT_PACKAGES` | Script internal | Prompt crates: commands and input. |
| `PROMPT_SPLIT_LIBRARIES` | Script internal | Expected prompt `.so` names without prefix/suffix. |
| `MANIFEST` | Script internal | Path to generated `retaprompt_split_sharedlibs_manifest.json`. |
| `OUT_DIR` | Script internal | Package output directory in `tools/package_prompt_split_sharedlibs.sh`; do not confuse with Cargo `OUT_DIR`. |
| `BIN_DIR` | Script internal | Termux target directory for executables. |
| `LIB_DIR` | Script internal | Termux target directory for `.so` files. |
| `SCRIPT_DIR` | Script internal | Directory of a tool wrapper. |
| `source` | Script internal | Local source path in copy helpers. |
| `dest` | Script internal | Local destination path in copy helpers. |
| `archive` | Script internal | Local archive name in regression checks against `.a` artifacts. |
| `facade_size` | Script internal | Measured size of `libreta.so` for size regressions. |
| `runtime_size` | Script internal | Measured size of `libreta_runtime.so` for size regressions. |

## Decision rules

1. Do not keep build variables permanently exported in your shell. The scripts set `RETA_LINK_CORE_SPLIT_LIBS`, `RETA_RENDER_LINK_SEMANTICS`, and `RETA_RUNTIME_LINK_CORE_COMPONENTS` only for the correct Cargo calls.
2. Use `RETA_LIB_PATH` only for the Reta facade. Prompt commands expect `libreta.so` there, not `libreta_runtime.so`.
3. Use `RETA_RENDER_LIB_PATH` only for the HTML renderer launcher. `rgrundStrukHtml` should enter through `libreta_render.so` as much as possible.
4. For reproducible size measurements, keep `RETA_BUILD_RUST_TOOL_BINS=0` and keep `RETA_BUILD_RUST_FRONTEND_BINS` unset/`0`; `1` is now a deliberate build failure.
5. For autocomplete/autosuggest debugging, do not modify the launchers. The logic lives in `libretaprompt_input.so`, especially in the prompt completion layer.
6. For Python/Rust parity, set architecture-comparison variables per test run, not globally in `.profile`.
7. For Termux, install into `$HOME/../usr/bin` and `$HOME/../usr/lib` or rely on package RUNPATH.

## Examples

### Local build with external target directory

```bash
CARGO_TARGET_DIR=/tmp/reta-target ./build.sh release
/tmp/reta-target/release/rreta -h
```

### Test the package layout without system installation

```bash
./tools/package_prompt_split_sharedlibs.sh release
cd target/release/retaprompt_split_sharedlibs_package
./rreta -h
./rrp -h
./rgrundStrukHtml blank
```

### Set a loader path for one run only

```bash
LD_LIBRARY_PATH="$PWD/target/release:${LD_LIBRARY_PATH:-}" target/release/rrp
```

### Enable prompt logging

```bash
RETA_PROMPT_SESSION_LOG=/tmp/retaPrompt.log target/release/rrp
```

### Limit parity diagnostics

```bash
RETA_ARCH_COMPARE_PY=1 RETA_PARALLEL_WORKERS=1 target/release/rreta -h
```

## Individual cards for all important environment variables

### `RETA_LIB_PATH`

**Area:** Runtime

**Meaning:** Path to `libreta.so` when `retaprompt_commands` cannot find the Reta facade next to the executable or through the loader path.

**Valid values:** Absolute or relative file path.

**Typical use:** Set during package tests, Termux copies, or development outside RPATH/RUNPATH.

**Risk:** Must point to the facade, not to `libreta_runtime.so`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_RENDER_LIB_PATH`

**Area:** Runtime

**Meaning:** Path to `libreta_render.so` for the dynamic `rgrundStrukHtml` launcher.

**Valid values:** Absolute or relative file path.

**Typical use:** Useful when `rgrundStrukHtml` is started from another directory.

**Risk:** Do not confuse it with `RETA_LIB_PATH`; HTML rendering enters through `libreta_render.so`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_CSV_PATH`

**Area:** Runtime

**Meaning:** Path to the CSV data directory of the Reta data base.

**Valid values:** Directory.

**Typical use:** Set when CSV files are not in the expected package layout.

**Risk:** A wrong path makes data and alias functions incomplete.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_BIN`

**Area:** Prompt

**Meaning:** Optional path to an external Reta executable if prompt commands are intentionally not routed through `libreta.so`.

**Valid values:** File path.

**Typical use:** Diagnostics or old setups only.

**Risk:** Normally leave unset in the new `.so` architecture.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_SESSION_LOG`

**Area:** Prompt

**Meaning:** Path for the retaPrompt session log.

**Valid values:** File path.

**Typical use:** Set when prompt input/output should be traceable.

**Risk:** Do not log secrets if the file may be shared.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `COLUMNS`

**Area:** Terminal

**Meaning:** Terminal width for wrapping, tables, and prompt display.

**Valid values:** Positive integer.

**Typical use:** Override auto detection in CI or pipe environments.

**Risk:** Very small values degrade table layout.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `LINES`

**Area:** Terminal

**Meaning:** Terminal height for console/TUI paths.

**Valid values:** Positive integer.

**Typical use:** Rarely needed; useful for reproducible tests.

**Risk:** Not every path consumes `LINES`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `HOME`

**Area:** System

**Meaning:** Base path for Termux target directories and some historical scripts.

**Valid values:** Directory.

**Typical use:** Normally supplied by the system.

**Risk:** Do not override during builds unless you know why.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `LD_LIBRARY_PATH`

**Area:** Loader

**Meaning:** Additional search path for the Linux/Android dynamic linker.

**Valid values:** Colon-separated directory list.

**Typical use:** Use only when RPATH/RUNPATH or package layout is not enough.

**Risk:** Can prefer the wrong `.so` version; RUNPATH is cleaner for production.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `DYLD_LIBRARY_PATH`

**Area:** Loader

**Meaning:** macOS equivalent of `LD_LIBRARY_PATH` for development ports.

**Valid values:** Colon-separated directory list.

**Typical use:** Relevant only for macOS experiments.

**Risk:** This project is primarily Linux/Termux-oriented.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `CARGO_TARGET_DIR`

**Area:** Cargo/build

**Meaning:** Alternative Cargo target directory.

**Valid values:** Directory.

**Typical use:** Set when artifacts should live outside `target/`.

**Risk:** Scripts derive `target/debug` or `target/release` from it.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_LINK_CORE_SPLIT_LIBS`

**Area:** Build

**Meaning:** Enables the link edge from `libreta.so` to the private core `.so` files in `build.rs`.

**Valid values:** `1` or unset.

**Typical use:** Set by build scripts.

**Risk:** Do not force to `0`; otherwise `libreta.so` can become heavy again.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_RENDER_LINK_SEMANTICS`

**Area:** Build

**Meaning:** Enables the edge `libreta_render.so -> libreta_semantics.so`.

**Valid values:** `1` or unset.

**Typical use:** Set by build scripts.

**Risk:** Without it, `rgrundStrukHtml` loses the intended render/semantics topology.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_RUNTIME_LINK_CORE_COMPONENTS`

**Area:** Build

**Meaning:** Enables edges from `libreta_runtime.so` to data, parse, semantics, table, render, and arch.

**Valid values:** `1` or unset.

**Typical use:** Set by build scripts.

**Risk:** Without it, isolated stubs can reappear.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_BUILD_RUST_TOOL_BINS`

**Area:** Build

**Meaning:** Additionally builds heavy Rust diagnostic and tool binaries.

**Valid values:** `1` or unset/`0`.

**Typical use:** Set only for developer diagnostics.

**Risk:** Do not use for package size measurements; final public binaries are C launchers.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_BUILD_RUST_FRONTEND_BINS`

**Area:** Build

**Meaning:** Retired/blocked. This variable used to build additional Rust prompt frontend binaries. That path is exactly what made `rrp`, `rrpl`, `rrpe`, and `rrpb` unnecessarily large again.

**Valid values:** unset or `0`.

**Typical use:** Do not set it anymore. The correct path is `./build.sh release` or `./tools/package_prompt_split_sharedlibs.sh release`; both create tiny C launchers.

**Risk:** `1` intentionally fails the build. This is a protection against size regression, not a bug.

**Programmer note:** Prompt behavior belongs in `libretaprompt_input.so` and `libretaprompt_commands.so`. Executables must be ABI launchers only. The guard scripts `tools/guard_prompt_frontend_sources.py` and `tools/guard_prompt_launcher_topology.sh` enforce this rule.

### `RETA_PROMPT_LAUNCHER_MAX_BYTES`

**Area:** Build/guard

**Meaning:** Size ceiling for the final prompt launchers `rrp`, `rrpl`, `rrpe`, and `rrpb`. The default is `262144` bytes.

**Valid values:** Positive byte count.

**Typical use:** Normally leave unset. Raise it only on a platform where a real C launcher legitimately carries more toolchain or loader metadata.

**Risk:** Do not use this variable to accept a Rust payload inside the launchers. If the guard fails, first check whether `rrp/rrpl/rrpe/rrpb` came from Rust binaries instead of `tools/launchers/*.c`.

**Programmer note:** `tools/guard_prompt_launcher_topology.sh` checks size, `DT_NEEDED` edges, forbidden direct `libreta*.so` edges, and Rust-payload symbols.

### `PROFILE`

**Area:** Cargo/script

**Meaning:** Cargo profile; in scripts it is derived from the first argument, `debug` or `release`.

**Valid values:** `debug` or `release`.

**Typical use:** Prefer `./build.sh debug` or `./build.sh release` over manual export.

**Risk:** Cargo also sets `PROFILE` inside build scripts.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `OUT_DIR`

**Area:** Cargo

**Meaning:** Cargo-provided output directory for build scripts.

**Valid values:** Directory.

**Typical use:** Do not set manually.

**Risk:** Used by Rust `build.rs` files for generated linker shims.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `CARGO_MANIFEST_DIR`

**Area:** Cargo

**Meaning:** Cargo-provided path to the current crate manifest.

**Valid values:** Directory.

**Typical use:** Do not set manually.

**Risk:** Useful for build-script path resolution.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RUSTFLAGS`

**Area:** Cargo

**Meaning:** Additional flags for rustc.

**Valid values:** String.

**Typical use:** Set only intentionally, for linker or symbol tests.

**Risk:** Can strongly change size and link topology.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PARALLEL`

**Area:** Parallel

**Meaning:** Compatible master switch for parallel execution; alias/source for the architecture parallel mode.

**Valid values:** `auto`, `off`, `threads`, `processes`, or similar mode values.

**Typical use:** Quickly test a global parallel mode.

**Risk:** More specific mode variables should win on conflict.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PARALLEL_MODE`

**Area:** Parallel

**Meaning:** Explicit parallel mode for the architecture layer.

**Valid values:** Mode string.

**Typical use:** Use when the architecture path must be controlled unambiguously.

**Risk:** Clearer for new scripts than the alias `RETA_PARALLEL`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PARALLEL_WORKERS`

**Area:** Parallel

**Meaning:** Number of workers for parallel execution.

**Valid values:** Positive integer.

**Typical use:** Limit on CI, Termux, or small devices.

**Risk:** Too high can stress memory, scheduling, and output order.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PARALLEL_CHUNK_SIZE`

**Area:** Parallel

**Meaning:** Chunk size for task batches.

**Valid values:** Positive integer.

**Typical use:** Tune scheduler overhead versus latency.

**Risk:** Small chunks increase overhead; large chunks reduce balancing.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PARALLEL_THRESHOLD`

**Area:** Parallel

**Meaning:** Minimum size from which parallel execution starts.

**Valid values:** Positive integer.

**Typical use:** Increase when small jobs are slower in parallel.

**Risk:** Too low makes simple tables unnecessarily expensive.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PARALLEL_START_METHOD`

**Area:** Parallel

**Meaning:** Start method for process/worker models in architecture context.

**Valid values:** String.

**Typical use:** Mostly useful for parity/Python reference paths.

**Risk:** Rust paths may not consume every Python start-method value.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_JOBS`

**Area:** Parallel

**Meaning:** Worker-count alias in the split runtime path.

**Valid values:** Positive integer.

**Typical use:** Short alias for batch/generator runs.

**Risk:** Merged with `RETA_THREADS`/`RETA_NUM_THREADS`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_THREADS`

**Area:** Parallel

**Meaning:** Thread-count alias in the split runtime path.

**Valid values:** Positive integer.

**Typical use:** Limit on small devices.

**Risk:** Do not set inconsistently with `RETA_JOBS`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_NUM_THREADS`

**Area:** Parallel

**Meaning:** Another thread-count alias.

**Valid values:** Positive integer.

**Typical use:** Compatibility with old scripts.

**Risk:** Prefer `RETA_PARALLEL_WORKERS` in new scripts.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PARALLEL_MIN_ITEMS`

**Area:** Parallel

**Meaning:** Minimum item count for parallel execution in the shared runtime path.

**Valid values:** Positive integer.

**Typical use:** Increase if parallel execution starts too early.

**Risk:** Alias: `RETA_PARALLEL_MIN`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PARALLEL_MIN`

**Area:** Parallel

**Meaning:** Short alias for `RETA_PARALLEL_MIN_ITEMS`.

**Valid values:** Positive integer.

**Typical use:** Old script compatibility.

**Risk:** Prefer the longer name in new documentation.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PARALLEL_ALLOW_NESTED`

**Area:** Parallel

**Meaning:** Allows nested parallel execution.

**Valid values:** Boolean-like: `1`, `true`, `yes`.

**Typical use:** Only for explicitly tested pipelines.

**Risk:** Can easily create oversubscription.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_GENERATORS`

**Area:** Parallel lane

**Meaning:** Controls the parallel strategy for lane `RETA_GENERATORS`.

**Valid values:** Mode string or boolean-like value.

**Typical use:** Set only when this single lane should differ from the global mode.

**Risk:** Lane values are more specific than global values.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_GENERATORS_PARALLEL`

**Area:** Parallel lane

**Meaning:** Enables parallel execution for lane `RETA_GENERATORS`.

**Valid values:** Boolean-like.

**Typical use:** Targeted performance experiments.

**Risk:** Can override conservative global settings.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_GENERATORS_SERIAL`

**Area:** Parallel lane

**Meaning:** Forces serial execution for lane `RETA_GENERATORS`.

**Valid values:** Boolean-like.

**Typical use:** Parity debugging or nondeterministic output.

**Risk:** Do not set together with `RETA_GENERATORS_PARALLEL`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_GENERATORS_PARALLEL_MIN_ITEMS`

**Area:** Parallel lane

**Meaning:** Minimum item count for parallel execution of lane `RETA_GENERATORS`.

**Valid values:** Positive integer.

**Typical use:** Fine tune the lane.

**Risk:** Alias: `RETA_GENERATORS_PARALLEL_MIN`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_GENERATORS_PARALLEL_MIN`

**Area:** Parallel lane

**Meaning:** Short alias for `RETA_GENERATORS_PARALLEL_MIN_ITEMS`.

**Valid values:** Positive integer.

**Typical use:** Compatibility.

**Risk:** Prefer the longer name in new scripts.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_OUTPUT`

**Area:** Parallel lane

**Meaning:** Controls the parallel strategy for lane `RETA_OUTPUT`.

**Valid values:** Mode string or boolean-like value.

**Typical use:** Set only when this single lane should differ from the global mode.

**Risk:** Lane values are more specific than global values.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_OUTPUT_PARALLEL`

**Area:** Parallel lane

**Meaning:** Enables parallel execution for lane `RETA_OUTPUT`.

**Valid values:** Boolean-like.

**Typical use:** Targeted performance experiments.

**Risk:** Can override conservative global settings.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_OUTPUT_SERIAL`

**Area:** Parallel lane

**Meaning:** Forces serial execution for lane `RETA_OUTPUT`.

**Valid values:** Boolean-like.

**Typical use:** Parity debugging or nondeterministic output.

**Risk:** Do not set together with `RETA_OUTPUT_PARALLEL`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_OUTPUT_PARALLEL_MIN_ITEMS`

**Area:** Parallel lane

**Meaning:** Minimum item count for parallel execution of lane `RETA_OUTPUT`.

**Valid values:** Positive integer.

**Typical use:** Fine tune the lane.

**Risk:** Alias: `RETA_OUTPUT_PARALLEL_MIN`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_OUTPUT_PARALLEL_MIN`

**Area:** Parallel lane

**Meaning:** Short alias for `RETA_OUTPUT_PARALLEL_MIN_ITEMS`.

**Valid values:** Positive integer.

**Typical use:** Compatibility.

**Risk:** Prefer the longer name in new scripts.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTH`

**Area:** Parallel lane

**Meaning:** Controls the parallel strategy for lane `RETA_WIDTH`.

**Valid values:** Mode string or boolean-like value.

**Typical use:** Set only when this single lane should differ from the global mode.

**Risk:** Lane values are more specific than global values.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTH_PARALLEL`

**Area:** Parallel lane

**Meaning:** Enables parallel execution for lane `RETA_WIDTH`.

**Valid values:** Boolean-like.

**Typical use:** Targeted performance experiments.

**Risk:** Can override conservative global settings.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTH_SERIAL`

**Area:** Parallel lane

**Meaning:** Forces serial execution for lane `RETA_WIDTH`.

**Valid values:** Boolean-like.

**Typical use:** Parity debugging or nondeterministic output.

**Risk:** Do not set together with `RETA_WIDTH_PARALLEL`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTH_PARALLEL_MIN_ITEMS`

**Area:** Parallel lane

**Meaning:** Minimum item count for parallel execution of lane `RETA_WIDTH`.

**Valid values:** Positive integer.

**Typical use:** Fine tune the lane.

**Risk:** Alias: `RETA_WIDTH_PARALLEL_MIN`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTH_PARALLEL_MIN`

**Area:** Parallel lane

**Meaning:** Short alias for `RETA_WIDTH_PARALLEL_MIN_ITEMS`.

**Valid values:** Positive integer.

**Typical use:** Compatibility.

**Risk:** Prefer the longer name in new scripts.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTHS`

**Area:** Parallel lane

**Meaning:** Controls the parallel strategy for lane `RETA_WIDTHS`.

**Valid values:** Mode string or boolean-like value.

**Typical use:** Set only when this single lane should differ from the global mode.

**Risk:** Lane values are more specific than global values.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTHS_PARALLEL`

**Area:** Parallel lane

**Meaning:** Enables parallel execution for lane `RETA_WIDTHS`.

**Valid values:** Boolean-like.

**Typical use:** Targeted performance experiments.

**Risk:** Can override conservative global settings.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTHS_SERIAL`

**Area:** Parallel lane

**Meaning:** Forces serial execution for lane `RETA_WIDTHS`.

**Valid values:** Boolean-like.

**Typical use:** Parity debugging or nondeterministic output.

**Risk:** Do not set together with `RETA_WIDTHS_PARALLEL`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTHS_PARALLEL_MIN_ITEMS`

**Area:** Parallel lane

**Meaning:** Minimum item count for parallel execution of lane `RETA_WIDTHS`.

**Valid values:** Positive integer.

**Typical use:** Fine tune the lane.

**Risk:** Alias: `RETA_WIDTHS_PARALLEL_MIN`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_WIDTHS_PARALLEL_MIN`

**Area:** Parallel lane

**Meaning:** Short alias for `RETA_WIDTHS_PARALLEL_MIN_ITEMS`.

**Valid values:** Positive integer.

**Typical use:** Compatibility.

**Risk:** Prefer the longer name in new scripts.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT`

**Area:** Parallel lane

**Meaning:** Controls the parallel strategy for lane `RETA_PROMPT`.

**Valid values:** Mode string or boolean-like value.

**Typical use:** Set only when this single lane should differ from the global mode.

**Risk:** Lane values are more specific than global values.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_PARALLEL`

**Area:** Parallel lane

**Meaning:** Enables parallel execution for lane `RETA_PROMPT`.

**Valid values:** Boolean-like.

**Typical use:** Targeted performance experiments.

**Risk:** Can override conservative global settings.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_SERIAL`

**Area:** Parallel lane

**Meaning:** Forces serial execution for lane `RETA_PROMPT`.

**Valid values:** Boolean-like.

**Typical use:** Parity debugging or nondeterministic output.

**Risk:** Do not set together with `RETA_PROMPT_PARALLEL`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_PARALLEL_MIN_ITEMS`

**Area:** Parallel lane

**Meaning:** Minimum item count for parallel execution of lane `RETA_PROMPT`.

**Valid values:** Positive integer.

**Typical use:** Fine tune the lane.

**Risk:** Alias: `RETA_PROMPT_PARALLEL_MIN`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_PARALLEL_MIN`

**Area:** Parallel lane

**Meaning:** Short alias for `RETA_PROMPT_PARALLEL_MIN_ITEMS`.

**Valid values:** Positive integer.

**Typical use:** Compatibility.

**Risk:** Prefer the longer name in new scripts.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_BATCH`

**Area:** Parallel lane

**Meaning:** Controls the parallel strategy for lane `RETA_PROMPT_BATCH`.

**Valid values:** Mode string or boolean-like value.

**Typical use:** Set only when this single lane should differ from the global mode.

**Risk:** Lane values are more specific than global values.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_BATCH_PARALLEL`

**Area:** Parallel lane

**Meaning:** Enables parallel execution for lane `RETA_PROMPT_BATCH`.

**Valid values:** Boolean-like.

**Typical use:** Targeted performance experiments.

**Risk:** Can override conservative global settings.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_BATCH_SERIAL`

**Area:** Parallel lane

**Meaning:** Forces serial execution for lane `RETA_PROMPT_BATCH`.

**Valid values:** Boolean-like.

**Typical use:** Parity debugging or nondeterministic output.

**Risk:** Do not set together with `RETA_PROMPT_BATCH_PARALLEL`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_BATCH_PARALLEL_MIN_ITEMS`

**Area:** Parallel lane

**Meaning:** Minimum item count for parallel execution of lane `RETA_PROMPT_BATCH`.

**Valid values:** Positive integer.

**Typical use:** Fine tune the lane.

**Risk:** Alias: `RETA_PROMPT_BATCH_PARALLEL_MIN`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PROMPT_BATCH_PARALLEL_MIN`

**Area:** Parallel lane

**Meaning:** Short alias for `RETA_PROMPT_BATCH_PARALLEL_MIN_ITEMS`.

**Valid values:** Positive integer.

**Typical use:** Compatibility.

**Risk:** Prefer the longer name in new scripts.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_PERSISTENCE_DB`

**Area:** Persistence

**Meaning:** SQLite/file path for architecture persistence.

**Valid values:** File path.

**Typical use:** Set when audit/persistence data should be stored permanently.

**Risk:** Takes precedence over `RETA_AUDIT_DB`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_AUDIT_DB`

**Area:** Persistence

**Meaning:** Compatible audit database path.

**Valid values:** File path.

**Typical use:** Old scripts and Python reference.

**Risk:** New scripts should use `RETA_PERSISTENCE_DB`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCHITECTURE_MODE`

**Area:** Architecture

**Meaning:** Main mode for architecture/topology paths.

**Valid values:** Mode string.

**Typical use:** When category/topology/activation logic should be explicitly enabled.

**Risk:** Aliases: `RETA_ARCH_MODE`, `RETA_ARCH`.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_MODE`

**Area:** Architecture

**Meaning:** Short alias for `RETA_ARCHITECTURE_MODE`.

**Valid values:** Mode string.

**Typical use:** Compatibility.

**Risk:** Do not set inconsistently with the main variable.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH`

**Area:** Architecture

**Meaning:** Shortest alias for architecture mode.

**Valid values:** Mode string.

**Typical use:** Quick shell tests.

**Risk:** The long name is clearer for durable scripts.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_TRACE`

**Area:** Architecture

**Meaning:** Enables architecture-layer trace output.

**Valid values:** Boolean-like or trace level.

**Typical use:** Morphisms/topology debugging.

**Risk:** Can produce a lot of output.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_COMPARE_PY`

**Area:** Parity

**Meaning:** Enables comparison with the Python reference path.

**Valid values:** Boolean-like.

**Typical use:** Check Rust/Python commutativity.

**Risk:** Requires a reachable Python reference path.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_COMPARE_PY_ARCH`

**Area:** Parity

**Meaning:** Also compares Python architecture paths.

**Valid values:** Boolean-like.

**Typical use:** Deep parity diagnostics.

**Risk:** Slower than a normal run.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_ROLLBACK_ANCHOR`

**Area:** Activation

**Meaning:** Anchor/marker for rollback or recovery point.

**Valid values:** String/ID.

**Typical use:** Activation-file or recovery tests.

**Risk:** Set only with documentation for the concrete scenario.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_ALLOW`

**Area:** Architecture

**Meaning:** Whitelist for architecture features.

**Valid values:** Comma-separated list.

**Typical use:** Feature slicing for tests.

**Risk:** Avoid unclear mixing with blocklists.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_BLOCK`

**Area:** Architecture

**Meaning:** Blocklist for architecture features.

**Valid values:** Comma-separated list.

**Typical use:** Disable selected paths.

**Risk:** Can alter parity.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_ACTIVATION_FILE`

**Area:** Activation

**Meaning:** Path to an activation file.

**Valid values:** File path.

**Typical use:** Load activations reproducibly.

**Risk:** File content must match the expected format.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_ACTIVATION_DIR`

**Area:** Activation

**Meaning:** Directory for activation files.

**Valid values:** Directory.

**Typical use:** Manage multiple activation files.

**Risk:** An explicit file can take precedence.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_ACTIVATION_RECOVERY_FILE`

**Area:** Recovery

**Meaning:** Path to a recovery file.

**Valid values:** File path.

**Typical use:** Recovery after activation/state tests.

**Risk:** Do not overwrite production data.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.

### `RETA_ARCH_ACTIVATION_RECOVERY`

**Area:** Recovery

**Meaning:** Enables activation recovery behavior.

**Valid values:** Boolean-like.

**Typical use:** Only for tested recovery paths.

**Risk:** Can hide expected failures if permanently set.

**Programmer note:** Read this variable as close as possible to the boundary where it is needed. Avoid hidden global assumptions inside executables. In the current shared-library architecture, runtime decisions belong in libraries; launchers should only forward paths, argv, and exit codes.


## Maintenance checklist for shell scripts

- New variable introduced? Document it here and in the German file.
- Variable controls build linking? Check `cargo:rerun-if-env-changed=...` in the responsible `build.rs`.
- Variable controls library search? It must not make a launcher contain heavy Rust code again.
- Variable controls prompt behavior? The actual logic must live in `libretaprompt_input.so` or `libretaprompt_commands.so`.
- Variable controls Reta core behavior? Decide whether it should be forwarded by `libreta.so` or evaluated inside `libreta_runtime.so`.
- Variable is script-internal only? Do not advertise it as user configuration.
- Variable affects package size? Update the size checks in `build.sh` and `tools/build_prompt_split_sharedlibs.sh`.

## Failure patterns

| Symptom | Likely cause | Check | Fix |
|---|---|---|---|
| `rrp` cannot find `libreta.so` | Loader path or `RETA_LIB_PATH` missing | `ldd target/release/rrp` | Use package layout or set `RETA_LIB_PATH` |
| `rgrundStrukHtml` is large again | Rust binary was built/copied instead of C launcher | `readelf -d rgrundStrukHtml` | Check build scripts and copy step |
| `libreta.so` larger than `libreta_runtime.so` | Facade carries core code again | `stat -c %s libreta.so libreta_runtime.so` | Check `RETA_LINK_CORE_SPLIT_LIBS=1` and runtime facade |
| Autosuggest only appears at line end | Hinter renders suffix only, not cursor position | Check `retaprompt_input_autosuggestion_at_cursor_json` | Rebuild `libretaprompt_input.so` |
| All core components have identical size | Stub regression | Compare sizes of `libreta_data/parse/...` | Check component functions and link guards |

## Relation to shared-library documentation

The `.so` documentation describes ABI, ownership, and topology per library. This file describes the variables that build, load, or diagnose that topology. Both document sets must be maintained together.
