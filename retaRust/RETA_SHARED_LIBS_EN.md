# Reta `.so` split — large English overview

This file is the central English overview of the dynamic `.so` topology. Detailed per-library documents live under `doc/shared-libs/en/`.

## Active topology

```text
rreta -> libreta.so -> libreta_data.so + libreta_parse.so + libreta_semantics.so + libreta_table.so + libreta_render.so + libreta_arch.so + libreta_runtime.so
rgrundStrukHtml -> libreta_render.so -> libreta_semantics.so
rrp/rrpl/rrpe -> libretaprompt_input.so + libretaprompt_commands.so
rrpb -> libretaprompt_commands.so
```

## Core rules

- `libreta.so` remains small and public.
- `libreta_runtime.so` carries the heavy non-interactive Reta core.
- `libreta_render.so` carries HTML generation for `rgrundStrukHtml`.
- `libretaprompt_input.so` carries interactive input, autocomplete, autosuggest, and history.
- `libretaprompt_commands.so` carries the command side and is the only prompt library for `rrpb`.
- Final executables are built as C launchers in the normal build path.
- Static archives are intentionally disabled in the active path.

## All `.so` documents

- [libreta.so](libreta.md) — public stable Reta C ABI facade
- [libreta_data.so](libreta_data.md) — data, words, aliases, CSV/catalog projections
- [libreta_parse.so](libreta_parse.md) — parsing, tokenization, and input morphisms
- [libreta_semantics.so](libreta_semantics.md) — semantics, selection spaces, topology, and presheaf
- [libreta_table.so](libreta_table.md) — tables, view state, width logic, and sheaf gluing
- [libreta_render.so](libreta_render.md) — rendering functors, especially GrundStrukHtml
- [libreta_arch.so](libreta_arch.md) — architecture metadata, category, morphism, and topology
- [libreta_runtime.so](libreta_runtime.md) — execution network and heavy Reta core carrier
- [libretaprompt_commands.so](libretaprompt_commands.md) — retaPrompt command side and command morphisms
- [libretaprompt_input.so](libretaprompt_input.md) — retaPrompt input, autocomplete, autosuggest, and history

## Shell variables

The large documentation of build, runtime, and internal shell variables is in `RETA_SHELL_VARIABLES_EN.md` and `doc/shell-variables/en/README.md`.

## Supplementary shell-variable documentation

The shared-library topology is controlled not only by Rust code and C launchers, but also by build, linker, loader, and runtime variables. The German documentation is in:

```text
RETA_SHELL_VARIABLES_DE.md
doc/shell-variables/de/README.md
```

The English version is in:

```text
RETA_SHELL_VARIABLES_EN.md
doc/shell-variables/en/README.md
```

For programmers, the important distinction is this: variables such as `RETA_LINK_CORE_SPLIT_LIBS`, `RETA_RENDER_LINK_SEMANTICS`, and `RETA_RUNTIME_LINK_CORE_COMPONENTS` are build-topology switches. Variables such as `RETA_LIB_PATH`, `RETA_RENDER_LIB_PATH`, `RETA_CSV_PATH`, and `LD_LIBRARY_PATH` are runtime/loader helpers. None of these variables is a reason to move algorithms back into executables.

The prompt launcher size policy is also documented in `PROMPT_LAUNCHER_SIZE_POLICY_EN.md`; the German version is in `PROMPT_LAUNCHER_SIZE_POLICY_DE.md`.
