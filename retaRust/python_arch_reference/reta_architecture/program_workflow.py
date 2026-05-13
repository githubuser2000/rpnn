# -*- coding: utf-8 -*-
"""Programmlauf-/Workflow-Schicht für Reta.

Diese Schicht besitzt die alte Top-Level-Orchestrierung des CLI-Programms:
Haupttabelle laden, Parameter in lokale/negative Schnitte übersetzen,
Spalten-/Tabellengenerierung aufrufen, Kombi-Joins durchführen und am Ende die
kanonische Ausgabe erzeugen.

Die einzelnen Legacy-Algorithmen bleiben bewusst unverändert in ihren
bestehenden Klassen; diese Schicht ist der explizite universelle Glue-Knoten
zwischen Parameterlaufzeit, Spaltenauswahl, Tabellengenerierung und Renderer.
"""
from __future__ import annotations

import csv
import html
import json
import os
import sys
from dataclasses import dataclass
from pathlib import Path

try:
    from orderedset import OrderedSet
except Exception:  # pragma: no cover - Legacy-Fallback
    OrderedSet = set


@dataclass(frozen=True)
class ProgramWorkflowBundle:
    repo_root: Path
    i18n: object
    csv_file_names: object
    gebrochen_spalten_maximum_plus1: int

    def _csv_path(self, csv_file_name: str) -> Path:
        return self.repo_root / "csv" / os.path.basename(csv_file_name)

    def _decode_religion_cell(self, cell: str, output_kind: str) -> str:
        if not (cell[:2] == "|{" and cell[-2:] == "}|"):
            return html.escape(cell, quote=True) if output_kind == "html" else cell
        payload = json.loads(cell[1:-1])
        if output_kind == "bbcode":
            return payload["bbcode"]
        if output_kind == "html":
            return payload["html"]
        return payload[""]

    def _requested_religion_output_kind(self, program) -> str:
        art_parameter = "".join(("--", self.i18n.ausgabeParas["art"], "="))
        if f"{art_parameter}{self.i18n.ausgabeArt['bbcode']}" in program.argv:
            return "bbcode"
        if f"{art_parameter}{self.i18n.ausgabeArt['html']}" in program.argv:
            return "html"
        return "plain"

    def _load_religion_table(self, program) -> None:
        if "Brython" in sys.version.split():
            raise SystemExit("Brython runtime is not supported by this refactor path.")

        output_kind = self._requested_religion_output_kind(program)
        place = self._csv_path(self.csv_file_names.religion)
        with open(place, mode="r", encoding="utf-8") as csv_file:
            raw_rows = list(enumerate(csv.reader(csv_file, delimiter=";")))

        parallel_result = None
        if raw_rows:
            try:
                from .parallel_execution import decode_religion_rows_in_processes

                parallel_result = decode_religion_rows_in_processes(
                    raw_rows,
                    output_kind,
                    config=getattr(program, "parallel_config", None),
                )
            except Exception as exc:  # pragma: no cover - deterministic serial fallback.
                parallel_result = None
                try:
                    program.parallel_csv_decode_result = {
                        "mode": "serial_fallback",
                        "operation": "decode_religion_rows",
                        "error": f"{type(exc).__name__}: {exc}",
                    }
                except Exception:
                    pass

        if parallel_result is not None:
            program.relitable = list(parallel_result.values)
            try:
                program.parallel_csv_decode_result = parallel_result.snapshot()
            except Exception:
                pass
        else:
            program.relitable = [
                [self._decode_religion_cell(cell, output_kind) for cell in col]
                for _row_index, col in raw_rows
            ]
            if raw_rows and not hasattr(program, "parallel_csv_decode_result"):
                try:
                    program.parallel_csv_decode_result = {
                        "mode": "serial",
                        "operation": "decode_religion_rows",
                        "rows": len(raw_rows),
                    }
                except Exception:
                    pass

        if program.relitable:
            program.RowsLen = len(program.relitable[0])
            for _egal in range(len(program.relitable) + 1, program.tables.hoechsteZeile[1024] + 2):
                program.relitable += [[""] * len(program.relitable[0])]
        else:
            program.RowsLen = 0

    def _apply_language_specific_motive_column(self, program) -> None:
        language = self.i18n.sprachen[self.i18n.sprachenWahl]
        change_motives_column = (
            self.i18n.tomDecodedMotivesLang["kr"]
            if language == "kr"
            else self.i18n.tomDecodedMotivesLang["cn"]
            if language == "cn"
            else self.i18n.tomDecodedMotivesLang["vn"]
            if language == "vn"
            else ""
        )
        if change_motives_column in ("", "de"):
            return
        place = self._csv_path(change_motives_column)
        with open(place, mode="r", encoding="utf-8") as csv_file:
            for row_index, col in enumerate(csv.reader(csv_file, delimiter=";")):
                try:
                    program.relitable[row_index][10] = col[0]
                except IndexError:
                    pass

    def _reset_runtime_flags(self, program) -> None:
        program.htmlOrBBcode = False
        program.breiteORbreiten = False
        program.keineleereninhalte = False
        program.tables.keineleereninhalte = False

    def _read_positive_and_negative_parameters(self, program, argv):
        (
            param_lines,
            program.rowsAsNumbers,
            program.rowsOfcombi,
            spaltenreihenfolgeundnurdiese,
            program.puniverseprims,
            program.generRows,
        ) = program.parametersToCommandsAndNumbers(argv)
        (
            param_lines_not,
            program.rowsAsNumbersNot,
            program.rowsOfcombiNot,
            spaltenreihenfolgeundnurdiese_not,
            program.puniverseprimsNot,
            program.generRowsNot,
        ) = program.parametersToCommandsAndNumbers(argv, "-")
        return param_lines, param_lines_not, spaltenreihenfolgeundnurdiese, spaltenreihenfolgeundnurdiese_not

    def bring_all_important_begin_things(self, program, argv) -> tuple:
        """Legacy-compatible replacement for Program.bringAllImportantBeginThings."""

        self._load_religion_table(program)
        self._apply_language_specific_motive_column(program)
        self._reset_runtime_flags(program)

        (
            param_lines,
            param_lines_not,
            spaltenreihenfolgeundnurdiese,
            _spaltenreihenfolgeundnurdiese_not,
        ) = self._read_positive_and_negative_parameters(program, argv)

        program.dataDict = [{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}]
        column_selection = program.architecture.bootstrap_column_selection(ordered_set_factory=OrderedSet)
        program.spaltenTypeNaming = column_selection.type_naming
        program.spaltenArtenKey_SpaltennummernValue = column_selection.new_bucket_map()

        program.storeParamtersForColumns()
        program.produceAllSpaltenNumbers()

        if program.htmlOrBBcode and not program.breiteORbreiten:
            # Keep legacy behaviour local to the table object. The old module-level
            # shellRowsAmount assignment did not update tableHandling.shellRowsAmount.
            program.tables.textWidth = 0

        table_preparation = program.architecture.bootstrap_table_preparation()
        param_lines, param_lines_not = table_preparation.deduplicate_parameter_sections(
            program.tables, param_lines, param_lines_not
        )
        column_selection.bind_program_sections(program, param_lines)

        table_generation = program.architecture.bootstrap_table_generation(csv_file_names=self.csv_file_names)
        table_generation_result = table_generation.build_for_program(program, param_lines, param_lines_not)

        return (
            program.RowsLen,
            param_lines,
            param_lines_not,
            program.relitable,
            program.rowsAsNumbers,
            table_generation_result.animals_professions_table,
            program.rowsOfcombi,
            table_generation_result.kombi_table_kombis,
            table_generation_result.maintable2subtable_relation,
            spaltenreihenfolgeundnurdiese,
            table_generation_result.prim_spalten,
            table_generation_result.gebr,
            table_generation_result.animals_professions_table2,
            table_generation_result.kombi_table_kombis2,
            table_generation_result.maintable2subtable_relation2,
        )

    def workflow_everything(self, program, argv) -> list:
        (
            program.RowsLen,
            param_lines,
            param_lines_not,
            program.relitable,
            program.rowsAsNumbers,
            animals_professions_table,
            program.rowsOfcombi,
            kombi_table_kombis,
            maintable2subtable_relation,
            spaltenreihenfolgeundnurdiese,
            prim_spalten,
            gebr,
            animals_professions_table2,
            kombi_table_kombis2,
            maintable2subtable_relation2,
        ) = self.bring_all_important_begin_things(program, argv)

        main_prepare = program.architecture.bootstrap_table_preparation().prepare_main_output(
            program.tables,
            param_lines,
            param_lines_not,
            program.relitable,
            program.rowsAsNumbers,
            gebr_spalten=gebr,
            prim_spalten=prim_spalten,
        )
        finally_display_lines = main_prepare.finally_display_lines
        new_table = main_prepare.new_table
        numlen = main_prepare.numlen
        rows_range = main_prepare.rows_range
        old2new_table = main_prepare.old2new_table

        if len(program.rowsOfcombi) > 0:
            new_table = self.combi_table_workflow(
                program,
                animals_professions_table,
                finally_display_lines,
                kombi_table_kombis,
                maintable2subtable_relation,
                new_table,
                old2new_table,
                param_lines,
                self.csv_file_names.kombi13,
            )

        if len(program.rowsOfcombi2) > 0:
            new_table = self.combi_table_workflow(
                program,
                animals_professions_table2,
                finally_display_lines,
                kombi_table_kombis2,
                maintable2subtable_relation2,
                new_table,
                old2new_table,
                param_lines,
                self.csv_file_names.kombi15,
            )

        new_table = program.tables.getOut.onlyThatColumns(new_table, spaltenreihenfolgeundnurdiese)
        program.newTable = new_table
        program.finallyDisplayLines = finally_display_lines
        program.rowsRange = rows_range
        program.numlen = numlen

        result = program.tables.getOut.cliOut(finally_display_lines, new_table, numlen, rows_range)
        program.architecture.sync_tables(
            program.tables,
            output_mode=program.architecture.morphisms.renderers.output_mode_for_tables(program.tables),
            finally_display_lines=finally_display_lines,
            rows_range=rows_range,
        )
        return result

    def combi_table_workflow(
        self,
        program,
        animals_professions_table,
        finally_display_lines,
        kombi_table_kombis,
        maintable2subtable_relation,
        new_table,
        old2new_table,
        param_lines,
        csv_file_name,
    ):
        chosen_kombi_lines = program.tables.getCombis.prepare_kombi(
            finally_display_lines,
            animals_professions_table,
            param_lines,
            finally_display_lines,
            kombi_table_kombis,
        )
        komb_rows = (
            program.rowsOfcombi
            if csv_file_name == self.csv_file_names.kombi13
            else (program.rowsOfcombi2 if csv_file_name == self.csv_file_names.kombi15 else None)
        )
        reli_table_len_until_now = len(new_table[0]) - (
            len(program.rowsOfcombi) + len(program.rowsOfcombi2)
            if csv_file_name == self.csv_file_names.kombi13
            else len(program.rowsOfcombi2)
            if csv_file_name == self.csv_file_names.kombi15
            else None
        )
        kombi_csv_number = (
            0
            if csv_file_name == self.csv_file_names.kombi13
            else 1
            if csv_file_name == self.csv_file_names.kombi15
            else None
        )
        kombi_prepare = program.architecture.bootstrap_table_preparation().prepare_kombi_output(
            program.tables,
            animals_professions_table,
            komb_rows,
            program.tables.getCombis.sumOfAllCombiRowsAmount,
            reli_table_len_until_now=reli_table_len_until_now,
            kombi_csv_number=kombi_csv_number,
        )
        kombi_tables = program.tables.getCombis.prepareTableJoin(
            chosen_kombi_lines, kombi_prepare.new_table
        )
        return program.tables.getCombis.tableJoin(
            new_table,
            kombi_tables,
            maintable2subtable_relation,
            old2new_table,
            komb_rows,
        )

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "repo_root": str(self.repo_root),
            "main_csv": getattr(self.csv_file_names, "religion", None),
            "kombi_csvs": [
                getattr(self.csv_file_names, "kombi13", None),
                getattr(self.csv_file_names, "kombi15", None),
            ],
            "orchestration_steps": [
                "load_religion_table",
                "decode_religion_csv_rows_in_process_chunks_when_enabled",
                "apply_language_specific_motive_column",
                "parse_positive_and_negative_parameters",
                "store_parameter_semantics",
                "produce_column_numbers",
                "bind_column_sections",
                "build_generated_tables",
                "prepare_output_table_via_table_preparation",
                "prepare_rows_in_process_chunks_when_enabled",
                "join_kombi_tables",
                "render_cli_output",
            ],
        }


def bootstrap_program_workflow(repo_root, i18n, csv_file_names, gebrochen_spalten_maximum_plus1) -> ProgramWorkflowBundle:
    return ProgramWorkflowBundle(
        repo_root=Path(repo_root).resolve(),
        i18n=i18n,
        csv_file_names=csv_file_names,
        gebrochen_spalten_maximum_plus1=gebrochen_spalten_maximum_plus1,
    )
