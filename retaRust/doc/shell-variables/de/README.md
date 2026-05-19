# Reta/RetaPrompt Shellvariablen — große Programmierer-Dokumentation

Diese Datei dokumentiert die Shell- und Umgebungsvariablen, die im aktuellen `.so`-Build, in den Runtime-Pfaden, in retaPrompt, in Termux-Skripten und in der Architektur-/Paritätsdiagnose relevant sind. Sie ist bewusst ausführlich: Ein Programmierer soll daraus ableiten können, welche Variable zur Build-Zeit, zur Link-Zeit, zur Laufzeit oder nur innerhalb eines Skripts Bedeutung hat.

Die wichtigste Regel dieser Version lautet: **Launcher bleiben klein. Logik gehört in `.so`-Libraries.** Deshalb sind Variablen wie `RETA_LIB_PATH` und `RETA_RENDER_LIB_PATH` nur Pfad-/Loader-Hilfen. Autocomplete, Autosuggest, Command-Ausführung, Rendering, Semantik und Runtime-Topologie werden in Libraries umgesetzt.

## Schnellstart

```bash
# normaler Shared-Library-Build
./build.sh release

# Paket erzeugen
./tools/package_prompt_split_sharedlibs.sh release

# wenn ein Launcher die Library nicht findet
RETA_LIB_PATH=target/release/libreta.so target/release/rrp
RETA_RENDER_LIB_PATH=target/release/libreta_render.so target/release/rgrundStrukHtml blank

# parallele Ausführung konservativ begrenzen
RETA_PARALLEL_WORKERS=2 RETA_PARALLEL_THRESHOLD=512 ./target/release/rreta -h
```

## Schichtenmodell

| Schicht | Beispiele | Aufgabe | Darf Algorithmik enthalten? |
|---|---|---|---|
| C-Launcher | `rreta`, `rrp`, `rrpl`, `rrpe`, `rrpb`, `rgrundStrukHtml` | argv, Pfad, Exit-Code, ABI-Aufruf | Nein |
| Öffentliche Fassade | `libreta.so` | stabile Reta-ABI, Delegation an Core | Nur dünne Delegation |
| Core-Komponenten | `libreta_data.so`, `libreta_parse.so`, `libreta_semantics.so`, `libreta_table.so`, `libreta_render.so`, `libreta_arch.so`, `libreta_runtime.so` | Daten, Parser, Semantik, Tabelle, Render, Architektur, Runtime | Ja |
| Prompt-Komponenten | `libretaprompt_commands.so`, `libretaprompt_input.so` | Command-Ausführung, Eingabe, Autocomplete, Autosuggest | Ja |
| Shellvariablen | `RETA_*`, `CARGO_*`, Loader-Pfade | Konfiguration und Diagnose | Nein, nur Steuerdaten |

## Runtime-, Prompt- und Loader-Variablen

| Variable | Bereich | Bedeutung | Werte | Wann setzen? | Vorsicht |
|---|---|---|---|---|---|
| `RETA_LIB_PATH` | Runtime | Pfad zu `libreta.so`, wenn `retaprompt_commands` die Reta-Fassade nicht neben dem Executable oder im Loader-Suchpfad findet. | Absoluter oder relativer Dateipfad. | Beim Pakettest, bei Termux-Kopien und bei Entwicklung außerhalb von RPATH/RUNPATH setzen. | Muss auf die Fassade zeigen, nicht auf `libreta_runtime.so`. |
| `RETA_RENDER_LIB_PATH` | Runtime | Pfad zu `libreta_render.so` für den dynamischen `rgrundStrukHtml`-Launcher. | Absoluter oder relativer Dateipfad. | Nützlich, wenn `rgrundStrukHtml` aus einem anderen Verzeichnis gestartet wird. | Nicht mit `RETA_LIB_PATH` verwechseln; HTML-Rendering geht über `libreta_render.so`. |
| `RETA_CSV_PATH` | Runtime | Pfad zum CSV-Datenverzeichnis der Reta-Datenbasis. | Verzeichnis. | Setzen, wenn die CSV-Dateien nicht im erwarteten Paketlayout liegen. | Bei falschem Pfad wirken Daten-/Aliasfunktionen unvollständig. |
| `RETA_BIN` | Prompt | Optionaler Pfad zu einer externen Reta-Ausführung, falls Prompt-Kommandos bewusst nicht über `libreta.so` laufen sollen. | Dateipfad. | Nur für Diagnose oder alte Setups. | Im neuen `.so`-Paradigma normalerweise nicht setzen. |
| `RETA_PROMPT_SESSION_LOG` | Prompt | Pfad für das Sitzungslog von retaPrompt. | Dateipfad. | Setzen, wenn Ein-/Ausgabe der Prompt-Sitzung nachvollziehbar gespeichert werden soll. | Keine geheimen Eingaben loggen, wenn die Datei geteilt wird. |
| `COLUMNS` | Terminal | Breite des Terminals für Wrap-, Tabellen- und Prompt-Anzeige. | Positive ganze Zahl. | Automatische Erkennung überschreiben, etwa in CI oder Pipe-Umgebungen. | Zu kleine Werte verschlechtern Tabellenlayout. |
| `LINES` | Terminal | Höhe des Terminals für Konsolen-/TUI-Funktionen. | Positive ganze Zahl. | Selten nötig; hilfreich für reproduzierbare Tests. | Nicht alle Pfade benutzen `LINES`. |
| `HOME` | System | Basis für Termux-Zielpfade und einige historische Skripte. | Verzeichnis. | Wird normalerweise vom System gesetzt. | Nicht im Build überschreiben, außer du weißt warum. |
| `LD_LIBRARY_PATH` | Loader | Zusätzliche Suchpfade des dynamischen Linux/Android-Linkers. | Doppelpunktgetrennte Verzeichnisliste. | Nur verwenden, wenn RPATH/RUNPATH oder Paketlayout nicht greifen. | Kann falsche `.so`-Versionen vorziehen; für Produktion besser RUNPATH verwenden. |
| `DYLD_LIBRARY_PATH` | Loader | macOS-Analog zu `LD_LIBRARY_PATH` für Entwicklungsports. | Doppelpunktgetrennte Verzeichnisliste. | Nur relevant auf macOS-Experimenten. | Dieses Projekt ist primär Linux/Termux-orientiert. |

## Build-, Cargo- und Link-Variablen

| Variable | Bereich | Bedeutung | Werte | Wann setzen? | Vorsicht |
|---|---|---|---|---|---|
| `CARGO_TARGET_DIR` | Cargo/Build | Alternatives Cargo-Target-Verzeichnis. | Verzeichnis. | Setzen, wenn Artefakte außerhalb von `target/` landen sollen. | Build- und Paket-Skripte leiten daraus `target/debug` oder `target/release` ab. |
| `RETA_LINK_CORE_SPLIT_LIBS` | Build | Aktiviert in `build.rs` die Link-Kante von `libreta.so` zu den privaten Core-`.so`s. | `1` oder unset. | Wird durch die Build-Skripte gesetzt. | Nicht manuell auf `0` setzen; sonst wird `libreta.so` wieder zu schwer. |
| `RETA_RENDER_LINK_SEMANTICS` | Build | Aktiviert die Link-Kante `libreta_render.so -> libreta_semantics.so`. | `1` oder unset. | Wird durch die Build-Skripte gesetzt. | Ohne diese Variable verliert `rgrundStrukHtml` die gewünschte Render/Semantik-Topologie. |
| `RETA_RUNTIME_LINK_CORE_COMPONENTS` | Build | Aktiviert die Link-Kanten von `libreta_runtime.so` zu `data`, `parse`, `semantics`, `table`, `render` und `arch`. | `1` oder unset. | Wird durch die Build-Skripte gesetzt. | Ohne diese Variable entstehen wieder isolierte Stubs. |
| `RETA_BUILD_RUST_TOOL_BINS` | Build | Baut zusätzlich schwere Rust-Diagnose- und Tool-Binaries. | `1` oder unset/`0`. | Nur bei Entwicklerdiagnose setzen. | Nicht für Paketgrößenmessung verwenden; finale öffentliche Binaries sind C-Launcher. |
| `RETA_BUILD_RUST_FRONTEND_BINS` | Build | Retired/gesperrt: frühere Variable zum Bauen schwerer Rust-Prompt-Frontends. | Muss unset oder `0` bleiben. | Nicht mehr setzen. | `1` bricht den Build absichtlich ab, damit `rrp/rrpl/rrpe/rrpb` nicht wieder groß werden. |
| `RETA_PROMPT_LAUNCHER_MAX_BYTES` | Build/Guard | Maximale erlaubte Größe eines Prompt-Launchers. | Positive Bytezahl, Standard `262144`. | Nur anheben, wenn C-Launcher auf einer Plattform legitime Zusatzgröße brauchen. | Nicht als Workaround für Rust-Payload erhöhen; erst `tools/guard_prompt_launcher_topology.sh` lesen. |
| `PROFILE` | Cargo/Script | Cargo-Profil; in Skripten aus dem ersten Argument `debug` oder `release` abgeleitet. | `debug` oder `release`. | Nicht direkt setzen; `./build.sh debug` oder `./build.sh release` verwenden. | Cargo selbst setzt `PROFILE` in Build-Skripten. |
| `OUT_DIR` | Cargo | Von Cargo gesetztes Ausgabeverzeichnis für Build-Skripte. | Verzeichnis. | Nicht manuell setzen. | Wird in Rust-`build.rs` für generierte Linker-Shims verwendet. |
| `CARGO_MANIFEST_DIR` | Cargo | Von Cargo gesetzter Pfad zum aktuellen Crate-Manifest. | Verzeichnis. | Nicht manuell setzen. | Hilfreich für Build-Script-Pfadberechnung. |
| `RUSTFLAGS` | Cargo | Zusätzliche Flags für rustc. | String. | Nur bewusst setzen, etwa für Linker-/Symboltests. | Kann die Größe und Link-Topologie stark verändern. |

## Parallelisierungsvariablen

| Variable | Bereich | Bedeutung | Werte | Wann setzen? | Vorsicht |
|---|---|---|---|---|---|
| `RETA_PARALLEL` | Parallel | Kompatibler Hauptschalter für Parallelisierung; Alias/Quelle für den Architektur-Parallelmodus. | `auto`, `off`, `threads`, `processes` oder ähnliche Moduswerte. | Schneller Test eines globalen Parallelmodus. | Bei Konflikt hat der spezifischere Modus Vorrang. |
| `RETA_PARALLEL_MODE` | Parallel | Expliziter Parallelmodus der Architektur-Schicht. | Modusstring. | Nutzen, wenn der Architekturpfad eindeutig gesteuert werden soll. | Dokumentiert und prüft besser als der Alias `RETA_PARALLEL`. |
| `RETA_PARALLEL_WORKERS` | Parallel | Anzahl Worker für parallele Ausführung. | Positive ganze Zahl. | Bei CI, Termux oder schwacher Hardware begrenzen. | Zu hoch kann Speicher, Scheduling und Ausgabe-Reihenfolge belasten. |
| `RETA_PARALLEL_CHUNK_SIZE` | Parallel | Chunk-Größe für Aufgabenbündel. | Positive ganze Zahl. | Tuning zwischen Scheduler-Overhead und Latenz. | Kleine Chunks erhöhen Overhead, große Chunks verschlechtern Balancing. |
| `RETA_PARALLEL_THRESHOLD` | Parallel | Mindestgröße, ab der parallelisiert wird. | Positive ganze Zahl. | Erhöhen, wenn kleine Aufgaben im Parallelmodus langsamer sind. | Zu niedrig macht einfache Tabellen unnötig teuer. |
| `RETA_PARALLEL_START_METHOD` | Parallel | Startmethode für Prozess-/Worker-Modell im Architekturkontext. | String. | Primär für Paritäts-/Python-Referenzpfade interessant. | Rust-Pfade verwenden nicht jeden Python-Startmethodenwert. |
| `RETA_JOBS` | Parallel | Worker-Anzahl im geteilten Runtime-Pfad; kompatibler Alias. | Positive ganze Zahl. | Kurzer Alias für Batch-/Generatorläufe. | Wird mit `RETA_THREADS`/`RETA_NUM_THREADS` zusammengeführt. |
| `RETA_THREADS` | Parallel | Thread-Anzahl im geteilten Runtime-Pfad; kompatibler Alias. | Positive ganze Zahl. | Begrenzung auf kleinen Geräten. | Nicht gleichzeitig widersprüchlich mit `RETA_JOBS` setzen. |
| `RETA_NUM_THREADS` | Parallel | Weitere Thread-Anzahl-Aliasvariable. | Positive ganze Zahl. | Kompatibilität mit älteren Skripten. | Bevorzuge für neue Skripte `RETA_PARALLEL_WORKERS`. |
| `RETA_PARALLEL_MIN_ITEMS` | Parallel | Mindestanzahl Elemente für parallele Ausführung im Shared-Runtime-Pfad. | Positive ganze Zahl. | Erhöhen, wenn Parallelisierung zu früh anspringt. | Alias: `RETA_PARALLEL_MIN`. |
| `RETA_PARALLEL_MIN` | Parallel | Kurzalias für `RETA_PARALLEL_MIN_ITEMS`. | Positive ganze Zahl. | Nur für alte Skripte. | Neue Dokumentation sollte den längeren Namen verwenden. |
| `RETA_PARALLEL_ALLOW_NESTED` | Parallel | Erlaubt verschachtelte Parallelisierung. | Bool-artig: `1`, `true`, `yes`. | Nur bei bewusst getesteten Pipelines. | Kann schnell Over-Subscription erzeugen. |
| `RETA_GENERATORS` | Parallel-Lane | Steuert die Parallelstrategie für die Lane `RETA_GENERATORS`. | Modusstring oder bool-artiger Wert. | Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus. | Lane-Werte sind spezieller als globale Werte. |
| `RETA_GENERATORS_PARALLEL` | Parallel-Lane | Aktiviert Parallelisierung für die Lane `RETA_GENERATORS`. | Bool-artig. | Für gezielte Performance-Experimente. | Kann globale konservative Einstellungen übersteuern. |
| `RETA_GENERATORS_SERIAL` | Parallel-Lane | Erzwingt serielle Ausführung für die Lane `RETA_GENERATORS`. | Bool-artig. | Bei Paritätsdebugging oder nichtdeterministischer Ausgabe. | Nicht gleichzeitig mit `{base}_PARALLEL` setzen. |
| `RETA_GENERATORS_PARALLEL_MIN_ITEMS` | Parallel-Lane | Mindestanzahl Elemente für Parallelisierung der Lane `RETA_GENERATORS`. | Positive ganze Zahl. | Feintuning der jeweiligen Lane. | Alias: `{base}_PARALLEL_MIN`. |
| `RETA_GENERATORS_PARALLEL_MIN` | Parallel-Lane | Kurzalias für `RETA_GENERATORS_PARALLEL_MIN_ITEMS`. | Positive ganze Zahl. | Kompatibilität. | Für neue Skripte längeren Namen bevorzugen. |
| `RETA_OUTPUT` | Parallel-Lane | Steuert die Parallelstrategie für die Lane `RETA_OUTPUT`. | Modusstring oder bool-artiger Wert. | Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus. | Lane-Werte sind spezieller als globale Werte. |
| `RETA_OUTPUT_PARALLEL` | Parallel-Lane | Aktiviert Parallelisierung für die Lane `RETA_OUTPUT`. | Bool-artig. | Für gezielte Performance-Experimente. | Kann globale konservative Einstellungen übersteuern. |
| `RETA_OUTPUT_SERIAL` | Parallel-Lane | Erzwingt serielle Ausführung für die Lane `RETA_OUTPUT`. | Bool-artig. | Bei Paritätsdebugging oder nichtdeterministischer Ausgabe. | Nicht gleichzeitig mit `{base}_PARALLEL` setzen. |
| `RETA_OUTPUT_PARALLEL_MIN_ITEMS` | Parallel-Lane | Mindestanzahl Elemente für Parallelisierung der Lane `RETA_OUTPUT`. | Positive ganze Zahl. | Feintuning der jeweiligen Lane. | Alias: `{base}_PARALLEL_MIN`. |
| `RETA_OUTPUT_PARALLEL_MIN` | Parallel-Lane | Kurzalias für `RETA_OUTPUT_PARALLEL_MIN_ITEMS`. | Positive ganze Zahl. | Kompatibilität. | Für neue Skripte längeren Namen bevorzugen. |
| `RETA_WIDTH` | Parallel-Lane | Steuert die Parallelstrategie für die Lane `RETA_WIDTH`. | Modusstring oder bool-artiger Wert. | Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus. | Lane-Werte sind spezieller als globale Werte. |
| `RETA_WIDTH_PARALLEL` | Parallel-Lane | Aktiviert Parallelisierung für die Lane `RETA_WIDTH`. | Bool-artig. | Für gezielte Performance-Experimente. | Kann globale konservative Einstellungen übersteuern. |
| `RETA_WIDTH_SERIAL` | Parallel-Lane | Erzwingt serielle Ausführung für die Lane `RETA_WIDTH`. | Bool-artig. | Bei Paritätsdebugging oder nichtdeterministischer Ausgabe. | Nicht gleichzeitig mit `{base}_PARALLEL` setzen. |
| `RETA_WIDTH_PARALLEL_MIN_ITEMS` | Parallel-Lane | Mindestanzahl Elemente für Parallelisierung der Lane `RETA_WIDTH`. | Positive ganze Zahl. | Feintuning der jeweiligen Lane. | Alias: `{base}_PARALLEL_MIN`. |
| `RETA_WIDTH_PARALLEL_MIN` | Parallel-Lane | Kurzalias für `RETA_WIDTH_PARALLEL_MIN_ITEMS`. | Positive ganze Zahl. | Kompatibilität. | Für neue Skripte längeren Namen bevorzugen. |
| `RETA_WIDTHS` | Parallel-Lane | Steuert die Parallelstrategie für die Lane `RETA_WIDTHS`. | Modusstring oder bool-artiger Wert. | Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus. | Lane-Werte sind spezieller als globale Werte. |
| `RETA_WIDTHS_PARALLEL` | Parallel-Lane | Aktiviert Parallelisierung für die Lane `RETA_WIDTHS`. | Bool-artig. | Für gezielte Performance-Experimente. | Kann globale konservative Einstellungen übersteuern. |
| `RETA_WIDTHS_SERIAL` | Parallel-Lane | Erzwingt serielle Ausführung für die Lane `RETA_WIDTHS`. | Bool-artig. | Bei Paritätsdebugging oder nichtdeterministischer Ausgabe. | Nicht gleichzeitig mit `{base}_PARALLEL` setzen. |
| `RETA_WIDTHS_PARALLEL_MIN_ITEMS` | Parallel-Lane | Mindestanzahl Elemente für Parallelisierung der Lane `RETA_WIDTHS`. | Positive ganze Zahl. | Feintuning der jeweiligen Lane. | Alias: `{base}_PARALLEL_MIN`. |
| `RETA_WIDTHS_PARALLEL_MIN` | Parallel-Lane | Kurzalias für `RETA_WIDTHS_PARALLEL_MIN_ITEMS`. | Positive ganze Zahl. | Kompatibilität. | Für neue Skripte längeren Namen bevorzugen. |
| `RETA_PROMPT` | Parallel-Lane | Steuert die Parallelstrategie für die Lane `RETA_PROMPT`. | Modusstring oder bool-artiger Wert. | Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus. | Lane-Werte sind spezieller als globale Werte. |
| `RETA_PROMPT_PARALLEL` | Parallel-Lane | Aktiviert Parallelisierung für die Lane `RETA_PROMPT`. | Bool-artig. | Für gezielte Performance-Experimente. | Kann globale konservative Einstellungen übersteuern. |
| `RETA_PROMPT_SERIAL` | Parallel-Lane | Erzwingt serielle Ausführung für die Lane `RETA_PROMPT`. | Bool-artig. | Bei Paritätsdebugging oder nichtdeterministischer Ausgabe. | Nicht gleichzeitig mit `{base}_PARALLEL` setzen. |
| `RETA_PROMPT_PARALLEL_MIN_ITEMS` | Parallel-Lane | Mindestanzahl Elemente für Parallelisierung der Lane `RETA_PROMPT`. | Positive ganze Zahl. | Feintuning der jeweiligen Lane. | Alias: `{base}_PARALLEL_MIN`. |
| `RETA_PROMPT_PARALLEL_MIN` | Parallel-Lane | Kurzalias für `RETA_PROMPT_PARALLEL_MIN_ITEMS`. | Positive ganze Zahl. | Kompatibilität. | Für neue Skripte längeren Namen bevorzugen. |
| `RETA_PROMPT_BATCH` | Parallel-Lane | Steuert die Parallelstrategie für die Lane `RETA_PROMPT_BATCH`. | Modusstring oder bool-artiger Wert. | Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus. | Lane-Werte sind spezieller als globale Werte. |
| `RETA_PROMPT_BATCH_PARALLEL` | Parallel-Lane | Aktiviert Parallelisierung für die Lane `RETA_PROMPT_BATCH`. | Bool-artig. | Für gezielte Performance-Experimente. | Kann globale konservative Einstellungen übersteuern. |
| `RETA_PROMPT_BATCH_SERIAL` | Parallel-Lane | Erzwingt serielle Ausführung für die Lane `RETA_PROMPT_BATCH`. | Bool-artig. | Bei Paritätsdebugging oder nichtdeterministischer Ausgabe. | Nicht gleichzeitig mit `{base}_PARALLEL` setzen. |
| `RETA_PROMPT_BATCH_PARALLEL_MIN_ITEMS` | Parallel-Lane | Mindestanzahl Elemente für Parallelisierung der Lane `RETA_PROMPT_BATCH`. | Positive ganze Zahl. | Feintuning der jeweiligen Lane. | Alias: `{base}_PARALLEL_MIN`. |
| `RETA_PROMPT_BATCH_PARALLEL_MIN` | Parallel-Lane | Kurzalias für `RETA_PROMPT_BATCH_PARALLEL_MIN_ITEMS`. | Positive ganze Zahl. | Kompatibilität. | Für neue Skripte längeren Namen bevorzugen. |

## Architektur-, Paritäts-, Persistenz- und Recovery-Variablen

| Variable | Bereich | Bedeutung | Werte | Wann setzen? | Vorsicht |
|---|---|---|---|---|---|
| `RETA_PERSISTENCE_DB` | Persistenz | SQLite-/Dateipfad für Architektur-Persistenz. | Dateipfad. | Setzen, wenn Audit-/Persistenzdaten dauerhaft gespeichert werden sollen. | Vorrang vor `RETA_AUDIT_DB`. |
| `RETA_AUDIT_DB` | Persistenz | Kompatibler Audit-Datenbankpfad. | Dateipfad. | Alte Skripte und Python-Referenz. | Neue Skripte sollten `RETA_PERSISTENCE_DB` nutzen. |
| `RETA_ARCHITECTURE_MODE` | Architektur | Hauptmodus für Architektur-/Topologiepfade. | Modusstring. | Wenn Kategorie-/Topologie-/Aktivierungslogik gezielt eingeschaltet wird. | Aliases: `RETA_ARCH_MODE`, `RETA_ARCH`. |
| `RETA_ARCH_MODE` | Architektur | Kurzalias für `RETA_ARCHITECTURE_MODE`. | Modusstring. | Kompatibilität. | Nicht widersprüchlich mit der Hauptvariable setzen. |
| `RETA_ARCH` | Architektur | Kürzester Alias für den Architekturmodus. | Modusstring. | Schnelle Shell-Tests. | Für dauerhafte Skripte ist der lange Name klarer. |
| `RETA_ARCH_TRACE` | Architektur | Aktiviert Trace-Ausgaben der Architektur-Schicht. | Bool-artig oder Trace-Level. | Bei Morphismus-/Topologie-Debugging. | Kann sehr viel Ausgabe erzeugen. |
| `RETA_ARCH_COMPARE_PY` | Parität | Aktiviert Vergleich mit Python-Referenzpfad. | Bool-artig. | Wenn Rust/Python-Kommutativität geprüft werden soll. | Braucht erreichbaren Python-Referenzpfad. |
| `RETA_ARCH_COMPARE_PY_ARCH` | Parität | Vergleicht zusätzlich Python-Architekturpfade. | Bool-artig. | Tiefe Paritätsdiagnose. | Langsamer als normaler Lauf. |
| `RETA_ARCH_ROLLBACK_ANCHOR` | Aktivierung | Anchor/Marker für Rollback oder Recovery-Punkt. | String/ID. | Bei Aktivierungsdatei- oder Recovery-Tests. | Nur mit Dokumentation des konkreten Szenarios setzen. |
| `RETA_ARCH_ALLOW` | Architektur | Whitelist für Architekturfeatures. | Kommagetrennte Liste. | Feature-Slicing für Tests. | Whitelist und Blocklist nicht unklar mischen. |
| `RETA_ARCH_BLOCK` | Architektur | Blocklist für Architekturfeatures. | Kommagetrennte Liste. | Gezieltes Deaktivieren einzelner Pfade. | Kann Parität verfälschen. |
| `RETA_ARCH_ACTIVATION_FILE` | Aktivierung | Pfad zu einer Aktivierungsdatei. | Dateipfad. | Wenn Aktivierungen reproduzierbar geladen werden sollen. | Dateiinhalt muss zum erwarteten Format passen. |
| `RETA_ARCH_ACTIVATION_DIR` | Aktivierung | Verzeichnis für Aktivierungsdateien. | Verzeichnis. | Mehrere Aktivierungsdateien verwalten. | Ein explizites File kann Vorrang haben. |
| `RETA_ARCH_ACTIVATION_RECOVERY_FILE` | Recovery | Pfad zu einer Recovery-Datei. | Dateipfad. | Wiederherstellung nach Aktivierungs-/State-Test. | Nicht mit Produktionsdaten überschreiben. |
| `RETA_ARCH_ACTIVATION_RECOVERY` | Recovery | Aktiviert Recovery-Verhalten der Architektur-Aktivierung. | Bool-artig. | Nur für getestete Recovery-Pfade. | Kann erwartete Fehler verdecken, wenn permanent gesetzt. |

## Skriptinterne Variablen

Diese Namen stehen in `build.sh`, `tools/*.sh` oder `termux_copy.sh`. Sie sind dokumentiert, damit Änderungen an den Skripten nachvollziehbar bleiben. Sie sind normalerweise **keine** Umgebungsvariablen, die ein Anwender exportieren soll.

| Name | Bereich | Erklärung |
|---|---|---|
| `ROOT_DIR` | Script intern | Repository-Wurzel, aus dem Skriptpfad berechnet. |
| `TARGET_DIR` | Script intern | Profilabhängiges Zielverzeichnis, meist `target/debug` oder `target/release`. |
| `CARGO_FLAGS` | Script intern | Array mit Cargo-Flags, etwa `--release`. |
| `CORE_COMPONENT_BASE_PACKAGES` | Script intern | Crates, die vor `reta_render` und Runtime gebaut werden. |
| `CORE_COMPONENT_PACKAGES` | Script intern | Core-Komponenten inklusive Render-Library. |
| `CORE_SPLIT_PACKAGES` | Script intern | Alle Core-Split-Crates inklusive Runtime. |
| `CORE_SPLIT_LIBRARIES` | Script intern | Namen der erwarteten `libreta_*.so`-Artefakte ohne Prefix/Suffix. |
| `PROMPT_SPLIT_PACKAGES` | Script intern | Prompt-Crates: Commands und Input. |
| `PROMPT_SPLIT_LIBRARIES` | Script intern | Namen der erwarteten Prompt-`.so`s ohne Prefix/Suffix. |
| `MANIFEST` | Script intern | Pfad zum generierten Manifest `retaprompt_split_sharedlibs_manifest.json`. |
| `OUT_DIR` | Script intern | Paket-Ausgabeverzeichnis in `tools/package_prompt_split_sharedlibs.sh`; nicht Cargo-`OUT_DIR` verwechseln. |
| `BIN_DIR` | Script intern | Termux-Zielverzeichnis für Executables. |
| `LIB_DIR` | Script intern | Termux-Zielverzeichnis für `.so`-Dateien. |
| `SCRIPT_DIR` | Script intern | Verzeichnis eines Tool-Wrappers. |
| `source` | Script intern | Lokaler Dateipfad in Kopierfunktionen. |
| `dest` | Script intern | Lokaler Zielpfad in Kopierfunktionen. |
| `archive` | Script intern | Lokaler Archivname in Regressionsprüfungen gegen `.a`-Artefakte. |
| `facade_size` | Script intern | Gemessene Größe von `libreta.so` für Größenregressionen. |
| `runtime_size` | Script intern | Gemessene Größe von `libreta_runtime.so` für Größenregressionen. |

## Entscheidungsregeln

1. Setze Build-Variablen nicht dauerhaft in deiner Shell. Die Skripte setzen `RETA_LINK_CORE_SPLIT_LIBS`, `RETA_RENDER_LINK_SEMANTICS` und `RETA_RUNTIME_LINK_CORE_COMPONENTS` bewusst nur für die passenden Cargo-Aufrufe.
2. Nutze `RETA_LIB_PATH` nur für die Reta-Fassade. Prompt-Kommandos erwarten dort `libreta.so`, nicht `libreta_runtime.so`.
3. Nutze `RETA_RENDER_LIB_PATH` nur für den HTML-Renderer-Launcher. `rgrundStrukHtml` soll maximal über `libreta_render.so` gehen.
4. Für reproduzierbare Paketgrößen: `RETA_BUILD_RUST_TOOL_BINS=0` lassen und `RETA_BUILD_RUST_FRONTEND_BINS` unset/`0` halten; `1` ist jetzt eine Build-Fehlermeldung.
5. Für Debugging von Autocomplete/Autosuggest: Nicht die Launcher anfassen. Die Logik liegt in `libretaprompt_input.so`, insbesondere in der Prompt-Completion-Schicht.
6. Für Python/Rust-Parität: Architekturvergleichsvariablen nur pro Testlauf setzen, nicht global in `.profile`.
7. Für Termux: bevorzugt in `$HOME/../usr/bin` und `$HOME/../usr/lib` installieren oder RUNPATH im Paketlayout verwenden.

## Beispiele

### Lokaler Build mit externem Target-Verzeichnis

```bash
CARGO_TARGET_DIR=/tmp/reta-target ./build.sh release
/tmp/reta-target/release/rreta -h
```

### Paketlayout testen, ohne Systeminstallation

```bash
./tools/package_prompt_split_sharedlibs.sh release
cd target/release/retaprompt_split_sharedlibs_package
./rreta -h
./rrp -h
./rgrundStrukHtml blank
```

### Loader-Pfad nur für einen Lauf setzen

```bash
LD_LIBRARY_PATH="$PWD/target/release:${LD_LIBRARY_PATH:-}" target/release/rrp
```

### Prompt-Log aktivieren

```bash
RETA_PROMPT_SESSION_LOG=/tmp/retaPrompt.log target/release/rrp
```

### Paritätsdiagnose begrenzen

```bash
RETA_ARCH_COMPARE_PY=1 RETA_PARALLEL_WORKERS=1 target/release/rreta -h
```

## Einzelkarten aller wichtigen Umgebungsvariablen

### `RETA_LIB_PATH`

**Bereich:** Runtime

**Bedeutung:** Pfad zu `libreta.so`, wenn `retaprompt_commands` die Reta-Fassade nicht neben dem Executable oder im Loader-Suchpfad findet.

**Gültige Werte:** Absoluter oder relativer Dateipfad.

**Typischer Einsatz:** Beim Pakettest, bei Termux-Kopien und bei Entwicklung außerhalb von RPATH/RUNPATH setzen.

**Risiko:** Muss auf die Fassade zeigen, nicht auf `libreta_runtime.so`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_RENDER_LIB_PATH`

**Bereich:** Runtime

**Bedeutung:** Pfad zu `libreta_render.so` für den dynamischen `rgrundStrukHtml`-Launcher.

**Gültige Werte:** Absoluter oder relativer Dateipfad.

**Typischer Einsatz:** Nützlich, wenn `rgrundStrukHtml` aus einem anderen Verzeichnis gestartet wird.

**Risiko:** Nicht mit `RETA_LIB_PATH` verwechseln; HTML-Rendering geht über `libreta_render.so`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_CSV_PATH`

**Bereich:** Runtime

**Bedeutung:** Pfad zum CSV-Datenverzeichnis der Reta-Datenbasis.

**Gültige Werte:** Verzeichnis.

**Typischer Einsatz:** Setzen, wenn die CSV-Dateien nicht im erwarteten Paketlayout liegen.

**Risiko:** Bei falschem Pfad wirken Daten-/Aliasfunktionen unvollständig.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_BIN`

**Bereich:** Prompt

**Bedeutung:** Optionaler Pfad zu einer externen Reta-Ausführung, falls Prompt-Kommandos bewusst nicht über `libreta.so` laufen sollen.

**Gültige Werte:** Dateipfad.

**Typischer Einsatz:** Nur für Diagnose oder alte Setups.

**Risiko:** Im neuen `.so`-Paradigma normalerweise nicht setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_SESSION_LOG`

**Bereich:** Prompt

**Bedeutung:** Pfad für das Sitzungslog von retaPrompt.

**Gültige Werte:** Dateipfad.

**Typischer Einsatz:** Setzen, wenn Ein-/Ausgabe der Prompt-Sitzung nachvollziehbar gespeichert werden soll.

**Risiko:** Keine geheimen Eingaben loggen, wenn die Datei geteilt wird.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `COLUMNS`

**Bereich:** Terminal

**Bedeutung:** Breite des Terminals für Wrap-, Tabellen- und Prompt-Anzeige.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Automatische Erkennung überschreiben, etwa in CI oder Pipe-Umgebungen.

**Risiko:** Zu kleine Werte verschlechtern Tabellenlayout.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `LINES`

**Bereich:** Terminal

**Bedeutung:** Höhe des Terminals für Konsolen-/TUI-Funktionen.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Selten nötig; hilfreich für reproduzierbare Tests.

**Risiko:** Nicht alle Pfade benutzen `LINES`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `HOME`

**Bereich:** System

**Bedeutung:** Basis für Termux-Zielpfade und einige historische Skripte.

**Gültige Werte:** Verzeichnis.

**Typischer Einsatz:** Wird normalerweise vom System gesetzt.

**Risiko:** Nicht im Build überschreiben, außer du weißt warum.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `LD_LIBRARY_PATH`

**Bereich:** Loader

**Bedeutung:** Zusätzliche Suchpfade des dynamischen Linux/Android-Linkers.

**Gültige Werte:** Doppelpunktgetrennte Verzeichnisliste.

**Typischer Einsatz:** Nur verwenden, wenn RPATH/RUNPATH oder Paketlayout nicht greifen.

**Risiko:** Kann falsche `.so`-Versionen vorziehen; für Produktion besser RUNPATH verwenden.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `DYLD_LIBRARY_PATH`

**Bereich:** Loader

**Bedeutung:** macOS-Analog zu `LD_LIBRARY_PATH` für Entwicklungsports.

**Gültige Werte:** Doppelpunktgetrennte Verzeichnisliste.

**Typischer Einsatz:** Nur relevant auf macOS-Experimenten.

**Risiko:** Dieses Projekt ist primär Linux/Termux-orientiert.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `CARGO_TARGET_DIR`

**Bereich:** Cargo/Build

**Bedeutung:** Alternatives Cargo-Target-Verzeichnis.

**Gültige Werte:** Verzeichnis.

**Typischer Einsatz:** Setzen, wenn Artefakte außerhalb von `target/` landen sollen.

**Risiko:** Build- und Paket-Skripte leiten daraus `target/debug` oder `target/release` ab.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_LINK_CORE_SPLIT_LIBS`

**Bereich:** Build

**Bedeutung:** Aktiviert in `build.rs` die Link-Kante von `libreta.so` zu den privaten Core-`.so`s.

**Gültige Werte:** `1` oder unset.

**Typischer Einsatz:** Wird durch die Build-Skripte gesetzt.

**Risiko:** Nicht manuell auf `0` setzen; sonst wird `libreta.so` wieder zu schwer.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_RENDER_LINK_SEMANTICS`

**Bereich:** Build

**Bedeutung:** Aktiviert die Link-Kante `libreta_render.so -> libreta_semantics.so`.

**Gültige Werte:** `1` oder unset.

**Typischer Einsatz:** Wird durch die Build-Skripte gesetzt.

**Risiko:** Ohne diese Variable verliert `rgrundStrukHtml` die gewünschte Render/Semantik-Topologie.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_RUNTIME_LINK_CORE_COMPONENTS`

**Bereich:** Build

**Bedeutung:** Aktiviert die Link-Kanten von `libreta_runtime.so` zu `data`, `parse`, `semantics`, `table`, `render` und `arch`.

**Gültige Werte:** `1` oder unset.

**Typischer Einsatz:** Wird durch die Build-Skripte gesetzt.

**Risiko:** Ohne diese Variable entstehen wieder isolierte Stubs.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_BUILD_RUST_TOOL_BINS`

**Bereich:** Build

**Bedeutung:** Baut zusätzlich schwere Rust-Diagnose- und Tool-Binaries.

**Gültige Werte:** `1` oder unset/`0`.

**Typischer Einsatz:** Nur bei Entwicklerdiagnose setzen.

**Risiko:** Nicht für Paketgrößenmessung verwenden; finale öffentliche Binaries sind C-Launcher.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_BUILD_RUST_FRONTEND_BINS`

**Bereich:** Build

**Bedeutung:** Retired/gesperrt. Diese Variable stand früher für zusätzliche Rust-Prompt-Frontend-Binaries. Genau dieser Pfad hat `rrp`, `rrpl`, `rrpe` und `rrpb` wieder unnötig groß gemacht.

**Gültige Werte:** unset oder `0`.

**Typischer Einsatz:** Nicht mehr setzen. Der korrekte Pfad ist `./build.sh release` oder `./tools/package_prompt_split_sharedlibs.sh release`; beide erzeugen kleine C-Launcher.

**Risiko:** `1` bricht den Build absichtlich ab. Das ist kein Fehler, sondern ein Schutz gegen Größenregression.

**Programmiererhinweis:** Prompt-Logik gehört in `libretaprompt_input.so` und `libretaprompt_commands.so`. Die Executables dürfen nur ABI-Launcher sein. Die Guard-Skripte `tools/guard_prompt_frontend_sources.py` und `tools/guard_prompt_launcher_topology.sh` prüfen diese Regel.

### `RETA_PROMPT_LAUNCHER_MAX_BYTES`

**Bereich:** Build/Guard

**Bedeutung:** Obergrenze für die Dateigröße der finalen Prompt-Launcher `rrp`, `rrpl`, `rrpe` und `rrpb`. Der Standardwert ist `262144` Bytes.

**Gültige Werte:** Positive ganze Bytezahl.

**Typischer Einsatz:** Normalerweise nicht setzen. Nur bei einer Plattform anheben, auf der ein echter C-Launcher durch Toolchain-/Loader-Metadaten größer ist.

**Risiko:** Diese Variable darf nicht benutzt werden, um eine Rust-Payload in den Launchern zu akzeptieren. Wenn der Guard anschlägt, zuerst prüfen, ob `rrp/rrpl/rrpe/rrpb` wieder aus Rust-Binaries statt aus `tools/launchers/*.c` stammen.

**Programmiererhinweis:** Der Guard `tools/guard_prompt_launcher_topology.sh` prüft Größe, `DT_NEEDED`-Kanten, verbotene `libreta*.so`-Kanten und Rust-Payload-Symbole.

### `PROFILE`

**Bereich:** Cargo/Script

**Bedeutung:** Cargo-Profil; in Skripten aus dem ersten Argument `debug` oder `release` abgeleitet.

**Gültige Werte:** `debug` oder `release`.

**Typischer Einsatz:** Nicht direkt setzen; `./build.sh debug` oder `./build.sh release` verwenden.

**Risiko:** Cargo selbst setzt `PROFILE` in Build-Skripten.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `OUT_DIR`

**Bereich:** Cargo

**Bedeutung:** Von Cargo gesetztes Ausgabeverzeichnis für Build-Skripte.

**Gültige Werte:** Verzeichnis.

**Typischer Einsatz:** Nicht manuell setzen.

**Risiko:** Wird in Rust-`build.rs` für generierte Linker-Shims verwendet.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `CARGO_MANIFEST_DIR`

**Bereich:** Cargo

**Bedeutung:** Von Cargo gesetzter Pfad zum aktuellen Crate-Manifest.

**Gültige Werte:** Verzeichnis.

**Typischer Einsatz:** Nicht manuell setzen.

**Risiko:** Hilfreich für Build-Script-Pfadberechnung.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RUSTFLAGS`

**Bereich:** Cargo

**Bedeutung:** Zusätzliche Flags für rustc.

**Gültige Werte:** String.

**Typischer Einsatz:** Nur bewusst setzen, etwa für Linker-/Symboltests.

**Risiko:** Kann die Größe und Link-Topologie stark verändern.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PARALLEL`

**Bereich:** Parallel

**Bedeutung:** Kompatibler Hauptschalter für Parallelisierung; Alias/Quelle für den Architektur-Parallelmodus.

**Gültige Werte:** `auto`, `off`, `threads`, `processes` oder ähnliche Moduswerte.

**Typischer Einsatz:** Schneller Test eines globalen Parallelmodus.

**Risiko:** Bei Konflikt hat der spezifischere Modus Vorrang.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PARALLEL_MODE`

**Bereich:** Parallel

**Bedeutung:** Expliziter Parallelmodus der Architektur-Schicht.

**Gültige Werte:** Modusstring.

**Typischer Einsatz:** Nutzen, wenn der Architekturpfad eindeutig gesteuert werden soll.

**Risiko:** Dokumentiert und prüft besser als der Alias `RETA_PARALLEL`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PARALLEL_WORKERS`

**Bereich:** Parallel

**Bedeutung:** Anzahl Worker für parallele Ausführung.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Bei CI, Termux oder schwacher Hardware begrenzen.

**Risiko:** Zu hoch kann Speicher, Scheduling und Ausgabe-Reihenfolge belasten.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PARALLEL_CHUNK_SIZE`

**Bereich:** Parallel

**Bedeutung:** Chunk-Größe für Aufgabenbündel.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Tuning zwischen Scheduler-Overhead und Latenz.

**Risiko:** Kleine Chunks erhöhen Overhead, große Chunks verschlechtern Balancing.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PARALLEL_THRESHOLD`

**Bereich:** Parallel

**Bedeutung:** Mindestgröße, ab der parallelisiert wird.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Erhöhen, wenn kleine Aufgaben im Parallelmodus langsamer sind.

**Risiko:** Zu niedrig macht einfache Tabellen unnötig teuer.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PARALLEL_START_METHOD`

**Bereich:** Parallel

**Bedeutung:** Startmethode für Prozess-/Worker-Modell im Architekturkontext.

**Gültige Werte:** String.

**Typischer Einsatz:** Primär für Paritäts-/Python-Referenzpfade interessant.

**Risiko:** Rust-Pfade verwenden nicht jeden Python-Startmethodenwert.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_JOBS`

**Bereich:** Parallel

**Bedeutung:** Worker-Anzahl im geteilten Runtime-Pfad; kompatibler Alias.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Kurzer Alias für Batch-/Generatorläufe.

**Risiko:** Wird mit `RETA_THREADS`/`RETA_NUM_THREADS` zusammengeführt.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_THREADS`

**Bereich:** Parallel

**Bedeutung:** Thread-Anzahl im geteilten Runtime-Pfad; kompatibler Alias.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Begrenzung auf kleinen Geräten.

**Risiko:** Nicht gleichzeitig widersprüchlich mit `RETA_JOBS` setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_NUM_THREADS`

**Bereich:** Parallel

**Bedeutung:** Weitere Thread-Anzahl-Aliasvariable.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Kompatibilität mit älteren Skripten.

**Risiko:** Bevorzuge für neue Skripte `RETA_PARALLEL_WORKERS`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PARALLEL_MIN_ITEMS`

**Bereich:** Parallel

**Bedeutung:** Mindestanzahl Elemente für parallele Ausführung im Shared-Runtime-Pfad.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Erhöhen, wenn Parallelisierung zu früh anspringt.

**Risiko:** Alias: `RETA_PARALLEL_MIN`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PARALLEL_MIN`

**Bereich:** Parallel

**Bedeutung:** Kurzalias für `RETA_PARALLEL_MIN_ITEMS`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Nur für alte Skripte.

**Risiko:** Neue Dokumentation sollte den längeren Namen verwenden.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PARALLEL_ALLOW_NESTED`

**Bereich:** Parallel

**Bedeutung:** Erlaubt verschachtelte Parallelisierung.

**Gültige Werte:** Bool-artig: `1`, `true`, `yes`.

**Typischer Einsatz:** Nur bei bewusst getesteten Pipelines.

**Risiko:** Kann schnell Over-Subscription erzeugen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_GENERATORS`

**Bereich:** Parallel-Lane

**Bedeutung:** Steuert die Parallelstrategie für die Lane `RETA_GENERATORS`.

**Gültige Werte:** Modusstring oder bool-artiger Wert.

**Typischer Einsatz:** Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus.

**Risiko:** Lane-Werte sind spezieller als globale Werte.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_GENERATORS_PARALLEL`

**Bereich:** Parallel-Lane

**Bedeutung:** Aktiviert Parallelisierung für die Lane `RETA_GENERATORS`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Für gezielte Performance-Experimente.

**Risiko:** Kann globale konservative Einstellungen übersteuern.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_GENERATORS_SERIAL`

**Bereich:** Parallel-Lane

**Bedeutung:** Erzwingt serielle Ausführung für die Lane `RETA_GENERATORS`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Bei Paritätsdebugging oder nichtdeterministischer Ausgabe.

**Risiko:** Nicht gleichzeitig mit `{base}_PARALLEL` setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_GENERATORS_PARALLEL_MIN_ITEMS`

**Bereich:** Parallel-Lane

**Bedeutung:** Mindestanzahl Elemente für Parallelisierung der Lane `RETA_GENERATORS`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Feintuning der jeweiligen Lane.

**Risiko:** Alias: `{base}_PARALLEL_MIN`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_GENERATORS_PARALLEL_MIN`

**Bereich:** Parallel-Lane

**Bedeutung:** Kurzalias für `RETA_GENERATORS_PARALLEL_MIN_ITEMS`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Kompatibilität.

**Risiko:** Für neue Skripte längeren Namen bevorzugen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_OUTPUT`

**Bereich:** Parallel-Lane

**Bedeutung:** Steuert die Parallelstrategie für die Lane `RETA_OUTPUT`.

**Gültige Werte:** Modusstring oder bool-artiger Wert.

**Typischer Einsatz:** Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus.

**Risiko:** Lane-Werte sind spezieller als globale Werte.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_OUTPUT_PARALLEL`

**Bereich:** Parallel-Lane

**Bedeutung:** Aktiviert Parallelisierung für die Lane `RETA_OUTPUT`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Für gezielte Performance-Experimente.

**Risiko:** Kann globale konservative Einstellungen übersteuern.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_OUTPUT_SERIAL`

**Bereich:** Parallel-Lane

**Bedeutung:** Erzwingt serielle Ausführung für die Lane `RETA_OUTPUT`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Bei Paritätsdebugging oder nichtdeterministischer Ausgabe.

**Risiko:** Nicht gleichzeitig mit `{base}_PARALLEL` setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_OUTPUT_PARALLEL_MIN_ITEMS`

**Bereich:** Parallel-Lane

**Bedeutung:** Mindestanzahl Elemente für Parallelisierung der Lane `RETA_OUTPUT`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Feintuning der jeweiligen Lane.

**Risiko:** Alias: `{base}_PARALLEL_MIN`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_OUTPUT_PARALLEL_MIN`

**Bereich:** Parallel-Lane

**Bedeutung:** Kurzalias für `RETA_OUTPUT_PARALLEL_MIN_ITEMS`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Kompatibilität.

**Risiko:** Für neue Skripte längeren Namen bevorzugen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTH`

**Bereich:** Parallel-Lane

**Bedeutung:** Steuert die Parallelstrategie für die Lane `RETA_WIDTH`.

**Gültige Werte:** Modusstring oder bool-artiger Wert.

**Typischer Einsatz:** Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus.

**Risiko:** Lane-Werte sind spezieller als globale Werte.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTH_PARALLEL`

**Bereich:** Parallel-Lane

**Bedeutung:** Aktiviert Parallelisierung für die Lane `RETA_WIDTH`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Für gezielte Performance-Experimente.

**Risiko:** Kann globale konservative Einstellungen übersteuern.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTH_SERIAL`

**Bereich:** Parallel-Lane

**Bedeutung:** Erzwingt serielle Ausführung für die Lane `RETA_WIDTH`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Bei Paritätsdebugging oder nichtdeterministischer Ausgabe.

**Risiko:** Nicht gleichzeitig mit `{base}_PARALLEL` setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTH_PARALLEL_MIN_ITEMS`

**Bereich:** Parallel-Lane

**Bedeutung:** Mindestanzahl Elemente für Parallelisierung der Lane `RETA_WIDTH`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Feintuning der jeweiligen Lane.

**Risiko:** Alias: `{base}_PARALLEL_MIN`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTH_PARALLEL_MIN`

**Bereich:** Parallel-Lane

**Bedeutung:** Kurzalias für `RETA_WIDTH_PARALLEL_MIN_ITEMS`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Kompatibilität.

**Risiko:** Für neue Skripte längeren Namen bevorzugen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTHS`

**Bereich:** Parallel-Lane

**Bedeutung:** Steuert die Parallelstrategie für die Lane `RETA_WIDTHS`.

**Gültige Werte:** Modusstring oder bool-artiger Wert.

**Typischer Einsatz:** Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus.

**Risiko:** Lane-Werte sind spezieller als globale Werte.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTHS_PARALLEL`

**Bereich:** Parallel-Lane

**Bedeutung:** Aktiviert Parallelisierung für die Lane `RETA_WIDTHS`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Für gezielte Performance-Experimente.

**Risiko:** Kann globale konservative Einstellungen übersteuern.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTHS_SERIAL`

**Bereich:** Parallel-Lane

**Bedeutung:** Erzwingt serielle Ausführung für die Lane `RETA_WIDTHS`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Bei Paritätsdebugging oder nichtdeterministischer Ausgabe.

**Risiko:** Nicht gleichzeitig mit `{base}_PARALLEL` setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTHS_PARALLEL_MIN_ITEMS`

**Bereich:** Parallel-Lane

**Bedeutung:** Mindestanzahl Elemente für Parallelisierung der Lane `RETA_WIDTHS`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Feintuning der jeweiligen Lane.

**Risiko:** Alias: `{base}_PARALLEL_MIN`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_WIDTHS_PARALLEL_MIN`

**Bereich:** Parallel-Lane

**Bedeutung:** Kurzalias für `RETA_WIDTHS_PARALLEL_MIN_ITEMS`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Kompatibilität.

**Risiko:** Für neue Skripte längeren Namen bevorzugen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT`

**Bereich:** Parallel-Lane

**Bedeutung:** Steuert die Parallelstrategie für die Lane `RETA_PROMPT`.

**Gültige Werte:** Modusstring oder bool-artiger Wert.

**Typischer Einsatz:** Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus.

**Risiko:** Lane-Werte sind spezieller als globale Werte.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_PARALLEL`

**Bereich:** Parallel-Lane

**Bedeutung:** Aktiviert Parallelisierung für die Lane `RETA_PROMPT`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Für gezielte Performance-Experimente.

**Risiko:** Kann globale konservative Einstellungen übersteuern.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_SERIAL`

**Bereich:** Parallel-Lane

**Bedeutung:** Erzwingt serielle Ausführung für die Lane `RETA_PROMPT`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Bei Paritätsdebugging oder nichtdeterministischer Ausgabe.

**Risiko:** Nicht gleichzeitig mit `{base}_PARALLEL` setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_PARALLEL_MIN_ITEMS`

**Bereich:** Parallel-Lane

**Bedeutung:** Mindestanzahl Elemente für Parallelisierung der Lane `RETA_PROMPT`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Feintuning der jeweiligen Lane.

**Risiko:** Alias: `{base}_PARALLEL_MIN`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_PARALLEL_MIN`

**Bereich:** Parallel-Lane

**Bedeutung:** Kurzalias für `RETA_PROMPT_PARALLEL_MIN_ITEMS`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Kompatibilität.

**Risiko:** Für neue Skripte längeren Namen bevorzugen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_BATCH`

**Bereich:** Parallel-Lane

**Bedeutung:** Steuert die Parallelstrategie für die Lane `RETA_PROMPT_BATCH`.

**Gültige Werte:** Modusstring oder bool-artiger Wert.

**Typischer Einsatz:** Nur setzen, wenn diese einzelne Lane anders laufen soll als der globale Modus.

**Risiko:** Lane-Werte sind spezieller als globale Werte.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_BATCH_PARALLEL`

**Bereich:** Parallel-Lane

**Bedeutung:** Aktiviert Parallelisierung für die Lane `RETA_PROMPT_BATCH`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Für gezielte Performance-Experimente.

**Risiko:** Kann globale konservative Einstellungen übersteuern.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_BATCH_SERIAL`

**Bereich:** Parallel-Lane

**Bedeutung:** Erzwingt serielle Ausführung für die Lane `RETA_PROMPT_BATCH`.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Bei Paritätsdebugging oder nichtdeterministischer Ausgabe.

**Risiko:** Nicht gleichzeitig mit `{base}_PARALLEL` setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_BATCH_PARALLEL_MIN_ITEMS`

**Bereich:** Parallel-Lane

**Bedeutung:** Mindestanzahl Elemente für Parallelisierung der Lane `RETA_PROMPT_BATCH`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Feintuning der jeweiligen Lane.

**Risiko:** Alias: `{base}_PARALLEL_MIN`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PROMPT_BATCH_PARALLEL_MIN`

**Bereich:** Parallel-Lane

**Bedeutung:** Kurzalias für `RETA_PROMPT_BATCH_PARALLEL_MIN_ITEMS`.

**Gültige Werte:** Positive ganze Zahl.

**Typischer Einsatz:** Kompatibilität.

**Risiko:** Für neue Skripte längeren Namen bevorzugen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_PERSISTENCE_DB`

**Bereich:** Persistenz

**Bedeutung:** SQLite-/Dateipfad für Architektur-Persistenz.

**Gültige Werte:** Dateipfad.

**Typischer Einsatz:** Setzen, wenn Audit-/Persistenzdaten dauerhaft gespeichert werden sollen.

**Risiko:** Vorrang vor `RETA_AUDIT_DB`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_AUDIT_DB`

**Bereich:** Persistenz

**Bedeutung:** Kompatibler Audit-Datenbankpfad.

**Gültige Werte:** Dateipfad.

**Typischer Einsatz:** Alte Skripte und Python-Referenz.

**Risiko:** Neue Skripte sollten `RETA_PERSISTENCE_DB` nutzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCHITECTURE_MODE`

**Bereich:** Architektur

**Bedeutung:** Hauptmodus für Architektur-/Topologiepfade.

**Gültige Werte:** Modusstring.

**Typischer Einsatz:** Wenn Kategorie-/Topologie-/Aktivierungslogik gezielt eingeschaltet wird.

**Risiko:** Aliases: `RETA_ARCH_MODE`, `RETA_ARCH`.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_MODE`

**Bereich:** Architektur

**Bedeutung:** Kurzalias für `RETA_ARCHITECTURE_MODE`.

**Gültige Werte:** Modusstring.

**Typischer Einsatz:** Kompatibilität.

**Risiko:** Nicht widersprüchlich mit der Hauptvariable setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH`

**Bereich:** Architektur

**Bedeutung:** Kürzester Alias für den Architekturmodus.

**Gültige Werte:** Modusstring.

**Typischer Einsatz:** Schnelle Shell-Tests.

**Risiko:** Für dauerhafte Skripte ist der lange Name klarer.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_TRACE`

**Bereich:** Architektur

**Bedeutung:** Aktiviert Trace-Ausgaben der Architektur-Schicht.

**Gültige Werte:** Bool-artig oder Trace-Level.

**Typischer Einsatz:** Bei Morphismus-/Topologie-Debugging.

**Risiko:** Kann sehr viel Ausgabe erzeugen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_COMPARE_PY`

**Bereich:** Parität

**Bedeutung:** Aktiviert Vergleich mit Python-Referenzpfad.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Wenn Rust/Python-Kommutativität geprüft werden soll.

**Risiko:** Braucht erreichbaren Python-Referenzpfad.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_COMPARE_PY_ARCH`

**Bereich:** Parität

**Bedeutung:** Vergleicht zusätzlich Python-Architekturpfade.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Tiefe Paritätsdiagnose.

**Risiko:** Langsamer als normaler Lauf.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_ROLLBACK_ANCHOR`

**Bereich:** Aktivierung

**Bedeutung:** Anchor/Marker für Rollback oder Recovery-Punkt.

**Gültige Werte:** String/ID.

**Typischer Einsatz:** Bei Aktivierungsdatei- oder Recovery-Tests.

**Risiko:** Nur mit Dokumentation des konkreten Szenarios setzen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_ALLOW`

**Bereich:** Architektur

**Bedeutung:** Whitelist für Architekturfeatures.

**Gültige Werte:** Kommagetrennte Liste.

**Typischer Einsatz:** Feature-Slicing für Tests.

**Risiko:** Whitelist und Blocklist nicht unklar mischen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_BLOCK`

**Bereich:** Architektur

**Bedeutung:** Blocklist für Architekturfeatures.

**Gültige Werte:** Kommagetrennte Liste.

**Typischer Einsatz:** Gezieltes Deaktivieren einzelner Pfade.

**Risiko:** Kann Parität verfälschen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_ACTIVATION_FILE`

**Bereich:** Aktivierung

**Bedeutung:** Pfad zu einer Aktivierungsdatei.

**Gültige Werte:** Dateipfad.

**Typischer Einsatz:** Wenn Aktivierungen reproduzierbar geladen werden sollen.

**Risiko:** Dateiinhalt muss zum erwarteten Format passen.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_ACTIVATION_DIR`

**Bereich:** Aktivierung

**Bedeutung:** Verzeichnis für Aktivierungsdateien.

**Gültige Werte:** Verzeichnis.

**Typischer Einsatz:** Mehrere Aktivierungsdateien verwalten.

**Risiko:** Ein explizites File kann Vorrang haben.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_ACTIVATION_RECOVERY_FILE`

**Bereich:** Recovery

**Bedeutung:** Pfad zu einer Recovery-Datei.

**Gültige Werte:** Dateipfad.

**Typischer Einsatz:** Wiederherstellung nach Aktivierungs-/State-Test.

**Risiko:** Nicht mit Produktionsdaten überschreiben.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.

### `RETA_ARCH_ACTIVATION_RECOVERY`

**Bereich:** Recovery

**Bedeutung:** Aktiviert Recovery-Verhalten der Architektur-Aktivierung.

**Gültige Werte:** Bool-artig.

**Typischer Einsatz:** Nur für getestete Recovery-Pfade.

**Risiko:** Kann erwartete Fehler verdecken, wenn permanent gesetzt.

**Programmiererhinweis:** Prüfe diese Variable möglichst nah an der Grenze, an der sie gebraucht wird. Baue keine versteckten globalen Annahmen in Executables ein. Für die aktuelle Shared-Library-Architektur gilt: Runtime-Entscheidungen gehören in Libraries, Launcher sollen nur Pfade, argv und Exit-Codes weiterreichen.


## Wartungscheckliste für Shellskripte

- Neue Variable eingeführt? Dann hier und in der englischen Datei dokumentieren.
- Variable steuert Build-Linking? Dann `cargo:rerun-if-env-changed=...` im zuständigen `build.rs` prüfen.
- Variable steuert Library-Suche? Dann darf sie nicht dazu führen, dass ein Launcher wieder schweren Rust-Code enthält.
- Variable steuert Prompt-Verhalten? Dann muss die eigentliche Logik in `libretaprompt_input.so` oder `libretaprompt_commands.so` liegen.
- Variable steuert Reta-Core? Dann prüfen, ob sie in `libreta.so` nur weitergereicht oder in `libreta_runtime.so` ausgewertet werden sollte.
- Variable ist nur scriptintern? Dann nicht als Nutzerkonfiguration bewerben.
- Variable beeinflusst Paketgrößen? Dann Größenprüfungen in `build.sh` und `tools/build_prompt_split_sharedlibs.sh` anpassen.

## Fehlerbilder

| Symptom | Wahrscheinliche Ursache | Prüfung | Lösung |
|---|---|---|---|
| `rrp` findet `libreta.so` nicht | Loader-Pfad oder `RETA_LIB_PATH` fehlt | `ldd target/release/rrp` | Paketlayout nutzen oder `RETA_LIB_PATH` setzen |
| `rgrundStrukHtml` ist wieder groß | Rust-Binary statt C-Launcher gebaut/kopiert | `readelf -d rgrundStrukHtml` | Buildskripte und Kopierschritt prüfen |
| `libreta.so` größer als `libreta_runtime.so` | Fassade trägt wieder Core-Code | `stat -c %s libreta.so libreta_runtime.so` | `RETA_LINK_CORE_SPLIT_LIBS=1` und Runtime-Fassade prüfen |
| Autosuggest nur am Zeilenende | Hinter rendert nur Suffix, nicht Cursor-Position | `retaprompt_input_autosuggestion_at_cursor_json` prüfen | `libretaprompt_input.so` neu bauen |
| Alle Core-Komponenten gleich groß | Stub-Regression | Größenvergleich `libreta_data/parse/...` | Komponentenfunktionen und Link-Guards prüfen |

## Beziehung zur Shared-Library-Dokumentation

Die `.so`-Dokumentation beschreibt ABI, Ownership und Topologie pro Library. Diese Datei beschreibt die Variablen, die diese Topologie bauen, laden oder diagnostizieren. Beide Dokumentationen müssen zusammen gepflegt werden.
