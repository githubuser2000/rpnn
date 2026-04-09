#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import csv
import io
import os
import sys
from collections import OrderedDict
from copy import copy, deepcopy
from enum import Enum
from typing import Iterable, Union

import bbcode

try:
    from orderedset import OrderedSet
except:
    OrderedSet = set

from center import (
    Multiplikationen,
    alxp,
    cliout,
    getTextWrapThings,
    i18n,
    infoLog,
    output,
    primfaktoren,
    primRepeat,
    re,
    x,
)
from lib4tables import (
    OutputSyntax,
    NichtsSyntax,
    bbCodeSyntax,
    couldBePrimeNumberPrimzahlkreuz,
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
from lib4tables_concat import Concat
from lib4tables_Enum import ST
from lib4tables_prepare import Prepare, setShellRowsAmount, shellRowsAmount


class BreakoutException(Exception):
    pass


class Tables:
    @property
    def NichtsOutputYes(self) -> bool:
        return type(self.getOut.outType) is NichtsSyntax

    @property
    def markdownOutputYes(self) -> bool:
        return type(self.getOut.outType) is markdownSyntax

    @property
    def bbcodeOutputYes(self) -> bool:
        return type(self.getOut.outType) is bbCodeSyntax

    @property
    def htmlOutputYes(self) -> bool:
        return type(self.getOut.outType) is htmlSyntax

    @property
    def outType(self) -> OutputSyntax:
        return self.getOut.outType

    @outType.setter
    def outType(self, value: OutputSyntax):
        self.getOut.outType = value

    @property
    def hoechsteZeile(self):
        return self.__hoechsteZeile

    @hoechsteZeile.setter
    def hoechsteZeile(self, value: int):
        if type(value) is int or (type(value) is str and str(value).isdecimal):
            self.__hoechsteZeile = {1024: value, 114: value}

    @property
    def generRows(self):
        return self.__generRows__

    @generRows.setter
    def generRows(self, value: set):
        self.__generRows__ = value

    @property
    def ifPrimMultis(self):
        return self.getPrepare.ifprimmultis

    @property
    def ifZeilenSetted(self):
        return self.getPrepare.ifZeilenSetted

    @ifZeilenSetted.setter
    def ifZeilenSetted(self, value: bool):
        self.getPrepare.ifZeilenSetted = value

    @ifPrimMultis.setter
    def ifPrimMultis(self, value: bool):
        self.getPrepare.ifprimmultis = value

    # @property
    # def primUniversePrimsSet(self):
    #    return self.puniverseprims

    @property
    def gebrUnivSet(self):
        return self.gebrUniv

    @property
    def breitenn(self):
        return self.getOut.breiten

    @breitenn.setter
    def breitenn(self, value: list):
        # global shellRowsAmount
        shellRowsAmount, _, _, _ = getTextWrapThings()
        for i, v in enumerate(copy(value)):
            value[i] = (
                v
                if shellRowsAmount > v + 7 or shellRowsAmount == 0
                else shellRowsAmount - 7
            )
        self.getPrepare.breiten = value
        self.getOut.breiten = value

    @property
    def nummeriere(self):
        """# Nummerierung der Zeilen, z.B. Religion 1,2,3"""
        return self.getOut.nummerierung

    @nummeriere.setter
    def nummeriere(self, value: bool):
        self.getOut.nummerierung = value
        self.getPrepare.nummerierung = value
        self.nummerierung = value

    @property
    def textHeight(self):
        return self.getOut.textHeight

    @textHeight.setter
    def textHeight(self, value: int):
        self.getOut.textHeight = value

    @property
    def textWidth(self):
        return self.textwidth

    @textWidth.setter
    def textWidth(self, value: int):
        shellRowsAmount, _, _, _ = getTextWrapThings()
        value = (
            value
            if (shellRowsAmount > value + 7 or shellRowsAmount == 0)
            and (
                value != 0
                or (self.bbcodeOutputYes or self.htmlOutputYes or self.getOut.oneTable)
            )
            else shellRowsAmount - 7
        )
        self.getPrepare.textWidth = value
        self.getOut.textWidth = value
        self.textwidth = value

    @staticmethod
    def fillBoth(liste1, liste2) -> Iterable[Union[list, list]]:
        """eine der beiden Listen erhält so viele Listenelemente
        aus Strings dazu wie die andere hat, bis beide gleich viel haben

        @type liste1: list
        @param liste1: die erste Liste
        @type liste2: list
        @param liste2: die zweite Liste
        @rtype: tuple(list[str],list[str])
        @return: 2 Listen mit gleicher Länger, maximiert statt minimiert
        """
        while len(liste1) < len(liste2):
            liste1 += [""]
        while len(liste2) < len(liste1):
            liste2 += [""]
        return liste1, liste2

    def __init__(self, hoechstZeil, Txt):
        if hoechstZeil is None:
            self.__hoechsteZeile = {1024: 1024, 114: 163}
        else:
            self.__hoechsteZeile = {1024: hoechstZeil, 114: hoechstZeil}

        self.keineUeberschriften = False
        self.rowNumDisplay2rowNumOrig = OrderedDict()
        self.generatedSpaltenParameter = OrderedDict()
        self.generatedSpaltenParameter_Tags = OrderedDict()
        self.getPrepare = Prepare(self, self.hoechsteZeile)
        self.getCombis = self.Combi(self)
        self.getConcat = Concat(self)
        self.getOut = self.Output(self, Txt)
        self.getMainTable = self.Maintable(self)
        self.textHeight = 0
        self.textWidth = 21
        self.nummeriere = True
        self.spaltegGestirn = False
        self.breitenn: list = []
        # welche Spalten von "primenumbers.csv"
        # self.puniverseprims: set = OrderedSet()
        # self.getOut.primUniversePrimsSet = self.puniverseprims
        # self.getConcat.primUniversePrimsSet = self.puniverseprims
        # self.getConcat.gebrUnivSet = self.gebrUniv
        self.religionNumbers: list = []
        self.getOut.religionNumbers = self.religionNumbers
        self.getPrepare.religionNumbers = self.religionNumbers
        self.getCombis.religionNumbers = self.religionNumbers
        self.getPrepare.ifprimmultis = False
        self.getCombis.rowsOfcombi = OrderedSet()
        # self.getPrepare.rowsAsNumbers = OrderedSet()
        self.__generRows__: set = OrderedSet()

    class Output:
        def __init__(self, tables, Txt):
            self.tables = tables
            self.__oneTable = False
            self.__color = True
            self.__outType: OutputSyntax = OutputSyntax()
            self.Txt = Txt
            self.resultingTable = []

        @property
        def outType(self) -> OutputSyntax:
            return self.__outType

        @outType.setter
        def outType(self, value: OutputSyntax):
            self.__outType = value

        @property
        def color(self):
            return self.__color

        @color.setter
        def color(self, value: bool):
            self.__color = value

        @property
        def oneTable(self):
            return self.__oneTable

        @oneTable.setter
        def oneTable(self, value: bool):
            self.__oneTable = value

        # @property
        # def primUniversePrimsSet(self):
        #    return self.puniverseprims

        # @primUniversePrimsSet.setter
        # def primUniversePrimsSet(self, value: set):
        #    self.puniverseprims = value

        @property
        def breitenn(self):
            return self.breiten

        @breitenn.setter
        def breitenn(self, value: list):
            self.breiten = value

        @property
        def nummeriere(self):
            """# Nummerierung der Zeilen, z.B. Religion 1,2,3"""
            return self.nummerierung

        @nummeriere.setter
        def nummeriere(self, value):
            self.nummerierung = value

        @property
        def textHeight(self):
            return self.textheight

        @textHeight.setter
        def textHeight(self, value):
            self.textheight = value

        @property
        def textWidth(self):
            return self.textwidth

        @textWidth.setter
        def textWidth(self, value):
            global shellRowsAmount
            self.textwidth = value

        def onlyThatColumns(self, table, onlyThatColumns):
            if len(onlyThatColumns) > 0:
                newTable = []
                for row in table:
                    newCol = []
                    for i in onlyThatColumns:
                        try:
                            newCol += [deepcopy(row[i - 1])]
                        except IndexError:
                            pass
                    newTable += [newCol]
                if len(newTable) > 0:
                    return newTable
                else:
                    return table
            else:
                return table

        def cliOut(
            self,
            finallyDisplayLinesSet: set,
            newTable: list,
            numlen: int,
            rowsRange: range,
        ):
            """gibt eine Tabelle aus

            @type finallyDisplayLines: set
            @param finallyDisplayLines: Zeilen die ausgegeben werden sollen
            @type newRows: list
            @param newRows: Tabelle um die es geht
            @type rowsRange: set
            @param rowsRange: range(spaltenanzahl)
            @rtype:
            @return: nichts
            """
            global output, shellRowsAmount, h_de, dic, fill
            shellRowsAmount, h_de, dic, fill = getTextWrapThings()

            def findMaxCellTextLen(
                finallyDisplayLines: set, newTable: list, rowsRange: range
            ):
                """Gibt eine Liste mit Integern zurück, bzw. eigentlich ein Dict.
                Die Integer sind die Zellbreite, die an einer Stelle mindestens
                maximal war. Dieses Maximum wird zurück gegeben, als eine Zahl,
                bestimmt durch Überprüfung mehrerer Felder.

                @type finallyDisplayLines: set
                @param finallyDisplayLines: Zeilen die ausgegeben werden sollen
                @type newTable: list
                @param newTable: Tabelle um die es geht
                @type rowsRange: set
                @param rowsRange: range(spaltenanzahl)
                @rtype: dict[int,int]
                @return: Zellbreiten
                """
                maxCellTextLen: dict = OrderedDict()
                # for k in finallyDisplayLines: # n Linien einer Zelle, d.h. 1 EL = n Zellen
                for k, (f, r) in enumerate(
                    zip(newTable, finallyDisplayLines)
                ):  # n Linien einer Zelle, d.h. 1 EL = n Zellen
                    for iterWholeLine, m in enumerate(
                        rowsRange
                    ):  # eine Bildhschirm-Zeile immer
                        # for i in self.rowsAsNumbers: # SUBzellen: je Teil-Linie für machen nebeneinander als Teil-Spalten
                        for i, c in enumerate(
                            newTable[k]
                        ):  # SUBzellen: je Teil-Linie für machen nebeneinander als Teil-Spalten
                            if not i in maxCellTextLen:
                                try:
                                    maxCellTextLen[i] = len(newTable[k][i][m])
                                except:
                                    pass
                            else:
                                try:
                                    textLen = len(newTable[k][i][m])
                                    if textLen > int(maxCellTextLen[i]):
                                        maxCellTextLen[i] = textLen
                                except:
                                    pass
                return maxCellTextLen

            def determineRowWidth(i, maxCellTextLen):
                if i < len(self.breiten):
                    certaintextwidth = self.breiten[i]
                else:
                    certaintextwidth = self.textwidth
                if certaintextwidth > maxCellTextLen[i] or (
                    certaintextwidth == 0
                    and not self.tables.bbcodeOutputYes
                    and not self.tables.htmlOutputYes
                ):
                    i_textwidth = maxCellTextLen[i]
                else:
                    i_textwidth = certaintextwidth
                return i_textwidth

            if len(finallyDisplayLinesSet) == 0 or (
                len(finallyDisplayLinesSet) == 1 and 0 in finallyDisplayLinesSet
            ):
                return

            maxCellTextLen = findMaxCellTextLen(
                finallyDisplayLinesSet, newTable, rowsRange
            )
            self.finallyDisplayLines: list = list(finallyDisplayLinesSet)
            self.finallyDisplayLines.sort()
            shellRowsAmount -= (
                len(str(self.finallyDisplayLines[-1])) + 1
                if len(self.finallyDisplayLines) > 0 and shellRowsAmount != 0
                else 0
            )
            self.finallyDisplayLines[0] = ""
            lastSubCellIndex = -1
            lastlastSubCellIndex = -2
            headingfinished = False
            if type(self.__outType) is csvSyntax:
                strio = io.StringIO(newline="\n")
                writer = csv.writer(
                    strio,
                    quoting=csv.QUOTE_NONE,
                    delimiter=";",
                    quotechar="",
                    escapechar="\\",
                )
            while (
                len(newTable) > 0
                and lastSubCellIndex < len(newTable[0]) - 1
                and lastSubCellIndex > lastlastSubCellIndex
            ):
                if type(self.__outType) in (htmlSyntax, bbCodeSyntax):
                    self.cliout2(self.__outType.beginTable)
                lastlastSubCellIndex = lastSubCellIndex
                tabelleLeer = False
                try:
                    for (
                        BigCellLineNumber,
                        (TablesLineOfBigCells, filteredLineNumbersofOrignal),
                    ) in enumerate(
                        zip(newTable, self.finallyDisplayLines)
                    ):  # n Linien einer Zelle, d.h. 1 EL = n Zellen
                        if BigCellLineNumber == 0 and self.tables.keineUeberschriften:
                            continue
                        for iterWholeLine, OneWholeScreenLine_AllSubCells in enumerate(
                            rowsRange
                        ):  # eine Bildhschirm-Zeile immer
                            # x(
                            #    "äää",
                            #    [
                            #         BigCellLineNumber,
                            #         (
                            #             TablesLineOfBigCells,
                            #             filteredLineNumbersofOrignal,
                            #         ),
                            #     ],
                            # )

                            # alxp(self.tables.keineleereninhalte)
                            # alxp(self.tables.keineleereninhalte)
                            if (
                                BigCellLineNumber == 0
                                and (
                                    self.tables.keineleereninhalte
                                    or all(
                                        [
                                            len(element.strip()) == 0
                                            for row in newTable[1:]
                                            for element in row[lastlastSubCellIndex + 1]
                                            # if len(a) > 1
                                        ]
                                    )
                                )
                                and iterWholeLine == 0
                                and all(
                                    [
                                        len(element) < 2
                                        for row in newTable[1:]
                                        for element in row[lastlastSubCellIndex + 1]
                                        # if len(a) > 1
                                    ]
                                )
                            ):
                                tabelleLeer = True
                                self.cliout2("")
                                # self.cliout2(i18n.keineTabellenAusgabe + ": ")
                            # x("___", filteredLineNumbersofOrignal)
                            line = (
                                (
                                    (
                                        [
                                            self.__outType.generateCell(
                                                -2,
                                                self.tables.generatedSpaltenParameter,
                                                self.tables.getPrepare.zeileWhichZaehlung(
                                                    int(filteredLineNumbersofOrignal)
                                                ),
                                                zeile=filteredLineNumbersofOrignal,
                                                tables=self.tables,
                                            ),
                                            (
                                                "█"
                                                if type(self.__outType)
                                                in [OutputSyntax]
                                                else str(
                                                    self.tables.getPrepare.zeileWhichZaehlung(
                                                        int(
                                                            filteredLineNumbersofOrignal
                                                        )
                                                    )
                                                )
                                            ),
                                            self.__outType.endCell,
                                        ]
                                        if self.tables.getPrepare.zeileWhichZaehlung(
                                            int(filteredLineNumbersofOrignal)
                                        )
                                        % 2
                                        == 0
                                        else [
                                            self.__outType.generateCell(
                                                -2,
                                                self.tables.generatedSpaltenParameter,
                                                self.tables.getPrepare.zeileWhichZaehlung(
                                                    int(filteredLineNumbersofOrignal)
                                                ),
                                                zeile=filteredLineNumbersofOrignal,
                                                tables=self.tables,
                                            ),
                                            (
                                                " "
                                                if type(self.__outType)
                                                in [OutputSyntax]
                                                else str(
                                                    self.tables.getPrepare.zeileWhichZaehlung(
                                                        int(
                                                            filteredLineNumbersofOrignal
                                                        )
                                                    )
                                                )
                                            ),
                                            self.__outType.endCell,
                                        ]
                                    )
                                    if str(filteredLineNumbersofOrignal).isdecimal
                                    and filteredLineNumbersofOrignal != ""
                                    and int(filteredLineNumbersofOrignal) > 0
                                    else [
                                        self.__outType.generateCell(
                                            -2,
                                            self.tables.generatedSpaltenParameter,
                                            zeile=filteredLineNumbersofOrignal,
                                            tables=self.tables,
                                        ),
                                        " ",
                                        self.__outType.endCell,
                                    ]
                                )
                                if self.nummerierung
                                else [""]
                            )
                            linePlus = (
                                [""]
                                if not self.nummerierung
                                else [
                                    self.__outType.generateCell(
                                        -1,
                                        self.tables.generatedSpaltenParameter,
                                        zeile=filteredLineNumbersofOrignal,
                                        tables=self.tables,
                                    ),
                                    "".rjust(numlen + 1)
                                    if iterWholeLine != 0
                                    else (
                                        str(filteredLineNumbersofOrignal) + " "
                                    ).rjust(numlen + 1),
                                    self.__outType.endCell,
                                ]
                            )
                            if type(self.__outType) is csvSyntax:
                                line = ["".join(line), "".join(linePlus)]
                            else:
                                line += linePlus
                            rowsEmpty = 0
                            sumWidths = 0
                            lastSubCellIndex = 0
                            emptyEntries: int = 0
                            entriesHere: int = 0
                            for (
                                subCellIndexRightLeft,
                                subCellContentLeftRight,
                            ) in enumerate(
                                newTable[BigCellLineNumber]
                            ):  # SUBzellen: je Teil-Linie für machen nebeneinander als Teil-Spalten
                                if (
                                    subCellIndexRightLeft > lastlastSubCellIndex
                                    or self.__oneTable
                                ):
                                    subCellWidth = determineRowWidth(
                                        subCellIndexRightLeft, maxCellTextLen
                                    )
                                    sumWidths += subCellWidth + 1
                                    # if True:
                                    if sumWidths < shellRowsAmount or self.__oneTable:
                                        lastSubCellIndex = subCellIndexRightLeft
                                        try:
                                            entry = newTable[BigCellLineNumber][
                                                subCellIndexRightLeft
                                            ][OneWholeScreenLine_AllSubCells]
                                            entriesHere += 1
                                            if len(entry.strip()) == 0 or (
                                                self.tables.keineleereninhalte
                                                and len(entry.strip()) < 2
                                            ):
                                                emptyEntries += 1
                                            if (
                                                self.color
                                                and type(self.__outType) is OutputSyntax
                                            ):
                                                coloredSubCell = self.colorize(
                                                    entry.replace("\n", "").ljust(
                                                        subCellWidth
                                                    ),
                                                    filteredLineNumbersofOrignal,
                                                )
                                            elif type(self.__outType) is csvSyntax:
                                                coloredSubCell = newTable[
                                                    BigCellLineNumber
                                                ][subCellIndexRightLeft][
                                                    OneWholeScreenLine_AllSubCells
                                                ].replace(
                                                    "\n", ""
                                                )
                                            else:
                                                coloredSubCell = (
                                                    self.__outType.generateCell(
                                                        subCellIndexRightLeft,
                                                        self.tables.generatedSpaltenParameter,
                                                        zeile=filteredLineNumbersofOrignal,
                                                        tables=self.tables,
                                                    )
                                                    + (
                                                        entry.replace("\n", "").ljust(
                                                            subCellWidth
                                                        )
                                                    )
                                                    + self.__outType.endCell
                                                )
                                            if type(self.__outType) is csvSyntax:
                                                line += [coloredSubCell]
                                            else:
                                                line += [
                                                    coloredSubCell,
                                                    " ",
                                                ]  # neben-Einander
                                        except:
                                            rowsEmpty += 1
                                            if (
                                                self.color
                                                and type(self.__outType) is OutputSyntax
                                            ):
                                                coloredSubCell = self.colorize(
                                                    "".ljust(subCellWidth),
                                                    filteredLineNumbersofOrignal,
                                                    True,
                                                )
                                            else:
                                                coloredSubCell = (
                                                    self.__outType.generateCell(
                                                        subCellIndexRightLeft,
                                                        self.tables.generatedSpaltenParameter,
                                                        zeile=filteredLineNumbersofOrignal,
                                                        tables=self.tables,
                                                    )
                                                    + "".ljust(subCellWidth)
                                                    + self.__outType.endCell
                                                )
                                            if type(self.__outType) is csvSyntax:
                                                line += [coloredSubCell]
                                            else:
                                                line += [
                                                    coloredSubCell,
                                                    " ",
                                                ]  # neben-Einander
                                    else:
                                        rowsEmpty += 1
                                else:
                                    rowsEmpty += 1

                            if rowsEmpty != len(self.rowsAsNumbers) and (
                                iterWholeLine < self.textheight or self.textheight == 0
                            ):  # and m < actualPartLineLen:
                                if False and type(self.__outType) is markdownSyntax:
                                    line += [
                                        self.__outType.generateCell(
                                            subCellIndexRightLeft,
                                            self.tables.generatedSpaltenParameter,
                                            zeile=filteredLineNumbersofOrignal,
                                            tables=self.tables,
                                        )
                                    ]

                                    if BigCellLineNumber > 0 and not headingfinished:
                                        headingfinished = True
                                    if BigCellLineNumber == 0 and not headingfinished:
                                        addionalLine = [""]
                                        lineB = "".join(line)
                                        for ll in lineB:
                                            if ll != self.__outType.generateCell(
                                                subCellIndexRightLeft,
                                                self.tables.generatedSpaltenParameter,
                                                zeile=filteredLineNumbersofOrignal,
                                                tables=self.tables,
                                            ):
                                                addionalLine += ["-"]
                                            else:
                                                addionalLine += [
                                                    self.__outType.generateCell(
                                                        subCellIndexRightLeft,
                                                        self.tables.generatedSpaltenParameter,
                                                        zeile=filteredLineNumbersofOrignal,
                                                        tables=self.tables,
                                                    )
                                                ]

                                        line += ["\n"] + addionalLine
                                if emptyEntries != entriesHere:
                                    if type(self.__outType) is csvSyntax:
                                        strio = io.StringIO(newline="")
                                        writer = csv.writer(
                                            strio,
                                            quoting=csv.QUOTE_MINIMAL,
                                            delimiter=";",
                                            quotechar='"',
                                        )

                                        writer.writerow(line)
                                        self.cliout2(strio.getvalue())
                                    else:
                                        if (
                                            type(filteredLineNumbersofOrignal) is str
                                            and filteredLineNumbersofOrignal == ""
                                        ):
                                            filteredLineNumbersofOrignal = 0
                                        self.cliout2(
                                            "".join(
                                                [
                                                    self.__outType.coloredBeginCol(
                                                        filteredLineNumbersofOrignal
                                                    )
                                                ]
                                                + line
                                                + [self.__outType.endZeile]
                                            )
                                        )
                                        if (
                                            type(self.__outType) is markdownSyntax
                                            and BigCellLineNumber == 0
                                        ):
                                            self.cliout2(
                                                "|:--:"
                                                * (
                                                    len(newTable[BigCellLineNumber])
                                                    + (2 if self.nummerierung else 0)
                                                )
                                                + "|"
                                            )
                                        elif type(self.__outType) is emacsSyntax and (
                                            BigCellLineNumber == 0
                                            or (
                                                len(
                                                    set(
                                                        primfaktoren(
                                                            filteredLineNumbersofOrignal,
                                                            True,
                                                        )
                                                    )
                                                )
                                                == 1
                                                and len(
                                                    (
                                                        primfaktoren(
                                                            filteredLineNumbersofOrignal,
                                                            True,
                                                        )
                                                    )
                                                )
                                                != 1
                                            )
                                        ):
                                            self.cliout2(
                                                "|----"
                                                + (
                                                    "+----"
                                                    * (
                                                        (
                                                            len(
                                                                newTable[
                                                                    BigCellLineNumber
                                                                ]
                                                            )
                                                            + (
                                                                2
                                                                if self.nummerierung
                                                                else 0
                                                            )
                                                        )
                                                        - 1
                                                    )
                                                )
                                                + "|"
                                            )
                    if tabelleLeer and filteredLineNumbersofOrignal != 0:
                        # self.cliout2("".join(("(", i18n.keineTabellenAusgabe, ")")))
                        tabelleLeer = False
                        # if self.tables.keineleereninhalte:
                        self.cliout2("")
                except BreakoutException:
                    pass
                if type(self.__outType) in (htmlSyntax, bbCodeSyntax):
                    self.cliout2(
                        self.__outType.endTable,
                    )
                if False and type(self.__outType) is csvSyntax:
                    csvText = strio.getvalue()
                    self.cliout2(
                        csvText,
                    )
                if self.__oneTable:
                    break
            return self.resultingTable

        def cliout2(self, text):
            janee: tuple[bool, str] = (
                (True, "bbcode")
                if self.tables.bbcodeOutputYes
                else (True, "html")
                if self.tables.htmlOutputYes
                else (True, "markdown")
                if type(self.__outType) is emacsSyntax
                else (True, "markdown")
                if type(self.__outType) is csvSyntax
                else (True, "markdown")
                if self.tables.markdownOutputYes
                else (False, "")
            )
            self.resultingTable += [text]
            if not self.tables.NichtsOutputYes:
                cliout(text, self.color and janee[0], janee[1])

        def colorize(self, text, num: int, rest=False) -> str:
            """Die Ausagabe der Tabelle wird coloriert

            @type text: str
            @param text: der zu colorierende Text
            @type num: int
            @param num: die Zeilennummer, die coloriert werden soll
            @type rest: bool
            @param rest: andere Colorierung
            @rtype: str
            @return: der colorierte Text
            """
            num = int(num) if str(num).isdecimal() else 0
            if num == 0:
                return "\033[41m" + "\033[30m" + "\033[4m" + text + "\033[0m"
            elif rest:
                if num % 2 == 0:
                    return "\033[47m" + "\033[30m" + text + "\033[0m" + "\033[0m"
                else:
                    return "\033[40m" + "\033[37m" + text + "\033[0m" + "\033[0m"
            elif moonNumber(num)[1] != []:
                if num % 2 == 0:
                    return "\033[106m" + "\033[30m" + text + "\033[0m" + "\033[0m"
                else:
                    return "\033[46m" + "\033[30m" + text + "\033[0m" + "\033[0m"
            elif len(primFak(num)) == 1:
                if num % 2 == 0:
                    return "\033[103m" + "\033[30m" + "\033[1m" + text + "\033[0m"
                else:
                    return "\033[43m" + "\033[30m" + text + "\033[0m" + "\033[0m"
            elif num % 2 == 0:
                return "\033[47m" + "\033[30m" + text + "\033[0m" + "\033[0m"
            else:
                return "\033[100m" + "\033[37m" + text + "\033[0m" + "\033[0m"

    class Combi:
        def __init__(self, tables):
            self.sumOfAllCombiRowsAmount = 0
            self.tables = tables
            self.parameterName = i18n.hauptForNeben["kombination"]
            """alle  Schritte für kombi:
            1. lesen: KombiTable und relation, was von kombitable zu haupt gehört
                      und matrix mit zellen sind zahlen der kombinationen
                      d.h. 3 Sachen sind das Ergebnis
            2. prepare: die Zeilen, die infrage kommen für Kombi, d.h.:
                                    key = haupttabellenzeilennummer
                                    value = kombitabellenzeilennummer
            3. Zeilenumbruch machen, wie es bei der Haupt+Anzeige-Tabelle auch gemacht wurde
               prepare4out
            4. Vorbereiten des Joinens beider Tabellen direkt hier ( in völlig falsche Klasse ) rein programmiert
               (Müsste ich unbedingt mal refactoren!)
            5. joinen
            6. noch mal nur das ausgeben lassen, das nur ausgegeben werden soll
            7. letztendliche Ausagebe von allem!!
            """

        def prepareTableJoin(self, ChosenKombiLines, newTable_kombi_1):
            KombiTables = []
            for key, value in ChosenKombiLines.items():
                """Zeilennummern der kombi, die hinten dran kommen sollen
                an die Haupt- und Anzeigetabelle
                key = haupttabellenzeilennummer
                value = kombitabellenzeilennummer

                oder doch:
                key = zeilennummer der kombi csv
                value = alle n und m von n/m oder n
                """
                tables = OrderedDict()
                for kombiLineNumber in value:
                    """
                    alle kombitabellenzeilennummern hier durchiterieren
                    pro haupttabellenzeilennummer (diese umschließende Schleife)

                    into = eine neue Tabelle mit nur erlaubten Zeilen, gemacht
                    aus der Tabelle von der kombi.csv, die schon mit Zeilenumbrüchen
                    usw. vorbereitet wurde.
                    """

                    into = self.tables.tableReducedInLinesByTypeSet(
                        newTable_kombi_1, OrderedSet({kombiLineNumber})
                    )
                    """into = self.tabless.tableReducedInLinesByTypeSet(
                        animalsProfessionsTable, {kombiLineNumber}
                    )"""
                    if len(into) > 0:
                        if key in tables:
                            """Ergibt Matrix:
                            KombigesamttabelleMitZeilenumbruchVorbereitung[kombi.csv Zeilenummer][nur die relevanten Spaltens ihre erste Spalte ]
                            d.h. das ist aus kombi.csv die erste Spalte mit den Kombinationszahlen
                            die hier zugeordnet zu den kombi.csv zeilennummern gespeichert werden,
                            d.h. nicht den haupt+ausgabezeilen
                            """
                            tables[key] += [into[0]]
                        else:
                            tables[key] = [into[0]]
                    """ Liste aus Tabellen: eine Untertabelle = was in Haupttabellenzeilennummer rein soll aus der Kombitabelle
                    Zusammen ist das die Matrix der Kombis, die an die Haupt+Anzeige Tabelle deneben ran soll
                """
                KombiTables += [tables]
            return KombiTables

        def removeOneNumber(self, hinein: list, colNum: int) -> list:
            """
            Das hier muss noch mal umprogrammiert werden und anstelle den Text zu bearbeiten sollte
            eine extra Datenstruktur eingebunden werden, damit dann so etwas wie SQL AND und OR möglich werden kann.

            Wenn diese Datenstruktur bestehen wird, dann kann diese Algorithmus baumartig weiter geführt werden,
            was bedeutet, dass wenn eine Zahl mehr mals vorkommt bei allen Kombinationen pro einer Hauptzahl,
            dann soll das zusammen gefasst werden und dafür macht es auch Sinn,

            all die Zahlen zu sortieren, seitlich und oben-unten
            """
            if (
                len(hinein) > 0
                and (
                    (self.tables.textwidth == 0 and self.tables.getOut.oneTable)
                    or self.tables.htmlOutputYes
                    or self.tables.bbcodeOutputYes
                )
                and len(self.tables.breitenn) == 0
            ):
                hinein4 = deepcopy(hinein)
                hinein3 = []
                for zellenzeile in hinein:
                    if len(zellenzeile) > 0 and zellenzeile[-1] == "-":
                        zellenzeile = zellenzeile[:-1]
                    hinein3 += [zellenzeile]

                hineinNeu: list = []
                hineinStr = "".join(hinein3)
                hineinold = hineinStr
                bis: int = hineinStr.find(") ")
                von: int = hineinStr.find("(")
                substr = hineinStr[von + 1 : bis - von]
                substrListA = substr.split("|")
                if substrListA != [""]:
                    substrList = []
                    for el in substrListA:
                        if len(el) > 0 and el[0] == "(":
                            substrList += [el[1:-1]]
                        else:
                            substrList += [el]
                    substrListList = []
                    for listEl in substrList:
                        substrListList += [listEl.split("/")]
                    newNumListList: list = []
                    for liste in substrListList:
                        numListPart = []
                        for listEl in liste:
                            assert len(listEl.strip()) != 0
                            if abs(int(colNum)) != abs(int(listEl)) or len(liste) != 1:
                                numListPart += [listEl]

                        newNumListList += [numListPart]
                    newNumStrList: list = []
                    for newNumListEl in newNumListList:
                        into = "/".join(newNumListEl)
                        if len(into) > 0:
                            newNumStrList += [into]

                    newNumListStr = "|".join(newNumStrList)
                    if len(newNumStrList) > 0:
                        result = "(" + newNumListStr + hineinold[bis - von :]
                    else:
                        result = hineinold[bis - von + 1 :]

                    result2: list = []
                    if self.tables.textWidth != 0:
                        result2 += self.tables.getPrepare.cellWork(
                            result,
                            self.tables.getPrepare.certaintextwidth,
                        )
                    else:
                        result2 = [result.replace("\n", "; ")]

                    return result2

            return hinein

        def tableJoin(
            self,
            mainTable: list,
            manySubTables: list,
            maintable2subtable_Relation: list,
            old2newRows: list,
            rowsOfcombi,
        ) -> list:
            """Verbindet kombi tabelle mit haupttabelle
            @type mainTable: list
            @param mainTable: Haupttabelle, die angezeigt werden soll
            @type manySubTables: list
            @param manySubTables: Die Teiltabellen, von denen Teile pro Spalte in die Haupttabelle als Spalten und Zeilen rein sollen
            @type maintable2subtable_Relation: list
            @param maintable2subtable_Relation: Wie die Kombitabelle in die Haupttabelle rein kommen soll, d.h. hier sind die Verknüpfungspunkte beider Seiten enthalten
            @type old2newRows: list
            @param old2newRows: list
            @type rowsOfcombi: list
            @param rowsOfcombi: Welche Spalten der Kombitabelle in die Haupttabelle rein sollen
            @rtype table2: list[list]
            @return table2: Die resultierende gesamte später anzuzeigende Haupttabelle

            """
            rowsOfcombi = list(rowsOfcombi)
            rowsOfcombi.sort()
            table2 = mainTable
            """ Hätte ich mich gleich für SQL entschieden, oder hätte ich Pandas gewählt, dann hätte ich diesen Komplizierten Mist nicht programmieren müssen!
            """

            if type(self.tables.getOut.outType) in [
                htmlSyntax,
                bbCodeSyntax,
            ]:
                oneLinePerLine = True
            else:
                oneLinePerLine = False

            for colNum, (reliNum, col) in enumerate(
                zip(self.religionNumbers, mainTable)
            ):
                """geht die Zeilen der anzuzeigenden Haupttabelle durch
                1. Zeilenummer, 2. richtige Nummer der Religion (z.B: 1-10), 3. anzuzeigende Haupttabellenzeile
                """
                for subTable in manySubTables:
                    """Liste aus Tabellen: eine Untertabelle = was in Haupttabellenzeilennummer rein soll aus der Kombitabelle
                    Zusammen ist das die Matrix der Kombis, die an die Haupt+Anzeige Tabelle deneben ran soll

                    hier werden also alle Orginal-Haupt+Anzeige Zeilen durchgegangen
                    """
                    if reliNum in subTable:
                        """Wenn z.B. Religion 2 als Spalte 2 auch als Spalte 2 drin ist als Zelle der kombis die als Zelle in die Haupt+Anzeige Tabelle rein soll
                        d.h. hier die Frage ob z.B. 2==2    viel mehr ist das nicht"""
                        for row, bigCell in enumerate(mainTable[colNum]):
                            """HauptTabellenzeilen werden durchIteriert"""
                            if old2newRows[1][row] in maintable2subtable_Relation[0]:
                                """Wenn Haupttabellenzeile der Kombitabellenzeile entspricht"""
                                subRowNum = maintable2subtable_Relation[0][
                                    old2newRows[1][row]
                                ]
                                for subTableCell in subTable[reliNum]:
                                    """Die zu wählenden Religionen z.B. 1-10 durchiterieren
                                    und dessen zugehörige subTableZellen die als Zellen in die Haupt+Anzeige Tabelle rein sollen
                                    genomen
                                    """
                                    if rowsOfcombi.index(subRowNum + 1) < len(
                                        subTableCell
                                    ) and subTableCell != [[""]]:
                                        """Hier kommt jetzt endlich die Zelle in die Zelle rein:
                                        D.h. die Sache aus der Kombitabelle kommt in die Zelle der Haupt+Anzeige-Tabelle rein.
                                        Dabei ist die Zelle in die die Zelle rein kommt, widerum selbst eine kleine Tabelle, eigentlich.
                                        """
                                        hinein = deepcopy(
                                            subTableCell[
                                                rowsOfcombi.index(subRowNum + 1)
                                            ]
                                        )
                                        hinein = self.removeOneNumber(hinein, reliNum)
                                        if oneLinePerLine:
                                            if (
                                                len(hinein) > 0
                                                and len(hinein[0].strip()) > 2
                                            ):
                                                if self.tables.htmlOutputYes:
                                                    hinein[0] = (
                                                        "<li>" + hinein[0] + "</li>"
                                                    )
                                                elif self.tables.bbcodeOutputYes:
                                                    hinein[0] = "[*]" + hinein[0]
                                                else:
                                                    hinein[0] += " |"

                                            if (
                                                len(table2[colNum][row]) == 1
                                                and table2[colNum][row][0] == ""
                                            ):
                                                table2[colNum][row] = hinein
                                            else:
                                                table2[colNum][row][-1] += hinein[0]
                                        else:
                                            if (
                                                len(table2[colNum][row]) == 1
                                                and table2[colNum][row][0] == ""
                                            ):
                                                table2[colNum][row] = hinein
                                            else:
                                                table2[colNum][row] += hinein
                                if oneLinePerLine and self.tables.htmlOutputYes:
                                    for z, cell in enumerate(table2[colNum][row]):
                                        table2[colNum][row][z] = "<ul>" + cell + "</ul>"
                                elif oneLinePerLine and self.tables.bbcodeOutputYes:
                                    for z, cell in enumerate(table2[colNum][row]):
                                        table2[colNum][row][z] = (
                                            "[list]" + cell + "[/list]"
                                        )
                                elif self.tables.textWidth == 0 and (
                                    self.tables.getOut.oneTable
                                    or self.tables.textWidth > shellRowsAmount - 7
                                ):
                                    table2[colNum][row] = [
                                        " | ".join(table2[colNum][row])
                                    ]
            return table2

        def prepare_kombi(
            self,
            finallyDisplayLines_kombi_1: set,
            kombiTable: list,
            paramLines: set,
            displayingZeilen: set,
            kombiTable_Kombis: list,
        ) -> dict:
            """Vorbereiten zum Kombinieren von Tabellen, wie bei einem SQL-Join
            siehe hier Return-Value, der hiermit erstellt wird.
            Nur darum geht es.

            @type finallyDisplayLines: set
            @param finallyDisplayLines: set
            @type kombiTable: list
            @param kombiTable: Tabelle um die es geht, die zur Haupttabelle dazu kommt
            @type paramLines: set
            @param paramLines: Befehle die aus den Shell Paramentern konstruiert wurden
            @type displayingZeilen: set
            @param displayingZeilen: Zeilen die angezeigt werden sollen
            @type kombiTable_Kombis: list
            @param kombiTable_Kombis: wird anscheinend hier gar nicht gebraucht
            @rtype: dict[set[int]]
            @return: ZeilenNummern die miteinander als Join kombiniert werden sollen zwischen Haupttabelle und weiterer
                key = haupttabellenzeilennummer
                value = kombitabellenzeilennummer
            """

            ChosenKombiLines: dict = OrderedDict()
            for condition in paramLines:
                if condition in ("ka", "ka2"):
                    for kombiLineNumber, kombiLine in enumerate(kombiTable_Kombis):
                        """kombiLineNumber ist die csv Zeilennummer in der Kombitabelle
                        kombiLine ist aus der ersten Spalte die jeweilige Liste an Zahlenkombinationen pro Zeile
                        """
                        for kombiNumber in kombiLine:
                            """kombiNumber ist demzufolge eine so eine Zahl
                            von n*m Zahlen
                            if: wenn eine dieser Zahlen zu denen gehört, die am Ende angezeigt werden sollen und
                            wenn diese Zahl eine ist, die genau der richtigen Anzeigezeile entspricht
                            """

                            if kombiNumber in displayingZeilen:
                                try:
                                    """Zugehörig zur richtigen Anzeigeezeile wird diese Kombizeile ausgewählt
                                    d.h. anzeige in zeile enthält die richtige kombizeile
                                    NUMMERN werden da rein gelistet
                                    key = haupttabellenzeilennummer
                                    value = kombitabellenzeilennummer
                                    """
                                    ChosenKombiLines[kombiNumber] |= {
                                        kombiLineNumber + 1
                                    }
                                except KeyError:
                                    ChosenKombiLines[kombiNumber] = OrderedSet(
                                        {kombiLineNumber + 1}
                                    )
            return ChosenKombiLines

        def readKombiCsv(
            self,
            relitable: list,
            rowsAsNumbers: set,
            rowsOfcombi: set,
            csvFileName: str,
        ) -> tuple:
            """Fügt eine Tabelle neben der self.relitable nicht daneben sondern als join an, wie ein sql-join
            Hier wird aber noch nicht die join Operation durchgeführt
            momentan ist es noch fix auf animalsProfessions.csv

            @type relitable: list
            @param relitable: Haupttabelle self.relitable
            @type rowsAsNumbers: set
            @param rowsAsNumbers: welche Spalten  der Anzeigetabelle sind gewählt
            @type rowsOfcombi: set
            @param rowsOfcombi: welche Spalten der kombi Tabelle dazu kommen sollen
            @rtype: tuple[list,list,list,list]
            @return: neue Tabelle - die der kombi.csv entspricht, haupttabelle self.relitable, \
                Liste mit allen Zeilen der neuen Tabelle aus der ersten Spalte je Liste aus allem darin \
                das mit Komma getrennt wurde , was zu was gehört als Info für den join später
            return kombiTable, self.relitable, kombiTable_Kombis, maintable2subtable_Relation
            """

            global folder
            place = os.path.join(
                os.getcwd(),
                os.path.dirname(__file__),
                "..",
                "csv",
                os.path.basename(csvFileName),
            )

            self.sumOfAllCombiRowsAmount += len(rowsOfcombi)
            self.relitable = relitable
            headingsAmount = len(self.relitable[0])
            self.maintable2subtable_Relation: tuple = (OrderedDict(), OrderedDict())
            if len(rowsOfcombi) > 0:
                with open(place, mode="r", encoding="utf-8") as csv_file:
                    self.kombiTable: list = []
                    self.kombiTable_Kombis: list = []
                    for z, col in enumerate(csv.reader(csv_file, delimiter=";")):
                        """jede Zeile in der kombi.csv"""
                        for i, row in enumerate(col):
                            """jede Spalte also dann eigentlich Zelle der kombi.csv"""
                            if (
                                i > 0
                                and col[i].strip() != ""
                                and len(col[0].strip()) != 0
                            ):
                                col[i] = (
                                    "(" + col[0] + ") " + col[i] + " (" + col[0] + ")"
                                )
                        self.kombiTable += [col]
                        self.kombiTable_Kombis_Col: list = []
                        if len(col) > 0 and z > 0:
                            """die Behandlung des Auslesens von Religionsnummern in Kombination
                            in der ersten Spalte der kombi.csv"""
                            for num in col[0].split("|"):
                                """self.kombiTable_Kombis:
                                Liste mit allen Zeilen der neuen Tabelle aus der ersten
                                Spalte je Liste aus allem darin das mit Komma getrennt wurde
                                """
                                self.kombiNumbersCorrectTestAndSet(num)

                            self.kombiTable_Kombis += [self.kombiTable_Kombis_Col]
                    self.relitable, animalsProfessionsCol = Tables.fillBoth(
                        self.relitable, list(self.kombiTable)
                    )
                    lastlen = 0
                    maxlen = 0
                    for i, (animcol, relicol) in enumerate(
                        zip(animalsProfessionsCol, self.relitable)
                    ):
                        """jede Zeile bei der Haupttabellenzeile der Kombitabellenzeile (noch) NICHT richtig entspricht
                        beide sind auf die gleiche richtig Länge vorher verlängert worden.
                        (irgendwie komisch von mir programmiert)
                        """
                        if i == 0:
                            """Zur richtigen Zeile kommt der Leerraum rein,
                            der später aufgefüllt wird durch die wirklichen
                            Inhalte der kombi.csv
                            """
                            lastlen = len(animcol)
                            if lastlen > maxlen:
                                maxlen = lastlen
                            for t, ac in enumerate(animcol[1:]):
                                """Spalte hinten dran und nächste usw.
                                entspricht Spalte in Kombitabelle und umgehert
                                genauso, also Äquivalenz!
                                """
                                self.maintable2subtable_Relation[0][
                                    len(self.relitable[0]) + t
                                ] = t
                                self.maintable2subtable_Relation[1][t] = (
                                    len(self.relitable[0]) + t
                                )
                            self.relitable[0] += list(animcol[1:]) + [""] * (
                                maxlen - len(animcol)
                            )
                        else:
                            """Zur richtigen Zeile kommt der Leerraum rein,
                            der später aufgefüllt wird durch die wirklichen
                            Inhalte der kombi.csv
                            """
                            self.relitable[i] += len(animcol[1:]) * [""] + [""] * (
                                maxlen - len(animcol)
                            )
                        if i == 0:
                            for u, heading in enumerate(self.relitable[0]):
                                for a in rowsOfcombi:
                                    if (
                                        u >= headingsAmount
                                        and u == headingsAmount + a - 1
                                    ):
                                        rowsAsNumbers.add(u)
                                        """ rowsAsNumbers müsste hier verzeigert sein
                                        Es kommen genau diese Spaltennummern hinzu,
                                        (die überzählig sind) die nicht mehr in der
                                        anzuzeigenden tabelle entahlten sind also
                                        zu hoch wären, weil es die dazu kommenden
                                        Spalten der kombi.csv sind.
                                        """
                                        if (
                                            len(self.tables.generatedSpaltenParameter)
                                            + self.tables.SpaltenVanillaAmount
                                            in self.tables.generatedSpaltenParameter
                                        ):
                                            raise ValueError

                                        into: list = []
                                        into2: list = []

                                        if csvFileName == i18n.csvFileNames.kombi13:
                                            for (
                                                elementParameter
                                            ) in self.tables.dataDict[3][a]:
                                                into += [
                                                    (
                                                        i18n.tableHandling.into[
                                                            "Kombination_(Galaxie_und_schwarzes_Loch)_(14_mit_13)"
                                                        ],
                                                        elementParameter,
                                                    )
                                                ]

                                                if (
                                                    elementParameter
                                                    == i18n.tableHandling.into["tiere"]
                                                ):
                                                    into2 = [
                                                        (
                                                            i18n.tableHandling.into[
                                                                "Wichtigstes_zum_gedanklich_einordnen"
                                                            ],
                                                            i18n.tableHandling.into[
                                                                "Zweitwichtigste"
                                                            ],
                                                        )
                                                    ]
                                                elif elementParameter in [
                                                    i18n.tableHandling.into["berufe"],
                                                    i18n.tableHandling.into[
                                                        "intelligenz"
                                                    ],
                                                ]:
                                                    into2 = [
                                                        (
                                                            i18n.tableHandling.into[
                                                                "Wichtigstes_zum_gedanklich_einordnen"
                                                            ],
                                                            i18n.tableHandling.into[
                                                                "Zweitwichtigste"
                                                            ],
                                                        )
                                                    ]
                                        elif csvFileName == i18n.csvFileNames.kombi15:
                                            for (
                                                elementParameter
                                            ) in self.tables.dataDict[8][a]:
                                                into += [
                                                    (
                                                        i18n.tableHandling.into[
                                                            "Kombination_(Universum_und_Galaxie)_(14_mit_15)"
                                                        ],
                                                        elementParameter,
                                                    )
                                                ]

                                        self.tables.generatedSpaltenParameter[
                                            len(self.tables.generatedSpaltenParameter)
                                            + self.tables.SpaltenVanillaAmount
                                        ] = ((into,) if into2 == [] else (into, into2))

            else:
                self.kombiTable = [[]]
                self.kombiTable_Kombis = [[]]

            return (
                self.kombiTable,
                self.relitable,
                self.kombiTable_Kombis,
                self.maintable2subtable_Relation,
            )

        def kombiNumbersCorrectTestAndSet(self, num):
            num = num.strip()
            if len(num) > 2 and num[0] == "(" and num[-1] == ")":
                self.kombiNumbersCorrectTestAndSet(num[1:-1])
                return
            if num.isdecimal() or (
                len(num) > 0 and num[0] in ["+", "-"] and num[1:].isdecimal()
            ):
                """Nummer ... Liste mit alles Zahlen einer Religionskombination
                in eine Zeile pro Religionskombination und nicht bereits hier
                mit was eine Religion mit anderen Zahlen kombiniert werden würde,
                denn das kommt später und wird genau daraus hier gebaut.
                """
                self.kombiTable_Kombis_Col += [abs(int(num))]
            elif len(num) > 2 and "/" in num:
                self.kombiNumbersCorrectTestAndSet(num[: num.find("/")])
                self.kombiNumbersCorrectTestAndSet(num[num.find("/") + 1 :])
                return
            else:
                raise BaseException(
                    "Die kombi.csv ist in der ersten Spalte nicht so wie sie sein soll mit den Zahlen. "
                    + str(num)
                    + " "
                    + str(type(num))
                    + " "
                    + str(len(num))
                )

    class Maintable:
        def __init__(self, tables):
            self.tables = tables

        def createSpalteGestirn(self, relitable: list, rowsAsNumbers: set):
            """Fügt self.relitable eine Spalte hinzu, ob eine Zahl ein Mond oder eine Sonne ist
            Die Information muss dazu kommt aus moonNumber(i)[1]

            @type relitable: list
            @param relitable: Haupttabelle self.relitable
            @type rowsAsNumbers: set
            @param rowsAsNumbers: welche Spalten der neuen Tabelle nur betroffen sind
            @rtype:
            @return: nichts
            """
            self.relitable = relitable
            if set(rowsAsNumbers) >= {64}:
                if len(self.relitable) > 0:
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
                    rowsAsNumbers.add(len(self.relitable[0]))
                    self.tables.generatedSpaltenParameter_Tags[
                        len(rowsAsNumbers) - 1
                    ] = frozenset({ST.sternPolygon, ST.universum, ST.galaxie})

                self.relitable[0] += [i18n.tableHandling.gestirnGrossschrift["Gestirn"]]
                self.relitable[1] += [
                    i18n.tableHandling.gestirnGrossschrift["Sonne (keine Potenzen)"]
                ]
                for i, line in enumerate(self.relitable[2:]):
                    line1 = []
                    if i % 3 == 1:
                        line1 += [
                            i18n.tableHandling.gestirnGrossschrift[
                                "wäre eine schwarze Sonne (-3*n), wenn ins Negative durch eine Typ 13 verdreht"
                            ]
                        ]

                    if moonNumber(i)[1] != []:
                        line1 += [
                            i18n.tableHandling.gestirnGrossschrift["Mond (Potenzen)"]
                        ]
                    else:
                        line1 += [
                            i18n.tableHandling.gestirnGrossschrift[
                                "Sonne (keine Potenzen)"
                            ]
                        ]
                    if i % 2 == 0:
                        line1 += [
                            i18n.tableHandling.gestirnGrossschrift["Planet (2*n)"],
                        ]
                    line += [
                        i18n.tableHandling.gestirnGrossschrift[", und außerdem "].join(
                            line1
                        )
                    ]

    def tableReducedInLinesByTypeSet(self, table: list, linesAllowed: set):
        """nur Zeilen aus dem set aus der Tabelle verwenden als Ausgabe der Tabelle

        @type table: list[list]
        @param table: Tabelle
        @type table: set[int]
        @param table: erlaubte Zeilen
        @rtype: list[list]
        @return: neue Tabelle mit nur den Zeilen aus linesAllowed
        """
        newTable: list = []
        for i, line in enumerate(table):
            if i in linesAllowed:
                newTable += [line]
        return newTable
