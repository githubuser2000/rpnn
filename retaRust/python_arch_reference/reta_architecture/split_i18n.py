from __future__ import annotations

from types import SimpleNamespace
from typing import Iterable, Tuple


DEFAULT_MODULE_NAMES: Tuple[str, ...] = (
    "i18n.words_context",
    "i18n.words_matrix",
    "i18n.words_runtime",
)


def build_split_i18n_proxy(module_names: Iterable[str] = DEFAULT_MODULE_NAMES):
    """Build a light-weight namespace from the physically split i18n modules.

    Stage 3 turned the old words monolith into several modules. A lot of the
    legacy code still expects a single `i18n` object, but it no longer has to
    come from the compatibility facade `i18n.words`. This helper assembles a
    concrete namespace directly from the split source modules.
    """

    proxy = SimpleNamespace()
    loaded_modules = {}
    for module_name in module_names:
        module = __import__(module_name, fromlist=["*"])
        loaded_modules[module_name] = module
        for name in dir(module):
            if name.startswith("_"):
                continue
            setattr(proxy, name, getattr(module, name))
    setattr(proxy, "__source_modules__", tuple(loaded_modules.keys()))
    return proxy
