# Stage 32 Architecture Boundaries

`reta_architecture/architecture_boundaries.py` introduces the Stage-32 boundary layer.

The boundary layer answers a concrete package question:

```text
Which Python module belongs to which architecture capsule,
and which imports cross capsule boundaries?
```

## Boundary route

```text
Python module
  → module owner capsule
  → import edge
  → source capsule / target capsule
  → boundary classification
  → boundary check
```

## Main dataclasses

```python
ModuleOwnershipSpec
ImportEdgeSpec
CapsuleImportEdgeSpec
CapsuleBoundarySpec
BoundaryCheckSpec
BoundaryValidationSpec
Stage32BoundaryPlan
ArchitectureBoundariesBundle
```

## Boundary intent

Stage 32 does not forbid cross-capsule imports. Some imports are intentional because the architecture is still compatible with legacy reta.

It does make such edges visible so later stages can separate runtime ownership, compatibility surfaces and categorical meta layers more cleanly.

## New probes

```bash
python -B -S reta_architecture_probe_py.py architecture-boundaries-json
python -B -S reta_architecture_probe_py.py architecture-boundaries-md
```

The boundary layer is metadata-only. It does not change CLI, Prompt, table generation or output behaviour.
