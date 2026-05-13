from __future__ import annotations

"""Generated-column morphisms for reta tables.

This module is the explicit architecture layer for the simpler generated-column
algorithms that used to live directly inside ``libs/lib4tables_concat.py``.  The
legacy ``Concat`` methods remain as compatibility wrappers, but their behavior is
owned here now.
"""

from dataclasses import dataclass, field
from fractions import Fraction
import re
from collections import OrderedDict
from copy import copy, deepcopy
from itertools import zip_longest
from typing import Iterable, List, Tuple

try:
    from orderedset import OrderedSet
except (ModuleNotFoundError, ImportError):
    OrderedSet = set

# Runtime dependencies are imported lazily. This keeps the architecture package
# importable in probe/test environments before optional UI libraries such as
# ``rich`` or ``prompt_toolkit`` are installed or stubbed.
_i18n = None
ST = None
primfaktoren = None
Multiplikationen = None
multiples = None
unique_everseen = None
couldBePrimeNumberPrimzahlkreuz = None
couldBePrimeNumberPrimzahlkreuz_fuer_aussen = None
couldBePrimeNumberPrimzahlkreuz_fuer_innen = None
Primzahlkreuz_pro_contra_strs = None
moonNumber = None
primCreativity = None
primMultiple = None


def _ensure_runtime_dependencies() -> None:
    global _i18n, ST, primfaktoren, Multiplikationen, multiples, unique_everseen
    global couldBePrimeNumberPrimzahlkreuz
    global couldBePrimeNumberPrimzahlkreuz_fuer_aussen
    global couldBePrimeNumberPrimzahlkreuz_fuer_innen
    global Primzahlkreuz_pro_contra_strs
    global moonNumber, primCreativity, primMultiple
    if _i18n is not None:
        return
    from .runtime_compat import (
        Multiplikationen as runtime_Multiplikationen,
        Primzahlkreuz_pro_contra_strs as prime_cross_pro_contra_strings,
        i18n as runtime_i18n,
        multiples as runtime_multiples,
        primfaktoren as runtime_primfaktoren,
        unique_everseen as runtime_unique_everseen,
    )
    from .number_theory import (
        couldBePrimeNumberPrimzahlkreuz as prime_cross_any,
        couldBePrimeNumberPrimzahlkreuz_fuer_aussen as prime_cross_outer,
        couldBePrimeNumberPrimzahlkreuz_fuer_innen as prime_cross_inner,
        moonNumber as moon_number,
        primCreativity as prim_creativity,
        primMultiple as prim_multiple,
    )
    from .tag_schema import ST as st_enum

    _i18n = runtime_i18n.concat
    ST = st_enum
    primfaktoren = runtime_primfaktoren
    Multiplikationen = runtime_Multiplikationen
    multiples = runtime_multiples
    unique_everseen = runtime_unique_everseen
    Primzahlkreuz_pro_contra_strs = prime_cross_pro_contra_strings
    couldBePrimeNumberPrimzahlkreuz = prime_cross_any
    couldBePrimeNumberPrimzahlkreuz_fuer_aussen = prime_cross_outer
    couldBePrimeNumberPrimzahlkreuz_fuer_innen = prime_cross_inner
    moonNumber = moon_number
    primCreativity = prim_creativity
    primMultiple = prim_multiple


@dataclass(frozen=True)
class GeneratedColumnSpec:
    """Metadata for one generated-column morphism."""

    method_name: str
    trigger_columns: Tuple[int, ...]
    tags: Tuple[str, ...] = field(default_factory=tuple)
    description: str = ""

    def snapshot(self) -> dict:
        return {
            "method_name": self.method_name,
            "trigger_columns": list(self.trigger_columns),
            "tags": list(self.tags),
            "description": self.description,
        }


class GeneratedColumnRegistry:
    """Registry of generated-column morphisms known to the architecture layer."""

    def __init__(self, specs: Iterable[GeneratedColumnSpec]):
        self._specs = tuple(specs)

    @property
    def specs(self) -> Tuple[GeneratedColumnSpec, ...]:
        return self._specs

    def names(self) -> List[str]:
        return [spec.method_name for spec in self._specs]

    def snapshot(self) -> dict:
        return {
            "class": "GeneratedColumnRegistry",
            "count": len(self._specs),
            "morphisms": [spec.snapshot() for spec in self._specs],
        }


DEFAULT_GENERATED_COLUMN_REGISTRY = GeneratedColumnRegistry(
    (
        GeneratedColumnSpec(
            "concatVervielfacheZeile",
            (19, 90),
            ("legacy-column-propagation",),
            "Propagates selected row content to multiples of the source row.",
        ),
        GeneratedColumnSpec(
            "concatModallogik",
            (),
            ("modal-logic", "generated-concepts"),
            "Generates modal-logic columns from selected concept-row pairs.",
        ),
        GeneratedColumnSpec(
            "concat1RowPrimUniverse2",
            (),
            ("prim-universe", "fractional-generated-column"),
            "Generates prime-universe and fractional relation columns from selected commands.",
        ),
        GeneratedColumnSpec(
            "concat1PrimzahlkreuzProContra",
            (),
            ("prime-cross", "pro-contra", "generated-column"),
            "Generates prime-cross pro/contra columns from row-number structure.",
        ),
        GeneratedColumnSpec(
            "concatPrimCreativityType",
            (64,),
            ("sternPolygon", "galaxie"),
            "Generates the prime/sun/moon creativity type column.",
        ),
        GeneratedColumnSpec(
            "concatGleichheitFreiheitDominieren",
            (132,),
            ("sternPolygon", "universum"),
            "Generates equality/freedom/domination classification from row number.",
        ),
        GeneratedColumnSpec(
            "concatGeistEmotionEnergieMaterieTopologie",
            (242,),
            ("sternPolygon", "universum"),
            "Generates mind/emotion/energy/matter/topology classification.",
        ),
        GeneratedColumnSpec(
            "concatMondExponzierenLogarithmusTyp",
            (64,),
            ("sternPolygon", "universum", "galaxie"),
            "Generates moon/exponent/logarithm relation columns.",
        ),
        GeneratedColumnSpec(
            "concatLovePolygon",
            (9,),
            ("sternPolygon", "galaxie", "gleichfoermigesPolygon"),
            "Generates love-polygon text from the existing structure-size columns.",
        ),
        GeneratedColumnSpec(
            "createSpalteGestirn",
            (64,),
            ("sternPolygon", "universum", "galaxie"),
            "Generates the Gestirn/Sonne/Mond/Planet classification column from row numbers.",
        ),
    )
)


def _generated_parameter_index(concat) -> int:
    return len(concat.tables.generatedSpaltenParameter) + concat.tables.SpaltenVanillaAmount


def _ensure_generated_parameter_slot_free(concat) -> int:
    index = _generated_parameter_index(concat)
    if index in concat.tables.generatedSpaltenParameter:
        raise ValueError
    return index


def concat_love_polygon(concat, relitable: list, rows_as_numbers: set) -> tuple:
    _ensure_runtime_dependencies()
    concat.relitable = relitable
    if set(rows_as_numbers) >= {9}:
        rows_as_numbers |= {len(concat.relitable[0])}
        concat.tables.generatedSpaltenParameter_Tags[len(rows_as_numbers) - 1] = frozenset(
            {ST.sternPolygon, ST.galaxie, ST.gleichfoermigesPolygon}
        )
        for i, _cols in enumerate(deepcopy(concat.relitable)):
            if concat.relitable[i][8].strip() != "":
                concat.relitable[i] += [
                    "".join(
                        (
                            concat.relitable[i][8],
                            _i18n.polygon1[" der eigenen Strukturgröße ("],
                            concat.relitable[i][4],
                            _i18n.polygon2[") auf dich bei gleichförmigen Polygonen"],
                        )
                    )
                ]
            else:
                concat.relitable[i] += [""]
        index = _ensure_generated_parameter_slot_free(concat)
        concat.tables.generatedSpaltenParameter[index] = concat.tables.dataDict[0][9]
    return concat.relitable, rows_as_numbers


def gleichheit_freiheit_vergleich(zahl: int) -> str:
    _ensure_runtime_dependencies()
    zahl = int(zahl)
    ausgabe_string_list = []
    if zahl % 4 == 0:
        ausgabe_string_list += [_i18n.gleichheitFreiheitVergleich["Dominieren, Unterordnen"]]
    if zahl % 4 == 1:
        ausgabe_string_list += [_i18n.gleichheitFreiheitVergleich["Freiheit"]]
    if zahl % 4 == 3:
        ausgabe_string_list += [_i18n.gleichheitFreiheitVergleich["Einschränkung der Freiheit"]]
    if zahl % 4 == 2:
        if (zahl - 2) % 8 == 0:
            ausgabe_string_list += [_i18n.gleichheitFreiheitVergleich["Gleichheit"]]
        if (zahl - 6) % 16 == 0:
            ausgabe_string_list += [_i18n.gleichheitFreiheitVergleich["den anderen überbieten wollen"]]
        if (zahl - 14) % 16 == 0:
            ausgabe_string_list += [_i18n.gleichheitFreiheitVergleich["den anderen unterbieten wollen"]]
    return "; ".join(ausgabe_string_list)


def geist_emotion_energie_materie_topologie(zahl: int) -> str:
    _ensure_runtime_dependencies()
    zahl = int(zahl)
    pr_fa = primfaktoren(zahl)
    auss = [couldBePrimeNumberPrimzahlkreuz_fuer_aussen(a) for a in pr_fa]
    innen = [couldBePrimeNumberPrimzahlkreuz_fuer_innen(a) for a in pr_fa]
    zwei = len([a for a in pr_fa if a == 2])
    gefuehl = any(auss)
    denken = any(innen)
    total_topologie = zwei > 1 and gefuehl
    etwas_topologie = (zwei > 1 or (zwei > 0 and gefuehl)) and not total_topologie
    total_materie = zwei > 4
    etwas_materie = zwei == 4
    wenig_materie = zwei == 3
    kaum_materie = zwei == 2
    x, y, z = denken, (2 in pr_fa), (3 in pr_fa)
    total_energie = x and y and z
    einermassen_energie = ((x and y) or (y and z) or (y and z)) and not total_energie
    kaum_energie = not einermassen_energie and not total_energie and (x or y or z)
    ausgabe_string_list = []
    if denken:
        ausgabe_string_list += [_i18n.energietopologie1["eine Denkart"]]
    if gefuehl:
        ausgabe_string_list += [_i18n.energietopologie1["eine Gefühlsart"]]
    if total_materie:
        ausgabe_string_list += [_i18n.energietopologie1["total eine Art, etwas geistig zu erzeugen"]]
    if total_topologie:
        ausgabe_string_list += [_i18n.energietopologie1["total eine Art zu erleben"]]
    if total_energie:
        ausgabe_string_list += [_i18n.energietopologie1["total eine Energie-Art"]]
    if etwas_topologie:
        ausgabe_string_list += [_i18n.energietopologie1["etwas eine Art zu erleben"]]
    if etwas_materie:
        ausgabe_string_list += [_i18n.energietopologie1["etwas eine Art, etwas geistig zu erzeugen"]]
    if wenig_materie:
        ausgabe_string_list += [_i18n.energietopologie1["wenig eine Art, etwas geistig zu erzeugen"]]
    if einermassen_energie:
        ausgabe_string_list += [_i18n.energietopologie1["einigermaßen eine Energie-Art"]]
    if kaum_energie:
        ausgabe_string_list += [_i18n.energietopologie1["kaum eine Energie-Art"]]
    if kaum_materie:
        ausgabe_string_list += [_i18n.energietopologie1["kaum eine Art, etwas geistig zu erzeugen"]]
    return "; ".join(ausgabe_string_list)


def concat_gleichheit_freiheit_dominieren(concat, relitable: list, rows_as_numbers: set) -> tuple:
    _ensure_runtime_dependencies()
    concat.relitable = relitable
    if set(rows_as_numbers) >= {132}:
        rows_as_numbers |= {len(concat.relitable[0])}
        concat.tables.generatedSpaltenParameter_Tags[len(rows_as_numbers) - 1] = frozenset(
            {ST.sternPolygon, ST.universum}
        )
        for i, _cols in enumerate(deepcopy(concat.relitable[: concat.tables.lastLineNumber + 1])):
            if i == 0:
                ausgabe_string = _i18n.gleichheitFreiheitVergleich[
                    "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert"
                ]
            else:
                ausgabe_string = gleichheit_freiheit_vergleich(i)
            concat.relitable[i] += [ausgabe_string]
        index = _ensure_generated_parameter_slot_free(concat)
        concat.tables.generatedSpaltenParameter[index] = concat.tables.dataDict[0][132]
    return concat.relitable, rows_as_numbers


def concat_geist_emotion_energie_materie_topologie(concat, relitable: list, rows_as_numbers: set) -> tuple:
    _ensure_runtime_dependencies()
    concat.relitable = relitable
    if set(rows_as_numbers) >= {242}:
        rows_as_numbers |= {len(concat.relitable[0])}
        concat.tables.generatedSpaltenParameter_Tags[len(rows_as_numbers) - 1] = frozenset(
            {ST.sternPolygon, ST.universum}
        )
        for i, _cols in enumerate(deepcopy(concat.relitable[: concat.tables.lastLineNumber + 1])):
            if i == 0:
                ausgabe_string = _i18n.ausgabeString[
                    "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art"
                ]
            else:
                ausgabe_string = geist_emotion_energie_materie_topologie(i)
            concat.relitable[i] += [ausgabe_string]
        index = _ensure_generated_parameter_slot_free(concat)
        concat.tables.generatedSpaltenParameter[index] = concat.tables.dataDict[0][242]
    return concat.relitable, rows_as_numbers


def concat_prim_creativity_type(concat, relitable: list, rows_as_numbers: set) -> tuple:
    _ensure_runtime_dependencies()
    concat.relitable = relitable
    if set(rows_as_numbers) >= {64}:
        rows_as_numbers |= {len(concat.relitable[0])}
        concat.tables.generatedSpaltenParameter_Tags[len(rows_as_numbers) - 1] = frozenset(
            {ST.sternPolygon, ST.galaxie}
        )
        for i, _cols in enumerate(deepcopy(concat.relitable[: concat.tables.lastLineNumber + 1])):
            prim_creativity_type = primCreativity(i)
            concat.relitable[i] += [
                _i18n.kreaZahl["Evolutions-Züchtungs-Kreativität"]
                if i == 0
                else (
                    _i18n.kreaZahl["0. Primzahl 1"]
                    if prim_creativity_type == 0
                    else (
                        _i18n.kreaZahl["1. Primzahl und Sonnenzahl"]
                        if prim_creativity_type == 1
                        else (
                            _i18n.kreaZahl["2. Sonnenzahl, aber keine Primzahl"]
                            if prim_creativity_type == 2
                            else _i18n.kreaZahl["3. Mondzahl"]
                        )
                    )
                )
            ]
        index = _ensure_generated_parameter_slot_free(concat)
        concat.tables.generatedSpaltenParameter[index] = concat.tables.dataDict[0][64]
    return concat.relitable, rows_as_numbers


def concat_mond_exponzieren_logarithmus_typ(concat, relitable: list, rows_as_numbers: set) -> tuple:
    _ensure_runtime_dependencies()
    concat.relitable = relitable
    if set(rows_as_numbers) >= {64}:
        hardcoded_couple = (44, 56)
        for rownum, rowheading in zip(
            hardcoded_couple,
            [
                _i18n.mondExpLog1["Mond-Typ eines Sternpolygons"],
                _i18n.mondExpLog1["Mond-Typ eines gleichförmigen Polygons"],
            ],
        ):
            rows_as_numbers |= {len(concat.relitable[0])}
            concat.tables.generatedSpaltenParameter_Tags[len(rows_as_numbers) - 1] = (
                frozenset({ST.sternPolygon, ST.universum, ST.galaxie})
                if rownum == 44
                else frozenset({ST.gleichfoermigesPolygon, ST.universum, ST.galaxie})
            )
            for i, _cols in enumerate(deepcopy(concat.relitable[: concat.tables.lastLineNumber + 1])):
                moon_types_of_1_num = moonNumber(i)
                if i == 0:
                    into = [rowheading]
                else:
                    into = [
                        "[list]"
                        if concat.tables.bbcodeOutputYes
                        else "<ul>"
                        if concat.tables.htmlOutputYes
                        else "",
                        "" if len(moon_types_of_1_num[0]) > 0 else _i18n.mondExpLog2["kein Mond"],
                    ]
                    for k, (basis, exponent_minus_2) in enumerate(zip(*moon_types_of_1_num)):
                        if k > 0:
                            into += [" | "]
                        if concat.tables.htmlOutputYes:
                            into += ["<li>"]
                        elif concat.tables.bbcodeOutputYes:
                            into += ["[*]"]
                        insert = re.sub(
                            r"<SG>",
                            concat.relitable[i][4].strip(),
                            concat.relitable[basis][rownum].rstrip(),
                        )
                        insert = re.sub(r"&lt;SG&gt;", concat.relitable[i][4].strip(), insert)
                        into += [
                            insert,
                            " - ",
                            concat.relitable[exponent_minus_2 + 2][10],
                            " | ",
                            "</li>" if concat.tables.htmlOutputYes else "",
                            concat.relitable[i][10],
                            " + ",
                            concat.relitable[i][11],
                            ", ",
                            concat.relitable[exponent_minus_2 + 2][85],
                        ]
                if concat.tables.htmlOutputYes and i != 0:
                    into += ["</ul>"]
                concat.relitable[i] += ["".join(into)]
            index = _ensure_generated_parameter_slot_free(concat)
            concat.tables.generatedSpaltenParameter[index] = concat.tables.dataDict[0][64]
    return concat.relitable, rows_as_numbers


def concat_vervielfache_zeile(concat, relitable: list, rowsAsNumbers: set) -> tuple:
    _ensure_runtime_dependencies()
    i18n = _i18n
    concat.relitable = relitable
    # reliCopy = deepcopy(relitable)
    spaltenToVervielfache: set = rowsAsNumbers & {90, 19}
    for s in spaltenToVervielfache:
        store = {}
        for z, zeileninhalt in enumerate(
            relitable[2 : concat.tables.lastLineNumber + 1], 2
        ):
            content = zeileninhalt[s]
            if len(content.strip()) > 0:
                store[(z, s)] = content  # interessant
        multis = {}
        for coords, content in store.items():
            vielfacher = 1
            ergebnis = vielfacher * coords[0]
            # multis[ergebnis] = [coords[0]]
            try:
                multis[ergebnis] += [coords[0]]  # interessant
                # spalten wo was hin soll = ursprungszeile1,2,3,...
            except (IndexError, KeyError):
                multis[ergebnis] = [coords[0]]  # interessant

            while ergebnis < len(relitable):
                vielfacher += 1
                ergebnis = vielfacher * coords[0]
                try:
                    multis[ergebnis] += [coords[0]]  # interessant
                    # spalten wo was hin soll = ursprungszeile1,2,3,...
                except (IndexError, KeyError):
                    multis[ergebnis] = [coords[0]]  # interessant
        for z, zeileninhalt in enumerate(
            relitable[2 : concat.tables.lastLineNumber + 1], 2
        ):
            # alle spalten und zeilen
            xx = False

            if len(relitable[z][s].strip()) != 0:
                if concat.tables.htmlOutputYes:
                    relitable[z][s] = ["<li>", relitable[z][s], "</li>"]
                elif concat.tables.bbcodeOutputYes:
                    relitable[z][s] = ["[*]", relitable[z][s]]
                else:
                    relitable[z][s] = [relitable[z][s], " | "]
            else:
                relitable[z][s] = [relitable[z][s]]

            if z in multis:
                for UrZeile in multis[z]:
                    if (
                        UrZeile != z
                        and "".join(relitable[z][s]) != store[(UrZeile, s)]
                        and "".join(relitable[z][s] + [" | "])
                        != store[(UrZeile, s)]
                        and "".join(["<li>"] + relitable[z][s] + ["</li>"])
                        != store[(UrZeile, s)]
                        and "".join(["[*]"] + relitable[z][s])
                        != store[(UrZeile, s)]
                    ):
                        if len(store[(UrZeile, s)]) != 0:
                            if concat.tables.htmlOutputYes:
                                relitable[z][s] += [
                                    "<li>",
                                    store[(UrZeile, s)],
                                    "</li>",
                                ]
                            elif concat.tables.bbcodeOutputYes:
                                relitable[z][s] += ["[*]", store[(UrZeile, s)]]
                            else:
                                xx = (
                                    True
                                    if not concat.tables.bbcodeOutputYes
                                    else False
                                )
                                relitable[z][s] += [store[(UrZeile, s)], " | "]

            if concat.tables.htmlOutputYes:
                relitable[z][s] = ["<ul>"] + relitable[z][s] + ["</ul>"]
            elif concat.tables.bbcodeOutputYes:
                relitable[z][s] = ["[list]"] + relitable[z][s] + ["[/list]"]
            if xx:
                relitable[z][s] = "".join(relitable[z][s][:-1])
            else:
                relitable[z][s] = "".join(relitable[z][s])

    return concat.relitable, rowsAsNumbers


def concat_modallogik(concat, relitable: list, conceptsRowsSetOfTuple: set, rowsAsNumbers: set) -> tuple:
    """setzt die Modallogik um, d.h. Kombination von 2 bisher Programmierten
    Funktionen: 1. vielfache von Primzahlen oder natürlichen Zahlen
    (zweiteres programmiere ich später) bilden
    und die andere Funtion 2. +- 1 +- 2 und Bedeutungsveränderung

    @type relitable: list
    @param relitable: Haupttabelle concat.relitable
    @return: relitable + weitere Tabelle daneben
    """
    _ensure_runtime_dependencies()
    i18n = _i18n

    def getModaloperatorsPerLineCells(lineWeAreAt: int) -> tuple:
        """Gibt ein Tuple aus Strings aus, dass die richtigen Modaloperatoren
        pro Zeile ausgibt
        @type int
        @param Zeile
        @return: Tupel aus Modaloperatoren
        """

        def getModaloperatorsPerLineCoordinates(lineWeAreAt: int) -> tuple:
            modalMainOperatorZeile: int = lineWeAreAt
            amountModaloperators: int = lineWeAreAt - 1
            modalOpElseOperatorsZeilenBegin: int = lineWeAreAt + 1
            modalOpElseOperatorsZeilenEnd: int = (
                lineWeAreAt + amountModaloperators + 1
            )
            return (
                modalMainOperatorZeile,
                modalOpElseOperatorsZeilenBegin,
                modalOpElseOperatorsZeilenEnd,
            )

        coords = getModaloperatorsPerLineCoordinates(lineWeAreAt)
        modaloperators: list = []
        try:
            # modaloperators += [concat.relitable[coords[0]][10]]
            modaloperators += [
                concat.relitable[coords[0]][97],
                concat.relitable[coords[0]][98],
            ]
        except:
            pass
        for coord in range(coords[1], coords[2]):
            try:
                # modaloperators += [concat.relitable[coord][42]]
                modaloperators += [concat.relitable[coord][42]]
            except IndexError:
                pass
        return tuple(modaloperators)

    def ModalLogikIntoTable(
        concept, distanceFromLine, i, into, vorkommenVielfacher_B
    ):
        try:
            modalOperatorenEn = vorkommenVielfacher_B[i][distanceFromLine]["modalS"]
            vervielfachterEn = vorkommenVielfacher_B[i][distanceFromLine][
                "vervielfachter"
            ]
            for modalOperatoren, vervielfachter in zip(
                modalOperatorenEn, vervielfachterEn
            ):
                try:
                    intoItsContent = (
                        concat.relitable[vervielfachter][concept[0]]
                        if abs(distanceFromLine) % 2 == 0
                        else concat.relitable[vervielfachter][concept[1]]
                    )

                    into[i] += [
                        "<li>"
                        if concat.tables.htmlOutputYes
                        else "[*]"
                        if concat.tables.bbcodeOutputYes
                        else ""
                    ]
                    into[i] += (
                        [
                            i18n.modalB["mittelstark überdurchschnittlich: "]
                            if abs(distanceFromLine) == 2
                            else (
                                i18n.modalB["überdurchschnittlich: "]
                                if abs(distanceFromLine) == 1
                                else (
                                    i18n.modalB[
                                        "mittelleicht überdurchschnittlich: "
                                    ]
                                    if abs(distanceFromLine) == 3
                                    else (
                                        i18n.modalB["sehr: "]
                                        if abs(distanceFromLine) == 0 != ""
                                        else i18n.modalB[
                                            "sehr leicht überdurchschnittlich: "
                                        ]
                                    )
                                )
                            ),
                            modalOperatoren[0],
                            " ",
                            intoItsContent
                            if modalOperatoren[0] == concat.relitable[1][97]
                            else intoItsContent.replace(
                                i18n.modalC["intrinsisch"], i18n.modalC["zuerst"]
                            ).replace(
                                i18n.modalC["extrinsisch"],
                                i18n.modalC["als zweites"],
                            ),
                            " ",
                            modalOperatoren[1],
                        ]
                        + (
                            (
                                [
                                    i18n.modalD[", nicht: "],
                                    ", ".join(modalOperatoren[2:]),
                                    i18n.modalD[" (das alles nicht): "],
                                    concat.relitable[vervielfachter][concept[0]]
                                    .replace(
                                        i18n.modalD["extrinsisch"],
                                        i18n.modalD["als zweites"],
                                    )
                                    .replace(
                                        i18n.modalD["intrinsisch"],
                                        i18n.modalD["zuerst"],
                                    ),
                                ]
                                if len(modalOperatoren) > 2
                                else [""]
                            )
                            if abs(distanceFromLine) % 2 == 1
                            else [""]
                        )
                        + [
                            " | "
                            if not concat.tables.htmlOutputYes
                            and not concat.tables.bbcodeOutputYes
                            else ""
                        ]
                    )
                    into[i] += ["</li>" if concat.tables.htmlOutputYes else ""]
                except (IndexError, KeyError):
                    pass
        except (IndexError, KeyError):
            pass

    def storeModalNvervielfachter(
        Orginal_i_mehrere,
        distanceFromLine,
        i,
        modalOperatorEnEn,
        vervielFachter,
        vorkommenVielfacher_B,
    ):
        vorkommenVielfacher_B[i][distanceFromLine] = {
            "i_origS": Orginal_i_mehrere,
            "modalS": modalOperatorEnEn,
            "vervielfachter": vervielFachter,
        }

    def prepareModalIntoTable(
        distanceFromLine,
        getModaloperatorsPerLineCells,
        i,
        storeModalNvervielfachter,
        vorkommenVielfacher,
        vorkommenVielfacher_B,
    ):
        i_with_a_distance = i + distanceFromLine
        try:
            modalOperatorEnEn: list = []
            Orginal_i_mehrere: list = []
            # vorkommenZeilenBegriffe: list = []
            vervielFachter: list = []
            # Ein Couple besteht aus der Zahl, ggf. Primzahl mit ihrem Vielfacher danach
            for couple in vorkommenVielfacher[i_with_a_distance]:
                vorkommen, vielfacher = couple[0], couple[1]
                modalOperatorEnEn += [(getModaloperatorsPerLineCells(vielfacher))]
                # vorkommenZeilenBegriffe += [
                #    vorkommen * vielfacher
                # ]
                vervielFachter += [vorkommen]
                Orginal_i_mehrere += [i_with_a_distance]
            """
            Was ist hier drin gespeichert?
                erster Parameter: das i von allen Distanzen -4 bis 4 mit 0
                zweiter Paramter: Ob: ModalOperator oder was war Orignal i von dem das hier der Vielfacher ist
                dahinter: liste von der Sache
            """
            try:
                vorkommenVielfacher_B[i][distanceFromLine] = OrderedDict(
                    {
                        "i_origS": Orginal_i_mehrere
                        + vorkommenVielfacher_B[i][distanceFromLine]["i_origS"],
                        "modalS": modalOperatorEnEn
                        + vorkommenVielfacher_B[i][distanceFromLine]["modalS"],
                        "vervielfachter": vervielFachter
                        + vorkommenVielfacher_B[i][distanceFromLine][
                            "vervielfachter"
                        ],
                    }
                )

            except (IndexError, KeyError):
                try:
                    storeModalNvervielfachter(
                        Orginal_i_mehrere,
                        distanceFromLine,
                        i,
                        modalOperatorEnEn,
                        vervielFachter,
                        vorkommenVielfacher_B,
                    )
                except (IndexError, KeyError):
                    vorkommenVielfacher_B[i] = OrderedDict()
                    storeModalNvervielfachter(
                        Orginal_i_mehrere,
                        distanceFromLine,
                        i,
                        modalOperatorEnEn,
                        vervielFachter,
                        vorkommenVielfacher_B,
                    )
            del vervielFachter
        except (IndexError, KeyError):
            pass

    def vorkommenNvielfacherPerItsProduct(
        einVorkommen, ergebnis, vielfacher, vorkommenVielfacher
    ):
        # print([einVorkommen, ergebnis, vielfacher])
        try:
            vorkommenVielfacher[ergebnis] += [
                (
                    einVorkommen,
                    vielfacher,
                )
            ]
        except (IndexError, KeyError):
            vorkommenVielfacher[ergebnis] = [
                (
                    einVorkommen,
                    vielfacher,
                )
            ]

    concat.relitable = relitable
    if len(conceptsRowsSetOfTuple) == 0:
        return concat.relitable, rowsAsNumbers

    distances = (-4, -3, -2, -1, 0, 1, 2, 3, 4)
    conceptsRowsSetOfTuple2: list = list(conceptsRowsSetOfTuple)
    reliTableCopy = deepcopy(concat.relitable)
    conceptsRowsSetOfTuple2.sort()
    for o, concept in enumerate(conceptsRowsSetOfTuple2):
        into: dict = {}
        einMalVorkommen = OrderedSet()
        for i, cols in enumerate(reliTableCopy):
            into[i] = [""]
            if i == 0:
                into[i] = [i18n.generiertWort["Generiert: "], cols[concept[0]]]
            elif cols[concept[0]].strip() != "":
                einMalVorkommen |= {i}

        vorkommenVielfacher: OrderedDict = OrderedDict()
        einMalVorkommen = tuple(einMalVorkommen)

        for (
            einVorkommen
        ) in (
            einMalVorkommen
        ):  # d.h. so ein Wort wie weise oder gut kommt in vor in der csv
            vielfacher = 1
            ergebnis = vielfacher * einVorkommen
            vorkommenNvielfacherPerItsProduct(
                einVorkommen, ergebnis, vielfacher, vorkommenVielfacher
            )
            while ergebnis < len(reliTableCopy):
                vielfacher += 1
                ergebnis = vielfacher * einVorkommen
                vorkommenNvielfacherPerItsProduct(
                    einVorkommen, ergebnis, vielfacher, vorkommenVielfacher
                )

        vorkommenVielfacher_B: OrderedDict = OrderedDict()
        for i, zeileninhalte in enumerate(
            reliTableCopy[1 : concat.tables.lastLineNumber + 1], 1
        ):
            for distanceFromLine in distances:
                prepareModalIntoTable(
                    distanceFromLine,
                    getModaloperatorsPerLineCells,
                    i,
                    storeModalNvervielfachter,
                    vorkommenVielfacher,
                    vorkommenVielfacher_B,
                )

        for i, zeileninhalte in enumerate(
            reliTableCopy[1 : concat.tables.lastLineNumber + 1], 1
        ):
            for distanceFromLine in distances:
                ModalLogikIntoTable(
                    concept, distanceFromLine, i, into, vorkommenVielfacher_B
                )
            # wenn i>0
            conditionNvs1perN = concept[0] in {
                62,
                63,
                *range(358, 367 + 1),
                *range(371, 374 + 1),
            }

            if conditionNvs1perN:
                fill_ = zeileninhalte[197]
            else:
                fill_ = zeileninhalte[4]

            if into[i] != [""]:
                into[i] += [
                    "<li>"
                    if concat.tables.htmlOutputYes
                    else "[*]"
                    if concat.tables.bbcodeOutputYes
                    else "",
                    i18n.allesNurBezogenAufSatz,
                    fill_,
                    "</li>" if concat.tables.htmlOutputYes else "",
                ]

        for w, cols in enumerate(reliTableCopy[: concat.tables.lastLineNumber + 1]):
            if concat.tables.htmlOutputYes and "<li>" in into[w]:
                into[w] = ["<ul>"] + into[w] + ["</ul>"]
            elif concat.tables.bbcodeOutputYes and "[*]" in into[w]:
                into[w] = ["[list]"] + into[w] + ["[/list]"]
            concat.relitable[w] += ["".join(into[w])]

        rowsAsNumbers |= {len(concat.relitable[0]) - 1}
        if conditionNvs1perN:
            concat.tables.generatedSpaltenParameter_Tags[
                len(rowsAsNumbers) - 1
            ] = frozenset({ST.gleichfoermigesPolygon, ST.galaxie})
        else:
            concat.tables.generatedSpaltenParameter_Tags[
                len(rowsAsNumbers) - 1
            ] = frozenset({ST.sternPolygon, ST.galaxie})
        if (
            len(concat.tables.generatedSpaltenParameter)
            + concat.tables.SpaltenVanillaAmount
            in concat.tables.generatedSpaltenParameter
        ):
            raise ValueError
        concat.tables.generatedSpaltenParameter[
            len(concat.tables.generatedSpaltenParameter)
            + concat.tables.SpaltenVanillaAmount
        ] = concat.tables.dataDict[1][conceptsRowsSetOfTuple2[o]]

    return concat.relitable, rowsAsNumbers


def concat_primzahlkreuz_pro_contra(self, relitable: list, rowsAsNumbers: set, generatedBefehle: set, ParametersMain) -> tuple:
    """Fügt eine Spalte ein, in der Primzahlen mit Vielfachern
    auf dem Niveau des Universums nicht einfach nur aus einer
    CSV Tabelle geladen werden, sondern durch Primzahlen und
    deren Vielfachern generiert werden.

    @type relitable: list
    @param relitable: Haupttabelle self.relitable
    @return: relitable + weitere Tabelle daneben
    """
    _ensure_runtime_dependencies()
    i18n = _i18n
    global Primzahlkreuz_pro_contra_strs

    if "primzahlkreuzprocontra" in generatedBefehle:
        self.relitable = relitable
        primAmounts = 0
        keinePrimzahl1, keinePrimzahl2 = True, True
        list1, list2 = [], []
        weiter1a, weiter1b, weiter2a, weiter2b = 0, 0, 0, 0
        proPro, contraContra = OrderedDict(), OrderedDict()
        proPro2, contraContra2 = OrderedDict(), OrderedDict()
        dreli = deepcopy(self.relitable)
        headline: str = i18n.headline1
        into_Str1: OrderedDict = OrderedDict()
        into_Str2: OrderedDict = OrderedDict()

        # for num, cols in zip_longest(range(0, 1025), dreli):
        if self.tables.hoechsteZeile[1024] >= len(dreli):
            bereich = zip_longest(
                range(0, self.tables.hoechsteZeile[1024] + 1), dreli
            )
        else:
            bereich = zip(range(0, self.tables.hoechsteZeile[1024] + 1), dreli)

        for num, cols in bereich:
            contraContra2[num] = OrderedSet()
            proPro2[num] = OrderedSet()

            if num == 0:
                into: list = [headline]
            else:
                into: list = []
            into1: list = []
            into2: list = []
            if couldBePrimeNumberPrimzahlkreuz(num):
                primAmounts += 1
            if primCreativity(num) == 1 or num == 1:
                if couldBePrimeNumberPrimzahlkreuz_fuer_innen(num):
                    list1 += [num]
                    if num > 16:
                        if keinePrimzahl1:
                            gegen = list2[weiter1b + 1]
                            weiter1b += 1
                        else:
                            gegen = list1[weiter1a]
                            weiter1a += 1
                        contraContra[num] = gegen
                        contraContra2[num] |= {gegen}
                        into1 += [i18n.gegen["gegen "] + str(gegen)]
                    elif num in (11, 5):
                        if num == 5:
                            gegen = 2
                        elif num == 11:
                            gegen = 2
                        contraContra[num] = gegen
                        contraContra2[num] |= {gegen}
                        into1 += [i18n.gegen["gegen "] + str(gegen)]

                    keinePrimzahl1 = False

                if num in (2, 3):
                    if num == 2:
                        gegen = 1
                        contraContra[num] = gegen
                        contraContra2[num] |= {gegen}
                        into1 += [i18n.gegen["gegen "] + str(gegen)]
                    elif num == 3:
                        pro = 1
                        proPro[num] = pro
                        proPro2[num] |= {pro}
                        into2 += [i18n.pro["pro "] + str(pro)]

                if couldBePrimeNumberPrimzahlkreuz_fuer_aussen(num):
                    list2 += [num]
                    if num > 16:
                        if keinePrimzahl2:
                            pro = list1[weiter2b + 1]
                            weiter2b += 1
                        else:
                            pro = list2[weiter2a]
                            weiter2a += 1
                        proPro[num] = pro
                        proPro2[num] |= {pro}
                        into2 += [i18n.pro["pro "] + str(pro)]
                    elif num in (7, 13):
                        if num == 7:
                            pro = 3
                        elif num == 13:
                            pro = 3
                        proPro[num] = pro
                        proPro2[num] |= {pro}
                        into2 += [i18n.pro["pro "] + str(pro)]

                    keinePrimzahl2 = False
            else:
                if couldBePrimeNumberPrimzahlkreuz_fuer_innen(num):
                    keinePrimzahl1 = True
                elif couldBePrimeNumberPrimzahlkreuz_fuer_aussen(num):
                    keinePrimzahl2 = True

                menge: OrderedSet = OrderedSet()
                for couple in primMultiple(num):
                    couple = list(couple)
                    couple.sort()
                    menge |= {tuple(couple)}
                paare = list(menge)

                for coupleA in paare:
                    # if primCreativity(couple[1]) == 1:
                    #    flagX = True
                    # elif primCreativity(couple[0]) == 1:
                    #    flagX = True
                    #    couple = (couple[1], couple[0])
                    # else:
                    #    flagX = False

                    # if flagX:
                    if coupleA[1] != 1 and coupleA[0] != 1:
                        for couple in (coupleA, (coupleA[1], coupleA[0])):
                            for firstOrSecond in (
                                (1, 0) if couple[0] != couple[1] else (1,)
                            ):
                                if (
                                    couldBePrimeNumberPrimzahlkreuz_fuer_innen(
                                        couple[firstOrSecond]
                                    )
                                    or couple[0] % 2 == 0
                                    or couple[1] % 2 == 0
                                ):
                                    try:
                                        gegen3 = int(
                                            couple[0 if firstOrSecond == 1 else 1]
                                            * contraContra[couple[firstOrSecond]]
                                        )
                                        contraContra[num] = gegen3
                                        contraContra2[num] |= {gegen3}
                                        into1 += [
                                            i18n.gegen["gegen "] + str(gegen3)
                                        ]
                                    except KeyError:
                                        pass
                                if (
                                    couldBePrimeNumberPrimzahlkreuz_fuer_aussen(
                                        couple[1]
                                    )
                                    or couple[1] % 3 == 0
                                    or couple[0] % 3 == 0
                                ):
                                    try:
                                        if num == 4:
                                            pass
                                            # print(
                                            #    f"{couple} ___ {firstOrSecond} | {proPro[couple[firstOrSecond]]}"
                                            # )
                                        pro3 = (
                                            int(
                                                couple[
                                                    0 if firstOrSecond == 1 else 1
                                                ]
                                            )
                                            * proPro[couple[firstOrSecond]]
                                        )
                                        proPro[num] = pro3
                                        proPro2[num] |= {pro3}
                                        into2 += [i18n.pro["pro "] + str(pro3)]
                                    except KeyError:
                                        pass

            # if self.tables.lastLineNumber >= num:
            try:
                text = cols[206].split("|")[1]
            except (KeyError, TypeError, IndexError):
                text = ""
            if len(text) > 0:
                into += [text]

            into1 = list(OrderedSet(into1))
            into2 = list(OrderedSet(into2))
            into_Str1[num] = (
                i18n.hineinversetzen[" Darin kann sich die "],
                str(num),
                i18n.hineinversetzen[" am Besten hineinversetzen."],
            )
            into_Str2[num] = (
                i18n.hineinversetzen[" Darin kann sich die "],
                str(num),
                i18n.hineinversetzen[" am Besten hineinversetzen."],
            )

            if num != 0:
                if self.tables.htmlOutputYes:
                    into = [
                        "<ul>",
                        "<li>" if len(into1) > 0 else "",
                        ", ".join(into1),
                        "".join(into_Str1[num]) if len(into1) > 0 else "",
                        "</li>" if len(into1) > 0 else "",
                        "<li>" if len(into2) > 0 else "",
                        ", ".join(into2),
                        "".join(into_Str2[num]) if len(into2) > 0 else "",
                        "</li>" if len(into2) > 0 else "",
                        "<li>" if len(into) > 0 else "",
                        ", ".join(into),
                        "</li>" if len(into) > 0 else "",
                        "</ul>",
                    ]
                elif self.tables.bbcodeOutputYes:
                    into = [
                        "[list]",
                        "[*]" if len(into1) > 0 else "",
                        ", ".join(into1),
                        "".join(into_Str1[num]) if len(into1) > 0 else "",
                        "[*]" if len(into2) > 0 else "",
                        ", ".join(into2),
                        "".join(into_Str2[num]) if len(into2) > 0 else "",
                        "[*]" if len(into) > 0 else "",
                        ", ".join(into),
                        "[/list]",
                    ]
                else:
                    into = [
                        ", ".join(into1),
                        "".join(into_Str1[num]) if len(into1) > 0 else "",
                        ", ".join(into2),
                        "".join(into_Str2[num]) if len(into2) > 0 else "",
                        ", ".join(into),
                    ]
                intoB = []
                for intoneu in into:
                    if len(intoneu) > 0:
                        intoB += [intoneu]
            else:
                intoB = into

            self.relitable[num] += (
                [
                    (
                        " | "
                        if not self.tables.htmlOutputYes
                        and not self.tables.bbcodeOutputYes
                        else ""
                    ).join(intoB)
                ]
                if len(into) > 0
                else [""]
            )
        # except (KeyError, TypeError):
        #    self.relitable[num] += ["-"]

        rowsAsNumbers |= {len(self.relitable[0]) - 1, len(self.relitable[0])}
        self.tables.generatedSpaltenParameter_Tags[
            len(rowsAsNumbers) - 1
        ] = frozenset({ST.sternPolygon, ST.universum})
        self.tables.generatedSpaltenParameter_Tags[
            len(rowsAsNumbers) - 2
        ] = frozenset({ST.sternPolygon, ST.universum})

        assert not (
            len(self.tables.generatedSpaltenParameter)
            + self.tables.SpaltenVanillaAmount
            in self.tables.generatedSpaltenParameter
        )

        kette = [
            [(ParametersMain.bedeutung[0], Primzahlkreuz_pro_contra_strs[0])],
            [(ParametersMain.procontra[0], Primzahlkreuz_pro_contra_strs[0])],
            [(ParametersMain.grundstrukturen[0], Primzahlkreuz_pro_contra_strs[1])],
        ]
        self.tables.generatedSpaltenParameter[
            len(self.tables.generatedSpaltenParameter)
            + self.tables.SpaltenVanillaAmount
        ] = kette
        self.tables.generatedSpaltenParameter[
            len(self.tables.generatedSpaltenParameter)
            + self.tables.SpaltenVanillaAmount
        ] = kette

        reverseContra: OrderedDict = OrderedDict()
        for key, value in contraContra2.items():
            for value2 in value:
                try:
                    reverseContra[value2] |= {key}
                except KeyError:
                    reverseContra[value2] = OrderedSet({key})
        reversePro: OrderedDict = OrderedDict()
        for key, value in proPro2.items():
            for value2 in value:
                try:
                    reversePro[value2] |= {key}
                except KeyError:
                    reversePro[value2] = OrderedSet({key})

        pro2: list
        contra2: list
        kette2: list
        for num, cols in enumerate(dreli[: self.tables.lastLineNumber + 1]):
            try:
                pro2 = list(reversePro[num])
            except KeyError:
                pro2 = []
            try:
                contra2 = list(reverseContra[num])
            except KeyError:
                contra2 = []

            if num == 0:
                kette2 = [headline]
            elif contra2 != [] or pro2 != []:
                dahinter1a = (
                    dreli[c][206].split("|")[1]
                    if c <= self.tables.lastLineNumber
                    and len(dreli[c][206].split("|")) == 2
                    and int(dreli[c][206].split("|")[0]) == num
                    else ""
                    for c in pro2
                )
                dahinter1b = []
                for a in dahinter1a:
                    if len(a) > 0:
                        dahinter1b += [a]
                dahinter1: str = " , ".join(dahinter1b)

                dahinter2a = (
                    dreli[c][206].split("|")[1]
                    if c <= self.tables.lastLineNumber
                    and len(dreli[c][206].split("|")) == 2
                    and int(dreli[c][206].split("|")[0]) == num
                    else ""
                    for c in contra2
                )
                dahinter2b = []
                for a in dahinter2a:
                    if len(a) > 0:
                        dahinter2b += [a]
                dahinter2: str = ", ".join(dahinter2b)

                dahinter1len: int = len(dahinter1)
                dahinter2len: int = len(dahinter2)

                kette2 = [
                    "[list]"
                    if self.tables.bbcodeOutputYes
                    else "<ul>"
                    if self.tables.htmlOutputYes
                    else "",
                    (
                        "[*]"
                        if self.tables.bbcodeOutputYes
                        else "<li>"
                        if self.tables.htmlOutputYes
                        else ""
                    )
                    if len(pro2) > 0
                    else "",
                    i18n.proIst["pro dieser Zahl sind: "]
                    if len(pro2) > 1
                    else i18n.proIst["pro dieser Zahl ist "]
                    if len(pro2) == 1
                    else "",
                    str(pro2)[1:-1],
                    ("</li>" if self.tables.htmlOutputYes else "")
                    if len(pro2) > 0
                    else "",
                    (
                        "[*]"
                        if self.tables.bbcodeOutputYes
                        else "<li>"
                        if self.tables.htmlOutputYes
                        else " ("
                    )
                    if dahinter1len > 0
                    else "",
                    dahinter1,
                    (
                        "</li>"
                        if self.tables.htmlOutputYes
                        else ""
                        if self.tables.bbcodeOutputYes
                        else ")"
                    )
                    if dahinter1len > 0
                    else "",
                    (
                        "<li>"
                        if self.tables.htmlOutputYes and len(contra2) > 0
                        else "[*]"
                        if self.tables.bbcodeOutputYes and len(contra2) > 0
                        else " | "
                        if len(pro2) > 0 and len(contra2) > 0
                        else ""
                    ),
                    i18n.contraIst[" contra dieser Zahl sind: "]
                    if len(contra2) > 1
                    else i18n.contraIst[" contra dieser Zahl ist "]
                    if len(contra2) == 1
                    else "",
                    str(contra2)[1:-1],
                    "</li>"
                    if self.tables.htmlOutputYes and len(contra2) > 0
                    else "",
                    (
                        "[*]"
                        if self.tables.bbcodeOutputYes
                        else "<li>"
                        if self.tables.htmlOutputYes
                        else " ("
                    )
                    if dahinter2len > 0
                    else "",
                    dahinter2,
                    (
                        "</li>"
                        if self.tables.htmlOutputYes
                        else ""
                        if self.tables.bbcodeOutputYes
                        else ")"
                    )
                    if dahinter2len > 0
                    else "",
                    "[/list]"
                    if self.tables.bbcodeOutputYes
                    else "</ul>"
                    if self.tables.htmlOutputYes
                    else "",
                    i18n.hineinversetzenSatz,
                ]
            else:
                kette2 = [
                    "-",
                ]

            self.relitable[num] += ["".join(kette2)]

    return self.relitable, rowsAsNumbers


def concat_prim_universe_row(
    concat,
    relitable: list,
    rowsAsNumbers: set,
    generatedBefehle: set,
    htmlTagParaClassWoerter: list,
) -> tuple:
    _ensure_runtime_dependencies()
    """Fügt eine Spalte ein, in der Primzahlen mit Vielfachern
    auf dem Niveau des Universums nicht einfach nur aus einer
    CSV Tabelle geladen werden, sondern durch Primzahlen und
    deren Vielfachern generiert werden.

    @type relitable: list
    @param relitable: Haupttabelle concat.relitable
    @return: relitable + weitere Tabelle daneben
    """
    concat.relitable = relitable
    # alxp("gebrochen")

    hardCodedCouple = (10, 42)
    transzendentalienNrezi = (5, 131)
    if len(generatedBefehle) > 0:
        # concat.tables.primUniverseRowNum = len(concat.relitable[0])
        # concat.tables.generatedSpaltenParameter_Tags[len(rowsAsNumbers)] = frozenset(
        #    {ST.sternPolygon, ST.galaxie}
        # )
        forGeneratedSpaltenParameter_Tags: dict = {
            "primMotivSternGebr": (
                (0, 0, frozenset({ST.sternPolygon, ST.galaxie, ST.gebrRat}), 1),
                (
                    0,
                    1,
                    frozenset(
                        {ST.sternPolygon, ST.galaxie, ST.universum, ST.gebrRat}
                    ),
                    1,
                ),
                (
                    0,
                    2,
                    frozenset(
                        {ST.sternPolygon, ST.galaxie, ST.universum, ST.gebrRat}
                    ),
                    1,
                ),
            ),
            "primStrukSternGebr": (
                (
                    0,
                    1,
                    frozenset(
                        {ST.sternPolygon, ST.galaxie, ST.universum, ST.gebrRat}
                    ),
                    1,
                ),
                (
                    0,
                    2,
                    frozenset(
                        {ST.sternPolygon, ST.galaxie, ST.universum, ST.gebrRat}
                    ),
                    1,
                ),
                (0, 3, frozenset({ST.sternPolygon, ST.universum, ST.gebrRat}), 1),
            ),
            "primMotivGleichfGebr": (
                (
                    1,
                    0,
                    frozenset({ST.gleichfoermigesPolygon, ST.galaxie, ST.gebrRat}),
                    1,
                ),
                (
                    1,
                    1,
                    frozenset(
                        {
                            ST.gleichfoermigesPolygon,
                            ST.galaxie,
                            ST.universum,
                            ST.gebrRat,
                        }
                    ),
                    1,
                ),
                (
                    1,
                    2,
                    frozenset(
                        {
                            ST.gleichfoermigesPolygon,
                            ST.galaxie,
                            ST.universum,
                            ST.gebrRat,
                        }
                    ),
                    1,
                ),
            ),
            "primStrukGleichfGebr": (
                (
                    1,
                    1,
                    frozenset(
                        {
                            ST.gleichfoermigesPolygon,
                            ST.galaxie,
                            ST.universum,
                            ST.gebrRat,
                        }
                    ),
                    1,
                ),
                (
                    1,
                    2,
                    frozenset(
                        {
                            ST.gleichfoermigesPolygon,
                            ST.galaxie,
                            ST.universum,
                            ST.gebrRat,
                        }
                    ),
                    1,
                ),
                (
                    1,
                    3,
                    frozenset(
                        {ST.gleichfoermigesPolygon, ST.universum, ST.gebrRat}
                    ),
                    1,
                ),
            ),
            "primMotivStern": (
                (0, 0, frozenset({ST.sternPolygon, ST.galaxie}), 0),
                (0, 1, frozenset({ST.sternPolygon, ST.galaxie, ST.universum}), 0),
                (0, 2, frozenset({ST.sternPolygon, ST.galaxie, ST.universum}), 0),
            ),
            "primStrukStern": (
                (0, 1, frozenset({ST.sternPolygon, ST.galaxie, ST.universum}), 0),
                (0, 2, frozenset({ST.sternPolygon, ST.galaxie, ST.universum}), 0),
                (0, 3, frozenset({ST.sternPolygon, ST.universum}), 0),
            ),
            "primMotivGleichf": (
                (1, 0, frozenset({ST.gleichfoermigesPolygon, ST.galaxie}), 0),
                (
                    1,
                    1,
                    frozenset(
                        {ST.gleichfoermigesPolygon, ST.galaxie, ST.universum}
                    ),
                    0,
                ),
                (
                    1,
                    2,
                    frozenset(
                        {ST.gleichfoermigesPolygon, ST.galaxie, ST.universum}
                    ),
                    0,
                ),
            ),
            "primStrukGleichf": (
                (
                    1,
                    1,
                    frozenset(
                        {ST.gleichfoermigesPolygon, ST.galaxie, ST.universum}
                    ),
                    0,
                ),
                (
                    1,
                    2,
                    frozenset(
                        {ST.gleichfoermigesPolygon, ST.galaxie, ST.universum}
                    ),
                    0,
                ),
                (1, 3, frozenset({ST.gleichfoermigesPolygon, ST.universum}), 0),
            ),
        }
        uni_ = (5, 131)
        gal_ = (10, 42)
        GalOrUni_nOrInvers = OrderedDict(
            {
                0: (gal_, gal_),
                1: (gal_, uni_),
                2: (uni_, gal_),
                3: (uni_, uni_),
            }
        )

        concat.gebrRatAllCombis = concat.findAllBruecheAndTheirCombinations()
        kombis2: OrderedDict = OrderedDict(
            {"mul": OrderedDict(), "div": OrderedDict()}
        )
        kombis1: OrderedDict = OrderedDict(
            {"stern": deepcopy(kombis2), "gleichf": deepcopy(kombis2)}
        )
        alleFractionErgebnisse2: OrderedDict = OrderedDict(
            {
                "UniUni": deepcopy(kombis1),
                "UniGal": deepcopy(kombis1),
                "GalUni": deepcopy(kombis1),
                "GalGal": deepcopy(kombis1),
            }
        )

        for KeyGalUniUniGal, ValueSternOrGleichf in concat.gebrRatAllCombis.items():
            for KeySternOrGleichf, ValueMulOrDiv in ValueSternOrGleichf.items():
                for KeyMulOrDiv, Couples in ValueMulOrDiv.items():
                    alleFractionErgebnisse2[KeyGalUniUniGal][KeySternOrGleichf][
                        KeyMulOrDiv
                    ] = (
                        concat.combineDicts(
                            concat.convertSetOfPaarenToDictOfNumToPaareMul(
                                Couples,
                                True if KeySternOrGleichf == "gleichf" else False,
                            ),
                            concat.convertFractionsToDictOfNumToPaareOfMulOfIntAndFraction(
                                concat.BruecheUni
                                if KeyGalUniUniGal[:3] == "Uni"
                                else concat.BruecheGal,
                                concat.BruecheUni
                                if KeyGalUniUniGal[3:] == "Uni"
                                else concat.BruecheGal,
                                True if KeySternOrGleichf == "gleichf" else False,
                            ),
                        )
                        if KeyMulOrDiv == "mul"
                        else concat.combineDicts(
                            concat.convertSetOfPaarenToDictOfNumToPaareDiv(
                                Couples,
                                True if KeySternOrGleichf == "gleichf" else False,
                            ),
                            OrderedDict(),
                        )
                    )

        """Wegen pypy3 == python3"""
        for key1, value1 in alleFractionErgebnisse2.items():
            for key2, value2 in value1.items():
                for key3, value3 in value2.items():
                    for key4, value4 in value3.items():
                        alleFractionErgebnisse2[key1][key2][key3][key4] = tuple(
                            unique_everseen(value4, key=frozenset)
                        )
                        # value4 = list(value4)
                        # value4 = sort(value4)
                        # value4.sort()

        # ALXP HIER NOCH NICHT FERTIG

        # hier geht es um die html class Parameter und um Tagging ob Galaxie oder Polygon
        koord2tag: OrderedDict
        koord2ParameterA: OrderedDict
        koord2Parameter: OrderedDict

        koord2tag, koord2ParameterA, koord2Parameter = (
            OrderedDict(),
            OrderedDict(),
            OrderedDict(),
        )

        for name, mehrereEinraege in forGeneratedSpaltenParameter_Tags.items():
            for drei in mehrereEinraege:
                try:
                    koord2tag[(drei[0], drei[1], drei[3])] |= {drei[2]}
                except KeyError:
                    koord2tag[(drei[0], drei[1], drei[3])] = OrderedSet({drei[2]})
            for befehl in generatedBefehle:
                if (
                    name == befehl
                ):  # ob der Befehl des Users mit den jeweils vorhandenen übereinstimmt
                    for drei in mehrereEinraege:
                        try:
                            koord2ParameterA[(drei[0], drei[1], drei[3])] |= {
                                befehl
                            }
                        except KeyError:
                            koord2ParameterA[
                                (drei[0], drei[1], drei[3])
                            ] = OrderedSet({befehl})

        for key, value in koord2tag.items():
            assert len(value) == 1
        for key, value in koord2ParameterA.items():
            koord2Parameter[key] = list(value)

        # stern vs gleichf:
        concat.transzendentalien: dict = OrderedDict(
            {
                _i18n.polygone["Sternpolygone"]: [],
                _i18n.polygone["gleichförmige Polygone"]: [],
            }
        )

        relitableCopy = deepcopy(concat.relitable[: concat.tables.lastLineNumber + 1])
        kombisNamen: tuple = (
            _i18n.kombisNamen["Motiv -> Motiv"],
            _i18n.kombisNamen["Motiv -> Strukur"],
            _i18n.kombisNamen["Struktur -> Motiv"],
            _i18n.kombisNamen["Struktur -> Strukur"],
        )
        kombisNamen2: tuple = (
            "GalGal",
            "GalUni",
            "UniGal",
            "UniUni",
        )

        # concat.rolle = []
        concat.motivation: dict = OrderedDict(
            {
                _i18n.polygone["Sternpolygone"]: [],
                _i18n.polygone["gleichförmige Polygone"]: [],
            }
        )
        # concat.ziel = []
        for zwei, (polytype, polytypename, transzType) in enumerate(
            zip(
                hardCodedCouple,
                [
                    _i18n.polygone["Sternpolygone"],
                    _i18n.polygone["gleichförmige Polygone"],
                ],
                transzendentalienNrezi,
            )
        ):
            for cols in concat.relitable:
                # concat.rolle += [cols[19]]
                concat.transzendentalien[polytypename] += [cols[transzType]]
                concat.motivation[polytypename] += [cols[polytype]]
                # concat.ziel += [cols[11]]

        for brr, ganzOrGebr in enumerate(
            ["", _i18n.faktorenbla[", mit Faktoren aus gebrochen-rationalen Zahlen"]]
        ):
            for zwei, (
                polytype,
                polytypename,
                transzType,
                sternOrGleichf,
            ) in enumerate(
                zip(
                    hardCodedCouple,
                    [
                        _i18n.polygone["Sternpolygone"],
                        _i18n.polygone["gleichförmige Polygone"],
                    ],
                    transzendentalienNrezi,
                    kombis1.keys(),
                )
            ):
                kombi_ = []

                # alle Kombis die von strukur oder motiven also 2x2 möglich sind
                for i, cols in enumerate(concat.relitable):
                    kombi_ += [
                        (
                            (
                                concat.motivation[polytypename][i],
                                concat.motivation[polytypename][i],
                            ),
                            (
                                concat.motivation[polytypename][i],
                                concat.transzendentalien[polytypename][i],
                            ),
                            (
                                concat.transzendentalien[polytypename][i],
                                concat.motivation[polytypename][i],
                            ),
                            (
                                concat.transzendentalien[polytypename][i],
                                concat.transzendentalien[polytypename][i],
                            ),
                        )
                    ]
                kombis: tuple = tuple(kombi_)
                # alle 2x2 kombis von motiven und struktur

                for nullBisDrei, (kombiUeberschrift, GalUniKombis) in enumerate(
                    zip(kombisNamen, kombisNamen2)
                ):
                    tag: frozenset = list(koord2tag[(zwei, nullBisDrei, brr)])[0]

                    concat.tables.generatedSpaltenParameter_Tags[
                        len(rowsAsNumbers)
                    ] = tag
                    rowsAsNumbers |= {
                        len(concat.relitable[0]),
                    }
                    if (zwei, nullBisDrei, brr) in koord2Parameter:
                        for i, cols in enumerate(relitableCopy):
                            if i == 0:
                                into = [
                                    _i18n.genMul["generierte Multiplikationen "],
                                    polytypename,
                                    " ",
                                    kombiUeberschrift,
                                    ganzOrGebr,
                                ]
                            else:
                                into = []
                                if concat.tables.htmlOutputYes:
                                    into += ["<ul>"]
                                elif concat.tables.bbcodeOutputYes:
                                    into += ["[list]"]
                                if brr == 0:
                                    multipless = multiples(i)
                                    multipless.sort()
                                    for k, multi in enumerate(multipless):
                                        if (
                                            k > 0
                                            and not concat.tables.htmlOutputYes
                                            and not concat.tables.bbcodeOutputYes
                                        ):
                                            into += [_i18n.ausserdem[", außerdem: "]]
                                        into += [
                                            "<li>"
                                            if concat.tables.htmlOutputYes
                                            else "[*]"
                                            if concat.tables.bbcodeOutputYes
                                            else "",
                                            "(",
                                            kombis[multi[0]][nullBisDrei][0]
                                            if len(
                                                kombis[multi[0]][nullBisDrei][
                                                    0
                                                ].strip()
                                            )
                                            > 3
                                            else "...",
                                            ") * (",
                                            kombis[multi[1]][nullBisDrei][1]
                                            if len(
                                                kombis[multi[1]][nullBisDrei][
                                                    1
                                                ].strip()
                                            )
                                            > 3
                                            else "...",
                                            ")",
                                            "</li>"
                                            if concat.tables.htmlOutputYes
                                            else "",
                                        ]
                                elif brr == 1:
                                    multipless = alleFractionErgebnisse2[
                                        GalUniKombis
                                    ][sternOrGleichf]["mul"]
                                    for k, multi in enumerate(
                                        zip_longest(
                                            multipless[i],
                                            fillvalue="",
                                        )
                                    ):
                                        multi = multi[0]
                                        try:
                                            multi[0]
                                            multi[1]
                                        except:
                                            continue

                                        von = concat.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
                                            multi[0],
                                            GalOrUni_nOrInvers[nullBisDrei][zwei],
                                            concat.readOneCSVAndReturn(
                                                2 if nullBisDrei in (2, 3) else 3
                                            ),
                                            False
                                            if nullBisDrei in (2, 3)
                                            else True,
                                        )
                                        bis = concat.spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie(
                                            multi[1],
                                            GalOrUni_nOrInvers[nullBisDrei][zwei],
                                            concat.readOneCSVAndReturn(
                                                2 if nullBisDrei in (1, 3) else 3
                                            ),
                                            False
                                            if nullBisDrei in (1, 3)
                                            else True,
                                        )

                                        if von is not None and bis is not None:
                                            von = von.strip()
                                            bis = bis.strip()
                                            if len(von) > 3 and len(bis) > 3:
                                                if (
                                                    k > 0
                                                    and not concat.tables.htmlOutputYes
                                                    and not concat.tables.bbcodeOutputYes
                                                    and len(into) > 0
                                                ):
                                                    into += [
                                                        _i18n.ausserdem[
                                                            "| außerdem: "
                                                        ]
                                                    ]
                                                into += [
                                                    "<li>"
                                                    if concat.tables.htmlOutputYes
                                                    else "[*]"
                                                    if concat.tables.bbcodeOutputYes
                                                    else "" '"',
                                                    '"',
                                                    von,
                                                    '"',
                                                    # concat.CSVsAlreadRead[place][
                                                    #    multi[0].numerator - 1
                                                    # ][multi[0].denominator - 1],
                                                    "<br>"
                                                    if concat.tables.htmlOutputYes
                                                    and (
                                                        len(von) > 30
                                                        or len(bis) > 30
                                                    )
                                                    else " ",
                                                    "(",
                                                    str(multi[0]),
                                                    ")*(",
                                                    str(multi[1]),
                                                    ")",
                                                    "<br>"
                                                    if concat.tables.htmlOutputYes
                                                    and (
                                                        len(von) > 30
                                                        or len(bis) > 30
                                                    )
                                                    else " ",
                                                    '"',
                                                    bis,
                                                    '"',
                                                    # concat.CSVsAlreadRead[place][
                                                    #    multi[1].numerator - 1
                                                    # ][multi[1].denominator - 1],
                                                    '"',
                                                    "</li>"
                                                    if concat.tables.htmlOutputYes
                                                    else "",
                                                ]
                                if concat.tables.htmlOutputYes:
                                    into += ["</ul>"]
                                elif concat.tables.bbcodeOutputYes:
                                    into += ["[/list]"]

                            concat.relitable[i] += ["".join(into)]

                        if (
                            len(concat.tables.generatedSpaltenParameter)
                            + concat.tables.SpaltenVanillaAmount
                            in concat.tables.generatedSpaltenParameter
                        ):
                            raise ValueError
                        kette = (
                            [
                                (
                                    _i18n.Multiplikationen_["Multiplikationen"],
                                    htmlTagParaClassWoerter[para][0][0][0][1],
                                )
                            ]
                            for para in koord2Parameter[(zwei, nullBisDrei, brr)]
                        )
                        if (
                            "primMotivStern"
                            in koord2Parameter[(zwei, nullBisDrei, brr)]
                        ):
                            kette = list(kette) + [
                                [
                                    (
                                        _i18n.nWichtigste[
                                            "Wichtigstes_zum_verstehen"
                                        ],
                                        _i18n.nWichtigste["Viertwichtigste"],
                                    )
                                ]
                            ]

                        concat.tables.generatedSpaltenParameter[
                            len(concat.tables.generatedSpaltenParameter)
                            + concat.tables.SpaltenVanillaAmount
                        ] = tuple(kette)

    return concat.relitable, rowsAsNumbers

def create_spalte_gestirn(tables, relitable: list, rows_as_numbers: set) -> tuple:
    """Generate the Gestirn/Sonne/Mond/Planet column for main tables.

    This is the architecture-owned version of the former
    ``Tables.Maintable.createSpalteGestirn`` method.  It mutates the legacy
    table/list structures exactly like the old method and also returns them for
    newer morphism-style callers.
    """
    from .number_theory import moonNumber as moon_number
    from .runtime_compat import i18n as runtime_i18n
    from .tag_schema import ST as st_enum

    if set(rows_as_numbers) >= {64}:
        if len(relitable) > 0:
            generated_index = len(tables.generatedSpaltenParameter) + tables.SpaltenVanillaAmount
            if generated_index in tables.generatedSpaltenParameter:
                raise ValueError
            tables.generatedSpaltenParameter[generated_index] = tables.dataDict[0][64]
            rows_as_numbers.add(len(relitable[0]))
            tables.generatedSpaltenParameter_Tags[len(rows_as_numbers) - 1] = frozenset(
                {st_enum.sternPolygon, st_enum.universum, st_enum.galaxie}
            )

        relitable[0] += [runtime_i18n.tableHandling.gestirnGrossschrift["Gestirn"]]
        relitable[1] += [
            runtime_i18n.tableHandling.gestirnGrossschrift["Sonne (keine Potenzen)"]
        ]
        for i, line in enumerate(relitable[2:], start=2):
            if moon_number(i)[1] != []:
                line1 = [runtime_i18n.tableHandling.gestirnGrossschrift["Mond (Potenzen)"]]
            else:
                line1 = [
                    runtime_i18n.tableHandling.gestirnGrossschrift[
                        "Sonne (keine Potenzen)"
                    ]
                ]
            if i % 2 == 0:
                line1 += [runtime_i18n.tableHandling.gestirnGrossschrift["Planet (2*n)"]]
            if i % 3 == 0:
                line1 += [
                    runtime_i18n.tableHandling.gestirnGrossschrift[
                        "wäre eine schwarze Sonne (-3*n), wenn ins Negative durch eine Typ 13 verdreht"
                    ]
                ]
            line += [
                runtime_i18n.tableHandling.gestirnGrossschrift[", und außerdem "].join(line1)
            ]
    return relitable, rows_as_numbers


@dataclass(frozen=True)
class GeneratedColumnsBundle:
    """Callable architecture bundle for generated-column morphisms."""

    registry: GeneratedColumnRegistry = DEFAULT_GENERATED_COLUMN_REGISTRY

    def snapshot(self) -> dict:
        data = self.registry.snapshot()
        data["class"] = "GeneratedColumnsBundle"
        return data

    def concat_love_polygon(self, concat, relitable: list, rows_as_numbers: set) -> tuple:
        return concat_love_polygon(concat, relitable, rows_as_numbers)

    def concat_vervielfache_zeile(self, concat, relitable: list, rows_as_numbers: set) -> tuple:
        return concat_vervielfache_zeile(concat, relitable, rows_as_numbers)

    def concat_modallogik(self, concat, relitable: list, concepts_rows_set_of_tuple: set, rows_as_numbers: set) -> tuple:
        return concat_modallogik(concat, relitable, concepts_rows_set_of_tuple, rows_as_numbers)

    def concat_gleichheit_freiheit_dominieren(self, concat, relitable: list, rows_as_numbers: set) -> tuple:
        return concat_gleichheit_freiheit_dominieren(concat, relitable, rows_as_numbers)

    def concat_geist_emotion_energie_materie_topologie(self, concat, relitable: list, rows_as_numbers: set) -> tuple:
        return concat_geist_emotion_energie_materie_topologie(concat, relitable, rows_as_numbers)

    def concat_prim_creativity_type(self, concat, relitable: list, rows_as_numbers: set) -> tuple:
        return concat_prim_creativity_type(concat, relitable, rows_as_numbers)

    def concat_mond_exponzieren_logarithmus_typ(self, concat, relitable: list, rows_as_numbers: set) -> tuple:
        return concat_mond_exponzieren_logarithmus_typ(concat, relitable, rows_as_numbers)

    def concat_primzahlkreuz_pro_contra(self, concat, relitable: list, rows_as_numbers: set, generated_befehle: set, parameters_main) -> tuple:
        return concat_primzahlkreuz_pro_contra(concat, relitable, rows_as_numbers, generated_befehle, parameters_main)

    def concat_prim_universe_row(self, concat, relitable: list, rows_as_numbers: set, generated_befehle: set, html_tag_para_class_woerter: list) -> tuple:
        return concat_prim_universe_row(concat, relitable, rows_as_numbers, generated_befehle, html_tag_para_class_woerter)

    def create_spalte_gestirn(self, tables, relitable: list, rows_as_numbers: set) -> tuple:
        return create_spalte_gestirn(tables, relitable, rows_as_numbers)


def bootstrap_generated_columns() -> GeneratedColumnsBundle:
    return GeneratedColumnsBundle()
