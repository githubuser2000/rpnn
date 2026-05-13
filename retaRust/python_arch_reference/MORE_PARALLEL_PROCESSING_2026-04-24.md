# Zusätzliche PyPy3-Prozessparallelisierung

Stand: 2026-04-24

## Ergebnis

Die bisherige Prozessparallelisierung für die spätere Datenzeilen-/Zellenvorbereitung wurde erweitert. `reta` und `retaPrompt` nutzen jetzt dieselbe zentrale Parallelisierungsschicht in `reta_architecture/parallel_execution.py`.

Neu oder erweitert parallelisierbar sind:

1. Religion-CSV-Zellendekodierung
2. Kombi-CSV-Zellenvorbereitung
3. Datenzeilen-/Zellenvorbereitung nach der Zeilenauswahl
4. Spaltenprojektion nach der Tabellenvorbereitung
5. maximale Zellbreiten-Scans vor der Ausgabe
6. Kombi-Join-Teiltabellenvorbereitung

Absichtlich seriell bleiben die riskanten mutierenden Teile: generierte Spalten, Tag-/Header-Gluing, finale Ausgabe und Stellen, die globalen Tabellenzustand verändern.

## Globale Kernzahl-Schicht

`reta_architecture/parallel_execution.py` enthält jetzt oben eine globale Erkennung der Prozessor-Kernzahlen:

- `RETA_PROCESSOR_CORES`
- `RETA_PHYSICAL_PROCESSOR_CORES`
- `RETA_VIRTUAL_PROCESSOR_CORES`
- `RETA_AVAILABLE_PROCESSOR_CORES`
- `RETA_PARALLEL_PROCESSOR_CORES`

`RETA_PROCESSOR_CORES` hält physische, virtuelle/logische und für den Prozess verfügbare Kerne. Ohne explizites `--parallel-workers=N` verwendet `ParallelExecutionConfig.resolved_workers` `RETA_PARALLEL_PROCESSOR_CORES` als Default.

Die Pools werden zusätzlich auf die Zahl tatsächlicher Chunks gedeckelt. Dadurch kann der Default alle verfügbaren logischen Kerne nutzen, ohne bei kleinen Chunk-Zahlen unnötige leere Prozesse zu starten.

In dieser Container-Umgebung wurden erkannt:

```text
physical = 56
virtual = 56
available = 56
default_workers = 56
```

Auf dem Zielsystem wird diese Erkennung zur Laufzeit neu durchgeführt.

## Aktivierung

PyPy/PyPy3 bleibt im Standardmodus `auto` und aktiviert Prozessparallelisierung ab Threshold. CPython bleibt im Standardmodus seriell, kann aber explizit erzwungen werden.

Beispiele:

```bash
pypy3 reta.py ...
pypy3 reta.py --no-parallel ...
pypy3 reta.py --parallel=processes --parallel-workers=4 --parallel-chunk-size=128 ...
python3 reta.py --parallel=processes --parallel-threshold=1 ...
```

Für `retaPrompt.py` / `rp` gelten dieselben Flags. Die Konfiguration wird beim Prompt-Start gelesen, aus `sys.argv` entfernt und an später ausgelöste `reta`-Läufe weitergereicht.

## Validierung

Ausgeführt mit `/usr/bin/python3`:

```bash
/usr/bin/python3 -m compileall -q reta.py retaPrompt.py reta_architecture tests
/usr/bin/python3 -m unittest tests.test_architecture_refactor -v
/usr/bin/python3 -m unittest tests.test_command_parity -v
```

Ergebnis:

- Architekturtests: 68 Tests OK
- Command-Parität: 1 Test OK
- serieller CLI-Lauf und erzwungener Prozesslauf mit `--parallel=processes --parallel-workers=2 --parallel-chunk-size=16 --parallel-threshold=16` erzeugen identische Ausgabe im geprüften Religionsspalten-Fall

Direkter PyPy3-Test war in dieser Container-Umgebung nicht möglich, weil `pypy3` hier nicht installiert ist. Der Prozesspfad wurde unter CPython explizit erzwungen getestet; PyPy3 nutzt denselben Prozesspfad im Auto-Modus.
