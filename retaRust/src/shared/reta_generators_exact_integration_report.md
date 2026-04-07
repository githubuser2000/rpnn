# Generator-Transcompilierung: exakte Integrationsstufe

## Gemacht

- `src/shared/reta_concat_generators_py.rs` neu eingebunden.
- `src/shared/reta_generators_inventory_py.rs` neu eingebunden.
- Generatorpipeline wird jetzt in `workflowEverything()` tatsächlich aufgerufen.
- Exakte Generator-Familien aus Python als Inventory verdrahtet:
  - `generated1`
  - `generated2`
  - `boolAndTupleSet1`
  - `metakonkret`
- `Program` trägt jetzt die Generator-Metadaten explizit weiter:
  - `generated2Codes`
  - `boolAndTupleSet1Options`
  - `metakonkretPairs`
- `spalteFuerGegenInnenAussenSeitlichPrim()` verwendet jetzt die exakte Python-Reihenfolge aus `boolAndTupleSet1` statt nur das früh plattgedrückte `getConcat_ones`.
- `concatModallogik()`, `concat1RowPrimUniverse2()`, `concat1PrimzahlkreuzProContra()` und `spalteMetaKontretTheorieAbstrakt_etc_1()` hängen jetzt wenigstens an der richtigen Python-Metadatenquelle statt an Blindwerten.

## Noch nicht bitgenau fertig

Die vollständige Zelllogik dieser Python-Funktionen ist weiterhin nicht komplett in Rust gezogen:

- `concatModallogik`
- `concat1RowPrimUniverse2`
- `concat1PrimzahlkreuzProContra`
- `spalteMetaKontretTheorieAbstrakt_etc`

Der wesentliche Fortschritt hier ist aber architektonisch wichtig: die Generatoren hängen jetzt nicht mehr nur lose in einer Datei herum, sondern sind in die `reta`-Ablaufkette und die Python-Familien-Metadaten eingebunden.

## Relevante Dateien

- `src/shared/mod.rs`
- `src/shared/reta_program_types.rs`
- `src/shared/reta_begin_py.rs`
- `src/shared/reta_workflow_py.rs`
- `src/shared/reta_concat_generators_py.rs`
- `src/shared/reta_generators_inventory_py.rs`
