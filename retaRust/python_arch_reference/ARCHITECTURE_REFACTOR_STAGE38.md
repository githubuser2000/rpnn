# Architecture Refactor Stage 38

Stage 38 is the second controlled activation after the Stage-36 activation plan.

## Before Stage 38

`libs/center.py` still owned arithmetic helpers such as:

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

Stage 37 had already activated row-range parsing, but the arithmetic helpers
still lived as direct legacy algorithms in `center.py`.

## Stage 38 change

The implementation now lives in:

```text
reta_architecture/arithmetic.py
```

`libs/center.py` keeps only compatibility wrappers around
`ARITHMETIC_MORPHISMS`.

## Paradigm mapping

| Reta piece | Stage-38 architecture role | Mathematical reading |
|---|---|---|
| numbers and row-range expressions | `ArithmeticExpression` | local arithmetic section |
| `multiples(...)` | `factor_pairs(...)` | morphism to factor-pair section |
| `primfaktoren(...)` | `prime_factors(...)` | morphism to prime-factor section |
| `teiler(...)` | `divisor_range(...)` | universal gluing over row-range topology |
| center wrappers | legacy facade | natural transformation to architecture path |
| `ArithmeticMorphismBundle` | new owner | activated architecture capsule |

## Categorical additions

```text
ActivatedArithmeticCategory
ArithmeticActivationFunctor
CenterArithmeticCompatibilityFunctor
ArithmeticRowRangeGluingFunctor
ArithmeticValidationFunctor
CenterArithmeticToArchitectureTransformation
ArithmeticRowRangeGluingTransformation
CenterArithmeticCompatibilitySquare
ArithmeticRowRangeGluingSquare
ActivatedArithmeticLaw
```

## Why this stage is deliberately small

The center arithmetic helpers are central enough to matter, but small enough to
activate safely.  They also compose naturally with Stage 37: divisor expansion
uses the activated row-range parser as its input topology.
