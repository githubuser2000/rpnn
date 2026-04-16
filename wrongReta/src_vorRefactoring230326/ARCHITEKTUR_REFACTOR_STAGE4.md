# Refactor Stage 4

## Neue Bündelung

- `src/domain/selection_state.rs`
  - ersetzt mehrere gekoppelte Bool-Felder durch Enums
- `src/processing/column_resolution/`
  - bündelt Hilfslogik für Spaltenauflösung

## Bool -> Enum

### 1. Leerinhalte
- Alt: `keineleereninhalte: bool`
- Neu: `empty_content_mode: EmptyContentMode`

### 2. Zeilenexpansion
- Alt: `vorher_vielfache: bool` + `vorher_primfaktoren: bool`
- Neu: `row_expansion_mode: RowExpansionMode`

### 3. Spaltenzustand
- Alt: `spalten_gefunden`, `spalten_gesucht`, `spalten_gesucht2`
- Neu: `column_request_state: ColumnRequestState`

### 4. Fraction-Inputs in PyPy-Kompatibilität
- Alt: `hidden_fraction_inputs: bool`
- Neu: `fraction_input_visibility: FractionInputVisibility`

## Warum das besser ist

Die alten Bool-Kombinationen kodierten implizite Zustände. Dadurch war nicht klar:
- welche Kombinationen gültig sind,
- welche Kombinationen eigentlich denselben Zustand meinen,
- und welche Kombinationen logisch unmöglich sein sollten.

Die Enums machen den Zustand explizit und reduzieren if-Ketten und Fehlerflächen.
