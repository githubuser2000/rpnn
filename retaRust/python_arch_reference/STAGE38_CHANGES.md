# Stage 38 Changes – Activated Arithmetic Morphisms

Stage 38 builds directly on Stage 37.

Stage 37 activated the row-range parser from `libs/center.py` into
`reta_architecture/row_ranges.py`.  Stage 38 performs the next small real
activation: the center-level arithmetic helpers are now owned by
`reta_architecture/arithmetic.py`.

## New source file

```text
reta_architecture/arithmetic.py
```

It defines:

```text
ArithmeticMorphismBundle
bootstrap_arithmetic_morphisms(...)
factor_pairs(...)
divisor_range(...)
prime_factors(...)
prime_repeat_legacy(...)
prime_repeat_pairs(...)
invert_int_value_dict(...)
has_digit(...)
modulo_table_lines(...)
```

## Legacy wrappers preserved

`libs/center.py` still exports the historical names:

```text
multiples
teiler
primfaktoren
primRepeat
primRepeat2
invert_dict_B
textHatZiffer
moduloA
```

but these functions now delegate through:

```text
ARITHMETIC_MORPHISMS = bootstrap_arithmetic_morphisms(...)
```

## Mathematical reading

```text
ArithmeticExpression
  -> FactorPairSet
  -> PrimeFactorSection
  -> DivisorSection
```

`teiler(...)` composes the Stage-37 row-range topology with the Stage-38
arithmetic morphisms:

```text
RowRangeExpression
  -> RowIndexSet
  -> factor-pair local sections
  -> glued divisor section
```

## New probe

```bash
python -B -S reta_architecture_probe_py.py arithmetic-json
```

## Runtime behavior

This stage is intentionally small but real.  The algorithmic owner changed from
`libs/center.py` to `reta_architecture/arithmetic.py`.  The observable legacy API
is preserved by wrapper delegation.
