# libretaprompt_input.so — retaPrompt-Eingabe, Autocomplete, Autosuggest und History

## Kurzfassung für Programmierer

`libretaprompt_input.so` ist die Shared-Library-Grenze für: **retaPrompt-Eingabe, Autocomplete, Autosuggest und History**.

Diese Bibliothek enthält den interaktiven Reedline-Prompt. Cursor-Mitte-Autocomplete und Cursor-Mitte-Autosuggest liegen hier in der Shared-Library, nicht im C-Launcher.

Diese Datei ist absichtlich ausführlich. Sie soll nicht nur sagen, _was_ gebaut wird, sondern auch, _warum_ diese ABI-Grenze existiert, welche Abhängigkeiten erlaubt sind, wie Speicherbesitz funktioniert, welche Fehlerbilder typisch sind und wie man die Bibliothek in der Praxis prüft.

## Artefakt und Quellorte

| Feld | Wert |
|---|---|
| Artefakt | `target/<profile>/libretaprompt_input.so` |
| Crate | `retaprompt_input` |
| Rust-Quelle | `crates/retaprompt_input/src/lib.rs` |
| C-Header | `crates/retaprompt_input/include/retaprompt_input.h` |
| Dokumentation | `doc/shared-libs/de/libretaprompt_input.md` und `doc/shared-libs/en/libretaprompt_input.md` |
| Build-Profil | `debug` oder `release` über `./build.sh <profile>` |

## Direkte Nutzer

- `rrp`
- `rrpl`
- `rrpe`

## Direkte dynamische Abhängigkeiten

- `libretaprompt_commands.so`

Wichtig: „direkt“ bedeutet hier `DT_NEEDED` oder bewusstes dynamisches Laden. Transitive Abhängigkeiten zählen nicht als direkte Verantwortung dieser Bibliothek. Genau diese Unterscheidung ist wichtig, damit `rrpb` command-only bleibt, `rrp/rrpl/rrpe` beide Prompt-Libraries tragen und `rgrundStrukHtml` direkt an `libreta_render.so` hängt.

## Architekturgrenze

Diese Bibliothek ist eine echte ABI-Grenze. Der Code hinter der Grenze darf sich intern ändern, solange die C-Oberfläche stabil bleibt. Die Grenze ist nicht dafür gedacht, beliebige Rust-Objekte nach außen zu leaken. Über die Grenze gehen nur:

- Ganzzahlen mit fester Breite wie `uint32_t`, `uint64_t`, `int32_t`, `size_t`, `uint8_t`,
- C-Strings als `const char *` für geliehene statische Daten,
- C-Strings als `char *` für allozierte Rückgaben,
- einfache C-Strukturen, wenn der Header sie ausdrücklich definiert,
- Exitcodes oder JSON als textuelle, sprachneutrale Datenform.

Nicht erlaubt sind als ABI-Vertrag:

- Rust-Referenzen,
- Rust-`String`, `Vec`, `HashMap`, `BTreeMap`, `IndexMap`,
- Panic über die ABI-Grenze,
- implizite Ownership nach dem Motto „der Aufrufer wird schon wissen, wer freigibt“,
- inoffizielle Symbole ohne Dokumentation.

## Mathematische Rolle

bidirektionaler Kanal: Benutzerzustand und Promptzustand werden über lokale Cursor-Sektionen gekoppelt.

Diese mathematische Rolle ist kein Schmuck. Sie ist eine praktische Architekturregel: ähnliche Morphismen gehören in dieselbe Library-Familie, aber nicht jede kleine Funktion bekommt eine eigene `.so`. Dadurch bleibt die Topologie verständlich und die Loader-/ABI-Komplexität beherrschbar.

## Öffentliche ABI-Symbole

- `retaprompt_input_run_kind_from_env`
- `retaprompt_input_run_current_executable_from_env`
- `retaprompt_input_run_any_current_executable_from_env`
- `retaprompt_input_run_launcher_kind_from_env`
- `retaprompt_input_run_rp_from_env`
- `retaprompt_input_run_rpl_from_env`
- `retaprompt_input_run_rpe_from_env`
- `retaprompt_input_autosuggestion_at_cursor_json`
- `retaprompt_input_free_string`

Als maschinenlesbare Sicht:

```text
retaprompt_input_run_kind_from_env
retaprompt_input_run_current_executable_from_env
retaprompt_input_run_any_current_executable_from_env
retaprompt_input_run_launcher_kind_from_env
retaprompt_input_run_rp_from_env
retaprompt_input_run_rpl_from_env
retaprompt_input_run_rpe_from_env
retaprompt_input_autosuggestion_at_cursor_json
retaprompt_input_free_string
```

## Speicherbesitz

JSON-Strings von retaprompt_input_autosuggestion_at_cursor_json; mit retaprompt_input_free_string freigeben.

Die Cursorposition für `retaprompt_input_autosuggestion_at_cursor_json` ist ein Byte-Offset in einem UTF-8-String. Liegt der Offset mitten in einem Codepoint, klemmt die Library ihn auf die vorherige gültige Zeichenkante.

Grundregel für alle Reta- und retaPrompt-Shared-Libraries:

```c
char *ptr = some_library_function(...);
/* ptr lesen, kopieren oder ausgeben */
some_matching_library_free_string(ptr);
```

Falsch ist:

```c
char *ptr = reta_data_shared_words_json();
reta_free_string(ptr);              /* falsch: falsche Library */
free(ptr);                          /* falsch: falscher Allocator */
```

Richtig ist:

```c
char *ptr = reta_data_shared_words_json();
reta_data_free_string(ptr);         /* richtig: gleiche ABI-Familie */
```

## Fehler- und Panic-Modell

Die ABI soll keine Rust-Panics nach C propagieren. Eintrittspunkte, die externe Programme starten oder Exitcodes liefern, sind mit Guard-Funktionen abgesichert. Bei Funktionen, die Strings zurückgeben, sind robuste Clients trotzdem defensiv:

- Null-Pointer prüfen,
- ungültige Eingaben vermeiden,
- UTF-8-Annahmen explizit halten,
- JSON nicht blind ausführen, sondern parsen,
- Exitcodes nicht ignorieren.

## Threading und Reentrancy

Die Bibliothek ist nicht als global mutierender Singleton-Vertrag zu verstehen. Trotzdem gibt es intern Caches, `OnceLock`s oder Runtime-Initialisierung. Für Programmierer heißt das:

- parallele Nutzung ist nur dort sicher, wo keine mutable Session geteilt wird,
- C-Strings nach Rückgabe gehören dem Aufrufer bis zur passenden Free-Funktion,
- globale Umgebungsvariablen wie `RETA_CSV_PATH` oder `RETA_LIB_PATH` sollten vor dem ersten Aufruf gesetzt werden,
- Tests mit wechselnden Umgebungsvariablen sollten Prozesse isolieren.

## Build-Pfad

Typischer Build:

```bash
./build.sh release
```

Geprüfter Shared-Library-Build:

```bash
./tools/build_prompt_split_sharedlibs.sh release
```

Paketierung:

```bash
./tools/package_prompt_split_sharedlibs.sh release
```

Wichtige Build-Regeln:

- Es werden dynamische `.so`-Bibliotheken gebaut, keine `.a`-Archive.
- Die finalen öffentlichen Executables werden im normalen Paketpfad als kleine C-Launcher erzeugt.
- `libreta.so` bleibt Fassade.
- `libreta_runtime.so` trägt den schweren Reta-Kern.
- `libreta_data.so`, `libreta_parse.so`, `libreta_semantics.so`, `libreta_table.so` und `libreta_render.so` dürfen nicht alle exakt dieselbe Stub-Größe haben.

## Dynamische Link-Prüfung

Nützliche Befehle:

```bash
readelf -d target/release/libretaprompt_input.so
nm -D --defined-only target/release/libretaprompt_input.so
```

Bei Launchern zusätzlich:

```bash
readelf -d target/release/rreta
readelf -d target/release/rgrundStrukHtml
readelf -d target/release/rrp
readelf -d target/release/rrpb
```

Die erwartete Topologie ist nicht nur Kosmetik. Sie verhindert, dass Code wieder in Executables oder falsche `.so`-Träger zurückwandert.

## RPATH/RUNPATH und Installation

Die Launcher werden mit Suchpfaden gebaut, die diese Layouts unterstützen:

```text
$ORIGIN
$ORIGIN/lib
$ORIGIN/../lib
```

Für Termux wird typischerweise nach `$HOME/../usr/bin` und `$HOME/../usr/lib` kopiert. Für portable Pakete können Executables neben den Libraries liegen oder in `bin/` mit Libraries in `../lib/`.

## Typische Regressionen

| Symptom | Wahrscheinliche Ursache | Prüfung |
|---|---|---|
| Library fehlt bei Programmstart | RPATH/RUNPATH oder Installationslayout falsch | `readelf -d <executable>` |
| Symbol fehlt | Crate nicht als `cdylib` gebaut oder Export entfernt | `nm -D --defined-only` |
| Executable wieder sehr groß | Rust-Binary statt C-Launcher verwendet | `file`, `readelf -d`, Buildskript prüfen |
| Alle Komponenten gleich groß | Komponenten sind wieder leere ABI-Stubs | Größenprüfung in `build.sh` |
| Fassade wieder riesig | `split-facade` nicht aktiv oder Engine in `libreta.so` | Größenregel `libreta.so < libreta_runtime.so` |
| Crash beim Freigeben | falsche Free-Funktion oder `free()` verwendet | Ownership-Regeln prüfen |

## Test- und Review-Checkliste

- ABI-Version vor Nutzung prüfen, wenn ein Client direkt lädt.
- Keine Rust-Typen über die C-Grenze reichen.
- Alle von dieser Library allozierten Strings mit der passenden Free-Funktion derselben Library freigeben.
- Keine Free-Funktion einer anderen Library verwenden, auch wenn der Typ `char *` identisch aussieht.
- Bei Paketierung RPATH/RUNPATH und `DT_NEEDED` mit `readelf -d` prüfen.
- Bei Symbolfragen `nm -D --defined-only` verwenden.
- Bei Größenregressionen prüfen, ob eine Library wieder nur Stub-Code trägt oder ob schwerer Code in die falsche .so gerutscht ist.
- Keine zyklischen öffentlichen ABI-Abhängigkeiten einführen.

## Erweiterungsregeln

Neue Funktionen sollten zuerst einer Verantwortung zugeordnet werden:

1. Daten/Katalog? Dann `libreta_data.so`.
2. Text/argv/Token? Dann `libreta_parse.so`.
3. Auswahl, Bedeutung, Parameterraum? Dann `libreta_semantics.so`.
4. Tabelle, View, Breite, Materialisierung? Dann `libreta_table.so`.
5. Ausgabeformat, HTML, BBCode, Plaintext? Dann `libreta_render.so`.
6. Architekturmetadaten, Topologie, Morphismuszählung? Dann `libreta_arch.so`.
7. Ausführung, Engine, Scheduler, Queue, Semaphore? Dann `libreta_runtime.so`.
8. Prompt-Command ohne interaktive Eingabe? Dann `libretaprompt_commands.so`.
9. Prompt-Eingabe, Completion, Suggest, History? Dann `libretaprompt_input.so`.
10. Öffentliche Reta-ABI? Nur dann `libreta.so`.

## Beispielhafte C-Nutzung

```c
#include "retaprompt_input.h"

int main(void) {
    /* Dieses Beispiel ist bewusst generisch. Details stehen im jeweiligen Header. */
    return 0;
}
```

## Wartungsnotiz

Diese Dokumentation gehört zur ABI. Wenn ein Symbol ergänzt, entfernt oder semantisch geändert wird, muss diese Datei zusammen mit dem Header und den Build-Prüfungen angepasst werden. Eine `.so`-Grenze ohne Dokumentation ist in diesem Projekt als unvollständig zu behandeln.
