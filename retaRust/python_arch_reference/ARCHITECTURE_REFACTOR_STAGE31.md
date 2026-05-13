# Reta Topology / Morphisms / Universal Properties / Presheaves / Sheaves / Categories / Functors / Natural Transformations Refactor – Stage 31

Stage 31 is the validation/coherence stage. It builds on the Stage-30 witness matrix and turns the full categorical architecture stack into an executable audit plus a readable coherence matrix.

## What Stage 31 adds

```text
CategoryTheoryBundle
ArchitectureMapBundle
ArchitectureContractsBundle
ArchitectureWitnessBundle
RepoManifest
    ↓
ArchitectureValidationBundle
    ↓
ArchitectureCoherenceBundle
```

The purpose is not to add a new runtime abstraction. The purpose is to make the existing Topologie-/Garben-/Kategorien-architecture internally checkable.

## New bundles

```python
ArchitectureValidationBundle
├─ checks
├─ layers
├─ summary
├─ text_diagram
├─ mermaid_diagram
└─ plan
```

```python
ArchitectureCoherenceBundle
├─ capsule_coherence
├─ functorial_routes
├─ naturality_coherence
├─ law_coherence
├─ validation
├─ text_diagram
├─ mermaid_diagram
└─ plan
```

## Main files

- `reta_architecture/architecture_validation.py`
- `reta_architecture/architecture_coherence.py`
- `reta_architecture/category_theory.py`
- `reta_architecture/architecture_map.py`
- `reta_architecture/architecture_contracts.py`
- `reta_architecture/architecture_witnesses.py`
- `reta_architecture/facade.py`
- `reta_architecture_probe_py.py`
- `tests/test_architecture_refactor.py`

## New probe commands

```bash
python -B -S reta_architecture_probe_py.py architecture-validation-json
python -B -S reta_architecture_probe_py.py architecture-validation-md
python -B -S reta_architecture_probe_py.py architecture-coherence-json
python -B -S reta_architecture_probe_py.py architecture-coherence-md
```

## Compatibility promise

Stage 31 intentionally makes no user-visible runtime change. CLI, Prompt, table generation and output behaviour remain protected by the existing regression and parity tests.

## Practical rule for future stages

A later extraction should now be justified in this order:

```text
1. Which capsule owns the semantics?
2. Which category/functor/natural transformation describes the move?
3. Which Stage-29 diagram or law protects it?
4. Which Stage-30 witness proves where it lives?
5. Which Stage-31 validation/coherence checks stay green?
```
