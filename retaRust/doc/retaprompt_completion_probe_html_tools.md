# retaPrompt Completion- und Probe/HTML-Portierungsstand

Dieser Stand behandelt nur die offenen Punkte:

1. Completion/Autosuggestion gegen Python-Verhalten testen.
2. Probe-/HTML-Extraktionstooling bewusst portieren oder markieren.

## Completion/Autosuggestion

Die interaktive Completion wurde nicht aggressiver gemacht. Die vorhandene
`reedline`-Completion und Autosuggestion bleibt unverändert. Neu sind nur:

- `retaprompt_completion_probe`: CLI-Probe, die die vorhandenen Rust-Kandidaten
  als JSON ausgibt.
- `tools/compare_retaprompt_completion.py`: Vergleich gegen die Python-Referenz
  `retaPrompt.py`/`NestedCompleter`.
- zusätzliche Rust-Tests im `retaprompt_input`-Crate, die konservative
  Python-Verhaltensverträge prüfen.

Der Vergleich prüft absichtlich keine vollständige Fuzzy-Ranking-Gleichheit.
Python `prompt_toolkit` kann Fuzzy-Kandidaten je nach Version anders sortieren.
Stattdessen werden Python-kritische Kandidaten und Modi geprüft, etwa `HELP`,
`reta`-Hauptparameter, Zeilen-/Spalten-/Kombinationswerte, `wahl15`/`wahl16`
und deaktivierte Completion im Löschmodus.

## Probe/HTML

`reta_domain_probe.rs` wurde näher an `reta_domain_probe_py.py` gerückt:

- `html-json <spaltennummer>` ist jetzt als Python-kompatibler Befehlsname
  vorhanden; `html <spaltennummer>` bleibt als Rust-Alias erhalten.
- `column-json` enthält wieder ein `html`-Objekt wie in Python.
- `pair-json` enthält keine HTML-Nutzlast mehr; dafür ist `pair-html-json`
  zuständig.
- `pair-html-json` folgt der Python-Form mit `input_main`, `input_parameter`,
  `canonical_main`, `canonical_parameter`, `columns` und `html`.
- Hauptparameter werden bei `pairs`, `pairs-json`, `main-columns` und
  `main-json` kanonisiert.

Zusätzlich gibt es nun `reta_extract_html_classes`, ein Rust-Gegenstück zu
`reta_extract_html_classes.py`. Es kann entweder ein `reta`-Binary starten oder
HTML von stdin parsen und schreibt `htmlclassesPy.jsonl`-kompatible JSONL-Zeilen.

## Befehle

```bash
cargo build -p retaprompt_input --bin retaprompt_completion_probe
cargo run -p retaprompt_input --bin retaprompt_completion_probe -- --line 'reta -zeilen --zeit=h'

python3 tools/compare_retaprompt_completion.py --python-repo ../reta --rust-repo .

cargo build --bin reta_domain_probe --bin reta_extract_html_classes
cargo run --bin reta_domain_probe -- html-json 2
cargo run --bin reta_extract_html_classes -- --stdin-html /tmp/htmlclassesPy.jsonl < /tmp/reta.html
```
