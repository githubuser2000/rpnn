# retaPrompt shared libraries: executable split state

## Fachliche Zieltrennung

Die aktive Aufteilung ist:

- `libreta.so`
  - öffentliche `reta`-ABI und stabile Fassade
  - hängt intern an den sieben privaten Reta-Core-Bibliotheken:
    - `libreta_data.so`
    - `libreta_parse.so`
    - `libreta_semantics.so`
    - `libreta_table.so`
    - `libreta_render.so`
    - `libreta_arch.so`
    - `libreta_runtime.so`
- `libretaprompt_commands.so`
  - retaPrompt-Command-Seite
  - öffentliche ABI:
    - `retaprompt_commands_run_kind_from_env`
    - `retaprompt_commands_run_current_executable_from_env`
    - `retaprompt_commands_run_rp_from_env`
    - `retaprompt_commands_run_rpl_from_env`
    - `retaprompt_commands_run_rpb_from_env`
    - `retaprompt_commands_run_rpe_from_env`
- `libretaprompt_input.so`
  - retaPrompt-Input-Seite für `rrp`, `rrpl`, `rrpe`
  - Prompt-Eingabe, Autocomplete und Autosuggest
  - öffentliche ABI:
    - `retaprompt_input_run_kind_from_env`
    - `retaprompt_input_run_current_executable_from_env`
    - `retaprompt_input_run_any_current_executable_from_env`
    - `retaprompt_input_run_launcher_kind_from_env`
    - `retaprompt_input_run_rp_from_env`
    - `retaprompt_input_run_rpl_from_env`
    - `retaprompt_input_run_rpe_from_env`

## Direkte Executable-Zuordnung

Der normale Build erzeugt die Prompt-Executables als kleine C-Launcher:

- `rreta` -> `libreta.so` -> `libreta_data.so` + `libreta_parse.so` + `libreta_semantics.so` + `libreta_table.so` + `libreta_render.so` + `libreta_arch.so` + `libreta_runtime.so`
- `rrpb` -> `libretaprompt_commands.so`
- `rrp`  -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpl` -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpe` -> `libretaprompt_input.so` + `libretaprompt_commands.so`

`rrpb` benutzt nur die Command-Library. `rrp`, `rrpl` und `rrpe` benutzen beide
Prompt-Libraries, weil dort Prompt-Eingabe mit Autocomplete/Autosuggest und die
Command-Seite zusammengehören.

## Build

```bash
./build.sh release
```

oder:

```bash
./build.sh debug
```

Für die geprüfte Shared-Library-Variante:

```bash
./tools/build_prompt_split_sharedlibs.sh release
```

Zum Paketieren:

```bash
./tools/package_prompt_split_sharedlibs.sh release
```

## Was aus den Executables herausgezogen wurde

`build.sh` baut standardmäßig nur die Library-Ziele und nicht die Rust-Tool- oder
Rust-Frontend-Binaries. Stattdessen werden nach dem Library-Build diese Launcher
erzeugt:

- `tools/launchers/rp.c`
- `tools/launchers/rpl.c`
- `tools/launchers/rpe.c`
- `tools/launchers/rpb.c`
- `tools/launchers/reta.c`

Damit bleibt in den finalen Prompt-Executables praktisch nur der ABI-Sprung in die
`.so`-Libraries. Die interaktiven Launcher enthalten zusätzlich einen bewusst gesetzten
Command-ABI-Anker, damit `libretaprompt_commands.so` nicht durch Linker-`--as-needed`
aus der Abhängigkeitsliste herausfällt.

Die Cargo-Frontend-Binaries bleiben für lokale Smoke-Tests sichtbar. Dadurch funktionieren Befehle wie:

```bash
cargo run --bin rrp -- -h
cargo run --bin rrpl -- -h
cargo run --bin rrpe -- -h
cargo run --bin rrpb -- -h
```

Der normale `build.sh`- und Paketweg baut sie trotzdem nicht als finale Executables. Wer sie zusätzlich testweise bauen will, kann das explizit tun:

```bash
RETA_BUILD_RUST_FRONTEND_BINS=1 ./build.sh debug
```

Danach werden die finalen `rrp`, `rrpl`, `rrpe`, `rrpb` wieder durch die
C-Launcher ersetzt.


## Autocomplete und Autosuggest mitten in der Eingabe

`libretaprompt_input.so` unterstützt Autocomplete und Autosuggest jetzt auch dann,
wenn der Cursor nicht am Ende der Eingabe steht. Die Completion ersetzt den Token
unter dem Cursor bis zum Token-Ende statt nur am Zeilenende anzuhängen. Autosuggest
erzeugt in diesem Fall eine `ReplaceRange`-Aktion, damit die rechte Pfeiltaste den
vorhandenen Token-Teil ersetzt. History-Hints bleiben absichtlich auf das Zeilenende
begrenzt, weil sie komplette frühere Eingaben fortsetzen.

## Maschinelle Prüfung

Der Build prüft mit `readelf`, falls verfügbar:

- `rrp`, `rrpl`, `rrpe` brauchen `libretaprompt_input.so` und `libretaprompt_commands.so`
- `rrpb` braucht `libretaprompt_commands.so`
- `rrpb` braucht nicht `libretaprompt_input.so`

Zusätzlich prüft `tools/build_prompt_split_sharedlibs.sh` die exportierten ABI-Symbole von
`libreta.so`, `libreta_runtime.so`, den sieben privaten Reta-Core-Bibliotheken und den zwei Prompt-Bibliotheken. Außerdem bricht der Build ab, wenn `libreta.so` nicht kleiner als `libreta_runtime.so` ist.

## Ehrliche Grenze

Diese Änderung minimiert die finalen Executables, korrigiert deren dynamische
Library-Zuordnung und sorgt jetzt auch dafür, dass `libreta.so` nur noch die dünne
öffentliche Fassade ist. Der schwere nicht-interaktive Kern liegt in
`libreta_runtime.so`. Sie garantiert noch nicht, dass zwischen allen Rust-`cdylib`-Dateien
selbst keinerlei Rust-Code doppelt eingebettet ist. Dafür müsste die interne
Rust-Code-Besitzstruktur weiter auf feinere echte ABI-Grenzen umgebaut werden.

## Reta-Core-Split

Die neue Reta-Core-Topologie ist separat dokumentiert:

- Deutsch: `RETA_SHARED_LIBS_DE.md` und `doc/shared-libs/de/README.md`
- English: `RETA_SHARED_LIBS_EN.md` und `doc/shared-libs/en/README.md`

Die Kernregel lautet: `rreta` hängt direkt nur an `libreta.so`; `libreta.so` hängt an den
sieben privaten Core-Bibliotheken. Damit bleibt die öffentliche ABI stabil, während
Daten, Parsing, Semantik, Tabelle, Rendering, Architektur und Runtime als eigene
`.so`-Grenzen aktiviert sind.
