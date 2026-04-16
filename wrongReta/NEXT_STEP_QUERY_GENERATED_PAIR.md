Nächster Schritt der Ausrottung der String-Ersetzungen
=====================================================

In diesem Patch wurden zwei weitere aggressive Normalize-/Replace-Pfade entfernt:

1. `src/table_printer/query_generated_pair.rs`
2. `src/argument_verarbeiter_generated_pair.rs`

Beide Module verwenden jetzt `matches_any_alias(...)` aus
`src/domain/parser/legacy_cli_typed.rs` statt Zeichenketten mit `_`, `-`, `/`
und Leerzeichen wegzuschreddern.

Was als nächstes dran ist
-------------------------

Die größten verbleibenden Altlasten sind jetzt:

- `src/processing/category_rules/normalize.rs`
- `src/processing/spalten_support/normalize.rs`
- `src/domain/category_map/normalize.rs`
- `src/domain/spalten_anfrage.rs`
- `src/domain/html_meta_builder.rs`
- Teile von `src/table_printer/query.rs`

Der sinnvollste nächste harte Schnitt ist:

- die drei `normalize.rs`-Hilfsmodule stilllegen,
- deren Aufrufer auf explizite Alias-Tabellen oder getypte Parser umziehen,
- und danach `domain/spalten_anfrage.rs` als Legacy-Pfad systematisch abbauen.
