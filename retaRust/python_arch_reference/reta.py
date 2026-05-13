#!/usr/bin/env pypy3
# -*- coding: utf-8 -*-
"""Reta CLI-Programmfassade.

Die Topologie-/Garben-/Morphismen-Architektur lebt in ``reta_architecture``.
Diese Datei hält nur noch die legacy-kompatible ``Program``-Oberfläche und
leitet die eigentliche Arbeit an explizite Architekturschichten weiter.
"""
from __future__ import annotations

import os
import platform
import sys
from pathlib import Path
from typing import Iterable, Optional, Union

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "libs"))

from center import i18n, retaHilfe, infoLog
from reta_architecture import ParameterSemanticsBuilder, RetaArchitecture
from reta_architecture.parallel_execution import bootstrap_parallel_execution, extract_parallel_config_from_argv
from reta_architecture.parameter_runtime import (
    apply_upper_limit_argument,
    apply_width_parameter,
    parameters_to_commands_and_numbers,
    produce_all_spalten_numbers,
    upper_limit_from_arguments,
    upper_limit_values_for_argument,
)
from reta_architecture.output_syntax import OutputSyntax
from reta_architecture.number_theory import primCreativity

gebrochenSpaltenMaximumPlus1 = i18n.gebrochenSpaltenMaximumPlus1
csvFileNames = i18n.csvFileNames
i18nR = i18n.retapy


def render_color(tag_name, value, options, parent, context):
    return '<span style="color:%s;">%s</span>' % (tag_name, value)


class Program:
    def produceAllSpaltenNumbers(self, neg=""):
        return produce_all_spalten_numbers(self, neg)

    def breiteBreitenSysArgvPara(self, cmd, neg) -> bool:
        return apply_width_parameter(self, cmd, neg)

    def apply_output_mode(self, outputtype: str) -> bool:
        breite_ist_null = "".join(("--", i18n.ausgabeParas["breite"], "=0"))
        applied = self.architecture.morphisms.renderers.apply_output_mode(
            self.tables,
            outputtype,
            zero_width_callback=lambda: self.breiteBreitenSysArgvPara(breite_ist_null[2:], ""),
        )
        if applied is None:
            return False
        if applied.marks_html_or_bbcode:
            self.htmlOrBBcode = True
        return True

    def storeParamtersForColumns(self):
        Program.ParametersMain = i18n.ParametersMain

        Program.lambdaGebrUnivUndGalax = lambda paraValues: {
            abs(int(chosen)) if chosen.isdecimal() else None
            for chosen in [value for value in (paraValues.split(","))]
        } - {None, 0, 1}

        Program.lambdaPrimGalax = lambda paraValues: {
            abs(int(chosen))
            if chosen.isdecimal() and primCreativity(abs(int(chosen))) == 1
            else None
            for chosen in [value for value in (paraValues.split(","))]
        } - {None, 0, 1}

        builder = ParameterSemanticsBuilder(
            self.architecture.schema,
            gebrochen_spalten_maximum_plus1=gebrochenSpaltenMaximumPlus1,
            invert_alles=self.__invertAlles,
            initial_data_dict=self.dataDict,
            prim_number_predicate=primCreativity,
            alles_parameter_names=Program.ParametersMain.alles,
        )
        build_result = builder.build()

        Program.paraNdataMatrix = list(self.architecture.schema.para_n_data_matrix)
        Program.paraNdataMatrixAugmented = build_result.para_n_data_matrix
        Program.kombiParaNdataMatrix = build_result.kombi_para_n_data_matrix
        Program.kombiParaNdataMatrix2 = build_result.kombi_para_n_data_matrix2

        self.kombiReverseDict = build_result.kombi_reverse_dict
        self.kombiReverseDict2 = build_result.kombi_reverse_dict2
        self.AllSimpleCommandSpalten = build_result.all_simple_command_columns
        self.paraMainDict = build_result.para_main_dict
        self.paraDict = build_result.para_dict
        self.dataDict = build_result.data_dict

        self.tables.dataDict = self.dataDict
        self.architecture.sync_program_semantics(self.paraDict, self.dataDict)
        self.architecture.sync_tables(self.tables)

    def parametersToCommandsAndNumbers(self, argv, neg="") -> Iterable[Union[set, set, set, list]]:
        return parameters_to_commands_and_numbers(self, argv, neg)

    def helpPage(self):
        retaHilfe()

    def bringAllImportantBeginThings(self, argv) -> tuple:
        return self.architecture.bootstrap_program_workflow(
            csv_file_names=csvFileNames
        ).bring_all_important_begin_things(self, argv)

    def oberesMaximumArg(self, arg) -> tuple:
        return upper_limit_values_for_argument(self, arg)

    def oberesMaximum2(self, argv2) -> Optional[int]:
        return upper_limit_from_arguments(self, argv2)

    def oberesMaximum(self, arg) -> bool:
        return apply_upper_limit_argument(self, arg)

    @property
    def propInfoLog(self) -> OutputSyntax:
        global infoLog
        return infoLog

    @propInfoLog.setter
    def propInfoLog(self, value: bool):
        global infoLog
        infoLog = value

    def __init__(
        self,
        argv=[],
        alternativeShellRowsAmount: Optional[int] = None,
        Txt=None,
        runAlles=True,
    ):
        raw_argv = [a.strip() for a in argv]
        inherited_parallel_config = getattr(Txt, "parallel_config", None) if Txt is not None else None
        self.argv, self.parallel_config = extract_parallel_config_from_argv(
            raw_argv,
            inherited=inherited_parallel_config,
        )
        self.allesParameters = 0
        repo_root = Path(__file__).resolve().parent
        self.architecture = getattr(Txt, "architecture", None) or RetaArchitecture.bootstrap(repo_root)
        self.architecture.parallel_execution = bootstrap_parallel_execution(self.parallel_config)
        if Txt is not None and getattr(Txt, "architecture", None) is None:
            Txt.architecture = self.architecture
        if Txt is not None:
            Txt.parallel_config = self.parallel_config
        self.tables = self.architecture.bootstrap_table_runtime().create_tables(
            self.oberesMaximum2(self.argv[1:]), Txt
        )
        self.tables.architecture = self.architecture
        self.tables.parallel_config = self.parallel_config
        self.tables.getPrepare.parallel_config = self.parallel_config

        self.breiteHasBeenOnceZero = False
        self.obZeilenBereicheAngegeben = False
        if platform.system() == "Windows":
            self.tables.getOut.color = False

        self.__runAlles = runAlles
        self.__invertAlles = False
        if runAlles:
            self.__resultingTable = self.workflowEverything(self.argv)

    def invertAlles(self):
        self.__invertAlles = True

    def run(self):
        if not self.__runAlles:
            self.__resultingTable = self.workflowEverything(self.argv)

    @property
    def resultingTable(self) -> list:
        return self.__resultingTable

    def workflowEverything(self, argv) -> list:
        return self.architecture.bootstrap_program_workflow(
            csv_file_names=csvFileNames
        ).workflow_everything(self, argv)

    def combiTableWorkflow(
        self,
        animalsProfessionsTable,
        finallyDisplayLines,
        kombiTable_Kombis,
        maintable2subtable_Relation,
        newTable,
        old2newTable,
        paramLines,
        csvFileName,
    ):
        return self.architecture.bootstrap_program_workflow(
            csv_file_names=csvFileNames
        ).combi_table_workflow(
            self,
            animalsProfessionsTable,
            finallyDisplayLines,
            kombiTable_Kombis,
            maintable2subtable_Relation,
            newTable,
            old2newTable,
            paramLines,
            csvFileName,
        )


if __name__ == "__main__":
    Program(sys.argv)
