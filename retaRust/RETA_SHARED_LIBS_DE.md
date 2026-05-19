# Reta `.so`-Split — deutsche große Übersicht

Diese Datei ist die zentrale deutsche Übersicht zur dynamischen `.so`-Topologie. Die ausführlichen Einzeldokumente stehen unter `doc/shared-libs/de/`.

## Aktive Topologie

```text
rreta -> libreta.so -> libreta_data.so + libreta_parse.so + libreta_semantics.so + libreta_table.so + libreta_render.so + libreta_arch.so + libreta_runtime.so
rgrundStrukHtml -> libreta_render.so -> libreta_semantics.so
rrp/rrpl/rrpe -> libretaprompt_input.so + libretaprompt_commands.so
rrpb -> libretaprompt_commands.so
```

## Kernregeln

- `libreta.so` bleibt klein und öffentlich.
- `libreta_runtime.so` trägt den schweren nicht-interaktiven Reta-Kern.
- `libreta_render.so` trägt die HTML-Erzeugung für `rgrundStrukHtml`.
- `libretaprompt_input.so` trägt interaktive Eingabe, Autocomplete, Autosuggest und History.
- `libretaprompt_commands.so` trägt die Command-Seite und ist die einzige Prompt-Library für `rrpb`.
- Die finalen Executables werden im normalen Build als C-Launcher gebaut.
- Static Archives sind im aktiven Pfad absichtlich deaktiviert.

## Alle `.so`-Dokumente

- [libreta.so](libreta.md) — öffentliche, stabile Reta-C-ABI-Fassade
- [libreta_data.so](libreta_data.md) — Daten, Wörter, Aliase, CSV-/Katalog-Projektionen
- [libreta_parse.so](libreta_parse.md) — Parsing, Tokenisierung und Input-Morphismen
- [libreta_semantics.so](libreta_semantics.md) — Semantik, Auswahlräume, Topologie und Prägarbe
- [libreta_table.so](libreta_table.md) — Tabellen, View-Zustand, Breitenlogik und Garben-Verklebung
- [libreta_render.so](libreta_render.md) — Rendering-Funktoren, insbesondere GrundStrukHtml
- [libreta_arch.so](libreta_arch.md) — Architektur-Metadaten, Kategorie, Morphismus und Topologie
- [libreta_runtime.so](libreta_runtime.md) — Ausführungsnetzwerk und schwerer Reta-Core-Träger
- [libretaprompt_commands.so](libretaprompt_commands.md) — retaPrompt-Befehlsseite und Command-Morphismen
- [libretaprompt_input.so](libretaprompt_input.md) — retaPrompt-Eingabe, Autocomplete, Autosuggest und History

## Shellvariablen

Die sehr große Dokumentation der Build-, Runtime- und internen Shellvariablen liegt in `RETA_SHELL_VARIABLES_DE.md` und `doc/shell-variables/de/README.md`.

## Ergänzende Shellvariablen-Dokumentation

Die Shared-Library-Topologie wird nicht nur durch Rust-Code und C-Launcher bestimmt, sondern auch durch Build-, Linker-, Loader- und Runtime-Variablen. Die ausführliche deutsche Dokumentation liegt in:

```text
RETA_SHELL_VARIABLES_DE.md
doc/shell-variables/de/README.md
```

Die englische Fassung liegt in:

```text
RETA_SHELL_VARIABLES_EN.md
doc/shell-variables/en/README.md
```

Für Programmierer ist besonders wichtig: Variablen wie `RETA_LINK_CORE_SPLIT_LIBS`, `RETA_RENDER_LINK_SEMANTICS` und `RETA_RUNTIME_LINK_CORE_COMPONENTS` sind Build-Topologie-Schalter. Variablen wie `RETA_LIB_PATH`, `RETA_RENDER_LIB_PATH`, `RETA_CSV_PATH` und `LD_LIBRARY_PATH` sind Laufzeit-/Loader-Hilfen. Keine dieser Variablen rechtfertigt, Algorithmik wieder in die Executables zu verschieben.

Die Größenpolicy für die Prompt-Launcher liegt zusätzlich in `PROMPT_LAUNCHER_SIZE_POLICY_DE.md`; die englische Fassung in `PROMPT_LAUNCHER_SIZE_POLICY_EN.md`.
