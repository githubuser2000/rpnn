# RETAPROMPT shared-library layout

Aktiver Cargo-Aufbau:

- `reta` -> `libreta.so` (Kern-Implementierung)
- `retaprompt_commands` -> `libretaprompt_commands.so` (alle retaPrompt-Befehle für `rpb`, `rpl`, `rpe`, `rp`)
- `retaprompt_input` -> `libretaprompt_input.so` (eigene CLI-Befehlseingabe für `rpl`, `rp`, `rpe`)
- `retaprompt_frontends` -> dünne Binaries `rp`, `rpl`, `rpe`, `rpb`

Wichtig:

- Der primäre Weg ist jetzt **Cargo direkt**, nicht die früheren `.sh`-Skripte.
- `retaprompt_input` hängt absichtlich von `retaprompt_commands` ab, damit die gemeinsame retaPrompt-Befehls-API nur an einer Stelle definiert wird.
- `retaprompt_frontends` ist die aktive Binärschicht, weil das Root-Paket `reta` sonst einen Cargo-Abhängigkeitszyklus zu den Split-Libs erzeugen würde.

Typischer Build:

```bash
cargo build --workspace
```

Explizit nur die drei dynamischen Libraries:

```bash
cargo build -p reta --lib
cargo build -p retaprompt_commands --lib
cargo build -p retaprompt_input --lib
```

Explizit die vier dünnen Frontends:

```bash
cargo build -p retaprompt_frontends --bins
```

Erwartete Artefakte unter `target/debug` oder `target/release`:

```text
libreta.so
libretaprompt_commands.so
libretaprompt_input.so
rp
rpl
rpe
rpb
```

Laufzeitkette:

```text
rp  -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
rpl -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
rpe -> libretaprompt_input.so -> libretaprompt_commands.so -> libreta.so
rpb -> libretaprompt_commands.so -> libreta.so
```
