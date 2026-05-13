# Stage 14 Änderungen

Aufgebaut auf Stage 13. Dieser Schritt zieht die große Shell-/CLI-Parameterlaufzeit aus `reta.py` in eine explizite Architekturschicht.

## Neu

- `reta_architecture/parameter_runtime.py`
  - neue Schicht für die alte CLI-/Shell-Parametersemantik
  - enthält jetzt:
    - `ParameterRuntimeBundle`
    - `bootstrap_parameter_runtime(...)`
    - `produce_all_spalten_numbers(...)`
    - `apply_width_parameter(...)`
    - `parameters_to_commands_and_numbers(...)`
    - `upper_limit_values_for_argument(...)`
    - `upper_limit_from_arguments(...)`
    - `apply_upper_limit_argument(...)`
  - importiert die Legacy-Laufzeitmodule lazy, damit kein `center` ↔ `reta_architecture`-Importzyklus entsteht.

- `reta_architecture/facade.py`
  - `RetaArchitecture` besitzt jetzt `parameter_runtime`
  - neuer Bootstrap: `bootstrap_parameter_runtime(...)`
  - `snapshot()` enthält jetzt `parameter_runtime`

- `reta_architecture/__init__.py`
  - exportiert die Parameter-Runtime-Schicht und ihre Wrapper-Funktionen

- `reta_architecture_probe_py.py`
  - neuer Befehl: `parameter-runtime-json`

- `reta_architecture/package_integrity.py`
  - `reta_architecture/parameter_runtime.py` ist jetzt Pflichtdatei

- `tests/test_architecture_refactor.py`
  - neue Checks für `ParameterRuntimeBundle`
  - prüft, dass `reta.py` die große Parameterlogik nicht mehr selbst besitzt

- `tests/test_command_parity.py`
  - Test-Stubs erweitert um:
    - `prompt_toolkit`
    - `textwrap2`
    - `pyphen`
  - dadurch sind Paritätsläufe mit `python -S` in mageren Umgebungen stabiler.

## Was aus `reta.py` herausgezogen wurde

- Spaltenauswertung:
  - `produceAllSpaltenNumbers(...)`
  - inklusive der alten `resultingSpaltenFromTuple(...)`-Logik
  - inklusive positiver/negativer Spalten-Buckets

- Breiten-/Ausgabeparameter:
  - `breiteBreitenSysArgvPara(...)`

- Zeilen-/Parameterparser:
  - `parametersToCommandsAndNumbers(...)`
  - inklusive Zeilenbereichs-, Typ-, Primzahl-, Potenz-, Vielfachen- und Kombi-Parameterlogik

- Obergrenzenlogik:
  - `oberesMaximumArg(...)`
  - `oberesMaximum2(...)`
  - `oberesMaximum(...)`

## Harte Wirkung

- `reta.py` ist von ca. **1333 Zeilen** auf ca. **548 Zeilen** gefallen.
- Die CLI-Parametersemantik ist jetzt eine eigene Architekturschicht statt ein Monolith im Program-Kern.
- Der Program-Kern delegiert jetzt an:
  - `parameter_runtime`
  - `column_selection`
  - `table_generation`
  - `output_semantics`
  - `semantics_builder`

## Geprüft

- `py_compile` über alle Python-Dateien: OK
- `tests.test_architecture_refactor`: **29 Tests, OK**
- `reta_architecture_probe_py.py parameter-runtime-json`: OK
- `reta_architecture_probe_py.py package-integrity-json`: OK
- `reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- echter `reta.py`-Smoke-Test: OK
- kleine manuelle Paritätsprüfung gegen das Original:
  - Shell-Ausgabe für `Religionen/sternpolygon`: byte-identisch

## Stabil beobachtete Kernwerte

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`
- `csv/religion.csv = 1043 Zeilen`
- `csv/vn-religion.csv = 1043 Zeilen`

## Ehrlicher Stand

Die große vollständige Command-Parity-Matrix habe ich nicht als finale Aussage verwendet, weil die Original-Subprozesse in dieser Umgebung mit Stub-Abhängigkeiten langsam und teils unhandlich sind. Ich habe stattdessen die Architekturtests, Probe-Kommandos, einen echten `reta.py`-Smoke-Test und eine kleine Original-vs-Stage-14-Parität geprüft.

Der nächste sinnvolle Block ist jetzt der Rest in `reta.py`: vor allem `bringAllImportantBeginThings(...)`, `workflowEverything(...)` und `combiTableWorkflow(...)`. Danach wäre `reta.py` fast nur noch Program-Fassade.
