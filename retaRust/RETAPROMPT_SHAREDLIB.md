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
    - `retaprompt_input_autosuggestion_at_cursor_json`
    - `retaprompt_input_free_string`

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



## Cursor-lokaler Autosuggest in `libretaprompt_input.so`

Die sichtbare Autosuggest-Logik liegt jetzt ausdrücklich in der Input-Shared-Library und nicht in den Executables. Das ist wichtig, weil `rrp`, `rrpl` und `rrpe` als kleine C-Launcher keine Prompt-Algorithmen enthalten sollen. Die relevante Rust-Quelle wird in `libretaprompt_input.so` kompiliert:

```text
src/prompt/completion.rs
crates/retaprompt_input/src/lib.rs
crates/retaprompt_input/include/retaprompt_input.h
```

Die Library berechnet nicht nur Vorschläge am Zeilenende, sondern auch Vorschläge für den Token unter dem Cursor. Wenn der Nutzer den Cursor in eine bereits eingegebene Zeile zurückbewegt und dort tippt, wird der aktuelle Tokenbereich analysiert. Daraus entsteht eine `ReplaceRange`-Aktion. Die rechte Pfeiltaste ersetzt dann den vorhandenen Tokenbereich, statt einen Suffix ans Zeilenende zu hängen.

Beispiel:

```text
reta -ze --zeit=heute
        ^ Cursor nach -ze
```

Die Library erkennt `-ze` als Fragment, berechnet `-zeilen` als kanonische Ersetzung und stellt den sichtbaren Ghost-Text lokal am Cursor dar:

```text
reta -ze[ilen] --zeit=heute
```

Technisch wird dafür ein ANSI-Hint erzeugt, der vom Zeilenende an die Cursorposition zurückspringt, den Ghost-Text und den erhaltenen Zeilentail rendert und den Terminalcursor wieder an die logische Eingabeposition zurücksetzt. Das ist notwendig, weil `reedline::Hinter` den Hint normalerweise am Ende der Zeile ausgibt.

Die zusätzliche ABI-Diagnose ist:

```c
char *retaprompt_input_autosuggestion_at_cursor_json(const char *line, size_t cursor);
void retaprompt_input_free_string(char *ptr);
```

Damit kann ein Testprogramm prüfen, ob die Shared-Library die Cursorposition richtig interpretiert. Der zurückgegebene JSON-String enthält unter anderem:

```json
{
  "present": true,
  "cursor": 8,
  "replace_start": 5,
  "replace_len": 3,
  "replacement": "-zeilen",
  "cursor_ghost": "ilen",
  "tail_after_replace": " --zeit=heute",
  "is_cursor_local": true,
  "accept_action": {
    "kind": "replace_range",
    "replace_start": 5,
    "replace_len": 3,
    "replacement": "-zeilen"
  }
}
```

Die Cursorposition ist ein Byte-Offset in einem UTF-8-String. Liegt der Offset mitten in einem UTF-8-Codepoint, klemmt die Library ihn auf die vorherige gültige Zeichenkante.

Der Speicher des Rückgabestrings gehört der Library. Jeder erfolgreiche Rückgabewert muss mit `retaprompt_input_free_string` freigegeben werden. Die C-Launcher rufen diese Diagnose-ABI nicht auf; sie ist für Tests, externe Tools und ABI-Nachweise gedacht. Die echte interaktive Prompt-Logik verwendet dieselbe Completion-Schicht intern.

## Prüfpunkte für Entwickler

- `nm -D libretaprompt_input.so` muss `retaprompt_input_autosuggestion_at_cursor_json` und `retaprompt_input_free_string` zeigen.
- `rrp`, `rrpl` und `rrpe` dürfen keine eigene Completion-Logik enthalten.
- `rrpb` darf weiterhin nicht an `libretaprompt_input.so` hängen.
- History-Autosuggest bleibt am Zeilenende, weil History komplette frühere Eingaben fortsetzt.
- Kontext-Autosuggest für Reta-Parameter, Werte und Struktur-Switches ist cursor-lokal.
- Die rechte Pfeiltaste muss `ReplaceRange` akzeptieren können.
- Änderungen an Anzeige, ANSI-Sequenzen oder Tokenbereich gehören in `src/prompt/completion.rs`, nicht in `tools/launchers/*.c`.

## Dokumentation

Die vollständige `.so`-Dokumentation liegt in:

```text
RETA_SHARED_LIBS_DE.md
RETA_SHARED_LIBS_EN.md
doc/shared-libs/de/*.md
doc/shared-libs/en/*.md
```

Die vollständige Shellvariablen-Dokumentation liegt in:

```text
RETA_SHELL_VARIABLES_DE.md
RETA_SHELL_VARIABLES_EN.md
doc/shell-variables/de/README.md
doc/shell-variables/en/README.md
```

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
