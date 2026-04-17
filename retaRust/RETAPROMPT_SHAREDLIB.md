# retaPrompt shared libraries: actual state vs. target state

## Fachliche Zieltrennung

Deine gewünschte Aufteilung ist fachlich klar:

- `libreta.so`
  - nur `reta`
  - kein retaPrompt-Besitz
- `libretaprompt_commands.so`
  - nur retaPrompt-Command-Seite
  - öffentliche ABI:
    - `retaprompt_commands_run_kind_from_env`
    - `retaprompt_commands_run_current_executable_from_env`
    - `retaprompt_commands_run_rp_from_env`
    - `retaprompt_commands_run_rpl_from_env`
    - `retaprompt_commands_run_rpb_from_env`
    - `retaprompt_commands_run_rpe_from_env`
- `libretaprompt_input.so`
  - nur retaPrompt-Input-/Launcher-Seite für `rp`, `rpl`, `rpe`
  - öffentliche ABI:
    - `retaprompt_input_run_kind_from_env`
    - `retaprompt_input_run_current_executable_from_env`
    - `retaprompt_input_run_any_current_executable_from_env`
    - `retaprompt_input_run_launcher_kind_from_env`
    - `retaprompt_input_run_rp_from_env`
    - `retaprompt_input_run_rpl_from_env`
    - `retaprompt_input_run_rpe_from_env`

## Was am alten Shim-Weg falsch war

Der frühere `tools/build_prompt_split_sharedlibs.sh` hat versucht, Doppelungen dadurch zu vermeiden,
dass die sichtbaren retaPrompt-Libraries nur noch C-Forwarder wurden und die eigentliche retaPrompt-
Laufzeit über `libreta.so` lief.

Das verletzt genau die gewünschte Trennung:

- prompt-spezifische ABI landet in `libreta.so`
- retaPrompt-Libraries werden künstlich leergezogen
- die öffentliche ABI der Command-Library wurde dabei sogar unvollständig
  (die Header/API wollten mehr als der Shim-Pfad tatsächlich exportierte)

Deshalb ist dieser Weg hier auf den einfachen Cargo- plus Launcher-Bau zurückgesetzt.

## Einfacher Bauweg

Der aktive einfache Bauweg bleibt:

```bash
./build.sh release
```

bzw.

```bash
./build.sh debug
```

Er macht genau zwei Dinge:

1. `cargo build --workspace`
2. danach die vier kleinen C-Launcher linken
   - `rp`, `rpl`, `rpe` gegen `libretaprompt_input.so`
   - `rpb` gegen `libretaprompt_commands.so`

## Wichtige Ehrlichkeit zum aktuellen Rust-Linking

Dieser einfache Bauweg respektiert die Library-Grenzen auf API- und Launcher-Ebene.
Er löst aber **noch nicht** die physische Entdoppelung des Rust-Codes zwischen den drei `.so`.

Grund:

- `retaprompt_commands` ist aktuell ein Rust-`cdylib` mit Rust-Abhängigkeit auf `reta`
- `retaprompt_input` ist aktuell ein Rust-`cdylib` mit Rust-Abhängigkeit auf `reta`
  und zusätzlich auf `retaprompt_commands`
- bei normalem Cargo-`cdylib`-Linking werden diese Rust-Abhängigkeiten nicht als saubere
  Laufzeitkette zwischen den drei öffentlichen `.so` realisiert, sondern in die jeweiligen
  Shared Objects eingebunden

Das heißt:

- die einfache Build-Kette bleibt einfach
- die fachliche Zuordnung bleibt richtig
- die vollständige Binär-Entdoppelung ist damit aber noch **nicht** erreicht

## Was für echte Entdoppelung noch nötig ist

Für echte Entdoppelung ohne falsches Verschieben nach `libreta.so` muss die Code-Besitzstruktur
geändert werden, nicht bloß das Verpackungsskript.

Dazu gehören insbesondere:

1. retaPrompt-spezifischen Code aus dem Root-`reta` herauslösen
2. `retaprompt_input -> retaprompt_commands` nicht mehr als normale Rust-`cdylib`-Abhängigkeit,
   sondern über eine echte ABI-/Laufzeitgrenze lösen
3. `retaprompt_commands -> reta` ebenfalls so anbinden, dass `reta` nicht in der Command-Library
   noch einmal als Rust-Code landet

Erst dann gilt wirklich:

- `libreta.so` nur `reta`
- `libretaprompt_commands.so` nur Command-Seite
- `libretaprompt_input.so` nur Input-/Launcher-Seite
- keine doppelte oder dreifache Eigenimplementierung in den drei `.so`
