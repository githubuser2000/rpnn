from __future__ import annotations

"""Numerical morphisms used by Reta's table, row and generated-column layers.

This module is the explicit architecture owner for the small arithmetic
predicates and decompositions that historically lived in ``libs/lib4tables.py``.
They are intentionally dependency-light: no CLI, renderer, i18n or table imports.
Legacy modules may keep compatibility names, while architecture modules can now
refer to this layer directly.
"""

import math
from dataclasses import dataclass
from functools import lru_cache
from typing import Iterable, Iterator, Sequence, Tuple


def moonNumber(num: int):
    """Return bases and exponent markers for historical Reta moon numbers."""
    results: list[int] = [1]
    exponent: list[int] = [1]
    exponent.pop()
    results.pop()
    for i in range(2, num):
        oneResult: float = num ** (1 / i)
        if round(oneResult) * 100000 == round(oneResult * 100000):
            results += [int(round(oneResult))]
            exponent += [int(i - 2)]
    return results, exponent


def primFak(n: int) -> list:
    """Return prime factors of ``n`` with repetitions, preserving legacy behaviour."""
    faktoren: list = [1]
    faktoren.pop()
    z = n
    while z > 1:
        i = 2
        gefunden = False
        while i * i <= n and not gefunden:
            if z % i == 0:
                gefunden = True
                p = i
            else:
                i = i + 1
        if not gefunden:
            p = z
        faktoren += [p]
        z = z // p
    return faktoren


def divisorGenerator(n):
    large_divisors = []
    for i in range(1, int(math.sqrt(n) + 1)):
        if n % i == 0:
            yield i
            if i * i != n:
                large_divisors.append(n / i)
    for divisor in reversed(large_divisors):
        yield divisor


@lru_cache(maxsize=10489)
def primRepeat(n: tuple) -> tuple:
    """Group repeated prime factors as ``(prime, amount)`` tuples."""
    n = list(n)
    n.reverse()
    c = 1
    b = None
    d: list = []
    for a in n:
        if b == a:
            c += 1
        else:
            c = 1
        d += [[a, c]]
        b = a
    d.reverse()
    b = None
    f: list = []
    for e, g in d:
        if b != e:
            if g == 1:
                f += [(e, 1)]
            else:
                f += [(e, g)]
        b = e
    return tuple(f)


def primCreativity(num: int):
    if num == 0:
        return 0
    fak = primRepeat(tuple(primFak(num)))
    if len(fak) == 1 and fak[0][1] == 1:
        return 1
    if len(fak) == 1:
        return 3
    if len(fak) < 1:
        return 0
    primAmounts = []
    for prim, primAmount in fak:
        primAmounts += [primAmount]
    for primAmount in primAmounts:
        divisors = set(divisorGenerator(primAmount)) - {1}
        if len(divisors) == 0:
            try:
                del schnittmenge
            except NameError:
                pass
            break
        try:
            schnittmenge &= divisors
        except NameError:
            schnittmenge = divisors
    try:
        if len(schnittmenge) != 0:
            return 3
        else:
            return 2
    except NameError:
        return 2
    return None


def primMultiple(n: int) -> list:
    """Return prime/multiple pairs used by Reta's legacy prime-multiple logic."""
    multiples = [(1, n)]
    for prim in primRepeat(tuple(primFak(n))):
        multiples += [(prim[0], round(n / prim[0]))]
    return multiples


def isPrimMultiple(isIt: int, multiples1: list, dontReturnList=True):
    """Test whether ``isIt`` is a requested multiple of one of its prime factors."""
    areThey: list = []
    multiples2 = primMultiple(isIt)
    for multiple1 in multiples1:
        for multiple2 in multiples2:
            areThey += [True if multiple1 == multiple2[1] else False]
            if dontReturnList and areThey[-1]:
                return True
    if dontReturnList:
        return False
    return areThey


def couldBePrimeNumberPrimzahlkreuz(num: int) -> bool:
    Under24 = (1, 5, 7, 11, 13, 17, 19, 23)
    return num % 24 in Under24


def couldBePrimeNumberPrimzahlkreuz_fuer_innen(num: int) -> bool:
    Under24 = (5, 11, 17, 23)
    return num % 24 in Under24


def couldBePrimeNumberPrimzahlkreuz_fuer_aussen(num: int) -> bool:
    Under24 = (1, 7, 13, 19)
    return num % 24 in Under24


@dataclass(frozen=True)
class NumberTheoryBundle:
    """Registry/facade for Reta's arithmetic morphisms."""

    def snapshot(self) -> dict:
        return {
            "class": "NumberTheoryBundle",
            "morphisms": [
                "moonNumber",
                "primFak",
                "divisorGenerator",
                "primRepeat",
                "primCreativity",
                "primMultiple",
                "isPrimMultiple",
                "couldBePrimeNumberPrimzahlkreuz",
                "couldBePrimeNumberPrimzahlkreuz_fuer_innen",
                "couldBePrimeNumberPrimzahlkreuz_fuer_aussen",
            ],
            "legacy_owner": "libs.lib4tables",
            "dependency_profile": "math-only",
        }

    moon_number = staticmethod(moonNumber)
    prime_factors = staticmethod(primFak)
    divisor_generator = staticmethod(divisorGenerator)
    prime_repeat = staticmethod(primRepeat)
    prime_creativity = staticmethod(primCreativity)
    prime_multiple = staticmethod(primMultiple)
    is_prime_multiple = staticmethod(isPrimMultiple)
    prime_cross_candidate = staticmethod(couldBePrimeNumberPrimzahlkreuz)
    prime_cross_inner_candidate = staticmethod(couldBePrimeNumberPrimzahlkreuz_fuer_innen)
    prime_cross_outer_candidate = staticmethod(couldBePrimeNumberPrimzahlkreuz_fuer_aussen)


def bootstrap_number_theory() -> NumberTheoryBundle:
    return NumberTheoryBundle()
