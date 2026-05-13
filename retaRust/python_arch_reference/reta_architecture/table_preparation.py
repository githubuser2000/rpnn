# -*- coding: utf-8 -*-
"""Tabellen-Ausgabevorbereitung als explizite Architekturschicht.

Die alte Klasse ``Prepare`` in ``libs/lib4tables_prepare.py`` bleibt als
kompatible Laufzeitfassade erhalten. Die eigentliche lokale-zu-globale
Ausgabevorbereitung lebt hier: Zeilenfilter, Spaltenauswahl, Tag-Gluing und
Zellenumbruch werden als benannte Morphismen sichtbar.
"""
from __future__ import annotations

from copy import deepcopy
from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class MainTablePreparationResult:
    finally_display_lines: object
    new_table: list
    numlen: int
    rows_range: object
    old2new_table: tuple

    def snapshot(self) -> dict:
        return {
            "class": "MainTablePreparationResult",
            "finally_display_lines_len": len(self.finally_display_lines),
            "new_table_len": len(self.new_table),
            "numlen": self.numlen,
            "rows_range_len": len(self.rows_range),
        }


@dataclass(frozen=True)
class KombiTablePreparationResult:
    finally_display_lines: object
    new_table: list
    line_len: int
    animals_professions_table: list
    old2new_table_animals_professions: tuple

    def snapshot(self) -> dict:
        return {
            "class": "KombiTablePreparationResult",
            "finally_display_lines_len": len(self.finally_display_lines),
            "new_table_len": len(self.new_table),
            "line_len": self.line_len,
            "animals_professions_table_len": len(self.animals_professions_table),
        }


def _tag_modules():
    """Importiere Legacy-Tag-Module lazy, um Importzyklen klein zu halten."""
    from . import tag_schema

    return tag_schema, tag_schema.ST


def prepare_output_table(
    prepare,
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
    """Legacy-kompatible Orchestrierung von ``Prepare.prepare4out``.

    Die Header-/Tag-Zeile bleibt seriell, weil sie die globalen
    ``generatedSpaltenParameter`` füllt. Die danach unabhängigen Datenzeilen
    können bei PyPy3 oder bei expliziter Aktivierung chunkweise in
    Multiprocessing-Workern vorbereitet und anschließend deterministisch
    zusammengeklebt werden.
    """
    (
        finallyDisplayLines,
        headingsAmount,
        newerTable,
        numlen,
        rowsRange,
    ) = select_display_lines(prepare, contentTable, paramLines, paramLinesNot)

    prepare.headingsAmount = headingsAmount
    old2Rows: tuple = ({}, {})
    reliNumbersBool = False if prepare.religionNumbers != [] else True

    selected_non_header_rows: list[tuple[int, list]] = []
    for u, line in enumerate(contentTable):
        if u in finallyDisplayLines or combiRows != 0:
            if u == 0:
                new2Lines = prepare_row_cells(
                    prepare,
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
                    kombiCSVNumber=kombiCSVNumber,
                )
                if new2Lines != []:
                    newerTable += [new2Lines]
            else:
                selected_non_header_rows.append((u, line))

    parallel_result = None
    if selected_non_header_rows:
        try:
            from .parallel_execution import (
                ParallelExecutionConfig,
                prepare_rows_in_processes,
            )

            config = getattr(prepare, "parallel_config", None) or ParallelExecutionConfig.from_environment()
            parallel_result = prepare_rows_in_processes(
                prepare,
                selected_non_header_rows,
                rows_as_numbers=rowsAsNumbers,
                combi_rows=combiRows,
                headings_amount=headingsAmount,
                religion_numbers_bool=reliNumbersBool,
                reli_table_len_until_now=reliTableLenUntilNow,
                kombi_csv_number=kombiCSVNumber,
                config=config,
            )
        except Exception as exc:  # pragma: no cover - Fallback protects CLI parity.
            parallel_result = None
            try:
                prepare.parallel_last_result = {
                    "mode": "serial_fallback",
                    "error": f"{type(exc).__name__}: {exc}",
                }
            except Exception:
                pass

    if parallel_result is not None:
        newerTable += parallel_result.rows
        if reliNumbersBool and isinstance(prepare.religionNumbers, list):
            prepare.religionNumbers += parallel_result.religion_numbers
        try:
            prepare.parallel_last_result = parallel_result.snapshot()
        except Exception:
            pass
    else:
        for u, line in selected_non_header_rows:
            new2Lines = prepare_row_cells(
                prepare,
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
                kombiCSVNumber=kombiCSVNumber,
            )

            if new2Lines != []:
                newerTable += [new2Lines]
        try:
            if not hasattr(prepare, "parallel_last_result"):
                prepare.parallel_last_result = {
                    "mode": "serial",
                    "rows": len(selected_non_header_rows),
                }
        except Exception:
            pass

    return finallyDisplayLines, newerTable, numlen, rowsRange, old2Rows


def select_display_lines(prepare, contentTable, paramLines, paramLinesNot):
    """Bestimme die global sichtbaren Tabellenzeilen aus lokalen Zeilenbedingungen."""
    newerTable: list = []
    if len(contentTable) > 0:
        headingsAmount = len(contentTable[0])
        rowsRange = range(headingsAmount)
    else:
        headingsAmount = 0
        rowsRange = range(0)

    finallyDisplayLines: set = prepare.FilterOriginalLines(
        set(prepare.originalLinesRange), paramLines
    )
    if len(paramLinesNot) != 0:
        finallyDisplayLines2 = prepare.FilterOriginalLines(
            deepcopy(finallyDisplayLines), paramLinesNot
        )
        hasAnythingCanged = (
            set(prepare.originalLinesRange) - finallyDisplayLines2 - {0}
        )
        if len(hasAnythingCanged) > 0:
            finallyDisplayLines -= finallyDisplayLines2

    if len(finallyDisplayLines) == 0:
        if prepare.ifZeilenSetted:
            finallyDisplayLines = set()
        else:
            finallyDisplayLines = set(range(prepare.hoechsteZeile[1024] + 1))

    finallyDisplayLines.add(0)
    finallyDisplayLines3: list = list(finallyDisplayLines)
    finallyDisplayLines3.sort()
    finallyDisplayLines = set(finallyDisplayLines3)
    numlen = len(str(finallyDisplayLines3[-1]))

    return finallyDisplayLines, headingsAmount, newerTable, numlen, rowsRange


def prepare_row_cells(
    prepare,
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
    """Bereite eine Tabellenzeile vor und klebe Header-Tags an die Ausgabespalten."""
    if reliNumbersBool:
        prepare.religionNumbers += [int(u)]
    new2Lines: list = []
    rowToDisplay = 0
    h = 0
    for t, cell in enumerate(line):
        if t in rowsAsNumbers:
            if u == 0:
                tag_output_column(
                    prepare,
                    combiRows,
                    gebrSpalten,
                    primSpalten,
                    reliTableLenUntilNow,
                    rowToDisplay,
                    t,
                    kombiCSVNumber=kombiCSVNumber,
                )

            rowToDisplay += 1
            certaintextwidth = prepare.setWidth(rowToDisplay, combiRows)
            prepare.certaintextwidth = certaintextwidth

            into = cell_work(prepare, cell, certaintextwidth)
            if into != [""] or True:
                new2Lines += [into]
            if u == 0:
                old2Rows[0][t] = h
                old2Rows[1][h] = t
            h += 1
    return new2Lines


def tag_output_column(
    prepare,
    combiRows,
    gebrSpalten,
    primSpalten,
    reliTableLenUntilNow,
    rowToDisplay,
    t,
    kombiCSVNumber,
):
    """Klebe Legacy-Tabellentags an generierte Spalten und Kombi-Spalten."""
    lib4tables_Enum, ST = _tag_modules()
    if combiRows == 0:
        try:
            if rowToDisplay not in prepare.tables.generatedSpaltenParameter:
                prepare.tables.generatedSpaltenParameter[rowToDisplay] = prepare.tables.dataDict[0][t]
                prepare.tables.generatedSpaltenParameter_Tags[rowToDisplay] = lib4tables_Enum.tableTags2[t]
            elif primSpalten is not None and t in primSpalten:
                prepare.tables.generatedSpaltenParameter_Tags[rowToDisplay] = frozenset(
                    {ST.sternPolygon, ST.universum, ST.galaxie}
                )
            elif (gebrSpalten["Gal"] is not None and t in gebrSpalten["Gal"]) or (
                gebrSpalten["Gal2"] is not None and t in gebrSpalten["Gal2"]
            ):
                prepare.tables.generatedSpaltenParameter_Tags[rowToDisplay] = frozenset(
                    {ST.sternPolygon, ST.galaxie, ST.gleichfoermigesPolygon, ST.gebrRat}
                )
            elif (gebrSpalten["Uni"] is not None and t in gebrSpalten["Uni"]) or (
                gebrSpalten["Uni2"] is not None and t in gebrSpalten["Uni2"]
            ):
                prepare.tables.generatedSpaltenParameter_Tags[rowToDisplay] = frozenset(
                    {ST.sternPolygon, ST.universum, ST.gleichfoermigesPolygon, ST.gebrRat}
                )
            elif (gebrSpalten["Emo"] is not None and t in gebrSpalten["Emo"]) or (
                gebrSpalten["Emo2"] is not None and t in gebrSpalten["Emo2"]
            ):
                prepare.tables.generatedSpaltenParameter_Tags[rowToDisplay] = frozenset(
                    {ST.sternPolygon, ST.keinParaOdMetaP, ST.gleichfoermigesPolygon, ST.gebrRat}
                )
            elif (gebrSpalten["Groe"] is not None and t in gebrSpalten["Groe"]) or (
                gebrSpalten["Groe2"] is not None and t in gebrSpalten["Groe2"]
            ):
                prepare.tables.generatedSpaltenParameter_Tags[rowToDisplay] = frozenset(
                    {ST.sternPolygon, ST.gleichfoermigesPolygon, ST.gebrRat, ST.keinParaOdMetaP}
                )
        except KeyError:
            pass
    else:
        assert kombiCSVNumber in (0, 1)
        try:
            prepare.tables.generatedSpaltenParameter_Tags[reliTableLenUntilNow + rowToDisplay] = (
                lib4tables_Enum.tableTags2_kombiTable[t]
                if kombiCSVNumber == 0
                else lib4tables_Enum.tableTags2_kombiTable2[t]
                if kombiCSVNumber == 1
                else None
            )
        except KeyError:
            pass


def cell_work(prepare, cell: str, certaintextwidth: int) -> list:
    """Legacy-kompatibler Zellenumbruch als isolierter Tabellenmorphismus."""
    cell = cell.strip()
    isItNone = prepare.wrapping(cell, certaintextwidth)
    cell2: tuple = tuple()
    rest: str = cell
    if certaintextwidth == 0:
        return [cell]

    while isItNone not in [None, ()]:
        cell2 += isItNone
        isItNone = prepare.wrapping(cell2[-1], certaintextwidth)
        rest = cell2[-1]
        cell2 = cell2[:-1]
        if len(rest) > certaintextwidth and isItNone is None:
            cell2 += (rest[0:certaintextwidth],)
            isItNone = (rest[certaintextwidth:],)
    else:
        cell2 += (rest[0:certaintextwidth],)
        newLines: list = []
        for _k, cellInCells in enumerate(cell2):
            newLines += [cellInCells]
    return newLines


@dataclass(frozen=True)
class TablePreparationBundle:
    """Explizite Ausgabevorbereitungs-Garbe über einer Legacy-Prepare-Instanz."""

    def prepare_output_table(self, prepare: Any, *args, **kwargs) -> tuple:
        return prepare_output_table(prepare, *args, **kwargs)

    def select_display_lines(self, prepare: Any, *args, **kwargs):
        return select_display_lines(prepare, *args, **kwargs)

    def prepare_row_cells(self, prepare: Any, *args, **kwargs):
        return prepare_row_cells(prepare, *args, **kwargs)

    def tag_output_column(self, prepare: Any, *args, **kwargs):
        return tag_output_column(prepare, *args, **kwargs)

    def cell_work(self, prepare: Any, *args, **kwargs):
        return cell_work(prepare, *args, **kwargs)

    def deduplicate_parameter_sections(self, tables: Any, param_lines: set, param_lines_not: set) -> tuple[set, set]:
        return tables.getPrepare.deleteDoublesInSets(param_lines, param_lines_not)

    def capture_last_line_number(self, tables: Any, content_table: list, param_lines: set, param_lines_not: set) -> int:
        finally_display_lines, _headings, _newer, _numlen, _rows_range = tables.getPrepare.prepare4out_beforeForLoop_SpaltenZeilenBestimmen(
            content_table,
            param_lines,
            param_lines_not,
        )
        line_numbers = list(finally_display_lines)
        line_numbers.sort()
        tables.lastLineNumber = line_numbers[-1]
        return tables.lastLineNumber

    def prepare_main_output(
        self,
        tables: Any,
        param_lines: set,
        param_lines_not: set,
        content_table: list,
        rows_as_numbers: set,
        gebr_spalten: dict,
        prim_spalten: set | None = None,
    ) -> MainTablePreparationResult:
        finally_display_lines, new_table, numlen, rows_range, old2new_table = tables.getPrepare.prepare4out(
            param_lines,
            param_lines_not,
            content_table,
            rows_as_numbers,
            gebr_spalten,
            primSpalten=prim_spalten,
        )
        return MainTablePreparationResult(
            finally_display_lines=finally_display_lines,
            new_table=new_table,
            numlen=numlen,
            rows_range=rows_range,
            old2new_table=old2new_table,
        )

    def prepare_kombi_output(
        self,
        tables: Any,
        animals_professions_table: list,
        komb_rows: object,
        sum_of_all_combi_rows_amount: int,
        reli_table_len_until_now: int,
        kombi_csv_number: int,
    ) -> KombiTablePreparationResult:
        try:
            from orderedset import OrderedSet
        except Exception:  # pragma: no cover - Legacy-Fallback
            OrderedSet = set
        (
            finally_display_lines,
            new_table,
            line_len,
            animals_table,
            old2new_table_animals_professions,
        ) = tables.getPrepare.prepare4out(
            OrderedSet(),
            OrderedSet(),
            animals_professions_table,
            komb_rows,
            {},
            sum_of_all_combi_rows_amount,
            reliTableLenUntilNow=reli_table_len_until_now,
            kombiCSVNumber=kombi_csv_number,
        )
        return KombiTablePreparationResult(
            finally_display_lines=finally_display_lines,
            new_table=new_table,
            line_len=line_len,
            animals_professions_table=animals_table,
            old2new_table_animals_professions=old2new_table_animals_professions,
        )

    def snapshot(self) -> dict:
        return {
            "class": "TablePreparationBundle",
            "display_line_morphism": "select_display_lines",
            "row_morphism": "prepare_row_cells",
            "tag_gluing_morphism": "tag_output_column",
            "cell_morphism": "cell_work",
            "parallel_row_morphism": "prepare_rows_in_processes",
            "deduplication_morphism": "deduplicate_parameter_sections",
            "last_line_morphism": "capture_last_line_number",
            "universal_operations": [
                "deduplicate_parameter_sections",
                "capture_last_line_number",
                "prepare_main_output",
                "prepare_kombi_output",
                "process_parallel_row_chunks",
            ],
            "main_table_result": "MainTablePreparationResult",
            "kombi_table_result": "KombiTablePreparationResult",
            "legacy_delegate": "libs.lib4tables_prepare.Prepare",
        }


def bootstrap_table_preparation() -> TablePreparationBundle:
    return TablePreparationBundle()
