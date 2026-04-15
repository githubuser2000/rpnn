# retaPrompt shared-library split

Ziel:

- `libreta.so` bleibt der gemeinsame Unterbau und trägt die eigentliche Implementierung.
- `libretaprompt_commands.so` bündelt die Befehls-/Command-Seite für `rpb`, `rp`, `rpl`, `rpe`.
- `libretaprompt_input.so` bündelt die eigene/interaktive CLI-Eingabeseite für `rp`, `rpl`, `rpe`.

Cargo-Struktur:

- Root-Paket `reta` erzeugt `libreta.so`.
- `crates/retaprompt_commands` erzeugt `libretaprompt_commands.so`.
- `crates/retaprompt_input` erzeugt `libretaprompt_input.so`.
- `crates/retaprompt_frontends` erzeugt nur noch die dünnen Launcher-Binaries `rp`, `rpl`, `rpe`, `rpb`.

Abhängigkeitsrichtung:

- `retaprompt_commands -> reta`
- `retaprompt_input -> retaprompt_commands -> reta`
- `rpb -> retaprompt_commands`
- `rp/rpl/rpe -> retaprompt_input`

Wichtige Folge:

- So viel retaPrompt-Logik wie möglich liegt in den drei `.so`-Libraries.
- Die vier Executables enthalten nur noch minimale `main()`-Einstiege.
- Die älteren Root-Binaries unter `src/bin/rp.rs`, `rpl.rs`, `rpe.rs`, `rpb.rs` bleiben als Altcode im Repository erhalten, sind aber nicht der aktive Cargo-Buildweg.

Bauen mit Cargo:

```bash
cargo build --workspace
```

Gezielt nur die drei dynamischen Libraries:

```bash
cargo build -p reta --lib
cargo build -p retaprompt_commands --lib
cargo build -p retaprompt_input --lib
```

Gezielt nur die dünnen Frontends:

```bash
cargo build -p retaprompt_frontends --bins
```
