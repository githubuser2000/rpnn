from __future__ import annotations

"""Architecture-owned compatibility adapters for historical table classes.

The old runtime exposed two stateful helper classes from legacy modules:
``lib4tables_prepare.Prepare`` and ``lib4tables_concat.Concat``.  Their deep
logic already lives in architecture modules, so this file keeps only the thin
adapter classes inside the architecture package itself.  External legacy
facades may continue to exist, but architecture code no longer imports them.
"""

from collections import OrderedDict
from typing import Iterable, Optional, Union

try:
    from orderedset import OrderedSet
except (ModuleNotFoundError, ImportError):
    OrderedSet = set

from .console_io import DefaultOrderedDict
from .table_preparation import (
    cell_work as architecture_cell_work,
    prepare_output_table as architecture_prepare_output_table,
    prepare_row_cells as architecture_prepare_row_cells,
    select_display_lines as architecture_select_display_lines,
    tag_output_column as architecture_tag_output_column,
)
from .row_filtering import (
    delete_doubles_in_sets as architecture_delete_doubles_in_sets,
    filter_original_lines as architecture_filter_original_lines,
    from_until as architecture_from_until,
    moonsun as architecture_moonsun,
    parameters_cmd_with_some_bereich as architecture_parameters_cmd_with_some_bereich,
    set_zaehlungen as architecture_set_zaehlungen,
    zeile_which_zaehlung as architecture_zeile_which_zaehlung,
)
from .table_wrapping import (
    Wraptype,
    alxwrap as architecture_alxwrap,
    bootstrap_table_wrapping,
    chunks as architecture_chunks,
    refresh_textwrap_runtime as architecture_refresh_textwrap_runtime,
    set_shell_rows_amount as architecture_set_shell_rows_amount,
    set_wrapping_type as architecture_set_wrapping_type,
    split_more_if_not_small as architecture_split_more_if_not_small,
    width_for_row as architecture_width_for_row,
    wrap_cell_text as architecture_wrap_cell_text,
)
from . import generated_columns as generated_column_morphisms
from . import meta_columns as meta_column_morphisms
from . import concat_csv as concat_csv_morphisms


_table_wrapping = bootstrap_table_wrapping(force_refresh=True)
shellRowsAmount = _table_wrapping.runtime.shell_rows_amount
h_de = _table_wrapping.runtime.h_de
dic = _table_wrapping.runtime.dic
fill = _table_wrapping.runtime.fill
wrappingType: Wraptype = _table_wrapping.runtime.wrapping_type


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
        self.zaehlungen = [0, {}, {}, {}, {}]
        self.religionNumbers = 0
        self.gezaehlt = False
        self.ifZeilenSetted = False

    def setZaehlungen(self, num: int):
        return architecture_set_zaehlungen(self, num)

    @property
    def breitenn(self):
        return self.breiten

    @breitenn.setter
    def breitenn(self, value: bool):
        self.breiten = value

    @property
    def nummeriere(self):
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

    def deleteDoublesInSets(self, set1: set, set2: set) -> Iterable[Union[set, set]]:
        return architecture_delete_doubles_in_sets(self, set1, set2)

    def fromUntil(self, a) -> tuple:
        return architecture_from_until(self, a)

    def zeileWhichZaehlung(self, zeile: int) -> int:
        return architecture_zeile_which_zaehlung(self, zeile)

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

    def concatLovePolygon(self, relitable: list, rowsAsNumbers: set) -> tuple:
        return generated_column_morphisms.concat_love_polygon(self, relitable, rowsAsNumbers)

    def gleichheitFreiheitVergleich(self, zahl: int) -> str:
        return generated_column_morphisms.gleichheit_freiheit_vergleich(zahl)

    def geistEmotionEnergieMaterieTopologie(self, zahl: int) -> str:
        return generated_column_morphisms.geist_emotion_energie_materie_topologie(zahl)

    def concatGleichheitFreiheitDominieren(
        self, relitable: list, rowsAsNumbers: set
    ) -> tuple:
        return generated_column_morphisms.concat_gleichheit_freiheit_dominieren(self, relitable, rowsAsNumbers)

    def concatGeistEmotionEnergieMaterieTopologie(
        self, relitable: list, rowsAsNumbers: set
    ) -> tuple:
        return generated_column_morphisms.concat_geist_emotion_energie_materie_topologie(self, relitable, rowsAsNumbers)

    def concatPrimCreativityType(self, relitable: list, rowsAsNumbers: set) -> tuple:
        return generated_column_morphisms.concat_prim_creativity_type(self, relitable, rowsAsNumbers)

    def concatMondExponzierenLogarithmusTyp(
        self, relitable: list, rowsAsNumbers: set
    ) -> tuple:
        return generated_column_morphisms.concat_mond_exponzieren_logarithmus_typ(self, relitable, rowsAsNumbers)

    def concatVervielfacheZeile(self, relitable: list, rowsAsNumbers: set) -> tuple:
        return generated_column_morphisms.concat_vervielfache_zeile(self, relitable, rowsAsNumbers)

    def concatModallogik(
        self, relitable: list, conceptsRowsSetOfTuple: set, rowsAsNumbers: set
    ) -> tuple:
        return generated_column_morphisms.concat_modallogik(
            self, relitable, conceptsRowsSetOfTuple, rowsAsNumbers
        )

    def convertSetOfPaarenToDictOfNumToPaareDiv(
        self, paareSet: OrderedSet, gleichf=False
    ) -> DefaultOrderedDict:
        return concat_csv_morphisms.convertSetOfPaarenToDictOfNumToPaareDiv(self, paareSet, gleichf)

    def convertSetOfPaarenToDictOfNumToPaareMul(
        self, paareSet: set, gleichf=False
    ) -> DefaultOrderedDict:
        return concat_csv_morphisms.convertSetOfPaarenToDictOfNumToPaareMul(self, paareSet, gleichf)

    def convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(
        self, fracs: set, fracs2: set, gleichf=False
    ) -> DefaultOrderedDict:
        return concat_csv_morphisms.convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(self, fracs, fracs2, gleichf)

    def combineDicts(
        self, a: DefaultOrderedDict, b: DefaultOrderedDict
    ) -> DefaultOrderedDict:
        return concat_csv_morphisms.combineDicts(self, a, b)

    def concat1PrimzahlkreuzProContra(
        self, relitable: list, rowsAsNumbers: set, generatedBefehle: set, ParametersMain
    ) -> tuple:
        return generated_column_morphisms.concat_primzahlkreuz_pro_contra(
            self, relitable, rowsAsNumbers, generatedBefehle, ParametersMain
        )

    def concat1RowPrimUniverse2(
        self,
        relitable: list,
        rowsAsNumbers: set,
        generatedBefehle: set,
        htmlTagParaClassWoerter: list,
    ) -> tuple:
        return generated_column_morphisms.concat_prim_universe_row(
            self, relitable, rowsAsNumbers, generatedBefehle, htmlTagParaClassWoerter
        )

    def spalteMetaKontretTheorieAbstrakt_etc_1(self, *args, **kwargs):
        return meta_column_morphisms.spalteMetaKontretTheorieAbstrakt_etc_1(self, *args, **kwargs)

    def spalteMetaKonkretAbstrakt_isGanzZahlig(self, *args, **kwargs):
        return meta_column_morphisms.spalteMetaKonkretAbstrakt_isGanzZahlig(self, *args, **kwargs)

    def spalteMetaKontretTheorieAbstrakt_etc(self, *args, **kwargs):
        return meta_column_morphisms.spalteMetaKontretTheorieAbstrakt_etc(self, *args, **kwargs)

    def spalteMetaKonkretTheorieAbstrakt_SetHtmlParameters(self, *args, **kwargs):
        return meta_column_morphisms.spalteMetaKonkretTheorieAbstrakt_SetHtmlParameters(self, *args, **kwargs)

    def spalteMetaKonkretTheorieAbstrakt_mainPart(self, *args, **kwargs):
        return meta_column_morphisms.spalteMetaKonkretTheorieAbstrakt_mainPart(self, *args, **kwargs)

    def spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta(self, *args, **kwargs):
        return meta_column_morphisms.spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta(self, *args, **kwargs)

    def spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText(self, *args, **kwargs):
        return meta_column_morphisms.spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText(self, *args, **kwargs)

    def getAllBrueche(self, *args, **kwargs):
        return meta_column_morphisms.getAllBrueche(self, *args, **kwargs)

    def readOneCSVAndReturn(self, *args, **kwargs):
        return meta_column_morphisms.readOneCSVAndReturn(self, *args, **kwargs)

    def findAllBruecheAndTheirCombinations(self, *args, **kwargs):
        return meta_column_morphisms.findAllBruecheAndTheirCombinations(self, *args, **kwargs)

    def spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(self, *args, **kwargs):
        return meta_column_morphisms.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(self, *args, **kwargs)

    def spalteMetaKonkretAbstrakt_UeberschriftenUndTags(self, *args, **kwargs):
        return meta_column_morphisms.spalteMetaKonkretAbstrakt_UeberschriftenUndTags(self, *args, **kwargs)

    def spalteFuerGegenInnenAussenSeitlichPrim(self, *args, **kwargs):
        return meta_column_morphisms.spalteFuerGegenInnenAussenSeitlichPrim(self, *args, **kwargs)

    def readConcatCsv_tabelleDazuColchange(
        self,
        zeilenNr: int,
        tabelleDazuCol: list,
        concatTable: int,
        ifTransponiert=False,
    ) -> list:
        return concat_csv_morphisms.readConcatCsv_tabelleDazuColchange(self, zeilenNr, tabelleDazuCol, concatTable, ifTransponiert)

    def readConcatCsv(
        self,
        relitable: list,
        rowsAsNumbers: set,
        concatTableSelection: set,
        concatTable: int = 1,
    ) -> tuple:
        return concat_csv_morphisms.readConcatCsv(self, relitable, rowsAsNumbers, concatTableSelection, concatTable)

    def readConcatCSV_choseCsvFile(self, concatTable):
        return concat_csv_morphisms.readConcatCSV_choseCsvFile(self, concatTable)

    def readConcatCsv_ChangeTableToAddToTable(self, concatTable, tableToAdd, transpose):
        return concat_csv_morphisms.readConcatCsv_ChangeTableToAddToTable(self, concatTable, tableToAdd, transpose)

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
        return concat_csv_morphisms.readConcatCsv_LoopBody(
            self, concatCSVspalten, concatTable, concatTableSelection, dazu, heading, rowsAsNumbers, u
        )

    def readConcatCsv_SetHtmlParamaters(self, concatTable, heading, u):
        return concat_csv_morphisms.readConcatCsv_SetHtmlParamaters(self, concatTable, heading, u)
