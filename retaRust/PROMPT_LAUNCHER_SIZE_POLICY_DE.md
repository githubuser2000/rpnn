# Prompt-Launcher-Größenpolicy

Diese Datei ist eine harte Wartungsregel für `rrp`, `rrpl`, `rrpe` und `rrpb`.

## Ziel

Die vier Prompt-Executables sind nur Launcher. Sie dürfen keine Prompt-Algorithmen,
keine Completion-Logik, keine Autosuggest-Logik, keine Command-Parser und keine
Reta-Ausführungslogik enthalten.

Erlaubte Topologie:

```text
rrp  -> libretaprompt_input.so + libretaprompt_commands.so
rrpl -> libretaprompt_input.so + libretaprompt_commands.so
rrpe -> libretaprompt_input.so + libretaprompt_commands.so
rrpb -> libretaprompt_commands.so
```

`rrpb` bleibt command-only. `rrp`, `rrpl` und `rrpe` halten beide Prompt-Libraries
direkt, damit Eingabe/Autocomplete/Autosuggest und Command-Ausführung getrennt in
`.so`-Libraries leben, aber beide ABI-Kanten sichtbar bleiben.

## Verboten

In `crates/retaprompt_frontends/src/bin/rp.rs`, `rpl.rs`, `rpe.rs` und `rpb.rs` sind
verboten:

```rust
retaprompt_input::...
retaprompt_commands::...
use retaprompt_input;
use retaprompt_commands;
```

Diese Aufrufe betten Rust-Crate-Code in die Executables ein und machen sie wieder groß.

## Automatische Guards

```bash
python3 tools/guard_prompt_frontend_sources.py
tools/guard_prompt_launcher_topology.sh target/release
```

Der Source-Guard prüft die Rust-Quellen der Cargo-Launcher. Der Topologie-Guard prüft
die final gelinkten Dateien. Der Standard-Grenzwert für jeden Prompt-Launcher ist
`262144` Bytes und kann nur über `RETA_PROMPT_LAUNCHER_MAX_BYTES` geändert werden.

Diese Variable darf nicht als Ausrede benutzt werden, um Rust-Payload in den Launchern
zu akzeptieren. Wenn der Guard anschlägt, ist fast immer der falsche Buildpfad oder ein
Rückfall auf Rust-Frontend-Binaries die Ursache.

## Retired Buildpfad

`RETA_BUILD_RUST_FRONTEND_BINS=1` ist retired und führt im aktiven Build zu einem
absichtlichen Fehler. Diese Variable war ein Einfallstor für große Frontend-Binaries.
