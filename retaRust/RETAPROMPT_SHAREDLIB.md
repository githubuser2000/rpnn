# retaPrompt shared library split

Zielzustand für retaPrompt:

- `libreta.so`
  - gesamte gemeinsame Implementierung und Python-nahe Kernlogik
- `libretaprompt_commands.so`
  - gemeinsame Befehls-/Command-Schicht für `rpb`, `rp`, `rpl`, `rpe`
- `libretaprompt_input.so`
  - eigene/interaktive CLI-Eingabeschicht für `rp`, `rpl`, `rpe`
  - oberster Launcher-Dispatch für `rp`, `rpl`, `rpe`, `rpb`

## Abhängigkeitsrichtung

```text
retaprompt_launcher -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
```

- `retaprompt_input` hängt bewusst von `retaprompt_commands` ab.
- `retaprompt_commands` hängt bewusst nur von `reta` ab.
- `reta` bleibt die Kernbibliothek.

## Aktiver Cargo-Bauweg

Die drei dynamischen Libraries kommen direkt aus Cargo:

```bash
cargo build -p reta --lib
cargo build -p retaprompt_commands --lib
cargo build -p retaprompt_input --lib
```

Der aktive Launcher ist genau **ein** Binary:

```bash
cargo build -p retaprompt_frontends --bin retaprompt_launcher
```

## Vier Namen über einen Launcher

`retaprompt_launcher` wertet `argv[0]` aus und entscheidet daraus, ob `rp`, `rpl`, `rpe` oder `rpb` gestartet wurde.

Praktische Nutzung im Build-Verzeichnis:

```bash
cd target/debug
ln -sf retaprompt_launcher rp
ln -sf retaprompt_launcher rpl
ln -sf retaprompt_launcher rpe
ln -sf retaprompt_launcher rpb
```

Dann gilt:

```text
rp  -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
rpl -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
rpe -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
rpb -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
```

Die fachliche Unterscheidung bleibt dabei in den Libraries:

- `rp`, `rpl`, `rpe` laufen im Input-Pfad
- `rpb` läuft im Command-Pfad

## Warum nur ein Launcher?

So bleibt in den Executables so wenig wie möglich:

- genau ein `main()`
- genau ein Sprung in `retaprompt_input`
- die komplette Namensauswertung und Laufartwahl lebt in den `.so`-Bibliotheken

Die früheren vier separaten Frontend-Binaries bleiben im Repository erhalten, sind aber nicht mehr aktiver Cargo-Bestandteil dieses Launcher-Pakets.
