from __future__ import annotations

import os
import re
import subprocess
import sys
from collections import OrderedDict
from copy import copy, deepcopy
from dataclasses import dataclass
from fractions import Fraction

from .runtime_compat import (
    BereichToNumbers2,
    cliout,
    i18n,
    invert_dict_B,
    isZeilenAngabe,
    kpattern,
    moduloA,
    primfaktoren,
    primRepeat,
    retaPromptHilfe,
    teiler,
    textHatZiffer,
    x,
)
from multis import mult2
from multis3 import mult3

from .prompt_language import custom_split2, isReTaParameter, verifyBruchNganzZahlBetweenCommas
from .prompt_session import PromptTextState

i18nRP = i18n.retaPrompt
retaProgram = None
TXT = PromptTextState
gebrochenErlaubteZahlen = set()
wahl15 = {}
wahl16 = {}
befehle = []


def configure_prompt_execution(*, prompt_runtime=None, prompt_language=None, completion_runtime=None) -> None:
    global retaProgram, gebrochenErlaubteZahlen, wahl15, wahl16, befehle
    if prompt_runtime is not None:
        retaProgram = prompt_runtime.program
    if prompt_language is not None:
        gebrochenErlaubteZahlen = set(prompt_language.gebrochen_erlaubte_zahlen)
        wahl15 = dict(prompt_language.wahl15)
        wahl16 = dict(prompt_language.wahl16)
        if "15" in wahl15:
            wahl15[""] = wahl15["15"]
        if "16" in wahl16:
            wahl16[""] = wahl16["16"]
    if completion_runtime is not None:
        befehle = list(completion_runtime.befehle)


@dataclass(frozen=True)
class PromptExecutionBundle:
    """Explicit execution layer for Reta prompt commands.

    The functions below are still intentionally legacy-compatible: they preserve
    the old names and side effects, while moving the deep prompt-command
    execution block out of ``retaPrompt.py`` into an architecture-owned module.
    """

    command_runner: object
    fraction_manager: object
    reta_executor: object

    def snapshot(self):
        return {
            "class": type(self).__name__,
            "command_runner": getattr(self.command_runner, "__name__", str(self.command_runner)),
            "fraction_manager": getattr(self.fraction_manager, "__name__", str(self.fraction_manager)),
            "reta_executor": getattr(self.reta_executor, "__name__", str(self.reta_executor)),
            "i18n_prompt": type(i18nRP).__name__,
        }

    def run_grosse_ausgabe(self, *args, **kwargs):
        return self.command_runner(*args, **kwargs)


def bootstrap_prompt_execution(*, architecture=None, i18n=None, prompt_runtime=None, prompt_language=None, completion_runtime=None, force_rebuild: bool = False) -> PromptExecutionBundle:
    if prompt_runtime is None:
        if architecture is not None and hasattr(architecture, "bootstrap_prompt_runtime"):
            prompt_runtime = architecture.bootstrap_prompt_runtime(i18n=i18n, force_rebuild=force_rebuild)
        else:
            from pathlib import Path
            from .prompt_runtime import bootstrap_prompt_runtime

            prompt_runtime = bootstrap_prompt_runtime(repo_root=Path(__file__).resolve().parent.parent, i18n=i18n, force_rebuild=force_rebuild)
    if prompt_language is None:
        if architecture is not None and hasattr(architecture, "bootstrap_prompt_language"):
            prompt_language = architecture.bootstrap_prompt_language(i18n=i18n, force_rebuild=force_rebuild)
        else:
            from pathlib import Path
            from .prompt_language import bootstrap_prompt_language

            prompt_language = bootstrap_prompt_language(repo_root=Path(__file__).resolve().parent.parent, i18n=i18n, force_rebuild=force_rebuild)
    if completion_runtime is None:
        if architecture is not None and hasattr(architecture, "bootstrap_completion_runtime"):
            completion_runtime = architecture.bootstrap_completion_runtime(i18n=i18n, force_rebuild=force_rebuild)
        else:
            from pathlib import Path
            from .completion_runtime import bootstrap_completion_runtime

            completion_runtime = bootstrap_completion_runtime(repo_root=Path(__file__).resolve().parent.parent, i18n=i18n, force_rebuild=force_rebuild)
    configure_prompt_execution(prompt_runtime=prompt_runtime, prompt_language=prompt_language, completion_runtime=completion_runtime)
    return PromptExecutionBundle(
        command_runner=PromptGrosseAusgabe,
        fraction_manager=bruchBereichsManagementAndWbefehl,
        reta_executor=retaExecuteNprint,
    )


def anotherOberesMaximum(zahlenBereichC, maxNum, Txt):
    maximizing = list(BereichToNumbers2(zahlenBereichC, False, 0))
    if len(maximizing) > 0:
        maximizing.sort()
        maxNum2 = maximizing[-1]
    else:
        maxNum2 = maxNum
    try:
        max1024 = Txt.programm.tables.hoechsteZeile[1024]
    except Exception:
        max1024 = retaProgram.tables.hoechsteZeile[1024]
    return (
        "--"
        + i18n.zeilenParas["oberesmaximum"]
        + "="
        + str(max(maxNum, maxNum2, max1024) + 1)
    )




def returnOnlyParasAsList(textList: str):
    liste = []
    for t in textList:
        if isReTaParameter(t):
            liste += [t]
    return liste




def grKl(A: set, B: set) -> tuple:
    """
    Gibt 2 Mengen zurück: eine Menge aus allem, das größer ist als im ersten Parameter aus dem zweiten Parameter
    und in die zweite Menge kommt alles, das kleiner ist, als in der ersten Menge aus der zweiten Menge
    """
    C = set()
    D = set()
    if len(B) == 0:
        return A, A
    for a in A:
        if a > max(B):
            C.add(a)
        elif a < min(B):
            D.add(a)
    return C, D




def getDictLimtedByKeyList(d: dict, keys) -> dict:
    """
    Gibt ein dict zurück, das aus einem dict gebildet wird, aber davon nur das nimmt, was an mehreren keys genommen werden soll.
    """
    return OrderedDict({k: d[k] for k in keys if k in d})




def bruchSpalt(text) -> list:
    """
    Gibt eine Liste aus Tupeln zurück, die entweder einen bis mehrere oder zwei Werte enthalten.
    Eingabe sind Brüche gemischt mit Textwerten
    Das Ergebnis bei zwei Werten ist der Bruch
    Bei ein bis mehreren Werten, also auch 2 handelt es sich um die Textwerte, welche zwischen den Brüchen waren.
    Die Reihenfolge vom Ergebnis ist die Gleiche, wie bei dem Eingabe-Text
    """
    if type(text) is not str:
        return []
    bruchSpalten: list = text.split("/")
    bruchSpaltenNeu = []
    bruchSpaltenNeu2 = []
    if len(bruchSpalten) < 2:
        """Ein Bruch hat immer mindestens 2 Zahlen"""
        return []
    keineZahl = OrderedDict()
    for k, bS in enumerate(bruchSpalten):
        keineZahlBefore = keineZahl
        zahl, keineZahl, bsNeu = OrderedDict(), OrderedDict(), []
        countChar = 0
        countNumber = 0
        wasNumber = False
        goNext = 0
        for char in bS:
            if char.isdecimal():
                """alles was Zahlen sind"""
                if not wasNumber:
                    goNext += 1
                try:
                    zahl[goNext] += char
                except KeyError:
                    zahl[goNext] = char
                wasNumber = True
                countNumber += 1
                countChar = 0
            else:
                """alles was keine Zahlen sind"""
                if wasNumber:
                    goNext += 1
                try:
                    keineZahl[goNext] += char
                except KeyError:
                    keineZahl[goNext] = char
                wasNumber = False
                countChar += 1
                countNumber = 0
        flag: bool = False
        allVergleich: list[bool] = [
            zahl2 > zahl1 for zahl1, zahl2 in zip(keineZahl.keys(), zahl.keys())
        ]
        """bool Liste wann es keine ist und wann eine zahl im string"""
        zahlSet: set = set(zahl.keys())
        keineZahlSet: set = set(keineZahl.keys())
        if len(zahlSet) == 0:
            return []
        anfang, ende = k == 0, k == len(bruchSpalten) - 1
        if anfang and all(allVergleich):
            flag = True
        elif ende and not any(allVergleich):
            flag = True
        elif (
            not anfang
            and not ende
            and keineZahlSet.issubset(range(min(zahlSet) + 1, max(zahlSet)))
        ):
            flag = True
        else:
            flag = False
        if flag is False:
            return []
        # bsAlt = bsNeu
        if len(keineZahlSet) > 0:
            zahlenGroesserSet, zahlenKleinerSet = grKl(zahlSet, keineZahlSet)
            """siehe erklärung der Fkt in Fkt"""
            zahlenKleinerDict: dict = getDictLimtedByKeyList(zahl, zahlenKleinerSet)
            zahlenGroesserDict: dict = getDictLimtedByKeyList(zahl, zahlenGroesserSet)
            """siehe erklärung der Fkt in Fkt"""
            if k == len(bruchSpalten) - 1 and len(zahlenGroesserDict) > 0:
                return []
            bsNeu = [zahlenKleinerDict, keineZahl, zahlenGroesserDict]
        elif k == 0 or k == len(bruchSpalten) - 1:
            bsNeu = [zahl]
        else:
            return []
        bruchSpaltenNeu += [bsNeu]
        if k == 1:
            vorZahl1 = (
                () if len(bruchSpaltenNeu[0]) == 1 else bruchSpaltenNeu[0][1].values()
            )
            vorZahl1 = tuple(vorZahl1)
            zahl1 = (
                bruchSpaltenNeu[0][0].values()
                if len(bruchSpaltenNeu[0]) == 1
                else bruchSpaltenNeu[0][2].values()
            )
            zahl2 = bruchSpaltenNeu[1][0].values()
            zahl1 = tuple(zahl1)
            zahl2 = tuple(zahl2)
            if k == len(bruchSpalten) - 1:
                nachZahl2 = (
                    ()
                    if len(bruchSpaltenNeu[-1]) == 1
                    else bruchSpaltenNeu[-1][1].values()
                )
                nachZahl2 = tuple(nachZahl2)
                bruchSpaltenNeu2 += [vorZahl1, zahl1 + zahl2, nachZahl2]
            else:
                bruchSpaltenNeu2 += [vorZahl1, zahl1 + zahl2]
        elif k == len(bruchSpalten) - 1 and k > 1:
            vorZahl1 = (
                () if len(bruchSpaltenNeu[-2]) == 1 else bruchSpaltenNeu[-2][1].values()
            )
            vorZahl1 = tuple(vorZahl1)
            zahl1 = (
                bruchSpaltenNeu[-2][0].values()
                if len(bruchSpaltenNeu[-2]) == 1
                else bruchSpaltenNeu[-2][2].values()
            )
            zahl2 = bruchSpaltenNeu[-1][0].values()
            zahl1 = tuple(zahl1)
            zahl2 = tuple(zahl2)
            nachZahl2 = (
                () if len(bruchSpaltenNeu[-1]) == 1 else bruchSpaltenNeu[-1][1].values()
            )
            nachZahl2 = tuple(nachZahl2)
            bruchSpaltenNeu2 += [vorZahl1, zahl1 + zahl2, nachZahl2]
        elif k > 1:
            vorZahl1 = (
                () if len(bruchSpaltenNeu[-2]) == 1 else bruchSpaltenNeu[-2][1].values()
            )
            vorZahl1 = tuple(vorZahl1)
            zahl1 = (
                bruchSpaltenNeu[-2][0].values()
                if len(bruchSpaltenNeu[-2]) == 1
                else bruchSpaltenNeu[-2][2].values()
            )
            zahl2 = bruchSpaltenNeu[-1][0].values()
            zahl1 = tuple(zahl1)
            zahl2 = tuple(zahl2)
            bruchSpaltenNeu2 += [vorZahl1, zahl1 + zahl2]
            # return bruchSpaltenNeu, bruchSpaltenNeu2
    return bruchSpaltenNeu2




def dictToList(dict_: dict) -> list:
    liste = []
    for key, value in dict_.items():
        liste += [value]
    return liste




def createRangesForBruchLists(bruchList: list) -> tuple:
    n1, n2 = [], []
    listenRange: range = range(0)
    listenRangeUrsprung: range = range(0)
    flag = 0
    # ergebnis: list[tuple[range | str]] = []
    ergebnis = []
    if (
        len(bruchList) == 3
        and len(bruchList[0]) == 0
        and len(bruchList[1]) == 2
        and len(bruchList[2]) == 0
        and (bruchList[1][0] + bruchList[1][1]).isdecimal()
    ):
        return [int(bruchList[1][0])], bruchList[1][1]
    for i, b in enumerate(bruchList):
        if flag == -1:
            return []
        if flag > 3:
            """illegal"""
            return []
        elif flag == 3:
            """Es war ein Bruch"""
            ergebnis += [str(n2[-2]), "-", str(n2[-1])]

            listenRange = range(int(n1[-2]), int(n1[-1]) + 1)
            listenRangeUrsprung = listenRange
            flag = -1
        if len(b) == 2 and (b[0] + b[1]).isdecimal():
            """Es ist ein Bruch"""
            if (
                len(bruchList) >= i
                and len(bruchList[i + 1]) == 1
                and bruchList[i + 1][0] == "-"
                and flag == 0
            ) or (
                i > 0
                and len(bruchList[i - 1]) == 1
                and bruchList[i - 1][0] == "-"
                and flag == 2
            ):
                n1 += [int(b[0])]
                n2 += [int(b[1])]
                flag += 1
            else:
                ergebnis += [b[1]]
                if (
                    len(listenRange) > 0
                    and i > 0
                    and len(bruchList[i - 1]) == 1
                    and bruchList[i - 1][0] == "+"
                ):
                    listenRange2 = []
                    for lr in listenRangeUrsprung:
                        listenRange2 += [lr + int(b[0]), lr - int(b[0])]
                    listenRange = listenRange2
                elif len(listenRange) == 0:
                    listenRange = [int(b[0])]
                    listenRangeUrsprung = listenRange
        elif len(b) == 1 and b[0] == "-" and flag > 0:
            flag += 1

        else:
            """Es ist kein Bruch"""
            flag = 0
            ergebnis += [*b]
    ergebnis2 = "".join(ergebnis)
    return listenRange, ergebnis2




def vorherVonAusschnittOderZaehlung(Txt: TXT, bereichsAngabe: str) -> str:
    if Txt.hasWithoutABC({i18n.befehle2["range"], i18n.befehle2["R"]}):
        return "".join(("--", i18n.zeilenParas["zaehlung"], "=", bereichsAngabe))
    else:
        return "".join(
            (("--", i18n.zeilenParas["vorhervonausschnitt"], "=", bereichsAngabe))
        )




def PromptGrosseAusgabe(
    IsPureOnlyReTaCmd,
    befehleBeenden,
    brueche,
    zahlenBereichC,
    ketten,
    loggingSwitch,
    maxNum,
    cmd_gave_output,
    zahlenAngaben_,
    ifKurzKurz,
    nurEinBefehl,
    Txt,
):
    # global alxp, cliout, i18n, invert_dict_B, isZeilenAngabe, isZeilenAngabe_betweenKommas, isZeilenBruchAngabe, moduloA, primfaktoren, primRepeat, retaPromptHilfe, teiler, textHatZiffer, x
    global i18nRP, sprachenWahl

    (
        EsGabzahlenAngaben,
        zahlenReiheKeineWteiler,
        bruch_GanzZahlReziproke,
        fullBlockIsZahlenbereichAndBruch,
        rangesBruecheDict,
        rangesBruecheDictReverse,
    ) = (False, "", [], False, {}, {})
    if not IsPureOnlyReTaCmd:
        (
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            zahlenReiheKeineWteiler,
            fullBlockIsZahlenbereichAndBruch,
            rangesBruecheDict,
            EsGabzahlenAngaben,
            rangesBruecheDictReverse,
            Txt.liste,
        ) = bruchBereichsManagementAndWbefehl(zahlenBereichC, Txt.liste, zahlenAngaben_)
    if Txt.hasWithoutABC({i18n.befehle2["mulpri"], i18n.befehle2["p"]}):
        Txt.liste += [
            i18n.befehle2["multis"],
            i18n.befehle2["prim"],
            i18n.befehle2["primfaktorenvergleich"],
        ]

    if ifPrintCmdAgain(Txt):
        if "--" + i18n.ausgabeParas["nocolor"] in Txt.listeE:
            print("[code]" + Txt.text + "[/code]")
        else:
            cliout("[code]" + Txt.text + "[/code]", True, i18n.ausgabeArt["bbcode"])
    if (
        ifKurzKurz
        and i18n.befehle2["keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"]
        not in Txt.listeE
    ):
        if ifPrintCmdAgain(Txt):
            if "--" + i18n.ausgabeParas["nocolor"] in Txt.listeE:
                print(
                    i18nRP.promptModeSatz2.format(
                        "[code]", " ".join(Txt.listeE), "[/code]", Txt.text
                    )
                )
            else:
                cliout(
                    i18nRP.promptModeSatz2.format(
                        "[code]", " ".join(Txt.listeE), "[/code]", Txt.text
                    ),
                    True,
                    "",
                )
        else:
            if "--" + i18n.ausgabeParas["nocolor"] in Txt.listeE:
                print(
                    i18nRP.promptModeSatz2.format(
                        "'", " ".join(Txt.listeE), "'", Txt.text
                    )
                )
            else:
                cliout(
                    i18nRP.promptModeSatz2.format(
                        "'", " ".join(Txt.listeE), "'", Txt.text
                    ),
                    True,
                    "",
                )
    if Txt.has({i18n.befehle2["abcd"], i18n.befehle2["abc"]}):
        buchstabe: str
        # befehlskette = list(
        #    set(Txt.text.split()) - {i18n.befehle2["multis"], i18n.befehle2["prim"]}
        # )
        befehlskette = Txt.text.split()
        if (
            len(befehlskette)
            == 2
            # and len(befehlskette[0]) > 1
            # and len(befehlskette[1]) > 1
        ):
            cmd_gave_output = True
            if True or befehlskette[1] not in i18nRP.replacements.values():
                if (
                    befehlskette[0] == i18n.befehle2["abc"]
                    or befehlskette[0] == i18n.befehle2["abcd"]
                ):
                    buchstaben = befehlskette[1]
                else:
                    buchstaben = befehlskette[0]
            else:
                if (
                    befehlskette[0] == i18n.befehle2["abc"]
                    or befehlskette[0] == i18n.befehle2["abcd"]
                ):
                    buchstaben = {
                        value: key for key, value in i18nRP.replacements.items()
                    }[befehlskette[1]]
                else:
                    buchstaben = {
                        value: key for key, value in i18nRP.replacements.items()
                    }[befehlskette[0]]
            print(
                str(
                    " ".join(
                        [
                            "".join(str(ord(buchstabe.lower()) - 96))
                            for buchstabe in buchstaben
                        ]
                    )
                )
            )
    if Txt.hasWithoutABC({i18n.befehle2["kurzbefehle"]}):
        cmd_gave_output = True
        print(
            "{}: {}\n{}".format(
                i18nRP.befehleWort["Kurzbefehle"],
                " ".join([b for b in befehle if len(b) == 1]),
                str(i18nRP.replacements),
            )
        )

    if Txt.hasWithoutABC({i18n.befehle2["befehle"]}):
        cmd_gave_output = True
        print("{}: {}".format(i18nRP.befehleWort["Befehle"], str(befehle)[1:-1]))
    if Txt.hasWithoutABC(
        {i18n.befehle2["h"], i18n.befehle2["help"], i18n.befehle2["hilfe"]}
    ):
        cmd_gave_output = True
        retaPromptHilfe()
    bedingungZahl, bedingungBrueche = (
        EsGabzahlenAngaben,
        (len(bruch_GanzZahlReziproke) > 0 or len(rangesBruecheDict) > 0)
        or len(rangesBruecheDictReverse) > 0,
    )
    if IsPureOnlyReTaCmd:
        cmd_gave_output = True
        import reta

        if (
            i18n.befehle2["keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"]
            not in Txt.listeE
            and not ifKurzKurz
        ):
            if not ifPrintCmdAgain(Txt):
                # weil sonst das doppelt gemacht wird
                cliout(" ".join(Txt.liste), True, "")

        Txt.liste2 = " ".join(Txt.liste)
        Txt.liste3 = Txt.liste2.split(" -")
        Txt.liste4 = Txt.liste3[:1] + ["-" + a for a in Txt.liste3[1:]]
        Txt.programm = reta.Program(Txt.liste4, Txt=Txt)

    zeiln1, zeiln2, zeiln3, zeiln4 = zeiln1234create(
        Txt,
        bedingungZahl,
        bruch_GanzZahlReziproke,
        zahlenBereichC,
        maxNum,
        zahlenReiheKeineWteiler,
    )

    if bedingungZahl:
        if Txt.hasWithoutABC({i18n.befehle2["thomas"], i18n.befehle2["t"]}):
            cmd_gave_output = True
            retaExecuteNprint(
                ketten,
                Txt.listeE,
                zeiln1,
                zeiln2,
                [
                    "".join(
                        ("--", i18n.ParametersMain.galaxie[0], "=", i18n.thomasWort)
                    ),
                ],
                "2",
                Txt,
            )

    if (
        False
        and {"english", "englisch"} & Txt.menge != set()
        and sys.argv[0].split(os.sep)[-1] == "rpl"
    ):
        cmd_gave_output = True
        sprachenWahl = "english"
        print("set to english")
        return loggingSwitch

    if fullBlockIsZahlenbereichAndBruch and (bedingungZahl or bedingungBrueche):
        if Txt.hasWithoutABC({i18n.befehle2["leeren"]}):
            for _ in range(os.get_terminal_size().lines + 1):
                print()
            cmd_gave_output = True

        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC({i18n.befehle2["emotion"], i18n.befehle2["E"]}),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.grundstrukturen[0],
                        "=",
                        i18n.emotionWort,
                    )
                )
            ],
            None,
            ("2,3", "4,5"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        if was_n_1proN_cmd:
            nennerZaehlerGleich = []
            if len(rangesBruecheDict) > 0:
                cmd_gave_output = True
                for nenner, zaehler in rangesBruecheDict.items():
                    hierBereich = ",".join(zaehler)
                    retaExecuteNprint(
                        ketten,
                        Txt.listeE,
                        vorherVonAusschnittOderZaehlung(Txt, hierBereich),
                        "",
                        [
                            "".join(
                                (
                                    "--",
                                    i18n.gebrochenUniGal["gebrochenemotion"][0],
                                    "=",
                                    str(nenner),
                                )
                            )
                        ],
                        "2",
                        Txt,
                    )
                    nennerZaehlerGleich += findEqualNennerZaehler(
                        hierBereich, nenner, nennerZaehlerGleich
                    )

            elif len(rangesBruecheDictReverse) > 0:
                cmd_gave_output = True
                for nenner, zaehler in rangesBruecheDictReverse.items():
                    hierBereich = ",".join(zaehler)
                    retaExecuteNprint(
                        ketten,
                        Txt.listeE,
                        vorherVonAusschnittOderZaehlung(Txt, hierBereich),
                        "",
                        [
                            "".join(
                                (
                                    "--",
                                    i18n.gebrochenUniGal["gebrochenemotion"][0],
                                    "=",
                                    str(nenner),
                                )
                            )
                        ],
                        "1",
                        Txt,
                    )
                    nennerZaehlerGleich += findEqualNennerZaehler(
                        hierBereich, nenner, nennerZaehlerGleich
                    )

        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC({i18n.befehle2["W"], i18n.befehle2["wirklichkeit"]}),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.grundstrukturen[0],
                        "=",
                        wahl15["10"],
                    )
                )
            ],
            None,
            ("1,2", "5"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )

        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC({i18n.befehle2["T"], i18n.befehle2["triebe"]}),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.grundstrukturen[0],
                        "=",
                        wahl15["6"],
                    )
                )
            ],
            None,
            ("1", "2"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC({i18n.befehle2["I"], i18n.befehle2["impulse"]}),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.grundstrukturen[0],
                        "=",
                        wahl15["5"],
                    )
                )
            ],
            None,
            ("1,4", "3"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC({i18n.befehle2["B"], i18n.befehle2["bewusstsein"]}),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.grundstrukturen[0],
                        "=",
                        wahl15["15"],
                    )
                )
            ],
            None,
            ("6", "7"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC({i18n.befehle2["geist"], i18n.befehle2["G"]}),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.grundstrukturen[0],
                        "=",
                        i18n.geistWort,
                    )
                )
            ],
            None,
            ("3", "4"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC({i18n.befehle2["freiheit"], i18n.befehle2["gleichheit"]}),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.planet[0],
                        "=",
                        i18n.befehle2["freiheit"],
                    )
                )
            ],
            None,
            ("1-4,8", "5-7"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC(
                {
                    i18n.befehle2["groesse"],
                }
            ),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.strukturgroesse[0],
                        "=",
                        i18n.organisationWort,
                    )
                )
            ],
            None,
            ("1-3", "99"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC(
                {
                    i18n.befehle2["groesse"],
                }
            ),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.strukturgroesse[0],
                        "=",
                        i18n.ParametersMain.strukturgroesse[0],
                    )
                )
            ],
            None,
            ("1,2", "4"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        if was_n_1proN_cmd:
            nennerZaehlerGleich = []
            if len(rangesBruecheDict) > 0:
                cmd_gave_output = True
                for nenner, zaehler in rangesBruecheDict.items():
                    hierBereich = ",".join(zaehler)
                    retaExecuteNprint(
                        ketten,
                        Txt.listeE,
                        vorherVonAusschnittOderZaehlung(Txt, hierBereich),
                        "",
                        [
                            "".join(
                                (
                                    "--",
                                    i18n.gebrochenUniGal["gebrochengroesse"][0],
                                    "=",
                                    str(nenner),
                                )
                            )
                        ],
                        "2",
                        Txt,
                    )
                    nennerZaehlerGleich += findEqualNennerZaehler(
                        hierBereich, nenner, nennerZaehlerGleich
                    )

            elif len(rangesBruecheDictReverse) > 0:
                cmd_gave_output = True
                for nenner, zaehler in rangesBruecheDictReverse.items():
                    hierBereich = ",".join(zaehler)
                    retaExecuteNprint(
                        ketten,
                        Txt.listeE,
                        vorherVonAusschnittOderZaehlung(Txt, hierBereich),
                        "",
                        [
                            "".join(
                                (
                                    "--",
                                    i18n.gebrochenUniGal["gebrochengroesse"][0],
                                    "=",
                                    str(nenner),
                                )
                            )
                        ],
                        "1",
                        Txt,
                    )
                    nennerZaehlerGleich += findEqualNennerZaehler(
                        hierBereich, nenner, nennerZaehlerGleich
                    )

        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC(
                {
                    i18n.befehle2["kugeln"],
                    i18n.befehle2["kreise"],
                }
            ),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.universum[0],
                        "=",
                        i18n.kugelnKreise[0],
                    )
                )
            ],
            None,
            ("1-2", "99"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC(
                {
                    i18n.befehle2["netzwerk"],
                }
            ),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.universum[0],
                        "=",
                        i18n.netzwerkWort,
                    )
                )
            ],
            None,
            ("1-3", "99"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC(
                {
                    i18n.befehle2["komplex"],
                }
            ),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.universum[0],
                        "=",
                        i18n.komplexWort,
                    )
                )
            ],
            None,
            ("1", "3"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC(
                {
                    i18n.befehle2["absicht"],
                    i18n.befehle2["absichten"],
                    i18n.befehle2["motiv"],
                    i18n.befehle2["motive"],
                    i18n.befehle2["a"],
                }
            ),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.menschliches[0],
                        "=",
                        i18n.motivationWort,
                    )
                )
            ],
            None,
            ("1", "3"),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        if was_n_1proN_cmd:
            if len(rangesBruecheDict) > 0:
                cmd_gave_output = True
                for nenner, zaehler in rangesBruecheDict.items():
                    retaExecuteNprint(
                        ketten,
                        Txt.listeE,
                        vorherVonAusschnittOderZaehlung(Txt, ",".join(zaehler)),
                        "",
                        [
                            "".join(
                                (
                                    "--",
                                    i18n.gebrochenUniGal["gebrochengalaxie"][0],
                                    "=",
                                    str(nenner),
                                )
                            )
                        ],
                        "2",
                        Txt,
                    )
            elif len(rangesBruecheDictReverse) > 0:
                cmd_gave_output = True
                for nenner, zaehler in rangesBruecheDictReverse.items():
                    retaExecuteNprint(
                        ketten,
                        Txt.listeE,
                        vorherVonAusschnittOderZaehlung(Txt, ",".join(zaehler)),
                        "",
                        [
                            "".join(
                                (
                                    "--",
                                    i18n.gebrochenUniGal["gebrochengalaxie"][0],
                                    "=",
                                    str(nenner),
                                )
                            )
                        ],
                        "1",
                        Txt,
                    )

        eigN, eigR = [], []
        for aa in Txt.listeE:
            if i18n.EIGS_N_R[0] == aa[: len(i18n.EIGS_N_R[0])]:
                eigN += [aa[len(i18n.EIGS_N_R[0]) :]]
            if i18n.EIGS_N_R[1] == aa[: len(i18n.EIGS_N_R[1])]:
                eigR += [aa[len(i18n.EIGS_N_R[0]) :]]

        if len(eigN) > 0:
            if len(zahlenBereichC) > 0:
                cmd_gave_output = True
                retaExecuteNprint(
                    ketten,
                    Txt.listeE,
                    zeiln1,
                    zeiln2,
                    ["".join(("--", i18n.konzeptE["konzept"], "=", (",".join(eigN))))],
                    None,
                    Txt,
                )

        if len(eigR) > 0:
            cmd_gave_output = True
            # zeilenAusReziprokenDazu = ",".join(
            #    [
            #        bruch.split("/")[0]
            #        for bruch in bruch_GanzZahlReziproke.split(",")
            #        if bruch.split("/")[0] != ""
            #    ]
            # )

            # if len(zeiln1) > 1 and i18n.zeilenParas["oberesmaximum"] not in zeiln1:
            #    zeiln1 += (
            #        "," if zeiln1[-1].isdecimal() else ""
            #    ) + zeilenAusReziprokenDazu
            # if len(zeiln2) > 1 and i18n.zeilenParas["oberesmaximum"] not in zeiln2:
            #    zeiln2 += (
            #        "," if zeiln2[-1].isdecimal() else ""
            #    ) + zeilenAusReziprokenDazu
            ZahlenAngabenCneu = zahlenBereichC + "," + bruch_GanzZahlReziproke
            ZahlenAngabenCneu = ZahlenAngabenCneu.replace(",,", ",")
            ZahlenAngabenCneu = ZahlenAngabenCneu.strip(",")

            TxtNeu = deepcopy(Txt)
            TxtNeu.text += " " + bruch_GanzZahlReziproke
            # zeiln1Neu, zeiln2Neu, _, _ = zeiln1234create(
            #    TxtNeu,
            #    lenbruch_GanzZahlReziproke > 0,
            #    "",
            #    cNeu,
            #    maxNum,
            #    zahlenReiheKeineWteiler
            #    + ("," if len(zahlenReiheKeineWteiler) > 0 else "")
            #    + bruch_GanzZahlReziproke,
            # )
            # x(
            #    "EIGR",
            #    (
            #        cNeu,
            #        ketten,
            #        Txt.listeE,
            #        " ".join((zeiln3, zeiln1)),
            #        " ".join((zeiln4, zeiln2)),
            #        zahlenReiheKeineWteiler,
            #    ),
            # )
            if len(ZahlenAngabenCneu) > 0:
                retaExecuteNprint(
                    ketten + ["-" + i18n.hauptForNeben["zeilen"], zeiln1, zeiln2],
                    Txt.listeE,
                    zeiln3,
                    zeiln4,
                    ["".join(("--", i18n.konzeptE["konzept2"], "=", (",".join(eigR))))],
                    None,
                    Txt,
                )
            del ZahlenAngabenCneu
        was_n_1proN_cmd, cmd_gave_output = retaCmdAbstraction_n_and_1pron(
            Txt.hasWithoutABC({i18n.befehle2["universum"], i18n.befehle2["u"]}),
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.universum[0],
                        "=",
                        i18n.transzendentalienWort,
                    )
                )
            ],
            [
                "".join(
                    (
                        "--",
                        i18n.ParametersMain.universum[0],
                        "=",
                        i18n.transzendentaliereziprokeWort,
                    )
                )
            ],
            (
                "1"
                + (
                    ",4"
                    if len(Txt.menge & set(befehle)) <= 2
                    and not Txt.hasWithoutABC(
                        {
                            i18n.befehle2[
                                "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
                            ],
                            i18n.befehle2["e"],
                            i18n.befehle2["ee"],
                            "--" + i18n.ausgabeParas["keineueberschriften"],
                        }
                    )
                    else ""
                ),
                "1"
                + (
                    ",2"
                    if len(Txt.menge & set(befehle)) <= 2
                    and not Txt.hasWithoutABC(
                        {
                            i18n.befehle2[
                                "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
                            ],
                            i18n.befehle2["e"],
                            i18n.befehle2["ee"],
                            "--" + i18n.ausgabeParas["keineueberschriften"],
                        }
                    )
                    else ""
                ),
            ),
            Txt,
            bruch_GanzZahlReziproke,
            zahlenBereichC,
            ketten,
            cmd_gave_output,
            zeiln1,
            zeiln2,
            zeiln3,
            zeiln4,
        )
        if was_n_1proN_cmd:
            nennerZaehlerGleich = []
            # nennerZaehlerMakesWholeNum = []
            # nennerZaehlerMakesWholeNumReziproke = []
            if len(rangesBruecheDict) > 0:
                cmd_gave_output = True
                for nenner, zaehler in rangesBruecheDict.items():
                    hierBereich = ",".join(zaehler)
                    retaExecuteNprint(
                        ketten,
                        Txt.listeE,
                        vorherVonAusschnittOderZaehlung(Txt, hierBereich),
                        "",
                        [
                            "".join(
                                (
                                    "--",
                                    i18n.gebrochenUniGal["gebrochenuniversum"][0],
                                    "=",
                                    str(nenner),
                                )
                            )
                        ],
                        "2",
                        Txt,
                    )
                    nennerZaehlerGleich += findEqualNennerZaehler(
                        hierBereich, nenner, nennerZaehlerGleich
                    )
                    # nennerZaehlerMakesWholeNumS = findNennerZaehlerMakesWholeNum(
                    #    hierBereich,
                    #    nenner,
                    #    nennerZaehlerMakesWholeNum,
                    #    nennerZaehlerMakesWholeNumReziproke,
                    # )
                    # nennerZaehlerMakesWholeNum += nennerZaehlerMakesWholeNumS[0]
                    # nennerZaehlerMakesWholeNumReziproke += nennerZaehlerMakesWholeNumS[
                    #    1
                    # ]

            elif len(rangesBruecheDictReverse) > 0:
                cmd_gave_output = True
                for nenner, zaehler in rangesBruecheDictReverse.items():
                    hierBereich = ",".join(zaehler)
                    retaExecuteNprint(
                        ketten,
                        Txt.listeE,
                        vorherVonAusschnittOderZaehlung(Txt, hierBereich),
                        "",
                        [
                            "".join(
                                (
                                    "--",
                                    i18n.gebrochenUniGal["gebrochenuniversum"][0],
                                    "=",
                                    str(nenner),
                                )
                            )
                        ],
                        "1",
                        Txt,
                    )
                    nennerZaehlerGleich += findEqualNennerZaehler(
                        hierBereich, nenner, nennerZaehlerGleich
                    )
                    # nennerZaehlerMakesWholeNumS = findNennerZaehlerMakesWholeNum(
                    #    nenner,
                    #    hierBereich,
                    #    nennerZaehlerMakesWholeNum,
                    #    nennerZaehlerMakesWholeNumReziproke,
                    # )
                    # nennerZaehlerMakesWholeNum += nennerZaehlerMakesWholeNumS[0]
                    # nennerZaehlerMakesWholeNumReziproke += nennerZaehlerMakesWholeNumS[
                    #    1
                    # ]
            if len(nennerZaehlerGleich) != 0:
                cmd_gave_output = True
                nennerZaehlerGleich = set(nennerZaehlerGleich)
                nennerZaehlerGleich = ",".join(nennerZaehlerGleich)
                retaExecuteNprint(
                    ketten,
                    Txt.listeE,
                    vorherVonAusschnittOderZaehlung(Txt, nennerZaehlerGleich),
                    "",
                    [
                        "".join(
                            (
                                "--",
                                i18n.ParametersMain.universum[0],
                                "=",
                                i18n.verhaeltnisgleicherzahlWort,
                            )
                        )
                    ],
                    "1",
                    Txt,
                )
#            if False and len(nennerZaehlerMakesWholeNum) != 0:
#                cmd_gave_output = True
#                nennerZaehlerMakesWholeNum = set(nennerZaehlerMakesWholeNum)
#                nennerZaehlerMakesWholeNum = ",".join(nennerZaehlerMakesWholeNum)
#                retaExecuteNprint(
#                    ketten,
#                    Txt.listeE,
#                    vorherVonAusschnittOderZaehlung(Txt, nennerZaehlerMakesWholeNum),
#                    "",
#                    [
#                        "".join(
#                            (
#                                "--",
#                                i18n.ParametersMain.universum[0],
#                                "=",
#                                i18n.transzendentalienWort,
#                            )
#                        )
#                    ],
#                    "4",
#                    Txt,
#                )
#            if False and len(nennerZaehlerMakesWholeNumReziproke) != 0:
#                cmd_gave_output = True
#                nennerZaehlerMakesWholeNumReziproke = set(
#                    nennerZaehlerMakesWholeNumReziproke
#                )
#                nennerZaehlerMakesWholeNumReziproke = ",".join(
#                    nennerZaehlerMakesWholeNumReziproke
#                )
#                retaExecuteNprint(
#                    ketten,
#                    Txt.listeE,
#                    vorherVonAusschnittOderZaehlung(
#                        Txt, nennerZaehlerMakesWholeNumReziproke
#                    ),
#                    "",
#                    [
#                        "".join(
#                            (
#                                "--",
#                                i18n.ParametersMain.universum[0],
#                                "=",
#                                i18n.transzendentaliereziprokeWort,
#                            )
#                        )
#                    ],
#                    "2",
#                    Txt,
#                )
    if bedingungZahl:
        if (
            len(
                {i18n.befehle2["prim24"], i18n.befehle2["primfaktorzerlegungModulo24"]}
                & Txt.mengeE
            )
            > 0
        ):
            cmd_gave_output = True

            for arg in BereichToNumbers2(zahlenReiheKeineWteiler):
                print(
                    str(arg)
                    + ": "
                    + str(primRepeat(primfaktoren(int(arg), True)))[1:-1]
                    .replace("'", "")
                    .replace(", ", " ")
                )

        if Txt.hasWithoutABC({i18n.befehle2["primfaktorenvergleich"]}):
            cmd_gave_output = True
            bereiche = {}
            for geschriebenerZahlenBereich in re.split(
                r"\s|,", zahlenReiheKeineWteiler
            ):
                zahlenBereichBerechnet = BereichToNumbers2(
                    geschriebenerZahlenBereich, False, 0
                )
                for zahlInZahlenBereich in zahlenBereichBerechnet:
                    primFaktoren = primfaktoren(zahlInZahlenBereich)
                    bereiche[zahlInZahlenBereich] = {
                        primZahl: primFaktoren.count(primZahl)
                        for primZahl in primFaktoren
                    }
            gemeinsamePrimzahlen = {}
            for i, (geschriebenerZahlenBereich, primMap) in enumerate(bereiche.items()):
                if i == 0:
                    gemeinsamePrimzahlen = set(primMap.keys())
                else:
                    gemeinsamePrimzahlen &= set(primMap.keys())
            primGemeinsameVorkommen = {}
            for gemeinsamePrimzahl in gemeinsamePrimzahlen:
                vorkommens = []
                for i, (geschriebenerZahlenBereich, primMap) in enumerate(
                    bereiche.items()
                ):
                    vorkommens += [primMap[gemeinsamePrimzahl]]
                primGemeinsameVorkommen[gemeinsamePrimzahl] = min(vorkommens)
            gemeinsamePrimzahlenMatrix = [
                [primzahl] * vorkommenAnzahl
                for primzahl, vorkommenAnzahl in primGemeinsameVorkommen.items()
            ]
            gemeinsamePrimzahlenStr = " * ".join(
                [
                    str(primZahl)
                    for primZahlListe in gemeinsamePrimzahlenMatrix
                    for primZahl in primZahlListe
                ]
            )
            if len(gemeinsamePrimzahlenStr.strip()) == 0:
                gemeinsamePrimzahlenStr = "1"
            from functools import reduce

            try:
                grGv = reduce(
                    lambda x, y: x * y,
                    [
                        primZahl
                        for primZahlListe in gemeinsamePrimzahlenMatrix
                        for primZahl in primZahlListe
                    ],
                )
            except TypeError:
                grGv = 1

            if len(bereiche) > 1 or not (
                Txt.hasWithoutABC({i18n.befehle2["p"]})
                or Txt.hasWithoutABC({i18n.befehle2["mulpri"]})
            ):
                print(
                    i18n.gemeinsamkeitenWort
                    + ": {} := {}".format(grGv, gemeinsamePrimzahlenStr)
                )
                for zahl, hierUnwichtig in bereiche.items():
                    dazu = " * ".join(
                        [str(p) for p in primfaktoren(round(zahl / grGv))]
                    )
                    print(
                        f"{round(zahl / grGv):<5} := {zahl:<5} / {grGv:<5} -> "
                        + (dazu if len(dazu.strip()) > 0 else "1")
                    )
            # print("Unterschiede: {}".format(d))

        if Txt.hasWithoutABC({i18n.befehle2["prim"], i18n.befehle2["primfaktorzerlegung"]}):
            for arg in BereichToNumbers2(zahlenReiheKeineWteiler, False, 0):
                cmd_gave_output = True
                print(
                    str(arg)
                    + ": "
                    + str(primRepeat(primfaktoren(int(arg))))[1:-1]
                    .replace("'", "")
                    .replace(", ", " ")
                )

        if Txt.hasWithoutABC({i18n.befehle2["multis3"]}) > 0:
            cmd_gave_output = True

            listeStrWerte = BereichToNumbers2(zahlenReiheKeineWteiler, False, 0)
            mult3arg, mult3m3 = mult3(listeStrWerte)
            print(str(mult3arg) + ": " + str(list(mult3m3)))

        if Txt.hasWithoutABC({i18n.befehle2["multis"]}) > 0:
            cmd_gave_output = True

            listeStrWerte = list(BereichToNumbers2(zahlenReiheKeineWteiler, False, 0))
            multiplesTexts, multiis = mult2(listeStrWerte)
            mulpriInfo = not (Txt.hasWithoutABC({i18n.befehle2["mulpri"]}) or Txt.hasWithoutABC({i18n.befehle2["p"]}))
            for i, (texxt, multii) in enumerate(zip(multiplesTexts, multiis)):
                if len(multii) > 0 or mulpriInfo:
                    print(texxt)
                else:
                    StrZahl = str(listeStrWerte[i])
                    print("".join((StrZahl,": ", StrZahl, " (", i18n.primzahlWort, ")")))

            # externCommand(i18n.befehle2["prim"], c)

        if len({i18n.befehle2["mond"]} & Txt.mengeE) > 0:
            cmd_gave_output = True
            retaExecuteNprint(
                ketten,
                Txt.listeE,
                zeiln1,
                zeiln2,
                [
                    "".join(
                        (
                            "--",
                            i18n.ParametersMain.bedeutung[0],
                            "=",
                            i18n.gestirnWort,
                        )
                    )
                ],
                "3-6",
                Txt,
            )

        if len({i18n.befehle2["modulo"]} & Txt.mengeE) > 0:
            cmd_gave_output = True
            moduloA([str(num) for num in BereichToNumbers2(zahlenBereichC)])
        if len({i18n.befehle2["alles"]} & Txt.mengeE) > 0:
            cmd_gave_output = True
            retaExecuteNprint(
                ketten,
                Txt.listeE,
                zeiln1,
                zeiln2,
                ["--" + i18n.ParametersMain.alles[0]],
                None,
                Txt,
            )

        if len({i18n.befehle2["primzahlkreuz"]} & Txt.mengeE) > 0:
            cmd_gave_output = True
            retaExecuteNprint(
                ketten,
                Txt.listeE,
                zeiln1,
                anotherOberesMaximum(zahlenBereichC, 1028, Txt),
                [
                    "".join(
                        (
                            "--",
                            i18n.ParametersMain.bedeutung[0],
                            "=",
                            i18n.primzahlkreuzWort,
                        )
                    )
                ],
                None,
                Txt,
            )
            import reta

        if Txt.hasWithoutABC({i18n.befehle2["richtung"], i18n.befehle2["r"]}):
            cmd_gave_output = True
            retaExecuteNprint(
                ketten,
                Txt.listeE,
                zeiln1,
                zeiln2,
                [
                    "".join(
                        (
                            "--",
                            i18n.ParametersMain.primzahlwirkung[0],
                            "=",
                            i18n.GalaxieabsichtWort,
                        )
                    )
                ],
                None,
                Txt,
            )
        if (
            len(Txt.listeE) > 0
            and any(
                [token[:3] == "16_" and token[:5] != "16_15" for token in Txt.listeE]
            )
            and i18n.befehle2["abc"] not in Txt.listeE
            and i18n.befehle2["abcd"] not in Txt.listeE
        ):
            cmd_gave_output = True
            import reta

            befehle16 = []
            for token in Txt.listeE:
                if token[:3] == "16_":
                    befehle16 += [wahl16[token[3:]]]
            grundstruk = ",".join(befehle16)
            retaExecuteNprint(
                ketten,
                Txt.listeE,
                zeiln1,
                zeiln2,
                [
                    "".join(
                        (
                            "--",
                            i18n.ParametersMain.multiversum[0],
                            "=",
                            grundstruk,
                        )
                    )
                ],
                None,
                Txt,
            )
        if (
            len(Txt.listeE) > 0
            and any(
                [token[:3] == "15_" or token[:5] == "16_15" for token in Txt.listeE]
            )
            and i18n.befehle2["abc"] not in Txt.listeE
            and i18n.befehle2["abcd"] not in Txt.listeE
        ):
            cmd_gave_output = True
            import reta

            befehle15 = []
            for token in Txt.listeE:
                try:
                    if token[:3] == "15_":
                        befehle15 += [wahl15[token[3:]]]
                    if token == "16_15":
                        befehle15 += [wahl15["15"]]
                    if token[:6] == "16_15_":
                        befehle15 += [wahl15[token[6:]]]
                except KeyError:
                    pass
            grundstruk = ",".join(befehle15)
            retaExecuteNprint(
                ketten,
                Txt.listeE,
                zeiln1,
                zeiln2,
                [
                    "".join(
                        (
                            "--",
                            i18n.ParametersMain.grundstrukturen[0],
                            "=",
                            grundstruk,
                        )
                    )
                ],
                None,
                Txt,
            )
    ifAbst = Txt.hasWithoutABC({i18n.befehle2["abstand"]})
    ifAbstPrim = Txt.hasWithoutABC({i18n.befehle2["abstandPrim"]})
    if (
        ifAbst or ifAbstPrim
    ):
        zBereiche: list = []
        for i, s in enumerate(Txt.liste):
            if isZeilenAngabe(s):
                zBereiche += [s]
        allAreNumbers = all((z.isdecimal() for z in zBereiche))
        def maxMenge(mengen):
            mengen = list(mengen)
            if not mengen:
                return set()
            maxMenge: set = mengen[0]
            for menge in mengen[1:]:
                if len(maxMenge) < len(menge):
                    maxMenge = menge
            return maxMenge

        if len(zBereiche) > 1:
            cmd_gave_output = True
            zeige1 = {}
            zeigeAll1 = {}
            zeige2 = {}
            zeigeAll2 = {}
            zahlenBereiche = set()
            for zB in zBereiche:
                zahlenBereiche |= {frozenset(BereichToNumbers2(zB))}
            for i, zB1 in enumerate(zahlenBereiche):
                for k, zB2 in enumerate(zahlenBereiche - maxMenge(zahlenBereiche)):
                    if zB1 != zB2:
                        for zZahl1 in zB2:
                            if ifAbst:
                                dictionary1 = {zZahl2: abs(zZahl1 - zZahl2) for zZahl2 in zB1}
                                if len(dictionary1.items()) > 1 or allAreNumbers:
                                    zeige1.update(dictionary1)
                                    zeigeAll1.update({zZahl1:str(dictionary1)[1:-1]})
                            if ifAbstPrim:
                                dictionary2 = {zZahl2: primRepeat(primfaktoren(int(abs(zZahl1 - zZahl2)))) for zZahl2 in zB1}
                                if len(dictionary2.items()) > 1 or allAreNumbers:
                                    zeige2.update(dictionary2)
                                    zeigeAll2.update({zZahl1:str(dictionary2)[1:-1]})
            for i, (key, value) in enumerate(zeigeAll1.items()):
                print(str(key)+"->: "+value)
            for i, (key, value) in enumerate(zeigeAll2.items()):
                print(str(key)+"->: "+value)

        elif Txt.hasWithoutABC({i18n.befehle2["abstand"]}):
            print(i18nRP.abstandMeldung)

    loggingSwitch, cmd_gave_output = PromptVonGrosserAusgabeSonderBefehlAusgaben(
        loggingSwitch, Txt, cmd_gave_output
    )
    if len(nurEinBefehl) > 0:
        Txt.liste = list(befehleBeenden)
        nurEinBefehl = " ".join(befehleBeenden)
        exit()
    if (
        not cmd_gave_output
        and len(Txt.liste) > 0
        and Txt.listeE[0] not in befehleBeenden
    ):
        if len(Txt.menge & set(befehle)) > 0:
            print(i18nRP.out1Saetze[0] + " ".join(Txt.listeE) + i18nRP.out1Saetze[1])
        else:
            print(i18nRP.out2Satz.format(" ".join(Txt.listeE)))
    return loggingSwitch




def retaCmdAbstraction_n_and_1pron(
    condition,
    paras,
    paras2,
    selectedCols,
    Txt,
    bruch_GanzZahlReziproke,
    zahlenBereichC,
    ketten,
    cmd_gave_output,
    zeiln1,
    zeiln2,
    zeiln3,
    zeiln4,
):
    """abstraction for commands giving results forr n and 1/n"""
    was_n_1proN_cmd = False
    if condition and (
        i18n.befehle2["abc"] not in Txt.listeE
        and i18n.befehle2["abcd"] not in Txt.listeE
    ):
        was_n_1proN_cmd = True
        if len(zahlenBereichC) > 0:
            cmd_gave_output = True
            retaExecuteNprint(
                ketten, Txt.listeE, zeiln1, zeiln2, paras, selectedCols[0], Txt
            )
        if (
            len(bruch_GanzZahlReziproke) > 0
            and textHatZiffer(bruch_GanzZahlReziproke)
            and zeiln3 != ""
        ):
            cmd_gave_output = True
            retaExecuteNprint(
                ketten,
                Txt.listeE,
                zeiln3,
                zeiln4,
                paras if paras2 in [None, [], ()] else paras2,
                selectedCols[1],
                Txt,
            )
    return was_n_1proN_cmd, cmd_gave_output




def ifPrintCmdAgain(Txt):
    return (
        "".join(("--", i18n.ausgabeParas["art"], "=", i18n.ausgabeArt["bbcode"]))
        in Txt.listeE
        # and "reta" == Txt.listeE[0]
    )




def zeiln1234create(
    Txt,
    bedingungZahl,
    bruch_GanzZahlReziproke,
    zahlenBereichC,
    maxNum,
    zahlenReiheKeineWteiler,
):
    if len(bruch_GanzZahlReziproke) > 0 and textHatZiffer(bruch_GanzZahlReziproke):
        zeiln3 = vorherVonAusschnittOderZaehlung(Txt, bruch_GanzZahlReziproke)
        zeiln4 = ""
    else:
        zeiln3 = "".join(("--", i18n.zeilenParas["vorhervonausschnitt"], "=0"))
        zeiln4 = ""
    if bedingungZahl:
        zahlenBereiche = str(zahlenBereichC).strip()
        if textHatZiffer(zahlenBereiche):
            if i18n.befehle2["einzeln"] not in Txt.listeE and (
                (i18n.befehle2["vielfache"] in Txt.listeE)
                or (
                    i18n.befehle2["v"] in Txt.listeE
                    and i18n.befehle2["abc"] not in Txt.listeE
                    and i18n.befehle2["abcd"] not in Txt.listeE
                )
            ):
                if (
                    zahlenReiheKeineWteiler[0] == "("
                    and zahlenReiheKeineWteiler[-1] == ")"
                ):
                    zahlenReiheKeineWteiler[0] == "["
                    zahlenReiheKeineWteiler[-1] == "]"
                if (
                    zahlenReiheKeineWteiler[0] == "["
                    and zahlenReiheKeineWteiler[-1] == "]"
                ) or (
                    zahlenReiheKeineWteiler[0] == "{"
                    and zahlenReiheKeineWteiler[-1] == "}"
                ):
                    zahlenReiheKeineWteiler2 = ",".join(
                        [
                            str(B)
                            for B in BereichToNumbers2(zahlenReiheKeineWteiler)
                            if B != 0
                        ]
                    )

                else:
                    zahlenReiheKeineWteiler2 = zahlenReiheKeineWteiler

                if len(Txt.menge & {i18n.befehle2["teiler"], i18n.befehle2["w"]}) == 0:
                    zeiln1 = (
                        "".join(("--", i18n.zeilenParas["vielfachevonzahlen"], "="))
                        + zahlenReiheKeineWteiler2
                    )
                else:
                    zeiln1 = ""
                zeiln2 = "".join(
                    [
                        vorherVonAusschnittOderZaehlung(Txt, zahlenBereiche),
                        ",",
                        ",".join(
                            [
                                i18n.befehle2["v"] + str(z)
                                for z in re.split(
                                    kpattern,
                                    zahlenReiheKeineWteiler2,
                                )
                            ]
                        ),
                    ]
                )

                # zeiln2 = ""
            else:
                zeiln1 = vorherVonAusschnittOderZaehlung(Txt, zahlenBereiche)
                zeiln2 = anotherOberesMaximum(zahlenBereichC, maxNum, Txt)
        else:
            zeiln1 = "".join(("--", i18n.zeilenParas["vorhervonausschnitt"], "=0"))
            zeiln2 = ""

    else:
        zeiln1 = ""
        zeiln2 = ""

    return zeiln1, zeiln2, zeiln3, zeiln4




def retaExecuteNprint(
    ketten: list,
    stextE,
    zeiln1: str,
    zeiln2: str,
    welcheSpalten: list,
    ErlaubteSpalten: str,
    Txt: TXT,
):
    import reta

    kette = [
        "reta",
        "".join(("-", i18n.hauptForNeben["zeilen"])),
        zeiln1,
        zeiln2,
        ("--"+i18n.zeilenParas["invertieren"] if i18n.befehle2["invertieren"] in stextE else ""),
        "".join(("-", i18n.hauptForNeben["spalten"])),
        "".join(welcheSpalten),
        "".join(("--", i18n.ausgabeParas["breite"], "=0")),
        "".join(("-", i18n.hauptForNeben["ausgabe"])),
        "".join(
            (
                "--",
                i18n.ausgabeParas["spaltenreihenfolgeundnurdiese"],
                "=",
                ErlaubteSpalten,
            )
        )
        if ErlaubteSpalten is not None
        else "",
        *[
            "--" + i18n.ausgabeParas["keineleereninhalte"]
            if i18n.befehle2["keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"]
            in stextE
            else ""
        ],
    ] + returnOnlyParasAsList(stextE)
    kette += ketten
    for el in kette:
        vorhervonaus: set = set()
        if i18n.zeilenParas["vorhervonausschnitt"]+"=" in el:
            vorhervonaus |= {el}
    if len(vorhervonaus) > 1:
        kette.remove(i18n.zeilenParas["vorhervonausschnitt"]+"=0")


    if (
        i18n.befehle2["keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"]
        not in stextE
    ):
        if ifPrintCmdAgain(Txt):
            if "--" + i18n.ausgabeParas["nocolor"] in stextE:
                print("[code]" + (" ".join(kette)) + "[/code]")
            else:
                cliout("[code]" + (" ".join(kette)) + "[/code]", True, i18n.ausgabeArt["bbcode"])
        else:
            if "--" + i18n.ausgabeParas["nocolor"] in stextE:
                print(" ".join(kette))
            else:
                cliout(" ".join(kette), True)
    reta.Program(kette, Txt=Txt)




def findEqualNennerZaehler(hierBereich, nenner, nennerZaehlerGleich):
    hierBereich2 = BereichToNumbers2(str(hierBereich))
    nenner2 = BereichToNumbers2(str(nenner))
    for nn3 in nenner2:
        for hB3 in hierBereich2:
            if nn3 == hB3 and nn3 not in [0, 1]:
                nennerZaehlerGleich += [str(nn3)]
    return nennerZaehlerGleich




def findNennerZaehlerMakesWholeNum(
    zaehler, nenner, wholeNumList, wholeNumListReziproke
):
    zaehler2 = BereichToNumbers2(str(zaehler))
    nenner2 = BereichToNumbers2(str(nenner))
    for nn3 in nenner2:
        for zz3 in zaehler2:
            ratNumRez: Fraction = Fraction(zz3, nn3)
            ratNum: Fraction = Fraction(nn3, zz3)
            if int(ratNum) == ratNum:
                wholeNumList += [str(int(ratNum))]
            if int(ratNumRez) == ratNumRez:
                wholeNumListReziproke += [str(int(ratNumRez))]
    return wholeNumList, wholeNumListReziproke




def bruchBereichsManagementAndWbefehl(zahlenBereichC, stext, zahlenAngaben_):
    bruch_GanzZahlReziproke = []
    bruch_GanzZahlReziprokeAbzug = []
    bruch_KeinGanzZahlReziproke = {}
    bruch_KeinGanzZahlReziprokeAbzug = {}
    bruch_KeinGanzZahlReziprok_ = []
    fullBlockIsZahlenbereichAndBruch = True
    rangesBruecheDict = {}
    rangesBruecheDictReverse: dict = {}
    bruch_KeinGanzZahlReziprokeEnDictAbzug = {}
    bruchRanges3Abzug = {}
    valueLenSum = 0
    zahlenAngaben_mehrere = []
    Minusse = {}
    pfaue = {}
    pfaueAbzug = {}
    # alxp(stext)
    for g, a in enumerate(stext):
        bruchAndGanzZahlEtwaKorrekterBereich = []
        bruchBereichsAngaben = []
        bruchRanges = []
        abzug = False
        if a[:1] != "-":
            for etwaBruch in custom_split2(a, ","):
                bruchRange, bruchBereichsAngabe = createRangesForBruchLists(
                    bruchSpalt(etwaBruch)
                )
                (
                    bruchAndGanzZahlEtwaKorrekterBereich,
                    bruchBereichsAngaben,
                    bruchRanges,
                    zahlenAngaben_,
                    etwaAllTrue,
                ) = verifyBruchNganzZahlBetweenCommas(
                    bruchAndGanzZahlEtwaKorrekterBereich,
                    bruchBereichsAngabe,
                    bruchBereichsAngaben,
                    bruchRange,
                    bruchRanges,
                    etwaBruch,
                    zahlenAngaben_,
                )
                if etwaAllTrue:
                    fullBlockIsZahlenbereichAndBruch = (
                        fullBlockIsZahlenbereichAndBruch
                        and all(bruchAndGanzZahlEtwaKorrekterBereich)
                    )

            if fullBlockIsZahlenbereichAndBruch:
                for bruchBereichsAngabe, bruchRange in zip(
                    bruchBereichsAngaben, bruchRanges
                ):
                    if isZeilenAngabe(bruchBereichsAngabe):
                        bruchRange = {b for b in bruchRange if b > 0}
                        EinsInBereichHier1 = BereichToNumbers2(bruchBereichsAngabe)
                        EinsInBereichHier = 1 in EinsInBereichHier1
                        if (
                            bruchBereichsAngabe[:1] == "-"
                            or bruchBereichsAngabe[:2] == i18n.befehle2["v"] + "-"
                        ):
                            minusHier = True
                            if bruchBereichsAngabe[:2] == i18n.befehle2["v"] + "-":
                                pass
                            if bruchBereichsAngabe[:1] == "-":
                                pass
                        else:
                            minusHier = False
                        if 1 in bruchRange:
                            if minusHier:
                                bruch_GanzZahlReziprokeAbzug += [bruchBereichsAngabe]
                            else:
                                bruch_GanzZahlReziproke += [bruchBereichsAngabe]
                        bruchRangeOhne1 = frozenset(set(bruchRange) - {1})
                        neuerBereich = ",".join(
                            {str(zahl) for zahl in EinsInBereichHier1} - {"1"}
                        )
                        Minusse[tuple(bruchRange)] = minusHier
                        if len(bruchRangeOhne1) > 0:
                            if minusHier:
                                try:
                                    bruch_KeinGanzZahlReziprokeAbzug[
                                        bruchRangeOhne1
                                    ] += [bruchBereichsAngabe]
                                    pfaueAbzug[bruchRangeOhne1] += [
                                        bruchBereichsAngabe[:1] == i18n.befehle2["v"]
                                    ]
                                except KeyError:
                                    bruch_KeinGanzZahlReziprokeAbzug[
                                        bruchRangeOhne1
                                    ] = [bruchBereichsAngabe]
                                    pfaueAbzug[bruchRangeOhne1] = [
                                        bruchBereichsAngabe[:1] == i18n.befehle2["v"]
                                    ]
                            else:
                                try:
                                    bruch_KeinGanzZahlReziproke[bruchRangeOhne1] += [
                                        neuerBereich
                                    ]
                                    pfaue[bruchRangeOhne1] += [
                                        bruchBereichsAngabe[:1] == i18n.befehle2["v"]
                                    ]
                                except KeyError:
                                    bruch_KeinGanzZahlReziproke[bruchRangeOhne1] = [
                                        neuerBereich
                                    ]
                                    pfaue[bruchRangeOhne1] = [
                                        bruchBereichsAngabe[:1] == i18n.befehle2["v"]
                                    ]
                        if EinsInBereichHier:
                            neueRange = ",".join([str(zahl) for zahl in bruchRange])
                            stext += [neueRange]
                            EsGabzahlenAngaben = True
                            zahlenAngaben_mehrere += [neueRange]
        zahlenAngaben_mehrere = list(set(zahlenAngaben_ + zahlenAngaben_mehrere))
        # x("zahlenAngaben_mehrere", zahlenAngaben_mehrere)
    try:
        EsGabzahlenAngaben
    except UnboundLocalError:
        EsGabzahlenAngaben = False
    if (i18n.befehle2["v"] in stext) or (i18n.befehle2["vielfache"] in stext):
        if not (
            (i18n.befehle2["e"] in stext)
            or (
                i18n.befehle2["keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"]
                in stext
            )
        ):
            if (
                len(bruch_GanzZahlReziproke) > 0
                or any(
                    [
                        any([1 in BereichToNumbers2(val2) for val2 in val])
                        for val in bruch_KeinGanzZahlReziproke.values()
                    ]
                )
                or EsGabzahlenAngaben
            ):
                print(i18nRP.out3Saetze)
        bdNeu = set()
        for bDazu in bruch_GanzZahlReziproke:
            for bDazu in BereichToNumbers2(bDazu):
                i = 1
                rechnung = i * bDazu
                while rechnung < retaProgram.tables.hoechsteZeile[1024]:
                    bdNeu |= {rechnung}
                    i += 1
                    rechnung = i * bDazu
        for bDazu in bruch_GanzZahlReziprokeAbzug:
            if bDazu[:1] == i18n.befehle2["v"]:
                bDazu = bDazu[1:]
            if bDazu[:1] == "-":
                bDazu = bDazu[1:]
            for bDazu in BereichToNumbers2(bDazu):
                i = 1
                rechnung = i * bDazu
                while rechnung < retaProgram.tables.hoechsteZeile[1024]:
                    try:
                        bdNeu -= {rechnung}
                        i += 1
                        rechnung = i * bDazu
                    except:
                        pass
        bruch_GanzZahlReziproke = ",".join((str(b) for b in bdNeu))
        bruchRanges3 = {}
        bruch_KeinGanzZahlReziprokeEnDict = {}
        for k, (brZahlen, no1brueche) in enumerate(bruch_KeinGanzZahlReziproke.items()):
            for no1bruch in no1brueche:
                if len(no1bruch) > 0 and no1bruch[0] == i18n.befehle2["v"]:
                    no1bruch = no1bruch[1:]
                if len(no1bruch) > 0 and no1bruch[0] == "-":
                    no1bruch = no1bruch[1:]
                    abzug = True
                else:
                    abzug = False
                no1brueche = BereichToNumbers2(no1bruch)
                for no1bruch in no1brueche:
                    i = 1
                    rechnung2 = no1bruch * i
                    while rechnung2 in gebrochenErlaubteZahlen:
                        if rechnung2 not in bruch_KeinGanzZahlReziprokeEnDict.values():
                            if abzug:
                                try:
                                    bruch_KeinGanzZahlReziprokeEnDictAbzug[k] += [
                                        rechnung2
                                    ]
                                except KeyError:
                                    bruch_KeinGanzZahlReziprokeEnDictAbzug[k] = [
                                        rechnung2
                                    ]
                            else:
                                try:
                                    bruch_KeinGanzZahlReziprokeEnDict[k] += [rechnung2]
                                except KeyError:
                                    bruch_KeinGanzZahlReziprokeEnDict[k] = [rechnung2]
                        i += 1
                        rechnung2 = no1bruch * i
            for br in brZahlen:
                i = 1
                rechnung = br * i
                while rechnung in gebrochenErlaubteZahlen:
                    if abzug:
                        try:
                            if rechnung not in bruchRanges3Abzug:
                                bruchRanges3Abzug[k] += [rechnung]
                        except KeyError:
                            bruchRanges3Abzug[k] = [rechnung]
                    else:
                        try:
                            if rechnung not in bruchRanges3:
                                bruchRanges3[k] += [rechnung]
                        except KeyError:
                            bruchRanges3[k] = [rechnung]
                    i += 1
                    rechnung = br * i

        for keyRanges, valueRanges in bruchRanges3.items():
            for (
                keyBrueche,
                valueBrueche,
            ) in bruch_KeinGanzZahlReziprokeEnDict.items():
                for eineRange in valueRanges:
                    for einBruch in valueBrueche:
                        if keyRanges == keyBrueche:
                            try:
                                strBruch = str(einBruch)
                                if strBruch not in rangesBruecheDict[eineRange]:
                                    rangesBruecheDict[eineRange] += [strBruch]
                            except KeyError:
                                rangesBruecheDict[eineRange] = [str(einBruch)]
        if len(bruchRanges3Abzug) > 0:
            rangesBruecheDict2 = deepcopy(rangesBruecheDict)
            for AbzugNenners, AbzugZaehlers in zip(
                bruchRanges3Abzug.values(),
                bruch_KeinGanzZahlReziprokeEnDictAbzug.values(),
            ):
                for aNenner, aZaehler in zip(AbzugNenners, AbzugZaehlers):
                    for key, value in zip(
                        bruchRanges3.values(), rangesBruecheDict.values()
                    ):
                        try:
                            if key.index(int(aNenner)) == value.index(str(aZaehler)):
                                try:
                                    value.remove(str(aZaehler))
                                except:
                                    pass
                                try:
                                    key.remove(str(aNenner))
                                except:
                                    pass
                                try:
                                    value.remove(aZaehler)
                                except:
                                    pass
                                try:
                                    key.remove(aNenner)
                                except:
                                    pass
                                rangesBruecheDict2[aNenner] = value
                        except ValueError:
                            pass
            rangesBruecheDict = rangesBruecheDict2
            bruchRanges3Abzug = {}
            bruch_KeinGanzZahlReziprokeEnDictAbzug = {}
    else:
        if (
            len(bruch_GanzZahlReziproke) == 0
            or type(bruch_GanzZahlReziproke) is not str
        ):
            bruch_GanzZahlReziproke = ",".join(
                (
                    ",".join(bruch_GanzZahlReziproke),
                    ",".join(bruch_GanzZahlReziprokeAbzug),
                )
            )
        elif type(bruch_GanzZahlReziproke) is str:
            bruch_GanzZahlReziproke += "," + (
                ",".join(
                    (
                        ",".join(bruch_GanzZahlReziproke),
                        ",".join(bruch_GanzZahlReziprokeAbzug),
                    )
                )
            )

        bruchDict = {}
        for (bruchRange, bruch_KeinGanzZahlReziprok_), pfauList in zip(
            bruch_KeinGanzZahlReziproke.items(), pfaue.values()
        ):
            bruch_KeinGanzZahlReziprok_2 = set()
            for pfau, nenners in zip(pfauList, bruch_KeinGanzZahlReziprok_):
                if pfau:
                    nenners = BereichToNumbers2(nenners)
                    for nenner in nenners:
                        i = 1
                        rechnung = i * int(nenner)
                        while rechnung in gebrochenErlaubteZahlen:
                            bruch_KeinGanzZahlReziprok_2 |= {str(rechnung)}
                            i += 1
                            rechnung = i * int(nenner)
                else:
                    bruch_KeinGanzZahlReziprok_2 |= set(re.split(kpattern, nenners))
            bruch_KeinGanzZahlReziprok_ = ",".join(bruch_KeinGanzZahlReziprok_2)
            for rangePunkt in bruchRange:
                try:
                    bruchDict[rangePunkt] |= {bruch_KeinGanzZahlReziprok_}
                except KeyError:
                    bruchDict[rangePunkt] = {bruch_KeinGanzZahlReziprok_}

                for (
                    bruchRangeA,
                    bruch_KeinGanzZahlReziprok_A,
                ) in bruch_KeinGanzZahlReziprokeAbzug.items():
                    bruch_KeinGanzZahlReziprok_A = ",".join(
                        bruch_KeinGanzZahlReziprok_A
                    )
                    for rangePunktA in bruchRangeA:
                        if rangePunkt == rangePunktA:
                            try:
                                bruchDict[rangePunkt] |= {
                                    bruch_KeinGanzZahlReziprok_,
                                    bruch_KeinGanzZahlReziprok_A,
                                }
                            except KeyError:
                                bruchDict[rangePunkt] = {
                                    bruch_KeinGanzZahlReziprok_,
                                    bruch_KeinGanzZahlReziprok_A,
                                }
        rangesBruecheDict = bruchDict
    rangesBruecheDict2 = {}
    bereicheVorherBestimmtSet = set()
    for key, values in rangesBruecheDict.items():
        bereichVorherBestimmt = [BereichToNumbers2(value) for value in values]
        bereicheVorherBestimmtSet2 = set()
        for b in bereichVorherBestimmt:
            bereicheVorherBestimmtSet2 |= b
        bereicheVorherBestimmtSet |= bereicheVorherBestimmtSet2
        rangesBruecheDict2[key] = list(bereicheVorherBestimmtSet2)
    valueLenSum += len(bereicheVorherBestimmtSet)
    dictLen = len(rangesBruecheDict)
    if dictLen != 0:
        avg = valueLenSum / dictLen
        if avg < 1:
            rangesBruecheDictReverse = invert_dict_B(rangesBruecheDict2)
            rangesBruecheDict = {}
    zahlenAngaben_mehrere = list(set(zahlenAngaben_mehrere))
    if len(zahlenAngaben_mehrere) > 0:
        zahlenAngaben_mehrereStr = ",".join(zahlenAngaben_mehrere)
        zahlenReiheKeineWteiler = copy(zahlenAngaben_mehrereStr)
        if i18n.befehle2["w"] in stext or i18n.befehle2["teiler"] in stext:
            zahlenAngaben_mehrereStr = ",".join(
                [
                    str(zahl)
                    for zahl in BereichToNumbers2(
                        ",".join(
                            [
                                str(z).split("+")[0]
                                for z in re.split(
                                    kpattern,
                                    zahlenReiheKeineWteiler,
                                )
                            ]
                        ),
                        False,
                        0,
                    )
                ]
            )
            zahlenBereichC: str = ",".join(teiler(zahlenAngaben_mehrereStr)[0])
            if len(zahlenReiheKeineWteiler) > 1:
                zahlenBereichC += "," + zahlenReiheKeineWteiler
        else:
            zahlenBereichC = zahlenAngaben_mehrereStr

    try:
        zahlenReiheKeineWteiler
    except (UnboundLocalError, NameError):
        zahlenReiheKeineWteiler = ""

    dazu = []
    sdazu = []
    bruch_GanzZahlReziprokeDazu = []
    EsGabzahlenAngaben, bruch_GanzZahlReziprokeDazu, dazu, sdazu = addMoreVals2(
        EsGabzahlenAngaben,
        bruch_GanzZahlReziprokeDazu,
        dazu,
        rangesBruecheDict,
        sdazu,
        False,
    )
    EsGabzahlenAngaben, bruch_GanzZahlReziprokeDazu, dazu, sdazu = addMoreVals2(
        EsGabzahlenAngaben,
        bruch_GanzZahlReziprokeDazu,
        dazu,
        rangesBruecheDictReverse,
        sdazu,
        True,
    )

    if len(dazu) > 0:
        zahlenBereichC = ",".join(
            filter(None, sdazu + re.split(kpattern, zahlenBereichC))
        )
        stext += [",".join(sdazu + dazu)]
        bruch_GanzZahlReziproke = ",".join(
            filter(
                None,
                bruch_GanzZahlReziprokeDazu
                + re.split(kpattern, bruch_GanzZahlReziproke),
            )
        )

    return (
        bruch_GanzZahlReziproke,
        zahlenBereichC,
        zahlenReiheKeineWteiler,
        fullBlockIsZahlenbereichAndBruch,
        rangesBruecheDict,
        len(zahlenAngaben_) > 0 or EsGabzahlenAngaben,
        rangesBruecheDictReverse,
        stext,
    )




def addMoreVals2(
    EsGabzahlenAngaben,
    bruch_GanzZahlReziprokeDazu,
    dazu,
    rangesBruecheOrReverseDict,
    sdazu,
    ifReverse,
):
    for key, values in rangesBruecheOrReverseDict.items():
        key = int(key)
        if key != 0:
            for value in BereichToNumbers2(",".join(values)):
                if value != 0:
                    bruch2 = (
                        Fraction(key, value) if not ifReverse else Fraction(value, key)
                    )
                    (
                        EsGabzahlenAngaben,
                        bruch_GanzZahlReziprokeDazu,
                        dazu,
                        sdazu,
                    ) = addMoreVals(
                        EsGabzahlenAngaben,
                        bruch2,
                        bruch_GanzZahlReziprokeDazu,
                        dazu,
                        sdazu,
                    )
    return EsGabzahlenAngaben, bruch_GanzZahlReziprokeDazu, dazu, sdazu




def addMoreVals(EsGabzahlenAngaben, bruch2, bruch_GanzZahlReziprokeDazu, dazu, sdazu):
    if bruch2.numerator % bruch2.denominator == 0:
        dazu += [str(int(bruch2))]
        sdazu += [str(int(bruch2))]
        EsGabzahlenAngaben = True
    if bruch2.denominator % bruch2.numerator == 0:
        dazu += ["1/" + str(int(bruch2**-1))]
        bruch_GanzZahlReziprokeDazu += [str(int(bruch2**-1))]
    return EsGabzahlenAngaben, bruch_GanzZahlReziprokeDazu, dazu, sdazu




def PromptVonGrosserAusgabeSonderBefehlAusgaben(loggingSwitch, Txt, cmd_gave_output):
    if (
        len(Txt.listeS) > 0
        and Txt.listeS[0] == i18n.befehle2["shell"]
        and not (
            Txt.has({i18n.befehle2["abc"], i18n.befehle2["abcd"]})
            and len(Txt.liste) == 2
        )
    ):
        cmd_gave_output = True
        try:
            process = subprocess.Popen([*Txt.listeS[1:]])
            process.wait()
        except:
            pass
    if (
        len(Txt.listeS) > 0
        and i18n.befehle2["python"] == Txt.listeS[0]
        and not (
            Txt.has({i18n.befehle2["abc"], i18n.befehle2["abcd"]})
            and len(Txt.liste) == 2
        )
    ):
        cmd_gave_output = True
        try:
            process = subprocess.Popen(["python3", "-c", " ".join(Txt.listeS[1:])])
            process.wait()
        except:
            pass
    if len(Txt.listeS) > 0 and i18n.befehle2["math"] == Txt.listeS[0]:
        cmd_gave_output = True
        for st in re.split(kpattern, "".join(Txt.listeS[1:2])):
            try:
                process = subprocess.Popen(["python3", "-c", "print(" + st + ")"])
                process.wait()
            except:
                pass
    if Txt.hasWithoutABC({i18n.befehle2["loggen"]}):
        cmd_gave_output = True
        loggingSwitch = True
    elif Txt.hasWithoutABC({i18n.befehle2["nichtloggen"]}):
        cmd_gave_output = True
        loggingSwitch = False
    return loggingSwitch, cmd_gave_output


