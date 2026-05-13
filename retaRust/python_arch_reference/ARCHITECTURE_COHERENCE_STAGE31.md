# Reta Stage 31 Architecture Coherence

This file is the human-readable companion to `reta_architecture/architecture_coherence.py`.

## Coherence stack

```text
ArchitectureCoherenceBundle
├─ capsule_coherence_matrix
│  └─ capsule → category → functor / natural transformation → diagram → law → witness
├─ functorial_route_matrix
│  └─ architecture-map flow → categorical kind → contract diagram → witness diagram
├─ naturality_coherence_matrix
│  └─ natural transformation → source/target functor → diagram → capsule witness
└─ law_coherence_matrix
   └─ refactor law → protected capsules → witness obligation
```

## Why this exists

The architecture had become well documented, but documentation alone is too soft for the next stages. The coherence bundle gives a single matrix for asking:

```text
What changes if I move this piece of reta?
```

The answer should identify the capsule, mathematical category, functor or natural transformation, protected diagram, law and witness.

## How to inspect it

```bash
python -B -S reta_architecture_probe_py.py architecture-coherence-json
python -B -S reta_architecture_probe_py.py architecture-coherence-md
```

## Reading rule

The old reta files are compatibility entrances. The new architecture capsules own the semantics. The coherence matrix is the cross-reference that keeps this split honest.
