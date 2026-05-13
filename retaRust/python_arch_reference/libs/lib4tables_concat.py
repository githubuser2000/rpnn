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
from reta_architecture import generated_columns as generated_column_morphisms
from reta_architecture import meta_columns as meta_column_morphisms
from reta_architecture import concat_csv as concat_csv_morphisms

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
