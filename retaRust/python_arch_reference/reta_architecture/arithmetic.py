from __future__ import annotations

"""Stage-38 activated arithmetic morphisms for Reta's center facade.

Historically ``libs/center.py`` owned a small but widely used cluster of
arithmetic helpers: factor pairs, divisor expansion from row-range syntax,
prime factor decomposition, repeated-prime formatting, dictionary inversion and
simple digit detection.  Stage 38 moves that concrete logic into the architecture
package while ``center.py`` keeps the historical function names as wrappers.

Mathematical reading:

* a number/range expression is a local arithmetic section;
* row-range parsing is reused as the Stage-37 input topology;
* prime/multiple/divisor expansion is a morphism into finite arithmetic sets;
* divisor expansion over a parsed range is a universal gluing of local factor
  sections;
* the old ``center`` API and the new architecture API are connected by a
  natural transformation: both routes must return the same arithmetic data.
"""

import math
from dataclasses import dataclass
from typing import Callable, Iterable, Optional

from .row_ranges import RowRangeMorphismBundle, bootstrap_row_range_morphisms


Classifier = Callable[[int], str]


def factor_pairs(value: int, include_one: bool = True):
    """Return factor pairs for ``value``, preserving legacy ``center.multiples`` behaviour."""
    pairs = set()
    for divisor in range(2, math.floor(math.sqrt(value) + 1)):
        quotient = value / divisor * 1000
        quotient = round(quotient) / 1000
        if quotient == round(quotient):
            pairs |= {(int(quotient), divisor)}
    if include_one:
        return list(pairs) + [(value, 1)]
    return list(pairs)


def prime_factors(value: int, modulo: bool = False) -> list[int]:
    """Return the legacy prime factor list used by ``center.primfaktoren``."""
    factors: list[int] = []
    remaining = value
    while remaining > 1:
        candidate = 2
        found = False
        while candidate * candidate <= value and not found:
            if remaining % candidate == 0:
                found = True
                prime = candidate
            else:
                candidate = candidate + 1
        if not found:
            prime = remaining
        factors += [prime % 24 if modulo else prime]
        remaining = remaining // prime
    return factors


def prime_repeat_legacy(values: list) -> list:
    """Format repeated prime factors like legacy ``center.primRepeat``.

    The historical function mutates the supplied list by reversing it.  This
    function deliberately preserves that side effect because external callers may
    accidentally rely on it.
    """
    values.reverse()
    count = 1
    previous = None
    grouped = []
    for value in values:
        if previous == value:
            count += 1
        else:
            count = 1
        grouped += [[value, count]]
        previous = value
    grouped.reverse()
    previous = None
    result = []
    for value, amount in grouped:
        if previous != value:
            if amount == 1:
                result += [value]
            else:
                result += [str(value) + "^" + str(amount)]
        previous = value
    return result


def prime_repeat_pairs(values: list) -> list[tuple[int, int]]:
    """Return repeated prime factors as ``(prime, amount)`` pairs.

    Like ``prime_repeat_legacy`` this intentionally preserves the legacy list
    reversal side effect.
    """
    values.reverse()
    count = 1
    previous = None
    grouped = []
    for value in values:
        if previous == value:
            count += 1
        else:
            count = 1
        grouped += [[value, count]]
        previous = value
    grouped.reverse()
    previous = None
    result: list[tuple[int, int]] = []
    for value, amount in grouped:
        if previous != value:
            if amount == 1:
                result += [(int(value), 1)]
            else:
                result += [(int(value), int(amount))]
        previous = value
    return result


def invert_int_value_dict(source: dict) -> dict[int, list[str]]:
    """Invert a dict of keys -> stringified integer lists like ``center.invert_dict_B``."""
    inverted: dict[int, list[str]] = {}
    for key, value_list in source.items():
        for value in value_list:
            int_value = int(value)
            if value not in inverted:
                inverted[int_value] = []
            str_key = str(key)
            if str_key not in inverted[int_value]:
                inverted[int_value].append(str_key)
    return inverted


def has_digit(text) -> bool:
    """Return whether ``text`` contains at least one decimal digit."""
    for char in text:
        if char.isdigit():
            return True
    return False


def divisor_range(
    range_expression: str,
    row_ranges: RowRangeMorphismBundle | None = None,
) -> tuple[list[str], set[int]]:
    """Expand a row-range expression to all non-trivial factor values.

    This mirrors ``center.teiler`` and reuses the Stage-37 row-range morphism as
    its input topology.
    """
    row_ranges = row_ranges or bootstrap_row_range_morphisms()
    numbers = row_ranges.range_to_numbers(range_expression, False, 0)
    divisor_values: set[int] = set()
    parallel_result = None
    try:
        from .parallel_execution import factor_pairs_in_processes

        parallel_result = factor_pairs_in_processes(numbers)
    except Exception:
        parallel_result = None
    if parallel_result is not None:
        for _number, pairs in parallel_result.values:
            for pair in set(tuple(item) for item in pairs):
                divisor_values |= set(pair)
    else:
        for number in numbers:
            for pair in set(factor_pairs(int(number))):
                divisor_values |= set(pair)
    if divisor_values != {1}:
        divisor_values -= {1}
    string_values = [str(value) for value in divisor_values]
    return string_values, divisor_values


def modulo_table_lines(values: Iterable, classify: Classifier | None = None) -> list[str]:
    """Return the lines printed by the legacy ``center.moduloA`` helper."""
    if classify is None:
        classify = lambda value: ""  # pragma: no cover - center normally supplies i18n.classify
    lines: list[str] = []
    for raw in values:
        for divisor in range(2, 26):
            mod = int(raw) % divisor
            complement = divisor - mod
            lines.append(f"{raw} % {divisor} = {mod} {classify(mod)}, {complement} {classify(complement)}")
    return lines


@dataclass(frozen=True)
class ArithmeticMorphismBundle:
    """Activated architecture owner for center-level arithmetic helpers."""

    row_ranges: RowRangeMorphismBundle
    classify: Classifier | None = None
    legacy_owner: str = "libs.center"
    activated_stage: int = 38

    def multiples(self, value: int, include_one: bool = True):
        return factor_pairs(value, include_one)

    def divisors_for_range(self, range_expression: str) -> tuple[list[str], set[int]]:
        return divisor_range(range_expression, self.row_ranges)

    def prime_factors(self, value: int, modulo: bool = False) -> list[int]:
        return prime_factors(value, modulo)

    def prime_repeat(self, values: list) -> list:
        return prime_repeat_legacy(values)

    def prime_repeat_pairs(self, values: list) -> list[tuple[int, int]]:
        return prime_repeat_pairs(values)

    def invert_dict(self, source: dict) -> dict[int, list[str]]:
        return invert_int_value_dict(source)

    def has_digit(self, text) -> bool:
        return has_digit(text)

    def modulo_lines(self, values: Iterable) -> list[str]:
        return modulo_table_lines(values, self.classify)

    def print_modulo_table(self, values: Iterable) -> None:
        for line in self.modulo_lines(values):
            print(line)

    def snapshot(self) -> dict:
        return {
            "class": "ArithmeticMorphismBundle",
            "stage": self.activated_stage,
            "legacy_owner": self.legacy_owner,
            "capsule": "InputPromptCapsule",
            "category": "ActivatedArithmeticCategory",
            "functor": "ArithmeticActivationFunctor",
            "natural_transformation": "CenterArithmeticToArchitectureTransformation",
            "depends_on": {
                "row_ranges": self.row_ranges.snapshot().get("class", "RowRangeMorphismBundle"),
                "row_range_stage": self.row_ranges.snapshot().get("stage"),
                "classify": self.classify is not None,
            },
            "morphisms": [
                "factor_pairs",
                "divisor_range",
                "prime_factors",
                "prime_repeat_legacy",
                "prime_repeat_pairs",
                "invert_int_value_dict",
                "has_digit",
                "modulo_table_lines",
            ],
            "compatibility_names": [
                "multiples",
                "teiler",
                "primfaktoren",
                "primRepeat",
                "primRepeat2",
                "invert_dict_B",
                "textHatZiffer",
                "moduloA",
            ],
            "observable_invariant": "center arithmetic wrappers and ArithmeticMorphismBundle return identical factor, divisor, prime-repeat and digit results",
        }


def bootstrap_arithmetic_morphisms(
    row_ranges: RowRangeMorphismBundle | None = None,
    classify: Classifier | None = None,
) -> ArithmeticMorphismBundle:
    return ArithmeticMorphismBundle(row_ranges=row_ranges or bootstrap_row_range_morphisms(), classify=classify)
