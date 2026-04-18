#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import sys
from copy import copy, deepcopy
from enum import Enum
from typing import Iterable, Optional, Union

import lib4tables_Enum
from center import (
    BereichToNumbers2,
    Multiplikationen,
    alxp,
    cliout,
    getTextWrapThings,
    i18n,
    infoLog,
    isZeilenAngabe,
    output,
    re,
    teiler,
    x,
    primRepeat,
    primfaktoren,
    primRepeat2
)
from lib4tables import isPrimMultiple, moonNumber, primFak
from lib4tables_Enum import ST

shellRowsAmount, h_de, dic, fill = getTextWrapThings()


def setShellRowsAmount(shellRowsAmount2: Optional[str]):
    global shellRowsAmount
    shellRowsAmount = shellRowsAmount2


class Wraptype(Enum):
    pyphen = 1
    pyhyphen = 2
    nohyphen = 3


wrappingType: Wraptype = Wraptype.pyhyphen
# wrappingType: Wraptype = Wraptype.nohyphen
# wrappingType: Wraptype = Wraptype.pyphen


def chunks(lst, n):
    """Yield successive n-sized chunks from lst."""
    for i in range(0, len(lst), n):
        yield lst[i : i + n]


def splitMoreIfNotSmall(textList: list, lenToBe: int) -> tuple:
    newList: list = []
    neededToBeDoneAtAll = False
    lenToBe -= 0
    for k, text in enumerate(textList):
        if len(text) > lenToBe:
            neededToBeDoneAtAll = True
    if neededToBeDoneAtAll:
        for k, text in enumerate(textList):
            if len(text) > lenToBe:
                newList += list(chunks(text, lenToBe))
            else:
                newList += [text]
    if neededToBeDoneAtAll:
        return tuple(newList)
    else:
        return tuple(textList)


def alxwrap(text: str, len_: int):
    global wrappingType
    """ Ich könnte hier beschleunigen, indem ich funktionszeiger verwende,
    anstelle jedes mal hier ein if durch gehen zu lassen
    """
    try:
        fill
    except NameError:
        return (text,)
    if "Brython" in sys.version.split():
        return (text,)
    try:
        # print("A")
        # print(wrappingType)
        return (
            dic.wrap(text, len_)
            if wrappingType == Wraptype.pyphen and len_ != 0
            else (
                splitMoreIfNotSmall(
                    fill(text, width=len_, use_hyphenator=h_de).split("\n"), len_
                )
                if wrappingType == Wraptype.pyhyphen and len_ != 0
                else (text,)
            )
        )
    except:
        # print("B")
        # print(wrappingType)
        return (
            dic.wrap(text, len_)
            if wrappingType == Wraptype.pyhyphen and len_ != 0
            else (
                splitMoreIfNotSmall(
                    fill(text, width=len_, use_hyphenator=h_de).split("\n"), len_
                )
                if wrappingType == Wraptype.pyphen and len_ != 0
                else (text,)
            )
        )


class Prepare:
    def __init__(self, tables, hoechsteZeile):
        global shellRowsAmount, h_de, dic, fill
        shellRowsAmount, h_de, dic, fill = getTextWrapThings()
        self.tables = tables
        self.hoechsteZeile = tables.hoechsteZeile
        self.originalLinesRange = range(tables.hoechsteZeile[1024] + 4)
        self.shellRowsAmount = shellRowsAmount
        self.zaehlungen = [
            0,
            {},
            {},
            {},
            {},
        ]  # Strukturangaben zur Zeile wegen Mondzahlen und Sonnenzahlen
        self.religionNumbers = 0
        self.gezaehlt = False
        self.ifZeilenSetted = False

    def setZaehlungen(
        self, num: int
    ):  # mehrere Zählungen finden festlegen zum später auslesen
        """Eine Zahl wird untersucht und die Variable self.zaehlungen wegen dieser Ergänzt
        self.zaehlungen bekommt informationen über mondzahlen und sonnenzahlen
        i ist eine zu Untersuchende Zahl kleinergeich num
        self.zaehlungen[4][i] bekommt die mondtypen, d.h. (Basis, Exponent) immer
        self.zaehlungen[1][zaehlung] welche zählung fängt mit welcher Zahl an
        self.zaehlungen[2][i] ist welcher Zählung es ist für eine beliebige Zahl: 1 ist 1-4, 2 ist 5-9, 3 ist 10-16
        self.zaehlungen[3][i] ist auch welche Zählung es ist für eine beliebige Zahl: 1 ist 1-4, 2 ist 5-9, 3 ist 10-16
        self.zaehlungen[0] ist bis zu welcher Zahl diese Untersuchung beim letzten Mal durchgeführt wurde
        self.zaehlungen  # [bis zu welcher zahl, {zaehlung:zahl},{zahl:zaehlung},{jede zahl,zugehoerigeZaehlung}]

        @type num: int
        @param num: zu untersuchende Zahl
        @rtype: kein Typ
        @return: nichts
        """
        num = self.originalLinesRange[-1]
        if self.gezaehlt:
            return
        else:
            self.gezaehlt = True
        wasMoon: bool = True
        if self.zaehlungen[0] == 0:
            isMoon = True
        else:
            isMoon = moonNumber(self.zaehlungen[0])[0] != []

        for i in range(int(self.zaehlungen[0]) + 1, num + 1):
            wasMoon = isMoon
            moonType = moonNumber(int(i))
            isMoon = moonType[0] != []
            if wasMoon and not isMoon:
                isMoon = False
                self.zaehlungen[1][len(self.zaehlungen[1]) + 1] = i
                self.zaehlungen[2][i] = len(self.zaehlungen[2]) + 1
            self.zaehlungen[3][i] = len(self.zaehlungen[2])
            self.zaehlungen[4][i] = moonType

    @property
    def breitenn(self):
        return self.breiten

    @breitenn.setter
    def breitenn(self, value: bool):
        self.breiten = value

    @property
    def nummeriere(self):
        """# Nummerierung der Zeilen, z.B. Religion 1,2,3"""
        return self.nummerierung

    @nummeriere.setter
    def nummeriere(self, value):
        self.nummerierung = value

    @property
    def textWidth(self):
        return self.textwidth

    @textWidth.setter
    def textWidth(self, value):
        self.textwidth = value

    def wrapping(self, text: str, length: int) -> list:
        """Hier wird der Zeilenumbruch umgesetzt

        @type text: str
        @param text: Der Text dessen Zeilen umbgebrochen werden sollen
        @type lenght: int
        @param lenght: ab welcher Zeilenlänge umgebrochen werden soll
        @rtype: list
        @return: Liste aus umgebrochenen Teilstrings
        """
        if len(text) > length and length != 0:
            isItNone = alxwrap(text, length)
        else:
            isItNone = None
        return isItNone

    def setWidth(self, rowToDisplay: int, combiRows1: int = 0) -> int:
        if self.shellRowsAmount == 0:
            return 0
        combiRows = combiRows1 if combiRows1 != 0 else len(self.rowsAsNumbers)
        if len(self.rowsAsNumbers) - combiRows < len(self.breiten):
            breiten: list = self.breiten[len(self.rowsAsNumbers) - combiRows :]
        else:
            breiten: list = []
        delta = -1
        if rowToDisplay + delta < len(breiten) and rowToDisplay + delta >= 0:
            certaintextwidth = breiten[rowToDisplay + delta]
        else:
            certaintextwidth = self.textwidth
        return certaintextwidth

    def parametersCmdWithSomeBereich(
        self,
        MehrereBereiche: str,
        symbol: str,
        neg: str,
        keineNegBeruecksichtigung: bool = False,
    ) -> set:
        """Erstellen des Befehls: Bereich

        @type MehrereBereiche: str
        @param MehrereBereiche: der Bereich von bis
        @type symbol: str
        @param symbol: welche Art Bereich soll es werden, symbol typisiert den Bereich
        @type neg: string
        @param neg: Vorzeichen, wenn es darum geht dass diese Zeilen nicht angezeigt werden sollen
        @rtype: set
        @return: Alle Zeilen die dann ausgegeben werden sollen
        """
        results = set()
        if keineNegBeruecksichtigung:
            if isZeilenAngabe(MehrereBereiche):
                results.add("".join(["_", symbol, "_", MehrereBereiche]))
        else:
            for EinBereich in MehrereBereiche.split(","):
                if (
                    (neg == "" and len(EinBereich) > 0 and EinBereich[0] != "-")
                    or (neg == EinBereich[: len(neg)] and len(neg) > 0)
                ) and len(EinBereich) > 0:
                    EinBereich = (
                        EinBereich[len(neg) :]
                        if neg == EinBereich[: len(neg)]
                        else EinBereich
                    )
                    if isZeilenAngabe(EinBereich):
                        results.add("".join(["_", symbol, "_", EinBereich]))
        return results

    #    def parametersCmdWithSomeBereich(
    #        self, MehrereBereiche: str, symbol: str, neg: str
    #    ) -> set:
    #        """Erstellen des Befehls: Bereich
    #
    #        @type MehrereBereiche: str
    #        @param MehrereBereiche: der Bereich von bis
    #        @type symbol: str
    #        @param symbol: welche Art Bereich soll es werden, symbol typisiert den Bereich
    #        @type neg: string
    #        @param neg: Vorzeichen, wenn es darum geht dass diese Zeilen nicht angezeigt werden sollen
    #        @rtype: set
    #        @return: Alle Zeilen die dann ausgegeben werden sollen
    #        """
    #
    #        results = set()
    #        for EinBereich in MehrereBereiche.split(","):
    #            if (
    #                (neg == "" and len(EinBereich) > 0 and EinBereich[0].isdecimal())
    #                or (neg == EinBereich[: len(neg)] and len(neg) > 0)
    #            ) and len(EinBereich) > 0:
    #                EinBereich = (
    #                    EinBereich[len(neg) :]
    #                    if neg == EinBereich[: len(neg)]
    #                    else EinBereich
    #                )
    #                if EinBereich.isdecimal():
    #                    EinBereich = EinBereich + "-" + EinBereich
    #                BereichCouple = EinBereich.split("-")
    #                if (
    #                    len(BereichCouple) == 2
    #                    and BereichCouple[0].isdecimal()
    #                    and BereichCouple[0] != "0"
    #                    and BereichCouple[1].isdecimal()
    #                    and BereichCouple[1] != "0"
    #                ):
    #                    results.add(
    #                        "".join([BereichCouple[0], "-", symbol, "-", BereichCouple[1]])
    #                    )
    #
    #        return results

    def deleteDoublesInSets(self, set1: set, set2: set) -> Iterable[Union[set, set]]:
        """Wenn etwas in 2 Mengen doppelt vorkommt wird es gelöscht
        @rtype: tuple[set,set]
        @return: Beide Mengen werden ausgegeben
        """
        intersection = set1 & set2
        return set1 - intersection, set2 - intersection

    def fromUntil(self, a) -> tuple:
        """2 Zahlen sollen ein ordentlicher Zahlenbereich sein, sonst werden sie es

        @rtype: tuple[int,int]
        @return: Eine Bereichsangabe
        """
        if a[0].isdecimal():
            a[0] = int(a[0])
            if len(a) == 2 and a[1].isdecimal():
                a[1] = int(a[1])
            elif len(a) == 1:
                swap = a[0]
                a[0] = 1
                a += [swap]
                a[0] = 1
            else:
                return (1, 1)
            return tuple(a)
        else:
            return (1, 1)

    def zeileWhichZaehlung(self, zeile: int) -> int:
        return self.zaehlungen[3][zeile]

    # ich wollte je pro extra num, nun nicht mehr nur sondern modular ein mal alles und dann pro nummer in 2 funktionen geteilt
    def moonsun(
        self, MoonNotSun: bool, numRangeYesZ: set, numRange, ifZaehlungenAtAll=True
    ):
        if not ifZaehlungenAtAll:
            self.setZaehlungen(self.originalLinesRange[-1])
        for n in numRange:
            if (self.zaehlungen[4][n][0] != []) == MoonNotSun:
                numRangeYesZ.add(n)
        return numRangeYesZ

    def FilterOriginalLines(self, numRange: set, paramLines: set) -> set:
        """Hier werden die Befehle der Angabe welche Zeilen angezeigt werden in konkrete Zeilen umgewandelt.

        @type results: Menge
        @param set: Bereiche von Zeilen einer Art: Anzeigen, ja, nein, von woanders, etc.
        @rtype: set
        @return: Mehrere Bereichsbezeichnugen
        """
        numRange -= {0}

        def cutset(wether, a: set, b: set) -> set:
            if wether:
                # result = a.intersection(b)
                result = a & b
                if result is None:
                    return set()
                else:
                    return result
            return a

        if (
            "all" in paramLines
            or len(set(paramLines) - {"ka", "ka2"}) == 0
            or not self.ifZeilenSetted
        ):
            numRange = set(range(1, self.hoechsteZeile[1024] + 1))
        else:
            numRange = set()
            # return set(self.originalLinesRange)

        # numRangeYesZ = set()
        if_a_AtAll = False
        mehrere: list = []
        ifTeiler = False

        for condition in paramLines:
            if "_a_" in condition[:3] and len(condition) > 3:
                if_a_AtAll = True
                mehrere += [condition[3:]]
            if condition[:3] == "_w_":
                ifTeiler = True
        if if_a_AtAll:
            numRange |= BereichToNumbers2(
                ",".join(mehrere), False, self.hoechsteZeile[1024] + 1
            )

            if ifTeiler:
                numRange |= teiler(",".join([str(c) for c in numRange]))[1]

            if len(numRange) != 0:
                mehrere = (",".join(mehrere)).split(",")
                for eins in mehrere:
                    ja1, ja2 = eins[:1] == "-", eins[:2] == i18n.befehle2["v"] + "-"
                    if ja1 or ja2:
                        if ja1:
                            eins = eins[1:]
                        if ja2:
                            eins = i18n.befehle2["v"] + eins[2:]
                        numRange -= BereichToNumbers2(
                            eins, False, self.hoechsteZeile[1024] + 1
                        )

        if_b_AtAll = False
        mehrere = []
        numRangeYesZ: set = set()
        for condition in paramLines:
            if "_b_" in condition[:3] and len(condition) > 3:
                if_b_AtAll = True
                mehrere += [condition[3:]]
        if if_b_AtAll:
            if len(numRange) == 0 and not if_a_AtAll and "all" not in paramLines:
                numRange = set(range(1, self.hoechsteZeile[114] + 1))

            numRangeYesZ |= BereichToNumbers2(
                ",".join(mehrere), True, self.hoechsteZeile[114] + 1
            )

            if len(numRangeYesZ) != 0:
                numRange &= numRangeYesZ

            if len(numRange) != 0:
                mehrere = (",".join(mehrere)).split(",")
                for eins in mehrere:
                    ja1, ja2 = eins[:1] == "-", eins[:2] == i18n.befehle2["v"] + "-"
                    if ja1 or ja2:
                        if ja1:
                            eins = eins[1:]
                        if ja2:
                            eins = i18n.befehle2["v"] + eins[2:]
                        numRange -= BereichToNumbers2(
                            eins, True, self.hoechsteZeile[1024] + 1
                        )

        numRangeYesZ = set()
        ifZeitAtAll = False
        for condition in paramLines:
            if "=" == condition:
                ifZeitAtAll = True
                numRangeYesZ |= {10}
            elif "<" == condition:
                ifZeitAtAll = True
                numRangeYesZ |= set(range(1, 10))
            elif ">" == condition:
                ifZeitAtAll = True
                numRangeYesZ |= set(range(11, self.hoechsteZeile[1024] + 1))
        if ifZeitAtAll:
            if (
                len(numRange) == 0
                and not if_b_AtAll
                and not if_a_AtAll
                and "all" not in paramLines
                and len(numRangeYesZ) == 0
            ):
                numRange = set(range(1, self.hoechsteZeile[1024] + 1))
            if if_a_AtAll or "all" in paramLines or if_b_AtAll:
                numRange &= numRangeYesZ
            else:
                numRange |= numRangeYesZ

        numRangeYesZ = set()
        ifZaehlungenAtAll = False
        mehrere = []
        for condition in paramLines:
            if "_n_" in condition[:3] and len(condition) > 3:
                numRangeYesZ |= BereichToNumbers2(
                    condition[3:], False, self.hoechsteZeile[1024] + 1
                )
                ifZaehlungenAtAll = True
                mehrere += [condition[3:]]
        if True or ifZaehlungenAtAll:
            self.setZaehlungen(self.originalLinesRange[-1])
        if ifZaehlungenAtAll:
            # self.setZaehlungen(self.originalLinesRange[-1])
            numRangeYesZ2 = set()
            if (
                len(numRange) == 0
                and not if_a_AtAll
                and not if_b_AtAll
                and "all" not in paramLines
            ):
                numRange = set(range(1, self.hoechsteZeile[1024] + 1))
            for n in numRange:  # nur die nummern, die noch infrage kommen
                for z in numRangeYesZ:
                    # 1-4:1,5-9:2 == jetzt ?
                    if self.zaehlungen[3][n] == int(z):
                        numRangeYesZ2 |= {n}
                        # numRange.remove(n)
            if ifZaehlungenAtAll:
                if len(numRangeYesZ2) > 0 and len(numRange) != 0:
                    numRange &= numRangeYesZ2
                elif len(numRange) == 0:
                    numRange |= numRangeYesZ2
            alxp(len(numRange))
            if len(numRange) != 0:
                mehrere = (",".join(mehrere)).split(",")
                # print(mehrere)
                minusBereiche = set()
                for eins in mehrere:
                    ja1, ja2 = eins[:1] == "-", eins[:2] == i18n.befehle2["v"] + "-"
                    if ja1 or ja2:
                        if ja1:
                            eins = eins[1:]
                        if ja2:
                            eins = i18n.befehle2["v"] + eins[2:]
                        minusBereiche |= BereichToNumbers2(
                            eins, False, self.hoechsteZeile[1024] + 1
                        )
                if len(minusBereiche) > 0:
                    for n in copy(numRange):
                        for z in minusBereiche:
                            if self.zaehlungen[3][n] == int(z):
                                numRange -= {n}

            # set().add
            # exit()
        # ANFANG
        ifTypAtAll = False
        numRangeYesZ = set()
        if len(numRange) == 0 and len(set(paramLines) - {"ka", "ka2"}) > 0:
            numRange = set(range(1, self.hoechsteZeile[1024] + 1))

        if (
            len(
                {"aussenerste", "innenerste", "aussenalle", "innenalle"}
                & set(paramLines)
            )
            > 0
        ):
            primList = {}
            innenAussen = {}
            innenAussen[1] = (True, False, True)
            numRangeB = numRange - {1, 2, 3}
            for n in numRangeB:
                primList[n] = primFak(n)
            for anfangsZahl, primZahlen in primList.items():
                NurEineZahl = len(primZahlen) == 1
                einFachVorkommen = NurEineZahl
                innenAussen[anfangsZahl] = (False, False, NurEineZahl)
                innen, aussen = False, False
                for primZahl in primZahlen:
                    if primZahl not in range(4):
                        innenOrAussen = primZahl % 6
                        innen = innenOrAussen == 1 or innen
                        aussen = innenOrAussen == 5 or aussen
                innenAussen[anfangsZahl] = (innen, aussen, einFachVorkommen)
            if len({"aussenerste"} & set(paramLines)) > 0:
                numRangeYesZ |= set(
                    (
                        anfangsZahl
                        for anfangsZahl, Tupel in innenAussen.items()
                        if Tupel[0] and Tupel[2]
                    )
                )
            if len({"innenerste"} & set(paramLines)) > 0:
                numRangeYesZ |= set(
                    (
                        anfangsZahl
                        for anfangsZahl, Tupel in innenAussen.items()
                        if Tupel[1] and Tupel[2]
                    )
                )
            if len({"aussenalle"} & set(paramLines)) > 0:
                numRangeYesZ |= set(
                    (
                        anfangsZahl
                        for anfangsZahl, Tupel in innenAussen.items()
                        if Tupel[0]
                    )
                )
            if len({"innenalle"} & set(paramLines)) > 0:
                numRangeYesZ |= set(
                    (
                        anfangsZahl
                        for anfangsZahl, Tupel in innenAussen.items()
                        if Tupel[1]
                    )
                )
            ifTypAtAll = len(numRangeYesZ) > 0
            numRange = cutset(ifTypAtAll, numRange, numRangeYesZ)
        # ENDE

        ifTypAtAll = False
        numRangeYesZ = set()
        if len(numRange) == 0 and len(set(paramLines) - {"ka", "ka2"}) > 0:
            numRange = set(range(1, self.hoechsteZeile[1024] + 1))

        for condition in paramLines:
            if "mond" in condition:
                numRangeYesZ, ifTypAtAll = (
                    self.moonsun(True, numRangeYesZ, numRange, ifZaehlungenAtAll),
                    True,
                )
            elif "schwarzesonne" in condition:
                ifTypAtAll = True
                for n in numRange:
                    if n % 3 == 0:
                        numRangeYesZ.add(n)
            elif "sonne" in condition:
                numRangeYesZ, ifTypAtAll = (
                    self.moonsun(False, numRangeYesZ, numRange, ifZaehlungenAtAll),
                    True,
                )
            elif "planet" in condition:
                ifTypAtAll = True
                for n in numRange:
                    if n % 2 == 0:
                        numRangeYesZ.add(n)
            elif "SonneMitMondanteil" in condition:
                ifTypAtAll = True
                for n in numRange:
                    booleans = {Faktor == 1 for primZahl, Faktor in primRepeat2(primfaktoren(n))}
                    # print(list(booleans))
                    if len({True, False} & booleans) > 1:
                        numRangeYesZ.add(n)

        numRange = cutset(ifTypAtAll, numRange, numRangeYesZ)

        primMultiples: list = []
        ifPrimAtAll = False
        for condition in paramLines:
            if len(condition) > 1 and condition[-1] == "p":
                ifPrimAtAll = True
                primMultiples += [int(condition[:-1])]

        numRangeYesZ = set()
        if ifPrimAtAll:
            if (
                len(numRange) == 0
                and not if_b_AtAll
                and not if_a_AtAll
                and "all" not in paramLines
                and not ifTypAtAll
            ):
                numRange = set(range(1, self.hoechsteZeile[1024] + 1))

            for n in numRange:
                if isPrimMultiple(n, primMultiples):
                    numRangeYesZ.add(n)
            numRange = cutset(ifPrimAtAll, numRange, numRangeYesZ)

        toPowerIt: list = []
        ifPowerAtall: bool = False

        for condition in paramLines:
            if "_^_" in condition[:3] and len(condition) > 3:
                ifPowerAtall = True
                mehrere += [condition[3:]]
        toPowerIt = list(BereichToNumbers2(",".join(mehrere)))
        #
        #
        if ifPowerAtall:
            numRangeYesZ = set()
            if len(numRange) == 0 and len(set(paramLines) - {"ka", "ka2"}) > 0:
                numRange = set(range(1, self.hoechsteZeile[1024] + 1))
            if len(numRange) > 0:
                lastEl = list(numRange)
                lastEl.sort()
                lastEl = lastEl[-1]
                for base in toPowerIt:
                    for n in range(lastEl):
                        onePower = pow(base, n)
                        # if onePower <= numRangeMax:
                        numRangeYesZ |= {onePower}
                        # else:
                        #    break
                numRange = cutset(ifPowerAtall, numRange, numRangeYesZ) - {1}

        numRangeYesZ = set()

        ifMultiplesFromAnyAtAll = False
        anyMultiples = []
        for condition in paramLines:
            if (
                len(condition) > 1
                and condition[-1] == "v"
                and condition[:-1].isdecimal()
            ):
                ifMultiplesFromAnyAtAll = True
                anyMultiples += [int(condition[:-1])]

        if ifMultiplesFromAnyAtAll:
            numRangeYesZ = set()
            for n in numRange:
                for divisor in anyMultiples:
                    if n % divisor == 0:
                        numRangeYesZ.add(n)
            numRange = cutset(ifMultiplesFromAnyAtAll, numRange, numRangeYesZ)

        # über 114 die Sonnen weg
        for n in copy(numRange - {0}):
            if (self.zaehlungen[4][n][0] == []) and (
                n > self.tables.hoechsteZeile[114]
            ):
                numRange.remove(n)
        invertieren = False
        for condition in paramLines:
            if condition[:3] == "_i_":
                invertieren = True
        if invertieren:
            numRangeList = list(numRange)
            numRangeList.sort()
            h = self.tables.hoechsteZeile[1024]
            numRange2Set = set()
            for i in range(1, h+1):
                if (i+1 in numRange or i-1 in numRange) and not i in numRange:
                    numRange2Set |= {i}
            numRange = set(numRange2Set)
        numRangeList = list(numRange)
        numRangeList.sort()
        numRange2Map = {i + 1: a for i, a in enumerate(numRangeList)}
        zJa = False
        numRangeNeu2 = set()
        for condition in paramLines:
            if "_z_" in condition[:3] and len(condition) > 3:
                zJa = True
                NumRangeNeu = set(numRange2Map.keys()) & BereichToNumbers2(
                    condition[3:], False, self.hoechsteZeile[1024] + 1
                )
                for a in NumRangeNeu:
                    numRangeNeu2 |= {numRange2Map[a]}
        if zJa:
            numRange &= numRangeNeu2
        yJa = False
        numRangeNeu2 = set()
        for condition in paramLines:
            if "_y_" in condition[:3] and len(condition) > 3:
                yJa = True
                NumRangeNeu = set(numRange2Map.keys()) & BereichToNumbers2(
                    condition[3:], True, self.hoechsteZeile[1024] + 1
                )
                for a in NumRangeNeu:
                    numRangeNeu2 |= {numRange2Map[a]}
        if yJa:
            numRange &= numRangeNeu2

        return numRange

    def prepare4out(
        self,
        paramLines: set,
        paramLinesNot: set,
        contentTable: list,
        rowsAsNumbers: set,
        gebrSpalten: dict,
        combiRows: int = 0,
        reliTableLenUntilNow=None,
        primSpalten: set = None,
        # gebrGalSpalten: set = None,
        # gebrUnivSpalten2: set = None,
        # gebrGalSpalten2: set = None,
        # gebrEmoSpalten: set = None,
        # gebrEmoSpalten2: set = None,
        # gebrGroeSpalten: set = None,
        # gebrGroeSpalten2: set = None,
        kombiCSVNumber: int = 0,
    ) -> tuple:
        """Aus einer Tabelle wird eine gemacht, bei der der Zeilenumbruch durchgeführt wird.
        Dabei werden alle Spalten und Zeilen entfernt die nicht ausgegeben werden sollen.

        @type paramLines: set
        @param paramLines: welche Linien ja, andere fallen weg
        @type paramLinesNot: set
        @param paramLinesNot: welche Linien nein, werden abgezogen von ja
        @type contentTable: list
        @param contentTable: die Tabelle die verändert werden soll
        @type rowsAsNumberss: set
        @param rowsAsNumberss: anzuzeigende Spalten
        @rtype: tuple[set,set,int,range,list]
        @return: Zeilen die ausgegeben werden sollen, neue Tabelle, Nummer der letzten Zeile , \
            range aus zu zeigenden Spalten 1-n nicht alle , welche neuen Spalten welche alten waren und umgekehrt
        return finallyDisplayLines, newerTable, numlen, rowsRange, old2Rows
        """
        # x("gebrSpaltenF", gebrSpalten)
        (
            finallyDisplayLines,
            headingsAmount,
            newerTable,
            numlen,
            rowsRange,
        ) = self.prepare4out_beforeForLoop_SpaltenZeilenBestimmen(
            contentTable, paramLines, paramLinesNot
        )

        self.headingsAmount = headingsAmount
        old2Rows: tuple = ({}, {})
        reliNumbersBool = False if self.religionNumbers != [] else True
        for u, line in enumerate(contentTable):
            if u in finallyDisplayLines or combiRows != 0:
                # x("gebrSpaltenE", gebrSpalten)
                new2Lines = self.prepare4out_LoopBody(
                    combiRows,
                    gebrSpalten,
                    headingsAmount,
                    line,
                    old2Rows,
                    primSpalten,
                    reliNumbersBool,
                    reliTableLenUntilNow,
                    rowsAsNumbers,
                    u,
                    kombiCSVNumber=kombiCSVNumber,
                )

                if new2Lines != []:
                    newerTable += [new2Lines]

        return finallyDisplayLines, newerTable, numlen, rowsRange, old2Rows

    def prepare4out_beforeForLoop_SpaltenZeilenBestimmen(
        self, contentTable, paramLines, paramLinesNot
    ):
        newerTable: list = []
        if len(contentTable) > 0:
            headingsAmount = len(contentTable[0])
            rowsRange = range(headingsAmount)
        else:
            headingsAmount = 0
            rowsRange = range(0)
        #
        finallyDisplayLines: set = self.FilterOriginalLines(
            set(self.originalLinesRange), paramLines
        )
        if len(paramLinesNot) != 0:
            finallyDisplayLines2 = self.FilterOriginalLines(
                deepcopy(finallyDisplayLines), paramLinesNot
            )
            hasAnythingCanged = (
                set(self.originalLinesRange) - finallyDisplayLines2 - {0}
            )
            if len(hasAnythingCanged) > 0:
                finallyDisplayLines -= finallyDisplayLines2

        if len(finallyDisplayLines) == 0:
            if self.ifZeilenSetted:
                finallyDisplayLines = set()
            else:
                finallyDisplayLines = set(range(self.hoechsteZeile[1024] + 1))

        finallyDisplayLines.add(0)
        finallyDisplayLines3: list = list(finallyDisplayLines)
        finallyDisplayLines3.sort()
        finallyDisplayLines = set(finallyDisplayLines3)
        #    maxPartLineLen = 0
        numlen = len(str(finallyDisplayLines3[-1]))

        return finallyDisplayLines, headingsAmount, newerTable, numlen, rowsRange

    def prepare4out_LoopBody(
        self,
        combiRows,
        gebrSpalten,
        headingsAmount,
        line,
        old2Rows,
        primSpalten,
        reliNumbersBool,
        reliTableLenUntilNow,
        rowsAsNumbers,
        u,
        kombiCSVNumber,
    ):
        if reliNumbersBool:
            self.religionNumbers += [int(u)]
        new2Lines: list = []
        rowToDisplay = 0
        h = 0
        newLines: list = [[]] * headingsAmount
        for t, cell in enumerate(line):
            if t in rowsAsNumbers:
                if u == 0:
                    self.prepare4out_Tagging(
                        combiRows,
                        gebrSpalten,
                        primSpalten,
                        reliTableLenUntilNow,
                        rowToDisplay,
                        t,
                        kombiCSVNumber=kombiCSVNumber,
                    )

                rowToDisplay += 1
                certaintextwidth = self.setWidth(rowToDisplay, combiRows)
                self.certaintextwidth = certaintextwidth

                into = self.cellWork(cell, certaintextwidth)
                if into != [""] or True:
                    new2Lines += [into]
                if u == 0:
                    old2Rows[0][t] = h
                    old2Rows[1][h] = t
                h += 1
        return new2Lines

    def prepare4out_Tagging(
        self,
        combiRows,
        gebrSpalten,
        primSpalten,
        reliTableLenUntilNow,
        rowToDisplay,
        t,
        kombiCSVNumber,
    ):
        if combiRows == 0:
            try:
                if rowToDisplay not in self.tables.generatedSpaltenParameter:
                    # x("JAHA",[self.tables.dataDict[0][t],t])
                    self.tables.generatedSpaltenParameter[
                        rowToDisplay
                    ] = self.tables.dataDict[0][t]
                    self.tables.generatedSpaltenParameter_Tags[
                        rowToDisplay
                    ] = lib4tables_Enum.tableTags2[t]
                elif primSpalten is not None and t in primSpalten:
                    self.tables.generatedSpaltenParameter_Tags[
                        rowToDisplay
                    ] = frozenset({ST.sternPolygon, ST.universum, ST.galaxie})
                elif (gebrSpalten["Gal"] is not None and t in gebrSpalten["Gal"]) or (
                    gebrSpalten["Gal2"] is not None and t in gebrSpalten["Gal2"]
                ):
                    self.tables.generatedSpaltenParameter_Tags[
                        rowToDisplay
                    ] = frozenset(
                        {
                            ST.sternPolygon,
                            ST.galaxie,
                            ST.gleichfoermigesPolygon,
                            ST.gebrRat,
                        }
                    )
                elif (gebrSpalten["Uni"] is not None and t in gebrSpalten["Uni"]) or (
                    gebrSpalten["Uni2"] is not None and t in gebrSpalten["Uni2"]
                ):
                    self.tables.generatedSpaltenParameter_Tags[
                        rowToDisplay
                    ] = frozenset(
                        {
                            ST.sternPolygon,
                            ST.universum,
                            ST.gleichfoermigesPolygon,
                            ST.gebrRat,
                        }
                    )

                elif (gebrSpalten["Emo"] is not None and t in gebrSpalten["Emo"]) or (
                    gebrSpalten["Emo2"] is not None and t in gebrSpalten["Emo2"]
                ):
                    self.tables.generatedSpaltenParameter_Tags[
                        rowToDisplay
                    ] = frozenset(
                        {
                            ST.sternPolygon,
                            ST.keinParaOdMetaP,
                            ST.gleichfoermigesPolygon,
                            ST.gebrRat,
                        }
                    )

                elif (gebrSpalten["Groe"] is not None and t in gebrSpalten["Groe"]) or (
                    gebrSpalten["Groe2"] is not None and t in gebrSpalten["Groe2"]
                ):
                    self.tables.generatedSpaltenParameter_Tags[
                        rowToDisplay
                    ] = frozenset(
                        {
                            ST.sternPolygon,
                            ST.gleichfoermigesPolygon,
                            ST.gebrRat,
                            ST.keinParaOdMetaP,
                        }
                    )

            except KeyError:
                pass

        else:
            assert kombiCSVNumber in (
                0,
                1,
            )
            try:
                self.tables.generatedSpaltenParameter_Tags[
                    reliTableLenUntilNow + rowToDisplay
                ] = (
                    lib4tables_Enum.tableTags2_kombiTable[t]
                    if kombiCSVNumber == 0
                    else lib4tables_Enum.tableTags2_kombiTable2[t]
                    if kombiCSVNumber == 1
                    else None
                )
            except KeyError:
                pass

    def cellWork(self, cell: str, certaintextwidth: int) -> list:
        """aus String mach Liste aus Strings mit korrektem Zeilenumbruch

        @type cell: str
        @param cell: Text, für in Teilstrings, korrekter Zeilenumbruch!
        @type newLines: list
        @param newLines: voran gegangene Versuche dieser Liste aus Strings
        @type certaintextwidth: int
        @param certaintextwidth: an dieser Stelle Zeilenumbruch
        @type t: int
        @param t: welcher Versuch einer Liste aus strings soll von dieser Funktion zurück gegeben werden
        @rtype: list
        @return: Liste aus Strings mit korrektem Zeilenumbruch
        """
        # certaintextwidth -= 1
        cell = cell.strip()
        isItNone = self.wrapping(cell, certaintextwidth)
        cell2: tuple = tuple()
        rest: str = cell
        if certaintextwidth == 0:
            return [cell]

        while isItNone not in [None, ()]:
            cell2 += isItNone
            isItNone = self.wrapping(cell2[-1], certaintextwidth)
            rest = cell2[-1]
            cell2 = cell2[:-1]
            if len(rest) > certaintextwidth and isItNone is None:
                cell2 += (rest[0:certaintextwidth],)
                isItNone = (rest[certaintextwidth:],)
        else:
            cell2 += (rest[0:certaintextwidth],)
            newLines: list = []
            for k, cellInCells in enumerate(cell2):
                newLines += [cellInCells]
        return newLines
