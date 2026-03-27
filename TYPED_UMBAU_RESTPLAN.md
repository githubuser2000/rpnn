# Typed-Umbau: Restarbeiten und Stand

Dieses Patch-Paket zieht den begonnene typed Request-Pfad weiter durch.

## Geänderte Dateien

- `src/domain/spalten_anfrage.rs`
- `src/domain/categories.rs`
- `src/domain/request_pipeline.rs`
- `src/domain/exact_generator_bridge.rs`
- `src/domain/resolve_cli_legacy_adapter.rs`
- `src/domain/python_html_meta.rs`
- `src/processing/kategorie_verarbeiter.rs`
- `src/processing/spalten_verarbeiter.rs`

## Ziel

- String-Split auf `to_cli()` aus dem inneren Pfad entfernen.
- typed `SpaltenAnfrage` zentraler machen.
- exact/generator/category-Resolver über denselben typed Request ansprechbar machen.
- zusätzliche `Menschliches`-Unterkategorien typisieren.

## Noch offen

- `KategorieMap` ist weiter datengetrieben und nicht vollständig `HashMap<SpaltenAnfrage, _>`.
- Das zweite, modellbasierte Request-System unter `src/domain/model/` existiert parallel noch weiter.
- `app/workflow.rs` konstruiert `ParametersMain` weiter aus dem letzten CLI-Eintrag statt direkt aus einem aggregierten typed Request-Modell.
- Für einen endgültigen Endzustand sollte eines der beiden Request-Modelle entfernt werden.
