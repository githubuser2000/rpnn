# -*- coding: utf-8 -*-

"""Shared bootstrap for reta's split words modules."""

import gettext
import os
import pprint
import sys

# import sys
from collections import OrderedDict, defaultdict, namedtuple

# from dataclasses import dataclass
from typing import Any, NamedTuple, Optional, Tuple, Union

# from typing import Optional, Union
import pprint

pp = pprint.PrettyPrinter(indent=4)
try:
    from orderedset import OrderedSet
except (ModuleNotFoundError, ImportError):
    OrderedSet = set


def alxp(text):
    global output
    """Für mich, damit ich mal alle prints ausschalten kann zum vorführen,
    wenn ich noch beim Entwicklen war."""
    if "-debug" in sys.argv:
        if type(text) is str:
            print(text)
        else:
            pp.pprint(text)


def x(text1, text):
    global output
    """Für mich, damit ich mal alle prints ausschalten kann zum vorführen,
    wenn ich noch beim Entwicklen war."""
    if "-debug" in sys.argv:
        if type(text) is str:
            print(text1 + ": " + text)
        else:
            print(text1 + ": ", end="")
            pp.pprint(text)


__all__ = [name for name in globals() if not name.startswith('__')]
