# Transcompilation stage: py reta arch -> Rust rreta / rretaPrompt

This stage starts the Rust transcompilation against the modular `py reta arch`
instead of the old monolith shape.

## New shared Rust architecture crate

`crates/reta_architecture` is now a workspace member and a dependency of:

- root `rreta` crate (`reta`)
- `retaprompt_commands`
- `retaprompt_input`

It currently ports the architecture spine that both sides need:

- network / dataflow / deterministic execution
- FIFO queue, LIFO stack, priority queue
- half-duplex and full-duplex channels
- resource semaphores
- symbolic topology and context refinement
- morphism graph and morphism composition
- presheaf local sections
- sheaf compatibility and deterministic gluing
- universal merge / column-bucket normalization
- category, functor and natural-transformation metadata

## Two Python references are intentional

`python_reference/` remains the legacy exact source used by existing generated
Rust files.  It has been refreshed for the current `kontinuum=m` / column `744`
case.

`python_arch_reference/` is a copy of the current modular `py reta arch` source.
This is the new architecture reference for future module-by-module ports.

## Runtime hooks

`rreta` now initializes the shared architecture runtime during preload and builds
a `RetaRunArchitecture` context from CLI arguments.

`rretaPrompt` now builds `PromptArchitectureContext` values for frontend startup
and one-shot prompt command execution.  These hooks do not change stdout/stderr;
they are structural transcompilation anchors.

## Enum/i18n refresh

The missing column `744` has been moved into the Rust exact i18n/tag layer:

- `src/shared/words_py.rs`
- `src/shared/words_python_like.rs`
- `src/shared/exact_i18n.rs`
- `src/runtime/mod.rs`
- `src/shared/reta_exact_tags_py.rs`
- `python_reference/lib4tables_Enum.py`
- `python_reference/libs/lib4tables_Enum.py`

`tableTags2_for_column(744)` now resolves to `{keinParaOdMetaP, sternPolygon}`.

## Build status

This environment has no `cargo`/`rustc`, so this stage could not be compiled
inside the sandbox.  Static checks were performed: changed Rust files have
balanced braces, generated Python architecture snapshots are valid JSON, and the
Rust source now contains the expected `744` mappings.
