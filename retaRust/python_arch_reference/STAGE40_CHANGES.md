# Stage 40 Changes – Activated Word-Completion Morphisms

Stage 40 is built directly on Stage 39.  Stage 37 activated the row-range
morphisms from `libs/center.py`; Stage 38 activated arithmetic morphisms; Stage
39 activated console/help/wrapping/utility morphisms.  Stage 40 performs the next
small runtime activation by moving the concrete word-completion matching logic
out of `libs/word_completerAlx.py` and into the architecture package.

## New owner

```text
libs/word_completerAlx.py
  legacy WordCompleter implementation
      ↓
reta_architecture/completion_word.py
  WordCompletionMorphismBundle
  ArchitectureWordCompleter
```

The old import surface remains valid:

```python
from word_completerAlx import WordCompleter
```

but the implementation now lives in:

```python
from reta_architecture.completion_word import ArchitectureWordCompleter
```

## Paradigm mapping

```text
word list / callable words       → local completion section / presheaf section
prompt document before cursor    → cursor-prefix open set / topology
prefix and middle matching       → morphism
Completion objects               → local completion-output section
WordCompleter facade             → natural transformation from legacy to architecture
WordCompletionMorphismBundle     → activated architecture owner
```

## New probe

```bash
python -B -S reta_architecture_probe_py.py word-completion-json
```

## Behaviour

Stage 40 is a controlled activation.  It preserves the old `WordCompleter` API
and is intended to keep prompt completion behaviour unchanged.
