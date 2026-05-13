# Stage 36 Changes – Activation / Commit / Rollback Layer

Stage 36 builds directly on Stage 35.

Stage 35 modeled every planned migration as a rehearsal open set with dry-run
moves, gate suites and readiness covers. Stage 36 does **not** execute these
moves. It adds a controlled activation envelope above them:

```text
RehearsalOpenSet
  -> ActivationWindow
RehearsalMove
  -> ActivationUnit
GateRehearsal
  -> ActivationGate
ActivationGate
  -> ActivationRollback
ActivationWindow + ActivationUnit + Gate + Rollback
  -> ActivationTransaction
```

The stage remains metadata-only. It makes future runtime moves commit-gated and
rollback-aware before any later stage actually moves behavior.

## New source file

```text
reta_architecture/architecture_activation.py
```

It defines:

- `ActivationWindowSpec`
- `ActivationUnitSpec`
- `ActivationGateSpec`
- `ActivationRollbackSpec`
- `ActivationTransactionSpec`
- `ActivationCheckSpec`
- `ActivationValidationSpec`
- `Stage36ArchitecturePlan`
- `ArchitectureActivationBundle`
- `bootstrap_architecture_activation(...)`

## New probes

```bash
python -B -S reta_architecture_probe_py.py architecture-activation-json
python -B -S reta_architecture_probe_py.py architecture-activation-md
```

## New categorical vocabulary

Stage 36 adds:

```text
ArchitectureActivationCategory
RehearsalActivationFunctor
GateActivationFunctor
ActivationTransactionFunctor
ActivationRollbackFunctor
ActivationValidationFunctor
ActivationCoherenceFunctor
RehearsalActivationNaturalityTransformation
ActivationRollbackValidationTransformation
```

## New contracts

Stage 36 adds these commutative diagrams:

```text
RehearsalActivationSquare
ActivationRollbackValidationSquare
```

and this law:

```text
ArchitectureActivationCommitLaw
```

## Runtime behavior

No CLI, prompt, table, CSV or output behavior is intentionally changed in Stage
36. The stage only records activation/commit/rollback structure for later
runtime refactors.
