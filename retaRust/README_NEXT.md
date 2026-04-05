# Transcompilation next

Dieser Stand friert die Python-Quelle härter ein.

## Neu

- `src/bin/reta.rs`
  - `Program` mit Methodenreihenfolge direkt aus Python
  - jede Methode als Rust-Stub
  - Python-Body jeder Methode als eingebettete String-Konstante
- `src/bin/grundStrukHtml.rs`
  - gesamte Python-Datei eingefroren
- `src/i18n/words.rs`
  - gesamte Python-Datei eingefroren
- `src/libs/LibRetaPrompt.rs`
  - gesamte Python-Datei eingefroren
- `src/libs/center.rs`
  - gesamte Python-Datei eingefroren
- `src/libs/tableHandling.rs`
  - gesamte Python-Datei eingefroren
- `python_reference/`
  - die Originaldateien separat zum direkten Vergleich

## Program-Methodenreihenfolge aus Python

- produceAllSpaltenNumbers
- breiteBreitenSysArgvPara
- storeParamtersForColumns
- parametersToCommandsAndNumbers
- helpPage
- bringAllImportantBeginThings
- oberesMaximumArg
- oberesMaximum2
- oberesMaximum
- propInfoLog
- propInfoLog
- __init__
- invertAlles
- run
- resultingTable
- workflowEverything
- combiTableWorkflow

## Nächster harter Block

1. `Program.produceAllSpaltenNumbers`
2. danach `__init__`
3. dann `run`
4. parallel die minimal nötigen Teile aus `center.py`, `tableHandling.py`, `words.py`
