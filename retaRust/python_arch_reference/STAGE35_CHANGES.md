# Stage 35 Changes – Rehearsal- und Readiness-Schicht

Stage 35 baut direkt auf Stage 34 auf. Stage 34 hatte aus der Impact-Schicht einen geordneten Migrationsplan gemacht. Stage 35 führt darüber eine Trockenlauf-Schicht ein: geplante Migrationen werden noch nicht ausgeführt, aber als topologische Open Sets, Refactor-Morphismen, Gate-Suites und Readiness-Covers prüfbar gemacht.

## Neues Modul

- `reta_architecture/architecture_rehearsal.py`

Das Modul enthält `RehearsalOpenSetSpec`, `RehearsalMoveSpec`, `GateRehearsalSpec`, `RehearsalCoverSpec`, `RehearsalCheckSpec`, `RehearsalValidationSpec`, `Stage35ArchitecturePlan`, `ArchitectureRehearsalBundle` und `bootstrap_architecture_rehearsal(...)`.

## Paradigma

Stage 35 erhält die Sprache Topologie, Morphismus, universelle Eigenschaft, Prägarbe, Garbe, math Kategorie, Funktor und natürliche Transformation. Migrationswellen werden zu Open Sets; Migrationsschritte werden zu Refactor-Morphismen; Gate-Suiten sind lokale Prüfsektionen; Rehearsal-Covers kleben diese lokalen Sektionen zu einer globalen Readiness-Garbe.

## Neue Probes

```bash
python -B -S reta_architecture_probe_py.py architecture-rehearsal-json
python -B -S reta_architecture_probe_py.py architecture-rehearsal-md
```

## Verhalten

Stage 35 ist metadata-only. Es werden keine CLI-, Prompt-, Tabellen- oder Output-Pfade fachlich verändert.
