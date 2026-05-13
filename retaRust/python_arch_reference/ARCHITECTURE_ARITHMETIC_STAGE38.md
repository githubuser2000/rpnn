# Stage 38 Arithmetic Architecture

`ArithmeticMorphismBundle` is the new activated owner for center-level arithmetic
helpers.

## Capsule

```text
InputPromptCapsule
├─ RowRangeMorphismBundle
└─ ArithmeticMorphismBundle
   ├─ ArithmeticExpression
   ├─ FactorPairSet
   ├─ PrimeFactorSection
   └─ DivisorSection
```

## Main morphisms

```text
factor_pairs
prime_factors
prime_repeat_legacy
prime_repeat_pairs
invert_int_value_dict
has_digit
divisor_range
modulo_table_lines
```

## Compatibility wrappers

`libs/center.py` still exports the historical names, but delegates to
`ARITHMETIC_MORPHISMS`:

```text
multiples      -> ArithmeticMorphismBundle.multiples
teiler         -> ArithmeticMorphismBundle.divisors_for_range
primfaktoren   -> ArithmeticMorphismBundle.prime_factors
primRepeat     -> ArithmeticMorphismBundle.prime_repeat
primRepeat2    -> ArithmeticMorphismBundle.prime_repeat_pairs
invert_dict_B  -> ArithmeticMorphismBundle.invert_dict
textHatZiffer  -> ArithmeticMorphismBundle.has_digit
moduloA        -> ArithmeticMorphismBundle.print_modulo_table
```

## Naturality requirement

```text
center.multiples(x)
=
architecture.arithmetic.factor_pairs(x)
```

and similarly for `teiler`, `primfaktoren`, `primRepeat`, `primRepeat2`,
`invert_dict_B` and `textHatZiffer`.

This is recorded as:

```text
CenterArithmeticToArchitectureTransformation
```

## Gluing requirement

`teiler(...)` is no longer a standalone center trick.  It is now read as:

```text
RowRangeMorphismBundle.range_to_numbers(...)
  -> factor_pairs for each local row number
  -> universal union/difference gluing
  -> DivisorSection
```

This is recorded as:

```text
ArithmeticRowRangeGluingTransformation
```

## Probe

```bash
python -B -S reta_architecture_probe_py.py arithmetic-json
```
