# Reta `.so`-Split — deutsche Übersicht

Die Struktur ist jetzt als echte dünne Fassade umgesetzt: `rreta` linkt direkt nur gegen `libreta.so`; `libreta.so` exportiert die stabile öffentliche ABI und leitet die schwere Engine-Ausführung an `libreta_runtime.so` weiter. Die übrigen privaten Core-Bibliotheken bleiben als Topologie- und ABI-Grenzen eingebunden.

Damit gilt im Split-Build: `libreta.so` soll klein sein, `libreta_runtime.so` trägt den schweren nicht-interaktiven Reta-Core. Das Build-Skript prüft diese Größenrichtung und bricht ab, wenn `libreta.so` wieder größer oder gleich groß wie `libreta_runtime.so` wird.

Die Prompt-Programme bleiben getrennt: `rrpb` verwendet nur `libretaprompt_commands.so`, während `rrp`, `rrpl` und `rrpe` zusätzlich `libretaprompt_input.so` für Autocomplete und Autosuggest verwenden.

Siehe `doc/shared-libs/de/README.md` für die Einzeldokumentation jeder Bibliothek.

## Aktueller Korrekturstand

`rgrundStrukHtml` wird jetzt als kleiner C-Launcher gebaut und nutzt `libreta_render.so` direkt; `libreta_render.so` verlinkt zusätzlich gegen `libreta_semantics.so`. Außerdem exportieren `libreta_data.so`, `libreta_parse.so`, `libreta_semantics.so`, `libreta_table.so` und `libreta_render.so` reale Komponentenfunktionen. Die Build-Skripte prüfen, dass diese fünf Bibliotheken nicht wieder alle exakt dieselbe Stub-Größe haben.


`cargo run --bin rreta -- -h` und `cargo run --bin rgrundStrukHtml -- -h` funktionieren wieder ohne Feature-Flag. `rgrundStrukHtml` ist auch im Cargo-Pfad ein dynamischer Launcher und lädt `libreta_render.so`, statt `reta::shared::grundstruk_exact` direkt einzubetten.
