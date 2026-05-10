#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
/*
DIREKT-TRANSCOMPILATIONSFRONT FÜR libs/center.py
Python-Quelle eingefroren für 1:1-Übernahme.
*/

pub const PYTHON_SOURCE__CENTER: &str = r#"#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import math
import os
import platform
import pprint
import re
import sys
from collections import OrderedDict

import i18n.words as i18n

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

from rich.console import Console
from rich.markdown import Markdown
from rich.syntax import Syntax
from enum import IntEnum

# gspattern = r"\s+(?![^(){}\[\]]*(?:\([^(){}\[\]]*\)[^(){}\[\]]*|{[^(){}\[\]]*}[^(){}\[\]]*|\[[^(){}\[\]]*\][^(){}\[\]]*))"

kpattern = r",(?![^\[\]\{\}\(\)]*[\]\}\)])"
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
    patternStr = "".join(
        ("^(", i18n.befehle2["v"], "?-?\d+\/\d+)(-\d+\/\d+)?((\+)(\d+\/\d+))*$")
    )
    pattern = re.compile(patternStr)
    return bool(re.fullmatch(pattern, g))


def isZeilenBruchOrGanzZahlAngabe(text):
    a = []
    for g in text.split(","):
        a += [isZeilenBruchAngabe_betweenKommas(g) or isZeilenAngabe_betweenKommas(g)]
    return all(a)


def isZeilenBruchAngabe(text):
    a = []
    stext = text.split(",")
    anyAtAll = any([len(txt) > 0 for txt in stext])
    for g in stext:
        a += [isZeilenBruchAngabe_betweenKommas(g) or (g == "" and anyAtAll)]
    # x("ALL", a)
    return all(a)


def isZeilenAngabe(text):
    a = []
    stext = re.split(r",(?![^\[\]\{\}\(\)]*[\]\}\)])", text)
    anyAtAll = any([len(txt) > 0 for txt in stext])
    for g in stext:
        a += [isZeilenAngabe_betweenKommas(g) or (g == "" and anyAtAll)]
    return all(a)


def isZeilenAngabe_betweenKommas(g):
    patternStr = "".join(("^(", i18n.befehle2["v"], "?-?\d+)(-\d+)?((\+)(\d+))*$"))
    pattern = re.compile(patternStr)
    generated1 = strAsGeneratorToListOfNumStrs(g)
    generated2 = strAsGeneratorToListOfNumStrs(g[1:])
    return (
        bool(re.fullmatch(pattern, g))
        or generated1 is not None
        or generated2 is not None
    )


def retaPromptHilfe():
    readMe = i18n.readMeFileNames.retaPrompt
    place = os.path.join(
        os.getcwd(), os.path.dirname(__file__), "..", "doc", os.path.basename(readMe)
    )
    with open(place, encoding="utf-8") as f:
        markdownText = f.read()
    abDa = markdownText.find("+++", 2)
    pattern = r"{#.*}"
    markdownText = re.sub(pattern, "", markdownText)
    console = Console()
    md = Markdown(markdownText[abDa + 3 :])
    console.print(md)


def retaHilfe():
    readMe = i18n.readMeFileNames.reta
    place = os.path.join(
        os.getcwd(), os.path.dirname(__file__), "..", "doc", os.path.basename(readMe)
    )
    with open(place, encoding="utf-8") as f:
        markdownText = f.read()
    print(markdownText)
    # return markdownText
    # for m in markdownText.split("\n"):
    #    console = Console(soft_wrap=False, width=len(m), markup=False)
    #    md = Markdown(m, style="emacs")
    #    console.print(
    #        md, new_line_start=False, no_wrap=True, soft_wrap=True, width=len(m)
    #    )
    # m = markdownText.split("\n")
    # if True:
    #    console = Console(soft_wrap=False, markup=False)
    #    md = Markdown(m, style="emacs")
    #    console.print(
    #        md, new_line_start=False, no_wrap=True, soft_wrap=True, width=len(m)
    #    )
    # from pygments.styles import get_all_styles
    # styles = list(get_all_styles())
    # print(styles)


def getTextWrapThings(maxLen=None) -> tuple:
    global shellRowsAmount
    if "Brython" not in sys.version.split():
        import html2text

        try:
            import pyphen

            dic = pyphen.Pyphen(
                lang="de_DE"
            )  # Bibliothek für Worteilumbruch bei Zeilenumbruch
        except (ModuleNotFoundError, ImportError):
            dic = None
        # from hyphen import Hyphenator
        try:
            from textwrap2 import fill
        except:
            fill = None
            import pyphen

        h_de = None
        # h_de = Hyphenator("de_DE")

        # if platform.system() != "Windows":
        #    try:
        #        ColumnsRowsAmount, shellRowsAmountStr = (
        #            os.popen("stty size", "r").read().split()
        #        )  # Wie viele Zeilen und Spalten hat die Shell ?
        #    except Exception:
        #        ColumnsRowsAmount, shellRowsAmountStr = "80", "80"
        # else:
        try:
            SiZe = os.get_terminal_size()
            ColumnsRowsAmount, shellRowsAmountStr = SiZe.columns, SiZe.lines
        except OSError:
            try:
                ColumnsRowsAmount, shellRowsAmountStr = (
                    os.popen("stty size", "r").read().split()
                )  # Wie viele Zeilen und Spalten hat die Shell ?
            except Exception:
                ColumnsRowsAmount = "80"

    else:
        html2text = None
        pyphen = None
        Hyphenator = None
        fill = None
    # shellBreite = os.get_terminal_size().columns
    shellBreite = int(ColumnsRowsAmount)
    ## shellRowsAmount = int(shellRowsAmountStr) if maxLen is None else int(maxLen)
    ## shellBreite = int(shellBreite)
    ## x("sbreite", shellBreite)

    return shellBreite, h_de, dic, fill


def x(text1, text):
    global output
    """Für mich, damit ich mal alle prints ausschalten kann zum vorführen,
    wenn ich noch beim Entwicklen war."""
    if infoLog and output:
        if type(text) is str:
            print(text1 + ": " + text)
        else:
            print(text1 + ": ", end="")
            pp.pprint(text)


def alxp(text):
    global output
    """Für mich, damit ich mal alle prints ausschalten kann zum vorführen,
    wenn ich noch beim Entwicklen war."""
    if infoLog and output:
        if type(text) is str:
            print(text)
        else:
            pp.pprint(text)


def chunks(lst, n):
    """Yield successive n-sized chunks from lst."""
    for i in range(0, len(lst), n):
        yield lst[i : i + n]


def cliout(text, color=False, stype=""):
    if output:
        if color and len(text) > 0:
            text = " ".join(text.split())
            # if stype == "html":
            #    text = text.replace("<tr","\n  <tr").replace("<td","\n    <td")
            console = Console(width=len(text))
            console.print(
                Syntax(text.strip(), stype, word_wrap=True, indent_guides=True), end=""
            )
        else:
            print(text)

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
    try:
        if text[0] == "(" and text[-1] == ")":
            text = "[" + text[1:-1] + "]"

        if (text[0] == "[" and text[-1] == "]") or (text[0] == "{" and text[-1] == "}"):
            try:
                result = eval(text)
                result = set(result)
                if type(result) is set and all((type(a) is int for a in result)):
                    return result  # ",".join((str(a) for a in result))
            except Exception:
                return None
    except Exception:
        return None
    return None


class DefaultOrderedDict(OrderedDict):
    # Source: http://stackoverflow.com/a/6190500/562769
    def __init__(self, default_factory=None, *a, **kw):
        if default_factory is not None and not isinstance(default_factory, Callable):
            raise TypeError("first argument must be callable")
        OrderedDict.__init__(self, *a, **kw)
        self.default_factory = default_factory

    def __getitem__(self, key):
        try:
            return OrderedDict.__getitem__(self, key)
        except KeyError:
            return self.__missing__(key)

    def __missing__(self, key):
        if self.default_factory is None:
            raise KeyError(key)
        self[key] = value = self.default_factory()
        return value

    def __reduce__(self):
        if self.default_factory is None:
            args = tuple()
        else:
            args = (self.default_factory,)
        return type(self), args, None, None, self.items()

    def copy(self):
        return self.__copy__()

    def __copy__(self):
        return type(self)(self.default_factory, self)

    def __deepcopy__(self, memo):
        import copy

        return type(self)(self.default_factory, copy.deepcopy(self.items()))

    def __repr__(self):
        return "OrderedDefaultDict(%s, %s)" % (
            self.default_factory,
            OrderedDict.__repr__(self),
        )


def unique_everseen(iterable, key=None):
    "List unique elements, preserving order. Remember all elements ever seen."
    # unique_everseen('AAAABBBCCDAABBB') --> A B C D
    # unique_everseen('ABBCcAD', str.lower) --> A B C D
    seen = OrderedSet()
    seen_add = seen.add
    if key is None:
        for element in filterfalse(seen.__contains__, iterable):
            seen_add(element)
            yield element
    else:
        for element in iterable:
            k = key(element)
            if k not in seen:
                seen_add(k)
                yield element


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
    # print(re.split(r",(?![^\[\]\{\}\(\)]*[\]\}\)])", MehrereBereiche))
    # print([s for s in re.split(r",(?![^\[\]\{\}\(\)]*[\]\}\)])", MehrereBereiche) if s])
    MehrereBereiche = ",".join(
        [s for s in re.split(r",(?![^\[\]\{\}\(\)]*[\]\}\)])", MehrereBereiche) if s]
    )
    if not isZeilenAngabe(MehrereBereiche):
        return set()

    if not vielfache and maxZahl == 0:
        maxZahl = float("inf")

    Bereiche: list = re.split(r",(?![^\[\]\{\}\(\)]*[\]\}\)])", MehrereBereiche)
    dazu: set[int] = set()
    hinfort: set[int] = set()
    menge: Optional[set[int]]

    for EinBereich in Bereiche:
        if len(EinBereich) > 1 and EinBereich[0] == "-":
            generated = strAsGeneratorToListOfNumStrs(EinBereich[1:])
            if generated is not None:
                hinfort |= generated
                continue
        elif len(EinBereich) > 0 and EinBereich[0] != "-":
            generated = strAsGeneratorToListOfNumStrs(EinBereich)
            if generated is not None:
                dazu |= generated
                continue
        if len(EinBereich) > 0 and EinBereich[0] == i18n.befehle2["v"]:
            EinBereich = EinBereich[1:]
            vielfache2 = True
        else:
            vielfache2 = False
        BereichToNumbers2_EinBereich(
            EinBereich,
            dazu,
            hinfort,
            1028 if (vielfache or vielfache2) and maxZahl == float("inf") else maxZahl,
            vielfache or vielfache2,
        )
    if allowLessEqZero:
        return dazu - hinfort
    else:
        return set(filter(lambda x: x > 0, dazu - hinfort))


def BereichToNumbers2_EinBereich(EinBereich, dazu, hinfort, maxZahl, vielfache):
    if len(EinBereich) > 1 and EinBereich[0] == "-":
        EinBereich = EinBereich[1:]
        menge = hinfort
    elif len(EinBereich) > 0 and EinBereich[0] != "-":
        menge = dazu
    else:
        menge = None
    around = []
    if menge is not None:
        BereichTuple2: list = EinBereich.split("+")
        if EinBereich.isdecimal():
            EinBereich = EinBereich + "-" + EinBereich
        elif len(BereichTuple2) > 0 and BereichTuple2[0].isdecimal():
            EinBereich = BereichTuple2[0] + "-" + BereichTuple2[0]
            if len(BereichTuple2) > 1:
                EinBereich += "+" + "+".join(BereichTuple2[1:])
        BereichCouple: list = EinBereich.split("-")

        BereichToNumbers2_EinBereich_Menge(
            BereichCouple, around, maxZahl, menge, vielfache
        )


def BereichToNumbers2_EinBereich_Menge(
    BereichCouple, around, maxZahl, menge, vielfache
):
    if (
        len(BereichCouple) == 2
        and BereichCouple[0].isdecimal()
        and BereichCouple[0] != "0"
        # and BereichCouple[1].isdecimal()
        # and BereichCouple[1] != "0"
    ):
        BereichPlusTuples = BereichCouple[1].split("+")
        if len(BereichPlusTuples) < 2:
            around = [0]
        else:
            richtig = True
            numList = []
            for t2 in BereichPlusTuples:
                if t2.isdecimal():
                    numList += [int(t2)]
                else:
                    richtig = False
            if richtig and len(numList) > 0:
                around = numList[1:]
                BereichCouple[1] = numList[0]
        if vielfache:
            BereichToNumbers2_EinBereich_Menge_vielfache(
                BereichCouple, around, maxZahl, menge
            )
        else:
            BereichToNumbers2_EinBereich_Menge_nichtVielfache(
                BereichCouple, around, maxZahl, menge
            )


def BereichToNumbers2_EinBereich_Menge_nichtVielfache(
    BereichCouple, around, maxZahl, menge
):
    for number in range(int(BereichCouple[0]), int(BereichCouple[1]) + 1):
        for a in around:
            c = number + a
            if c < maxZahl:
                menge |= {c}
            d = number - a
            if d > 0 and d < maxZahl:
                menge |= {d}


def BereichToNumbers2_EinBereich_Menge_vielfache(BereichCouple, around, maxZahl, menge):
    i = 0
    if len(around) == 0 or len(set(around) - {0}) == 0:
        while all([int(BereichCouple[0]) * i < maxZahl - a for a in around]):
            i += 1
            for number in range(int(BereichCouple[0]), int(BereichCouple[1]) + 1):
                c = number * i
                if c <= maxZahl:
                    menge |= {c}
    else:
        while all([int(BereichCouple[0]) * i < maxZahl - a for a in around]):
            i += 1
            for number in range(int(BereichCouple[0]), int(BereichCouple[1]) + 1):
                for a in around:
                    c = (number * i) + a
                    if c <= maxZahl:
                        menge |= {c}
                    d = (number * i) - a
                    if d > 0 and d < maxZahl:
                        menge |= {d}


# @lru_cache(maxsize=10489)
def multiples(a, mul1=True):
    """
    findet für eine Zahl alle Kombinationen aus möglichen Multiplikationen aus ganzen Zahlen, die diese Zahl ergibt
    @type a: int
    @param a: Produkt von mehreren möglichen Faktoren
    @type mul1: bool
    @param mul1: ob auch 1 * a als Faktor als geordnetes Paar mit am Ende dazu kommen soll
    @return: gibt Liste an Paaren von Faktoren aus
    """
    menge = set()
    for b in range(2, math.floor(math.sqrt(a) + 1)):
        c = a / b * 1000
        c = round(c) / 1000
        if c == round(c):
            menge |= {(int(c), b)}
    if mul1:
        menge = list(menge) + [(a, 1)]
    else:
        menge = list(menge)
    # menge.sort()
    return menge


def teiler(zahlenBereichsAngabe):
    ZahlenBereichMenge = BereichToNumbers2(zahlenBereichsAngabe, False, 0)
    ZahlenWbereichMenge = set()
    for each1 in ZahlenBereichMenge:
        for each2 in set(multiples(int(each1))):
            ZahlenWbereichMenge |= set(each2)
    if ZahlenWbereichMenge != {1}:
        ZahlenWbereichMenge -= {1}
    zahlenWBereichStringListe = [str(each2) for each2 in ZahlenWbereichMenge]
    return zahlenWBereichStringListe, ZahlenWbereichMenge


def invert_dict_B(d):
    new_dict = {}
    for key, value_list in d.items():
        for value in value_list:
            intVal = int(value)
            if value not in new_dict:
                new_dict[intVal] = []
            strKey = str(key)
            if strKey not in new_dict[intVal]:
                new_dict[intVal].append(strKey)
    return new_dict


# def invert_dict(d):
#    new_dict = {}
#    for key, value_list in d.items():
#        for value in value_list:
#            if value not in new_dict:
#                new_dict[value] = []
#            new_dict[value].append(key)
#    return new_dict


def textHatZiffer(text) -> bool:
    for char in text:
        if char.isdigit():
            return True
    return False


def primfaktoren(n, modulo=False):
    """zerlegt eine Zahl in ihre Primfaktoren

    >>> primfaktoren(24)
    [2, 2, 2, 3]

    """

    faktoren = []
    z = n
    while z > 1:
        # bestimme den kleinsten Primfaktor p von z
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
        # füge p in die Liste der Faktoren ein
        if modulo:
            faktoren += [p % 24]
        else:
            faktoren += [p]
        z = z // p
    return faktoren


def primRepeat(n):
    n.reverse()
    c = 1
    b = None
    d = []
    for a in n:
        if b == a:
            c += 1
        else:
            c = 1
        d += [[a, c]]
        b = a
    d.reverse()
    b = None
    f = []
    for e, g in d:
        if b != e:
            if g == 1:
                f += [e]
            else:
                f += [str(e) + "^" + str(g)]
        b = e

    return f

def primRepeat2(n):
    n.reverse()
    c = 1
    b = None
    d = []
    for a in n:
        if b == a:
            c += 1
        else:
            c = 1
        d += [[a, c]]
        b = a
    d.reverse()
    b = None
    f = []
    for e, g in d:
        if b != e:
            if g == 1:
                f += [(int(e), 1)]
            else:
                f += [(int(e), int(g))]
        b = e

    return f


classify = i18n.classify


def moduloA(zahlen):
    for arg in zahlen:
        for var in range(2, 26):
            print(f"{arg} % {var} = ", end="")
            mod = int(arg) % var
            print(f"{mod} {classify(mod)}", end=", ")
            mod = var - mod
            print(f"{mod} {classify(mod)}")
"#;

use std::collections::{BTreeMap, BTreeSet};

use crate::shared::reta_program_types::Program;

pub const kpattern: &str = r",(?![^\[\]\{\}\(\)]*[\]\}\)])";
const V_PREFIX: char = 'v';

fn split_kpattern_comma_here(tail_after_comma: &str) -> bool {
    for ch in tail_after_comma.chars() {
        if matches!(ch, ']' | '}' | ')') {
            return false;
        }
        if matches!(ch, '[' | '{' | '(') {
            return true;
        }
    }
    true
}

pub fn split_kpattern_commas(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut i = 0usize;
    while i < chars.len() {
        let (byte_idx, ch) = chars[i];
        if ch == ',' && split_kpattern_comma_here(&text[byte_idx + ch.len_utf8()..]) {
            out.push(std::mem::take(&mut current));
        } else {
            current.push(ch);
        }
        i += 1;
    }
    out.push(current);
    out
}

fn is_ascii_decimal(text: &str) -> bool {
    !text.is_empty() && text.chars().all(|ch| ch.is_ascii_digit())
}

fn is_signed_ascii_decimal(text: &str) -> bool {
    if let Some(rest) = text.strip_prefix('-') {
        is_ascii_decimal(rest)
    } else {
        is_ascii_decimal(text)
    }
}

fn parse_i64_literal(text: &str) -> Option<i64> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    let rest = trimmed.strip_prefix('+').unwrap_or(trimmed);
    if let Some(rest2) = rest.strip_prefix('-') {
        if is_ascii_decimal(rest2) {
            return trimmed.parse::<i64>().ok();
        }
        None
    } else if is_ascii_decimal(rest) {
        trimmed.parse::<i64>().ok()
    } else {
        None
    }
}

/// Python center.py::strAsGeneratorToListOfNumStrs.
///
/// Keep Python's outer-shape check byte-for-byte strict, then reuse the
/// already-ported Python-like integer expression parser from the active reta
/// core for the literal contents.
pub fn strAsGeneratorToListOfNumStrs(text: &str) -> Option<BTreeSet<i64>> {
    if text.is_empty() {
        return None;
    }

    let owned_storage;
    let literal: &str = if text.starts_with('(') && text.ends_with(')') && text.len() >= 2 {
        owned_storage = Some(format!("[{}]", &text[1..text.len() - 1]));
        owned_storage.as_deref().unwrap_or(text)
    } else {
        owned_storage = None;
        text
    };

    if !((literal.starts_with('[') && literal.ends_with(']'))
        || (literal.starts_with('{') && literal.ends_with('}')))
    {
        return None;
    }

    Program::parse_python_like_int_set_expr_py(literal)
}

pub fn isZeilenAngabe_betweenKommas(g: &str) -> bool {
    if strAsGeneratorToListOfNumStrs(g).is_some() {
        return true;
    }
    if g.len() > 1 && g.starts_with('-') && strAsGeneratorToListOfNumStrs(&g[1..]).is_some() {
        return true;
    }

    let rest = g.strip_prefix(V_PREFIX).unwrap_or(g);
    if rest.is_empty() {
        return false;
    }
    let mut plus_parts = rest.split('+');
    let Some(first) = plus_parts.next() else {
        return false;
    };
    let range_ok = if let Some((left, right)) = first.rsplit_once('-') {
        if left.is_empty() {
            is_signed_ascii_decimal(first)
        } else {
            is_signed_ascii_decimal(left) && is_ascii_decimal(right)
        }
    } else {
        is_signed_ascii_decimal(first)
    };
    range_ok && plus_parts.all(is_ascii_decimal)
}

pub fn isZeilenAngabe(text: &str) -> bool {
    let parts = split_kpattern_commas(text);
    let any_at_all = parts.iter().any(|part| !part.is_empty());
    parts
        .iter()
        .all(|part| isZeilenAngabe_betweenKommas(part) || (part.is_empty() && any_at_all))
}

pub fn isZeilenBruchAngabe_betweenKommas(g: &str) -> bool {
    let rest = g.strip_prefix(V_PREFIX).unwrap_or(g);
    let mut plus_parts = rest.split('+');
    let Some(first) = plus_parts.next() else {
        return false;
    };

    fn fraction(text: &str, signed: bool) -> bool {
        let body = if signed {
            text.strip_prefix('-').unwrap_or(text)
        } else {
            text
        };
        let Some((left, right)) = body.split_once('/') else {
            return false;
        };
        is_ascii_decimal(left) && is_ascii_decimal(right)
    }

    let range_ok = if let Some((left, right)) = first.rsplit_once('-') {
        if left.is_empty() {
            fraction(first, true)
        } else {
            fraction(left, true) && fraction(right, false)
        }
    } else {
        fraction(first, true)
    };

    range_ok && plus_parts.all(|part| fraction(part, false))
}

pub fn isZeilenBruchOrGanzZahlAngabe(text: &str) -> bool {
    text.split(',')
        .all(|part| isZeilenBruchAngabe_betweenKommas(part) || isZeilenAngabe_betweenKommas(part))
}

pub fn isZeilenBruchAngabe(text: &str) -> bool {
    let parts: Vec<&str> = text.split(',').collect();
    let any_at_all = parts.iter().any(|part| !part.is_empty());
    parts
        .iter()
        .all(|part| isZeilenBruchAngabe_betweenKommas(part) || (part.is_empty() && any_at_all))
}

pub fn BereichToNumbers2(
    MehrereBereiche: &str,
    vielfache: bool,
    maxZahl: i64,
    allowLessEqZero: bool,
) -> BTreeSet<i64> {
    let mehrere_bereiche = split_kpattern_commas(MehrereBereiche)
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(",");

    if !isZeilenAngabe(&mehrere_bereiche) {
        return BTreeSet::new();
    }

    let max_is_inf = !vielfache && maxZahl == 0;
    let bereiche = split_kpattern_commas(&mehrere_bereiche);
    let mut dazu = BTreeSet::new();
    let mut hinfort = BTreeSet::new();

    for mut ein_bereich in bereiche {
        if ein_bereich.len() > 1 && ein_bereich.starts_with('-') {
            if let Some(generated) = strAsGeneratorToListOfNumStrs(&ein_bereich[1..]) {
                hinfort.extend(generated);
                continue;
            }
        } else if !ein_bereich.is_empty() && !ein_bereich.starts_with('-') {
            if let Some(generated) = strAsGeneratorToListOfNumStrs(&ein_bereich) {
                dazu.extend(generated);
                continue;
            }
        }

        let vielfache2 = if ein_bereich.starts_with(V_PREFIX) {
            ein_bereich.remove(0);
            true
        } else {
            false
        };
        let max_for_branch = if (vielfache || vielfache2) && max_is_inf {
            Some(1028)
        } else if max_is_inf {
            None
        } else {
            Some(maxZahl)
        };
        BereichToNumbers2_EinBereich(
            &ein_bereich,
            &mut dazu,
            &mut hinfort,
            max_for_branch,
            vielfache || vielfache2,
        );
    }

    dazu.retain(|value| !hinfort.contains(value));
    if !allowLessEqZero {
        dazu.retain(|value| *value > 0);
    }
    dazu
}

fn less_than_max(value: i64, maxZahl: Option<i64>) -> bool {
    maxZahl.map(|max| value < max).unwrap_or(true)
}

fn less_or_equal_max(value: i64, maxZahl: Option<i64>) -> bool {
    maxZahl.map(|max| value <= max).unwrap_or(true)
}

fn BereichToNumbers2_EinBereich(
    EinBereich: &str,
    dazu: &mut BTreeSet<i64>,
    hinfort: &mut BTreeSet<i64>,
    maxZahl: Option<i64>,
    vielfache: bool,
) {
    let (target, mut ein_bereich): (&mut BTreeSet<i64>, String) =
        if EinBereich.len() > 1 && EinBereich.starts_with('-') {
            (hinfort, EinBereich[1..].to_string())
        } else if !EinBereich.is_empty() && !EinBereich.starts_with('-') {
            (dazu, EinBereich.to_string())
        } else {
            return;
        };

    let parts = ein_bereich
        .split('+')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    if is_ascii_decimal(&ein_bereich) {
        ein_bereich = format!("{0}-{0}", ein_bereich);
    } else if !parts.is_empty() && is_ascii_decimal(&parts[0]) {
        ein_bereich = format!("{0}-{0}", parts[0]);
        if parts.len() > 1 {
            ein_bereich.push('+');
            ein_bereich.push_str(&parts[1..].join("+"));
        }
    }

    let BereichCouple = ein_bereich
        .split('-')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    BereichToNumbers2_EinBereich_Menge(&BereichCouple, maxZahl, target, vielfache);
}

fn BereichToNumbers2_EinBereich_Menge(
    BereichCouple: &[String],
    maxZahl: Option<i64>,
    menge: &mut BTreeSet<i64>,
    vielfache: bool,
) {
    if BereichCouple.len() != 2 || !is_ascii_decimal(&BereichCouple[0]) || BereichCouple[0] == "0" {
        return;
    }

    let mut around = Vec::new();
    let mut right = BereichCouple[1].clone();
    let plus = right.split('+').map(|s| s.to_string()).collect::<Vec<_>>();
    if plus.len() < 2 {
        around.push(0);
    } else {
        let mut nums = Vec::new();
        for part in &plus {
            let Some(value) = parse_i64_literal(part) else {
                return;
            };
            if value < 0 {
                return;
            }
            nums.push(value);
        }
        if !nums.is_empty() {
            around = nums[1..].to_vec();
            right = nums[0].to_string();
        }
    }

    if !is_ascii_decimal(&right) {
        return;
    }
    let start = BereichCouple[0].parse::<i64>().unwrap_or(0);
    let end = right.parse::<i64>().unwrap_or(0);
    if vielfache {
        BereichToNumbers2_EinBereich_Menge_vielfache(start, end, &around, maxZahl, menge);
    } else {
        BereichToNumbers2_EinBereich_Menge_nichtVielfache(start, end, &around, maxZahl, menge);
    }
}

fn BereichToNumbers2_EinBereich_Menge_nichtVielfache(
    start: i64,
    end: i64,
    around: &[i64],
    maxZahl: Option<i64>,
    menge: &mut BTreeSet<i64>,
) {
    for number in start..=end {
        for a in around {
            let c = number + *a;
            if less_than_max(c, maxZahl) {
                menge.insert(c);
            }
            let d = number - *a;
            if d > 0 && less_than_max(d, maxZahl) {
                menge.insert(d);
            }
        }
    }
}

fn BereichToNumbers2_EinBereich_Menge_vielfache(
    start: i64,
    end: i64,
    around: &[i64],
    maxZahl: Option<i64>,
    menge: &mut BTreeSet<i64>,
) {
    let mut i = 0i64;
    let only_zero_around = around.is_empty() || around.iter().all(|value| *value == 0);
    loop {
        let keep_going = around.iter().all(|a| {
            let limit = maxZahl.map(|max| max - *a);
            limit.map(|limit| start * i < limit).unwrap_or(true)
        });
        if !keep_going {
            break;
        }
        i += 1;
        for number in start..=end {
            if only_zero_around {
                let c = number * i;
                if less_or_equal_max(c, maxZahl) {
                    menge.insert(c);
                }
            } else {
                for a in around {
                    let c = (number * i) + *a;
                    if less_or_equal_max(c, maxZahl) {
                        menge.insert(c);
                    }
                    let d = (number * i) - *a;
                    if d > 0 && less_than_max(d, maxZahl) {
                        menge.insert(d);
                    }
                }
            }
        }
    }
}

/// Python `center.chunks`: return successive `n` sized chunks.
///
/// The Python version is a generator.  The Rust port materializes the chunks so
/// callers get a simple, ownership-safe value while keeping the same partition
/// boundaries.
pub fn chunks<T: Clone>(lst: &[T], n: usize) -> Vec<Vec<T>> {
    if n == 0 {
        panic!("range() arg 3 must not be zero");
    }
    lst.chunks(n).map(|chunk| chunk.to_vec()).collect()
}

#[allow(non_snake_case)]
pub fn textHatZiffer(text: &str) -> bool {
    text.chars().any(|ch| ch.is_numeric())
}

const PY_SET_LINEAR_PROBES: usize = 9;
const PY_SET_PERTURB_SHIFT: u32 = 5;
const PY_HASH_XXPRIME_1: u64 = 11_400_714_785_074_694_791;
const PY_HASH_XXPRIME_2: u64 = 14_029_467_366_897_019_727;
const PY_HASH_XXPRIME_5: u64 = 2_870_177_450_012_600_261;

fn python_int_hash_bits(value: i64) -> u64 {
    // CPython hashes compact integers as the integer itself.  A result of -1 is
    // reserved as an error sentinel and is therefore remapped to -2.
    if value == -1 {
        (-2i64) as u64
    } else {
        value as u64
    }
}

fn python_pair_hash_bits(pair: (i64, i64)) -> u64 {
    // CPython tuplehash (3.8+): xxHash primes, wrapping arithmetic, and the
    // final signed -1 remap.  `center.multiples` exposes this through
    // `list(set(...))`, so reproducing the hash is observable output.
    let mut acc = PY_HASH_XXPRIME_5;
    for value in [pair.0, pair.1] {
        let lane = python_int_hash_bits(value);
        acc = acc.wrapping_add(lane.wrapping_mul(PY_HASH_XXPRIME_2));
        acc = acc.rotate_left(31);
        acc = acc.wrapping_mul(PY_HASH_XXPRIME_1);
    }
    acc = acc.wrapping_add(2u64 ^ (PY_HASH_XXPRIME_5 ^ 3_527_539));
    if acc == u64::MAX {
        1_546_275_796
    } else {
        acc
    }
}

#[derive(Clone, Debug)]
struct PythonPairSet {
    table: Vec<Option<(i64, i64)>>,
    used: usize,
    fill: usize,
}

impl PythonPairSet {
    fn new() -> Self {
        Self {
            table: vec![None; 8],
            used: 0,
            fill: 0,
        }
    }

    fn items(&self) -> Vec<(i64, i64)> {
        self.table.iter().filter_map(|entry| *entry).collect()
    }

    fn insert_clean(&mut self, item: (i64, i64)) -> bool {
        let hash = python_pair_hash_bits(item);
        let mask = self.table.len() - 1;
        let mut index = (hash as usize) & mask;
        let mut perturb = hash;

        loop {
            let mut probes = if index + PY_SET_LINEAR_PROBES <= mask {
                PY_SET_LINEAR_PROBES
            } else {
                0
            };

            loop {
                match self.table[index] {
                    None => {
                        self.table[index] = Some(item);
                        return false;
                    }
                    Some(existing) if existing == item => return true,
                    Some(_) => {}
                }

                if probes == 0 {
                    break;
                }
                index += 1;
                probes -= 1;
            }

            perturb >>= PY_SET_PERTURB_SHIFT;
            index = index
                .wrapping_mul(5)
                .wrapping_add(1)
                .wrapping_add(perturb as usize)
                & mask;
        }
    }

    fn resize(&mut self, min_used: usize) {
        let old = self.items();
        let mut new_size = 8usize;
        while new_size <= min_used {
            new_size = new_size.saturating_mul(2);
        }
        self.table = vec![None; new_size];
        self.fill = self.used;
        for item in old {
            let _ = self.insert_clean(item);
        }
    }

    fn update_like_set_merge<I>(&mut self, source_order: I)
    where
        I: IntoIterator<Item = (i64, i64)>,
    {
        let source = source_order.into_iter().collect::<Vec<_>>();
        if source.is_empty() {
            return;
        }

        let mask = self.table.len() - 1;
        if self.fill.saturating_add(source.len()).saturating_mul(5) >= mask.saturating_mul(3) {
            self.resize(self.used.saturating_add(source.len()).saturating_mul(2));
        }

        for item in source {
            if !self.insert_clean(item) {
                self.used += 1;
                self.fill += 1;
            }
        }
    }
}

/// Python `center.multiples(a, mul1=True)`.
///
/// The Python version builds a `set` with `menge |= {(int(c), b)}` and exposes
/// the table order through `list(menge)`.  This keeps that CPython-visible order
/// instead of sorting the divisor pairs, then appends `(a, 1)` exactly like
/// Python.
pub fn multiples(a: i64, mul1: bool) -> Vec<(i64, i64)> {
    if a < 0 {
        panic!("math domain error");
    }
    let mut menge = PythonPairSet::new();
    let upper = ((a as f64).sqrt() + 1.0).floor() as i64;
    for b in 2..upper {
        let c = ((a as f64 / b as f64) * 1000.0).round() / 1000.0;
        if c == c.round() {
            menge.update_like_set_merge(std::iter::once((c as i64, b)));
        }
    }
    let mut out = menge.items();
    if mul1 {
        out.push((a, 1));
    }
    out
}

/// Python `multis.mult`: render multiplication pairs for every decimal input.
pub fn mult<T: ToString>(liste: &[T]) -> Vec<String> {
    let mut ergebnis = Vec::new();
    for arg in liste {
        let text = arg.to_string();
        if !is_ascii_decimal(&text) {
            continue;
        }
        if let Ok(n) = text.parse::<i64>() {
            ergebnis.push(format!("{}: {:?}", text, multiples(n, true)));
        }
    }
    ergebnis
}

/// Python `multis.mult2`: return rendered multiplication pairs and the raw
/// non-trivial pair lists for callers that need to keep working with factors.
pub fn mult2<T: ToString>(liste: &[T]) -> (Vec<String>, Vec<Vec<(i64, i64)>>) {
    let mut ergebnis1 = Vec::new();
    let mut ergebnis2 = Vec::new();
    for arg in liste {
        let text = arg.to_string();
        if !is_ascii_decimal(&text) {
            continue;
        }
        if let Ok(n) = text.parse::<i64>() {
            let couples: Vec<(i64, i64)> = multiples(n, true)
                .into_iter()
                .filter(|(a, b)| *a != 1 && *b != 1)
                .collect();
            ergebnis1.push(format!("{}: {:?}", text, couples));
            ergebnis2.push(couples);
        }
    }
    (ergebnis1, ergebnis2)
}

pub fn teiler(zahlenBereichsAngabe: &str) -> (Vec<String>, BTreeSet<i64>) {
    let zahlen_bereich_menge = BereichToNumbers2(zahlenBereichsAngabe, false, 0, false);
    let mut zahlen_wbereich_menge: BTreeSet<i64> = BTreeSet::new();
    for each1 in zahlen_bereich_menge {
        for (left, right) in multiples(each1, true) {
            zahlen_wbereich_menge.insert(left);
            zahlen_wbereich_menge.insert(right);
        }
    }
    if zahlen_wbereich_menge != BTreeSet::from([1]) {
        zahlen_wbereich_menge.remove(&1);
    }
    let zahlen_wbereich_string_liste = zahlen_wbereich_menge
        .iter()
        .map(|each2| each2.to_string())
        .collect::<Vec<_>>();
    (zahlen_wbereich_string_liste, zahlen_wbereich_menge)
}

#[allow(non_snake_case)]
pub fn invert_dict_B<K, V>(d: &BTreeMap<K, Vec<V>>) -> BTreeMap<i64, Vec<String>>
where
    K: Ord + ToString,
    V: ToString,
{
    let mut new_dict: BTreeMap<i64, Vec<String>> = BTreeMap::new();
    for (key, value_list) in d {
        for value in value_list {
            let Ok(int_val) = value.to_string().parse::<i64>() else {
                continue;
            };
            let str_key = key.to_string();
            let entry = new_dict.entry(int_val).or_default();
            if !entry.iter().any(|existing| existing == &str_key) {
                entry.push(str_key);
            }
        }
    }
    new_dict
}

pub fn primfaktoren(n: i64, modulo: bool) -> Vec<i64> {
    let mut faktoren = Vec::new();
    let mut z = n;
    while z > 1 {
        let mut i = 2i64;
        let mut gefunden = false;
        let mut p = z;
        while i * i <= n && !gefunden {
            if z % i == 0 {
                gefunden = true;
                p = i;
            } else {
                i += 1;
            }
        }
        if !gefunden {
            p = z;
        }
        if modulo {
            faktoren.push(p % 24);
        } else {
            faktoren.push(p);
        }
        z /= p;
    }
    faktoren
}

#[allow(non_snake_case)]
pub fn primRepeat(mut n: Vec<i64>) -> Vec<String> {
    n.reverse();
    let mut c = 1i64;
    let mut b: Option<i64> = None;
    let mut d: Vec<(i64, i64)> = Vec::new();
    for a in n {
        if b == Some(a) {
            c += 1;
        } else {
            c = 1;
        }
        d.push((a, c));
        b = Some(a);
    }
    d.reverse();
    b = None;
    let mut f = Vec::new();
    for (e, g) in d {
        if b != Some(e) {
            if g == 1 {
                f.push(e.to_string());
            } else {
                f.push(format!("{e}^{g}"));
            }
        }
        b = Some(e);
    }
    f
}

#[allow(non_snake_case)]
pub fn primRepeat2(mut n: Vec<i64>) -> Vec<(i64, i64)> {
    n.reverse();
    let mut c = 1i64;
    let mut b: Option<i64> = None;
    let mut d: Vec<(i64, i64)> = Vec::new();
    for a in n {
        if b == Some(a) {
            c += 1;
        } else {
            c = 1;
        }
        d.push((a, c));
        b = Some(a);
    }
    d.reverse();
    b = None;
    let mut f = Vec::new();
    for (e, g) in d {
        if b != Some(e) {
            if g == 1 {
                f.push((e, 1));
            } else {
                f.push((e, g));
            }
        }
        b = Some(e);
    }
    f
}

pub fn classify(mod_value: i64) -> Option<&'static str> {
    match mod_value {
        0 => Some("ja"),
        1 => Some("Gegenteil"),
        2 => Some("ähnlich"),
        3 => Some("entferntes Gegenteil"),
        4 => Some("entfernt ähnlich"),
        _ => None,
    }
}

fn classify_for_f_string(mod_value: i64) -> &'static str {
    classify(mod_value).unwrap_or("None")
}

#[allow(non_snake_case)]
pub fn moduloA_text<T: ToString>(zahlen: &[T]) -> String {
    let mut out = String::new();
    for arg in zahlen {
        let arg_text = arg.to_string();
        let Ok(arg_int) = arg_text.parse::<i64>() else {
            continue;
        };
        for var in 2..26 {
            let mod_value = arg_int % var;
            let complement = var - mod_value;
            out.push_str(&format!(
                "{arg_text} % {var} = {mod_value} {}, {complement} {}\n",
                classify_for_f_string(mod_value),
                classify_for_f_string(complement)
            ));
        }
    }
    out
}

#[allow(non_snake_case)]
pub fn moduloA<T: ToString>(zahlen: &[T]) {
    print!("{}", moduloA_text(zahlen));
}

#[cfg(test)]
mod rust_center_tests {
    use super::*;

    #[test]
    fn generator_literals_match_python_shape() {
        assert_eq!(
            strAsGeneratorToListOfNumStrs("[1,2,2]"),
            Some(BTreeSet::from([1, 2]))
        );
        assert_eq!(
            strAsGeneratorToListOfNumStrs("(3,4)"),
            Some(BTreeSet::from([3, 4]))
        );
        assert_eq!(strAsGeneratorToListOfNumStrs("{}"), Some(BTreeSet::new()));
        assert_eq!(strAsGeneratorToListOfNumStrs(" [1]"), None);
    }

    #[test]
    fn zeilen_validation_uses_python_comma_rule() {
        assert!(isZeilenAngabe("1-3,[5,6],"));
        assert!(isZeilenAngabe("v2,3+1"));
        assert!(!isZeilenAngabe(""));
        assert!(isZeilenBruchAngabe("1/2,3/4,"));
        assert!(isZeilenBruchOrGanzZahlAngabe("1/2,3"));
    }

    #[test]
    fn bereich_to_numbers_core_cases_follow_python() {
        assert_eq!(
            BereichToNumbers2("1-3", false, 1028, false),
            BTreeSet::from([1, 2, 3])
        );
        assert_eq!(
            BereichToNumbers2("5+1", false, 1028, false),
            BTreeSet::from([4, 6])
        );
        assert_eq!(
            BereichToNumbers2("1-3,-2", false, 1028, false),
            BTreeSet::from([1, 3])
        );
        assert_eq!(
            BereichToNumbers2("v2", false, 10, false),
            BTreeSet::from([2, 4, 6, 8, 10])
        );
    }

    #[test]
    fn remaining_center_number_helpers_follow_python_shapes() {
        assert_eq!(
            chunks(&[1, 2, 3, 4, 5], 2),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
        assert!(textHatZiffer("abc2"));
        assert_eq!(multiples(12, true), vec![(4, 3), (6, 2), (12, 1)]);
        assert_eq!(teiler("12").1, BTreeSet::from([2, 3, 4, 6, 12]));
        assert_eq!(primfaktoren(24, false), vec![2, 2, 2, 3]);
        assert_eq!(primfaktoren(24, true), vec![2, 2, 2, 3]);
        assert_eq!(
            primRepeat(vec![2, 2, 2, 3]),
            vec!["2^3".to_string(), "3".to_string()]
        );
        assert_eq!(primRepeat2(vec![2, 2, 2, 3]), vec![(2, 3), (3, 1)]);
    }

    #[test]
    fn invert_dict_b_and_modulo_text_keep_python_display_contract() {
        let mut source = BTreeMap::new();
        source.insert("a".to_string(), vec![1, 2]);
        source.insert("b".to_string(), vec![1]);
        let mut expected = BTreeMap::new();
        expected.insert(1, vec!["a".to_string(), "b".to_string()]);
        expected.insert(2, vec!["a".to_string()]);
        assert_eq!(invert_dict_B(&source), expected);

        let text = moduloA_text(&["5"]);
        assert!(text.starts_with("5 % 2 = 1 Gegenteil, 1 Gegenteil\n"));
        assert!(text.contains("5 % 5 = 0 ja, 5 None\n"));
    }
}
