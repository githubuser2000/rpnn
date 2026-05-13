from __future__ import annotations

from collections import namedtuple
from dataclasses import dataclass
from typing import Callable, Dict


COLUMN_BUCKET_NAMES: tuple[str, ...] = (
    "ordinary",
    "generated1",
    "concat1",
    "kombi1",
    "boolAndTupleSet1",
    "gebroUni1",
    "gebrGal1",
    "generated2",
    "kombi2",
    "gebrEmo1",
    "gebrGroe1",
    "metakonkret",
    "ordinaryNot",
    "generate1dNot",
    "concat1Not",
    "kombi1Not",
    "boolAndTupleSet1Not",
    "gebroUni1Not",
    "gebrGal1Not",
    "generated2Not",
    "kombi2Not",
    "gebrEmo1Not",
    "gebrGroe1Not",
    "metakonkretNot",
)

_COLUMN_TYPE = namedtuple("SpaltenTyp", " ".join(COLUMN_BUCKET_NAMES))
_COLUMN_BUCKET_VALUES: tuple[tuple[int, int], ...] = tuple(
    (negation, bucket)
    for negation in (0, 1)
    for bucket in range(12)
)


@dataclass(frozen=True)
class ColumnSelectionBundle:
    """Legacy-compatible column-bucket model for reta's Spalten selection.

    This is the explicit architecture layer for the old local pair-space
    `(positive|negative, bucket-type)` used by `reta.Program`. It keeps the
    exact field names and bucket coordinates, but stops `reta.py` from owning the
    schema inline.
    """

    ordered_set_factory: Callable[[], object] = set

    @property
    def type_naming(self):
        return _COLUMN_TYPE(*_COLUMN_BUCKET_VALUES)

    @property
    def bucket_values(self) -> tuple[tuple[int, int], ...]:
        return _COLUMN_BUCKET_VALUES

    @property
    def bucket_names(self) -> tuple[str, ...]:
        return COLUMN_BUCKET_NAMES

    def new_bucket_map(self) -> Dict[tuple[int, int], object]:
        return {key: self.ordered_set_factory() for key in self.bucket_values}

    def bind_program_sections(self, program, param_lines) -> None:
        """Bind normalized bucket sections back onto the legacy Program object.

        The method is deliberately side-effectful: it is a compatibility adapter
        for the current Program runtime. Architecturally, this is the point where
        the local column-bucket presheaf is interpreted as the program's concrete
        selected column sections.
        """

        buckets = program.spaltenArtenKey_SpaltennummernValue
        naming = program.spaltenTypeNaming

        program.rowsAsNumbers = buckets[naming.ordinary]
        program.generRows = buckets[naming.generated1]
        program.puniverseprims = buckets[naming.concat1]
        program.rowsOfcombi = buckets[naming.kombi1]
        program.rowsOfcombi2 = buckets[naming.kombi2]
        program.onlyGenerated = buckets[naming.boolAndTupleSet1]

        ones = []
        for item in program.onlyGenerated:
            try:
                if len(item) == 1:
                    ones += item
            except TypeError:
                continue
        program.tables.getConcat.ones = ones

        if len(program.rowsOfcombi) > 0:
            param_lines.add("ka")
        if len(program.rowsOfcombi2) > 0:
            param_lines.add("ka2")

        program.tables.generRows = program.generRows
        program.tables.getPrepare.rowsAsNumbers = program.rowsAsNumbers
        program.tables.getOut.rowsAsNumbers = program.rowsAsNumbers
        program.tables.SpaltenVanillaAmount = len(program.rowsAsNumbers)

    def snapshot(self) -> dict:
        return {
            "class": "ColumnSelectionBundle",
            "bucket_names": list(self.bucket_names),
            "bucket_values": [list(value) for value in self.bucket_values],
            "positive_bucket_count": 12,
            "negative_bucket_count": 12,
        }


def bootstrap_column_selection(ordered_set_factory=None) -> ColumnSelectionBundle:
    return ColumnSelectionBundle(ordered_set_factory=ordered_set_factory or set)
