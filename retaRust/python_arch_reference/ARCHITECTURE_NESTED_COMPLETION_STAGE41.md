# Stage 41 Nested Completion Architecture

`reta_architecture/completion_nested.py` owns the hierarchical prompt
completion logic previously implemented directly in `libs/nestedAlx.py`.

## Capsule

```text
InputPromptCapsule
└─ NestedCompletionMorphismBundle
   ├─ ArchitectureNestedCompleter
   ├─ ComplSitua
   ├─ NestedCompletionRuntimeView
   └─ prompt-toolkit Completion output
```

## Legacy facade

```text
libs/nestedAlx.py
└─ imports ArchitectureNestedCompleter as NestedCompleter
```

This preserves the historical prompt imports:

```python
from nestedAlx import ComplSitua, NestedCompleter
```

## Activated category

```text
ActivatedNestedCompletionCategory
├─ NestedCompletionMorphismBundle
├─ NestedCompletionOpenSet
├─ NestedOptionSection
└─ NestedCompletionCandidateSection
```

## Functors and natural transformations

```text
NestedCompletionActivationFunctor
LegacyNestedCompleterCompatibilityFunctor
NestedCompletionPromptFunctor
NestedCompletionValidationFunctor

NestedCompleterToArchitectureTransformation
NestedCompletionValidationTransformation
```

## Probe

```bash
python -B -S reta_architecture_probe_py.py nested-completion-json
```
