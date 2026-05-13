from __future__ import annotations

"""CSV and fractional gluing morphisms for reta table concatenation.

This module owns the remaining CSV-presheaf and Bruch/fraction gluing logic
that used to live inside ``libs/lib4tables_concat.py``.  Legacy ``Concat``
methods remain as wrappers, but table CSV attachment and fraction-indexed
selection are now explicit architecture morphisms.
"""

import csv
import os
from dataclasses import dataclass
from fractions import Fraction
from typing import Tuple

DefaultOrderedDict = None
OrderedSet = None
csvNames = None
i18n = None
nPmEnum = None


def _ensure_runtime_dependencies() -> None:
    global DefaultOrderedDict, OrderedSet, csvNames, i18n, nPmEnum
    if i18n is not None:
        return
    try:
        from orderedset import OrderedSet as ordered_set
    except (ModuleNotFoundError, ImportError):
        ordered_set = set
    from .runtime_compat import (
        DefaultOrderedDict as default_ordered_dict,
        i18n as runtime_i18n,
        nPmEnum as runtime_nPmEnum,
    )

    DefaultOrderedDict = default_ordered_dict
    OrderedSet = ordered_set
    csvNames = runtime_i18n.csvFileNames
    i18n = runtime_i18n.concat
    nPmEnum = runtime_nPmEnum


@dataclass(frozen=True)
class ConcatCsvSpec:
    method_name: str
    description: str
    tags: Tuple[str, ...] = ()

    def snapshot(self) -> dict:
        return {
            "method_name": self.method_name,
            "description": self.description,
            "tags": list(self.tags),
        }


@dataclass(frozen=True)
class ConcatCsvBundle:
    specs: Tuple[ConcatCsvSpec, ...]

    def snapshot(self) -> dict:
        return {
            "class": "ConcatCsvBundle",
            "count": len(self.specs),
            "morphisms": [spec.snapshot() for spec in self.specs],
            "csv_sources": ["prim", "bruch13", "bruch15", "bruch7", "bruchStrukGroesse"],
            "fraction_helpers": [
                "convertSetOfPaarenToDictOfNumToPaareDiv",
                "convertSetOfPaarenToDictOfNumToPaareMul",
                "convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction",
                "combineDicts",
            ],
        }

    def read_concat_csv(self, concat, relitable: list, rows_as_numbers: set, concat_table_selection: set, concat_table: int = 1):
        return readConcatCsv(concat, relitable, rows_as_numbers, concat_table_selection, concat_table)

    def choose_csv_file(self, concat, concat_table: int):
        return readConcatCSV_choseCsvFile(concat, concat_table)

    def combine_dicts(self, concat, a, b):
        return combineDicts(concat, a, b)


def bootstrap_concat_csv() -> ConcatCsvBundle:
    return ConcatCsvBundle(
        specs=(
            ConcatCsvSpec(
                "readConcatCsv",
                "Glues an external CSV presheaf section into the current global table section.",
                ("csv", "presheaf", "gluing"),
            ),
            ConcatCsvSpec(
                "readConcatCsv_tabelleDazuColchange",
                "Transforms fraction-indexed CSV columns into meta/concrete cell content.",
                ("fraction", "meta", "morphism"),
            ),
            ConcatCsvSpec(
                "readConcatCsv_SetHtmlParamaters",
                "Registers generated CSV columns in the HTML/tag parameter sheaf.",
                ("html", "tags", "generated-column"),
            ),
            ConcatCsvSpec(
                "convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction",
                "Builds number-indexed fraction-pair sections used by generated prime-universe columns.",
                ("fraction", "brueche", "relation"),
            ),
        )
    )

def convertSetOfPaarenToDictOfNumToPaareDiv(concat, paareSet: OrderedSet, gleichf=False) -> DefaultOrderedDict:
    """Macht aus einem Set aus Paaren eins von verschiedenen möglichen dicts mit key int und value liste aus paaren"""
    _ensure_runtime_dependencies()
    result: DefaultOrderedDict = DefaultOrderedDict(OrderedSet)
    paareSet: tuple = tuple(paareSet)
    for paar in paareSet:
        paar = tuple(paar)
        div = paar[0] / paar[1] if not gleichf else paar[1] / paar[0]
        div = round(div * 1000) / 1000
        assert div == round(div)
        result[int(div)] |= {paar}
    return result

def convertSetOfPaarenToDictOfNumToPaareMul(concat, paareSet: set, gleichf=False) -> DefaultOrderedDict:
    """Macht aus einem Set aus Paaren eins von verschiedenen möglichen dicts mit key int und value liste aus paaren"""
    _ensure_runtime_dependencies()
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

def convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(concat, fracs: set, fracs2: set, gleichf=False) -> DefaultOrderedDict:
    _ensure_runtime_dependencies()
    result: DefaultOrderedDict = DefaultOrderedDict(OrderedSet)
    if not gleichf:
        for frac in tuple(fracs):
            for zusatzMul in range(1, concat.tables.hoechsteZeile[1024] + 1):
                paar = (frac, Fraction(frac.denominator) * zusatzMul)
                mul = paar[0] * paar[1]
                mulr = round(mul)
                mul = round(mul * 1000) / 1000
                assert mulr == mul
                if mul > concat.tables.hoechsteZeile[1024]:
                    break
                result[int(mul)] |= {paar}
        for frac in tuple(fracs):
            for zusatzMul in range(concat.tables.hoechsteZeile[1024], 0, -1):
                faktor = Fraction(frac.denominator) / zusatzMul
                if faktor in fracs2 or faktor.numerator == 1:
                    paar = (frac, faktor)
                    mul = paar[0] * paar[1]
                    mulr = round(mul)
                    if mul > concat.tables.hoechsteZeile[1024]:
                        break
                    if mulr == mul:
                        result[int(mul)] |= {paar}
    else:
        for frac in tuple(fracs):
            for zusatzDiv in range(1, concat.tables.hoechsteZeile[1024] + 1):
                paar = (frac, 1 / Fraction(frac.numerator) / zusatzDiv)
                div = 1 / (paar[1] * paar[0])
                divr = round(div)
                div = round(div * 1000) / 1000
                assert divr == div
                if div > concat.tables.hoechsteZeile[1024]:
                    break
                result[int(divr)] |= {paar}
        for frac in tuple(fracs):
            for zusatzDiv in range(1, concat.tables.hoechsteZeile[1024] + 1):
                faktor = 1 / frac / zusatzDiv
                if faktor in fracs2 or faktor.numerator == 1:
                    paar = (frac, faktor)
                    mul = 1 / (paar[1] * paar[0])
                    mulr = round(mul)
                    mul = round(mul * 1000) / 1000
                    assert mulr == mul
                    if 1 / mul > concat.tables.hoechsteZeile[1024]:
                        break
                    result[int(mulr)] |= {paar}
    return result

def combineDicts(concat, a: DefaultOrderedDict, b: DefaultOrderedDict) -> DefaultOrderedDict:
    _ensure_runtime_dependencies()
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

def readConcatCsv_tabelleDazuColchange(concat, zeilenNr: int, tabelleDazuCol: list, concatTable: int, ifTransponiert=False) -> list:
    _ensure_runtime_dependencies()
    tabelleDazuColNeu: list = []
    for i, cell in enumerate(tabelleDazuCol, 1):
        gebrRatZahl = Fraction(zeilenNr, i) if not ifTransponiert else Fraction(i, zeilenNr)
        cellNeu = concat.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(gebrRatZahl, concat.struktAndInversSpalten, concat.gebrUnivTable4metaKonkret, concatTable not in nPmEnum.uni())
        tabelleDazuColNeu += [cellNeu if cellNeu is not None else '']
    return tabelleDazuColNeu

def readConcatCsv(concat, relitable: list, rowsAsNumbers: set, concatTableSelection: set, concatTable: int=1) -> tuple:
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
    _ensure_runtime_dependencies()
    global folder
    spaltenDict: dict = {1: None, nPmEnum.uniN: (5, 131), nPmEnum.uni1pN: (5, 131), nPmEnum.galN: (10, 42), nPmEnum.gal1pN: (10, 42), nPmEnum.emoN: (243, 284), nPmEnum.emo1pN: (243, 284), nPmEnum.groeN: (4, 197), nPmEnum.groe1pN: (4, 197)}
    if concatTable != 1:
        concat.struktAndInversSpalten: tuple = spaltenDict[concatTable]
        concat.gebrUnivTable4metaKonkret = concat.readOneCSVAndReturn(concatTable)

    def transpose(matrix):
        t = []
        x: int
        y: int
        for x in range(len(matrix[0])):
            t += [[]]
            for y in range(len(matrix)):
                t[x] += [matrix[y][x]]
        return t
    concat.relitable = relitable
    concatCSVspalten: set = OrderedSet()
    if len(concatTableSelection) > 0 and concatTable in range(1, 10):
        tableToAdd = concat.readOneCSVAndReturn(concatTable)
        tableToAdd = concat.readConcatCsv_ChangeTableToAddToTable(concatTable, tableToAdd, transpose)
        if concatTable == 1:
            tableToAdd2 = [[i18n.primVielGen['Primzahlvielfache, nicht generiert']]]
            for t, zeile in enumerate(tableToAdd[1:], 1):
                zeileNeu = []
                for zelle in zeile:
                    if len(zelle.strip()) > 3:
                        zeileNeu += [('<li>' if concat.tables.htmlOutputYes else '' if not concat.tables.bbcodeOutputYes else '[*]') + zelle + ('</li>' if concat.tables.htmlOutputYes else '')]
                zeileNeu = [('' if concat.tables.htmlOutputYes or concat.tables.bbcodeOutputYes else ' | ').join((['<ul>'] if concat.tables.htmlOutputYes else [''] if not concat.tables.bbcodeOutputYes else ['[list]']) + zeileNeu + (['</ul>'] if concat.tables.htmlOutputYes else [''] if not concat.tables.bbcodeOutputYes else ['[/list]']))]
                tableToAdd2 += [zeileNeu]
            tableToAdd = tableToAdd2
        concat.relitable, tableToAdd = concat.tables.fillBoth(concat.relitable, tableToAdd)
        lastlen = 0
        maxlen = 0
        for i, (tabelleDazuCol, relicol) in enumerate(zip(tableToAdd, concat.relitable)):
            lastlen = len(tabelleDazuCol)
            if lastlen > maxlen:
                maxlen = lastlen
            dazu = list(tabelleDazuCol) + [''] * (maxlen - len(tabelleDazuCol))
            if i != 0 and concatTable in range(2, 10):
                dazu = concat.readConcatCsv_tabelleDazuColchange(i, dazu, concatTable, concatTable in nPmEnum.einsPn() and concatTable not in nPmEnum.n())
            concat.relitable[i] += dazu
            if i == 0:
                for u, heading in enumerate(dazu):
                    concat.readConcatCsv_LoopBody(concatCSVspalten, concatTable, concatTableSelection, dazu, heading, rowsAsNumbers, u)
    return (concat.relitable, rowsAsNumbers, concatCSVspalten)

def readConcatCSV_choseCsvFile(concat, concatTable):
    _ensure_runtime_dependencies()
    place = os.path.join(os.getcwd(), os.path.dirname(__file__), '..', 'csv', os.path.basename(csvNames.prim if concatTable == 1 else csvNames.bruch13 if concatTable in nPmEnum.gal() else csvNames.bruch15 if concatTable in nPmEnum.uni() else csvNames.bruch7 if concatTable in nPmEnum.emo() else csvNames.bruchStrukGroesse if concatTable in nPmEnum.groe() else None))
    return place

def readConcatCsv_ChangeTableToAddToTable(concat, concatTable, tableToAdd, transpose):
    _ensure_runtime_dependencies()
    if concatTable in nPmEnum.einsPn():
        tableToAdd = transpose(tableToAdd)
    if concatTable in range(2, 10):
        tableToAdd = [[('n/' + str(n + 1) if concatTable in nPmEnum.n() else str(n + 1) + '/n' if concatTable in nPmEnum.einsPn() else i18n.GalOrUniOrFehler['Fehler']) + (' ' + i18n.GalOrUniOrFehler['Universum'] if concatTable in nPmEnum.uni() else ' ' + i18n.GalOrUniOrFehler['Galaxie'] if concatTable in nPmEnum.gal() else ' ' + i18n.GalOrUniOrFehler['Emotion'] if concatTable in nPmEnum.emo() else ' ' + i18n.GalOrUniOrFehler['Strukturgroesse'] if concatTable in nPmEnum.groe() else i18n.GalOrUniOrFehler['Fehler']) for n in range(len(tableToAdd[0]))]] + tableToAdd
    return tableToAdd

def readConcatCsv_LoopBody(concat, concatCSVspalten, concatTable, concatTableSelection, dazu, heading, rowsAsNumbers, u):
    _ensure_runtime_dependencies()
    if u + 2 in concatTableSelection and concatTable in range(2, 10) or concatTable == 1:
        if concatTable not in range(2, 10) or u + 1 != len(dazu):
            delta = 1 if concatTable in range(2, 10) else 0
            selectedSpalten = u + len(concat.relitable[0]) - len(dazu) + delta
            rowsAsNumbers.add(selectedSpalten)
            concatCSVspalten.add(selectedSpalten)
            if len(concat.tables.generatedSpaltenParameter) + concat.tables.SpaltenVanillaAmount in concat.tables.generatedSpaltenParameter:
                raise ValueError
            concat.readConcatCsv_SetHtmlParamaters(concatTable, heading, u)

def readConcatCsv_SetHtmlParamaters(concat, concatTable, heading, u):
    _ensure_runtime_dependencies()
    if concatTable in range(2, 10):
        rangeToDataDict = {2: 6, 3: 6, 4: 5, 5: 5, 6: 9, 7: 9, 8: 10, 9: 10}
        concat.tables.generatedSpaltenParameter[len(concat.tables.generatedSpaltenParameter) + concat.tables.SpaltenVanillaAmount] = ([concat.tables.dataDict[rangeToDataDict[concatTable]][u + 2][0][0]],)
    if concatTable == 1:
        intoHtmlPara = ([(i18n.multipl['Multiplikationen'], i18n.notGen['Nicht_generiert'])],)
        concat.tables.generatedSpaltenParameter[len(concat.tables.generatedSpaltenParameter) + concat.tables.SpaltenVanillaAmount] = intoHtmlPara
