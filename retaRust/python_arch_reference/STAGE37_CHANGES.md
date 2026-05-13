# Stage 37 Changes – Activated Row-Range Morphisms

Stage 37 builds directly on Stage 36.

Stage 36 created activation windows, commit gates, rollback sections and
transactions, but it intentionally did not move runtime behavior. Stage 37 is
the first small controlled activation: the row-range parser that historically
lived in `libs/center.py` is now owned by `reta_architecture/row_ranges.py`.

The legacy API remains intact:

```text
center.BereichToNumbers2
center.isZeilenAngabe
center.isZeilenBruchAngabe
center.strAsGeneratorToListOfNumStrs
```

but these names now delegate to:

```text
RowRangeMorphismBundle
range_to_numbers
is_row_range
is_fraction_range
str_as_generator_to_set
```

## New source file

```text
reta_architecture/row_ranges.py
```

It defines:

- `RowRangeMorphismBundle`
- `bootstrap_row_range_morphisms(...)`
- `range_to_numbers(...)`
- `is_row_range(...)`
- `is_fraction_range(...)`
- `is_row_range_token(...)`
- `str_as_generator_to_set(...)`
- compatibility aliases for the historical German function names

## New probe

```bash
python -B -S reta_architecture_probe_py.py row-ranges-json
```

## Runtime behavior

This stage is intentionally small but real. The algorithmic owner changed from
`libs/center.py` to `reta_architecture/row_ranges.py`. The observable legacy API
is preserved by wrapper delegation.

## Categorical reading

```text
libs.center legacy row-range API
  --CenterRowRangeCompatibilityFunctor-->
RowRangeMorphismBundle
  --RowRangeInputFunctor-->
LocalSectionCategory
```

Natural transformations:

```text
CenterRowRangeToArchitectureTransformation
RowRangeValidationTransformation
```

New diagrams:

```text
CenterRowRangeCompatibilitySquare
RowRangeValidationSquare
```

New law:

```text
ActivatedRowRangeLaw
```
