# Architecture Refactor Stage 41

Stage 41 is the fifth real activation after the long categorical control stack.
The previous activations were:

- Stage 37: row-range morphisms from `libs/center.py`
- Stage 38: arithmetic morphisms from `libs/center.py`
- Stage 39: console/help/utility morphisms from `libs/center.py`
- Stage 40: word-completion morphisms from `libs/word_completerAlx.py`

Stage 41 activates the hierarchical completion state machine from
`libs/nestedAlx.py`.

## New owner

```text
libs/nestedAlx.py
  → reta_architecture/completion_nested.py
  → NestedCompletionMorphismBundle
```

The legacy module keeps compatibility names but no longer owns the real
transition logic.

## Paradigm mapping

| Reta piece | New architecture role | Mathematical reading |
|---|---|---|
| prompt text before cursor | local input section | Prägarbe |
| completion situation | open set in prompt topology | Topologie |
| `matchTextAlx` | situation-selection morphism | Morphismus |
| `paraZeilen`, `paraSpalten`, `paraAusgabe`, `paraKombination` | local option-section builders | Prägarben-Sektionen |
| `gleichKomma*` value expansion | gluing of runtime vocabulary and i18n value sections | universelle Eigenschaft |
| `get_completions` | output-candidate morphism | Morphismus |
| `nestedAlx.NestedCompleter` facade | old path to new owner | natürliche Transformation |

## Contract

```text
nestedAlx.NestedCompleter
    ↓ LegacyNestedCompleterCompatibilityFunctor
NestedCompletionMorphismBundle
    ↓ NestedCompletionPromptFunctor
NestedCompletionCandidateSection
```

must agree with the direct architecture path through
`ArchitectureNestedCompleter`.
