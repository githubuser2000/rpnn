# retaprompt_input

`retaprompt_input` ist die Eingabe-Shared-Library für die interaktiven Prompt-Frontends:

```text
rrp  -> libretaprompt_input.so + libretaprompt_commands.so
rrpl -> libretaprompt_input.so + libretaprompt_commands.so
rrpe -> libretaprompt_input.so + libretaprompt_commands.so
```

`rrpb` ist absichtlich nicht Teil dieser Eingabeschicht. `rrpb` ist command-only und hängt nur an `libretaprompt_commands.so`.

## Aufgabe der Library

Diese Crate enthält die Logik für:

- interaktive Prompt-Eingabe,
- Prompt-Modi,
- Completion-Kontext,
- Autocomplete,
- Autosuggest,
- Cursor-Positionen innerhalb einer bereits eingegebenen Zeile,
- rechte-Pfeiltaste-Aktionen,
- Weiterleitung zur Command-Library.

Die finalen Executables sollen diese Logik nicht enthalten. Sie werden im Paketweg als kleine C-Launcher gebaut. Die Launcher springen nur in die ABI von `libretaprompt_input.so` oder `libretaprompt_commands.so`.

## Öffentliche ABI

Die Library exportiert unter anderem:

```c
int retaprompt_input_run_kind_from_env(int kind);
int retaprompt_input_run_current_executable_from_env(void);
int retaprompt_input_run_any_current_executable_from_env(void);
int retaprompt_input_run_launcher_kind_from_env(int kind);
int retaprompt_input_run_rp_from_env(void);
int retaprompt_input_run_rpl_from_env(void);
int retaprompt_input_run_rpe_from_env(void);
char *retaprompt_input_autosuggestion_at_cursor_json(const char *line, size_t cursor);
void retaprompt_input_free_string(char *ptr);
```

Der Header liegt in:

```text
crates/retaprompt_input/include/retaprompt_input.h
```

## Cursor-lokaler Autosuggest

Autocomplete funktionierte bereits für Textstellen mitten in der Eingabe. Der kritische Punkt war Autosuggest: `reedline` rendert Hints normalerweise am Ende der Zeile. Dadurch erschien der Vorschlag erst am Textende, obwohl die Completion den Token unter dem Cursor richtig erkannte.

Die Lösung liegt in dieser Library:

```text
src/prompt/completion.rs
```

Die Completion-Schicht berechnet für die aktuelle Cursorposition eine `ReplaceRange`-Aktion und zusätzlich einen cursor-lokalen Ghost-Text. Beispiel:

```text
reta -ze --zeit=heute
        ^ Cursor nach -ze
```

Die Library berechnet:

```text
replace_start = 5
replace_len   = 3
replacement   = -zeilen
cursor_ghost  = ilen
tail_after_replace =  --zeit=heute
```

Das sichtbare Ergebnis ist logisch:

```text
reta -ze[ilen] --zeit=heute
```

Die rechte Pfeiltaste akzeptiert nicht bloß einen Suffix. Sie ersetzt den passenden Tokenbereich durch die kanonische Completion. Genau deshalb ist die Aktion als `RightArrowAcceptAction::ReplaceRange` modelliert.

## ABI-Diagnose für Cursor-Autosuggest

Für Tests und externe Tools gibt es:

```c
char *retaprompt_input_autosuggestion_at_cursor_json(const char *line, size_t cursor);
void retaprompt_input_free_string(char *ptr);
```

Beispielausgabe:

```json
{
  "present": true,
  "cursor": 8,
  "display": " → -zeilen",
  "insert": "",
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

Der Rückgabestring ist heap-allokiert und muss mit `retaprompt_input_free_string` freigegeben werden. Diese Ownership-Regel ist dieselbe C-ABI-Regel wie bei anderen String-Rückgaben im Projekt: wer den String von der Library bekommt, gibt ihn mit der passenden Free-Funktion derselben Library zurück.

## Nicht in die Executables verschieben

Wenn Autocomplete, Autosuggest oder rechte-Pfeiltaste-Verhalten geändert werden, dann gehört die Änderung hierhin:

```text
src/prompt/completion.rs
crates/retaprompt_input/src/lib.rs
crates/retaprompt_input/include/retaprompt_input.h
```

Nicht hierhin:

```text
tools/launchers/rp.c
tools/launchers/rpl.c
tools/launchers/rpe.c
tools/launchers/rpb.c
```

Die C-Launcher bleiben absichtlich dumm. Sie sollen keine Tokenisierung, keine Completion-Kandidaten, keine History-Logik und keine ANSI-Hint-Logik kennen.

## Build- und Paketpfad

```bash
./build.sh release
./tools/package_prompt_split_sharedlibs.sh release
```

Der Build prüft, dass `libretaprompt_input.so` die Cursor-Autosuggest-Symbole exportiert. Außerdem prüft er, dass `rrp`, `rrpl` und `rrpe` an `libretaprompt_input.so` und `libretaprompt_commands.so` hängen, während `rrpb` nur die Command-Library benötigt.

## Weitere Dokumentation

```text
RETA_SHARED_LIBS_DE.md
RETA_SHARED_LIBS_EN.md
RETA_SHELL_VARIABLES_DE.md
RETA_SHELL_VARIABLES_EN.md
doc/shared-libs/de/libretaprompt_input.md
doc/shared-libs/en/libretaprompt_input.md
doc/shell-variables/de/README.md
doc/shell-variables/en/README.md
```
