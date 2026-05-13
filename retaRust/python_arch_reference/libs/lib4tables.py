#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from __future__ import annotations

"""Legacy compatibility facade for Reta table helpers.

The semantic owners were extracted into explicit architecture modules:

* ``reta_architecture.output_syntax`` owns renderer syntax morphisms.
* ``reta_architecture.number_theory`` owns arithmetic morphisms.

This module intentionally remains import-compatible for older code that still
uses ``from lib4tables import ...``.
"""

import math as math

from reta_architecture.output_syntax import (
    NichtsSyntax,
    OutputSyntax,
    bbCodeSyntax,
    csvSyntax,
    emacsSyntax,
    htmlSyntax,
    markdownSyntax,
)
from reta_architecture.number_theory import (
    couldBePrimeNumberPrimzahlkreuz,
    couldBePrimeNumberPrimzahlkreuz_fuer_aussen,
    couldBePrimeNumberPrimzahlkreuz_fuer_innen,
    divisorGenerator,
    isPrimMultiple,
    moonNumber,
    primCreativity,
    primFak,
    primMultiple,
    primRepeat,
)

__all__ = [
    "math",
    "NichtsSyntax",
    "OutputSyntax",
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
    "couldBePrimeNumberPrimzahlkreuz_fuer_innen",
    "couldBePrimeNumberPrimzahlkreuz_fuer_aussen",
]
