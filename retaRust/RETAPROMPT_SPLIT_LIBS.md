# retaPrompt split libraries

## Gewollte Rollen

- `libreta.so`
  - nur `reta`
- `libretaprompt_commands.so`
  - retaPrompt-Command-Library
  - für `rpb`, `rp`, `rpl`, `rpe`
- `libretaprompt_input.so`
  - retaPrompt-Input-/Launcher-Library
  - für `rp`, `rpl`, `rpe`
  - oberer Launcher-Dispatch für `rp`, `rpl`, `rpb`, `rpe`

## Direkte Launcher-Zuordnung

- `rp`  -> `libretaprompt_input.so`
- `rpl` -> `libretaprompt_input.so`
- `rpe` -> `libretaprompt_input.so`
- `rpb` -> `libretaprompt_commands.so`

## Was der einfache Build im aktuellen Stand wirklich leistet

Der einfache Build (`build.sh`) hält die öffentliche ABI und die Launcher-Zuordnung ein.
Er hält **nicht automatisch** die Binär-Entdoppelung zwischen den drei `.so` ein,
weil die aktuellen Rust-`cdylib`-Abhängigkeiten dafür noch zu eng gekoppelt sind.

## Konsequenz

Die fachliche Festlegung ist richtig.
Die bisherige Shim-Lösung über `libreta.so` war dafür die falsche Umsetzung.
Für echte Entdoppelung ist eine weitere Code-Aufteilung nötig; ein bloß komplizierteres
Verpackungsskript reicht dafür nicht.
