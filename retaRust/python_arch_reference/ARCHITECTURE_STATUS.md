# Reta Architekturstatus (aktuell)
Stand: 2026-04-24

## Kurzfazit

Die mathematische Architektur ist implementiert, aktiviert und aktuell ohne
verbleibende Pflicht-Baustellen im Migrationsplan.

- Kategorien: **26**
- Funktoren: **72**
- natürliche Transformationen: **37**
- Stage-42-Fortschrittsoverlay: **30** beobachtete Surfaces,
  **34** Step-Progress-Einträge, **7** Wellen, **0** Outstanding-Items
- Wellenstatus: **M0–M6 alle `implemented_or_retained`**
- gemischte Owner: **keine mehr**
- Paketintegrität: **keine fehlenden Pflichtpfade**
- Parallelisierung: **prozessbasierte Tabellen-/Zeilen-Chunk-Schicht vorhanden**

## Seit der alten Restliste geschlossen

### 1. `libs/lib4tables_Enum.py`

Der frühere Misch-Owner ist geschlossen:

- das eigentliche Tag-/Enum-Schema lebt in `reta_architecture/tag_schema.py`
- `libs/lib4tables_Enum.py` ist nur noch Legacy-Kompatibilitätsfläche
- die Fortschrittsschicht klassifiziert die Datei nun als
  `extracted_to_compatibility_facade`

### 2. Paritätstest ohne externes Referenzarchiv

`tests/test_command_parity.py` ist nicht mehr auf
`/mnt/data/reta.todel.zip` angewiesen.

Die Suite arbeitet jetzt in dieser Reihenfolge:

1. externes Referenzarchiv verwenden, falls vorhanden
2. sonst per `git archive` aus einem Baseline-Commit ein temporäres
   Vergleichsarchiv erzeugen
3. optionalen Commit über `RETA_PARITY_BASELINE_COMMIT` respektieren

Damit ist der letzte echte Umgebungsblocker aus der alten Restliste im normalen
Repository-Checkout beseitigt.

## Zusätzlich bereinigt: keine Architektur-Rückimporte mehr in die Legacy-Fassaden

Die neue Architektur importiert ihre Runtime-Helfer jetzt nicht mehr aus
`libs/center.py`, `libs/lib4tables_prepare.py` oder
`libs/lib4tables_concat.py`. Stattdessen liegen die letzten Kompatibilitätsnamen
und Adapter innerhalb des Architekturpakets selbst:

- `reta_architecture/runtime_compat.py` bündelt i18n-, Range-, Arithmetik- und
  Console-Helfer architekturintern
- `reta_architecture/table_adapters.py` enthält die dünnen Architektur-Adapter
  für die früheren Klassen `Prepare` und `Concat`
- die betroffenen Architekturmodule (`row_filtering.py`, `prompt_execution.py`,
  `prompt_preparation.py`, `parameter_runtime.py`, `table_output.py`,
  `generated_columns.py`, `meta_columns.py`, `concat_csv.py`, `combi_join.py`,
  `table_wrapping.py`, `table_runtime.py`) greifen nur noch auf Architekturcode
  zu

Praktisch heißt das: Die neue Schicht hängt nicht mehr rückwärts an den alten
Wrapperdateien. Die Legacy-Dateien bleiben nur noch als optionale äußere
Kompatibilitätsoberfläche erhalten.


## Seit 2026-04-24 ergänzt: prozessbasierte Chunk-Parallelisierung

Für PyPy3 wurde eine optionale Multiprocessing-Schicht ergänzt und danach
erweitert. Sie parallelisiert weiterhin nicht die riskanten mutierenden
Spalten-/Tag-Gluing-Stellen, sondern nur deterministische, chunkbare
Tabellenarbeit:

- Religion-CSV-Zellendekodierung
- Kombi-CSV-Zellenvorbereitung
- Datenzeilen-/Zellenvorbereitung nach der Zeilenauswahl
- Spaltenprojektion nach der Tabellenvorbereitung
- maximale Zellbreiten-Scans vor der Ausgabe
- Kombi-Join-Teiltabellenvorbereitung

Neu ist außerdem eine globale Kernzahl-Schicht in
`reta_architecture/parallel_execution.py`: `RETA_PROCESSOR_CORES` hält physische,
virtuelle/logische und verfügbare Kernzahlen; `RETA_PARALLEL_PROCESSOR_CORES`
ist der Default für Worker-Prozesse. `reta.py` und `retaPrompt.py` akzeptieren
weiterhin u. a. `--parallel=processes`, `--parallel-workers=N`,
`--parallel-chunk-size=N`, `--parallel-threshold=N` und `--no-parallel`. Im
Standardmodus ist die Parallelisierung auf PyPy/PyPy3 automatisch ab Threshold
aktiv; auf CPython bleibt sie ohne explizite Aktivierung aus. Details stehen in
`PARALLEL_PROCESSING_2026-04-24.md`.

## Seit 2026-04-26 ergänzt: Ausführungsnetzwerk und Persistenz-/Audit-Schicht

Die mathematischen Kernschichten bleiben rein. Queues, Stacks, Semaphoren,
Halfduplex-/Fullduplex-Kanäle und SQLite-Persistenz wurden als eigene Kapseln
oberhalb der Semantik ergänzt:

- `reta_architecture/execution_network.py` enthält `ExecutionTask`,
  `FifoTaskQueue`, `LifoTaskStack`, `PriorityTaskQueue`, `ResourceSemaphore`,
  `HalfDuplexChannel`, `FullDuplexChannel`, `ExecutionNetworkBundle` und den
  deterministischen Reducer `execute_tasks_deterministically`.
- `reta_architecture/persistence.py` enthält eine SQLite-Schicht für lokale
  Sektionen, Garben-Snapshots, Ausführungsläufe, Audit-Events und Cache-Einträge.
- Die Kategorie-Theorie-Schicht kennt jetzt zusätzlich
  `ExecutionNetworkCategory`, `SchedulerCategory`, `ChannelCategory` und
  `PersistenceCategory`.
- Die neuen Funktoren und natürlichen Transformationen formulieren die zentrale
  Bedingung: parallele oder persistierte Pfade dürfen die semantische Ausgabe
  nicht verändern.

Die bestehende Prozessparallelisierung nutzt die neue Execution-Network-Schicht
für ihre generischen Chunk-Maps. Die reinen Schichten `topology.py`,
`presheaves.py`, `sheaves.py`, `morphisms.py`, `universal.py` bleiben frei von
Queue-/DB-/Semaphore-Mechanik.

## Was jetzt noch bleibt

Nur noch optionale Nacharbeiten, keine Pflichtpunkte des Migrationsplans:

- `libs/center.py` weiter verdünnen
- `reta.py` weiter verdünnen
- `libs/lib4tables_prepare.py` weiter verdünnen
- `libs/lib4tables_concat.py` weiter verdünnen
- zusätzliche Paritätsfälle ergänzen
- optional eine versionierte Fixture zusätzlich zur Git-Baseline pflegen, falls
  Tests einmal in einem exportierten Tree ohne `.git` laufen sollen

Diese Punkte sind Optimierungen, keine strukturellen Architektur-Blocker.

## Validierungslage

Aktuell verifiziert:

- Architektur-Validierung: **51/51 Checks bestanden**
- Kohärenz: **passed**
- Rehearsal: **passed**
- Activation: **passed**
- Fortschrittsoverlay: **passed**
- Paketintegrität: **missing_required = []**

Zusätzlich erfolgreich ausgeführt:

- `/usr/bin/python3 -m compileall -q reta.py retaPrompt.py reta_architecture tests`
- `/usr/bin/python3 -m unittest tests.test_architecture_refactor -v` → **68 Tests OK**
- `/usr/bin/python3 -m unittest tests.test_command_parity -v` → **1 Test OK**
- CLI-Paritätsprobe: serieller Lauf und erzwungener Prozesslauf mit `--parallel=processes --parallel-workers=2 --parallel-chunk-size=16 --parallel-threshold=16` erzeugen identische Ausgabe

## Praktische Lesart

Die ursprüngliche Zielmenge — **Topologie, Morphismus, universelle
Eigenschaft, Prägarbe, Garbe, Kategorie, Funktor, natürliche Transformation** —
ist im Projekt nicht mehr bloß beschrieben, sondern in den aktiven
Architekturschichten verankert. Der Projektzustand ist deshalb nicht mehr
„Umbau läuft“, sondern **„Umbau abgeschlossen; verbleiben nur noch optionale
Nachverdichtungen“**.
