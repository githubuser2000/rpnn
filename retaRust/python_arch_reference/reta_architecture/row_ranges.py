from __future__ import annotations

"""Stage-37 row-range morphisms for Reta input semantics.

Historically the row-range parser lived in ``libs/center.py`` as a cluster of
free functions with German compatibility names such as ``BereichToNumbers2`` and
``isZeilenAngabe``.  Stage 37 is the first controlled activation after the
Stage-36 commit/rollback planning layer: the parser logic is now owned by the
architecture package while ``center`` keeps only thin legacy wrappers.

Mathematical reading:

* a row-range expression is a local section over the input/prompt open set;
* token validation is a morphism from raw text to a row-range open set;
* expansion is a morphism from a row-range section to a finite row index set;
* subtractive ranges and multiple-ranges glue through a small universal union /
  difference construction;
* the old ``center`` API and the new architecture API are connected by a
  natural transformation: both routes must produce the same row set.
"""

from dataclasses import dataclass
from typing import Optional, Sequence

from .input_semantics import RowRangeSyntax


def str_as_generator_to_set(text: str) -> Optional[set[int]]:
    """Parse bracket/brace/tuple row set syntax, preserving legacy behaviour."""
    try:
        text = str(text)
        if text[0] == "(" and text[-1] == ")":
            text = "[" + text[1:-1] + "]"

        if (text[0] == "[" and text[-1] == "]") or (text[0] == "{" and text[-1] == "}"):
            try:
                result = eval(text)  # noqa: S307 - compatibility with historical center.py parser
                result = set(result)
                if type(result) is set and all((type(a) is int for a in result)):
                    return result
            except Exception:
                return None
    except Exception:
        return None
    return None


def is_fraction_range_token(text: str, syntax: RowRangeSyntax | None = None) -> bool:
    syntax = syntax or RowRangeSyntax()
    return syntax.is_fraction_range_token(text)


def is_integer_range_token(text: str, syntax: RowRangeSyntax | None = None) -> bool:
    syntax = syntax or RowRangeSyntax()
    return syntax.is_integer_range_token(text)


def is_row_range_token(text: str, syntax: RowRangeSyntax | None = None) -> bool:
    syntax = syntax or RowRangeSyntax()
    text = str(text)
    generated1 = str_as_generator_to_set(text)
    generated2 = str_as_generator_to_set(text[1:])
    return syntax.is_integer_range_token(text) or generated1 is not None or generated2 is not None


def is_fraction_or_integer_range(text: str, syntax: RowRangeSyntax | None = None) -> bool:
    syntax = syntax or RowRangeSyntax()
    return all(
        is_fraction_range_token(g, syntax) or is_row_range_token(g, syntax)
        for g in syntax.split_comma_list(str(text))
    )


def is_fraction_range(text: str, syntax: RowRangeSyntax | None = None) -> bool:
    syntax = syntax or RowRangeSyntax()
    tokens = syntax.split_comma_list(str(text))
    any_at_all = any(len(token) > 0 for token in tokens)
    return all(is_fraction_range_token(g, syntax) or (g == "" and any_at_all) for g in tokens)


def is_row_range(text: str, syntax: RowRangeSyntax | None = None) -> bool:
    syntax = syntax or RowRangeSyntax()
    tokens = syntax.split_comma_list(str(text))
    any_at_all = any(len(token) > 0 for token in tokens)
    return all(is_row_range_token(g, syntax) or (g == "" and any_at_all) for g in tokens)


def add_non_multiple_values(range_couple, around, max_value, target: set[int]) -> None:
    for number in range(int(range_couple[0]), int(range_couple[1]) + 1):
        for offset in around:
            plus = number + offset
            if plus < max_value:
                target |= {plus}
            minus = number - offset
            if minus > 0 and minus < max_value:
                target |= {minus}


def add_multiple_values(range_couple, around, max_value, target: set[int]) -> None:
    i = 0
    if len(around) == 0 or len(set(around) - {0}) == 0:
        while all([int(range_couple[0]) * i < max_value - offset for offset in around]):
            i += 1
            for number in range(int(range_couple[0]), int(range_couple[1]) + 1):
                value = number * i
                if value <= max_value:
                    target |= {value}
    else:
        while all([int(range_couple[0]) * i < max_value - offset for offset in around]):
            i += 1
            for number in range(int(range_couple[0]), int(range_couple[1]) + 1):
                for offset in around:
                    plus = (number * i) + offset
                    if plus <= max_value:
                        target |= {plus}
                    minus = (number * i) - offset
                    if minus > 0 and minus < max_value:
                        target |= {minus}


def add_range_couple_values(range_couple, around, max_value, target: set[int], multiples: bool) -> None:
    if (
        len(range_couple) == 2
        and range_couple[0].isdecimal()
        and range_couple[0] != "0"
    ):
        plus_tuples = range_couple[1].split("+")
        if len(plus_tuples) < 2:
            around = [0]
        else:
            valid = True
            numbers = []
            for token in plus_tuples:
                if token.isdecimal():
                    numbers += [int(token)]
                else:
                    valid = False
            if valid and len(numbers) > 0:
                around = numbers[1:]
                range_couple[1] = numbers[0]
        if multiples:
            add_multiple_values(range_couple, around, max_value, target)
        else:
            add_non_multiple_values(range_couple, around, max_value, target)


def add_single_range_segment(segment: str, include: set[int], exclude: set[int], max_value, multiples: bool) -> None:
    if len(segment) > 1 and segment[0] == "-":
        segment = segment[1:]
        target = exclude
    elif len(segment) > 0 and segment[0] != "-":
        target = include
    else:
        target = None
    around = []
    if target is not None:
        plus_tuple = segment.split("+")
        if segment.isdecimal():
            segment = segment + "-" + segment
        elif len(plus_tuple) > 0 and plus_tuple[0].isdecimal():
            segment = plus_tuple[0] + "-" + plus_tuple[0]
            if len(plus_tuple) > 1:
                segment += "+" + "+".join(plus_tuple[1:])
        range_couple = segment.split("-")
        add_range_couple_values(range_couple, around, max_value, target, multiples)


def range_to_numbers(
    ranges_text: str,
    multiples: bool = False,
    max_value: int = 1028,
    allow_less_equal_zero: bool = False,
    syntax: RowRangeSyntax | None = None,
) -> set[int]:
    """Expand a Reta row-range expression to row numbers.

    The implementation mirrors the historical ``BereichToNumbers2`` semantics,
    including subtractive segments, ``v``-prefixed multiples and explicit set
    literals such as ``{1,2,3}``.
    """
    syntax = syntax or RowRangeSyntax()
    ranges_text = syntax.compact_comma_list(str(ranges_text))
    if not is_row_range(ranges_text, syntax):
        return set()

    if not multiples and max_value == 0:
        max_value = float("inf")

    segments = syntax.split_comma_list(ranges_text)
    include: set[int] = set()
    exclude: set[int] = set()

    for segment in segments:
        if len(segment) > 1 and segment[0] == "-":
            generated = str_as_generator_to_set(segment[1:])
            if generated is not None:
                exclude |= generated
                continue
        elif len(segment) > 0 and segment[0] != "-":
            generated = str_as_generator_to_set(segment)
            if generated is not None:
                include |= generated
                continue
        if len(segment) > 0 and segment[0] == syntax.multiple_prefix:
            segment = segment[1:]
            segment_multiples = True
        else:
            segment_multiples = False
        add_single_range_segment(
            segment,
            include,
            exclude,
            1028 if (multiples or segment_multiples) and max_value == float("inf") else max_value,
            multiples or segment_multiples,
        )
    result = include - exclude
    if allow_less_equal_zero:
        return result
    return set(filter(lambda value: value > 0, result))


@dataclass(frozen=True)
class RowRangeMorphismBundle:
    """Activated architecture owner for Reta row-range parsing morphisms."""

    syntax: RowRangeSyntax
    legacy_owner: str = "libs.center"
    activated_stage: int = 37

    def str_as_generator(self, text: str) -> Optional[set[int]]:
        return str_as_generator_to_set(text)

    def is_fraction_token(self, text: str) -> bool:
        return is_fraction_range_token(text, self.syntax)

    def is_integer_token(self, text: str) -> bool:
        return is_integer_range_token(text, self.syntax)

    def is_row_token(self, text: str) -> bool:
        return is_row_range_token(text, self.syntax)

    def is_fraction_range(self, text: str) -> bool:
        return is_fraction_range(text, self.syntax)

    def is_row_range(self, text: str) -> bool:
        return is_row_range(text, self.syntax)

    def is_fraction_or_integer_range(self, text: str) -> bool:
        return is_fraction_or_integer_range(text, self.syntax)

    def range_to_numbers(
        self,
        ranges_text: str,
        multiples: bool = False,
        max_value: int = 1028,
        allow_less_equal_zero: bool = False,
    ) -> set[int]:
        return range_to_numbers(
            ranges_text,
            multiples=multiples,
            max_value=max_value,
            allow_less_equal_zero=allow_less_equal_zero,
            syntax=self.syntax,
        )

    def add_single_range_segment(self, segment: str, include: set[int], exclude: set[int], max_value, multiples: bool) -> None:
        add_single_range_segment(segment, include, exclude, max_value, multiples)

    def add_range_couple_values(self, range_couple, around, max_value, target: set[int], multiples: bool) -> None:
        add_range_couple_values(range_couple, around, max_value, target, multiples)

    def add_non_multiple_values(self, range_couple, around, max_value, target: set[int]) -> None:
        add_non_multiple_values(range_couple, around, max_value, target)

    def add_multiple_values(self, range_couple, around, max_value, target: set[int]) -> None:
        add_multiple_values(range_couple, around, max_value, target)

    def snapshot(self) -> dict:
        return {
            "class": "RowRangeMorphismBundle",
            "stage": self.activated_stage,
            "legacy_owner": self.legacy_owner,
            "capsule": "InputPromptCapsule",
            "category": "ActivatedRowRangeCategory",
            "functor": "RowRangeActivationFunctor",
            "natural_transformation": "CenterRowRangeToArchitectureTransformation",
            "syntax": self.syntax.snapshot(),
            "morphisms": [
                "str_as_generator_to_set",
                "is_fraction_range_token",
                "is_integer_range_token",
                "is_row_range_token",
                "is_fraction_or_integer_range",
                "is_fraction_range",
                "is_row_range",
                "range_to_numbers",
                "add_single_range_segment",
                "add_range_couple_values",
                "add_non_multiple_values",
                "add_multiple_values",
            ],
            "compatibility_names": [
                "strAsGeneratorToListOfNumStrs",
                "isZeilenBruchAngabe_betweenKommas",
                "isZeilenBruchOrGanzZahlAngabe",
                "isZeilenBruchAngabe",
                "isZeilenAngabe",
                "isZeilenAngabe_betweenKommas",
                "BereichToNumbers2",
                "BereichToNumbers2_EinBereich",
                "BereichToNumbers2_EinBereich_Menge",
                "BereichToNumbers2_EinBereich_Menge_nichtVielfache",
                "BereichToNumbers2_EinBereich_Menge_vielfache",
            ],
            "observable_invariant": "center wrappers and architecture morphisms return identical row sets for accepted row-range expressions",
        }


def bootstrap_row_range_morphisms(syntax: RowRangeSyntax | None = None) -> RowRangeMorphismBundle:
    return RowRangeMorphismBundle(syntax=syntax or RowRangeSyntax())


# Legacy spelling aliases used by compatibility wrappers and tests.
strAsGeneratorToListOfNumStrs = str_as_generator_to_set
BereichToNumbers2 = range_to_numbers
BereichToNumbers2_EinBereich = add_single_range_segment
BereichToNumbers2_EinBereich_Menge = add_range_couple_values
BereichToNumbers2_EinBereich_Menge_nichtVielfache = add_non_multiple_values
BereichToNumbers2_EinBereich_Menge_vielfache = add_multiple_values
