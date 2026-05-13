# Stage 35 Rehearsal Architecture

Die Rehearsal-Schicht ist die Trockenlauf-Schicht über dem Stage-34-Migrationsplan.

```text
ArchitectureRehearsalBundle
├─ RehearsalOpenSetSpec
├─ RehearsalMoveSpec
├─ GateRehearsalSpec
├─ RehearsalCoverSpec
└─ RehearsalValidationSpec
```

## Topologie

Jede Stage-34-Migrationswelle `M0` bis `M6` wird als Rehearsal-Open-Set gelesen. Dieses Open Set enthält die betroffenen Kapseln und geplanten Kandidaten.

## Morphismus

Ein `RehearsalMoveSpec` ist der trockenlaufende Refactor-Morphismus eines geplanten MigrationStep. Er hat Quelle, Ziel, Vorbedingungen, Nachbedingungen, Gate-Anforderungen und Rollback-Anker.

## Universelle Eigenschaft, Prägarbe und Garbe

Die Gate-Rehearsals sind lokale Prüfsektionen. Ein `RehearsalCoverSpec` klebt sie zu einer globalen Readiness-Garbe über der Migrationswelle.

## Funktoren und natürliche Transformationen

`MigrationStepRehearsalFunctor`, `MigrationGateRehearsalFunctor`, `RehearsalCoverFunctor` und `RehearsalGateValidationFunctor` verbinden Migration, Gate-Binding, Trockenlauf und Validierung. Die natürlichen Transformationen `MigrationRehearsalNaturalityTransformation` und `RehearsalReadinessValidationTransformation` sichern die kommutierenden Pfade.
