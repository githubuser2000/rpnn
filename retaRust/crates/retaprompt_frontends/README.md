# retaprompt_frontends

Diese Crate bleibt als historische Rust-Frontend-Schicht im Repository. Im normalen
`build.sh` wird sie absichtlich nicht mehr gebaut, damit die finalen Prompt-Executables
nicht wieder Rust-Frontend-Code enthalten.

Aktive Executables entstehen stattdessen als kleine C-Launcher aus `tools/launchers`:

- `rrp`  -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpl` -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpe` -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpb` -> `libretaprompt_commands.so`

`rrpb` ist command-only. `rrp`, `rrpl` und `rrpe` hängen an beiden Libraries, weil die
Prompt-Frontends Eingabe/Autocomplete/Autosuggest und Command-Logik gemeinsam brauchen.

Die Rust-Frontend-Binaries sind hinter der Feature-Flag `rust-frontends` gated und können weiterhin explizit für Experimente gebaut werden:

```bash
RETA_BUILD_RUST_FRONTEND_BINS=1 ./build.sh debug
```

Der aktive Paketweg bleibt aber die `.so`-Variante:

```bash
./tools/package_prompt_split_sharedlibs.sh release
```
