# Reta shared-library topology — English programmer documentation

This documentation describes **all currently built `.so` libraries** of the Reta/retaPrompt architecture. It is written for programmers: ABI, headers, ownership, build checks, `DT_NEEDED`, RPATH/RUNPATH, common regressions, and extension rules.

## Target topology

```text
rreta
  -> libreta.so
       -> libreta_data.so
       -> libreta_parse.so
       -> libreta_semantics.so
       -> libreta_table.so
       -> libreta_render.so
       -> libreta_arch.so
       -> libreta_runtime.so

rgrundStrukHtml
  -> libreta_render.so
       -> libreta_semantics.so

rrp / rrpl / rrpe
  -> libretaprompt_input.so
  -> libretaprompt_commands.so

rrpb
  -> libretaprompt_commands.so
```

## Principle

Final executables stay small. Program logic lives in `.so` libraries. `libreta.so` is the thin public facade; `libreta_runtime.so` carries the heavy non-interactive core. `rgrundStrukHtml` uses `libreta_render.so` directly. `rrpb` remains command-only. `rrp`, `rrpl`, and `rrpe` use both the input and command libraries.

## Autocomplete/autosuggest boundary

Autocomplete and autosuggest belong to `libretaprompt_input.so`. The C launchers contain no algorithm for that. Mid-cursor autosuggest is computed in the shared library and is additionally exported as ABI diagnostics via `retaprompt_input_autosuggestion_at_cursor_json`.

## Per-library documents

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

## Additional documentation

- `RETA_SHARED_LIBS_DE.md` — German root overview.
- `RETA_SHARED_LIBS_EN.md` — English root overview.
- `RETA_SHELL_VARIABLES_DE.md` — large German documentation of shell/environment variables.
- `RETA_SHELL_VARIABLES_EN.md` — large English documentation of shell/environment variables.
- `doc/shell-variables/de/README.md` and `doc/shell-variables/en/README.md` — packageable variant of the variable documentation.
