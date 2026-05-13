# Stage 36 Activation Architecture

`ArchitectureActivationBundle` is the new Stage-36 bundle. It turns Stage-35
rehearsal artifacts into activation artifacts without executing them.

## Objects

```text
ActivationWindowSpec
  Topological window derived from a Stage-35 rehearsal open set.

ActivationUnitSpec
  Morphism-like activation envelope derived from one rehearsed migration move.

ActivationGateSpec
  Local presheaf-like gate section containing preflight, commit, postflight and
  rollback commands.

ActivationRollbackSpec
  Rollback section protecting the diagrams and laws attached to an activation.

ActivationTransactionSpec
  Universal gluing of all activation units in one migration wave.

ActivationValidationSpec
  Checks that windows, units, gates, rollbacks, transactions, diagrams, laws and
  natural transformations fit together.
```

## Functorial reading

```text
ArchitectureRehearsalCategory
  --RehearsalActivationFunctor-->
ArchitectureActivationCategory
```

Gate sections are transported through:

```text
GateActivationFunctor
ActivationRollbackFunctor
ActivationTransactionFunctor
ActivationValidationFunctor
ActivationCoherenceFunctor
```

The relevant natural transformations are:

```text
RehearsalActivationNaturalityTransformation
ActivationRollbackValidationTransformation
```

## Commutative diagrams

```text
RehearsalActivationSquare
```

says that taking a rehearsed move to an activation unit is compatible with the
Stage-35 gate/readiness cover.

```text
ActivationRollbackValidationSquare
```

says that activation validation and rollback validation commute: a future commit
is acceptable only if its rollback section protects the same diagrams and laws.

## Validation result expected by the probe

The Stage-36 probe should report:

```text
stage: 36
windows: 7
units: 23
gates: 23
rollbacks: 23
transactions: 7
validation: passed
```

## Runtime behavior

Stage 36 is intentionally a metadata-only commit envelope. It does not change
runtime behavior.
