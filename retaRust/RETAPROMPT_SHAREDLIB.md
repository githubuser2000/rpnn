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
rp/rpl/rpe/rpb -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
```

- `retaprompt_input` hängt bewusst von `retaprompt_commands` ab.
- `retaprompt_commands` hängt bewusst nur von `reta` ab.
- `reta` bleibt die Kernbibliothek.

## Aktiver Cargo-Bauweg

Die drei dynamischen Libraries werden im aktiven Verpackungsweg bewusst als kleine Forwarder mit expliziter Link-Abhängigkeitskette gebaut, damit `libreta.so` nicht mehrfach dupliziert wird und die Dateibeziehungen im ELF-Metadatenraum sichtbar bleiben:

```bash
./tools/build_prompt_split_sharedlibs.sh
```

Die vier Launcher sind dabei absichtlich extrem klein und enthalten nur den festen Einstiegspunkt je Frontend.

## Vier dünne Launcher

Es gibt vier extrem kleine Launcher-Binaries:

- `rp` ruft fest `retaprompt_input_run_launcher_kind_from_env(1)` auf
- `rpl` ruft fest `retaprompt_input_run_launcher_kind_from_env(2)` auf
- `rpb` ruft fest `retaprompt_input_run_launcher_kind_from_env(3)` auf
- `rpe` ruft fest `retaprompt_input_run_launcher_kind_from_env(4)` auf

Damit wissen die Executables selbst bereits, welches Frontend sie starten sollen, und müssen nicht erst über Dateinamen geraten.

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

## Warum vier feste Mini-Launcher?

So bleibt in den Executables fast nichts übrig:

- genau ein `main()`
- genau ein fester Sprung in `retaprompt_input`
- die komplette Fachlogik lebt weiter in den `.so`-Bibliotheken

Zusätzlich ist die Abhängigkeit jetzt auch auf ELF-Ebene klar:

- `rp/rpl/rpe/rpb` kennen `libretaprompt_input.so`
- `libretaprompt_input.so` kennt `libretaprompt_commands.so`
- `libretaprompt_commands.so` kennt `libreta.so`
