from __future__ import annotations

"""Output syntax morphisms for Reta table renderers.

Stage 24 extracts the concrete renderer syntax classes from the old
``libs.lib4tables`` module. These classes are now owned by the architecture
layer; ``libs.lib4tables`` remains a small compatibility facade for legacy
imports.
"""

from collections import OrderedDict
from dataclasses import dataclass

from .number_theory import primCreativity
from .split_i18n import build_split_i18n_proxy

_i18n_proxy = build_split_i18n_proxy()
i18n = _i18n_proxy.lib4tables

class NichtsSyntax:
    mode_name = "nichts"
    force_one_table = False
    force_zero_width = False
    marks_html_or_bbcode = False
    def coloredBeginCol(self, num: int, rest: bool = False):
        return ""

    def generateCell(
        self, num: int, dataDict: dict, content=None, zeile=None, tables=None
    ) -> str:
        return ""

    beginTable = ""
    endTable = ""
    beginCell = ""
    endCell = ""
    beginZeile = ""
    endZeile = ""


class OutputSyntax:
    mode_name = "shell"
    force_one_table = False
    force_zero_width = False
    marks_html_or_bbcode = False
    def coloredBeginCol(self, num: int, rest: bool = False):
        return self.beginZeile

    def generateCell(
        self, num: int, dataDict: dict, content=None, zeile=None, tables=None
    ) -> str:
        return self.beginCell

    beginTable = ""
    endTable = ""
    beginCell = ""
    endCell = ""
    beginZeile = ""
    endZeile = ""


class csvSyntax(OutputSyntax):
    mode_name = "csv"
    force_one_table = True
    force_zero_width = True
    beginTable = ""
    endTable = ""
    beginCell = ""
    endCell = ""
    beginZeile = ""
    endZeile = ""


class emacsSyntax(OutputSyntax):
    mode_name = "emacs"
    force_one_table = True
    force_zero_width = True
    # print(OutputSyntax)
    # def generateCell(
    #    self, spalte: int, SpaltenParameter: dict, content=None, zeile=None, tables=None
    # ) -> str:
    #    return "|"

    beginTable = ""
    endTable = ""
    beginCell = "|"
    endCell = ""
    beginZeile = ""
    endZeile = "|"


class markdownSyntax(OutputSyntax):
    mode_name = "markdown"
    force_one_table = True
    force_zero_width = True
    # print(OutputSyntax)
    # def generateCell(
    #    self, spalte: int, SpaltenParameter: dict, content=None, zeile=None, tables=None
    # ) -> str:
    #    return "|"

    beginTable = ""
    endTable = ""
    beginCell = "|"
    endCell = ""
    beginZeile = ""
    endZeile = "|"


class bbCodeSyntax(OutputSyntax):
    mode_name = "bbcode"
    marks_html_or_bbcode = True
    def coloredBeginCol(self, num: int, rest: bool = False):
        num = int(num) if str(num).isdecimal() else 0
        numberType = primCreativity(num)

        if rest:
            # wenn der Fallm eintritt dass es leerer Text ist der frei ist
            return "[tr]"
            if num == 0:
                return "[tr]"
            elif num % 2 == 0:
                return "[tr]"
            else:
                return "[tr]"
        elif numberType == 1:
            if num % 2 == 0:
                return '[tr="background-color:#66ff66;color:#000000;"]'
            else:
                return '[tr="background-color:#009900;color:#ffffff;"]'
        elif numberType == 2 or num == 1:
            if num % 2 == 0:
                return '[tr="background-color:#ffff66;color:#000099;"]'
            else:
                return '[tr="background-color:#555500;color:#aaaaff;"]'
        elif numberType == 3:
            if num % 2 == 0:
                return '[tr="background-color:#9999ff;color:#202000;"]'
            else:
                return '[tr="background-color:#000099;color:#ffff66;"]'
        elif num == 0:
            return '[tr="background-color:#ff2222;color:#002222;"]'

    def generateCell(
        self, spalte: int, SpaltenParameter: dict, content=None, zeile=None, tables=None
    ) -> str:
        spalte = int(spalte)
        spalte += 2
        return "".join(
            (
                "[td",
                (
                    '="background-color:#000000;color:#ffffff"'
                    if content is not None and int(content) % 2 == 0
                    else '="background-color:#ffffff;color:#000000"'
                )
                if spalte == 0
                else '=""',
                "]",
            )
        )

    beginTable = "[table]"
    endTable = "[/table]"
    beginCell = "[td]"
    endCell = "[/td]"
    beginZeile = "[tr]"
    endZeile = "[/tr]"


class htmlSyntax(OutputSyntax):
    mode_name = "html"
    marks_html_or_bbcode = True
    def __init__(self):
        self.zeile = 0

    def coloredBeginCol(self, num: int, rest: bool = False) -> str:
        num = int(num) if str(num).isdecimal() else 0
        numberType = primCreativity(num)
        self.zeile = num
        if rest:
            # wenn der Fallm eintritt dass es leerer Text ist der frei ist
            return "<tr>\n"
            if num == 0:
                return "<tr>\n"
            elif num % 2 == 0:
                return "<tr>\n"
            else:
                return "<tr>\n"
        elif numberType == 1:
            if num % 2 == 0:
                return '<tr style="background-color:#66ff66;color:#000000;">\n'
            else:
                return '<tr style="background-color:#009900;color:#ffffff;">\n'
        elif numberType == 2 or num == 1:
            if num % 2 == 0:
                return '<tr style="background-color:#ffff66;color:#000099;">\n'
            else:
                return '<tr style="background-color:#555500;color:#aaaaff;">\n'
        elif numberType == 3:
            if num % 2 == 0:
                return '<tr style="background-color:#9999ff;color:#202000;">\n'
            else:
                return '<tr style="background-color:#000099;color:#ffff66;">\n'
        elif num == 0:
            return '<tr style="background-color:#ff2222;color:#002222;">\n'

    def generateCell(
        self, spalte: int, SpaltenParameter: dict, content=None, zeile=None, tables=None
    ) -> str:
        if zeile == "":
            zeile = 0
        spalte = int(spalte)
        if spalte == -2:
            tupleOfListsOfCouples = (
                [
                    (i18n.zaehlung["Zählung"], ""),
                ],
            )
        elif spalte == -1:
            tupleOfListsOfCouples = (
                [
                    (i18n.nummerier["Nummerierung"], ""),
                ],
            )
        else:
            try:
                # print(SpaltenParameter[spalte])
                tupleOfListsOfCouples = SpaltenParameter[spalte]
            except:
                if str(spalte).isdecimal():
                    raise ValueError
                tupleOfListsOfCouples = (("?", "?"),)
        # damit pypy3 == python3
        things1: OrderedDict[int, list] = OrderedDict()

        listOfListsOfCouples: list = list(tupleOfListsOfCouples)
        # listOfListsOfCouples.sort(
        #    key=lambda x: sorted([(s.upper(), v) for s, v in x])
        # )  # damit pypy3 == python3
        # print(listOfListsOfCouples)
        for c, couples in enumerate(listOfListsOfCouples):
            for paraNum in (0, 1):
                if len(couples[0]) > paraNum:
                    if len(couples[0]) > paraNum:
                        # i = 0
                        para1o2name = couples[0][paraNum]
                        # while (
                        #    len(couples) > i + 1
                        #    and len(couples[i + 1]) > 0
                        #    and para1o2name.strip() == ""
                        # ):
                        #    i += 1
                        #    para1o2name = couples[i][paraNum]
                        if len(para1o2name.strip()) != 0 or True:
                            if paraNum == 1:
                                para1o2name = "".join(["p3_", str(c), "_", para1o2name])
                            try:
                                things1[paraNum] += [para1o2name]
                                # things1[paraNum].sort(key=lambda x: x.upper())

                            except KeyError:
                                things1[paraNum]: list = [
                                    para1o2name,
                                ]

        things: OrderedDict = OrderedDict()  # damit pypy3 == python3

        for key, values in things1.items():
            for i, el in enumerate(values):
                if el != i18n.alles["alles"]:
                    try:
                        things[key] += (
                            "✗" if key == 0 else "",
                            el,
                            ",",
                        )
                    except KeyError:
                        things[key] = (
                            "✗" if key == 0 else "",
                            el,
                            ",",
                        )
            things[key] = "".join(things[key])

        spalte += 2
        if len(things) < 2:
            return ""
        else:
            p4: str
            try:
                p4a = tables.generatedSpaltenParameter_Tags[spalte - 2]
                # print(str(p4a))
                p4b: list = []
                for a in p4a:
                    p4b += [str(a.value)]
                # p4b.sort(key=lambda x: x.upper())
                p4 = ",".join(p4b)
            except KeyError:
                p4 = ""
            except:
                p4 = ""
            return "".join(
                ("<td",)
                + (
                    (
                        ' class="',
                        "z_",
                        str(int(zeile)),
                        " r_",
                        str(spalte),
                        " p1_",
                        things[0],
                        ",",
                        " p2_",
                        (things[1] if len(things) > 1 else ""),
                        " p4_",
                        p4,
                        '"',
                    )
                    if zeile == 0
                    else ()
                )
                + (
                    (
                        ' style="background-color:#000000;color:#ffffff;"'
                        if content is not None and int(content) % 2 == 0
                        else ' style="background-color:#ffffff;color:#000000;"'
                        if spalte == 0
                        else "",
                    )
                    if spalte in (0, 1)
                    # else (' style="display:none"',)
                    # if zeile == 0
                    else (
                        ' class="tdSymbole" style="background-image: url('
                        ');background-size: cover;background-repeat: no-repeat;background-position: right; "',
                    )
                    if "Symbole" in things1[0]
                    else ()
                )
                + (">\n",)
            )

    beginTable = '<table border=0 id="bigtable">'
    endTable = "</table>\n"
    beginCell = "<td>\n"
    endCell = "\n</td>\n"
    # beginZeile = "<tr>"
    beginZeile = ""
    endZeile = "</tr>\n"


OUTPUT_SYNTAX_CLASSES = {
    "shell": OutputSyntax,
    "nichts": NichtsSyntax,
    "csv": csvSyntax,
    "bbcode": bbCodeSyntax,
    "html": htmlSyntax,
    "emacs": emacsSyntax,
    "markdown": markdownSyntax,
}


@dataclass(frozen=True)
class OutputSyntaxBundle:
    classes: dict[str, type]

    def snapshot(self) -> dict:
        return {
            "class": "OutputSyntaxBundle",
            "modes": {
                name: {
                    "class": cls.__name__,
                    "force_one_table": bool(getattr(cls, "force_one_table", False)),
                    "force_zero_width": bool(getattr(cls, "force_zero_width", False)),
                    "marks_html_or_bbcode": bool(getattr(cls, "marks_html_or_bbcode", False)),
                }
                for name, cls in sorted(self.classes.items())
            },
            "legacy_owner": "libs.lib4tables",
            "architecture_owner": "reta_architecture.output_syntax",
        }

    def class_for(self, mode: str) -> type:
        return self.classes[mode]


def bootstrap_output_syntax() -> OutputSyntaxBundle:
    return OutputSyntaxBundle(classes=dict(OUTPUT_SYNTAX_CLASSES))


def output_syntax_snapshot() -> dict:
    return bootstrap_output_syntax().snapshot()


__all__ = [
    "NichtsSyntax",
    "OutputSyntax",
    "csvSyntax",
    "emacsSyntax",
    "markdownSyntax",
    "bbCodeSyntax",
    "htmlSyntax",
    "OUTPUT_SYNTAX_CLASSES",
    "OutputSyntaxBundle",
    "bootstrap_output_syntax",
    "output_syntax_snapshot",
]
