# Stage 34 Architecture Migration Bundle

`reta_architecture/architecture_migration.py` ist die Stage-34-Migrationsplanung über der Stage-33-Impact-Schicht.

## Kapselposition

```text
CategoricalMetaCapsule
└─ ArchitectureMigrationBundle
   ├─ MigrationWaveSpec
   ├─ MigrationStepSpec
   ├─ MigrationGateBindingSpec
   ├─ MigrationInvariantSpec
   ├─ MigrationValidationSpec
   └─ Stage34ArchitecturePlan
```

## Datenfluss

```text
ArchitectureImpactBundle
  -> ImpactSourceSpec
  -> MigrationCandidateSpec
  -> ArchitectureMigrationBundle
  -> MigrationWaveSpec
  -> MigrationGateBindingSpec
  -> MigrationInvariantSpec
```

## Kommutierende Diagramme

```text
ImpactMigrationPlanningSquare
MigrationGateCoherenceSquare
```

Diese Diagramme binden die Migrationsplanung an die bestehenden Stage-29/33-Verträge zurück.

## Refactor-Gesetz

```text
ArchitectureMigrationOrderingLaw
```

Das Gesetz bedeutet: Eine alte reta-Komponente darf später nicht beliebig verschoben werden. Sie braucht einen sichtbaren Migrationsschritt, eine Welle, Gates, Diagramme, Gesetze und natürliche Transformationen.

## Probe

```bash
python -B -S reta_architecture_probe_py.py architecture-migration-json
```

Die Probe liefert Counts, Wellen, Schritte, Gate-Bindings, Invarianten und Validierungsstatus.
