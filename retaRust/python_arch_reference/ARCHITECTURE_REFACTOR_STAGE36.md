# Architecture Refactor Stage 36

Stage 36 continues the reta architecture refactor under the agreed paradigm:

```text
Topologie
Morphismus
universelle Eigenschaft
Prägarben
Garben
math Kategorie
Funktor
natürliche Transformation
```

After Stage 35 the architecture had a rehearsal/readiness layer. Those rehearsal
moves were still dry runs. Stage 36 adds the next missing capsule in the
categorical meta layer: a guarded activation layer.

## Gesamtarchitektur nach Stage 36

```text
RetaArchitectureRoot
├─ SchemaTopologyCapsule
├─ LocalSectionCapsule
├─ SemanticSheafCapsule
├─ InputPromptCapsule
├─ WorkflowGluingCapsule
├─ TableCoreCapsule
├─ GeneratedRelationCapsule
├─ OutputRenderingCapsule
├─ CompatibilityCapsule
└─ CategoricalMetaCapsule
   ├─ CategoryTheoryBundle
   ├─ ArchitectureMapBundle
   ├─ ArchitectureContractsBundle
   ├─ ArchitectureWitnessBundle
   ├─ ArchitectureValidationBundle
   ├─ ArchitectureCoherenceBundle
   ├─ ArchitectureTraceBundle
   ├─ ArchitectureBoundariesBundle
   ├─ ArchitectureImpactBundle
   ├─ ArchitectureMigrationBundle
   ├─ ArchitectureRehearsalBundle
   └─ ArchitectureActivationBundle
```

## Stage-36-Lesart

```text
ImpactSource
  -> MigrationCandidate
  -> MigrationWave
  -> MigrationStep
  -> MigrationGateBinding
  -> RehearsalOpenSet
  -> RehearsalMove
  -> GateRehearsal
  -> RehearsalCover
  -> ActivationWindow
  -> ActivationUnit
  -> ActivationGate
  -> ActivationRollback
  -> ActivationTransaction
  -> ActivationValidation
```

The activation layer is the first explicit commit envelope. It still does not
move code, but it records which rehearsal moves would become a future activation,
which gates must pass, what rollback anchor protects the move, and how local
activation units glue into a transaction.

## What of reta becomes what

| reta part | Stage-36 role | Mathematical reading |
|---|---|---|
| `i18n/words.py` | activation unit in topology/data waves | topological basis / semantic index |
| `csv/*.csv` | activation unit in local-section wave | presheaf local section |
| `retaPrompt.py` | activation unit in prompt wave | raw-command morphism |
| `reta.py` | activation unit in workflow/compatibility wave | universal gluing + legacy naturality |
| `libs/tableHandling.py` | activation unit in table-core/generated/output waves | global table section, endofunctors, renderer functors |
| `libs/lib4tables_concat.py` | activation unit in generated-relation wave | generated-column endofunctors |
| `reta_architecture/architecture_activation.py` | meta activation capsule | commit/rollback category |

## Rule for future stages

A future runtime extraction should not directly move behavior. It must first be
represented as:

```text
rehearsed move -> activation unit -> commit gate -> rollback section -> transaction
```

and the corresponding diagrams and natural transformations must remain valid.
