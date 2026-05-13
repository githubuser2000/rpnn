# Reta Stage 29 Architecture Contracts

This file is the human-readable companion to `reta_architecture/architecture_contracts.py`.

## Contract stack

```text
ArchitectureContractsBundle
├─ Commutative diagrams
│  ├─ RawCommandNaturalitySquare
│  ├─ PresheafSheafGluingSquare
│  ├─ UniversalWorkflowTableSquare
│  ├─ GeneratedColumnStateSyncSquare
│  ├─ RuntimeStateProjectionSquare
│  ├─ RenderedOutputParitySquare
│  ├─ LegacyArchitectureCompatibilitySquare
│  └─ ArchitectureMapContractReflectionTriangle
├─ Capsule contracts
│  ├─ RetaArchitectureRoot
│  ├─ SchemaTopologyCapsule
│  ├─ LocalSectionCapsule
│  ├─ SemanticSheafCapsule
│  ├─ InputPromptCapsule
│  ├─ WorkflowGluingCapsule
│  ├─ TableCoreCapsule
│  ├─ GeneratedRelationCapsule
│  ├─ OutputRenderingCapsule
│  ├─ CompatibilityCapsule
│  └─ CategoricalMetaCapsule
└─ Refactor laws
   ├─ topology / presheaf / sheaf laws
   ├─ universal workflow law
   ├─ generated/state sync law
   ├─ output-normalization law
   └─ legacy-compatibility law
```

## Why this exists

Stage 28 answered where the architecture pieces live. Stage 29 answers what must stay true when code moves again.

The guiding contract is:

```text
A future extraction is valid only if the relevant topology/presheaf/sheaf/workflow/state/output/legacy diagram still commutes.
```

## How to inspect it

```bash
python -B -S reta_architecture_probe_py.py architecture-contracts-json
python -B -S reta_architecture_probe_py.py architecture-contracts-md
```
