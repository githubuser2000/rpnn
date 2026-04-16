# Refactor Stage 5

Diese Stufe geht tiefer als reines Verschieben von Dateien.

## Neu in Stage 5

### processing/
- `category_rules/`
  - `normalize.rs`
  - `pypy_compat.rs`
  - `generator_inference.rs`
  - `exact_columns.rs`
- `spalten_support/`
  - `normalize.rs`
  - `exact_merge.rs`
  - `selection_sync.rs`
  - `defaults.rs`

### domain/
- `generator_logic/`
  - `common.rs`
  - `csv_source.rs`
  - `number_theory.rs`
  - `modal.rs`
- `category_map/`
  - `normalize.rs`
  - `inference.rs`
  - `exact_lookup.rs`

## Inhaltliche Vereinfachungen
- Bool/Statuslogik bleibt in Stage 4 auf Enums umgestellt.
- `kategorie_verarbeiter.rs` ist jetzt nur noch Orchestrierung.
- `spalten_verarbeiter.rs` ist deutlich flacher, weil Exact-Merge, Fallback und Sync ausgelagert sind.
- `categories.rs` nutzt Hilfslogik aus `domain/category_map`.
- `generator_registry.rs` verwendet erste ausgelagerte Helper für Token-, CSV- und Zahlentheorie-Logik.

## Noch offene große Brocken
- `domain/generator_registry.rs` ist weiterhin der größte Kandidat für tiefe Zerlegung.
- `domain/categories.rs` enthält weiterhin sehr große Daten-Initialisierung.
- Ein echter nächster Schritt wäre:
  - Datenkonstanten aus `categories.rs` in mehrere thematische Dateien
  - Generatoren in thematische Gruppen (`modal`, `meta_konkret`, `prim`, `love`, `freiheit`)
  - ergänzende Unit-Tests für die extrahierten Helper
