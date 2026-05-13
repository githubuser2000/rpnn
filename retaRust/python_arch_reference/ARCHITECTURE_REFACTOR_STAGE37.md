# Architecture Refactor Stage 37

Stage 37 is the first activation after the Stage-36 commit/rollback planning
layer.

## Before Stage 37

`libs/center.py` still owned the concrete row-range expansion algorithm:

```text
BereichToNumbers2
BereichToNumbers2_EinBereich
BereichToNumbers2_EinBereich_Menge
isZeilenAngabe
strAsGeneratorToListOfNumStrs
```

The new architecture already had `RowRangeSyntax` inside `input_semantics.py`,
but the actual morphism from row-range text to row-number sets was still mostly
legacy-owned.

## Stage 37 change

The implementation now lives in:

```text
reta_architecture/row_ranges.py
```

`libs/center.py` now keeps only compatibility wrappers around
`ROW_RANGE_MORPHISMS`.

## Paradigm mapping

| Reta piece | Stage-37 architecture role | Mathematical reading |
|---|---|---|
| raw text such as `1-3`, `v2-2`, `{1,2,5}` | `RowRangeExpression` | local input section |
| `RowRangeSyntax` | syntax/topology basis | open row-range context |
| `range_to_numbers` | parser/expander | morphism to row-index set |
| include/exclude sets | glued local pieces | universal union/difference construction |
| center wrappers | legacy facade | natural transformation to architecture path |

## Why this stage is deliberately small

A large activation would risk breaking prompt and table behavior. Row-range
parsing is a good first activation because it is central, testable, and can be
kept behind the old API names.
