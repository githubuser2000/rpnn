#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Compatibility facade for the Reta table runtime.

Stage 25 moves the actual ``Tables`` global table section into
``reta_architecture.table_runtime``.  This module remains for old imports such
as ``from tableHandling import Tables`` and for historical helper re-exports.
"""
from __future__ import annotations

from center import cliout, getTextWrapThings, i18n, infoLog, output
from lib4tables_prepare import setShellRowsAmount, shellRowsAmount
from reta_architecture.number_theory import (
    couldBePrimeNumberPrimzahlkreuz,
    divisorGenerator,
    isPrimMultiple,
    moonNumber,
    primCreativity,
    primFak,
    primMultiple,
    primRepeat,
)
from reta_architecture.output_syntax import (
    NichtsSyntax,
    OutputSyntax,
    bbCodeSyntax,
    csvSyntax,
    emacsSyntax,
    htmlSyntax,
    markdownSyntax,
)
from reta_architecture.table_runtime import (
    BreakoutException,
    OUTPUT_SEMANTICS,
    TableRuntimeBundle,
    Tables,
    bootstrap_table_runtime,
)

__all__ = [
    "BreakoutException",
    "OUTPUT_SEMANTICS",
    "TableRuntimeBundle",
    "Tables",
    "bootstrap_table_runtime",
    "OutputSyntax",
    "NichtsSyntax",
    "csvSyntax",
    "emacsSyntax",
    "markdownSyntax",
    "bbCodeSyntax",
    "htmlSyntax",
    "moonNumber",
    "primFak",
    "divisorGenerator",
    "primRepeat",
    "primCreativity",
    "primMultiple",
    "isPrimMultiple",
    "couldBePrimeNumberPrimzahlkreuz",
    "setShellRowsAmount",
    "shellRowsAmount",
    "cliout",
    "getTextWrapThings",
    "i18n",
    "infoLog",
    "output",
]
