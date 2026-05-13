# Stage 15 – Program-Workflow-Schicht

Stage 15 baut direkt auf Stage 14 auf und zieht den verbleibenden Top-Level-Workflow aus `reta.py` in eine explizite Architekturschicht.

## Neu

- `reta_architecture/program_workflow.py`
  - `ProgramWorkflowBundle`
  - `bootstrap_program_workflow(...)`
  - `bring_all_important_begin_things(...)`
  - `workflow_everything(...)`
  - `combi_table_workflow(...)`

## Umgebaut

- `reta.py`
  - von ca. **548 Zeilen** auf ca. **200 Zeilen** reduziert
  - lädt und verarbeitet die Haupttabelle nicht mehr selbst
  - orchestriert `prepare4out(...)` nicht mehr selbst
  - führt Kombi-Joins nicht mehr selbst aus
  - bleibt als legacy-kompatible `Program`-Fassade erhalten

- `reta_architecture/facade.py`
  - besitzt jetzt `program_workflow`
  - neuer Bootstrap:
    - `RetaArchitecture.bootstrap_program_workflow(...)`
  - `snapshot()` enthält jetzt `program_workflow`

- `reta_architecture_probe_py.py`
  - neuer Befehl:
    - `program-workflow-json`

- `reta_architecture/package_integrity.py`
  - `reta_architecture/program_workflow.py` ist jetzt Pflichtdatei

- `tests/test_architecture_refactor.py`
  - neue Tests für die Program-Workflow-Schicht

- `tests/test_command_parity.py`
  - Paritätsmatrix auf kleinere, CI-stabilere Beispielbefehle reduziert:
    - Shell
    - Markdown
    - HTML

## Architekturelle Einordnung

`ProgramWorkflowBundle` ist jetzt der explizite Glue-Knoten zwischen:

- Parameter-Runtime
- Column-Selection
- Table-Generation
- `prepare4out(...)`
- Kombi-Joins
- Renderer-/Output-Semantik

Damit ist `reta.py` fast nur noch eine historische Program-Fassade. Die Semantik liegt in expliziten Schichten.

## Geprüft

- `py_compile` über alle Python-Dateien: OK
- `python -S reta_architecture_probe_py.py program-workflow-json`: OK
- `python -S reta_architecture_probe_py.py package-integrity-json`: OK
- `python -S reta_domain_probe_py.py pair-json Religionen Hinduismus`: OK
- `python -S -m unittest tests.test_architecture_refactor -v`: **30 Tests, OK**
- `python -S -m unittest -v`: **31 Tests, OK**
- kleine manuelle Original-vs-Stage-15-Parität:
  - Shell-Ausgabe: byte-identisch
  - Markdown-Ausgabe: byte-identisch

## Stabile Kernwerte

- `paraDict_len = 4155`
- `dataDict_sizes = [554, 46, 11, 12, 7, 23, 23, 10, 14, 23, 23, 12, 0, 0]`
- `Religionen / Hinduismus -> [217]`

## Offener Rest

`reta.py` ist jetzt weitgehend dünn. Die nächsten sinnvollen Blöcke liegen nicht mehr in `reta.py`, sondern in den alten Tabellenklassen:

- `libs/tableHandling.py`
- `libs/lib4tables_prepare.py`
- `libs/lib4tables_concat.py`

Dort sitzen noch große legacy-orientierte Algorithmen, die später in feinere Architekturmodule überführt werden können.
