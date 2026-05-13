# Dritte Prozessparallelisierung für PyPy3

Stand: 2026-04-27

## Ziel

Mehr Prozessparallelisierung für PyPy3, aber nur dort, wo sie deterministisch, grob chunkbar und ohne gemeinsame Mutation bleibt.

Die mathematischen Kernschichten bleiben rein:

```text
reta_architecture/topology.py
reta_architecture/presheaves.py
reta_architecture/sheaves.py
reta_architecture/morphisms.py
reta_architecture/universal.py
```

In diese Dateien wurde keine Queue-, Thread-, Prozesspool-, Semaphore-, Channel- oder Datenbanklogik eingemischt. `universal.py` nutzt nur einen optionalen Prozesspfad für die reine Spaltenbucket-Normalisierung und fällt deterministisch auf den seriellen Pfad zurück.

## Neu prozessparallelisiert

### 1. Zeilen-Zähl- und Moon-Scans

Dateien:

```text
reta_architecture/parallel_execution.py
reta_architecture/row_filtering.py
```

Neue Funktion:

```text
moon_numbers_in_processes()
```

Verwendung:

```text
set_zaehlungen()
```

Nur die reine `moonNumber(number)`-Berechnung läuft in Prozess-Chunks. Die fortlaufende Zählzustandsmutation bleibt seriell.

Universelle Eigenschaft:

```text
parallel berechnete Moon-Paare
        ↓ seriell in Zahlenreihenfolge reduziert
identischer Zählzustand wie seriell
```

### 2. Prime-Factor-Batches für Innen/Außen-Zeilenlogik

Dateien:

```text
reta_architecture/parallel_execution.py
reta_architecture/row_filtering.py
```

Neue Funktion:

```text
prime_factors_in_processes()
```

Verwendung:

```text
inside/outside row filtering
```

Nur die Primfaktorlisten pro Zahl werden parallel berechnet. Die spätere Semantikentscheidung bleibt seriell.

### 3. Zahlenfilter über Zeilenmengen

Dateien:

```text
reta_architecture/parallel_execution.py
reta_architecture/row_filtering.py
```

Neue Funktion:

```text
filter_numbers_in_processes()
```

Unterstützte Modi:

```text
sonne_mit_mondanteil
prime_multiples
ordinary_multiples
modulo
moon
```

Verwendet für:

```text
SonneMitMondanteil
Primzahlvielfache
gewöhnliche Vielfache
```

Die Ergebnismenge wird als Set zurückgegeben und anschließend im Hauptprozess in den bestehenden Tabellenzustand eingebunden.

### 4. Arithmetik-/Teilerbatches

Dateien:

```text
reta_architecture/parallel_execution.py
reta_architecture/arithmetic.py
```

Neue Funktion:

```text
factor_pairs_in_processes()
```

Verwendung:

```text
divisor_range()
```

Die Faktorpaare pro Zahl werden in Prozess-Chunks berechnet. Die Ausgabeform des Legacy-Pfads bleibt seriell und unverändert zusammengesetzt.

### 5. Spaltenbucket-Normalisierung

Dateien:

```text
reta_architecture/parallel_execution.py
reta_architecture/universal.py
```

Neue Funktion:

```text
normalize_column_buckets_in_processes()
```

Diese Funktion normalisiert positive/negative Spaltenbucket-Paare in Prozess-Chunks, wenn die Payload groß genug ist. Die Endmontage bleibt seriell.

Universelle Eigenschaft:

```text
parallel normalisierte Bucket-Paare
        ↓
dieselbe Normalform wie normalize_column_buckets() seriell
```

### 6. Package-/Manifest-Hashing

Dateien:

```text
reta_architecture/package_integrity.py
```

Neu:

```text
_manifest_entries_parallel()
_manifest_file_worker()
```

`RepoManifest.from_tree()` sammelt die Dateiliste weiter seriell, kann aber Lesen, Hashing und CSV-Zeilenzählen in Prozess-Chunks ausführen. Die Manifest-Digest-Montage bleibt seriell und sortiert.

Universelle Eigenschaft:

```text
parallel(file_digest entries) + sorted serial digest assembly
=
serial manifest digest
```

### 7. Persistenz-Batch-Vorbereitung

Dateien:

```text
reta_architecture/persistence.py
```

Neue Funktionen:

```text
persist_sections_batch()
persist_sheaf_snapshots_batch()
cache_put_many()
```

Nur JSON-/Digest-Vorbereitung wird optional in Prozess-Chunks ausgeführt. SQLite-Schreiboperationen bleiben seriell im Hauptprozess.

Universelle Eigenschaft:

```text
parallel vorbereitete Persistenzrecords
        ↓ serieller SQLite-Commit
identische Persistenzsemantik wie Einzelwrites
```

## Kategorie-Theorie erweitert

Neue Zählung:

```text
26 Kategorien
77 Funktoren
42 natürliche Transformationen
```

Vorheriger Stand nach Execution-/Persistence-Schicht:

```text
26 Kategorien
72 Funktoren
37 natürliche Transformationen
```

Neue Funktoren:

```text
RowFilterProcessFunctor
ArithmeticBatchExecutionFunctor
PackageIntegrityExecutionFunctor
PersistenceBatchPreparationFunctor
ProcessExecutionAuditFunctor
```

Neue natürliche Transformationen:

```text
RowFilterProcessNaturalityTransformation
ArithmeticBatchProcessNaturalityTransformation
PackageIntegrityProcessNaturalityTransformation
PersistenceBatchPreparationNaturalityTransformation
ProcessExecutionAuditNaturalityTransformation
```

Zusätzlich wurden Stage-43-Vertragsdiagramme ergänzt:

```text
ExecutionProcessParallelNaturalitySquare
ChannelPromptExecutionNaturalitySquare
PersistenceRoundTripNaturalitySquare
CacheAuditPersistenceNaturalitySquare
```

Damit sind auch die Execution-/Persistence-Transformationen in Vertrags- und Witness-Schicht abgedeckt.

## Bewusst weiterhin seriell

Diese Bereiche bleiben absichtlich nicht prozessparallel:

```text
Header-/Tag-Gluing
finales Rendering
finales Output-Schreiben
SQLite-Writes
globale Tabellenmutation
Prompt-Zustandsmutation
```

Grund:

```text
parallel rechnen, seriell kleben
```

Das schützt Ausgabeparität und verhindert Race Conditions.

## Validierung

Gezielte Checks:

```text
/usr/bin/python3 -m compileall -q reta_architecture tests reta.py retaPrompt.py
```

Architektur-Referenzen:

```text
Kategorien: 26
Funktoren: 77
natürliche Transformationen: 42
Contract validation: passed
Witness validation: passed
Architecture validation: passed, 51/51
```

Prozessparallelitätschecks:

```text
moon_numbers_in_processes == serial moonNumber map
prime_factors_in_processes == serial prime_factors map
filter_numbers_in_processes(prime_multiples) == serial isPrimMultiple filter
filter_numbers_in_processes(ordinary_multiples) == serial divisor filter
factor_pairs_in_processes covers every input number
normalize_column_buckets_in_processes == serial normalize_column_buckets
```

Persistenz-Batch-Checks:

```text
persist_sections_batch: passed
persist_sheaf_snapshots_batch: passed
cache_put_many/cache_get: passed
```

Manifest-Checks:

```text
serial manifest and process manifest produced identical:
file_count
total_bytes
digest prefix
csv_line_counts count
missing_required count
```

Einschränkung:

Ein vollständiger normaler `unittest`-Durchlauf konnte in dieser Container-Sitzung nicht sauber als beendeter Prozess bestätigt werden. Mehrere Tests bzw. Bootstraps erreichten ihre erwarteten Ausgaben, aber der Shell-Prozess wurde anschließend vom Container-Wrapper nicht zuverlässig als beendet zurückgemeldet. Deshalb wurden die neuen Schichten mit gezielten Architektur-, Prozess-, Persistenz- und Manifestchecks validiert.

## Ergebnis

Die dritte Prozessparallelisierung erweitert PyPy3-taugliche Multiprocessing-Pfade auf:

```text
Zeilenzählung
Moon-/Sonnen-Zeilenlogik
Primfaktor-Batches
Vielfachenfilter
Arithmetik-/Teilerbatches
Spaltenbucket-Normalisierung
Repository-Manifest-Hashing
Persistenz-/Cache-Batch-Vorbereitung
```

Die Architekturregel bleibt erhalten:

```text
Prozesse für reine lokale Arbeit.
Serielle Reduktion für globale Wahrheit.
Keine Thread-Parallelisierung.
Keine Vermischung mit Topologien, Prägarben, Garben, Morphismen, Funktoren oder natürlichen Transformationen.
```
