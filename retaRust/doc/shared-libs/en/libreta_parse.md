# libreta_parse.so — parsing, tokenization, and input morphisms

## Programmer summary

`libreta_parse.so` is the shared-library boundary for: **parsing, tokenization, and input morphisms**.

This component owns the shell/prompt tokenizer and forms the clean boundary between textual surface and semantic interpretation.

This file is intentionally detailed. It explains not only _what_ is built, but also _why_ this ABI boundary exists, which dependencies are allowed, how memory ownership works, which failure modes are common, and how to verify the library in practice.

## Artifact and source locations

| Field | Value |
|---|---|
| Artifact | `target/<profile>/libreta_parse.so` |
| Crate | `reta_parse` |
| Rust source | `crates/reta_parse/src/lib.rs` |
| C header | `crates/reta_parse/include/reta_parse.h` |
| Documentation | `doc/shared-libs/de/libreta_parse.md` and `doc/shared-libs/en/libreta_parse.md` |
| Build profile | `debug` or `release` via `./build.sh <profile>` |

## Direct consumers

- `libreta.so`
- `libreta_runtime.so`
- `parser probes`

## Direct dynamic dependencies

- keine direkte private Pflichtabhängigkeit / no direct private mandatory dependency

Important: “direct” means `DT_NEEDED` or intentional dynamic loading. Transitive dependencies are not the direct responsibility of this library. That distinction keeps `rrpb` command-only, makes `rrp`/`rrpl`/`rrpe` carry both prompt libraries, and keeps `rgrundStrukHtml` directly linked to `libreta_render.so`.

## Architecture boundary

This library is a real ABI boundary. Internal code may change as long as the C surface remains stable. The boundary is not meant to leak arbitrary Rust objects. Only these shapes should cross it:

- fixed-width integers such as `uint32_t`, `uint64_t`, `int32_t`, `size_t`, `uint8_t`,
- C strings as `const char *` for borrowed static data,
- C strings as `char *` for allocated return values,
- simple C structs explicitly defined by the header,
- exit codes or JSON as language-neutral textual data.

The following are not acceptable ABI contracts:

- Rust references,
- Rust `String`, `Vec`, `HashMap`, `BTreeMap`, `IndexMap`,
- panic propagation across the ABI boundary,
- implicit ownership where the caller has to guess who frees memory,
- undocumented unofficial symbols.

## Mathematical role

family of morphisms from raw text into tokenized, canonicalizable request fragments.

This mathematical role is not decoration. It is a practical architecture rule: similar morphisms belong in the same library family, but every tiny function does not get its own `.so`. This keeps the topology understandable and loader/ABI complexity manageable.

## Public ABI symbols

- `reta_parse_abi_version`
- `reta_parse_abi_anchor`
- `reta_parse_abi_library_name`
- `reta_parse_abi_crate_name`
- `reta_parse_abi_role_de`
- `reta_parse_abi_role_en`
- `reta_parse_abi_math_de`
- `reta_parse_abi_math_en`
- `reta_parse_abi_manifest_json`
- `reta_parse_shell_token_count`
- `reta_parse_shell_tokens_json`
- `reta_parse_free_string`

Machine-readable view:

```text
reta_parse_abi_version
reta_parse_abi_anchor
reta_parse_abi_library_name
reta_parse_abi_crate_name
reta_parse_abi_role_de
reta_parse_abi_role_en
reta_parse_abi_math_de
reta_parse_abi_math_en
reta_parse_abi_manifest_json
reta_parse_shell_token_count
reta_parse_shell_tokens_json
reta_parse_free_string
```

## Memory ownership

Token JSON returned by reta_parse_shell_tokens_json; release with reta_parse_free_string.

General rule for all Reta and retaPrompt shared libraries:

```c
char *ptr = some_library_function(...);
/* read, copy, or print ptr */
some_matching_library_free_string(ptr);
```

Wrong:

```c
char *ptr = reta_data_shared_words_json();
reta_free_string(ptr);              /* wrong: different library */
free(ptr);                          /* wrong: different allocator */
```

Right:

```c
char *ptr = reta_data_shared_words_json();
reta_data_free_string(ptr);         /* right: same ABI family */
```

## Error and panic model

The ABI must not propagate Rust panics into C. Entry points that start external program paths or return exit codes are guarded. For string-returning functions, robust clients should still be defensive:

- check for null pointers,
- avoid invalid inputs,
- keep UTF-8 assumptions explicit,
- parse JSON instead of executing or blindly concatenating it,
- do not ignore exit codes.

## Threading and reentrancy

The library should not be treated as a global mutable singleton contract. Internally, there may still be caches, `OnceLock`s, or runtime initialization. For programmers this means:

- concurrent use is safe only where no mutable session is shared,
- returned C strings are owned by the caller until the matching free function is called,
- global environment variables such as `RETA_CSV_PATH` or `RETA_LIB_PATH` should be set before the first call,
- tests that change environment variables should isolate processes.

## Build path

Typical build:

```bash
./build.sh release
```

Verified shared-library build:

```bash
./tools/build_prompt_split_sharedlibs.sh release
```

Packaging:

```bash
./tools/package_prompt_split_sharedlibs.sh release
```

Important build rules:

- Dynamic `.so` libraries are built; `.a` archives are not part of the active path.
- Final public executables are created as tiny C launchers in the normal package path.
- `libreta.so` remains a facade.
- `libreta_runtime.so` carries the heavy Reta core.
- `libreta_data.so`, `libreta_parse.so`, `libreta_semantics.so`, `libreta_table.so`, and `libreta_render.so` must not all have the exact same stub size.

## Dynamic link verification

Useful commands:

```bash
readelf -d target/release/libreta_parse.so
nm -D --defined-only target/release/libreta_parse.so
```

For launchers as well:

```bash
readelf -d target/release/rreta
readelf -d target/release/rgrundStrukHtml
readelf -d target/release/rrp
readelf -d target/release/rrpb
```

The expected topology is not cosmetic. It prevents code from drifting back into executables or the wrong `.so` carrier.

## RPATH/RUNPATH and installation

The launchers are built with search paths that support these layouts:

```text
$ORIGIN
$ORIGIN/lib
$ORIGIN/../lib
```

On Termux the usual copy target is `$HOME/../usr/bin` and `$HOME/../usr/lib`. Portable packages may keep executables next to libraries or in `bin/` with libraries in `../lib/`.

## Common regressions

| Symptom | Likely cause | Check |
|---|---|---|
| Library missing at program start | RPATH/RUNPATH or install layout is wrong | `readelf -d <executable>` |
| Symbol missing | Crate not built as `cdylib` or export removed | `nm -D --defined-only` |
| Executable large again | Rust binary used instead of C launcher | `file`, `readelf -d`, build script |
| All components same size | Components collapsed back into ABI stubs | size check in `build.sh` |
| Facade huge again | `split-facade` inactive or engine moved into `libreta.so` | size rule `libreta.so < libreta_runtime.so` |
| Crash while freeing | wrong free function or `free()` used | ownership rules |

## Test and review checklist

- Check the ABI version before use when a client loads the library directly.
- Do not pass Rust types across the C boundary.
- Release strings allocated by this library with the matching free function from the same library.
- Do not use a free function from another library just because the type is also `char *`.
- When packaging, verify RPATH/RUNPATH and `DT_NEEDED` with `readelf -d`.
- Use `nm -D --defined-only` when checking exported symbols.
- For size regressions, check whether a library collapsed back into a stub or heavy code moved into the wrong .so.
- Do not introduce cyclic public ABI dependencies.

## Extension rules

Assign new functionality to a responsibility before exporting it:

1. Data/catalog? Use `libreta_data.so`.
2. Text/argv/token? Use `libreta_parse.so`.
3. Selection, meaning, parameter space? Use `libreta_semantics.so`.
4. Table, view, width, materialization? Use `libreta_table.so`.
5. Output format, HTML, BBCode, plaintext? Use `libreta_render.so`.
6. Architecture metadata, topology, morphism counts? Use `libreta_arch.so`.
7. Execution, engine, scheduler, queue, semaphore? Use `libreta_runtime.so`.
8. Prompt command without interactive input? Use `libretaprompt_commands.so`.
9. Prompt input, completion, suggest, history? Use `libretaprompt_input.so`.
10. Public Reta ABI? Only then use `libreta.so`.

## Minimal C usage shape

```c
#include "reta_parse.h"

int main(void) {
    /* This example is intentionally generic. Details are in the header. */
    return 0;
}
```

## Maintenance note

This documentation is part of the ABI. If a symbol is added, removed, or semantically changed, this file must be updated together with the header and build checks. An undocumented `.so` boundary is incomplete in this project.
