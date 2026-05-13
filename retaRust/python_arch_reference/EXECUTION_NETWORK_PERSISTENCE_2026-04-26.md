# Execution Network und Persistenz/Audit für `reta`

Stand: 2026-04-26

## Ziel

Die neue Schicht setzt die Vorgabe um:

```text
Baue Netzwerkmechanik für Ausführung.
Baue Datenbankmechanik für Persistenz und Audit.
Halte Topologien, Garben, Morphismen, Funktoren und natürliche Transformationen rein.
```

Die mathematischen Kernmodule wurden nicht mit Queue-, Stack-, Semaphore- oder Datenbanklogik vermischt. Stattdessen gibt es zwei neue, eigene Architekturmodule.

## Neue Runtime-Kapsel: `reta_architecture/execution_network.py`

Enthält:

```text
ExecutionNetworkConfig
ExecutionTask
ExecutionResult
ExecutionRunResult
FifoTaskQueue
LifoTaskStack
PriorityTaskQueue
ResourceSemaphore
HalfDuplexChannel
FullDuplexChannel
ExecutionNetworkBundle
execute_tasks_deterministically
deterministic_reduce
```

Die zentrale universelle Eigenschaft lautet:

```text
parallel oder seriell ausgeführte lokale Tasks
        ↓ deterministische Reduktion nach Originalindex
dasselbe geordnete globale Resultat
```

Damit wird die vorhandene PyPy3-taugliche Prozessparallelisierung nicht semantisch verändert, sondern sauberer gekapselt. `parallel_execution.py` verwendet die neue Execution-Network-Schicht für generische Prozess-Chunk-Maps.

## Neue Persistenz-/Audit-Kapsel: `reta_architecture/persistence.py`

Enthält eine SQLite-Schicht für:

```text
open_contexts
local_sections
sheaf_snapshots
execution_runs
audit_events
cache_entries
```

Wichtige Funktionen:

```text
persist_section / load_section
persist_sheaf_snapshot / load_sheaf_snapshot
persist_execution_run
record_audit_event / query_audit_events
cache_put / cache_get / invalidate_cache
stable_digest
```

Die zentrale Bedingung lautet:

```text
load(persist(section)) == section
```

sofern der gespeicherte Hash zum Kontext und Payload passt.

## Neue Kategorien

Die Kategorie-Theorie-Schicht enthält jetzt zusätzlich:

```text
ExecutionNetworkCategory
SchedulerCategory
ChannelCategory
PersistenceCategory
```

Aktuelle Zählung:

```text
26 Kategorien
72 Funktoren
37 natürliche Transformationen
```

## Neue wichtige Funktoren

```text
TableChunkExecutionFunctor
ExecutionResultGluingFunctor
SchedulerResourceFunctor
ChannelPromptFunctor
PresheafPersistenceFunctor
SheafPersistenceFunctor
TableStatePersistenceFunctor
AuditValidationPersistenceFunctor
CacheMaterializationFunctor
PersistenceAuditFunctor
```

## Neue wichtige natürliche Transformationen

```text
ParallelExecutionNaturalityTransformation
SchedulerExecutionNaturalityTransformation
ChannelPromptNaturalityTransformation
PresheafPersistenceRoundTripTransformation
SheafPersistenceRoundTripTransformation
TableStatePersistenceTransformation
CacheCoherenceTransformation
AuditPersistenceValidationTransformation
```

## Architekturgrenze

Nicht geändert wurden die reinen Bedeutungsschichten:

```text
reta_architecture/topology.py
reta_architecture/presheaves.py
reta_architecture/sheaves.py
reta_architecture/morphisms.py
reta_architecture/universal.py
```

Dort gibt es weiterhin keine Queue-/DB-/Semaphore-Mechanik.

## Bedienung der Persistenz

Standardmäßig nutzt die Persistenzschicht `:memory:` und schreibt keine Datei. Für eine echte Audit-Datenbank kann gesetzt werden:

```bash
export RETA_PERSISTENCE_DB=/pfad/zu/reta_audit.sqlite
```

oder programmatisch:

```python
from reta_architecture.persistence import bootstrap_persistence, PersistenceConfig

persistence = bootstrap_persistence(config=PersistenceConfig(db_path="reta_audit.sqlite"))
with persistence.connect() as con:
    ...
```

## Validierung

Geprüft wurden Syntax/Import der neuen Schichten, Kategorie-Theorie-Zählung, Prozess-Chunk-Nutzung über `parallel_execution.py` und SQLite-Roundtrips für lokale Sektionen, Garben-Snapshots, Audit-Events und Cache-Einträge.
