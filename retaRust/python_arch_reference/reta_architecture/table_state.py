#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from __future__ import annotations

"""Mutable table-state sections for Reta.

Stage 26 separates the table runtime's raw mutable state from the ``Tables``
class itself.  ``Tables`` remains the legacy-compatible global table section,
but the state it owns is now grouped into explicit sections:

* highest-row bounds
* generated-column metadata
* display flags and religion-number state
* display-row -> original-row mapping

This is intentionally small and conservative: old attributes continue to point
to the same mutable dict/list objects, so legacy code can still mutate them in
place while the architecture can inspect the state as named sections.
"""

from collections import OrderedDict
from dataclasses import dataclass, field
from typing import Any, Callable, MutableMapping

try:
    from orderedset import OrderedSet
except (ModuleNotFoundError, ImportError):  # pragma: no cover - minimal env fallback
    OrderedSet = set


def _highest_rows(hoechst_zeil: int | None) -> dict[int, int]:
    if hoechst_zeil is None:
        return {1024: 1024, 114: 163}
    return {1024: hoechst_zeil, 114: hoechst_zeil}


@dataclass
class GeneratedColumnSection:
    """Mutable metadata section for generated table columns."""

    parameters: MutableMapping[int, Any] = field(default_factory=OrderedDict)
    tags: MutableMapping[int, Any] = field(default_factory=OrderedDict)

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "parameters_len": len(self.parameters),
            "tags_len": len(self.tags),
            "parameters_type": type(self.parameters).__name__,
            "tags_type": type(self.tags).__name__,
        }


@dataclass
class TableDisplayState:
    """Mutable display/session flags shared by prepare, combi and output layers."""

    keine_ueberschriften: bool = False
    keine_leeren_inhalte: bool = False
    spalte_gestirn: bool = False
    religion_numbers: list = field(default_factory=list)

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "keine_ueberschriften": self.keine_ueberschriften,
            "keine_leeren_inhalte": self.keine_leeren_inhalte,
            "spalte_gestirn": self.spalte_gestirn,
            "religion_numbers_len": len(self.religion_numbers),
        }


@dataclass
class TableStateSections:
    """Named mutable sections owned by a ``Tables`` instance."""

    highest_rows: dict[int, int]
    display: TableDisplayState = field(default_factory=TableDisplayState)
    generated_columns: GeneratedColumnSection = field(default_factory=GeneratedColumnSection)
    row_display_to_original: MutableMapping[int, int] = field(default_factory=OrderedDict)
    generated_rows_factory: Callable[[], Any] = OrderedSet

    def new_generated_rows(self):
        return self.generated_rows_factory()

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "highest_rows": dict(self.highest_rows),
            "display": self.display.snapshot(),
            "generated_columns": self.generated_columns.snapshot(),
            "row_display_to_original_len": len(self.row_display_to_original),
            "generated_rows_factory": getattr(self.generated_rows_factory, "__name__", str(self.generated_rows_factory)),
        }


@dataclass(frozen=True)
class TableStateBundle:
    """Factory/metadata object for table-state sections."""

    ordered_dict_factory: Callable[[], MutableMapping] = OrderedDict
    ordered_set_factory: Callable[[], Any] = OrderedSet

    def create_sections(self, hoechst_zeil: int | None = None) -> TableStateSections:
        return TableStateSections(
            highest_rows=_highest_rows(hoechst_zeil),
            generated_columns=GeneratedColumnSection(
                parameters=self.ordered_dict_factory(),
                tags=self.ordered_dict_factory(),
            ),
            row_display_to_original=self.ordered_dict_factory(),
            generated_rows_factory=self.ordered_set_factory,
        )

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "sections": [
                "highest_rows",
                "display",
                "generated_columns",
                "row_display_to_original",
                "generated_rows",
            ],
            "architecture_owner": "reta_architecture.table_state",
            "legacy_owner": "reta_architecture.table_runtime.Tables",
        }


def bootstrap_table_state(
    ordered_dict_factory: Callable[[], MutableMapping] | None = None,
    ordered_set_factory: Callable[[], Any] | None = None,
) -> TableStateBundle:
    return TableStateBundle(
        ordered_dict_factory=ordered_dict_factory or OrderedDict,
        ordered_set_factory=ordered_set_factory or OrderedSet,
    )
