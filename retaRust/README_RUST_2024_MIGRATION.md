# reta: Rust-2024-Umbau in Python-Richtung

Dieser Stand zieht das Cargo-/Crate-Gerüst von Rust 2021 auf Rust 2024 um, ohne die Python-Architektur künstlich in eine andere Rust-Struktur zu pressen.

## Gemachte Änderungen

- `edition = "2024"`
- Paketname von `reta_bitexact_direction_pass` auf `reta` gezogen
- Library-Name von `reta_shared` auf `reta` gezogen
- `default-run = "reta"` gesetzt
- `autobins = false` gesetzt, damit die Binärziele explizit und stabil bleiben
- `grundStrukHtml` als explizites Binärziel eingetragen
- `src/bin/reta.rs` auf den neuen Library-Namen umgestellt
- kleine idiomatische Rust-Glättung in `src/bin/reta.rs`:
  - `len() > 0` -> `is_empty()`
  - Iteration über Referenzen statt expliziter `.iter()`-Vergleiche in Bedingungen

## Warum diese Richtung

Die Python-Architektur von reta ist der Maßstab. Deshalb wurde **nicht** aggressiv in generische Rust-Schichten, neue Modulbäume oder rein rust-idiomatische Umbenennungen umgebaut. Die Cargo-/Crate-Ebene wurde auf 2024 gehoben, aber die transkompilierte Python-Struktur bleibt die führende Struktur.

## Was absichtlich noch nicht massenhaft umbenannt wurde

Es gibt im Projekt viele historisch gewachsene Dateinamen und Modulnamen. Eine flächige Umbenennung aller Dateien und Symbole nur wegen Rust-Stil wäre für bitgenaue Python-Parität riskant. Deshalb ist dieser Umbau bewusst konservativ:

- Python-nahe Namen bleiben erhalten
- nur die Crate-/Binär-Grenze wurde auf `reta` ausgerichtet
- kein inhaltlicher Wegfall

## Nächste harte Prüfungen lokal

- `cargo build --bins`
- `cargo run --bin reta -- -zeilen --vorhervonausschnitt=1-9 -spalten --alles`
- `cargo run --bin grundStrukHtml`
- Python/Rust-Diff für bekannte Generatorfälle und HTML-Ausgabe

## Erwartbare Nacharbeit

Rust 2024 kann an einzelnen Stellen zusätzliche Compilerdiagnosen auslösen. Ohne lokalen Compilerlauf in dieser Umgebung wurde deshalb nur der sichere, architektonisch saubere 2024-Umbau an der Crate-Grenze vorgenommen.
