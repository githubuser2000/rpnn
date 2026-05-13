# Reta Topology / Morphisms / Universal Properties / Presheaves / Sheaves / Categories / Functors / Natural Transformations Refactor – Stage 30

Stage 30 is the witness stage. It builds on the Stage-29 architecture contracts and turns symbolic contracts into a concrete repository witness matrix.

## What Stage 30 adds

```text
CategoryTheoryBundle
    + ArchitectureMapBundle
    + ArchitectureContractsBundle
    + repository tree
        ↓
ArchitectureWitnessBundle
```

The new layer records where a mathematical architecture claim is represented in actual reta code, tests or probes.

## New bundle

```python
ArchitectureWitnessBundle
├─ anchor_witnesses
├─ capsule_slices
├─ diagram_witnesses
├─ naturality_witnesses
├─ obligations
├─ validation
└─ plan
```

The most important new idea is the capsule slice:

```text
legacy reta owner
    → new architecture capsule
        → mathematical role
            → protected contract
                → concrete witness anchors
```

## Main files

- `reta_architecture/architecture_witnesses.py`
- `reta_architecture/facade.py`
- `reta_architecture/architecture_map.py`
- `reta_architecture_probe_py.py`
- `tests/test_architecture_refactor.py`

## New probe commands

```bash
python -B -S reta_architecture_probe_py.py architecture-witnesses-json
python -B -S reta_architecture_probe_py.py architecture-witnesses-md
```

## Compatibility promise

Stage 30 intentionally makes no user-visible runtime change. It only adds inspection, traceability and validation metadata. The CLI, Prompt, table generation and output paths remain protected by the existing parity tests.

## Practical rule for future stages

A later extraction should now be justified in this order:

```text
1. Which capsule owns the semantics?
2. Which category/functor/natural transformation describes the move?
3. Which Stage-29 diagram or law protects it?
4. Which Stage-30 witness proves where it lives in the repository?
```
