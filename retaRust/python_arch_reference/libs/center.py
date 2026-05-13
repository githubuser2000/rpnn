#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import math
import os
import platform
import pprint
import re
import sys
from collections import OrderedDict

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
if REPO_ROOT not in sys.path:
    sys.path.insert(0, REPO_ROOT)

from reta_architecture.input_semantics import RowRangeSyntax
from reta_architecture.row_ranges import bootstrap_row_range_morphisms
from reta_architecture.arithmetic import bootstrap_arithmetic_morphisms
from reta_architecture.console_io import (
    Console,
    Markdown,
    Syntax,
    DefaultOrderedDict,
    bootstrap_console_io_morphisms,
)
from reta_architecture.split_i18n import build_split_i18n_proxy

i18n = build_split_i18n_proxy()
ROW_RANGE_SYNTAX = RowRangeSyntax.from_i18n(i18n)
ROW_RANGE_MORPHISMS = bootstrap_row_range_morphisms(ROW_RANGE_SYNTAX)

try:
    from collections import Callable
except ImportError:
    from typing import Callable

from itertools import filterfalse
from typing import Optional

try:
    from orderedset import OrderedSet
except (ModuleNotFoundError, ImportError):
    OrderedSet = set

from enum import IntEnum

# gspattern = r"\s+(?![^(){}\[\]]*(?:\([^(){}\[\]]*\)[^(){}\[\]]*|{[^(){}\[\]]*}[^(){}\[\]]*|\[[^(){}\[\]]*\][^(){}\[\]]*))"

kpattern = ROW_RANGE_SYNTAX.comma_split_pattern
Primzahlkreuz_pro_contra_strs = i18n.Primzahlkreuz_pro_contra_strs_Dict[
    (
        "Primzahlkreuz_pro_contra",
        "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)",
    )
]
# try:
#    from numba import jit
# except:
#
#    def jit(nopython=None, parallel=True, cache=True):
#        def _jit(f):
#            return f
#
#        return _jit


# originalLinesRange = range(1028)  # Maximale Zeilenanzahl

infoLog = False
output = True
pp = pprint.PrettyPrinter(indent=4)

for arg in sys.argv:
    if arg == "-" + i18n.mainParaCmds["debug"]:
        infoLog = True

Multiplikationen = i18n.Multiplikationen
classify = i18n.classify
ARITHMETIC_MORPHISMS = bootstrap_arithmetic_morphisms(ROW_RANGE_MORPHISMS, classify=classify)
CONSOLE_IO_MORPHISMS = bootstrap_console_io_morphisms(REPO_ROOT)
shellRowsAmount: int


class nPmEnum(IntEnum):
    galN = 2
    gal1pN = 3
    uniN = 4
    uni1pN = 5
    emoN = 6
    emo1pN = 7
    groeN = 8
    groe1pN = 9

    @classmethod
    def gal(cls):
        return cls.galN, cls.gal1pN

    @classmethod
    def uni(cls):
        return cls.uniN, cls.uni1pN

    @classmethod
    def emo(cls):
        return cls.emoN, cls.emo1pN

    @classmethod
    def groe(cls):
        return cls.groeN, cls.groe1pN

    @classmethod
    def n(cls):
        return cls.galN, cls.uniN, cls.emoN, cls.groeN

    @classmethod
    def einsPn(cls):
        return cls.gal1pN, cls.uni1pN, cls.emo1pN, cls.groe1pN


def isZeilenBruchAngabe_betweenKommas(g):
    return ROW_RANGE_MORPHISMS.is_fraction_token(g)


def isZeilenBruchOrGanzZahlAngabe(text):
    return ROW_RANGE_MORPHISMS.is_fraction_or_integer_range(text)


def isZeilenBruchAngabe(text):
    return ROW_RANGE_MORPHISMS.is_fraction_range(text)


def isZeilenAngabe(text):
    return ROW_RANGE_MORPHISMS.is_row_range(text)


def isZeilenAngabe_betweenKommas(g):
    return ROW_RANGE_MORPHISMS.is_row_token(g)


def retaPromptHilfe():
    """Legacy wrapper for Stage-39 console/help morphism."""
    return CONSOLE_IO_MORPHISMS.print_reta_prompt_help(i18n)


def retaHilfe():
    """Legacy wrapper for Stage-39 full reta help morphism."""
    return CONSOLE_IO_MORPHISMS.print_reta_help(i18n)


def getTextWrapThings(maxLen=None) -> tuple:
    """Legacy wrapper for Stage-39 terminal wrapping morphism."""
    return CONSOLE_IO_MORPHISMS.text_wrap_runtime(maxLen)


def x(text1, text):
    """Legacy wrapper for Stage-39 labelled debug-output morphism."""
    return CONSOLE_IO_MORPHISMS.debug_pair(text1, text, infoLog, output, pp)


def alxp(text):
    """Legacy wrapper for Stage-39 unlabelled debug-output morphism."""
    return CONSOLE_IO_MORPHISMS.debug_value(text, infoLog, output, pp)


def chunks(lst, n):
    """Legacy wrapper for Stage-39 finite-section chunking morphism."""
    return CONSOLE_IO_MORPHISMS.chunks(lst, n)


def cliout(text, color=False, stype=""):
    """Legacy wrapper for Stage-39 CLI-output morphism."""
    return CONSOLE_IO_MORPHISMS.cliout(text, color=color, stype=stype, output_enabled=output)

    # class AlxList(list):
    # def __eq__(self, bla):
    # return hash(str(super())) == hash(str(bla))

    # def __gt__(self, bla):
    # return hash(str(super())) > hash(str(bla))

    # def __ge__(self, bla):
    # return hash(str(super())) >= hash(str(bla))

    # def __lt__(self, bla):
    # return hash(str(super())) < hash(str(bla))

    # def __le__(self, bla):
    # return hash(str(super())) <= hash(str(bla))


# def sort(array):
# less: list = []
# equal: list = []
# greater: list = []

# if len(array) > 1:
# pivot = array[0]
# pivot: list = list(pivot)
# pivot2: list = pivot
# for x in array:
# x = list(x)
# x2 = x
# if x2 < pivot2:
# less.append(x)
# elif x2 == pivot2:
# equal.append(x)
# elif x2 > pivot2:
# greater.append(x)
# # Don't forget to return something!
# return (
# sort(less) + equal + sort(greater)
# )  # Just use the + operator to join lists
# # Note that you want equal ^^^^^ not pivot
# else:  # You need to handle the part at the end of the recursion - when you only have one element in your array, just return the array.
# return array


def strAsGeneratorToListOfNumStrs(text: str) -> set:
    return ROW_RANGE_MORPHISMS.str_as_generator(text)


# DefaultOrderedDict is imported from reta_architecture.console_io in Stage 39.


def unique_everseen(iterable, key=None):
    """Legacy wrapper for Stage-39 order-preserving uniqueness morphism."""
    return CONSOLE_IO_MORPHISMS.unique_everseen(iterable, key=key, ordered_set_factory=OrderedSet)


# @jit(nopython=True, parallel=True, cache=True)
# def BereichToNumbers(MehrereBereiche: str) -> set:
#
#    Bereiche: list = MehrereBereiche.split(",")
#    dazu: set[int] = set()
#    hinfort: set[int] = set()
#    menge: Optional[set[int]]
#
#    for EinBereich in Bereiche:
#        if len(EinBereich) > 1 and EinBereich[0] == "-":
#            EinBereich = EinBereich[1:]
#            menge = hinfort
#            generated = strAsGeneratorToListOfNumStrs(EinBereich[1:])
#            if generated is not None:
#                hinfort |= generated
#                continue
#        elif len(EinBereich) > 0 and EinBereich[0] != "-":
#            menge = dazu
#            generated = strAsGeneratorToListOfNumStrs(EinBereich)
#            if generated is not None:
#                dazu |= generated
#                continue
#        else:
#            menge = None
#
#        if menge is not None:
#            if EinBereich.isdecimal():
#                EinBereich = EinBereich + "-" + EinBereich
#            BereichCouple: list = EinBereich.split("-")
#            if (
#                len(BereichCouple) == 2
#                and BereichCouple[0].isdecimal()
#                and BereichCouple[0] != "0"
#                and BereichCouple[1].isdecimal()
#                and BereichCouple[1] != "0"
#            ):
#                for number in range(int(BereichCouple[0]), int(BereichCouple[1]) + 1):
#                    menge |= {number}
#    return dazu - hinfort


# @jit(nopython=True, parallel=True, cache=True)
def BereichToNumbers2(
    MehrereBereiche: str, vielfache=False, maxZahl: int = 1028, allowLessEqZero=False
) -> set:
    return ROW_RANGE_MORPHISMS.range_to_numbers(
        MehrereBereiche,
        multiples=vielfache,
        max_value=maxZahl,
        allow_less_equal_zero=allowLessEqZero,
    )


def BereichToNumbers2_EinBereich(EinBereich, dazu, hinfort, maxZahl, vielfache):
    return ROW_RANGE_MORPHISMS.add_single_range_segment(EinBereich, dazu, hinfort, maxZahl, vielfache)


def BereichToNumbers2_EinBereich_Menge(
    BereichCouple, around, maxZahl, menge, vielfache
):
    return ROW_RANGE_MORPHISMS.add_range_couple_values(BereichCouple, around, maxZahl, menge, vielfache)


def BereichToNumbers2_EinBereich_Menge_nichtVielfache(
    BereichCouple, around, maxZahl, menge
):
    return ROW_RANGE_MORPHISMS.add_non_multiple_values(BereichCouple, around, maxZahl, menge)


def BereichToNumbers2_EinBereich_Menge_vielfache(BereichCouple, around, maxZahl, menge):
    return ROW_RANGE_MORPHISMS.add_multiple_values(BereichCouple, around, maxZahl, menge)


# @lru_cache(maxsize=10489)
def multiples(a, mul1=True):
    """Legacy wrapper for Stage-38 ArithmeticMorphismBundle.factor_pairs."""
    return ARITHMETIC_MORPHISMS.multiples(a, mul1)

def teiler(zahlenBereichsAngabe):
    """Legacy wrapper for Stage-38 arithmetic divisor gluing."""
    return ARITHMETIC_MORPHISMS.divisors_for_range(zahlenBereichsAngabe)

def invert_dict_B(d):
    """Legacy wrapper for Stage-38 arithmetic dictionary inversion."""
    return ARITHMETIC_MORPHISMS.invert_dict(d)

def textHatZiffer(text) -> bool:
    """Legacy wrapper for Stage-38 digit-detection morphism."""
    return ARITHMETIC_MORPHISMS.has_digit(text)

def primfaktoren(n, modulo=False):
    """Legacy wrapper for Stage-38 prime-factor morphism."""
    return ARITHMETIC_MORPHISMS.prime_factors(n, modulo)

def primRepeat(n):
    """Legacy wrapper for Stage-38 repeated-prime formatter."""
    return ARITHMETIC_MORPHISMS.prime_repeat(n)

def primRepeat2(n):
    """Legacy wrapper for Stage-38 repeated-prime pair morphism."""
    return ARITHMETIC_MORPHISMS.prime_repeat_pairs(n)

def moduloA(zahlen):
    """Legacy wrapper for Stage-38 modulo-table morphism."""
    return ARITHMETIC_MORPHISMS.print_modulo_table(zahlen)

