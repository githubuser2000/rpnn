# Stage 32 Architecture Traces

`reta_architecture/architecture_traces.py` introduces the Stage-32 trace layer.

The trace layer answers a practical refactor question:

```text
Given an old reta component, where does it live in the new architecture,
and through which mathematical contracts is it protected?
```

## Trace route

A typical route is:

```text
reta component
  → ArchitectureCapsuleSpec
  → CategorySpec / FunctorSpec / NaturalTransformationSpec
  → CommutativeDiagramSpec
  → RefactorLawSpec
  → AnchorWitnessSpec
  → Validation / Coherence check
```

## Main dataclasses

```python
TraceHopSpec
RetaComponentTraceSpec
CapsuleTraceSpec
StageHistoryTraceSpec
TraceValidationSpec
Stage32ArchitecturePlan
ArchitectureTraceBundle
```

## Examples of traced components

```text
i18n/words.py
csv/*.csv
reta.py
retaPrompt.py
libs/center.py
libs/LibRetaPrompt.py
libs/tableHandling.py
libs/lib4tables_prepare.py
libs/lib4tables_concat.py
reta_architecture/category_theory.py
reta_architecture/architecture_map.py
reta_architecture/architecture_contracts.py
reta_architecture/architecture_witnesses.py
reta_architecture/architecture_validation.py
reta_architecture/architecture_coherence.py
```

## New probes

```bash
python -B -S reta_architecture_probe_py.py architecture-traces-json
python -B -S reta_architecture_probe_py.py architecture-traces-md
```

The trace layer is metadata-only. It does not change the observed reta runtime behaviour.
