# -*- coding: utf-8 -*-

"""Compatibility facade for reta's split words namespace.

The original `i18n/words.py` monolith has been split into bootstrap,
context/schema, matrix, and runtime layers. Importers may keep importing
`i18n.words`; this module now re-exports the split modules in the same
namespace.
"""

from .words_bootstrap import *
from .words_context import *
from .words_matrix import *
from .words_runtime import *

MODULE_SPLIT = {
    'bootstrap': 'i18n.words_bootstrap',
    'context': 'i18n.words_context',
    'matrix': 'i18n.words_matrix',
    'runtime': 'i18n.words_runtime',
    'legacy_monolith': 'i18n.words_legacy_monolith',
}

__all__ = [name for name in globals() if not name.startswith('__')]
