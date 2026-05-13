# Stage 25 – Table-Runtime als globale Tabellensektion

Stage 25 baut direkt auf Stage 24 auf. Der nächste Legacy-Block war `libs/tableHandling.py`: Dort lebte nach den vorherigen Extraktionen noch die eigentliche `Tables`-Runtimeklasse. Diese Klasse ist jetzt in die Architektur verschoben.

## Neue Architektur-Schicht

Neu ist:

- `reta_architecture/table_runtime.py`

Sie enthält jetzt:

- `Tables`
- `BreakoutException`
- `TableRuntimeBundle`
- `bootstrap_table_runtime(...)`

`Tables` ist damit nicht mehr Eigentum der alten `libs`-Schicht, sondern die explizite globale Tabellen-Sektion der neuen Architektur. Die Klasse bindet weiterhin die historischen Runtime-Komponenten zusammen:

- `Prepare`
- `Concat`
- `KombiJoin`
- `TableOutput`
- Generated-Column-Morphismen

## Wirkung auf `libs/tableHandling.py`

`libs/tableHandling.py` ist jetzt eine dünne Kompatibilitätsfassade.

- vorher Stage 24: ca. 275 Zeilen
- jetzt Stage 25: ca. 68 Zeilen

Das Modul re-exportiert nur noch alte Importpfade wie:

- `Tables`
- Output-Syntaxklassen
- Zahlenmorphismen
- `shellRowsAmount` / `setShellRowsAmount`
- `cliout`, `getTextWrapThings`, `i18n`, `infoLog`, `output`

Der eigentliche Tabellenkern liegt in `reta_architecture.table_runtime`.

## Wirkung auf `reta.py`

`reta.py` importiert `Tables` jetzt direkt aus:

```python
from reta_architecture.table_runtime import Tables
```

Außerdem kommen `OutputSyntax` und `primCreativity` nicht mehr über `tableHandling`, sondern direkt aus:

```python
from reta_architecture.output_syntax import OutputSyntax
from reta_architecture.number_theory import primCreativity
```

Damit hängt der Programmlauf nicht mehr an der alten TableHandling-Fassade.

## Architektur-Integration

- `RetaArchitecture` besitzt jetzt `table_runtime`.
- Neuer Bootstrap:
  - `RetaArchitecture.bootstrap_table_runtime(...)`
- `RetaArchitecture.snapshot()` enthält jetzt `table_runtime`.
- `reta_architecture_probe_py.py` hat den neuen Befehl:
  - `table-runtime-json`
- `reta_architecture/package_integrity.py` behandelt `reta_architecture/table_runtime.py` als Pflichtdatei.

## Importzyklen

Beim Herausziehen war wichtig, keinen neuen Zyklus über `center -> reta_architecture -> table_runtime -> center` zu erzeugen. Deshalb lädt `table_runtime.py` die historischen Legacy-Komponenten `Prepare`, `Concat` und `getTextWrapThings` lazy in kleinen Helperfunktionen.

## Tests

Geprüft wurde:

- `py_compile` über alle Python-Dateien: OK
- `reta_architecture_probe_py.py table-runtime-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- Smoke-Test `Religionen/sternpolygon`: OK
- `python -B -S -m unittest tests.test_architecture_refactor -v`: 45 Tests, OK
- `python -B -S -m unittest tests.test_command_parity -v`: 1 Paritätsmatrix-Test, OK
- `python -B -S -m unittest -v`: 46 Tests, OK

## Bedeutung

Mit Stage 25 ist nun auch `tableHandling.py` keine semantische Hauptdatei mehr. Der Tabellenzustand selbst ist als Architekturbaustein sichtbar und abfragbar. Das ist ein wichtiger Schritt, weil `Tables` der zentrale mutable Runtime-Knoten zwischen globaler Tabellensektion, Output-Morphismen, Zeilenfilterung, CSV-Gluing, Kombi-Joins und generierten Spalten ist.

Der nächste sinnvolle Block ist jetzt nicht mehr `tableHandling.py`, sondern die verbliebene Legacy-Fassadenlogik in `lib4tables_prepare.py` und `lib4tables_concat.py` weiter zu verdünnen oder die `TableRuntime` stärker typisiert in mehrere kleinere Sektionen zu zerlegen.
