# Stage 19 – CSV-/Bruch-Gluing aus `lib4tables_concat.py` herausgezogen

Stage 19 baut direkt auf Stage 18 auf. Der Schwerpunkt war der verbliebene CSV-/Bruch-/`readConcatCsv(...)`-Block in `libs/lib4tables_concat.py`.

## Neu

- `reta_architecture/concat_csv.py`
  - neue explizite Architektur-Schicht für CSV-Prägarben-Gluing und Bruch-/Fraction-Morphismen
  - enthält jetzt:
    - `ConcatCsvSpec`
    - `ConcatCsvBundle`
    - `bootstrap_concat_csv(...)`
    - `readConcatCsv(...)`
    - `readConcatCsv_tabelleDazuColchange(...)`
    - `readConcatCsv_ChangeTableToAddToTable(...)`
    - `readConcatCsv_LoopBody(...)`
    - `readConcatCsv_SetHtmlParamaters(...)`
    - `readConcatCSV_choseCsvFile(...)`
    - `convertSetOfPaarenToDictOfNumToPaareDiv(...)`
    - `convertSetOfPaarenToDictOfNumToPaareMul(...)`
    - `convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(...)`
    - `combineDicts(...)`

- `libs/lib4tables_concat.py`
  - bleibt als Legacy-kompatible `Concat`-Fassade erhalten
  - delegiert CSV-/Bruch-/`readConcatCsv(...)`-Logik jetzt an `reta_architecture.concat_csv`
  - Umfang fiel von ca. **581 Zeilen** auf ca. **252 Zeilen**

- `reta_architecture/table_generation.py`
  - besitzt jetzt explizit ein `ConcatCsvBundle`
  - nutzt `self.concat_csv.read_concat_csv(...)` statt direkt `program.tables.getConcat.readConcatCsv(...)`
  - `snapshot()` zeigt die CSV-Gluing-Schicht jetzt mit an

- `reta_architecture/facade.py`
  - besitzt jetzt `concat_csv`
  - neuer Bootstrap:
    - `RetaArchitecture.bootstrap_concat_csv(...)`
  - `snapshot()` enthält jetzt `concat_csv`

- `reta_architecture/__init__.py`
  - exportiert jetzt:
    - `ConcatCsvSpec`
    - `ConcatCsvBundle`
    - `bootstrap_concat_csv`

- `reta_architecture_probe_py.py`
  - neuer Befehl:
    - `concat-csv-json`

- `reta_architecture/package_integrity.py`
  - `reta_architecture/concat_csv.py` ist jetzt Pflichtdatei

## Zusätzlich repariert

Beim gezielten Test eines Bruch-/CSV-Gluing-Befehls fiel ein alter Fehler in `reta_architecture/parameter_runtime.py` auf:

- `parameter_runtime.py` referenzierte an mehreren Stellen noch `Program.ParametersMain`, `Program.lambdaPrimGalax` und `Program.lambdaGebrUnivUndGalax`.
- Das funktionierte nur zufällig in Pfaden, die diese gebrochen-rationalen Parameter nicht berührten.
- Stage 19 ersetzt diese Referenzen durch `type(self).ParametersMain`, `type(self).lambdaPrimGalax` und `type(self).lambdaGebrUnivUndGalax`.

Dadurch läuft jetzt auch ein repräsentativer Bruch-/CSV-Gluing-Befehl:

```bash
python -S reta.py -zeilen --vorhervonausschnitt=1-3 -spalten --gebrochenuniversum=5 --breite=40
```

## Tests erweitert

- `tests/test_architecture_refactor.py`
  - neue Tests für `ConcatCsvBundle`
  - neue Delegationsprüfung: `lib4tables_concat.py` delegiert an `reta_architecture.concat_csv`
  - Paketmanifest erwartet jetzt `reta_architecture/concat_csv.py`

- `tests/test_command_parity.py`
  - Paritätsmatrix enthält jetzt zusätzlich:
    - `shell-fractional-csv-gluing`
    - `-zeilen --vorhervonausschnitt=1-3 -spalten --gebrochenuniversum=5 --breite=40`

## Geprüft

- `py_compile` über alle Python-Dateien: OK
- `reta_architecture_probe_py.py concat-csv-json`: OK
- `reta_architecture_probe_py.py table-generation-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Test `Religionen/sternpolygon`: OK
- Smoke-Test `gebrochenuniversum=5`: OK
- `python -S -m unittest tests.test_architecture_refactor -v`: **36 Tests, OK**
- `python -S -m unittest tests.test_command_parity -v`: **1 Paritätsmatrix-Test, OK**
- `python -S -m unittest -v`: **37 Tests, OK**

## Stabile Kernwerte

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`
- `csv/religion.csv`: **1043 Zeilen**
- `csv/vn-religion.csv`: **1043 Zeilen**

## Paketstatus

- fehlende Pflichtdateien: **0**
- Runtime-Artefakte im finalen Baum: **0**
- ZIP-Test: OK

## Nächster sinnvoller Block

Der größte verbliebene Legacy-Bereich liegt jetzt nicht mehr in `readConcatCsv(...)`, sondern in der weiteren Zerlegung von:

- `reta_architecture/generated_columns.py`
- `reta_architecture/meta_columns.py`
- `libs/tableHandling.py`

Besonders `generated_columns.py` ist nach den letzten Extraktionen groß geworden. Der nächste gute Schritt wäre daher, generierte Spalten nach Familien aufzuteilen: Modal-/Prim-/Polygon-/Kreativitäts-/Mond-/ProContra-Morphismen.
