# Stage 17: Generated-Column-Morphismenschicht

Stage 17 baut direkt auf Stage 16 auf und nimmt den ersten sauberen Schnitt im nächsten großen Restblock vor: `libs/lib4tables_concat.py`.

## Neu

- `reta_architecture/generated_columns.py`
  - neue explizite Architekturschicht für einfache generierte Spaltenmorphismen
  - enthält jetzt:
    - `GeneratedColumnSpec`
    - `GeneratedColumnRegistry`
    - `GeneratedColumnsBundle`
    - `bootstrap_generated_columns(...)`
  - besitzt eine Registry für die wichtigsten generierten Spaltenmorphismen:
    - `concatVervielfacheZeile`
    - `concatPrimCreativityType`
    - `concatGleichheitFreiheitDominieren`
    - `concatGeistEmotionEnergieMaterieTopologie`
    - `concatMondExponzierenLogarithmusTyp`
    - `concatLovePolygon`

- `libs/lib4tables_concat.py`
  - die folgenden alten Methoden sind jetzt nur noch Kompatibilitätswrapper:
    - `concatLovePolygon(...)`
    - `gleichheitFreiheitVergleich(...)`
    - `geistEmotionEnergieMaterieTopologie(...)`
    - `concatGleichheitFreiheitDominieren(...)`
    - `concatGeistEmotionEnergieMaterieTopologie(...)`
    - `concatPrimCreativityType(...)`
    - `concatMondExponzierenLogarithmusTyp(...)`
  - der eigentliche Code dieser Morphismen lebt jetzt in `reta_architecture/generated_columns.py`
  - `lib4tables_concat.py` sank von ca. 3134 auf ca. 2849 Zeilen

- `reta_architecture/table_generation.py`
  - nutzt jetzt `GeneratedColumnsBundle` für die herausgezogenen generierten Spalten
  - Snapshot enthält jetzt `generated_columns_registry`

- `reta_architecture/facade.py`
  - `RetaArchitecture` besitzt jetzt `generated_columns`
  - neuer Bootstrap:
    - `RetaArchitecture.bootstrap_generated_columns(...)`
  - `snapshot()` enthält jetzt `generated_columns`

- `reta_architecture_probe_py.py`
  - neuer Befehl:
    - `generated-columns-json`

- `reta_architecture/package_integrity.py`
  - `reta_architecture/generated_columns.py` ist jetzt Pflichtdatei

- `tests/test_architecture_refactor.py`
  - neue Regression für die Generated-Columns-Schicht

## Architektonischer Gewinn

Die einfachen generierten Spalten sind nicht mehr bloß historische Methoden in `Concat`, sondern explizite Morphismen in einer eigenen Architekturschicht. `Concat` bleibt als Legacy-Fassade bestehen, aber die Eigentümerschaft liegt jetzt bei `reta_architecture/generated_columns.py`.

Das ist der richtige nächste Schritt, weil Generated Columns in Reta semantisch keine normalen CSV-Daten sind. Sie sind abgeleitete Sektionen: Aus vorhandenen Spalten, Zeilennummern, Tags und Ausgabemodi werden neue Spalten erzeugt und wieder in die globale Tabellensektion eingeklebt.

## Bewusst noch nicht komplett erledigt

`libs/lib4tables_concat.py` ist noch nicht vollständig zerlegt. Die schwereren Blöcke leben weiterhin dort, besonders:

- `concatModallogik(...)`
- `concat1PrimzahlkreuzProContra(...)`
- `concat1RowPrimUniverse2(...)`
- `spalteMetaKontretTheorieAbstrakt_etc_1(...)`
- `spalteMetaKonkretTheorieAbstrakt_*`
- `readConcatCsv(...)` und seine Helper

Stage 17 zieht also nicht den ganzen Concat-Monolithen heraus, sondern den ersten gut abgegrenzten Generator-Morphismenblock.

## Geprüft

- `py_compile` über alle Python-Dateien: OK
- `python -S -m unittest -v`: 34 Tests, OK
- `reta_architecture_probe_py.py generated-columns-json`: OK
- `reta_architecture_probe_py.py table-generation-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- echter `reta.py`-Smoke-Test für `Religionen/sternpolygon`: OK

## Stabile Kernwerte

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`
