# Reta Shared-Library-Topologie — deutsche Programmierer-Dokumentation

Diese Dokumentation beschreibt **alle aktuell gebauten `.so`-Bibliotheken** der Reta-/retaPrompt-Architektur. Sie ist auf Programmierer zugeschnitten: ABI, Header, Ownership, Build-Prüfungen, `DT_NEEDED`, RPATH/RUNPATH, typische Regressionen und Erweiterungsregeln.

## Zieltopologie

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

## Grundsatz

Die finalen Executables bleiben klein. Programmlogik liegt in `.so`-Bibliotheken. `libreta.so` ist die dünne öffentliche Fassade; `libreta_runtime.so` trägt den schweren nicht-interaktiven Kern. `rgrundStrukHtml` nutzt `libreta_render.so` direkt. `rrpb` bleibt command-only. `rrp`, `rrpl` und `rrpe` nutzen sowohl die Input- als auch die Command-Library.

## Autocomplete/Autosuggest-Grenze

Autocomplete und Autosuggest gehören zu `libretaprompt_input.so`. Die C-Launcher enthalten dafür keinen Algorithmus. Cursor-Mitte-Autosuggest wird in der Shared-Library berechnet und über `retaprompt_input_autosuggestion_at_cursor_json` zusätzlich als ABI-Diagnose exportiert.

## Einzeldokumente

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

## Ergänzende Dokumentation

- `RETA_SHARED_LIBS_DE.md` — Root-Übersicht.
- `RETA_SHARED_LIBS_EN.md` — englische Root-Übersicht.
- `RETA_SHELL_VARIABLES_DE.md` — große deutsche Dokumentation der Shell-/Umgebungsvariablen.
- `RETA_SHELL_VARIABLES_EN.md` — große englische Dokumentation der Shell-/Umgebungsvariablen.
- `doc/shell-variables/de/README.md` und `doc/shell-variables/en/README.md` — paketierbare Variante der Variablendokumentation.
