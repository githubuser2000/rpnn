# Refactor Stage 2

Dieses Refactoring führt eine klarere Schichtenstruktur ein, ohne die bestehende Fachlogik zu entfernen.

## Neue Struktur

- `src/app/`
  - Einstieg und Orchestrierung
- `src/processing/`
  - Verarbeitung von Eingaben und Kategorien
- `src/data_access/`
  - CSV/SQLite-Zugriff
- `src/domain/`
  - Fachlogik, Kategoriemodelle, Generator-Mappings, Kompatibilitätslogik
- bestehende Spezialordner bleiben erhalten:
  - `src/cli/`
  - `src/column_manager/`
  - `src/table_printer/`
  - `src/reta_ausgabe/`
  - `src/input_help/`
  - `src/multiples_teiler/`

## Bewusst beibehalten

Zur Minimierung des Risikos wurden Legacy-Kompatibilitätsmodule auf Top-Level beibehalten:

- `workflows.rs`
- `argument_verarbeiter.rs`
- `kategorie_verarbeiter.rs`
- `csv_importer.rs`
- `data_fetcher.rs`
- `tabellen_utils.rs`
- `column_categories_complete.rs`
- `generated_columns_words_registry.rs`
- `exact_generator_bridge.rs`
- `python_exact_mappings.rs`
- `pypy_compat.rs`

Diese Dateien re-exportieren nur noch aus den neuen Schichten. Dadurch bleiben alte `crate::...`-Pfade weitgehend nutzbar, während neuer Code bereits auf die neue Struktur zeigen kann.

## Nächste sinnvolle Schritte

1. `table_printer/printer.rs` weiter zerlegen:
   - Breitenberechnung
   - Chunking
   - Zeilennummern
   - Rendering
2. `cli/parser.rs` aufteilen in:
   - Tokenisierung
   - Bereichsparser
   - Spaltenparser
   - Breitenparser
3. `kategorie_verarbeiter.rs` in kleinere Mapping-Dateien aufspalten:
   - Kombi-Mappings
   - Fraction-Mappings
   - Generator-Inferenz
4. `generated_columns_words_registry.rs` und `column_categories_complete.rs` in datenlastige Teilmodule aufteilen.
