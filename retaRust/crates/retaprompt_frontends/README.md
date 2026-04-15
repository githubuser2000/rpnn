# retaprompt_frontends

Aktive Binärschicht als **einziger gemeinsamer Launcher** für `rp`, `rpl`, `rpe`, `rpb`.

Diese Crate enthält absichtlich praktisch keine Fachlogik. Die eigentliche retaPrompt-Logik liegt in den drei dynamischen Libraries:

- `libreta.so` — gemeinsamer Unterbau
- `libretaprompt_commands.so` — gemeinsame Befehls-/Command-Schicht für `rpb`, `rp`, `rpl`, `rpe`
- `libretaprompt_input.so` — eigene/interaktive CLI-Eingabeschicht für `rp`, `rpl`, `rpe` **und** oberster Launcher-Dispatch für alle vier Namen

Abhängigkeitsrichtung:

- `retaprompt_input -> retaprompt_commands -> reta`
- `retaprompt_launcher -> retaprompt_input`

Der Launcher entscheidet ausschließlich über `argv[0]`, ob er als `rp`, `rpl`, `rpe` oder `rpb` läuft.

Beabsichtigte Nutzung nach `cargo build -p retaprompt_frontends --bin retaprompt_launcher`:

```bash
cd target/debug
ln -sf retaprompt_launcher rp
ln -sf retaprompt_launcher rpl
ln -sf retaprompt_launcher rpe
ln -sf retaprompt_launcher rpb
```

Danach gilt:

- `./rp` -> Input-Frontend
- `./rpl` -> Input-Frontend
- `./rpe` -> Input-Frontend
- `./rpb` -> Command-Frontend

Die alten vier Einzel-Binärquellen bleiben im Repository erhalten, sind aber nicht mehr Teil des aktiven Cargo-Builds dieser Crate.
