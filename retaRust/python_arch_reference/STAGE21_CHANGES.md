# Stage 21 Änderungen

Aufbauend auf Stage 20 wurde der nächste Block aus `libs/tableHandling.py` herausgezogen.

## Neu

- `reta_architecture/combi_join.py`
  - neue explizite Schicht für Kombi-Join-/Relationenmorphismen
  - enthält jetzt:
    - `KombiJoin`
    - `KombiJoinBundle`
    - `bootstrap_combi_join(...)`
    - `prepareTableJoin(...)`
    - `removeOneNumber(...)`
    - `tableJoin(...)`
    - `prepare_kombi(...)`
    - `readKombiCsv(...)`
    - `kombiNumbersCorrectTestAndSet(...)`

- `reta_architecture/generated_columns.py`
  - `createSpalteGestirn(...)` ist jetzt ein expliziter Generated-Column-Morphismus
  - `GeneratedColumnsBundle` besitzt jetzt `create_spalte_gestirn(...)`
  - die Registry enthält jetzt den Morphismus `createSpalteGestirn`

- `libs/tableHandling.py`
  - die alte verschachtelte Klasse `Tables.Combi` lebt nicht mehr dort als Implementierung
  - `Tables.Combi = KombiJoin` bleibt als Kompatibilitätsalias erhalten
  - `Tables.Maintable.createSpalteGestirn(...)` ist nur noch ein Wrapper auf die Generated-Column-Schicht
  - Umfang fiel von ca. 881 Zeilen auf ca. 275 Zeilen

- `reta_architecture/table_generation.py`
  - kennt jetzt `KombiJoinBundle`
  - der Snapshot enthält jetzt `combi_join`
  - `createSpalteGestirn` wird direkt über `GeneratedColumnsBundle` aufgerufen

- `RetaArchitecture`
  - besitzt jetzt `combi_join`
  - neuer Bootstrap:
    - `bootstrap_combi_join(...)`
  - `snapshot()` enthält jetzt `combi_join`

- `reta_architecture_probe_py.py`
  - neuer Befehl:
    - `combi-join-json`

- `reta_architecture/package_integrity.py`
  - `reta_architecture/combi_join.py` ist jetzt Pflichtdatei

## Zusätzliche Robustheitskorrekturen

- `libs/center.py`
  - besitzt jetzt lokale Fallbacks für fehlendes `rich`
  - besitzt jetzt lokale Fallbacks für fehlendes `textwrap2` / `pyphen`
  - dadurch laufen Architektur-Probes und Smoke-Tests auch mit `python -S` in mageren Umgebungen

- `reta_architecture/prompt_session.py`
  - besitzt jetzt lokale Fallbacks für fehlendes `prompt_toolkit`
  - dadurch bleibt die Architekturschicht auch ohne installierte Prompt-UI inspizierbar

## Geprüft

- `py_compile` über alle Python-Dateien: OK
- `reta_architecture_probe_py.py combi-join-json`: OK
- `reta_architecture_probe_py.py generated-columns-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Test `Religionen/sternpolygon`: OK
- `python -B -S -m unittest tests.test_architecture_refactor -v`: **39 Tests, OK**
- `python -B -S -m unittest tests.test_command_parity -v`: **1 Paritätsmatrix-Test, OK**
- `python -B -S -m unittest -v`: **40 Tests, OK**

## Stabile Kernwerte

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`
- `csv/religion.csv`: **1043 Zeilen**
- `csv/vn-religion.csv`: **1043 Zeilen**

## Paketstatus

- Dateien im Manifest: **283**
- fehlende Pflichtdateien: **0**
- Runtime-Artefakte: **0**
- ZIP-Test: OK

## Noch offen

Der nächste sinnvolle Block ist jetzt die restliche Tabellen-Kernstruktur in `libs/tableHandling.py`: `Tables` selbst kann weiter zu einer dünnen globalen Tabellen-Sektionsfassade werden. Danach sind `lib4tables_prepare.py` und tiefe Renderer-/Zellformatierungsdetails die nächsten Kandidaten.
