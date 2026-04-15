# retaPrompt Shared-Library Layout

Die aktive Cargo-Struktur für retaPrompt ist jetzt bewusst so aufgeteilt, dass
möglichst viel Lauf- und Dispatch-Logik in den drei `.so`-Bibliotheken liegt und
möglichst wenig in den vier Frontend-Binaries.

## Aktive Bibliotheken

- `libreta.so`
  - gesamte gemeinsame reta-Implementierung
  - Prompt-Grundlogik
  - Profile, Ausführung, Parser, Kommandos, UI-Layer

- `libretaprompt_commands.so`
  - gemeinsame retaPrompt-Befehlsbibliothek für `rpb`, `rp`, `rpl`, `rpe`
  - enthält die kommandoseitige Frontend-Zuordnung
  - kann anhand von `argv[0]` selbst erkennen, ob `rp`, `rpl`, `rpb` oder `rpe`
    gestartet wurde

- `libretaprompt_input.so`
  - eigene/interaktive Befehlseingabe für `rp`, `rpl`, `rpe`
  - hängt absichtlich von `libretaprompt_commands.so` ab
  - verwendet deren gemeinsame Frontend-Zuordnung als Unterbau
  - kann anhand von `argv[0]` selbst erkennen, ob `rp`, `rpl` oder `rpe`
    gestartet wurde

## Aktive Frontend-Binaries

Die aktiven Binaries liegen im Paket `crates/retaprompt_frontends` und sind
absichtlich extrem dünn:

- `rp`  -> nur Aufruf von `retaprompt_input::run_current_executable_from_env()`
- `rpl` -> nur Aufruf von `retaprompt_input::run_current_executable_from_env()`
- `rpe` -> nur Aufruf von `retaprompt_input::run_current_executable_from_env()`
- `rpb` -> nur Aufruf von `retaprompt_commands::run_current_executable_from_env()`

Damit liegt nicht nur die eigentliche Prompt-Implementierung, sondern auch die
Frontend-Auswahl und Profil-Dispatch-Logik in den `.so`-Bibliotheken statt in
den Executables.

## Abhängigkeitsrichtung

```text
libreta.so
  ↑
libretaprompt_commands.so
  ↑
libretaprompt_input.so
```

und

```text
rp  ─┐
rpl ─┼─> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
rpe ─┘

rpb ----> libretaprompt_commands.so -> libreta.so
```

## Cargo-Bauweg

Die drei `.so`-Bibliotheken und die vier dünnen Frontend-Binaries werden direkt
über Cargo gebaut:

```bash
cargo build --workspace
```

oder gezielt:

```bash
cargo build -p reta --lib
cargo build -p retaprompt_commands --lib
cargo build -p retaprompt_input --lib
cargo build -p retaprompt_frontends --bins
```
