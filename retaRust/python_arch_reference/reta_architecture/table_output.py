#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Table output rendering layer for Reta.

This module is the explicit table-output morphism layer extracted from the
legacy ``Tables.Output`` nested class.  It keeps the historical rendering
behaviour, but moves it out of ``libs/tableHandling.py`` so that the table
runtime can be treated as a global section while concrete output formats are
handled as renderer morphisms.
"""
from __future__ import annotations

import csv
import io
from collections import OrderedDict
from copy import deepcopy
from dataclasses import dataclass
from typing import Type

from .runtime_compat import cliout, getTextWrapThings, primfaktoren
from .output_syntax import (
    OutputSyntax,
    bbCodeSyntax,
    csvSyntax,
    emacsSyntax,
    htmlSyntax,
    markdownSyntax,
)
from .number_theory import moonNumber, primFak


class BreakoutException(Exception):
    pass


class TableOutput:
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
            parallel_result = None
            try:
                from .parallel_execution import select_columns_in_processes

                parallel_result = select_columns_in_processes(
                    table,
                    list(onlyThatColumns),
                    config=getattr(self.tables, "parallel_config", None),
                )
            except Exception as exc:  # pragma: no cover - serial fallback protects CLI parity.
                parallel_result = None
                try:
                    self.parallel_last_result = {
                        "mode": "serial_fallback",
                        "operation": "select_columns",
                        "error": f"{type(exc).__name__}: {exc}",
                    }
                except Exception:
                    pass
            if parallel_result is not None:
                try:
                    self.parallel_last_result = parallel_result.snapshot()
                except Exception:
                    pass
                return parallel_result.values if len(parallel_result.values) > 0 else table

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

        parallel_width_result = None
        try:
            from .parallel_execution import max_cell_text_len_in_processes

            width_source_table = list(newTable)[: len(finallyDisplayLinesSet)]
            parallel_width_result = max_cell_text_len_in_processes(
                width_source_table,
                rowsRange,
                config=getattr(self.tables, "parallel_config", None),
            )
        except Exception as exc:  # pragma: no cover - serial fallback protects CLI parity.
            parallel_width_result = None
            try:
                self.parallel_width_result = {
                    "mode": "serial_fallback",
                    "operation": "max_cell_text_len",
                    "error": f"{type(exc).__name__}: {exc}",
                }
            except Exception:
                pass
        if parallel_width_result is not None:
            maxCellTextLen = parallel_width_result.values
            try:
                self.parallel_width_result = parallel_width_result.snapshot()
            except Exception:
                pass
        else:
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
                quotechar='"',
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



@dataclass(frozen=True)
class TableOutputBundle:
    """Bootstrap handle for the table-output morphism layer."""

    output_class: Type[TableOutput] = TableOutput

    def create(self, tables, text_state):
        return self.output_class(tables, text_state)

    def snapshot(self) -> dict:
        return {
            "class": "TableOutputBundle",
            "output_class": self.output_class.__name__,
            "responsibility": "table-output-rendering-morphism",
            "legacy_nested_class": "Tables.Output",
        }


def bootstrap_table_output() -> TableOutputBundle:
    return TableOutputBundle()
