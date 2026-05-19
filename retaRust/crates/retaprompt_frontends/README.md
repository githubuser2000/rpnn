# retaprompt_frontends

Diese Crate darf **keine schweren Prompt-Frontends** mehr erzeugen. Die vier öffentlichen
Targets `rrp`, `rrpl`, `rrpe` und `rrpb` sind nur noch ABI-Launcher. Sie rufen keine
`retaprompt_input::`- oder `retaprompt_commands::`-Rust-APIs direkt auf, sondern springen
über `extern "C"` in die Shared Libraries.

Aktive Topologie:

- `rrp`  -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpl` -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpe` -> `libretaprompt_input.so` + `libretaprompt_commands.so`
- `rrpb` -> `libretaprompt_commands.so`

`rrpb` ist command-only. `rrp`, `rrpl` und `rrpe` brauchen beide Libraries, weil
Input, Autocomplete und Autosuggest in `libretaprompt_input.so` liegen, während die
Command-Seite in `libretaprompt_commands.so` liegt.

## Harte Regel

In diesen Dateien darf kein direkter Rust-Crate-Aufruf stehen:

```text
src/bin/rp.rs
src/bin/rpl.rs
src/bin/rpe.rs
src/bin/rpb.rs
```

Verboten sind insbesondere:

```rust
retaprompt_input::...
retaprompt_commands::...
use retaprompt_input;
use retaprompt_commands;
```

Die Guard-Datei prüft das automatisch:

```bash
python3 tools/guard_prompt_frontend_sources.py
```

Die fertig gelinkten Executables werden zusätzlich geprüft:

```bash
tools/guard_prompt_launcher_topology.sh target/release
```

Diese Prüfung erzwingt:

- Größenlimit für `rrp`, `rrpl`, `rrpe`, `rrpb`, standardmäßig `262144` Bytes,
- keine Rust-Payload-Symbole im Launcher,
- `rrp/rrpl/rrpe` brauchen direkt beide Prompt-`.so`s,
- `rrpb` braucht nur `libretaprompt_commands.so`,
- keiner der Prompt-Launcher darf direkt `libreta*.so` brauchen.

## Warum diese Crate noch existiert

Sie bleibt für Cargo-Kompatibilität und Diagnose-Targets im Workspace. Das heißt aber
nicht, dass sie wieder Prompt-Logik in die Executables einbetten darf. Das alte Muster

```rust
fn main() {
    std::process::exit(retaprompt_input::run_rp_from_env());
}
```

ist ausdrücklich falsch, weil dadurch `rrp` wieder ein großes Rust-Frontend wird.


## Cargo-Kompatibilität aus dem Workspace-Root

`crates/retaprompt_frontends` bleibt wieder in `workspace.default-members`. Dadurch
funktionieren diese Smoke-Test-Befehle aus dem Repository-Root ohne zusätzliche
Package-Auswahl:

```bash
cargo run --bin rrp -- -h
cargo run --bin rrpl -- -h
cargo run --bin rrpe -- -h
cargo run --bin rrpb -- -h
```

Das macht die Executables nicht wieder groß, weil die vier Targets nur ABI-Launcher
sind. Der Source-Guard erzwingt, dass sie keine `retaprompt_input::`- oder
`retaprompt_commands::`-Rust-APIs direkt aufrufen. Die fertig gelinkten Launcher
werden zusätzlich durch `tools/guard_prompt_launcher_topology.sh` auf Größe und
`DT_NEEDED`-Topologie geprüft.

Wenn ein einzelnes Paket bewusst gewählt werden soll, bleibt auch diese Form gültig:

```bash
cargo run -p retaprompt_frontends --bin rrpb -- -h
```

## Richtiger Build

```bash
./build.sh release
./tools/package_prompt_split_sharedlibs.sh release
```

Der Paketweg baut die Shared Libraries und linkt danach die kleinen C-Launcher aus
`tools/launchers`. `RETA_BUILD_RUST_FRONTEND_BINS=1` ist absichtlich retired und führt
im aktiven Buildpfad zum Fehler, weil diese Variable früher die Größenregression
verursacht hat.
