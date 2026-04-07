# rpnn modulweise Überschreib-ZIP – Scaffold

Dieses ZIP ist **kein erfundenes Gesamt-Refactoring**, sondern ein **modulweises Überschreib-Gerüst** mit den Pfaden,
die in deinem rpnn/reta-Projekt bei den letzten Fehlerbildern zentral waren.

Wichtig:
- Die Dateien sind als **overwrite scaffold** angelegt, also in derselben Modulstruktur wie im Repo.
- Ohne deinen aktuellen Repository-Stand kann ich hier **nicht seriös die echten finalen Rust-Dateien rekonstruieren**.
- Deshalb enthält dieses ZIP:
  - die **richtigen Zielpfade**,
  - pro Modul eine **präzise Zielbeschreibung**,
  - einen **sauberen Platz für echten Code**,
  - und ein **apply_overwrite.sh**, damit du später ein echtes Paket direkt drüberkopieren kannst.

## Zielmodule

- `src/domain/spalten_anfrage.rs`
- `src/domain/python_source_of_truth.rs`
- `src/domain/python_html_meta.rs`
- `src/reta_ausgabe/cli_output.rs`
- `src/table_printer/printer.rs`
- `src/table_printer/table_utils.rs`
- `src/input_help/input_validation.rs`
- `src/cli/mod.rs`

## Wofür dieses Paket gedacht ist

Für genau den Fall, den du mehrfach beschrieben hast:
- **keine monolithische reta_py.rs-Überfüllung**,
- stattdessen **modulweise Überschreib-Dateien**,
- mit Fokus auf:
  - typed parsing statt String-Normalisierung,
  - Python als source of truth,
  - `--alles` robust,
  - HTML-Meta/Classes algorithmisch,
  - Table-Layout parity,
  - Generator-/Spaltenrouting sauber.

## Nächster sinnvoller Schritt

Sobald du mir den aktuellen Repo-Stand oder die betroffenen Dateien gibst, kann ich aus genau diesem Scaffold ein **echtes overwrite-ZIP mit finalem Rust-Code** machen.
