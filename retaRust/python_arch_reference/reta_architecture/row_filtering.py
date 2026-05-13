#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Zeilenfilter- und Bereichsmorphismen für Reta.

Diese Schicht enthält die frühere tiefe Zeilenlogik aus
``libs.lib4tables_prepare.Prepare``: Bereichsausdrücke, Zählungsgruppen,
Mond/Sonne/Planet-Filter, Prim-Multiple, Potenzen und Positionsfilter werden
hier als explizite Morphismen von Parameter-Sektionen zu konkreten
Zeilenmengen modelliert. ``Prepare`` bleibt nur eine kompatible Fassade.
"""
from __future__ import annotations

from copy import copy
from dataclasses import dataclass
from typing import Iterable, Union

from .runtime_compat import (
    BereichToNumbers2,
    alxp,
    i18n,
    isZeilenAngabe,
    teiler,
    primfaktoren,
    primRepeat2,
)
from .number_theory import isPrimMultiple, moonNumber, primFak


def set_zaehlungen(
    prepare, num: int
):  # mehrere Zählungen finden festlegen zum später auslesen
        """Eine Zahl wird untersucht und die Variable prepare.zaehlungen wegen dieser Ergänzt
        prepare.zaehlungen bekommt informationen über mondzahlen und sonnenzahlen
        i ist eine zu Untersuchende Zahl kleinergeich num
        prepare.zaehlungen[4][i] bekommt die mondtypen, d.h. (Basis, Exponent) immer
        prepare.zaehlungen[1][zaehlung] welche zählung fängt mit welcher Zahl an
        prepare.zaehlungen[2][i] ist welcher Zählung es ist für eine beliebige Zahl: 1 ist 1-4, 2 ist 5-9, 3 ist 10-16
        prepare.zaehlungen[3][i] ist auch welche Zählung es ist für eine beliebige Zahl: 1 ist 1-4, 2 ist 5-9, 3 ist 10-16
        prepare.zaehlungen[0] ist bis zu welcher Zahl diese Untersuchung beim letzten Mal durchgeführt wurde
        prepare.zaehlungen  # [bis zu welcher zahl, {zaehlung:zahl},{zahl:zaehlung},{jede zahl,zugehoerigeZaehlung}]

        @type num: int
        @param num: zu untersuchende Zahl
        @rtype: kein Typ
        @return: nichts
        """
        num = prepare.originalLinesRange[-1]
        if prepare.gezaehlt:
            return
        else:
            prepare.gezaehlt = True
        wasMoon: bool = True
        if prepare.zaehlungen[0] == 0:
            isMoon = True
        else:
            isMoon = moonNumber(prepare.zaehlungen[0])[0] != []

        numbers_to_count = list(range(int(prepare.zaehlungen[0]) + 1, num + 1))
        moon_pairs = None
        try:
            from .parallel_execution import moon_numbers_in_processes

            parallel_result = moon_numbers_in_processes(
                numbers_to_count,
                config=getattr(prepare, "parallel_config", None),
            )
            if parallel_result is not None:
                moon_pairs = parallel_result.values
                try:
                    prepare.parallel_counting_result = parallel_result.snapshot()
                except Exception:
                    pass
        except Exception as exc:  # pragma: no cover - serial fallback protects row parity.
            try:
                prepare.parallel_counting_result = {
                    "mode": "serial_fallback",
                    "operation": "moon_numbers",
                    "error": f"{type(exc).__name__}: {exc}",
                }
            except Exception:
                pass
        if moon_pairs is None:
            moon_pairs = [(i, moonNumber(int(i))) for i in numbers_to_count]

        for i, moonType in moon_pairs:
            wasMoon = isMoon
            isMoon = moonType[0] != []
            if wasMoon and not isMoon:
                isMoon = False
                prepare.zaehlungen[1][len(prepare.zaehlungen[1]) + 1] = i
                prepare.zaehlungen[2][i] = len(prepare.zaehlungen[2]) + 1
            prepare.zaehlungen[3][i] = len(prepare.zaehlungen[2])
            prepare.zaehlungen[4][i] = moonType

def parameters_cmd_with_some_bereich(
    prepare,
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

def delete_doubles_in_sets(prepare, set1: set, set2: set) -> Iterable[Union[set, set]]:
        """Wenn etwas in 2 Mengen doppelt vorkommt wird es gelöscht
        @rtype: tuple[set,set]
        @return: Beide Mengen werden ausgegeben
        """
        intersection = set1 & set2
        return set1 - intersection, set2 - intersection

def from_until(prepare, a) -> tuple:
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

def zeile_which_zaehlung(prepare, zeile: int) -> int:
        return prepare.zaehlungen[3][zeile]

def moonsun(
    prepare, MoonNotSun: bool, numRangeYesZ: set, numRange, ifZaehlungenAtAll=True
):
        if not ifZaehlungenAtAll:
            prepare.setZaehlungen(prepare.originalLinesRange[-1])
        for n in numRange:
            if (prepare.zaehlungen[4][n][0] != []) == MoonNotSun:
                numRangeYesZ.add(n)
        return numRangeYesZ

def filter_original_lines(prepare, numRange: set, paramLines: set) -> set:
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
            or not prepare.ifZeilenSetted
        ):
            numRange = set(range(1, prepare.hoechsteZeile[1024] + 1))
        else:
            numRange = set()
            # return set(prepare.originalLinesRange)

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
                ",".join(mehrere), False, prepare.hoechsteZeile[1024] + 1
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
                            eins, False, prepare.hoechsteZeile[1024] + 1
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
                numRange = set(range(1, prepare.hoechsteZeile[114] + 1))

            numRangeYesZ |= BereichToNumbers2(
                ",".join(mehrere), True, prepare.hoechsteZeile[114] + 1
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
                            eins, True, prepare.hoechsteZeile[1024] + 1
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
                numRangeYesZ |= set(range(11, prepare.hoechsteZeile[1024] + 1))
        if ifZeitAtAll:
            if (
                len(numRange) == 0
                and not if_b_AtAll
                and not if_a_AtAll
                and "all" not in paramLines
                and len(numRangeYesZ) == 0
            ):
                numRange = set(range(1, prepare.hoechsteZeile[1024] + 1))
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
                    condition[3:], False, prepare.hoechsteZeile[1024] + 1
                )
                ifZaehlungenAtAll = True
                mehrere += [condition[3:]]
        if True or ifZaehlungenAtAll:
            prepare.setZaehlungen(prepare.originalLinesRange[-1])
        if ifZaehlungenAtAll:
            # prepare.setZaehlungen(prepare.originalLinesRange[-1])
            numRangeYesZ2 = set()
            if (
                len(numRange) == 0
                and not if_a_AtAll
                and not if_b_AtAll
                and "all" not in paramLines
            ):
                numRange = set(range(1, prepare.hoechsteZeile[1024] + 1))
            for n in numRange:  # nur die nummern, die noch infrage kommen
                for z in numRangeYesZ:
                    # 1-4:1,5-9:2 == jetzt ?
                    if prepare.zaehlungen[3][n] == int(z):
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
                            eins, False, prepare.hoechsteZeile[1024] + 1
                        )
                if len(minusBereiche) > 0:
                    for n in copy(numRange):
                        for z in minusBereiche:
                            if prepare.zaehlungen[3][n] == int(z):
                                numRange -= {n}

            # set().add
            # exit()
        # ANFANG
        ifTypAtAll = False
        numRangeYesZ = set()
        if len(numRange) == 0 and len(set(paramLines) - {"ka", "ka2"}) > 0:
            numRange = set(range(1, prepare.hoechsteZeile[1024] + 1))

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
            parallel_prime_result = None
            try:
                from .parallel_execution import prime_factors_in_processes

                parallel_prime_result = prime_factors_in_processes(
                    sorted(numRangeB),
                    config=getattr(prepare, "parallel_config", None),
                )
            except Exception as exc:  # pragma: no cover - serial fallback protects row parity.
                parallel_prime_result = None
                try:
                    prepare.parallel_row_filter_result = {
                        "mode": "serial_fallback",
                        "operation": "prime_factors",
                        "error": f"{type(exc).__name__}: {exc}",
                    }
                except Exception:
                    pass
            if parallel_prime_result is not None:
                primList = {number: factors for number, factors in parallel_prime_result.values}
                try:
                    prepare.parallel_row_filter_result = parallel_prime_result.snapshot()
                except Exception:
                    pass
            else:
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
            numRange = set(range(1, prepare.hoechsteZeile[1024] + 1))

        for condition in paramLines:
            if "mond" in condition:
                numRangeYesZ, ifTypAtAll = (
                    prepare.moonsun(True, numRangeYesZ, numRange, ifZaehlungenAtAll),
                    True,
                )
            elif "schwarzesonne" in condition:
                ifTypAtAll = True
                for n in numRange:
                    if n % 3 == 0:
                        numRangeYesZ.add(n)
            elif "sonne" in condition:
                numRangeYesZ, ifTypAtAll = (
                    prepare.moonsun(False, numRangeYesZ, numRange, ifZaehlungenAtAll),
                    True,
                )
            elif "planet" in condition:
                ifTypAtAll = True
                for n in numRange:
                    if n % 2 == 0:
                        numRangeYesZ.add(n)
            elif "SonneMitMondanteil" in condition:
                ifTypAtAll = True
                parallel_filter_result = None
                try:
                    from .parallel_execution import filter_numbers_in_processes

                    parallel_filter_result = filter_numbers_in_processes(
                        numRange,
                        "sonne_mit_mondanteil",
                        config=getattr(prepare, "parallel_config", None),
                    )
                except Exception as exc:  # pragma: no cover - serial fallback protects row parity.
                    parallel_filter_result = None
                    try:
                        prepare.parallel_row_filter_result = {
                            "mode": "serial_fallback",
                            "operation": "sonne_mit_mondanteil",
                            "error": f"{type(exc).__name__}: {exc}",
                        }
                    except Exception:
                        pass
                if parallel_filter_result is not None:
                    numRangeYesZ |= set(parallel_filter_result.values)
                    try:
                        prepare.parallel_row_filter_result = parallel_filter_result.snapshot()
                    except Exception:
                        pass
                else:
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
                numRange = set(range(1, prepare.hoechsteZeile[1024] + 1))

            parallel_filter_result = None
            try:
                from .parallel_execution import filter_numbers_in_processes

                parallel_filter_result = filter_numbers_in_processes(
                    numRange,
                    "prime_multiples",
                    primMultiples,
                    config=getattr(prepare, "parallel_config", None),
                )
            except Exception as exc:  # pragma: no cover - serial fallback protects row parity.
                parallel_filter_result = None
                try:
                    prepare.parallel_row_filter_result = {
                        "mode": "serial_fallback",
                        "operation": "prime_multiples",
                        "error": f"{type(exc).__name__}: {exc}",
                    }
                except Exception:
                    pass
            if parallel_filter_result is not None:
                numRangeYesZ |= set(parallel_filter_result.values)
                try:
                    prepare.parallel_row_filter_result = parallel_filter_result.snapshot()
                except Exception:
                    pass
            else:
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
                numRange = set(range(1, prepare.hoechsteZeile[1024] + 1))
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
            parallel_filter_result = None
            try:
                from .parallel_execution import filter_numbers_in_processes

                parallel_filter_result = filter_numbers_in_processes(
                    numRange,
                    "ordinary_multiples",
                    anyMultiples,
                    config=getattr(prepare, "parallel_config", None),
                )
            except Exception as exc:  # pragma: no cover - serial fallback protects row parity.
                parallel_filter_result = None
                try:
                    prepare.parallel_row_filter_result = {
                        "mode": "serial_fallback",
                        "operation": "ordinary_multiples",
                        "error": f"{type(exc).__name__}: {exc}",
                    }
                except Exception:
                    pass
            if parallel_filter_result is not None:
                numRangeYesZ |= set(parallel_filter_result.values)
                try:
                    prepare.parallel_row_filter_result = parallel_filter_result.snapshot()
                except Exception:
                    pass
            else:
                for n in numRange:
                    for divisor in anyMultiples:
                        if n % divisor == 0:
                            numRangeYesZ.add(n)
            numRange = cutset(ifMultiplesFromAnyAtAll, numRange, numRangeYesZ)

        # über 114 die Sonnen weg
        for n in copy(numRange - {0}):
            if (prepare.zaehlungen[4][n][0] == []) and (
                n > prepare.tables.hoechsteZeile[114]
            ):
                numRange.remove(n)
        invertieren = False
        for condition in paramLines:
            if condition[:3] == "_i_":
                invertieren = True
        if invertieren:
            numRangeList = list(numRange)
            numRangeList.sort()
            h = prepare.tables.hoechsteZeile[1024]
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
                    condition[3:], False, prepare.hoechsteZeile[1024] + 1
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
                    condition[3:], True, prepare.hoechsteZeile[1024] + 1
                )
                for a in NumRangeNeu:
                    numRangeNeu2 |= {numRange2Map[a]}
        if yJa:
            numRange &= numRangeNeu2

        return numRange


@dataclass(frozen=True)
class RowFilteringBundle:
    """Explizite Morphismenschicht für die Zeilenauswahl."""

    def parameters_cmd_with_some_bereich(self, prepare, mehrere_bereiche: str, symbol: str, neg: str, keine_neg_beruecksichtigung: bool = False) -> set:
        return parameters_cmd_with_some_bereich(prepare, mehrere_bereiche, symbol, neg, keine_neg_beruecksichtigung)

    def filter_original_lines(self, prepare, num_range: set, param_lines: set) -> set:
        return filter_original_lines(prepare, num_range, param_lines)

    def set_zaehlungen(self, prepare, num: int) -> None:
        return set_zaehlungen(prepare, num)

    def snapshot(self) -> dict:
        return {
            "class": "RowFilteringBundle",
            "legacy_owner": "libs.lib4tables_prepare.Prepare",
            "range_command_morphism": "parameters_cmd_with_some_bereich",
            "row_filter_morphism": "filter_original_lines",
            "counting_morphism": "set_zaehlungen",
            "condition_families": [
                "absolute_ranges",
                "relative_ranges",
                "time_relation",
                "zaehlungen",
                "prim_mod6_inside_outside",
                "moon_sun_planet",
                "prime_multiples",
                "powers",
                "ordinary_multiples",
                "neighbour_inversion",
                "z_y_position_filters",
            ],
        }


def bootstrap_row_filtering() -> RowFilteringBundle:
    return RowFilteringBundle()
