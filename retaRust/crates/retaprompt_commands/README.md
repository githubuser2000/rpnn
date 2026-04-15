# retaprompt_commands

Diese Zusatzbibliothek thematisiert ausschließlich die **Befehlsseite** für:

- `rp`
- `rpl`
- `rpe`
- `rpb`

Sie enthält keinen `reta`-Kerncode, sondern ruft nur `reta::prompt` auf.
Für das statische Archiv `libretaprompt_commands.a` wird absichtlich **kein** Rust-`staticlib`
benutzt, sondern ein winziger C-Forwarder, damit `libreta.a` nicht dupliziert wird.
