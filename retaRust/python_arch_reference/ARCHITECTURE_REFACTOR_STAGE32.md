# Reta Topology / Morphisms / Universal Properties / Presheaves / Sheaves / Categories / Functors / Natural Transformations Refactor – Stage 32

Stage 32 is the trace and boundary stage. It builds on the Stage-31 validation/coherence layer and turns the architecture into a navigable map of concrete reta components and concrete module boundaries.

## What Stage 32 adds

```text
CategoryTheoryBundle
ArchitectureMapBundle
ArchitectureContractsBundle
ArchitectureWitnessBundle
ArchitectureValidationBundle
ArchitectureCoherenceBundle
    ↓
ArchitectureTraceBundle
ArchitectureBoundariesBundle
```

The purpose is not to add runtime behaviour. The purpose is to make the existing categorical architecture easier to follow and safer to extend.

## New bundles

```python
ArchitectureTraceBundle
├─ component_traces
├─ capsule_traces
├─ stage_traces
├─ validation
├─ text_diagram
├─ mermaid_diagram
└─ plan
```

```python
ArchitectureBoundariesBundle
├─ module_ownership
├─ import_edges
├─ capsule_edges
├─ capsule_boundaries
├─ checks
├─ validation
├─ text_diagram
├─ mermaid_diagram
└─ plan
```

## Main files

- `reta_architecture/architecture_traces.py`
- `reta_architecture/architecture_boundaries.py`
- `reta_architecture/category_theory.py`
- `reta_architecture/architecture_map.py`
- `reta_architecture/architecture_contracts.py`
- `reta_architecture/architecture_coherence.py`
- `reta_architecture/architecture_validation.py`
- `reta_architecture/facade.py`
- `reta_architecture_probe_py.py`
- `tests/test_architecture_refactor.py`

## New probe commands

```bash
python -B -S reta_architecture_probe_py.py architecture-traces-json
python -B -S reta_architecture_probe_py.py architecture-traces-md
python -B -S reta_architecture_probe_py.py architecture-boundaries-json
python -B -S reta_architecture_probe_py.py architecture-boundaries-md
```

## Compatibility promise

Stage 32 intentionally makes no user-visible runtime change. CLI, Prompt, table generation and output behaviour remain protected by the existing regression and parity tests.

## Practical rule for future stages

A later extraction should now be justified in this order:

```text
1. Which old reta component is touched?
2. Which capsule owns it?
3. Which category/functor/natural transformation describes the move?
4. Which diagram/law/witness protects the move?
5. Which module boundary changes?
6. Which validation/coherence/boundary checks stay green?
```
