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

Die Rust-Frontend-Binaries bleiben für lokale Cargo-Tests sichtbar. Dadurch funktionieren Befehle wie:

```bash
cargo run --bin rrp -- -h
cargo run --bin rrpl -- -h
cargo run --bin rrpe -- -h
cargo run --bin rrpb -- -h
```

Der normale `build.sh`- und Paketweg baut diese Rust-Frontend-Binaries trotzdem nicht als finale Executables. Die finalen Dateien werden nach dem Library-Build durch die kleinen C-Launcher ersetzt. Wer die Rust-Frontend-Binaries zusätzlich testweise bauen will, kann das explizit tun:

```bash
RETA_BUILD_RUST_FRONTEND_BINS=1 ./build.sh debug
```

Der aktive Paketweg bleibt aber die `.so`-Variante:

```bash
./tools/package_prompt_split_sharedlibs.sh release
```
