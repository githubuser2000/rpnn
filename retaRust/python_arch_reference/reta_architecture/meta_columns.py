from __future__ import annotations

"""Meta-column morphisms for reta table concatenation.

The functions in this module own the generated meta/concrete/theory/abstract
column logic that used to live directly in ``libs/lib4tables_concat.py``.  The
legacy ``Concat`` methods are retained as compatibility wrappers, but this
module is now the architectural owner for this family of morphisms.
"""

import csv
from collections import OrderedDict
from copy import deepcopy
from dataclasses import dataclass
from fractions import Fraction
from typing import Tuple

try:
    from orderedset import OrderedSet
except (ModuleNotFoundError, ImportError):
    OrderedSet = set

ST = None
i18n = None
nPmEnum = None
couldBePrimeNumberPrimzahlkreuz = None
primCreativity = None
primFak = None
primRepeat = None


def _ensure_runtime_dependencies() -> None:
    global ST, i18n, nPmEnum, couldBePrimeNumberPrimzahlkreuz
    global primCreativity, primFak, primRepeat
    if i18n is not None:
        return
    from .runtime_compat import i18n as runtime_i18n, nPmEnum as runtime_nPmEnum
    from .number_theory import (
        couldBePrimeNumberPrimzahlkreuz as prime_cross_predicate,
        primCreativity as prim_creativity,
        primFak as prim_fak,
        primRepeat as prim_repeat,
    )
    from .tag_schema import ST as st_enum

    ST = st_enum
    i18n = runtime_i18n.concat
    nPmEnum = runtime_nPmEnum
    couldBePrimeNumberPrimzahlkreuz = prime_cross_predicate
    primCreativity = prim_creativity
    primFak = prim_fak
    primRepeat = prim_repeat


@dataclass(frozen=True)
class MetaColumnSpec:
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
class MetaColumnsBundle:
    specs: Tuple[MetaColumnSpec, ...]

    def snapshot(self) -> dict:
        return {
            "class": "MetaColumnsBundle",
            "count": len(self.specs),
            "morphisms": [spec.snapshot() for spec in self.specs],
        }

    def spalte_meta_kontret_theorie_abstrakt_etc_1(self, concat, relitable: list, rows_as_numbers: set, geordnete_paare: set):
        return spalteMetaKontretTheorieAbstrakt_etc_1(concat, relitable, rows_as_numbers, geordnete_paare)

    def read_one_csv_and_return(self, concat, wahl) -> list:
        return readOneCSVAndReturn(concat, wahl)

    def find_all_brueche_and_their_combinations(self, concat):
        return findAllBruecheAndTheirCombinations(concat)


def bootstrap_meta_columns() -> MetaColumnsBundle:
    return MetaColumnsBundle(
        specs=(
            MetaColumnSpec(
                "spalteMetaKontretTheorieAbstrakt_etc_1",
                "Entry point for generated meta/concrete/theory/abstract columns.",
                ("meta", "theorie", "abstrakt", "konkret"),
            ),
            MetaColumnSpec(
                "spalteFuerGegenInnenAussenSeitlichPrim",
                "Classifies prime-cross generated columns as pro/contra/inside/outside/sideways.",
                ("primzahlkreuz", "meta"),
            ),
            MetaColumnSpec(
                "readOneCSVAndReturn",
                "CSV section cache used by meta and fractional generated-column morphisms.",
                ("prägarbe", "csv"),
            ),
        )
    )


def spalteMetaKontretTheorieAbstrakt_etc_1(
    self, relitable: list, rowsAsNumbers: set, geordnetePaare: set
):
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
    _ensure_runtime_dependencies()
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
