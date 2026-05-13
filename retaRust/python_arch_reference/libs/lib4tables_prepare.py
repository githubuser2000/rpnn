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
from reta_architecture.table_preparation import (
    cell_work as architecture_cell_work,
    prepare_output_table as architecture_prepare_output_table,
    prepare_row_cells as architecture_prepare_row_cells,
    select_display_lines as architecture_select_display_lines,
    tag_output_column as architecture_tag_output_column,
)
from reta_architecture.row_filtering import (
    delete_doubles_in_sets as architecture_delete_doubles_in_sets,
    filter_original_lines as architecture_filter_original_lines,
    from_until as architecture_from_until,
    moonsun as architecture_moonsun,
    parameters_cmd_with_some_bereich as architecture_parameters_cmd_with_some_bereich,
    set_zaehlungen as architecture_set_zaehlungen,
    zeile_which_zaehlung as architecture_zeile_which_zaehlung,
)
from reta_architecture.table_wrapping import (
    Wraptype,
    alxwrap as architecture_alxwrap,
    bootstrap_table_wrapping,
    chunks as architecture_chunks,
    get_shell_rows_amount as architecture_get_shell_rows_amount,
    refresh_textwrap_runtime as architecture_refresh_textwrap_runtime,
    set_shell_rows_amount as architecture_set_shell_rows_amount,
    set_wrapping_type as architecture_set_wrapping_type,
    split_more_if_not_small as architecture_split_more_if_not_small,
    width_for_row as architecture_width_for_row,
    wrap_cell_text as architecture_wrap_cell_text,
)

_table_wrapping = bootstrap_table_wrapping(force_refresh=True)
shellRowsAmount = _table_wrapping.runtime.shell_rows_amount
h_de = _table_wrapping.runtime.h_de
dic = _table_wrapping.runtime.dic
fill = _table_wrapping.runtime.fill
wrappingType: Wraptype = _table_wrapping.runtime.wrapping_type
# wrappingType: Wraptype = Wraptype.nohyphen
# wrappingType: Wraptype = Wraptype.pyphen


def _sync_wrapping_runtime():
    architecture_set_wrapping_type(wrappingType)
    return bootstrap_table_wrapping()


def setShellRowsAmount(shellRowsAmount2: Optional[str]):
    global shellRowsAmount
    shellRowsAmount = shellRowsAmount2
    architecture_set_shell_rows_amount(shellRowsAmount2)


def chunks(lst, n):
    return architecture_chunks(lst, n)


def splitMoreIfNotSmall(textList: list, lenToBe: int) -> tuple:
    return architecture_split_more_if_not_small(textList, lenToBe)


def alxwrap(text: str, len_: int):
    architecture_set_wrapping_type(wrappingType)
    return architecture_alxwrap(text, len_)


class Prepare:
    def __init__(self, tables, hoechsteZeile):
        global shellRowsAmount, h_de, dic, fill
        runtime = architecture_refresh_textwrap_runtime(wrapping_type=wrappingType)
        shellRowsAmount = runtime.shell_rows_amount
        h_de = runtime.h_de
        dic = runtime.dic
        fill = runtime.fill
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
    ):
        return architecture_set_zaehlungen(self, num)

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
        architecture_set_wrapping_type(wrappingType)
        return architecture_wrap_cell_text(text, length)

    def setWidth(self, rowToDisplay: int, combiRows1: int = 0) -> int:
        return architecture_width_for_row(self, rowToDisplay, combiRows1)

    def parametersCmdWithSomeBereich(
        self,
        MehrereBereiche: str,
        symbol: str,
        neg: str,
        keineNegBeruecksichtigung: bool = False,
    ) -> set:
        return architecture_parameters_cmd_with_some_bereich(
            self, MehrereBereiche, symbol, neg, keineNegBeruecksichtigung
        )

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
        return architecture_delete_doubles_in_sets(self, set1, set2)

    def fromUntil(self, a) -> tuple:
        return architecture_from_until(self, a)

    def zeileWhichZaehlung(self, zeile: int) -> int:
        return architecture_zeile_which_zaehlung(self, zeile)

    # ich wollte je pro extra num, nun nicht mehr nur sondern modular ein mal alles und dann pro nummer in 2 funktionen geteilt
    def moonsun(
        self, MoonNotSun: bool, numRangeYesZ: set, numRange, ifZaehlungenAtAll=True
    ):
        return architecture_moonsun(self, MoonNotSun, numRangeYesZ, numRange, ifZaehlungenAtAll)

    def FilterOriginalLines(self, numRange: set, paramLines: set) -> set:
        return architecture_filter_original_lines(self, numRange, paramLines)

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
        kombiCSVNumber: int = 0,
    ) -> tuple:
        return architecture_prepare_output_table(
            self,
            paramLines,
            paramLinesNot,
            contentTable,
            rowsAsNumbers,
            gebrSpalten,
            combiRows=combiRows,
            reliTableLenUntilNow=reliTableLenUntilNow,
            primSpalten=primSpalten,
            kombiCSVNumber=kombiCSVNumber,
        )

    def prepare4out_beforeForLoop_SpaltenZeilenBestimmen(
        self, contentTable, paramLines, paramLinesNot
    ):
        return architecture_select_display_lines(self, contentTable, paramLines, paramLinesNot)

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
        return architecture_prepare_row_cells(
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
        )

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
        return architecture_tag_output_column(
            self,
            combiRows,
            gebrSpalten,
            primSpalten,
            reliTableLenUntilNow,
            rowToDisplay,
            t,
            kombiCSVNumber,
        )

    def cellWork(self, cell: str, certaintextwidth: int) -> list:
        return architecture_cell_work(self, cell, certaintextwidth)
