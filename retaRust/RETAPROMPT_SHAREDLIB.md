# retaPrompt shared libraries: executable split state

## Fachliche Zieltrennung

Die aktive Aufteilung ist:

- `libreta.so`
  - `reta`-Kern und öffentliche `reta`-ABI
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

- `rreta` -> `libreta.so`
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

`build.sh` baut standardmäßig nicht mehr die Rust-Frontend-Binaries aus
`retaprompt_frontends`. Stattdessen werden nach dem Library-Build diese Launcher
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

Wer die alten Rust-Frontend-Binaries trotzdem testweise bauen will, kann das explizit tun. Die Cargo-Bins sind dafür hinter der Feature-Flag `rust-frontends` gated; `build.sh` setzt sie nur in diesem expliziten Modus:

```bash
RETA_BUILD_RUST_FRONTEND_BINS=1 ./build.sh debug
```

Danach werden die finalen `rrp`, `rrpl`, `rrpe`, `rrpb` trotzdem wieder durch die
C-Launcher ersetzt.

## Maschinelle Prüfung

Der Build prüft mit `readelf`, falls verfügbar:

- `rrp`, `rrpl`, `rrpe` brauchen `libretaprompt_input.so` und `libretaprompt_commands.so`
- `rrpb` braucht `libretaprompt_commands.so`
- `rrpb` braucht nicht `libretaprompt_input.so`

Zusätzlich prüft `tools/build_prompt_split_sharedlibs.sh` die exportierten ABI-Symbole der
drei `.so`-Libraries.

## Ehrliche Grenze

Diese Änderung minimiert die finalen Executables und korrigiert deren dynamische
Library-Zuordnung. Sie garantiert noch nicht, dass zwischen den Rust-`cdylib`-Dateien
selbst keinerlei Rust-Code doppelt eingebettet ist. Dafür müsste die interne
Rust-Code-Besitzstruktur weiter auf echte ABI-Grenzen umgebaut werden.
