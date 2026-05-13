from __future__ import annotations

"""Text-wrapping and display-width morphisms for table preparation.

This module owns the small but historically sticky part of the output
preparation logic that decides how cells are wrapped for shell-like renderers
and which width applies to a displayed row.  ``libs.lib4tables_prepare`` keeps
its old function names as compatibility wrappers, while the behavior is now an
explicit architecture layer.
"""

import sys
from dataclasses import dataclass
from enum import Enum
from typing import Any, Optional


class Wraptype(Enum):
    pyphen = 1
    pyhyphen = 2
    nohyphen = 3


@dataclass
class TextWrapRuntime:
    shell_rows_amount: Optional[int]
    h_de: Any
    dic: Any
    fill: Any
    wrapping_type: Wraptype = Wraptype.pyhyphen

    def snapshot(self) -> dict:
        return {
            "class": "TextWrapRuntime",
            "shell_rows_amount": self.shell_rows_amount,
            "has_hyphenator": self.h_de is not None,
            "has_dictionary": self.dic is not None,
            "has_fill": self.fill is not None,
            "wrapping_type": self.wrapping_type.name,
        }


_RUNTIME: TextWrapRuntime | None = None


def refresh_textwrap_runtime(wrapping_type: Wraptype | None = None) -> TextWrapRuntime:
    """Refresh the wrapping runtime from the legacy center provider."""
    global _RUNTIME
    try:
        from .runtime_compat import getTextWrapThings

        shell_rows_amount, h_de, dic, fill = getTextWrapThings()
    except Exception:
        shell_rows_amount, h_de, dic, fill = 0, None, None, None
    if wrapping_type is None and _RUNTIME is not None:
        wrapping_type = _RUNTIME.wrapping_type
    _RUNTIME = TextWrapRuntime(
        shell_rows_amount=shell_rows_amount,
        h_de=h_de,
        dic=dic,
        fill=fill,
        wrapping_type=wrapping_type or Wraptype.pyhyphen,
    )
    return _RUNTIME


def textwrap_runtime() -> TextWrapRuntime:
    global _RUNTIME
    if _RUNTIME is None:
        _RUNTIME = refresh_textwrap_runtime()
    return _RUNTIME


def set_shell_rows_amount(shell_rows_amount: Optional[int]) -> None:
    textwrap_runtime().shell_rows_amount = shell_rows_amount


def get_shell_rows_amount() -> Optional[int]:
    return textwrap_runtime().shell_rows_amount


def set_wrapping_type(wrapping_type: Wraptype) -> None:
    textwrap_runtime().wrapping_type = wrapping_type


def get_wrapping_type() -> Wraptype:
    return textwrap_runtime().wrapping_type


def chunks(lst, n):
    """Yield successive n-sized chunks from lst."""
    for i in range(0, len(lst), n):
        yield lst[i : i + n]


def split_more_if_not_small(text_list: list, len_to_be: int) -> tuple:
    new_list: list = []
    needed_to_be_done_at_all = False
    len_to_be -= 0
    for _k, text in enumerate(text_list):
        if len(text) > len_to_be:
            needed_to_be_done_at_all = True
    if needed_to_be_done_at_all:
        for _k, text in enumerate(text_list):
            if len(text) > len_to_be:
                new_list += list(chunks(text, len_to_be))
            else:
                new_list += [text]
    if needed_to_be_done_at_all:
        return tuple(new_list)
    return tuple(text_list)


def alxwrap(text: str, len_: int, wrapping_type: Wraptype | None = None):
    """Wrap a single string using the configured reta wrapping backend."""
    runtime = textwrap_runtime()
    fill = runtime.fill
    dic = runtime.dic
    h_de = runtime.h_de
    wrapping_type = wrapping_type or runtime.wrapping_type
    if fill is None:
        return (text,)
    if "Brython" in sys.version.split():
        return (text,)
    try:
        return (
            dic.wrap(text, len_)
            if wrapping_type == Wraptype.pyphen and len_ != 0
            else (
                split_more_if_not_small(
                    fill(text, width=len_, use_hyphenator=h_de).split("\n"), len_
                )
                if wrapping_type == Wraptype.pyhyphen and len_ != 0
                else (text,)
            )
        )
    except Exception:
        return (
            dic.wrap(text, len_)
            if wrapping_type == Wraptype.pyhyphen and len_ != 0
            else (
                split_more_if_not_small(
                    fill(text, width=len_, use_hyphenator=h_de).split("\n"), len_
                )
                if wrapping_type == Wraptype.pyphen and len_ != 0
                else (text,)
            )
        )


def wrap_cell_text(text: str, length: int, wrapping_type: Wraptype | None = None) -> list | tuple | None:
    """Return wrapped text parts only when wrapping is needed."""
    if len(text) > length and length != 0:
        return alxwrap(text, length, wrapping_type=wrapping_type)
    return None


def width_for_row(prepare, row_to_display: int, combi_rows1: int = 0) -> int:
    """Compute the effective text width for one displayed row."""
    shell_rows_amount = getattr(prepare, "shellRowsAmount", get_shell_rows_amount())
    if shell_rows_amount == 0:
        return 0
    combi_rows = combi_rows1 if combi_rows1 != 0 else len(prepare.rowsAsNumbers)
    if len(prepare.rowsAsNumbers) - combi_rows < len(prepare.breiten):
        breiten: list = prepare.breiten[len(prepare.rowsAsNumbers) - combi_rows :]
    else:
        breiten = []
    delta = -1
    if row_to_display + delta < len(breiten) and row_to_display + delta >= 0:
        return breiten[row_to_display + delta]
    return prepare.textwidth


@dataclass(frozen=True)
class TableWrappingBundle:
    runtime: TextWrapRuntime

    def wrap_text(self, text: str, length: int):
        return wrap_cell_text(text, length)

    def width_for_row(self, prepare, row_to_display: int, combi_rows1: int = 0) -> int:
        return width_for_row(prepare, row_to_display, combi_rows1)

    def snapshot(self) -> dict:
        return {
            "class": "TableWrappingBundle",
            "runtime": self.runtime.snapshot(),
            "morphisms": [
                "alxwrap",
                "wrap_cell_text",
                "width_for_row",
                "split_more_if_not_small",
            ],
            "legacy_owner": "libs.lib4tables_prepare.Prepare",
        }


def bootstrap_table_wrapping(force_refresh: bool = False) -> TableWrappingBundle:
    runtime = refresh_textwrap_runtime() if force_refresh or _RUNTIME is None else textwrap_runtime()
    return TableWrappingBundle(runtime=runtime)
