#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import math
import sys

sys.path.insert(0, "libs")
from collections import OrderedDict
from enum import Enum
from fractions import Fraction
from functools import lru_cache
from center import (
    Multiplikationen,
    alxp,
    cliout,
    getTextWrapThings,
    i18n,
    infoLog,
    multiples,
    output,
    re,
    x,
)

i18n = i18n.lib4tables


class NichtsSyntax:
    def coloredBeginCol(self, num: int, rest: bool = False):
        return ""

    def generateCell(
        self, num: int, dataDict: dict, content=None, zeile=None, tables=None
    ) -> str:
        return ""

    beginTable = ""
    endTable = ""
    beginCell = ""
    endCell = ""
    beginZeile = ""
    endZeile = ""


class OutputSyntax:
    def coloredBeginCol(self, num: int, rest: bool = False):
        return self.beginZeile

    def generateCell(
        self, num: int, dataDict: dict, content=None, zeile=None, tables=None
    ) -> str:
        return self.beginCell

    beginTable = ""
    endTable = ""
    beginCell = ""
    endCell = ""
    beginZeile = ""
    endZeile = ""


class csvSyntax(OutputSyntax):
    beginTable = ""
    endTable = ""
    beginCell = ""
    endCell = ""
    beginZeile = ""
    endZeile = ""


class emacsSyntax(OutputSyntax):
    # print(OutputSyntax)
    # def generateCell(
    #    self, spalte: int, SpaltenParameter: dict, content=None, zeile=None, tables=None
    # ) -> str:
    #    return "|"

    beginTable = ""
    endTable = ""
    beginCell = "|"
    endCell = ""
    beginZeile = ""
    endZeile = "|"


class markdownSyntax(OutputSyntax):
    # print(OutputSyntax)
    # def generateCell(
    #    self, spalte: int, SpaltenParameter: dict, content=None, zeile=None, tables=None
    # ) -> str:
    #    return "|"

    beginTable = ""
    endTable = ""
    beginCell = "|"
    endCell = ""
    beginZeile = ""
    endZeile = "|"


class bbCodeSyntax(OutputSyntax):
    def coloredBeginCol(self, num: int, rest: bool = False):
        num = int(num) if str(num).isdecimal() else 0
        numberType = primCreativity(num)

        if rest:
            # wenn der Fallm eintritt dass es leerer Text ist der frei ist
            return "[tr]"
            if num == 0:
                return "[tr]"
            elif num % 2 == 0:
                return "[tr]"
            else:
                return "[tr]"
        elif numberType == 1:
            if num % 2 == 0:
                return '[tr="background-color:#66ff66;color:#000000;"]'
            else:
                return '[tr="background-color:#009900;color:#ffffff;"]'
        elif numberType == 2 or num == 1:
            if num % 2 == 0:
                return '[tr="background-color:#ffff66;color:#000099;"]'
            else:
                return '[tr="background-color:#555500;color:#aaaaff;"]'
        elif numberType == 3:
            if num % 2 == 0:
                return '[tr="background-color:#9999ff;color:#202000;"]'
            else:
                return '[tr="background-color:#000099;color:#ffff66;"]'
        elif num == 0:
            return '[tr="background-color:#ff2222;color:#002222;"]'

    def generateCell(
        self, spalte: int, SpaltenParameter: dict, content=None, zeile=None, tables=None
    ) -> str:
        spalte = int(spalte)
        spalte += 2
        return "".join(
            (
                "[td",
                (
                    '="background-color:#000000;color:#ffffff"'
                    if content is not None and int(content) % 2 == 0
                    else '="background-color:#ffffff;color:#000000"'
                )
                if spalte == 0
                else '=""',
                "]",
            )
        )

    beginTable = "[table]"
    endTable = "[/table]"
    beginCell = "[td]"
    endCell = "[/td]"
    beginZeile = "[tr]"
    endZeile = "[/tr]"


class htmlSyntax(OutputSyntax):
    def __init__(self):
        self.zeile = 0

    def coloredBeginCol(self, num: int, rest: bool = False) -> str:
        num = int(num) if str(num).isdecimal() else 0
        numberType = primCreativity(num)
        self.zeile = num
        if rest:
            # wenn der Fallm eintritt dass es leerer Text ist der frei ist
            return "<tr>\n"
            if num == 0:
                return "<tr>\n"
            elif num % 2 == 0:
                return "<tr>\n"
            else:
                return "<tr>\n"
        elif numberType == 1:
            if num % 2 == 0:
                return '<tr style="background-color:#66ff66;color:#000000;">\n'
            else:
                return '<tr style="background-color:#009900;color:#ffffff;">\n'
        elif numberType == 2 or num == 1:
            if num % 2 == 0:
                return '<tr style="background-color:#ffff66;color:#000099;">\n'
            else:
                return '<tr style="background-color:#555500;color:#aaaaff;">\n'
        elif numberType == 3:
            if num % 2 == 0:
                return '<tr style="background-color:#9999ff;color:#202000;">\n'
            else:
                return '<tr style="background-color:#000099;color:#ffff66;">\n'
        elif num == 0:
            return '<tr style="background-color:#ff2222;color:#002222;">\n'

    def generateCell(
        self, spalte: int, SpaltenParameter: dict, content=None, zeile=None, tables=None
    ) -> str:
        if zeile == "":
            zeile = 0
        spalte = int(spalte)
        if spalte == -2:
            tupleOfListsOfCouples = (
                [
                    (i18n.zaehlung["Zählung"], ""),
                ],
            )
        elif spalte == -1:
            tupleOfListsOfCouples = (
                [
                    (i18n.nummerier["Nummerierung"], ""),
                ],
            )
        else:
            try:
                # print(SpaltenParameter[spalte])
                tupleOfListsOfCouples = SpaltenParameter[spalte]
            except:
                if str(spalte).isdecimal():
                    raise ValueError
                tupleOfListsOfCouples = (("?", "?"),)
        # damit pypy3 == python3
        things1: OrderedDict[int, list] = OrderedDict()

        listOfListsOfCouples: list = list(tupleOfListsOfCouples)
        # listOfListsOfCouples.sort(
        #    key=lambda x: sorted([(s.upper(), v) for s, v in x])
        # )  # damit pypy3 == python3
        # print(listOfListsOfCouples)
        for c, couples in enumerate(listOfListsOfCouples):
            for paraNum in (0, 1):
                if len(couples[0]) > paraNum:
                    if len(couples[0]) > paraNum:
                        # i = 0
                        para1o2name = couples[0][paraNum]
                        # while (
                        #    len(couples) > i + 1
                        #    and len(couples[i + 1]) > 0
                        #    and para1o2name.strip() == ""
                        # ):
                        #    i += 1
                        #    para1o2name = couples[i][paraNum]
                        if len(para1o2name.strip()) != 0 or True:
                            if paraNum == 1:
                                para1o2name = "".join(["p3_", str(c), "_", para1o2name])
                            try:
                                things1[paraNum] += [para1o2name]
                                # things1[paraNum].sort(key=lambda x: x.upper())

                            except KeyError:
                                things1[paraNum]: list = [
                                    para1o2name,
                                ]

        things: OrderedDict = OrderedDict()  # damit pypy3 == python3

        for key, values in things1.items():
            for i, el in enumerate(values):
                if el != i18n.alles["alles"]:
                    try:
                        things[key] += (
                            "✗" if key == 0 else "",
                            el,
                            ",",
                        )
                    except KeyError:
                        things[key] = (
                            "✗" if key == 0 else "",
                            el,
                            ",",
                        )
            things[key] = "".join(things[key])

        spalte += 2
        if len(things) < 2:
            return ""
        else:
            p4: str
            try:
                p4a = tables.generatedSpaltenParameter_Tags[spalte - 2]
                # print(str(p4a))
                p4b: list = []
                for a in p4a:
                    p4b += [str(a.value)]
                # p4b.sort(key=lambda x: x.upper())
                p4 = ",".join(p4b)
            except KeyError:
                p4 = ""
            except:
                p4 = ""
            return "".join(
                ("<td",)
                + (
                    (
                        ' class="',
                        "z_",
                        str(int(zeile)),
                        " r_",
                        str(spalte),
                        " p1_",
                        things[0],
                        ",",
                        " p2_",
                        (things[1] if len(things) > 1 else ""),
                        " p4_",
                        p4,
                        '"',
                    )
                    if zeile == 0
                    else ()
                )
                + (
                    (
                        ' style="background-color:#000000;color:#ffffff;"'
                        if content is not None and int(content) % 2 == 0
                        else ' style="background-color:#ffffff;color:#000000;"'
                        if spalte == 0
                        else "",
                    )
                    if spalte in (0, 1)
                    # else (' style="display:none"',)
                    # if zeile == 0
                    else (
                        ' class="tdSymbole" style="background-image: url('
                        ');background-size: cover;background-repeat: no-repeat;background-position: right; "',
                    )
                    if "Symbole" in things1[0]
                    else ()
                )
                + (">\n",)
            )

    beginTable = '<table border=0 id="bigtable">'
    endTable = "</table>\n"
    beginCell = "<td>\n"
    endCell = "\n</td>\n"
    # beginZeile = "<tr>"
    beginZeile = ""
    endZeile = "</tr>\n"


# @jit(nopython=True, parallel=False, cache=True)
def moonNumber(num: int):
    """Hier wird der Zeilenumbruch umgesetzt

    @type num: int
    @param num: zu untersuchende Zahl
    @rtype: tuple(list[int],list[int])
    @return: () wenn keine Mondzahl, sonst 2 Listen mit gleicher Länge aus Elementen: Liste1: Basis der Mondzahl, Liste2: Exponent der Mondzahl
    """
    results: list = [1]
    exponent: list = [1]
    exponent.pop()
    results.pop()
    for i in range(2, num):
        oneResult: float = num ** (1 / i)
        if round(oneResult) * 100000 == round(oneResult * 100000):
            results += [int(round(oneResult))]
            exponent += [int(i - 2)]

    return results, exponent


# @jit(nopython=True, parallel=False, cache=True)
def primFak(n: int) -> list:
    """Alle Primfaktoren einer Zahl als Liste mit mehrfachvorkommen, sofern ja

    @type n: int
    @param n: ein natürliche Zahl
    @rtype: list
    @return: alle Primfaktoren, ggf. mit Mehrfachvorkommen
    """
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


# @lru_cache(maxsize=10489)
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


# def getLogarithmOnlyAsPureInt(potenz: int, basis: int) -> int:
#    exponent = math.log(potenz) / math.log(basis)
#    if exponent == round(exponent):
#        return exponent
#    else:
#        return None


@lru_cache(maxsize=10489)
def primRepeat(n: tuple) -> tuple:
    """Primfaktoren werden zusammengefasst in Liste aus Primfaktor hoch n

    @type n:  list
    @param n: Primfaktoren
    @rtype: list[tuple(n1,n2)]
    @return: Liste aus geordneten Paaren mit Primfaktor hoch n
    """
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


def primMultiple(n: int) -> list:
    """Gibt Liste aus geordneten Paaren aus mit Primzahl und Vielfacher der Primzahl aus denen die Zahl n besteht

    @type n: int
    @param n: eine natürliche Zahl, die zu untersuchen ist
    @rtype: list[tuple(primzahl,vielfacher der Primzahl)] oder bool
    @return: Primzahl und dessen Vielfacher, das mehrmals, so viele Primzahlen wie es gibt, aus denen n besteht
    """
    multiples = [(1, n)]
    for prim in primRepeat(tuple(primFak(n))):
        multiples += [(prim[0], round(n / prim[0]))]
    return multiples


def isPrimMultiple(isIt: int, multiples1: list, dontReturnList=True):
    """Ist die Zahl der Vielfache in überhaupt irgendeiner Primzahl

    @type isIt: int
    @param isIt: die Zahl die zu untersuchen ist
    @type multiples1: list
    @param multiple1: Liste an Vielfachern von denen einer zutreffen muss, bei [2,3] ist es True, wenn es das zweifache oder dreifache einer Primzahl ist
    @type dontReturnList: bool
    @param dontReturnList: wenn ja, dann wird nur ausgegeben ob es ein Vielfacher einer Primzahl ist, ansonsten die Liste für welche Vielfacher es zutrifft
    @rtype: list[bool] oder bool
    @return: True wenn Primzahlvielfacher, Liste aus True für ja für welche Multiplikatioren ja
    """
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
