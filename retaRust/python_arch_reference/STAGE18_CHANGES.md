# Stage 18 Änderungen

Stage 18 baut direkt auf Stage 17 auf und setzt den Concat-Refactor fort. Der Schwerpunkt lag diesmal auf den schweren Generated-Column- und Meta-Column-Pfaden, ohne neu anzufangen.

## Neu bzw. deutlich erweitert

- `reta_architecture/generated_columns.py`
  - besitzt jetzt die schweren Generated-Column-Morphismen:
    - `concatVervielfacheZeile(...)`
    - `concatModallogik(...)`
    - `concat1RowPrimUniverse2(...)`
    - `concat1PrimzahlkreuzProContra(...)`
  - `GeneratedColumnRegistry` kennt jetzt **9** Generated-Column-Morphismen.
  - Runtime-Abhängigkeiten werden lazy geladen, damit die Architekturschicht importierbar bleibt.

- `reta_architecture/meta_columns.py`
  - neue explizite Schicht für Meta-/Konkret-/Theorie-/Abstrakt-Spaltenmorphismen.
  - enthält u. a.:
    - `spalteMetaKontretTheorieAbstrakt_etc_1(...)`
    - `spalteMetaKontretTheorieAbstrakt_etc(...)`
    - `spalteFuerGegenInnenAussenSeitlichPrim(...)`
    - `readOneCSVAndReturn(...)`
    - `findAllBruecheAndTheirCombinations(...)`
  - `MetaColumnsBundle` macht diese Morphismen abfragbar.

- `libs/lib4tables_concat.py`
  - ist jetzt weitgehend eine Kompatibilitätsfassade für herausgezogene Concat-/Generated-/Meta-Morphismen.
  - Umfang fiel auf ca. **677 Zeilen**.
  - Die Meta-Methoden wurden absichtlich als `*args/**kwargs`-Wrapper angebunden, weil mehrere interne Aufrufe historisch unterschiedliche Signaturen verwenden. Dadurch bleiben alte Aufrufpfade kompatibel, während die Semantik in der neuen Schicht liegt.

- `reta_architecture/facade.py`, `reta_architecture/__init__.py`, `reta_architecture/package_integrity.py`
  - kennen jetzt die Meta-Column-Schicht.
  - `RetaArchitecture.snapshot()` enthält jetzt auch `meta_columns`.

- `reta_architecture_probe_py.py`
  - neuer Befehl:
    - `meta-columns-json`

- `tests/test_architecture_refactor.py`
  - neue Regression für `MetaColumnsBundle`.

## Wichtigste Architekturwirkung

Stage 18 trennt zwei Familien, die vorher im selben Concat-Monolithen lagen:

1. **Generated Columns**: abgeleitete Spalten aus vorhandenen Zahlen-, Struktur- und Konzeptbedingungen.
2. **Meta Columns**: Meta/Konkret/Theorie/Abstrakt- und Prim-Innen/Außen/Seitlich-Klassifikationen.

Damit wird `lib4tables_concat.py` nicht mehr als semantischer Besitzer dieser Logik behandelt, sondern als Legacy-kompatibler Adapter.

## Geprüft

- `py_compile`: OK
- `python -S -m unittest -v`: **35 Tests, OK**
- `reta_architecture_probe_py.py generated-columns-json`: OK, Registry mit **9** Morphismen
- `reta_architecture_probe_py.py meta-columns-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Test `Religionen/sternpolygon`: OK
- Meta-Column-Smoke-Test `--universummetakonkret=meta`: OK

## Stabile Kernwerte

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

## Noch offen

Der nächste harte Block ist jetzt die verbleibende CSV-/Bruch-/`readConcatCsv(...)`-Logik in `libs/lib4tables_concat.py` sowie die spätere weitere Zerlegung des mittlerweile großen `generated_columns.py`.
