# Stage 13 – Spalten-Buckets und Tabellen-Gluing aus `reta.py` herausgezogen

Dieser Schritt baut direkt auf Stage 12 auf. Es wurde nicht neu begonnen; die vorhandene Topologie-/Morphismen-/Garben-/Prägarben-/Universalstruktur wurde erweitert.

## Neuer Architekturblock: `reta_architecture/column_selection.py`

Neu hinzugefügt:

- `COLUMN_BUCKET_NAMES`
- `ColumnSelectionBundle`
- `bootstrap_column_selection(...)`

Diese Schicht besitzt jetzt das frühere Inline-Schema der Spaltenarten aus `reta.py`:

- positive Buckets `(0, 0)` bis `(0, 11)`
- negative Buckets `(1, 0)` bis `(1, 11)`
- Legacy-kompatibles `SpaltenTyp`-NamedTuple mit Feldern wie `ordinary`, `generated1`, `concat1`, `kombi1`, `gebroUni1`, `gebrGal1`, `metakonkret`, usw.

Der konkrete Bindungspunkt ist:

- `ColumnSelectionBundle.bind_program_sections(...)`

Damit werden die Bucket-Sektionen zurück auf den bestehenden `Program`-Runtime-Zustand gebunden:

- `rowsAsNumbers`
- `generRows`
- `puniverseprims`
- `rowsOfcombi`
- `rowsOfcombi2`
- `onlyGenerated`
- `tables.getConcat.ones`
- `tables.SpaltenVanillaAmount`

Architektonisch ist das die explizite Schicht für die lokale Spaltenauswahl-Prägarbe.

## Neuer Architekturblock: `reta_architecture/table_generation.py`

Neu hinzugefügt:

- `TableGenerationBundle`
- `TableGenerationResult`
- `bootstrap_table_generation(...)`

Diese Schicht übernimmt den früher direkt in `reta.py` liegenden Gluing-Block für:

- `readConcatCsv(...)`
- Prim-/Gebrochen-Spalten-Ankleben
- frühe `prepare4out_beforeForLoop_SpaltenZeilenBestimmen(...)`-Bestimmung
- generierte Spalten:
  - `concatVervielfacheZeile`
  - `concatModallogik`
  - `concatPrimCreativityType`
  - `concatGleichheitFreiheitDominieren`
  - `concatGeistEmotionEnergieMaterieTopologie`
  - `concatMondExponzierenLogarithmusTyp`
  - `concat1RowPrimUniverse2`
  - `concat1PrimzahlkreuzProContra`
  - `concatLovePolygon`
  - `spalteFuerGegenInnenAussenSeitlichPrim`
  - `spalteMetaKontretTheorieAbstrakt_etc_1`
  - `createSpalteGestirn`
- Kombi-Joins über `readKombiCsv(...)`
- Synchronisation in die Tabellen-/Generated-Column-Sheaves

Der alte Algorithmus bleibt erhalten. Geändert wurde die Architekturposition: `reta.py` orchestriert diesen Block nicht mehr selbst, sondern delegiert an `TableGenerationBundle.build_for_program(...)`.

## Änderungen an `RetaArchitecture`

`reta_architecture/facade.py` hat jetzt zusätzlich:

- `column_selection: ColumnSelectionBundle`
- `bootstrap_column_selection(...)`
- `bootstrap_table_generation(...)`

`snapshot()` enthält jetzt außerdem:

- `column_selection`
- `table_generation`

## Änderungen an `reta.py`

`reta.py` wurde weiter reduziert:

- vorher Stage 12: ca. 1613 Zeilen
- nach Stage 13: ca. 1333 Zeilen

Entfernt bzw. ausgelagert aus `reta.py`:

- Inline-Erzeugung des `SpaltenTyp`-NamedTuple
- Inline-Erzeugung der 24 Spalten-Buckets
- direkte Bindung von `rowsAsNumbers`, `generRows`, `puniverseprims`, `rowsOfcombi`, usw.
- der große Block `CsvTheirsSpalten`
- direkter Generated-Column-/Concat-/Kombi-Gluing-Block

`reta.py` delegiert jetzt an:

```python
column_selection = self.architecture.bootstrap_column_selection(ordered_set_factory=OrderedSet)
self.spaltenTypeNaming = column_selection.type_naming
self.spaltenArtenKey_SpaltennummernValue = column_selection.new_bucket_map()
column_selection.bind_program_sections(self, paramLines)

table_generation = self.architecture.bootstrap_table_generation(csv_file_names=csvFileNames)
table_generation_result = table_generation.build_for_program(self, paramLines, paramLinesNot)
```

## Probe-Kommandos

`reta_architecture_probe_py.py` hat neue Kommandos:

```bash
python reta_architecture_probe_py.py column-selection-json
python reta_architecture_probe_py.py table-generation-json
```

## Tests

`tests/test_architecture_refactor.py` wurde erweitert um:

- `test_column_selection_layer_is_explicit`
- `test_table_generation_layer_is_explicit`
- `test_reta_program_delegates_column_and_table_generation_layers`

`package_integrity.py` verlangt jetzt zusätzlich:

- `reta_architecture/column_selection.py`
- `reta_architecture/table_generation.py`

## Geprüfte Stabilität

Getestet in dieser Umgebung mit externen Test-Stubs für fehlende optionale UI-Abhängigkeiten:

- `py_compile` über alle Python-Dateien: OK
- `tests.test_architecture_refactor`: 28 Tests, OK
- `tests.test_command_parity`: 1 Paritätsmatrix-Test, OK
- `python -m unittest -v`: 29 Tests, OK
- `reta_architecture_probe_py.py column-selection-json`: OK
- `reta_architecture_probe_py.py table-generation-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK nach Entfernen von Runtime-Artefakten
- direkte Markdown-Smoke-Ausgabe: OK

Stabile Kernwerte:

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

## Architekturgewinn

Der wichtigste Gewinn dieses Schritts: `reta.py` enthält nicht mehr selbst den großen Spalten-/Generated-Column-/Concat-/Kombi-Gluing-Block. Damit ist ein weiterer zentraler Teil der früher monolithischen Programmlogik in explizite Architekturschichten verschoben:

- Spaltenauswahl als explizite Kontext-/Prägarben-Schicht
- Tabellenaufbau als explizite universelle Gluing-Schicht
- vorhandene Concat-/Prepare-/Combi-Algorithmen bleiben als konkrete Morphismen erhalten
