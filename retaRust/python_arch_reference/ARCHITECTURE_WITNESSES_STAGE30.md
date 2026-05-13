# Reta Stage 30 Architecture Witnesses

This file is the human-readable companion to `reta_architecture/architecture_witnesses.py`.

## Witness stack

```text
ArchitectureWitnessBundle
├─ Anchor witnesses
│  └─ resolve file/glob/symbolic owners against the repository tree
├─ Capsule slices
│  └─ old reta owner → new capsule → math role → protected contract
├─ Diagram witnesses
│  └─ Stage-29 commutative diagrams with implementation anchors and probe commands
├─ Naturality witnesses
│  └─ natural transformations tied to diagrams and concrete capsules
└─ Refactor obligations
   └─ laws and diagrams that future stages must preserve
```

## Why this exists

Stage 29 answered what must commute. Stage 30 answers where that commutation is witnessed.

The guiding contract is:

```text
A future extraction is valid only if its old owner, new capsule, mathematical role,
contract and witness remain traceable.
```

## How to inspect it

```bash
python -B -S reta_architecture_probe_py.py architecture-witnesses-json
python -B -S reta_architecture_probe_py.py architecture-witnesses-md
```

## Concrete reading

`reta.py`, `retaPrompt.py` and `libs/*` are now mostly compatibility entrances and witnesses. The new semantic owners live in `reta_architecture/*` capsules. Stage 30 makes that relation explicit instead of relying on memory from the stage history.
