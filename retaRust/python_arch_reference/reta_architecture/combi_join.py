from __future__ import annotations

"""Kombi join/relation morphisms for reta tables.

This module owns the old ``Tables.Combi`` behavior.  ``libs.tableHandling``
keeps a compatibility alias so existing callers can still use ``Tables.Combi``
or ``tables.getCombis``.
"""

import csv
import os
from collections import OrderedDict
from dataclasses import dataclass
from copy import deepcopy

try:
    from orderedset import OrderedSet
except (ModuleNotFoundError, ImportError):
    OrderedSet = set

i18n = None
htmlSyntax = None
bbCodeSyntax = None
shellRowsAmount = 0


def _ensure_runtime_dependencies() -> None:
    global i18n, htmlSyntax, bbCodeSyntax, shellRowsAmount
    if i18n is not None:
        return
    from .output_syntax import bbCodeSyntax as bb_code_syntax, htmlSyntax as html_syntax
    from .runtime_compat import i18n as runtime_i18n
    from .table_wrapping import get_shell_rows_amount

    i18n = runtime_i18n
    htmlSyntax = html_syntax
    bbCodeSyntax = bb_code_syntax
    shellRowsAmount = get_shell_rows_amount() or 0


class KombiJoin:
    def __init__(self, tables):
        _ensure_runtime_dependencies()
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
        parallel_result = None
        try:
            from .parallel_execution import prepare_kombi_join_tables_in_processes

            parallel_result = prepare_kombi_join_tables_in_processes(
                ChosenKombiLines,
                newTable_kombi_1,
                config=getattr(self.tables, "parallel_config", None),
            )
        except Exception as exc:  # pragma: no cover - serial fallback protects CLI parity.
            parallel_result = None
            try:
                self.parallel_last_result = {
                    "mode": "serial_fallback",
                    "operation": "prepare_kombi_join_tables",
                    "error": f"{type(exc).__name__}: {exc}",
                }
            except Exception:
                pass
        if parallel_result is not None:
            try:
                self.parallel_last_result = parallel_result.snapshot()
            except Exception:
                pass
            return parallel_result.values

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
                raw_rows = list(enumerate(csv.reader(csv_file, delimiter=";")))

            self.kombiTable: list = []
            self.kombiTable_Kombis: list = []
            parallel_result = None
            if raw_rows:
                try:
                    from .parallel_execution import decode_kombi_rows_in_processes

                    parallel_result = decode_kombi_rows_in_processes(
                        raw_rows,
                        config=getattr(self.tables, "parallel_config", None),
                    )
                except Exception as exc:  # pragma: no cover - deterministic serial fallback.
                    parallel_result = None
                    try:
                        self.parallel_csv_decode_result = {
                            "mode": "serial_fallback",
                            "operation": "decode_kombi_rows",
                            "error": f"{type(exc).__name__}: {exc}",
                        }
                    except Exception:
                        pass

            if parallel_result is not None:
                for z, col, kombi_numbers in parallel_result.values:
                    self.kombiTable += [col]
                    if len(col) > 0 and z > 0:
                        self.kombiTable_Kombis += [kombi_numbers]
                try:
                    self.parallel_csv_decode_result = parallel_result.snapshot()
                except Exception:
                    pass
            else:
                for z, col in raw_rows:
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
                if raw_rows and not hasattr(self, "parallel_csv_decode_result"):
                    try:
                        self.parallel_csv_decode_result = {
                            "mode": "serial",
                            "operation": "decode_kombi_rows",
                            "rows": len(raw_rows),
                        }
                    except Exception:
                        pass
            self.relitable, animalsProfessionsCol = self.tables.fillBoth(
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





def _kombi_csv_sources() -> list:
    try:
        import i18n.words_runtime as words_runtime

        return [
            getattr(words_runtime.csvFileNames, "kombi13", None),
            getattr(words_runtime.csvFileNames, "kombi15", None),
        ]
    except Exception:
        return []


@dataclass(frozen=True)
class KombiJoinBundle:
    """Inspectable factory for the Kombi join/relation layer."""

    implementation: type = KombiJoin

    def create(self, tables) -> KombiJoin:
        return self.implementation(tables)

    def snapshot(self) -> dict:
        return {
            "class": "KombiJoinBundle",
            "implementation": self.implementation.__name__,
            "morphisms": [
                "prepareTableJoin",
                "removeOneNumber",
                "tableJoin",
                "prepare_kombi",
                "readKombiCsv",
                "kombiNumbersCorrectTestAndSet",
            ],
            "csv_sources": _kombi_csv_sources(),
            "role": "Kombi CSV presheaf sections -> joined table section",
        }


def bootstrap_combi_join() -> KombiJoinBundle:
    return KombiJoinBundle()
