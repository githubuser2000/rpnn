# retaprompt_frontends

Aktive dünne Binärschicht für `rp`, `rpl`, `rpe`, `rpb`.

Diese Crate enthält absichtlich nahezu keine Fachlogik.
Die eigentliche retaPrompt-Logik liegt in den drei dynamischen Libraries:

- `libreta.so` — gemeinsamer Unterbau
- `libretaprompt_commands.so` — gemeinsame Befehls-/Command-Schicht für `rpb`, `rp`, `rpl`, `rpe`
- `libretaprompt_input.so` — eigene/interaktive CLI-Eingabeschicht für `rp`, `rpl`, `rpe`

Abhängigkeitsrichtung:

- `retaprompt_input -> retaprompt_commands -> reta`
- `rpb -> retaprompt_commands`
- `rp/rpl/rpe -> retaprompt_input`

Die vier Binärdateien sollen nur noch minimale `main()`-Einstiege sein.
