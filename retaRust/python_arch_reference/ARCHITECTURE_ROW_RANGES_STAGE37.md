# Stage 37 Row-Range Architecture

`RowRangeMorphismBundle` is the new activated owner for row-range parsing.

## Capsule

```text
InputPromptCapsule
└─ RowRangeMorphismBundle
   ├─ RowRangeSyntax
   ├─ RowRangeExpression
   └─ RowIndexSet
```

## Main morphisms

```text
str_as_generator_to_set
is_fraction_range_token
is_integer_range_token
is_row_range_token
is_fraction_or_integer_range
is_fraction_range
is_row_range
range_to_numbers
add_single_range_segment
add_range_couple_values
add_non_multiple_values
add_multiple_values
```

## Compatibility wrappers

`libs/center.py` still exports the historical names, but delegates to
`ROW_RANGE_MORPHISMS`:

```text
BereichToNumbers2 -> RowRangeMorphismBundle.range_to_numbers
isZeilenAngabe -> RowRangeMorphismBundle.is_row_range
isZeilenBruchAngabe -> RowRangeMorphismBundle.is_fraction_range
strAsGeneratorToListOfNumStrs -> RowRangeMorphismBundle.str_as_generator
```

## Naturality requirement

```text
center.BereichToNumbers2(text)
=
architecture.row_ranges.range_to_numbers(text)
```

for accepted row-range expressions. This is recorded as
`CenterRowRangeToArchitectureTransformation`.

## Probe

```bash
python -B -S reta_architecture_probe_py.py row-ranges-json
```
