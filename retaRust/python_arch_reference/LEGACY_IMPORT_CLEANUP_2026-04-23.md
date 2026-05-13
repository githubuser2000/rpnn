# Legacy-Import-Cleanup im Architekturpaket
Stand: 2026-04-23

## Ergebnis

Die letzten Architektur-Rückimporte aus den Legacy-Fassaden wurden entfernt.

- verbotene Treffer in `reta_architecture/` für `center`, `lib4tables_prepare`, `lib4tables_concat`, `lib4tables`, `tableHandling`: **0**
- neue Architekturmodule: `runtime_compat.py`, `table_adapters.py`
- betroffene Architekturmodule umgestellt: **11**

## Umgestellte Dateien

- `reta_architecture/runtime_compat.py`
- `reta_architecture/table_adapters.py`
- `reta_architecture/row_filtering.py`
- `reta_architecture/prompt_preparation.py`
- `reta_architecture/prompt_execution.py`
- `reta_architecture/parameter_runtime.py`
- `reta_architecture/table_output.py`
- `reta_architecture/generated_columns.py`
- `reta_architecture/meta_columns.py`
- `reta_architecture/concat_csv.py`
- `reta_architecture/combi_join.py`
- `reta_architecture/table_wrapping.py`
- `reta_architecture/table_runtime.py`
- `ARCHITECTURE_STATUS.md`

## Was technisch geändert wurde

- `runtime_compat.py` bündelt die früher aus `center.py` gezogenen Runtime-Namen nun architekturintern.
- `table_adapters.py` hält die dünnen Klassenadapter `Prepare` und `Concat` innerhalb des Architekturpakets.
- `table_runtime.py` erzeugt `Prepare` und `Concat` jetzt aus `reta_architecture.table_adapters` statt aus `lib4tables_prepare.py` oder `lib4tables_concat.py`.
- `row_filtering.py`, `prompt_preparation.py`, `prompt_execution.py`, `parameter_runtime.py`, `table_output.py`, `generated_columns.py`, `meta_columns.py`, `concat_csv.py`, `combi_join.py`, `table_wrapping.py` nutzen nur noch Architektur-Helfer.
- In `prompt_preparation.py` wurden zusätzlich vier offensichtliche Schreibfehler `Txt.liste = x` auf `Txt.liste = tx` korrigiert.

## Verifikation

Ausgeführt und erfolgreich:

- `python -m py_compile reta_architecture/runtime_compat.py reta_architecture/table_adapters.py reta_architecture/row_filtering.py reta_architecture/prompt_preparation.py reta_architecture/prompt_execution.py reta_architecture/parameter_runtime.py reta_architecture/table_output.py reta_architecture/meta_columns.py reta_architecture/concat_csv.py reta_architecture/combi_join.py reta_architecture/table_wrapping.py reta_architecture/table_runtime.py reta_architecture/generated_columns.py`
- `python -m unittest tests.test_architecture_refactor -v`
- `python -m unittest tests.test_command_parity -v`
