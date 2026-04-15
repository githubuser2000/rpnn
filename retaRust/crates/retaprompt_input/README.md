# retaprompt_input

Diese Zusatzbibliothek thematisiert ausschließlich die **eigene Befehlseingabe** für:

- `rp`
- `rpl`
- `rpe`

Sie enthält keinen `reta`-Kerncode, sondern ruft nur `reta::prompt` auf.
Für das statische Archiv `libretaprompt_input.a` wird absichtlich **kein** Rust-`staticlib`
benutzt, sondern ein winziger C-Forwarder, damit `libreta.a` nicht dupliziert wird.
