# Reta Stage 31 Architecture Validation

This file is the human-readable companion to `reta_architecture/architecture_validation.py`.

## Validation stack

```text
ArchitectureValidationBundle
├─ CategoryTheoryBundle checks
│  ├─ category references
│  ├─ functor references
│  ├─ natural transformation references
│  └─ required paradigm terms
├─ ArchitectureMapBundle checks
│  ├─ stage marker
│  ├─ flow capsule references
│  └─ containment references
├─ ArchitectureContractsBundle checks
│  ├─ validation status
│  └─ expected diagrams/laws
├─ ArchitectureWitnessBundle checks
│  ├─ validation status
│  ├─ diagram witness coverage
│  └─ naturality witness coverage
└─ Repository checks
   ├─ package integrity
   └─ Stage-31 Markdown history
```

## Why this exists

Stage 29 says what must commute. Stage 30 says where it is witnessed. Stage 31 asks whether these layers still agree after the next refactor step.

## How to inspect it

```bash
python -B -S reta_architecture_probe_py.py architecture-validation-json
python -B -S reta_architecture_probe_py.py architecture-validation-md
```

## Validation rule

A later stage should not move semantics across capsule boundaries while this validation layer reports missing categories, broken functor references, missing naturality witnesses, missing required source files or missing Stage documentation.
