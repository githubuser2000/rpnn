#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import csv
import os
import sys
from collections import OrderedDict, defaultdict
from copy import copy, deepcopy
from fractions import Fraction
from itertools import zip_longest

try:
    from orderedset import OrderedSet
except (ModuleNotFoundError, ImportError):
    OrderedSet = set

from center import (
    DefaultOrderedDict,
    Multiplikationen,
    Primzahlkreuz_pro_contra_strs,
    alxp,
    cliout,
    getTextWrapThings,
    i18n,
    infoLog,
    multiples,
    output,
    primfaktoren,
    re,
    unique_everseen,
    x,
    nPmEnum,
)
from lib4tables import (
    OutputSyntax,
    bbCodeSyntax,
    couldBePrimeNumberPrimzahlkreuz,
    couldBePrimeNumberPrimzahlkreuz_fuer_aussen,
    couldBePrimeNumberPrimzahlkreuz_fuer_innen,
    csvSyntax,
    divisorGenerator,
    emacsSyntax,
    htmlSyntax,
    isPrimMultiple,
    markdownSyntax,
    math,
    moonNumber,
    primCreativity,
    primFak,
    primMultiple,
    primRepeat,
)
from lib4tables_Enum import ST

csvNames = i18n.csvFileNames
i18n = i18n.concat


# Primzahlkreuz_pro_contra_strs = (
#    "Primzahlkreuz pro contra",
#    "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)",
# )
class Concat:
    def __init__(self, tables):
        self.tables = tables
        self.ones = OrderedSet()
        self.CSVsAlreadRead = OrderedDict()
        self.CSVsSame = OrderedDict(
            {1: (1,), 2: (2, 4), 3: (3, 5), 4: (2, 4), 5: (3, 5)}
        )
        self.BruecheUni = OrderedSet()
        self.BruecheGal = OrderedSet()
        self.gebrRatMulSternUni = OrderedSet()
        self.gebrRatDivSternUni = OrderedSet()
        self.gebrRatMulGleichfUni = OrderedSet()
        self.gebrRatDivGleichfUni = OrderedSet()
        self.gebrRatMulSternGal = OrderedSet()
        self.gebrRatDivSternGal = OrderedSet()
        self.gebrRatMulGleichfGal = OrderedSet()
        self.gebrRatDivGleichfGal = OrderedSet()

    # @property
    # def gebrUnivSet(self):
    #    return self.puniverseprims

    # @gebrUnivSet.setter
    # def gebrUnivSet(self, value: set):
    #    self.gebrUniv = value

    # @property
    # def primUniversePrimsSet(self):
    #    return self.puniverseprims

    # @primUniversePrimsSet.setter
    # def primUniversePrimsSet(self, value: set):
    #    self.puniverseprims = value

    def concatLovePolygon(self, relitable: list, rowsAsNumbers: set) -> tuple:
        self.relitable = relitable
        if set(rowsAsNumbers) >= {9}:
            rowsAsNumbers |= {len(self.relitable[0])}
            self.tables.generatedSpaltenParameter_Tags[
                len(rowsAsNumbers) - 1
            ] = frozenset({ST.sternPolygon, ST.galaxie, ST.gleichfoermigesPolygon})
            for i, cols in enumerate(deepcopy(self.relitable)):
                if self.relitable[i][8].strip() != "":
                    self.relitable[i] += [
                        "".join(
                            (
                                self.relitable[i][8],
                                i18n.polygon1[" der eigenen Strukturgröße ("],
                                self.relitable[i][4],
                                i18n.polygon2[
                                    ") auf dich bei gleichförmigen Polygonen"
                                ],
                            )
                        )
                    ]
                else:
                    self.relitable[i] += [""]
            if (
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
                in self.tables.generatedSpaltenParameter
            ):
                raise ValueError
            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = self.tables.dataDict[0][9]
        return self.relitable, rowsAsNumbers

    def gleichheitFreiheitVergleich(self, zahl: int) -> str:
        zahl = int(zahl)
        ausgabeStringList = []
        if zahl % 4 == 0:
            ausgabeStringList += [
                i18n.gleichheitFreiheitVergleich["Dominieren, Unterordnen"]
            ]
        if zahl % 4 == 1:
            ausgabeStringList += [i18n.gleichheitFreiheitVergleich["Freiheit"]]
        if zahl % 4 == 3:
            ausgabeStringList += [
                i18n.gleichheitFreiheitVergleich["Einschränkung der Freiheit"]
            ]
        if zahl % 4 == 2:
            if (zahl - 2) % 8 == 0:
                ausgabeStringList += [i18n.gleichheitFreiheitVergleich["Gleichheit"]]
            if (zahl - 6) % 16 == 0:
                ausgabeStringList += [
                    i18n.gleichheitFreiheitVergleich["den anderen überbieten wollen"]
                ]
            if (zahl - 14) % 16 == 0:
                ausgabeStringList += [
                    i18n.gleichheitFreiheitVergleich["den anderen unterbieten wollen"]
                ]
        ausgabeString = "; ".join(ausgabeStringList)
        return ausgabeString

    def geistEmotionEnergieMaterieTopologie(self, zahl: int) -> str:
        zahl = int(zahl)
        prFa = primfaktoren(zahl)
        auss = [couldBePrimeNumberPrimzahlkreuz_fuer_aussen(a) for a in prFa]
        innen = [couldBePrimeNumberPrimzahlkreuz_fuer_innen(a) for a in prFa]
        zwei = len([a for a in prFa if a == 2])
        gefuehl = any(auss)
        denken = any(innen)
        # anZahlInnen = len([a for a in innen if True])
        totalTopologie = zwei > 1 and gefuehl
        etwasTopologie = (zwei > 1 or (zwei > 0 and gefuehl)) and not totalTopologie
        totalMaterie = zwei > 4
        etwasMaterie = zwei == 4
        wenigMaterie = zwei == 3
        kaumMaterie = zwei == 2
        x, y, z = denken, (2 in prFa), (3 in prFa)
        totalEnerge = x and y and z
        einermassenEnergie = ((x and y) or (y and z) or (y and z)) and not totalEnerge
        kaumEnergie = not einermassenEnergie and not totalEnerge and (x or y or z)
        ausgabeStringList = []
        if denken:
            ausgabeStringList += [i18n.energietopologie1["eine Denkart"]]
        if gefuehl:
            ausgabeStringList += [i18n.energietopologie1["eine Gefühlsart"]]
        if totalMaterie:
            ausgabeStringList += [
                i18n.energietopologie1["total eine Art, etwas geistig zu erzeugen"]
            ]
        if totalTopologie:
            ausgabeStringList += [i18n.energietopologie1["total eine Art zu erleben"]]
        if totalEnerge:
            ausgabeStringList += [i18n.energietopologie1["total eine Energie-Art"]]
        if etwasTopologie:
            ausgabeStringList += [i18n.energietopologie1["etwas eine Art zu erleben"]]
        if etwasMaterie:
            ausgabeStringList += [
                i18n.energietopologie1["etwas eine Art, etwas geistig zu erzeugen"]
            ]
        if wenigMaterie:
            ausgabeStringList += [
                i18n.energietopologie1["wenig eine Art, etwas geistig zu erzeugen"]
            ]
        if einermassenEnergie:
            ausgabeStringList += [
                i18n.energietopologie1["einigermaßen eine Energie-Art"]
            ]
        if kaumEnergie:
            ausgabeStringList += [i18n.energietopologie1["kaum eine Energie-Art"]]
        if kaumMaterie:
            ausgabeStringList += [
                i18n.energietopologie1["kaum eine Art, etwas geistig zu erzeugen"]
            ]
        ausgabeString = "; ".join(ausgabeStringList)
        return ausgabeString

    def concatGleichheitFreiheitDominieren(
        self, relitable: list, rowsAsNumbers: set
    ) -> tuple:
        self.relitable = relitable
        if set(rowsAsNumbers) >= {132}:
            rowsAsNumbers |= {len(self.relitable[0])}
            self.tables.generatedSpaltenParameter_Tags[
                len(rowsAsNumbers) - 1
            ] = frozenset({ST.sternPolygon, ST.universum})
            for i, cols in enumerate(
                deepcopy(self.relitable[: self.tables.lastLineNumber + 1])
            ):
                if i == 0:
                    ausgabeString = i18n.gleichheitFreiheitVergleich[
                        "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert"
                    ]
                else:
                    ausgabeString = self.gleichheitFreiheitVergleich(i)
                self.relitable[i] += [ausgabeString]

            if (
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
                in self.tables.generatedSpaltenParameter
            ):
                raise ValueError

            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = self.tables.dataDict[0][132]

        return self.relitable, rowsAsNumbers

    def concatGeistEmotionEnergieMaterieTopologie(
        self, relitable: list, rowsAsNumbers: set
    ) -> tuple:
        self.relitable = relitable
        if set(rowsAsNumbers) >= {242}:
            rowsAsNumbers |= {len(self.relitable[0])}
            self.tables.generatedSpaltenParameter_Tags[
                len(rowsAsNumbers) - 1
            ] = frozenset({ST.sternPolygon, ST.universum})
            for i, cols in enumerate(
                deepcopy(self.relitable[: self.tables.lastLineNumber + 1])
            ):
                if i == 0:
                    ausgabeString = i18n.ausgabeString[
                        "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art"
                    ]
                else:
                    ausgabeString = self.geistEmotionEnergieMaterieTopologie(i)
                self.relitable[i] += [ausgabeString]

            if (
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
                in self.tables.generatedSpaltenParameter
            ):
                raise ValueError

            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = self.tables.dataDict[0][242]

        return self.relitable, rowsAsNumbers

    def concatPrimCreativityType(self, relitable: list, rowsAsNumbers: set) -> tuple:
        self.relitable = relitable
        if set(rowsAsNumbers) >= {64}:
            rowsAsNumbers |= {len(self.relitable[0])}
            self.tables.generatedSpaltenParameter_Tags[
                len(rowsAsNumbers) - 1
            ] = frozenset({ST.sternPolygon, ST.galaxie})
            for i, cols in enumerate(
                deepcopy(self.relitable[: self.tables.lastLineNumber + 1])
            ):
                primCreativityType = primCreativity(i)
                self.relitable[i] += [
                    i18n.kreaZahl["Evolutions-Züchtungs-Kreativität"]
                    if i == 0
                    else (
                        i18n.kreaZahl["0. Primzahl 1"]
                        if primCreativityType == 0
                        else (
                            i18n.kreaZahl["1. Primzahl und Sonnenzahl"]
                            if primCreativityType == 1
                            else (
                                i18n.kreaZahl["2. Sonnenzahl, aber keine Primzahl"]
                                if primCreativityType == 2
                                else i18n.kreaZahl["3. Mondzahl"]
                            )
                        )
                    )
                ]

            if (
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
                in self.tables.generatedSpaltenParameter
            ):
                raise ValueError

            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = self.tables.dataDict[0][64]

        return self.relitable, rowsAsNumbers

    def concatMondExponzierenLogarithmusTyp(
        self, relitable: list, rowsAsNumbers: set
    ) -> tuple:
        self.relitable = relitable
        if set(rowsAsNumbers) >= {64}:
            hardcodedCouple = (44, 56)
            for rownum, rowheading in zip(
                hardcodedCouple,
                [
                    i18n.mondExpLog1["Mond-Typ eines Sternpolygons"],
                    i18n.mondExpLog1["Mond-Typ eines gleichförmigen Polygons"],
                ],
            ):
                rowsAsNumbers |= {len(self.relitable[0])}
                self.tables.generatedSpaltenParameter_Tags[len(rowsAsNumbers) - 1] = (
                    frozenset({ST.sternPolygon, ST.universum, ST.galaxie})
                    if rownum == 44
                    else frozenset(
                        {ST.gleichfoermigesPolygon, ST.universum, ST.galaxie}
                    )
                )
                for i, cols in enumerate(
                    deepcopy(self.relitable[: self.tables.lastLineNumber + 1])
                ):
                    moonTypesOf1Num = moonNumber(i)
                    if i == 0:
                        into = [rowheading]
                    else:
                        into = [
                            "[list]"
                            if self.tables.bbcodeOutputYes
                            else "<ul>"
                            if self.tables.htmlOutputYes
                            else "",
                            ""
                            if len(moonTypesOf1Num[0]) > 0
                            else i18n.mondExpLog2["kein Mond"],
                        ]
                        for k, (basis, exponentMinus2) in enumerate(
                            zip(*moonTypesOf1Num)
                        ):
                            if k > 0:
                                into += [" | "]
                            if self.tables.htmlOutputYes:
                                into += ["<li>"]
                            elif self.tables.bbcodeOutputYes:
                                into += ["[*]"]
                            insert = re.sub(
                                r"<SG>",
                                self.relitable[i][4].strip(),
                                self.relitable[basis][rownum].rstrip(),
                            )
                            insert = re.sub(
                                r"&lt;SG&gt;",
                                self.relitable[i][4].strip(),
                                insert,
                            )
                            into += [
                                insert,
                                " - ",
                                self.relitable[exponentMinus2 + 2][10],
                                " | ",
                                "</li>" if self.tables.htmlOutputYes else "",
                                self.relitable[i][10],
                                " + ",
                                self.relitable[i][11],
                                ", ",
                                self.relitable[exponentMinus2 + 2][85],
                            ]
                    if self.tables.htmlOutputYes and i != 0:
                        into += ["</ul>"]
                    self.relitable[i] += ["".join(into)]
                assert not (
                    len(self.tables.generatedSpaltenParameter)
                    + self.tables.SpaltenVanillaAmount
                    in self.tables.generatedSpaltenParameter
                )

                self.tables.generatedSpaltenParameter[
                    len(self.tables.generatedSpaltenParameter)
                    + self.tables.SpaltenVanillaAmount
                ] = self.tables.dataDict[0][64]

        return self.relitable, rowsAsNumbers

    def concatVervielfacheZeile(self, relitable: list, rowsAsNumbers: set) -> tuple:
        self.relitable = relitable
        # reliCopy = deepcopy(relitable)
        spaltenToVervielfache: set = rowsAsNumbers & {90, 19}
        for s in spaltenToVervielfache:
            store = {}
            for z, zeileninhalt in enumerate(
                relitable[2 : self.tables.lastLineNumber + 1], 2
            ):
                content = zeileninhalt[s]
                if len(content.strip()) > 0:
                    store[(z, s)] = content  # interessant
            multis = {}
            for coords, content in store.items():
                vielfacher = 1
                ergebnis = vielfacher * coords[0]
                # multis[ergebnis] = [coords[0]]
                try:
                    multis[ergebnis] += [coords[0]]  # interessant
                    # spalten wo was hin soll = ursprungszeile1,2,3,...
                except (IndexError, KeyError):
                    multis[ergebnis] = [coords[0]]  # interessant

                while ergebnis < len(relitable):
                    vielfacher += 1
                    ergebnis = vielfacher * coords[0]
                    try:
                        multis[ergebnis] += [coords[0]]  # interessant
                        # spalten wo was hin soll = ursprungszeile1,2,3,...
                    except (IndexError, KeyError):
                        multis[ergebnis] = [coords[0]]  # interessant
            for z, zeileninhalt in enumerate(
                relitable[2 : self.tables.lastLineNumber + 1], 2
            ):
                # alle spalten und zeilen
                xx = False

                if len(relitable[z][s].strip()) != 0:
                    if self.tables.htmlOutputYes:
                        relitable[z][s] = ["<li>", relitable[z][s], "</li>"]
                    elif self.tables.bbcodeOutputYes:
                        relitable[z][s] = ["[*]", relitable[z][s]]
                    else:
                        relitable[z][s] = [relitable[z][s], " | "]
                else:
                    relitable[z][s] = [relitable[z][s]]

                if z in multis:
                    for UrZeile in multis[z]:
                        if (
                            UrZeile != z
                            and "".join(relitable[z][s]) != store[(UrZeile, s)]
                            and "".join(relitable[z][s] + [" | "])
                            != store[(UrZeile, s)]
                            and "".join(["<li>"] + relitable[z][s] + ["</li>"])
                            != store[(UrZeile, s)]
                            and "".join(["[*]"] + relitable[z][s])
                            != store[(UrZeile, s)]
                        ):
                            if len(store[(UrZeile, s)]) != 0:
                                if self.tables.htmlOutputYes:
                                    relitable[z][s] += [
                                        "<li>",
                                        store[(UrZeile, s)],
                                        "</li>",
                                    ]
                                elif self.tables.bbcodeOutputYes:
                                    relitable[z][s] += ["[*]", store[(UrZeile, s)]]
                                else:
                                    xx = (
                                        True
                                        if not self.tables.bbcodeOutputYes
                                        else False
                                    )
                                    relitable[z][s] += [store[(UrZeile, s)], " | "]

                if self.tables.htmlOutputYes:
                    relitable[z][s] = ["<ul>"] + relitable[z][s] + ["</ul>"]
                elif self.tables.bbcodeOutputYes:
                    relitable[z][s] = ["[list]"] + relitable[z][s] + ["[/list]"]
                if xx:
                    relitable[z][s] = "".join(relitable[z][s][:-1])
                else:
                    relitable[z][s] = "".join(relitable[z][s])

        return self.relitable, rowsAsNumbers

    def concatModallogik(
        self, relitable: list, conceptsRowsSetOfTuple: set, rowsAsNumbers: set
    ) -> tuple:
        """setzt die Modallogik um, d.h. Kombination von 2 bisher Programmierten
        Funktionen: 1. vielfache von Primzahlen oder natürlichen Zahlen
        (zweiteres programmiere ich später) bilden
        und die andere Funtion 2. +- 1 +- 2 und Bedeutungsveränderung

        @type relitable: list
        @param relitable: Haupttabelle self.relitable
        @return: relitable + weitere Tabelle daneben
        """

        def getModaloperatorsPerLineCells(lineWeAreAt: int) -> tuple:
            """Gibt ein Tuple aus Strings aus, dass die richtigen Modaloperatoren
            pro Zeile ausgibt
            @type int
            @param Zeile
            @return: Tupel aus Modaloperatoren
            """

            def getModaloperatorsPerLineCoordinates(lineWeAreAt: int) -> tuple:
                modalMainOperatorZeile: int = lineWeAreAt
                amountModaloperators: int = lineWeAreAt - 1
                modalOpElseOperatorsZeilenBegin: int = lineWeAreAt + 1
                modalOpElseOperatorsZeilenEnd: int = (
                    lineWeAreAt + amountModaloperators + 1
                )
                return (
                    modalMainOperatorZeile,
                    modalOpElseOperatorsZeilenBegin,
                    modalOpElseOperatorsZeilenEnd,
                )

            coords = getModaloperatorsPerLineCoordinates(lineWeAreAt)
            modaloperators: list = []
            try:
                # modaloperators += [self.relitable[coords[0]][10]]
                modaloperators += [
                    self.relitable[coords[0]][97],
                    self.relitable[coords[0]][98],
                ]
            except:
                pass
            for coord in range(coords[1], coords[2]):
                try:
                    # modaloperators += [self.relitable[coord][42]]
                    modaloperators += [self.relitable[coord][42]]
                except IndexError:
                    pass
            return tuple(modaloperators)

        def ModalLogikIntoTable(
            concept, distanceFromLine, i, into, vorkommenVielfacher_B
        ):
            try:
                modalOperatorenEn = vorkommenVielfacher_B[i][distanceFromLine]["modalS"]
                vervielfachterEn = vorkommenVielfacher_B[i][distanceFromLine][
                    "vervielfachter"
                ]
                for modalOperatoren, vervielfachter in zip(
                    modalOperatorenEn, vervielfachterEn
                ):
                    try:
                        intoItsContent = (
                            self.relitable[vervielfachter][concept[0]]
                            if abs(distanceFromLine) % 2 == 0
                            else self.relitable[vervielfachter][concept[1]]
                        )

                        into[i] += [
                            "<li>"
                            if self.tables.htmlOutputYes
                            else "[*]"
                            if self.tables.bbcodeOutputYes
                            else ""
                        ]
                        into[i] += (
                            [
                                i18n.modalB["mittelstark überdurchschnittlich: "]
                                if abs(distanceFromLine) == 2
                                else (
                                    i18n.modalB["überdurchschnittlich: "]
                                    if abs(distanceFromLine) == 1
                                    else (
                                        i18n.modalB[
                                            "mittelleicht überdurchschnittlich: "
                                        ]
                                        if abs(distanceFromLine) == 3
                                        else (
                                            i18n.modalB["sehr: "]
                                            if abs(distanceFromLine) == 0 != ""
                                            else i18n.modalB[
                                                "sehr leicht überdurchschnittlich: "
                                            ]
                                        )
                                    )
                                ),
                                modalOperatoren[0],
                                " ",
                                intoItsContent
                                if modalOperatoren[0] == self.relitable[1][97]
                                else intoItsContent.replace(
                                    i18n.modalC["intrinsisch"], i18n.modalC["zuerst"]
                                ).replace(
                                    i18n.modalC["extrinsisch"],
                                    i18n.modalC["als zweites"],
                                ),
                                " ",
                                modalOperatoren[1],
                            ]
                            + (
                                (
                                    [
                                        i18n.modalD[", nicht: "],
                                        ", ".join(modalOperatoren[2:]),
                                        i18n.modalD[" (das alles nicht): "],
                                        self.relitable[vervielfachter][concept[0]]
                                        .replace(
                                            i18n.modalD["extrinsisch"],
                                            i18n.modalD["als zweites"],
                                        )
                                        .replace(
                                            i18n.modalD["intrinsisch"],
                                            i18n.modalD["zuerst"],
                                        ),
                                    ]
                                    if len(modalOperatoren) > 2
                                    else [""]
                                )
                                if abs(distanceFromLine) % 2 == 1
                                else [""]
                            )
                            + [
                                " | "
                                if not self.tables.htmlOutputYes
                                and not self.tables.bbcodeOutputYes
                                else ""
                            ]
                        )
                        into[i] += ["</li>" if self.tables.htmlOutputYes else ""]
                    except (IndexError, KeyError):
                        pass
            except (IndexError, KeyError):
                pass

        def storeModalNvervielfachter(
            Orginal_i_mehrere,
            distanceFromLine,
            i,
            modalOperatorEnEn,
            vervielFachter,
            vorkommenVielfacher_B,
        ):
            vorkommenVielfacher_B[i][distanceFromLine] = {
                "i_origS": Orginal_i_mehrere,
                "modalS": modalOperatorEnEn,
                "vervielfachter": vervielFachter,
            }

        def prepareModalIntoTable(
            distanceFromLine,
            getModaloperatorsPerLineCells,
            i,
            storeModalNvervielfachter,
            vorkommenVielfacher,
            vorkommenVielfacher_B,
        ):
            i_with_a_distance = i + distanceFromLine
            try:
                modalOperatorEnEn: list = []
                Orginal_i_mehrere: list = []
                # vorkommenZeilenBegriffe: list = []
                vervielFachter: list = []
                # Ein Couple besteht aus der Zahl, ggf. Primzahl mit ihrem Vielfacher danach
                for couple in vorkommenVielfacher[i_with_a_distance]:
                    vorkommen, vielfacher = couple[0], couple[1]
                    modalOperatorEnEn += [(getModaloperatorsPerLineCells(vielfacher))]
                    # vorkommenZeilenBegriffe += [
                    #    vorkommen * vielfacher
                    # ]
                    vervielFachter += [vorkommen]
                    Orginal_i_mehrere += [i_with_a_distance]
                """
                Was ist hier drin gespeichert?
                    erster Parameter: das i von allen Distanzen -4 bis 4 mit 0
                    zweiter Paramter: Ob: ModalOperator oder was war Orignal i von dem das hier der Vielfacher ist
                    dahinter: liste von der Sache
                """
                try:
                    vorkommenVielfacher_B[i][distanceFromLine] = OrderedDict(
                        {
                            "i_origS": Orginal_i_mehrere
                            + vorkommenVielfacher_B[i][distanceFromLine]["i_origS"],
                            "modalS": modalOperatorEnEn
                            + vorkommenVielfacher_B[i][distanceFromLine]["modalS"],
                            "vervielfachter": vervielFachter
                            + vorkommenVielfacher_B[i][distanceFromLine][
                                "vervielfachter"
                            ],
                        }
                    )

                except (IndexError, KeyError):
                    try:
                        storeModalNvervielfachter(
                            Orginal_i_mehrere,
                            distanceFromLine,
                            i,
                            modalOperatorEnEn,
                            vervielFachter,
                            vorkommenVielfacher_B,
                        )
                    except (IndexError, KeyError):
                        vorkommenVielfacher_B[i] = OrderedDict()
                        storeModalNvervielfachter(
                            Orginal_i_mehrere,
                            distanceFromLine,
                            i,
                            modalOperatorEnEn,
                            vervielFachter,
                            vorkommenVielfacher_B,
                        )
                del vervielFachter
            except (IndexError, KeyError):
                pass

        def vorkommenNvielfacherPerItsProduct(
            einVorkommen, ergebnis, vielfacher, vorkommenVielfacher
        ):
            # print([einVorkommen, ergebnis, vielfacher])
            try:
                vorkommenVielfacher[ergebnis] += [
                    (
                        einVorkommen,
                        vielfacher,
                    )
                ]
            except (IndexError, KeyError):
                vorkommenVielfacher[ergebnis] = [
                    (
                        einVorkommen,
                        vielfacher,
                    )
                ]

        self.relitable = relitable

        distances = (-4, -3, -2, -1, 0, 1, 2, 3, 4)
        conceptsRowsSetOfTuple2: list = list(conceptsRowsSetOfTuple)
        reliTableCopy = deepcopy(self.relitable)
        conceptsRowsSetOfTuple2.sort()
        for o, concept in enumerate(conceptsRowsSetOfTuple2):
            into: dict = {}
            einMalVorkommen = OrderedSet()
            for i, cols in enumerate(reliTableCopy):
                into[i] = [""]
                if i == 0:
                    into[i] = [i18n.generiertWort["Generiert: "], cols[concept[0]]]
                elif cols[concept[0]].strip() != "":
                    einMalVorkommen |= {i}

            vorkommenVielfacher: OrderedDict = OrderedDict()
            einMalVorkommen = tuple(einMalVorkommen)

            for (
                einVorkommen
            ) in (
                einMalVorkommen
            ):  # d.h. so ein Wort wie weise oder gut kommt in vor in der csv
                vielfacher = 1
                ergebnis = vielfacher * einVorkommen
                vorkommenNvielfacherPerItsProduct(
                    einVorkommen, ergebnis, vielfacher, vorkommenVielfacher
                )
                while ergebnis < len(reliTableCopy):
                    vielfacher += 1
                    ergebnis = vielfacher * einVorkommen
                    vorkommenNvielfacherPerItsProduct(
                        einVorkommen, ergebnis, vielfacher, vorkommenVielfacher
                    )

            vorkommenVielfacher_B: OrderedDict = OrderedDict()
            for i, zeileninhalte in enumerate(
                reliTableCopy[1 : self.tables.lastLineNumber + 1], 1
            ):
                for distanceFromLine in distances:
                    prepareModalIntoTable(
                        distanceFromLine,
                        getModaloperatorsPerLineCells,
                        i,
                        storeModalNvervielfachter,
                        vorkommenVielfacher,
                        vorkommenVielfacher_B,
                    )

            for i, zeileninhalte in enumerate(
                reliTableCopy[1 : self.tables.lastLineNumber + 1], 1
            ):
                for distanceFromLine in distances:
                    ModalLogikIntoTable(
                        concept, distanceFromLine, i, into, vorkommenVielfacher_B
                    )
                # wenn i>0
                conditionNvs1perN = concept[0] in {
                    62,
                    63,
                    *range(358, 367 + 1),
                    *range(371, 374 + 1),
                }

                if conditionNvs1perN:
                    fill_ = zeileninhalte[197]
                else:
                    fill_ = zeileninhalte[4]

                if into[i] != [""]:
                    into[i] += [
                        "<li>"
                        if self.tables.htmlOutputYes
                        else "[*]"
                        if self.tables.bbcodeOutputYes
                        else "",
                        i18n.allesNurBezogenAufSatz,
                        fill_,
                        "</li>" if self.tables.htmlOutputYes else "",
                    ]

            for w, cols in enumerate(reliTableCopy[: self.tables.lastLineNumber + 1]):
                if self.tables.htmlOutputYes and "<li>" in into[w]:
                    into[w] = ["<ul>"] + into[w] + ["</ul>"]
                elif self.tables.bbcodeOutputYes and "[*]" in into[w]:
                    into[w] = ["[list]"] + into[w] + ["[/list]"]
                self.relitable[w] += ["".join(into[w])]

            rowsAsNumbers |= {len(self.relitable[0]) - 1}
            if conditionNvs1perN:
                self.tables.generatedSpaltenParameter_Tags[
                    len(rowsAsNumbers) - 1
                ] = frozenset({ST.gleichfoermigesPolygon, ST.galaxie})
            else:
                self.tables.generatedSpaltenParameter_Tags[
                    len(rowsAsNumbers) - 1
                ] = frozenset({ST.sternPolygon, ST.galaxie})
            if (
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
                in self.tables.generatedSpaltenParameter
            ):
                raise ValueError
            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = self.tables.dataDict[1][conceptsRowsSetOfTuple2[o]]

        return self.relitable, rowsAsNumbers

    def convertSetOfPaarenToDictOfNumToPaareDiv(
        self, paareSet: OrderedSet, gleichf=False
    ) -> DefaultOrderedDict:
        """Macht aus einem Set aus Paaren eins von verschiedenen möglichen dicts mit key int und value liste aus paaren"""
        # px2 = list(paareSet)
        # px2.sort()
        result: DefaultOrderedDict = DefaultOrderedDict(OrderedSet)
        paareSet: tuple = tuple(paareSet)
        for paar in paareSet:
            paar = tuple(paar)
            div = paar[0] / paar[1] if not gleichf else paar[1] / paar[0]
            div = round(div * 1000) / 1000
            # paar2 = list(paar)
            # paar2.sort()
            assert div == round(div)
            result[int(div)] |= {paar}
        return result

    def convertSetOfPaarenToDictOfNumToPaareMul(
        self, paareSet: set, gleichf=False
    ) -> DefaultOrderedDict:
        """Macht aus einem Set aus Paaren eins von verschiedenen möglichen dicts mit key int und value liste aus paaren"""
        result: DefaultOrderedDict = DefaultOrderedDict(OrderedSet)

        for paar in tuple(paareSet):
            paar = tuple(paar)
            mul = paar[0] * paar[1]
            if gleichf:
                mul = 1 / mul
            mulr = round(mul)
            mul = round(mul * 1000) / 1000
            assert mul == mulr
            result[int(mulr)] |= {paar}
        return result

    def convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(
        self, fracs: set, fracs2: set, gleichf=False
    ) -> DefaultOrderedDict:
        result: DefaultOrderedDict = DefaultOrderedDict(OrderedSet)
        if not gleichf:
            for frac in tuple(fracs):
                # for zusatzMul in range(1, 1025):
                for zusatzMul in range(1, self.tables.hoechsteZeile[1024] + 1):
                    paar = (frac, Fraction(frac.denominator) * zusatzMul)
                    mul = paar[0] * paar[1]
                    mulr = round(mul)
                    mul = round(mul * 1000) / 1000
                    assert mulr == mul
                    # if mul > 1024:
                    if mul > self.tables.hoechsteZeile[1024]:
                        break
                    result[int(mul)] |= {paar}

            for frac in tuple(fracs):
                # for zusatzMul in range(1024, 0, -1):
                for zusatzMul in range(self.tables.hoechsteZeile[1024], 0, -1):
                    faktor = Fraction(frac.denominator) / zusatzMul
                    if (faktor in fracs2) or faktor.numerator == 1:
                        paar = (frac, faktor)
                        mul = paar[0] * paar[1]
                        mulr = round(mul)
                        # if mul > 1024:
                        if mul > self.tables.hoechsteZeile[1024]:
                            break
                        if mulr == mul:
                            result[int(mul)] |= {paar}

        else:
            for frac in tuple(fracs):
                # for zusatzDiv in range(1, 1025):
                for zusatzDiv in range(1, self.tables.hoechsteZeile[1024] + 1):
                    paar = (frac, 1 / Fraction(frac.numerator) / zusatzDiv)
                    div = 1 / (paar[1] * paar[0])
                    divr = round(div)
                    div = round(div * 1000) / 1000
                    assert divr == div
                    # if div > 1024:
                    if div > self.tables.hoechsteZeile[1024]:
                        break
                    result[int(divr)] |= {paar}

            for frac in tuple(fracs):
                # for zusatzDiv in range(1, 1025):
                for zusatzDiv in range(1, self.tables.hoechsteZeile[1024] + 1):
                    faktor = (1 / frac) / zusatzDiv
                    if faktor in fracs2 or faktor.numerator == 1:
                        paar = (frac, faktor)
                        mul = 1 / (paar[1] * paar[0])
                        mulr = round(mul)
                        mul = round(mul * 1000) / 1000
                        assert mulr == mul
                        # if 1 / mul > 1024:
                        if 1 / mul > self.tables.hoechsteZeile[1024]:
                            break
                        result[int(mulr)] |= {paar}

        ##result2: DefaultOrderedDict = DefaultOrderedDict(list)
        # for key, value in result.items():
        #    result2[key] = list(value)

        # x("BRUCH?", result)
        return result

    def combineDicts(
        self, a: DefaultOrderedDict, b: DefaultOrderedDict
    ) -> DefaultOrderedDict:
        e: DefaultOrderedDict = DefaultOrderedDict(OrderedSet)

        for key, value in a.items():
            e[key] |= value
        for key, value in b.items():
            e[key] |= value

        for key, value in e.items():
            newValue = OrderedSet()
            for v in value:
                newValue |= {tuple(v)}
            e[key] = newValue

        return e

    def concat1PrimzahlkreuzProContra(
        self, relitable: list, rowsAsNumbers: set, generatedBefehle: set, ParametersMain
    ) -> tuple:
        """Fügt eine Spalte ein, in der Primzahlen mit Vielfachern
        auf dem Niveau des Universums nicht einfach nur aus einer
        CSV Tabelle geladen werden, sondern durch Primzahlen und
        deren Vielfachern generiert werden.

        @type relitable: list
        @param relitable: Haupttabelle self.relitable
        @return: relitable + weitere Tabelle daneben
        """
        global Primzahlkreuz_pro_contra_strs

        if "primzahlkreuzprocontra" in generatedBefehle:
            self.relitable = relitable
            primAmounts = 0
            keinePrimzahl1, keinePrimzahl2 = True, True
            list1, list2 = [], []
            weiter1a, weiter1b, weiter2a, weiter2b = 0, 0, 0, 0
            proPro, contraContra = OrderedDict(), OrderedDict()
            proPro2, contraContra2 = OrderedDict(), OrderedDict()
            dreli = deepcopy(self.relitable)
            headline: str = i18n.headline1
            into_Str1: OrderedDict = OrderedDict()
            into_Str2: OrderedDict = OrderedDict()

            # for num, cols in zip_longest(range(0, 1025), dreli):
            if self.tables.hoechsteZeile[1024] >= len(dreli):
                bereich = zip_longest(
                    range(0, self.tables.hoechsteZeile[1024] + 1), dreli
                )
            else:
                bereich = zip(range(0, self.tables.hoechsteZeile[1024] + 1), dreli)

            for num, cols in bereich:
                contraContra2[num] = OrderedSet()
                proPro2[num] = OrderedSet()

                if num == 0:
                    into: list = [headline]
                else:
                    into: list = []
                into1: list = []
                into2: list = []
                if couldBePrimeNumberPrimzahlkreuz(num):
                    primAmounts += 1
                if primCreativity(num) == 1 or num == 1:
                    if couldBePrimeNumberPrimzahlkreuz_fuer_innen(num):
                        list1 += [num]
                        if num > 16:
                            if keinePrimzahl1:
                                gegen = list2[weiter1b + 1]
                                weiter1b += 1
                            else:
                                gegen = list1[weiter1a]
                                weiter1a += 1
                            contraContra[num] = gegen
                            contraContra2[num] |= {gegen}
                            into1 += [i18n.gegen["gegen "] + str(gegen)]
                        elif num in (11, 5):
                            if num == 5:
                                gegen = 2
                            elif num == 11:
                                gegen = 2
                            contraContra[num] = gegen
                            contraContra2[num] |= {gegen}
                            into1 += [i18n.gegen["gegen "] + str(gegen)]

                        keinePrimzahl1 = False

                    if num in (2, 3):
                        if num == 2:
                            gegen = 1
                            contraContra[num] = gegen
                            contraContra2[num] |= {gegen}
                            into1 += [i18n.gegen["gegen "] + str(gegen)]
                        elif num == 3:
                            pro = 1
                            proPro[num] = pro
                            proPro2[num] |= {pro}
                            into2 += [i18n.pro["pro "] + str(pro)]

                    if couldBePrimeNumberPrimzahlkreuz_fuer_aussen(num):
                        list2 += [num]
                        if num > 16:
                            if keinePrimzahl2:
                                pro = list1[weiter2b + 1]
                                weiter2b += 1
                            else:
                                pro = list2[weiter2a]
                                weiter2a += 1
                            proPro[num] = pro
                            proPro2[num] |= {pro}
                            into2 += [i18n.pro["pro "] + str(pro)]
                        elif num in (7, 13):
                            if num == 7:
                                pro = 3
                            elif num == 13:
                                pro = 3
                            proPro[num] = pro
                            proPro2[num] |= {pro}
                            into2 += [i18n.pro["pro "] + str(pro)]

                        keinePrimzahl2 = False
                else:
                    if couldBePrimeNumberPrimzahlkreuz_fuer_innen(num):
                        keinePrimzahl1 = True
                    elif couldBePrimeNumberPrimzahlkreuz_fuer_aussen(num):
                        keinePrimzahl2 = True

                    menge: OrderedSet = OrderedSet()
                    for couple in primMultiple(num):
                        couple = list(couple)
                        couple.sort()
                        menge |= {tuple(couple)}
                    paare = list(menge)

                    for coupleA in paare:
                        # if primCreativity(couple[1]) == 1:
                        #    flagX = True
                        # elif primCreativity(couple[0]) == 1:
                        #    flagX = True
                        #    couple = (couple[1], couple[0])
                        # else:
                        #    flagX = False

                        # if flagX:
                        if coupleA[1] != 1 and coupleA[0] != 1:
                            for couple in (coupleA, (coupleA[1], coupleA[0])):
                                for firstOrSecond in (
                                    (1, 0) if couple[0] != couple[1] else (1,)
                                ):
                                    if (
                                        couldBePrimeNumberPrimzahlkreuz_fuer_innen(
                                            couple[firstOrSecond]
                                        )
                                        or couple[0] % 2 == 0
                                        or couple[1] % 2 == 0
                                    ):
                                        try:
                                            gegen3 = int(
                                                couple[0 if firstOrSecond == 1 else 1]
                                                * contraContra[couple[firstOrSecond]]
                                            )
                                            contraContra[num] = gegen3
                                            contraContra2[num] |= {gegen3}
                                            into1 += [
                                                i18n.gegen["gegen "] + str(gegen3)
                                            ]
                                        except KeyError:
                                            pass
                                    if (
                                        couldBePrimeNumberPrimzahlkreuz_fuer_aussen(
                                            couple[1]
                                        )
                                        or couple[1] % 3 == 0
                                        or couple[0] % 3 == 0
                                    ):
                                        try:
                                            if num == 4:
                                                pass
                                                # print(
                                                #    f"{couple} ___ {firstOrSecond} | {proPro[couple[firstOrSecond]]}"
                                                # )
                                            pro3 = (
                                                int(
                                                    couple[
                                                        0 if firstOrSecond == 1 else 1
                                                    ]
                                                )
                                                * proPro[couple[firstOrSecond]]
                                            )
                                            proPro[num] = pro3
                                            proPro2[num] |= {pro3}
                                            into2 += [i18n.pro["pro "] + str(pro3)]
                                        except KeyError:
                                            pass

                # if self.tables.lastLineNumber >= num:
                try:
                    text = cols[206].split("|")[1]
                except (KeyError, TypeError, IndexError):
                    text = ""
                if len(text) > 0:
                    into += [text]

                into1 = list(OrderedSet(into1))
                into2 = list(OrderedSet(into2))
                into_Str1[num] = (
                    i18n.hineinversetzen[" Darin kann sich die "],
                    str(num),
                    i18n.hineinversetzen[" am Besten hineinversetzen."],
                )
                into_Str2[num] = (
                    i18n.hineinversetzen[" Darin kann sich die "],
                    str(num),
                    i18n.hineinversetzen[" am Besten hineinversetzen."],
                )

                if num != 0:
                    if self.tables.htmlOutputYes:
                        into = [
                            "<ul>",
                            "<li>" if len(into1) > 0 else "",
                            ", ".join(into1),
                            "".join(into_Str1[num]) if len(into1) > 0 else "",
                            "</li>" if len(into1) > 0 else "",
                            "<li>" if len(into2) > 0 else "",
                            ", ".join(into2),
                            "".join(into_Str2[num]) if len(into2) > 0 else "",
                            "</li>" if len(into2) > 0 else "",
                            "<li>" if len(into) > 0 else "",
                            ", ".join(into),
                            "</li>" if len(into) > 0 else "",
                            "</ul>",
                        ]
                    elif self.tables.bbcodeOutputYes:
                        into = [
                            "[list]",
                            "[*]" if len(into1) > 0 else "",
                            ", ".join(into1),
                            "".join(into_Str1[num]) if len(into1) > 0 else "",
                            "[*]" if len(into2) > 0 else "",
                            ", ".join(into2),
                            "".join(into_Str2[num]) if len(into2) > 0 else "",
                            "[*]" if len(into) > 0 else "",
                            ", ".join(into),
                            "[/list]",
                        ]
                    else:
                        into = [
                            ", ".join(into1),
                            "".join(into_Str1[num]) if len(into1) > 0 else "",
                            ", ".join(into2),
                            "".join(into_Str2[num]) if len(into2) > 0 else "",
                            ", ".join(into),
                        ]
                    intoB = []
                    for intoneu in into:
                        if len(intoneu) > 0:
                            intoB += [intoneu]
                else:
                    intoB = into

                self.relitable[num] += (
                    [
                        (
                            " | "
                            if not self.tables.htmlOutputYes
                            and not self.tables.bbcodeOutputYes
                            else ""
                        ).join(intoB)
                    ]
                    if len(into) > 0
                    else [""]
                )
            # except (KeyError, TypeError):
            #    self.relitable[num] += ["-"]

            rowsAsNumbers |= {len(self.relitable[0]) - 1, len(self.relitable[0])}
            self.tables.generatedSpaltenParameter_Tags[
                len(rowsAsNumbers) - 1
            ] = frozenset({ST.sternPolygon, ST.universum})
            self.tables.generatedSpaltenParameter_Tags[
                len(rowsAsNumbers) - 2
            ] = frozenset({ST.sternPolygon, ST.universum})

            assert not (
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
                in self.tables.generatedSpaltenParameter
            )

            kette = [
                [(ParametersMain.bedeutung[0], Primzahlkreuz_pro_contra_strs[0])],
                [(ParametersMain.procontra[0], Primzahlkreuz_pro_contra_strs[0])],
                [(ParametersMain.grundstrukturen[0], Primzahlkreuz_pro_contra_strs[1])],
            ]
            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = kette
            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = kette

            reverseContra: OrderedDict = OrderedDict()
            for key, value in contraContra2.items():
                for value2 in value:
                    try:
                        reverseContra[value2] |= {key}
                    except KeyError:
                        reverseContra[value2] = OrderedSet({key})
            reversePro: OrderedDict = OrderedDict()
            for key, value in proPro2.items():
                for value2 in value:
                    try:
                        reversePro[value2] |= {key}
                    except KeyError:
                        reversePro[value2] = OrderedSet({key})

            pro2: list
            contra2: list
            kette2: list
            for num, cols in enumerate(dreli[: self.tables.lastLineNumber + 1]):
                try:
                    pro2 = list(reversePro[num])
                except KeyError:
                    pro2 = []
                try:
                    contra2 = list(reverseContra[num])
                except KeyError:
                    contra2 = []

                if num == 0:
                    kette2 = [headline]
                elif contra2 != [] or pro2 != []:
                    dahinter1a = (
                        dreli[c][206].split("|")[1]
                        if c <= self.tables.lastLineNumber
                        and len(dreli[c][206].split("|")) == 2
                        and int(dreli[c][206].split("|")[0]) == num
                        else ""
                        for c in pro2
                    )
                    dahinter1b = []
                    for a in dahinter1a:
                        if len(a) > 0:
                            dahinter1b += [a]
                    dahinter1: str = " , ".join(dahinter1b)

                    dahinter2a = (
                        dreli[c][206].split("|")[1]
                        if c <= self.tables.lastLineNumber
                        and len(dreli[c][206].split("|")) == 2
                        and int(dreli[c][206].split("|")[0]) == num
                        else ""
                        for c in contra2
                    )
                    dahinter2b = []
                    for a in dahinter2a:
                        if len(a) > 0:
                            dahinter2b += [a]
                    dahinter2: str = ", ".join(dahinter2b)

                    dahinter1len: int = len(dahinter1)
                    dahinter2len: int = len(dahinter2)

                    kette2 = [
                        "[list]"
                        if self.tables.bbcodeOutputYes
                        else "<ul>"
                        if self.tables.htmlOutputYes
                        else "",
                        (
                            "[*]"
                            if self.tables.bbcodeOutputYes
                            else "<li>"
                            if self.tables.htmlOutputYes
                            else ""
                        )
                        if len(pro2) > 0
                        else "",
                        i18n.proIst["pro dieser Zahl sind: "]
                        if len(pro2) > 1
                        else i18n.proIst["pro dieser Zahl ist "]
                        if len(pro2) == 1
                        else "",
                        str(pro2)[1:-1],
                        ("</li>" if self.tables.htmlOutputYes else "")
                        if len(pro2) > 0
                        else "",
                        (
                            "[*]"
                            if self.tables.bbcodeOutputYes
                            else "<li>"
                            if self.tables.htmlOutputYes
                            else " ("
                        )
                        if dahinter1len > 0
                        else "",
                        dahinter1,
                        (
                            "</li>"
                            if self.tables.htmlOutputYes
                            else ""
                            if self.tables.bbcodeOutputYes
                            else ")"
                        )
                        if dahinter1len > 0
                        else "",
                        (
                            "<li>"
                            if self.tables.htmlOutputYes and len(contra2) > 0
                            else "[*]"
                            if self.tables.bbcodeOutputYes and len(contra2) > 0
                            else " | "
                            if len(pro2) > 0 and len(contra2) > 0
                            else ""
                        ),
                        i18n.contraIst[" contra dieser Zahl sind: "]
                        if len(contra2) > 1
                        else i18n.contraIst[" contra dieser Zahl ist "]
                        if len(contra2) == 1
                        else "",
                        str(contra2)[1:-1],
                        "</li>"
                        if self.tables.htmlOutputYes and len(contra2) > 0
                        else "",
                        (
                            "[*]"
                            if self.tables.bbcodeOutputYes
                            else "<li>"
                            if self.tables.htmlOutputYes
                            else " ("
                        )
                        if dahinter2len > 0
                        else "",
                        dahinter2,
                        (
                            "</li>"
                            if self.tables.htmlOutputYes
                            else ""
                            if self.tables.bbcodeOutputYes
                            else ")"
                        )
                        if dahinter2len > 0
                        else "",
                        "[/list]"
                        if self.tables.bbcodeOutputYes
                        else "</ul>"
                        if self.tables.htmlOutputYes
                        else "",
                        i18n.hineinversetzenSatz,
                    ]
                else:
                    kette2 = [
                        "-",
                    ]

                self.relitable[num] += ["".join(kette2)]

        return self.relitable, rowsAsNumbers

    def concat1RowPrimUniverse2(
        self,
        relitable: list,
        rowsAsNumbers: set,
        generatedBefehle: set,
        htmlTagParaClassWoerter: list,
    ) -> tuple:
        """Fügt eine Spalte ein, in der Primzahlen mit Vielfachern
        auf dem Niveau des Universums nicht einfach nur aus einer
        CSV Tabelle geladen werden, sondern durch Primzahlen und
        deren Vielfachern generiert werden.

        @type relitable: list
        @param relitable: Haupttabelle self.relitable
        @return: relitable + weitere Tabelle daneben
        """
        self.relitable = relitable
        # alxp("gebrochen")

        hardCodedCouple = (10, 42)
        transzendentalienNrezi = (5, 131)
        if len(generatedBefehle) > 0:
            # self.tables.primUniverseRowNum = len(self.relitable[0])
            # self.tables.generatedSpaltenParameter_Tags[len(rowsAsNumbers)] = frozenset(
            #    {ST.sternPolygon, ST.galaxie}
            # )
            forGeneratedSpaltenParameter_Tags: dict = {
                "primMotivSternGebr": (
                    (0, 0, frozenset({ST.sternPolygon, ST.galaxie, ST.gebrRat}), 1),
                    (
                        0,
                        1,
                        frozenset(
                            {ST.sternPolygon, ST.galaxie, ST.universum, ST.gebrRat}
                        ),
                        1,
                    ),
                    (
                        0,
                        2,
                        frozenset(
                            {ST.sternPolygon, ST.galaxie, ST.universum, ST.gebrRat}
                        ),
                        1,
                    ),
                ),
                "primStrukSternGebr": (
                    (
                        0,
                        1,
                        frozenset(
                            {ST.sternPolygon, ST.galaxie, ST.universum, ST.gebrRat}
                        ),
                        1,
                    ),
                    (
                        0,
                        2,
                        frozenset(
                            {ST.sternPolygon, ST.galaxie, ST.universum, ST.gebrRat}
                        ),
                        1,
                    ),
                    (0, 3, frozenset({ST.sternPolygon, ST.universum, ST.gebrRat}), 1),
                ),
                "primMotivGleichfGebr": (
                    (
                        1,
                        0,
                        frozenset({ST.gleichfoermigesPolygon, ST.galaxie, ST.gebrRat}),
                        1,
                    ),
                    (
                        1,
                        1,
                        frozenset(
                            {
                                ST.gleichfoermigesPolygon,
                                ST.galaxie,
                                ST.universum,
                                ST.gebrRat,
                            }
                        ),
                        1,
                    ),
                    (
                        1,
                        2,
                        frozenset(
                            {
                                ST.gleichfoermigesPolygon,
                                ST.galaxie,
                                ST.universum,
                                ST.gebrRat,
                            }
                        ),
                        1,
                    ),
                ),
                "primStrukGleichfGebr": (
                    (
                        1,
                        1,
                        frozenset(
                            {
                                ST.gleichfoermigesPolygon,
                                ST.galaxie,
                                ST.universum,
                                ST.gebrRat,
                            }
                        ),
                        1,
                    ),
                    (
                        1,
                        2,
                        frozenset(
                            {
                                ST.gleichfoermigesPolygon,
                                ST.galaxie,
                                ST.universum,
                                ST.gebrRat,
                            }
                        ),
                        1,
                    ),
                    (
                        1,
                        3,
                        frozenset(
                            {ST.gleichfoermigesPolygon, ST.universum, ST.gebrRat}
                        ),
                        1,
                    ),
                ),
                "primMotivStern": (
                    (0, 0, frozenset({ST.sternPolygon, ST.galaxie}), 0),
                    (0, 1, frozenset({ST.sternPolygon, ST.galaxie, ST.universum}), 0),
                    (0, 2, frozenset({ST.sternPolygon, ST.galaxie, ST.universum}), 0),
                ),
                "primStrukStern": (
                    (0, 1, frozenset({ST.sternPolygon, ST.galaxie, ST.universum}), 0),
                    (0, 2, frozenset({ST.sternPolygon, ST.galaxie, ST.universum}), 0),
                    (0, 3, frozenset({ST.sternPolygon, ST.universum}), 0),
                ),
                "primMotivGleichf": (
                    (1, 0, frozenset({ST.gleichfoermigesPolygon, ST.galaxie}), 0),
                    (
                        1,
                        1,
                        frozenset(
                            {ST.gleichfoermigesPolygon, ST.galaxie, ST.universum}
                        ),
                        0,
                    ),
                    (
                        1,
                        2,
                        frozenset(
                            {ST.gleichfoermigesPolygon, ST.galaxie, ST.universum}
                        ),
                        0,
                    ),
                ),
                "primStrukGleichf": (
                    (
                        1,
                        1,
                        frozenset(
                            {ST.gleichfoermigesPolygon, ST.galaxie, ST.universum}
                        ),
                        0,
                    ),
                    (
                        1,
                        2,
                        frozenset(
                            {ST.gleichfoermigesPolygon, ST.galaxie, ST.universum}
                        ),
                        0,
                    ),
                    (1, 3, frozenset({ST.gleichfoermigesPolygon, ST.universum}), 0),
                ),
            }
            uni_ = (5, 131)
            gal_ = (10, 42)
            GalOrUni_nOrInvers = OrderedDict(
                {
                    0: (gal_, gal_),
                    1: (gal_, uni_),
                    2: (uni_, gal_),
                    3: (uni_, uni_),
                }
            )

            self.gebrRatAllCombis = self.findAllBruecheAndTheirCombinations()
            kombis2: OrderedDict = OrderedDict(
                {"mul": OrderedDict(), "div": OrderedDict()}
            )
            kombis1: OrderedDict = OrderedDict(
                {"stern": deepcopy(kombis2), "gleichf": deepcopy(kombis2)}
            )
            alleFractionErgebnisse2: OrderedDict = OrderedDict(
                {
                    "UniUni": deepcopy(kombis1),
                    "UniGal": deepcopy(kombis1),
                    "GalUni": deepcopy(kombis1),
                    "GalGal": deepcopy(kombis1),
                }
            )

            for KeyGalUniUniGal, ValueSternOrGleichf in self.gebrRatAllCombis.items():
                for KeySternOrGleichf, ValueMulOrDiv in ValueSternOrGleichf.items():
                    for KeyMulOrDiv, Couples in ValueMulOrDiv.items():
                        alleFractionErgebnisse2[KeyGalUniUniGal][KeySternOrGleichf][
                            KeyMulOrDiv
                        ] = (
                            self.combineDicts(
                                self.convertSetOfPaarenToDictOfNumToPaareMul(
                                    Couples,
                                    True if KeySternOrGleichf == "gleichf" else False,
                                ),
                                self.convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(
                                    self.BruecheUni
                                    if KeyGalUniUniGal[:3] == "Uni"
                                    else self.BruecheGal,
                                    self.BruecheUni
                                    if KeyGalUniUniGal[3:] == "Uni"
                                    else self.BruecheGal,
                                    True if KeySternOrGleichf == "gleichf" else False,
                                ),
                            )
                            if KeyMulOrDiv == "mul"
                            else self.combineDicts(
                                self.convertSetOfPaarenToDictOfNumToPaareDiv(
                                    Couples,
                                    True if KeySternOrGleichf == "gleichf" else False,
                                ),
                                OrderedDict(),
                            )
                        )

            """Wegen pypy3 == python3"""
            for key1, value1 in alleFractionErgebnisse2.items():
                for key2, value2 in value1.items():
                    for key3, value3 in value2.items():
                        for key4, value4 in value3.items():
                            alleFractionErgebnisse2[key1][key2][key3][key4] = tuple(
                                unique_everseen(value4, key=frozenset)
                            )
                            # value4 = list(value4)
                            # value4 = sort(value4)
                            # value4.sort()

            # ALXP HIER NOCH NICHT FERTIG

            # hier geht es um die html class Parameter und um Tagging ob Galaxie oder Polygon
            koord2tag: OrderedDict
            koord2ParameterA: OrderedDict
            koord2Parameter: OrderedDict

            koord2tag, koord2ParameterA, koord2Parameter = (
                OrderedDict(),
                OrderedDict(),
                OrderedDict(),
            )

            for name, mehrereEinraege in forGeneratedSpaltenParameter_Tags.items():
                for drei in mehrereEinraege:
                    try:
                        koord2tag[(drei[0], drei[1], drei[3])] |= {drei[2]}
                    except KeyError:
                        koord2tag[(drei[0], drei[1], drei[3])] = OrderedSet({drei[2]})
                for befehl in generatedBefehle:
                    if (
                        name == befehl
                    ):  # ob der Befehl des Users mit den jeweils vorhandenen übereinstimmt
                        for drei in mehrereEinraege:
                            try:
                                koord2ParameterA[(drei[0], drei[1], drei[3])] |= {
                                    befehl
                                }
                            except KeyError:
                                koord2ParameterA[
                                    (drei[0], drei[1], drei[3])
                                ] = OrderedSet({befehl})

            for key, value in koord2tag.items():
                assert len(value) == 1
            for key, value in koord2ParameterA.items():
                koord2Parameter[key] = list(value)

            # stern vs gleichf:
            self.transzendentalien: dict = OrderedDict(
                {
                    i18n.polygone["Sternpolygone"]: [],
                    i18n.polygone["gleichförmige Polygone"]: [],
                }
            )

            relitableCopy = deepcopy(self.relitable[: self.tables.lastLineNumber + 1])
            kombisNamen: tuple = (
                i18n.kombisNamen["Motiv -> Motiv"],
                i18n.kombisNamen["Motiv -> Strukur"],
                i18n.kombisNamen["Struktur -> Motiv"],
                i18n.kombisNamen["Struktur -> Strukur"],
            )
            kombisNamen2: tuple = (
                "GalGal",
                "GalUni",
                "UniGal",
                "UniUni",
            )

            # self.rolle = []
            self.motivation: dict = OrderedDict(
                {
                    i18n.polygone["Sternpolygone"]: [],
                    i18n.polygone["gleichförmige Polygone"]: [],
                }
            )
            # self.ziel = []
            for zwei, (polytype, polytypename, transzType) in enumerate(
                zip(
                    hardCodedCouple,
                    [
                        i18n.polygone["Sternpolygone"],
                        i18n.polygone["gleichförmige Polygone"],
                    ],
                    transzendentalienNrezi,
                )
            ):
                for cols in self.relitable:
                    # self.rolle += [cols[19]]
                    self.transzendentalien[polytypename] += [cols[transzType]]
                    self.motivation[polytypename] += [cols[polytype]]
                    # self.ziel += [cols[11]]

            for brr, ganzOrGebr in enumerate(
                ["", i18n.faktorenbla[", mit Faktoren aus gebrochen-rationalen Zahlen"]]
            ):
                for zwei, (
                    polytype,
                    polytypename,
                    transzType,
                    sternOrGleichf,
                ) in enumerate(
                    zip(
                        hardCodedCouple,
                        [
                            i18n.polygone["Sternpolygone"],
                            i18n.polygone["gleichförmige Polygone"],
                        ],
                        transzendentalienNrezi,
                        kombis1.keys(),
                    )
                ):
                    kombi_ = []

                    # alle Kombis die von strukur oder motiven also 2x2 möglich sind
                    for i, cols in enumerate(self.relitable):
                        kombi_ += [
                            (
                                (
                                    self.motivation[polytypename][i],
                                    self.motivation[polytypename][i],
                                ),
                                (
                                    self.motivation[polytypename][i],
                                    self.transzendentalien[polytypename][i],
                                ),
                                (
                                    self.transzendentalien[polytypename][i],
                                    self.motivation[polytypename][i],
                                ),
                                (
                                    self.transzendentalien[polytypename][i],
                                    self.transzendentalien[polytypename][i],
                                ),
                            )
                        ]
                    kombis: tuple = tuple(kombi_)
                    # alle 2x2 kombis von motiven und struktur

                    for nullBisDrei, (kombiUeberschrift, GalUniKombis) in enumerate(
                        zip(kombisNamen, kombisNamen2)
                    ):
                        tag: frozenset = list(koord2tag[(zwei, nullBisDrei, brr)])[0]

                        self.tables.generatedSpaltenParameter_Tags[
                            len(rowsAsNumbers)
                        ] = tag
                        rowsAsNumbers |= {
                            len(self.relitable[0]),
                        }
                        if (zwei, nullBisDrei, brr) in koord2Parameter:
                            for i, cols in enumerate(relitableCopy):
                                if i == 0:
                                    into = [
                                        i18n.genMul["generierte Multiplikationen "],
                                        polytypename,
                                        " ",
                                        kombiUeberschrift,
                                        ganzOrGebr,
                                    ]
                                else:
                                    into = []
                                    if self.tables.htmlOutputYes:
                                        into += ["<ul>"]
                                    elif self.tables.bbcodeOutputYes:
                                        into += ["[list]"]
                                    if brr == 0:
                                        multipless = multiples(i)
                                        multipless.sort()
                                        for k, multi in enumerate(multipless):
                                            if (
                                                k > 0
                                                and not self.tables.htmlOutputYes
                                                and not self.tables.bbcodeOutputYes
                                            ):
                                                into += [i18n.ausserdem[", außerdem: "]]
                                            into += [
                                                "<li>"
                                                if self.tables.htmlOutputYes
                                                else "[*]"
                                                if self.tables.bbcodeOutputYes
                                                else "",
                                                "(",
                                                kombis[multi[0]][nullBisDrei][0]
                                                if len(
                                                    kombis[multi[0]][nullBisDrei][
                                                        0
                                                    ].strip()
                                                )
                                                > 3
                                                else "...",
                                                ") * (",
                                                kombis[multi[1]][nullBisDrei][1]
                                                if len(
                                                    kombis[multi[1]][nullBisDrei][
                                                        1
                                                    ].strip()
                                                )
                                                > 3
                                                else "...",
                                                ")",
                                                "</li>"
                                                if self.tables.htmlOutputYes
                                                else "",
                                            ]
                                    elif brr == 1:
                                        multipless = alleFractionErgebnisse2[
                                            GalUniKombis
                                        ][sternOrGleichf]["mul"]
                                        for k, multi in enumerate(
                                            zip_longest(
                                                multipless[i],
                                                fillvalue="",
                                            )
                                        ):
                                            multi = multi[0]
                                            try:
                                                multi[0]
                                                multi[1]
                                            except:
                                                continue

                                            von = self.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
                                                multi[0],
                                                GalOrUni_nOrInvers[nullBisDrei][zwei],
                                                self.readOneCSVAndReturn(
                                                    2 if nullBisDrei in (2, 3) else 3
                                                ),
                                                False
                                                if nullBisDrei in (2, 3)
                                                else True,
                                            )
                                            bis = self.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
                                                multi[1],
                                                GalOrUni_nOrInvers[nullBisDrei][zwei],
                                                self.readOneCSVAndReturn(
                                                    2 if nullBisDrei in (1, 3) else 3
                                                ),
                                                False
                                                if nullBisDrei in (1, 3)
                                                else True,
                                            )

                                            if von is not None and bis is not None:
                                                von = von.strip()
                                                bis = bis.strip()
                                                if len(von) > 3 and len(bis) > 3:
                                                    if (
                                                        k > 0
                                                        and not self.tables.htmlOutputYes
                                                        and not self.tables.bbcodeOutputYes
                                                        and len(into) > 0
                                                    ):
                                                        into += [
                                                            i18n.ausserdem[
                                                                "| außerdem: "
                                                            ]
                                                        ]
                                                    into += [
                                                        "<li>"
                                                        if self.tables.htmlOutputYes
                                                        else "[*]"
                                                        if self.tables.bbcodeOutputYes
                                                        else "" '"',
                                                        '"',
                                                        von,
                                                        '"',
                                                        # self.CSVsAlreadRead[place][
                                                        #    multi[0].numerator - 1
                                                        # ][multi[0].denominator - 1],
                                                        "<br>"
                                                        if self.tables.htmlOutputYes
                                                        and (
                                                            len(von) > 30
                                                            or len(bis) > 30
                                                        )
                                                        else " ",
                                                        "(",
                                                        str(multi[0]),
                                                        ")*(",
                                                        str(multi[1]),
                                                        ")",
                                                        "<br>"
                                                        if self.tables.htmlOutputYes
                                                        and (
                                                            len(von) > 30
                                                            or len(bis) > 30
                                                        )
                                                        else " ",
                                                        '"',
                                                        bis,
                                                        '"',
                                                        # self.CSVsAlreadRead[place][
                                                        #    multi[1].numerator - 1
                                                        # ][multi[1].denominator - 1],
                                                        '"',
                                                        "</li>"
                                                        if self.tables.htmlOutputYes
                                                        else "",
                                                    ]
                                    if self.tables.htmlOutputYes:
                                        into += ["</ul>"]
                                    elif self.tables.bbcodeOutputYes:
                                        into += ["[/list]"]

                                self.relitable[i] += ["".join(into)]

                            if (
                                len(self.tables.generatedSpaltenParameter)
                                + self.tables.SpaltenVanillaAmount
                                in self.tables.generatedSpaltenParameter
                            ):
                                raise ValueError
                            kette = (
                                [
                                    (
                                        i18n.Multiplikationen_["Multiplikationen"],
                                        htmlTagParaClassWoerter[para][0][0][0][1],
                                    )
                                ]
                                for para in koord2Parameter[(zwei, nullBisDrei, brr)]
                            )
                            if (
                                "primMotivStern"
                                in koord2Parameter[(zwei, nullBisDrei, brr)]
                            ):
                                kette = list(kette) + [
                                    [
                                        (
                                            i18n.nWichtigste[
                                                "Wichtigstes_zum_verstehen"
                                            ],
                                            i18n.nWichtigste["Viertwichtigste"],
                                        )
                                    ]
                                ]

                            self.tables.generatedSpaltenParameter[
                                len(self.tables.generatedSpaltenParameter)
                                + self.tables.SpaltenVanillaAmount
                            ] = tuple(kette)

        return self.relitable, rowsAsNumbers

    def spalteMetaKontretTheorieAbstrakt_etc_1(
        self, relitable: list, rowsAsNumbers: set, geordnetePaare: set
    ):
        self.relitable = relitable
        self.rowsAsNumbers = rowsAsNumbers
        if len(geordnetePaare) > 0:
            self.gebrUnivTable4metaKonkret = self.readOneCSVAndReturn(nPmEnum.uniN)
        for paar in tuple(geordnetePaare):
            self.spalteMetaKontretTheorieAbstrakt_etc(
                relitable,
                rowsAsNumbers,
                paar[0],
                1 if paar[1] == 0 else 2 if paar[1] == 1 else 3,
            )
        return self.relitable, self.rowsAsNumbers

    def spalteMetaKonkretAbstrakt_isGanzZahlig(self, zahl, spaltenWahl) -> bool:
        zahl = (1 / zahl) if spaltenWahl else zahl
        zahl %= 1
        if zahl < 0.00001 or zahl > 0.99999:
            return True
        else:
            return False

    def spalteMetaKontretTheorieAbstrakt_etc(
        self,
        relitable: list,
        rowsAsNumbers: set,
        metavariable: int = 2,
        lower1greater2both3: int = 3,
    ) -> tuple:
        """
        1. nächste Zeile wird Anzeigezeile
        2. Diese Zeile erhält die Tags
        3. Vorwörternamen in Struktur: z.B. Meta-
        4. for alle Strukturalien und inverse Strukturalien
        5. ob diese beiden oder eins der beiden
        6. Überschriften und Tags: Schritt 1. und 2. in Schleife noch mal. <s>Dann ist doch das erste bei beiden schon eins zu viel!</s>
        7. Haupttabelle erhält schon direkt die Überschriften
        <s>8. Da waren 3 Zeilen Unsinn Code</s>

        9. for zeile 2 bis Unten und mit i++
        9. a) Vorwörter werden bestimmt
        9. a) 1. startet mit (zahl1,zahl1) und die eine wird dann immer größer und die andere kleiner
        9. a) 2. Fkt switching verdoppelt und halbiert (wenn Zahl 2) und wählt die spalte, ob strukturalie oder reziproke strukturalie
        9. a) 3. Fkt makeVorwort macht die Wiederholungen eines solchen Vorwortes
        9. a) 4.

        9. b) Text kommt in Zelle
        9. b) 1. wenn Ursprungszelle etwas enthält
        9. b) 2. Vorwörter, dann Transzendentalie, dann Zahl dazu in Klammern
        9. b) 3. das insofern in Schleife, dass mehreres Sowas in eine Zelle kann mit "|" dazwischen

        10. HTML Paramter werden aus dem dataDict genommen, damit sie für die html ausgabe genutzt werden können.

        Wie mache ich gebr rat univ rein?
        1. bei (zahl1,noch mal zahl1) bzw. (i,i) msste ich schauen, wenn ein wert None werden würde, weil er nicht mehr ganzzahlig wäre
        2. dann ein if für diesen fall und else wie gehabt.
        3. im if fall die komma zahl bestimmen
        4. die kommazahl als rationalen bruch maximal vereinfachen mit rational zahl lib von python
        5. Vorwörter weiter spinnen lassen, aber schauen, dass es richtig gemacht wird
        5. a) dafür brauche ich eine iterator variable extra noch mal zur Kommazahl
        6. aus dem rechteck der großen tabelle mit den richtigen koordinaten das universum-wort raus nehmen

        Alle Schleifen ineinander noch mal
        for zeilen in 4 spalten aus 2 echten spalten, also beim zweiten mal verdreht, so dass es 4 sind
        darin

        for nur meta nur konkret oder beides auf ein mal
        darin funktion mainpart und funktion html parameter; mainpart:

        for relitable zeile 2, d.h. der nummer 2, d.h. überschrift und nummer 1 wird ausgelassen
        darin fkt 1. vorwortbehandlung 2. fkt insert texte; 1. vorwortbehandlung:

        wiederhole bis beides None ist, also meta hochzählen und konkret runter zählen
        2. fkt insert:

        for sammlung von variablen: hochzähl- und runterzählvariable, spalte - entweder strukturalie oder reziproke davon, und vorwörter

        darin if bedingungen: ob spalte strukturalie oder reziproke strukturalie
        """

        self.relitable = relitable
        # rowsAsNumbers |= {
        #    len(self.relitable[0]),
        # }
        # self.tables.generatedSpaltenParameter_Tags[len(rowsAsNumbers) - 1] = frozenset(
        #    {ST.sternPolygon, ST.universum}
        # )
        """bis hier hin waren es die Vorinitialisierungen von Variablen"""

        # def switching(metavariable: int, lower1greater2both3: int, row: int):
        def switching(newCol: int, moreAndLess: tuple) -> tuple:
            """2 neue Koordinaten der Tabelle durch 3 Parameter, d.h. einer, newCol, gilt für beide
            Immer eine halbierung und dopplung oder verdreifachung und ..., etc.
            und wechsel der Spalte von den 2 Spalten"""
            spaltenWahl: int
            newCol, spaltenWahl = (
                (self.transzendentalienSpalten[0], 0)
                if newCol == self.transzendentalienSpalten[1]
                else (self.transzendentalienSpalten[1], 1)
            )
            try:
                mulresult = moreAndLess[0] * metavariable
            except:
                pass
            a = (
                mulresult
                if not moreAndLess[0] is None and mulresult < len(relitable)
                # würde zu früh abbrechen and len((relitable[moreAndLess[0] * metavariable][newCol]).strip()) > 3
                else None
            )
            if (
                moreAndLess[1] is not None
                and moreAndLess[1] < 100
                and moreAndLess[1] > 0.01
            ):
                divresult = moreAndLess[1] / metavariable

                if (
                    newCol == self.transzendentalienSpalten[ifInvers]
                    and type(moreAndLess[1]) is not Fraction
                ):
                    moreAndLess = (moreAndLess[0], Fraction(1, moreAndLess[1]))

                b = (
                    Fraction(metavariable, moreAndLess[1])
                    if self.spalteMetaKonkretAbstrakt_isGanzZahlig(
                        moreAndLess[1], False
                    )
                    else Fraction(1, moreAndLess[1]) / Fraction(metavariable)
                )

                # b = (
                #    (
                #        int(divresult)
                #        if self.spalteMetaKonkretAbstrakt_isGanzZahlig(divresult, False)
                #        else (
                #            Fraction(metavariable, moreAndLess[1])
                #            # Fraction(1, moreAndLess[1]) * Fraction(metavariable)
                #            if self.spalteMetaKonkretAbstrakt_isGanzZahlig(
                #                moreAndLess[1], False
                #            )
                #            # else Fraction(moreAndLess[1] * metavariable)
                #            else Fraction(1, moreAndLess[1]) / Fraction(metavariable)
                #        )
                #    )
                #    if newCol != self.transzendentalienSpalten[ifInvers]
                #    and not self.spalteMetaKonkretAbstrakt_isGanzZahlig(
                #        moreAndLess[1], True
                #    )
                #    else Fraction(moreAndLess[1], metavariable)
                # )

            else:
                b = None
            # if newCol == self.transzendentalienSpalten[ifInvers]:
            #    b = 11

            if b is not None:
                if Fraction(b) in self.gebrRatEtwaSchonMalDabeiGewesen:
                    b = None
                else:
                    self.gebrRatEtwaSchonMalDabeiGewesen |= {Fraction(b)}

            moreAndLess = (a, b)

            return newCol, moreAndLess

        metaOrWhat = OrderedDict(
            {
                2: (
                    (i18n.metaOrWhat["Meta-Thema: "], i18n.metaOrWhat["Konkretes: "]),
                    (i18n.metaOrWhat["Meta-"], i18n.metaOrWhat["Konkret-"]),
                ),
                3: (
                    (i18n.metaOrWhat["Theorie-Thema: "], i18n.metaOrWhat["Praxis: "]),
                    (i18n.metaOrWhat["Theorie-"], i18n.metaOrWhat["Praxis-"]),
                ),
                4: (
                    (
                        i18n.metaOrWhat["Planungs-Thema: "],
                        i18n.metaOrWhat["Umsetzungs-Thema: "],
                    ),
                    (i18n.metaOrWhat["Planung-"], i18n.metaOrWhat["Umsetzung-"]),
                ),
                5: (
                    (
                        i18n.metaOrWhat["Anlass-Thema: "],
                        i18n.metaOrWhat["Wirkungs-Thema: "],
                    ),
                    (i18n.metaOrWhat["Anlass-"], i18n.metaOrWhat["wirkung-"]),
                ),
                6: (
                    (
                        i18n.metaOrWhat["Kraft-Gebung: "],
                        i18n.metaOrWhat["Verstärkungs-Thema: "],
                    ),
                    (i18n.metaOrWhat["Kraft-geben-"], i18n.metaOrWhat["Verstärkung-"]),
                ),
                7: (
                    (
                        i18n.metaOrWhat["Beherrschung: "],
                        i18n.metaOrWhat["Richtung-Thema: "],
                    ),
                    (i18n.metaOrWhat["beherrschend-"], i18n.metaOrWhat["Richtung-"]),
                ),
            }
        )

        def makeVorwort(
            wiederholungen: int, vorworte2: tuple, less1ormore2: int
        ) -> str:
            return (
                vorworte2[less1ormore2 - 1] * wiederholungen
                if wiederholungen > 1
                else vorworte2[less1ormore2 - 1]
            )

        """Haupt-Teil, das davor waren Vorbereitungen
        das große Durchiterieren beginnt durch die Tabelle mit anschließendem erweitern dieser, um Spalten"""
        self.struktAndInversSpalten: tuple = (5, 131)

        for ifInvers, self.transzendentalienSpalten in enumerate(
            (
                self.struktAndInversSpalten,
                (self.struktAndInversSpalten[1], self.struktAndInversSpalten[0]),
            )
        ):
            for bothRows in (
                [0, 1]
                if lower1greater2both3 == 3
                else [
                    0,
                ]
                if lower1greater2both3 == 1
                else [
                    1,
                ]
                if lower1greater2both3 == 2
                else []
            ):
                rowsAsNumbers = self.spalteMetaKonkretTheorieAbstrakt_mainPart(
                    bothRows,
                    ifInvers,
                    makeVorwort,
                    metaOrWhat,
                    metavariable,
                    relitable,
                    rowsAsNumbers,
                    switching,
                    self.transzendentalienSpalten,
                )

                self.spalteMetaKonkretTheorieAbstrakt_SetHtmlParameters(
                    lower1greater2both3, metavariable
                )

        return self.relitable, rowsAsNumbers

    def spalteMetaKonkretTheorieAbstrakt_SetHtmlParameters(
        self, lower1greater2both3, metavariable
    ):
        if lower1greater2both3 != 3:
            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = self.tables.dataDict[11][(metavariable, lower1greater2both3 - 1)]
        else:
            for both in (
                0,
                1,
            ):
                self.tables.generatedSpaltenParameter[
                    len(self.tables.generatedSpaltenParameter)
                    + self.tables.SpaltenVanillaAmount
                ] = self.tables.dataDict[4][(metavariable, both)]

    def spalteMetaKonkretTheorieAbstrakt_mainPart(
        self,
        bothRows,
        ifInvers,
        makeVorwort,
        metaOrWhat,
        metavariable,
        relitable,
        rowsAsNumbers,
        switching,
        transzendentalienSpalten,
    ):
        rowsAsNumbers = self.spalteMetaKonkretAbstrakt_UeberschriftenUndTags(
            bothRows, ifInvers, metavariable, rowsAsNumbers
        )

        self.transzendentalienSpalten = transzendentalienSpalten
        # for i, row in enumerate(relitable[2:], 2):
        #    moreAndLess = (i, i)  # 1. wert "*2" und 2. "/3"
        #    neue2KoordNeue2Vorwoerter: list = []
        for i, row in enumerate(relitable[2 : self.tables.lastLineNumber + 1], 2):
            self.gebrRatEtwaSchonMalDabeiGewesen = OrderedSet()
            moreAndLess = (i, i)  # 1. wert "*2" und 2. "/3"
            neue2KoordNeue2Vorwoerter: list = []
            newCol = self.transzendentalienSpalten[0]
            neue2KoordNeue2Vorwoerter = (
                self.spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta(
                    makeVorwort,
                    metaOrWhat,
                    metavariable,
                    moreAndLess,
                    neue2KoordNeue2Vorwoerter,
                    newCol,
                    switching,
                )
            )

            self.spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText(
                bothRows,
                i,
                ifInvers,
                neue2KoordNeue2Vorwoerter,
                relitable,
                self.transzendentalienSpalten,
            )
        return rowsAsNumbers

    def spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta(
        self,
        makeVorwort,
        metaOrWhat,
        metavariable,
        moreAndLess,
        neue2KoordNeue2Vorwoerter,
        newCol,
        switching,
    ):
        while not (moreAndLess[0] is None and moreAndLess[1] is None):
            newCol, moreAndLess = switching(newCol, moreAndLess)
            # if type(moreAndLess[1]) is Fraction and moreAndLess[1] is not None:
            #    gebrStrukWort = (
            #        self.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
            #            moreAndLess[1]
            #        )
            #    )
            #    if gebrStrukWort is None or len(gebrStrukWort.strip()) < 4:
            #        moreAndLess = (moreAndLess[0], None)
            #        if moreAndLess[0] is None and moreAndLess[1] is None:
            #            break
            vorworte2 = metaOrWhat[metavariable][
                0 if len(neue2KoordNeue2Vorwoerter) == 0 else 1
            ]
            vorwort1: str = makeVorwort(
                len(neue2KoordNeue2Vorwoerter) + 1, vorworte2, 1
            )
            vorwort2: str = makeVorwort(
                len(neue2KoordNeue2Vorwoerter) + 1, vorworte2, 2
            )
            neue2KoordNeue2Vorwoerter += [(moreAndLess, newCol, vorwort1, vorwort2)]
        return neue2KoordNeue2Vorwoerter

    def spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText(
        self,
        bothRows,
        i,
        ifInvers,
        neue2KoordNeue2Vorwoerter,
        relitable,
        transzendentalienSpalten,
    ):
        intoList = []
        thema = ""
        self.transzendentalienSpalten = transzendentalienSpalten

        for vier in neue2KoordNeue2Vorwoerter[:-1]:
            if (
                bothRows == 0  # bei meta aber nicht bei konkret
                and not vier[0][0] is None
                and len(relitable[vier[0][0]][vier[1]].strip()) > 3
                # and False
            ):
                intoList += [
                    "<li>"
                    if self.tables.htmlOutputYes
                    else "[*]"
                    if self.tables.bbcodeOutputYes
                    else "",
                    vier[bothRows + 2],
                    thema,
                    relitable[vier[0][0]][vier[1]],
                    " (",
                    "1/"
                    if vier[1] != self.transzendentalienSpalten[ifInvers]
                    and vier[0][1] != 1
                    else "",
                    str(vier[0][0]),
                    ")",
                    "</li>"
                    if self.tables.htmlOutputYes
                    else " | "
                    if not self.tables.bbcodeOutputYes
                    else "",
                ]
            elif (
                bothRows == 1  # bei konkret aber nicht meta
                and not vier[0][1] is None
                and not type(vier[0][1]) is Fraction
                and len(relitable[vier[0][1]][vier[1]].strip()) > 3
                # and False
            ):
                intoList += [
                    "<li>"
                    if self.tables.htmlOutputYes
                    else "[*]"
                    if self.tables.bbcodeOutputYes
                    else "",
                    vier[bothRows + 2],
                    thema,
                    relitable[vier[0][1]][vier[1]],
                    " (",
                    "1/"
                    if vier[1] != self.transzendentalienSpalten[ifInvers]
                    and vier[0][1] != 1
                    else "",
                    str(vier[0][1]),
                    ")",
                    "</li>"
                    if self.tables.htmlOutputYes
                    else " | "
                    if not self.tables.bbcodeOutputYes
                    else "",
                ]
            elif (
                bothRows == 1  # bei konkret aber nicht meta
                and not vier[0][1] is None
                and type(vier[0][1]) is Fraction
                # and False
                # and self.struktAndInversSpalten == transzendentalienSpalten
            ):
                gebrStrukWort = (
                    self.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
                        vier[0][1],
                        self.struktAndInversSpalten,
                        self.gebrUnivTable4metaKonkret,
                        False,
                    )
                )
                if gebrStrukWort is not None:
                    if len(gebrStrukWort.strip()) > 3:
                        # sys.stderr.write("gebrRatMulStern")
                        intoList += [
                            "<li>"
                            if self.tables.htmlOutputYes
                            else ""
                            if not self.tables.bbcodeOutputYes
                            else "[*]",
                            vier[bothRows + 2],
                            thema,
                            gebrStrukWort,
                            "(",
                            str(vier[0][1].numerator),
                            ("/" + str(vier[0][1].denominator))
                            if vier[0][1].denominator > 1
                            else "",
                            ")",
                            "</li>"
                            if self.tables.htmlOutputYes
                            else " | "
                            if not self.tables.bbcodeOutputYes
                            else "",
                        ]
                    else:
                        pass
                else:
                    pass
                    # sys.stderr.write("gebrRatDivStern")
                    # vier[0][1] = None
                    # vier[0] = (vier[0][0], None)
                    # intoList = [None]
            thema = i18n.themaWort
        self.relitable[i] += [
            "".join(
                (
                    ["<ul>"]
                    if self.tables.htmlOutputYes
                    else [""]
                    if not self.tables.bbcodeOutputYes
                    else ["[list]"]
                )
                + intoList
                + (
                    ["</ul>"]
                    if self.tables.htmlOutputYes
                    else [""]
                    if not self.tables.bbcodeOutputYes
                    else ["[/list]"]
                )
            )
        ]

    def getAllBrueche(self, gebrUnivTable4metaKonkret):
        menge = OrderedSet()
        for i, a in enumerate(gebrUnivTable4metaKonkret[1:]):
            for k, b in enumerate(a[1:]):
                b = b.strip()
                if len(b) > 3:
                    frac = Fraction(i + 2, k + 2)
                    if frac.denominator != 1 and frac.numerator != 1:
                        menge |= {frac}
        # x("BRUCH", menge)
        return menge

    def readOneCSVAndReturn(self, wahl) -> list:
        place = self.readConcatCSV_choseCsvFile(wahl)
        if place in self.CSVsAlreadRead:
            return self.CSVsAlreadRead[place]
        else:
            with open(place, mode="r", encoding="utf-8") as csv_file:
                gebrRatTable = list(csv.reader(csv_file, delimiter=";"))
            self.CSVsAlreadRead[place] = gebrRatTable
            if wahl in nPmEnum.uni():
                self.BruecheUni = tuple(self.getAllBrueche(gebrRatTable))

            if wahl in nPmEnum.gal():
                self.BruecheGal = tuple(self.getAllBrueche(gebrRatTable))

            if wahl in nPmEnum.emo():
                self.BruecheEmo = tuple(self.getAllBrueche(gebrRatTable))

            if wahl in nPmEnum.groe():
                self.BruecheStrukGroesse = tuple(self.getAllBrueche(gebrRatTable))

            return gebrRatTable

    def findAllBruecheAndTheirCombinations(self):
        self.readOneCSVAndReturn(nPmEnum.galN)
        self.readOneCSVAndReturn(nPmEnum.uniN)
        # self.readOneCSVAndReturn(6)
        # self.readOneCSVAndReturn(7)
        kombis2 = OrderedDict({"mul": OrderedSet(), "div": OrderedSet()})
        kombis1 = OrderedDict(
            {"stern": deepcopy(kombis2), "gleichf": deepcopy(kombis2)}
        )
        gebrRatAllCombis = OrderedDict(
            {
                "UniUni": deepcopy(kombis1),
                "UniGal": deepcopy(kombis1),
                "GalUni": deepcopy(kombis1),
                "GalGal": deepcopy(kombis1),
            }
        )

        for brueche1, brueche2, GalOrUni1, GalOrUni2 in zip(
            (self.BruecheGal, self.BruecheGal, self.BruecheUni, self.BruecheUni),
            (self.BruecheGal, self.BruecheUni, self.BruecheGal, self.BruecheUni),
            ("Gal", "Gal", "Uni", "Uni"),
            ("Gal", "Uni", "Gal", "Uni"),
        ):
            brueche1 = list(brueche1)
            brueche2 = list(brueche2)
            brueche1.sort()
            brueche2.sort()
            for BruecheUn in brueche1:
                for BruecheUn2 in brueche2:
                    if BruecheUn != BruecheUn2:
                        couple = OrderedSet({(BruecheUn, BruecheUn2)})
                        if (
                            round(BruecheUn * BruecheUn2)
                            == round(BruecheUn * BruecheUn2 * 1000) / 1000
                        ):
                            gebrRatAllCombis[GalOrUni1 + GalOrUni2]["stern"][
                                "mul"
                            ] |= deepcopy(couple)

                        if round(BruecheUn / BruecheUn2) == round(
                            BruecheUn / BruecheUn2 * 1000
                        ):
                            gebrRatAllCombis[GalOrUni1 + GalOrUni2]["stern"][
                                "div"
                            ] |= deepcopy(couple)

                        if round(1 / (BruecheUn * BruecheUn2)) == (
                            round(1000 / (BruecheUn * BruecheUn2)) / 1000
                        ):
                            gebrRatAllCombis[GalOrUni1 + GalOrUni2]["gleichf"][
                                "mul"
                            ] |= deepcopy(couple)

                        if round(1 / (BruecheUn / BruecheUn2)) == (
                            round(1000 / (BruecheUn / BruecheUn2)) / 1000
                        ):
                            gebrRatAllCombis[GalOrUni1 + GalOrUni2]["gleichf"][
                                "div"
                            ] |= deepcopy(couple)

        """
        for a in gebrRatAllCombis["UniUni"]["stern"]["mul"]:
            a = list(a)
            a = a[0] * a[1]
            assert a == round(a)
        """

        return gebrRatAllCombis

    def spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
        self,
        koord: Fraction,
        n_and_invers_spalten,
        gebrTable4metaKonkretAndMore,
        isNotUniverse=True,
    ) -> str:
        isUniverse = not isNotUniverse
        if koord.denominator == 0 or koord.numerator == 0:
            return ""
        elif koord.denominator > 100 or koord.numerator > 100:
            return None
        elif koord.numerator == 1:
            if (
                len(self.relitable[koord.denominator][n_and_invers_spalten[1]].strip())
                > 3
            ):
                strukname = (
                    (
                        self.relitable[koord.denominator][n_and_invers_spalten[1]],
                        " (1/",
                        str(koord.denominator),
                        ")",
                        "; "
                        if len(self.relitable[koord.denominator][201]) > 2
                        and not self.tables.htmlOutputYes
                        else "",
                        "<br>"
                        if len(self.relitable[koord.denominator][201]) > 2
                        and self.tables.htmlOutputYes
                        else "",
                        self.relitable[koord.denominator][201],
                    )
                    if isUniverse
                    else (self.relitable[koord.denominator][n_and_invers_spalten[1]],)
                )
                return "".join(strukname)
            else:
                return ""
        elif koord.denominator == 1:
            if (
                len(self.relitable[koord.numerator][n_and_invers_spalten[0]].strip())
                > 3
            ):
                strukname = (
                    (
                        self.relitable[koord.numerator][n_and_invers_spalten[0]],
                        " (",
                        str(koord.numerator),
                        ")",
                        "; "
                        if len(self.relitable[koord.numerator][198]) > 2
                        and not self.tables.htmlOutputYes
                        else "",
                        "<br>"
                        if len(self.relitable[koord.numerator][198]) > 2
                        and self.tables.htmlOutputYes
                        else "",
                        self.relitable[koord.numerator][198],
                    )
                    if isUniverse
                    else (self.relitable[koord.numerator][n_and_invers_spalten[0]],)
                )
                # x("strukname_", strukname)
                return "".join(strukname)
            else:
                return ""
        else:
            try:
                return gebrTable4metaKonkretAndMore[koord.numerator - 1][
                    koord.denominator - 1
                ]
            except (KeyError, IndexError):
                return ""

    def spalteMetaKonkretAbstrakt_UeberschriftenUndTags(
        self, bothRows, ifInvers, metavariable, rowsAsNumbers
    ):
        rowsAsNumbers |= {len(self.relitable[0])}
        self.tables.generatedSpaltenParameter_Tags[len(rowsAsNumbers) - 1] = (
            frozenset(
                {
                    ST.gleichfoermigesPolygon if ifInvers else ST.sternPolygon,
                    ST.universum,
                }
            )
            if bothRows == 0
            else frozenset(
                {
                    ST.gleichfoermigesPolygon if ifInvers else ST.sternPolygon,
                    ST.universum,
                    ST.gebrRat,
                }
            )
        )
        self.relitable[1] += [""]
        if bothRows == 0:
            if metavariable == 2:
                self.relitable[0] += [i18n.metaKonkret["Meta"]]
            if metavariable == 3:
                self.relitable[0] += [i18n.metaKonkret["Theorie"]]
            if metavariable == 4:
                self.relitable[0] += [i18n.metaKonkret["Management"]]
            if metavariable == 5:
                self.relitable[0] += [i18n.metaKonkret["ganzheitlich"]]
            if metavariable == 6:
                self.relitable[0] += [
                    i18n.metaKonkret["Verwertung, Unternehmung, Geschäft"]
                ]
            if metavariable == 7:
                self.relitable[0] += [i18n.metaKonkret["regieren, beherrschen"]]
        if bothRows == 1:
            if metavariable == 2:
                self.relitable[0] += [i18n.metaKonkret["Konkretes"]]
            if metavariable == 3:
                self.relitable[0] += [i18n.metaKonkret["Praxis"]]
            if metavariable == 4:
                self.relitable[0] += [i18n.metaKonkret["verändernd"]]
            if metavariable == 5:
                self.relitable[0] += [i18n.metaKonkret["darüber hinaus gehend"]]
            if metavariable == 6:
                self.relitable[0] += [i18n.metaKonkret["wertvoll"]]
            if metavariable == 7:
                self.relitable[0] += [i18n.metaKonkret["Richtung"]]
        self.relitable[0][-1] += (
            i18n.metaKonkret[" für 1/n statt n"]
            if ifInvers == 1
            else i18n.metaKonkret[" für n"]
        )
        return rowsAsNumbers

    def spalteFuerGegenInnenAussenSeitlichPrim(
        self, relitable: list, rowsAsNumbers: set
    ) -> tuple:
        def PrimAnswer2(i: int) -> str:
            return self.lastPrimAnswers[i]

        def PrimAnswer(i: int) -> str:
            if i > 3:
                if self.primAmounts != self.oldPrimAmounts:
                    if self.primAmounts % 2 == 0:
                        return i18n.innenAussen["für innen"]
                    else:
                        return i18n.innenAussen["für außen"]
                else:
                    return ""
            elif i == 2:
                return i18n.innenAussen['"für seitlich und gegen Schwächlinge innen"']
            elif i == 3:
                return i18n.innenAussen['"gegen seitlich und für Schwächlinge innen"']
            elif i == 1:
                return i18n.innenAussen["für außen"]
            else:
                return ""

        self.relitable = relitable
        # extraSpalten = (5, 10, 42, 131, 138)
        extraSpalten = deepcopy(self.ones)
        extraSpalten = sorted(
            extraSpalten,
            key=lambda x: -1 * float("inf") if x is None else x,
        )
        spaltenNamen = OrderedDict(
            {
                5: i18n.spaltenNamen["Transzendentalien, Strukturalien, Universum n"],
                10: i18n.spaltenNamen["Galaxie n"],
                42: i18n.spaltenNamen["Galaxie 1/n"],
                131: i18n.spaltenNamen[
                    "Transzendentalien, Strukturalien, Universum 1/n"
                ],
                138: i18n.spaltenNamen[
                    "Dagegen-Gegen-Transzendentalien, Gegen-Strukturalien, Universum n"
                ],
                202: i18n.spaltenNamen[
                    "neutrale Gegen-Transzendentalien, Gegen-Strukturalien, Universum n"
                ],
                None: i18n.spaltenNamen["Richtung-Richtung"],
            }
        )
        tags = [
            (ST.sternPolygon, ST.universum),
            (ST.sternPolygon, ST.universum),
            (ST.sternPolygon, ST.galaxie),
            (ST.gleichfoermigesPolygon, ST.galaxie),
            (ST.gleichfoermigesPolygon, ST.universum),
            (ST.sternPolygon, ST.universum),
            (ST.sternPolygon, ST.universum),
        ]

        for r, kk in enumerate(extraSpalten):
            rowsAsNumbers |= {
                len(self.relitable[0]) + r,
            }
            self.tables.generatedSpaltenParameter_Tags[len(rowsAsNumbers) - 1] = tags[r]

        vergangenheit: list = []
        for kkk, kk in enumerate(extraSpalten):
            self.primAmounts = 0
            self.oldPrimAmounts = 0
            self.lastPrimAnswers: OrderedDict = OrderedDict()
            for i, cols in enumerate(relitable[: self.tables.lastLineNumber + 1]):
                into = (
                    [""]
                    if i != 0
                    else [
                        i18n.primRicht["Primzahlwirkung (7, Richtung) "],
                        spaltenNamen[kk],
                    ]
                )

                self.oldPrimAmounts = self.primAmounts
                if couldBePrimeNumberPrimzahlkreuz(i):
                    self.primAmounts += 1
                if primCreativity(i) == 1:
                    into = [PrimAnswer(i)]
                    self.lastPrimAnswers[i] = "".join(into)

                elif i > 1:
                    for couple in primRepeat(tuple(primFak(i))):
                        if couple[1] == 1:
                            into += [PrimAnswer2(couple[0]), " + "]
                        elif kk is not None:
                            into += [
                                str(relitable[couple[1]][kk]),
                                " * ",
                                PrimAnswer2(couple[0]),
                                " + ",
                            ]
                        else:
                            into += [
                                "[",
                                str(vergangenheit[couple[1]]),
                                i18n.letztEnd["] * letztendlich: "],
                                PrimAnswer2(couple[0]),
                                " + ",
                            ]
                    into = into[:-1]
                elif i == 1:
                    into = [PrimAnswer(1)]
                into = ["".join(into)]
                if kk is None:
                    vergangenheit += into
                self.relitable[i] += into

        for r, kk in enumerate(extraSpalten):
            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = self.tables.dataDict[4][(extraSpalten[r],)]

        return self.relitable, rowsAsNumbers

    def readConcatCsv_tabelleDazuColchange(
        self,
        zeilenNr: int,
        tabelleDazuCol: list,
        concatTable: int,
        ifTransponiert=False,
    ) -> list:
        tabelleDazuColNeu: list = []

        for i, cell in enumerate(tabelleDazuCol, 1):
            # x("transposeA", ifTransponiert)
            gebrRatZahl = (
                Fraction(zeilenNr, i) if not ifTransponiert else Fraction(i, zeilenNr)
            )
            # x("gebrRatZahl", gebrRatZahl)
            # x("whyNone", self.struktAndInversSpalten)
            cellNeu = self.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
                gebrRatZahl,
                self.struktAndInversSpalten,
                self.gebrUnivTable4metaKonkret,
                concatTable not in nPmEnum.uni(),
            )
            tabelleDazuColNeu += [cellNeu if cellNeu is not None else ""]

        return tabelleDazuColNeu

    def readConcatCsv(
        self,
        relitable: list,
        rowsAsNumbers: set,
        concatTableSelection: set,
        concatTable: int = 1,
    ) -> tuple:
        """Fügt eine Tabelle neben der self.relitable an
        momentan ist es noch fix auf primnumbers.csv
        aber das wird gerade geändert

        @type relitable: list
        @param relitable: Haupttabelle self.relitable
        @type rowsAsNumbers: set
        @param rowsAsNumbers: welche Spalten der neuen Tabelle dazu kommen sollen
        @rtype: list[list]
        @return: relitable + weitere Tabelle daneben
        """
        global folder
        # x("concatTableSelection", concatTableSelection)
        spaltenDict: dict = {
            1: None,
            nPmEnum.uniN: (5, 131),
            nPmEnum.uni1pN: (5, 131),
            nPmEnum.galN: (10, 42),
            nPmEnum.gal1pN: (10, 42),
            nPmEnum.emoN: (243, 284),
            nPmEnum.emo1pN: (243, 284),
            nPmEnum.groeN: (4, 197),
            nPmEnum.groe1pN: (4, 197),
        }
        if concatTable != 1:
            self.struktAndInversSpalten: tuple = spaltenDict[concatTable]
            self.gebrUnivTable4metaKonkret = self.readOneCSVAndReturn(concatTable)
            # x("WhyNone2", self.struktAndInversSpalten)

        def transpose(matrix):
            t = []
            x: int
            y: int
            for x in range(len(matrix[0])):
                t += [[]]
                for y in range(len(matrix)):
                    t[x] += [matrix[y][x]]
            return t

        self.relitable = relitable
        concatCSVspalten: set = OrderedSet()
        if len(concatTableSelection) > 0 and concatTable in range(1, 10):
            tableToAdd = self.readOneCSVAndReturn(concatTable)
            tableToAdd = self.readConcatCsv_ChangeTableToAddToTable(
                concatTable, tableToAdd, transpose
            )
            if concatTable == 1:
                tableToAdd2 = [[i18n.primVielGen["Primzahlvielfache, nicht generiert"]]]
                for t, zeile in enumerate(tableToAdd[1:], 1):
                    zeileNeu = []
                    for zelle in zeile:
                        if len(zelle.strip()) > 3:
                            zeileNeu += [
                                (
                                    "<li>"
                                    if self.tables.htmlOutputYes
                                    else ""
                                    if not self.tables.bbcodeOutputYes
                                    else "[*]"
                                )
                                + zelle
                                + ("</li>" if self.tables.htmlOutputYes else "")
                            ]
                    zeileNeu = [
                        (
                            ""
                            if self.tables.htmlOutputYes or self.tables.bbcodeOutputYes
                            else " | "
                        ).join(
                            (
                                ["<ul>"]
                                if self.tables.htmlOutputYes
                                else [""]
                                if not self.tables.bbcodeOutputYes
                                else ["[list]"]
                            )
                            + zeileNeu
                            + (
                                ["</ul>"]
                                if self.tables.htmlOutputYes
                                else [""]
                                if not self.tables.bbcodeOutputYes
                                else ["[/list]"]
                            )
                        )
                    ]
                    tableToAdd2 += [zeileNeu]
                tableToAdd = tableToAdd2

            self.relitable, tableToAdd = self.tables.fillBoth(
                self.relitable, tableToAdd
            )
            lastlen = 0
            maxlen = 0
            for i, (tabelleDazuCol, relicol) in enumerate(
                zip(tableToAdd, self.relitable)
            ):
                lastlen = len(tabelleDazuCol)
                if lastlen > maxlen:
                    maxlen = lastlen
                dazu = list(tabelleDazuCol) + [""] * (maxlen - len(tabelleDazuCol))

                if i != 0 and concatTable in range(2, 10):
                    dazu = self.readConcatCsv_tabelleDazuColchange(
                        i,
                        dazu,
                        concatTable,
                        concatTable in nPmEnum.einsPn()
                        and concatTable not in nPmEnum.n(),
                    )

                self.relitable[i] += dazu
                if i == 0:
                    for u, heading in enumerate(dazu):
                        # x("concatTableSelectionB", concatTableSelection)
                        self.readConcatCsv_LoopBody(
                            concatCSVspalten,
                            concatTable,
                            concatTableSelection,
                            dazu,
                            heading,
                            rowsAsNumbers,
                            u,
                        )

        # x("rowsAsNumbers", rowsAsNumbers)
        return self.relitable, rowsAsNumbers, concatCSVspalten

    def readConcatCSV_choseCsvFile(self, concatTable):
        place = os.path.join(
            os.getcwd(),
            os.path.dirname(__file__),
            "..",
            "csv",
            os.path.basename(
                csvNames.prim
                if concatTable == 1
                else csvNames.bruch13
                if concatTable in nPmEnum.gal()
                else csvNames.bruch15
                if concatTable in nPmEnum.uni()
                else csvNames.bruch7
                if concatTable in nPmEnum.emo()
                else csvNames.bruchStrukGroesse
                if concatTable in nPmEnum.groe()
                else None
            ),
        )
        return place

    def readConcatCsv_ChangeTableToAddToTable(self, concatTable, tableToAdd, transpose):
        if concatTable in nPmEnum.einsPn():
            tableToAdd = transpose(tableToAdd)
        if concatTable in range(2, 10):
            tableToAdd = [
                [
                    (
                        ("n/" + str(n + 1))
                        if concatTable in nPmEnum.n()
                        else (str(n + 1) + "/n")
                        if concatTable in nPmEnum.einsPn()
                        else i18n.GalOrUniOrFehler["Fehler"]
                    )
                    + (
                        " " + i18n.GalOrUniOrFehler["Universum"]
                        if concatTable in nPmEnum.uni()
                        else " " + i18n.GalOrUniOrFehler["Galaxie"]
                        if concatTable in nPmEnum.gal()
                        else " " + i18n.GalOrUniOrFehler["Emotion"]
                        if concatTable in nPmEnum.emo()
                        else " " + i18n.GalOrUniOrFehler["Strukturgroesse"]
                        if concatTable in nPmEnum.groe()
                        else i18n.GalOrUniOrFehler["Fehler"]
                    )
                    for n in range(len(tableToAdd[0]))
                ]
            ] + tableToAdd
        return tableToAdd

    def readConcatCsv_LoopBody(
        self,
        concatCSVspalten,
        concatTable,
        concatTableSelection,
        dazu,
        heading,
        rowsAsNumbers,
        u,
    ):
        # x("concatTable0", concatTable)
        # x("concatTableSelection_I", concatTableSelection)
        # x("u+2", u + 2)
        if (u + 2 in concatTableSelection and concatTable in range(2, 10)) or (
            concatTable == 1  # and int(heading) in concatTableSelection
        ):
            # x("concatTable1", concatTable)
            if concatTable not in range(2, 10) or u + 1 != len(dazu):
                delta = 1 if concatTable in range(2, 10) else 0
                selectedSpalten = u + len(self.relitable[0]) - len(dazu) + delta
                rowsAsNumbers.add(selectedSpalten)
                concatCSVspalten.add(selectedSpalten)
                # x("concatTable2", concatTable)
                if (
                    len(self.tables.generatedSpaltenParameter)
                    + self.tables.SpaltenVanillaAmount
                    in self.tables.generatedSpaltenParameter
                ):
                    raise ValueError

                self.readConcatCsv_SetHtmlParamaters(concatTable, heading, u)

    def readConcatCsv_SetHtmlParamaters(self, concatTable, heading, u):
        if concatTable in range(2, 10):
            rangeToDataDict = {2: 6, 3: 6, 4: 5, 5: 5, 6: 9, 7: 9, 8: 10, 9: 10}
            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
            ] = ([self.tables.dataDict[rangeToDataDict[concatTable]][u + 2][0][0]],)
            # print(self.tables.dataDict[rangeToDataDict[concatTable]][u + 2])
            # print(self.tables.dataDict[rangeToDataDict[concatTable]][u + 2][0][0])

        if concatTable == 1:
            intoHtmlPara = (
                [
                    (
                        i18n.multipl["Multiplikationen"],
                        i18n.notGen["Nicht_generiert"],
                    )
                ],
            )

            self.tables.generatedSpaltenParameter[
                len(self.tables.generatedSpaltenParameter)
                + self.tables.SpaltenVanillaAmount
                # ] = self.tables.dataDict[2][int(heading)]
            ] = intoHtmlPara
