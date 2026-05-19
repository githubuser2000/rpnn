# Reta `.so`-Split — deutsche Übersicht

Die Struktur ist jetzt als echte dünne Fassade umgesetzt: `rreta` linkt direkt nur gegen `libreta.so`; `libreta.so` exportiert die stabile öffentliche ABI und leitet die schwere Engine-Ausführung an `libreta_runtime.so` weiter. Die übrigen privaten Core-Bibliotheken bleiben als Topologie- und ABI-Grenzen eingebunden.

Damit gilt im Split-Build: `libreta.so` soll klein sein, `libreta_runtime.so` trägt den schweren nicht-interaktiven Reta-Core. Das Build-Skript prüft diese Größenrichtung und bricht ab, wenn `libreta.so` wieder größer oder gleich groß wie `libreta_runtime.so` wird.

Die Prompt-Programme bleiben getrennt: `rrpb` verwendet nur `libretaprompt_commands.so`, während `rrp`, `rrpl` und `rrpe` zusätzlich `libretaprompt_input.so` für Autocomplete und Autosuggest verwenden.

Siehe `doc/shared-libs/de/README.md` für die Einzeldokumentation jeder Bibliothek.
