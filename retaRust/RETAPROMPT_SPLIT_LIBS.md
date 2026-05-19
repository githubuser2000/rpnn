# retaPrompt split libraries

## Gewollte Rollen

- `libreta.so`
  - nur `reta`
- `libretaprompt_commands.so`
  - retaPrompt-Command-Library
  - direkte Command-Seite für `rrpb`
  - Command-Parsen/-Ausführen für `rrp`, `rrpl`, `rrpe`
- `libretaprompt_input.so`
  - retaPrompt-Input-Library
  - eigene/interaktive Eingabe mit Autocomplete und Autosuggest für `rrp`, `rrpl`, `rrpe`

## Direkte Launcher-Zuordnung

- `rreta` -> `libreta.so`
- `rrpb` -> `libretaprompt_commands.so`
- `rrp`  -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpl` -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpe` -> `libretaprompt_input.so` + `libretaprompt_commands.so`

Damit ist `rrpb` bewusst command-only. Die drei Prompt-Frontends bekommen beide
Prompt-Bibliotheken, weil die Eingabeseite Autocomplete/Autosuggest bereitstellt
und die Command-Seite die eigentliche Befehlslogik trägt.

## Binary-Regel

Die Prompt-Executables werden im normalen `build.sh` nicht als Rust-Frontend-Binaries
gebaut, sondern nach dem Cargo-Build als kleine C-Launcher erzeugt. Dadurch wandert
maximal viel Code aus den Executables in die `.so`-Artefakte.

Die Prüfung ist hart:

- `rrp`, `rrpl`, `rrpe` müssen per `DT_NEEDED` beide Libraries sehen:
  - `libretaprompt_input.so`
  - `libretaprompt_commands.so`
- `rrpb` muss per `DT_NEEDED` nur `libretaprompt_commands.so` sehen
- `rrpb` darf nicht direkt gegen `libretaprompt_input.so` hängen

## Was der Build im aktuellen Stand leistet

Der Build hält die öffentliche ABI und die Launcher-Zuordnung ein und entfernt die
prompt-spezifische Rust-Frontend-Logik aus den finalen Executables.

Die vollständige Entdoppelung zwischen den Rust-`cdylib`-Artefakten selbst ist damit
noch nicht automatisch gelöst. Rust-`cdylib`-Abhängigkeiten können weiterhin Code in
die jeweiligen Shared Objects einbetten. Diese Änderung zieht aber die ausführbaren
Frontends auf die richtige dynamische Split-Struktur herunter.
