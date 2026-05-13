#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from __future__ import annotations

"""Table runtime/global section layer for Reta.

Stage 25 moves the legacy ``Tables`` runtime out of ``libs/tableHandling.py``.
``Tables`` is the mutable global table section that ties the prepare, concat,
combi-join, generated-column and output morphisms together.  The old module is
kept as a compatibility facade for external imports.
"""

from collections import OrderedDict
from copy import copy
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Union

try:
    from orderedset import OrderedSet
except (ModuleNotFoundError, ImportError):
    OrderedSet = set

from .output_semantics import RetaOutputSemantics, bootstrap_output_semantics
from .output_syntax import OutputSyntax
from .table_output import TableOutput
from .combi_join import KombiJoin
from .generated_columns import bootstrap_generated_columns
from .table_state import TableStateBundle, bootstrap_table_state

OUTPUT_SEMANTICS = bootstrap_output_semantics(Path(__file__).resolve().parent.parent)


class BreakoutException(Exception):
    pass


def _get_text_wrap_things():
    from .runtime_compat import getTextWrapThings

    return getTextWrapThings()


def _prepare_class():
    from .table_adapters import Prepare

    return Prepare


def _concat_class():
    from .table_adapters import Concat

    return Concat


class Tables:
    """Legacy-compatible mutable table section.

    The class is intentionally still mutable because the historical Reta runtime
    updates table, row, output and generated-column state in-place.  What changed
    in Stage 25 is ownership: this object now belongs to the architecture layer,
    while ``libs.tableHandling`` merely re-exports it.
    """

    @property
    def keineUeberschriften(self) -> bool:
        return self._state_sections.display.keine_ueberschriften

    @keineUeberschriften.setter
    def keineUeberschriften(self, value: bool):
        self._state_sections.display.keine_ueberschriften = bool(value)

    @property
    def keineleereninhalte(self) -> bool:
        return self._state_sections.display.keine_leeren_inhalte

    @keineleereninhalte.setter
    def keineleereninhalte(self, value: bool):
        self._state_sections.display.keine_leeren_inhalte = bool(value)

    @property
    def spaltegGestirn(self) -> bool:
        return self._state_sections.display.spalte_gestirn

    @spaltegGestirn.setter
    def spaltegGestirn(self, value: bool):
        self._state_sections.display.spalte_gestirn = bool(value)

    @property
    def tableStateSnapshot(self) -> dict:
        return self._state_sections.snapshot()

    @property
    def outputModeName(self) -> str:
        semantics = getattr(getattr(self, "architecture", None), "output_semantics", OUTPUT_SEMANTICS)
        return semantics.mode_for_tables(self)

    @property
    def NichtsOutputYes(self) -> bool:
        return self.outputModeName == "nichts"

    @property
    def markdownOutputYes(self) -> bool:
        return self.outputModeName == "markdown"

    @property
    def bbcodeOutputYes(self) -> bool:
        return self.outputModeName == "bbcode"

    @property
    def htmlOutputYes(self) -> bool:
        return self.outputModeName == "html"

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

    @property
    def gebrUnivSet(self):
        return self.gebrUniv

    @property
    def breitenn(self):
        return self.getOut.breiten

    @breitenn.setter
    def breitenn(self, value: list):
        shell_rows_amount, _, _, _ = _get_text_wrap_things()
        for i, v in enumerate(copy(value)):
            value[i] = (
                v
                if shell_rows_amount > v + 7 or shell_rows_amount == 0
                else shell_rows_amount - 7
            )
        self.getPrepare.breiten = value
        self.getOut.breiten = value

    @property
    def nummeriere(self):
        """Nummerierung der Zeilen, z.B. Religion 1,2,3."""
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
        shell_rows_amount, _, _, _ = _get_text_wrap_things()
        value = (
            value
            if (shell_rows_amount > value + 7 or shell_rows_amount == 0)
            and (
                value != 0
                or (self.bbcodeOutputYes or self.htmlOutputYes or self.getOut.oneTable)
            )
            else shell_rows_amount - 7
        )
        self.getPrepare.textWidth = value
        self.getOut.textWidth = value
        self.textwidth = value

    @staticmethod
    def fillBoth(liste1, liste2) -> Iterable[Union[list, list]]:
        """Pad two lists with empty strings until both have the same length."""
        while len(liste1) < len(liste2):
            liste1 += [""]
        while len(liste2) < len(liste1):
            liste2 += [""]
        return liste1, liste2

    def __init__(self, hoechstZeil, Txt, state_bundle: TableStateBundle | None = None):
        self._state_bundle = state_bundle or bootstrap_table_state()
        self._state_sections = self._state_bundle.create_sections(hoechstZeil)
        self.__hoechsteZeile = self._state_sections.highest_rows

        Prepare = _prepare_class()
        Concat = _concat_class()

        self.rowNumDisplay2rowNumOrig = self._state_sections.row_display_to_original
        self.generatedSpaltenParameter = self._state_sections.generated_columns.parameters
        self.generatedSpaltenParameter_Tags = self._state_sections.generated_columns.tags
        self.getPrepare = Prepare(self, self.hoechsteZeile)
        self.getCombis = self.Combi(self)
        self.getConcat = Concat(self)
        self.getOut = TableOutput(self, Txt)
        self.getMainTable = self.Maintable(self)
        self.textHeight = 0
        self.textWidth = 21
        self.nummeriere = True
        self.spaltegGestirn = False
        self.breitenn: list = []
        self.religionNumbers: list = self._state_sections.display.religion_numbers
        self.getOut.religionNumbers = self.religionNumbers
        self.getPrepare.religionNumbers = self.religionNumbers
        self.getCombis.religionNumbers = self.religionNumbers
        self.getPrepare.ifprimmultis = False
        self.getCombis.rowsOfcombi = OrderedSet()
        self.__generRows__: set = self._state_sections.new_generated_rows()

    # Legacy compatibility: the implementation now lives in reta_architecture.combi_join.
    Combi = KombiJoin

    class Maintable:
        def __init__(self, tables):
            self.tables = tables
            self.generated_columns = bootstrap_generated_columns()

        def createSpalteGestirn(self, relitable: list, rowsAsNumbers: set):
            """Compatibility wrapper for the architecture-owned Gestirn morphism."""
            self.generated_columns.create_spalte_gestirn(
                self.tables,
                relitable,
                rowsAsNumbers,
            )

    def tableReducedInLinesByTypeSet(self, table: list, linesAllowed: set):
        """Return a table restricted to rows listed in ``linesAllowed``."""
        newTable: list = []
        for i, line in enumerate(table):
            if i in linesAllowed:
                newTable += [line]
        return newTable


@dataclass(frozen=True)
class TableRuntimeBundle:
    table_class: type = Tables
    output_semantics: RetaOutputSemantics = OUTPUT_SEMANTICS
    table_state: TableStateBundle = bootstrap_table_state()

    def create_tables(self, hoechst_zeil=None, txt=None) -> Tables:
        return self.table_class(hoechst_zeil, txt, state_bundle=self.table_state)

    def snapshot(self) -> dict:
        return {
            "class": self.__class__.__name__,
            "table_class": self.table_class.__name__,
            "owns_legacy_tables": True,
            "legacy_facade": "libs/tableHandling.py",
            "state_sections": self.table_state.snapshot(),
            "component_morphisms": [
                "Prepare",
                "Concat",
                "KombiJoin",
                "TableOutput",
                "GeneratedColumns",
            ],
        }


def bootstrap_table_runtime(
    output_semantics: RetaOutputSemantics | None = None,
    table_state: TableStateBundle | None = None,
) -> TableRuntimeBundle:
    return TableRuntimeBundle(
        output_semantics=output_semantics or OUTPUT_SEMANTICS,
        table_state=table_state or bootstrap_table_state(),
    )
