#!/usr/bin/env python3
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
