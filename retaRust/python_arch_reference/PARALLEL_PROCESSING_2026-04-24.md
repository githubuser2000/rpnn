# Prozessbasierte Chunk-Parallelisierung für PyPy3

Stand: 2026-04-24

## Ziel

Für PyPy3 gibt es keinen GIL-freien CPU-Thread-Speedup. Deshalb wurde keine
Thread-Schicht eingebaut, sondern eine optionale **Multiprocessing-Schicht** für
chunk-basierte Tabellen-/Zeilengenerierung.

Der Schnitt ist bewusst konservativ:

- Stark mutierendes CSV-/Spalten-Gluing bleibt seriell, soweit es globale
  Tabellenzustände, generierte Spaltenparameter oder Tags schreibt.
- Die spätere Zeilen-/Zellenvorbereitung wird chunkweise parallelisiert, weil
  die Zeilen nach der Zeilenauswahl unabhängig berechnet werden können.
- Zusätzlich parallelisiert sind jetzt die sicheren Pure-Table-Schnitte:
  Religion-CSV-Zellendekodierung, Kombi-CSV-Zellenvorbereitung,
  Spaltenprojektion nach der Tabellenvorbereitung, maximale Zellbreiten-Scans
  und Kombi-Join-Teiltabellenvorbereitung.
- Die Header-/Tag-Zeile bleibt seriell, weil dort die globalen
  `generatedSpaltenParameter` und Tag-Zuordnungen geschrieben werden.
- Die Worker liefern vorbereitete Stücke zurück; der Hauptprozess klebt die
  Chunks deterministisch in Originalreihenfolge zusammen.

## Neue Architektur-Schicht

Neu hinzugefügt:

- `reta_architecture/parallel_execution.py`

Diese Schicht enthält:

- `ParallelExecutionConfig`
- `ParallelExecutionBundle`
- `ProcessorCoreCounts`
- globale Kernzahl-Konstanten:
  `RETA_PROCESSOR_CORES`, `RETA_PHYSICAL_PROCESSOR_CORES`,
  `RETA_VIRTUAL_PROCESSOR_CORES`, `RETA_AVAILABLE_PROCESSOR_CORES`,
  `RETA_PARALLEL_PROCESSOR_CORES`
- `ParallelRowsResult`
- `ParallelOperationResult`
- `extract_parallel_config_from_argv()`
- `apply_parallel_environment()`
- `prepare_rows_in_processes()`
- `decode_religion_rows_in_processes()`
- `decode_kombi_rows_in_processes()`
- `select_columns_in_processes()`
- `max_cell_text_len_in_processes()`
- `prepare_kombi_join_tables_in_processes()`

Die Schicht ist in `RetaArchitecture` eingebunden und erscheint in
`architecture.snapshot()` als `parallel_execution`.

## Aktivierung

Standardverhalten:

- auf **PyPy/PyPy3**: `mode=auto` aktiviert Prozessparallelisierung ab dem
  Schwellwert
- auf **CPython**: `mode=auto` bleibt aus, damit keine Prozesskosten entstehen
- ohne explizites `--parallel-workers=N` verwendet die Config
  `RETA_PARALLEL_PROCESSOR_CORES` als globale Default-Prozesszahl; das ist die
  Zahl der für den Prozess sichtbaren virtuellen/logischen Kerne, mit
  zusätzlichem Snapshot der physischen Kerne
- Pools werden auf die Zahl vorhandener Chunks gedeckelt, damit keine leeren
  Worker gestartet werden
- auf allen Runtimes kann Parallelisierung explizit erzwungen oder deaktiviert
  werden

CLI-Beispiele:

```bash
pypy3 reta.py --parallel=processes --parallel-workers=4 --parallel-chunk-size=128 ...
python3 reta.py --parallel=processes --parallel-workers=4 --parallel-threshold=1 ...
python3 reta.py --no-parallel ...
```

Umgebungsvariablen:

```bash
RETA_PARALLEL=processes
RETA_PARALLEL_WORKERS=4
RETA_PARALLEL_CHUNK_SIZE=128
RETA_PARALLEL_THRESHOLD=128
RETA_PARALLEL_START_METHOD=fork
```

`retaPrompt.py` akzeptiert dieselben Parallel-Flags beim Start. Die Flags werden
aus `sys.argv` entfernt und als Prozessumgebung an die später im Prompt
aufgerufenen `reta.Program(...)`-Läufe weitergegeben.

## Geänderte Dateien

- `reta_architecture/parallel_execution.py` neu
- `reta_architecture/table_preparation.py`
  - `prepare_output_table()` verarbeitet Header seriell und Datenzeilen optional
    prozessparallel
  - Snapshot enthält jetzt `parallel_row_morphism`
- `reta_architecture/program_workflow.py`
  - Religion-CSV-Zellen werden optional in Prozess-Chunks dekodiert
- `reta_architecture/combi_join.py`
  - Kombi-CSV-Zellen und Kombi-Join-Teiltabellen werden optional in
    Prozess-Chunks vorbereitet
- `reta_architecture/table_output.py`
  - Spaltenprojektion und maximale Zellbreiten-Scans werden optional in
    Prozess-Chunks vorbereitet
- `reta_architecture/facade.py`
  - `ParallelExecutionBundle` ist Teil der Architektur
- `reta_architecture/__init__.py`
  - Exporte für Parallelisierung ergänzt
- `reta_architecture/program_workflow.py`
  - Workflow-Snapshot nennt die optionalen Prozess-Chunk-Stufen
- `reta_architecture/package_integrity.py`
  - neuer Pflichtpfad `reta_architecture/parallel_execution.py`
- `reta.py`
  - strippt Parallel-Flags aus `argv`
  - setzt `parallel_config` auf Program/Tables/Prepare
- `retaPrompt.py`
  - strippt Parallel-Flags beim Prompt-Start
  - propagiert die Config über Umgebungsvariablen an Prompt-ausgelöste
    Reta-Läufe
- `tests/test_architecture_refactor.py`
  - Architektur- und Funktionsregressionen für die neue Schicht

## Validierung

Ausgeführt mit `/usr/bin/python3`:

```bash
/usr/bin/python3 -m compileall -q reta.py retaPrompt.py reta_architecture tests
/usr/bin/python3 -m unittest tests.test_architecture_refactor -v
/usr/bin/python3 -m unittest tests.test_command_parity -v
```

Ergebnis:

- `tests.test_architecture_refactor`: **68 Tests OK**
- `tests.test_command_parity`: **1 Test OK**
- zusätzlicher CLI-Vergleich: serieller Lauf und erzwungener Prozesslauf mit
  `--parallel=processes --parallel-workers=2 --parallel-chunk-size=16
  --parallel-threshold=16` erzeugen identische Ausgabe für den geprüften
  Religionsspalten-Fall.

## Erwartete Performance

Das wird nur schneller, wenn genug nichttriviale Zeilen/Zellen zu berechnen
sind. Bei kleinen Tabellen ist der Prozess-Overhead größer als der Gewinn.
Darum gibt es `threshold` und `chunk_size`.

Für echte Messungen ist die sinnvolle Matrix:

```text
pypy3 reta.py ...                         # PyPy-JIT, auto-parallel ab Threshold
pypy3 reta.py --no-parallel ...           # PyPy-JIT, seriell
python3 reta.py ...                       # CPython, seriell
python3 reta.py --parallel=processes ...  # CPython, erzwungene Prozesse
```
