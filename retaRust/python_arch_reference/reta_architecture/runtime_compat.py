from __future__ import annotations

"""Architecture-local compatibility runtime helpers.

This module centralises the small set of names that historically came from the
legacy ``center`` facade but are already owned by architecture modules:

* split i18n proxy
* row-range morphisms
* arithmetic morphisms
* console/help/wrapping helpers
* small compatibility enums/constants still expected by migrated code

The goal is simple: architecture modules can depend on these names without
re-importing legacy wrapper modules from ``libs``.
"""

import pprint
import sys
from enum import IntEnum
from pathlib import Path

from .arithmetic import bootstrap_arithmetic_morphisms
from .console_io import (
    Console,
    DefaultOrderedDict,
    Markdown,
    Syntax,
    bootstrap_console_io_morphisms,
)
from .input_semantics import RowRangeSyntax
from .row_ranges import bootstrap_row_range_morphisms
from .split_i18n import build_split_i18n_proxy


REPO_ROOT = Path(__file__).resolve().parent.parent

i18n = build_split_i18n_proxy()
ROW_RANGE_SYNTAX = RowRangeSyntax.from_i18n(i18n)
ROW_RANGE_MORPHISMS = bootstrap_row_range_morphisms(ROW_RANGE_SYNTAX)
ARITHMETIC_MORPHISMS = bootstrap_arithmetic_morphisms(
    ROW_RANGE_MORPHISMS,
    classify=i18n.classify,
)
CONSOLE_IO_MORPHISMS = bootstrap_console_io_morphisms(REPO_ROOT)

infoLog = any(arg == "-" + i18n.mainParaCmds["debug"] for arg in sys.argv)
output = True
pp = pprint.PrettyPrinter(indent=4)

Multiplikationen = i18n.Multiplikationen
kpattern = ROW_RANGE_SYNTAX.comma_split_pattern
Primzahlkreuz_pro_contra_strs = i18n.Primzahlkreuz_pro_contra_strs_Dict[
    (
        "Primzahlkreuz_pro_contra",
        "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)",
    )
]


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


def BereichToNumbers2(
    MehrereBereiche: str,
    vielfache: bool = False,
    maxZahl: int = 1028,
    allowLessEqZero: bool = False,
) -> set:
    return ROW_RANGE_MORPHISMS.range_to_numbers(
        MehrereBereiche,
        multiples=vielfache,
        max_value=maxZahl,
        allow_less_equal_zero=allowLessEqZero,
    )


isZeilenAngabe = ROW_RANGE_MORPHISMS.is_row_range


def retaPromptHilfe():
    return CONSOLE_IO_MORPHISMS.print_reta_prompt_help(i18n)



def retaHilfe():
    return CONSOLE_IO_MORPHISMS.print_reta_help(i18n)



def getTextWrapThings(maxLen=None) -> tuple:
    return CONSOLE_IO_MORPHISMS.text_wrap_runtime(maxLen)



def x(text1, text):
    return CONSOLE_IO_MORPHISMS.debug_pair(text1, text, infoLog, output, pp)



def alxp(text):
    return CONSOLE_IO_MORPHISMS.debug_value(text, infoLog, output, pp)



def chunks(lst, n):
    return CONSOLE_IO_MORPHISMS.chunks(lst, n)



def cliout(text, color: bool = False, stype: str = ""):
    return CONSOLE_IO_MORPHISMS.cliout(text, color=color, stype=stype, output_enabled=output)



def unique_everseen(iterable, key=None):
    return CONSOLE_IO_MORPHISMS.unique_everseen(iterable, key=key)



def multiples(a, mul1: bool = True):
    return ARITHMETIC_MORPHISMS.multiples(a, mul1)



def teiler(zahlenBereichsAngabe):
    return ARITHMETIC_MORPHISMS.divisors_for_range(zahlenBereichsAngabe)



def invert_dict_B(d):
    return ARITHMETIC_MORPHISMS.invert_dict(d)



def textHatZiffer(text) -> bool:
    return ARITHMETIC_MORPHISMS.has_digit(text)



def primfaktoren(n, modulo: bool = False):
    return ARITHMETIC_MORPHISMS.prime_factors(n, modulo)



def primRepeat(values):
    return ARITHMETIC_MORPHISMS.prime_repeat(values)



def primRepeat2(values):
    return ARITHMETIC_MORPHISMS.prime_repeat_pairs(values)



def moduloA(zahlen):
    return ARITHMETIC_MORPHISMS.print_modulo_table(zahlen)
