from __future__ import annotations

import re
import sys
from copy import copy, deepcopy
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from typing import Dict, List, MutableMapping, Optional, Sequence, Set, Tuple

from .completion_runtime import CompletionRuntimeBundle, bootstrap_completion_runtime
from .facade import RetaArchitecture
from .prompt_runtime import PromptRuntimeBundle, bootstrap_prompt_runtime


class PromptModus(Enum):
    normal = 0
    speichern = 1
    loeschenStart = 2
    speicherungAusgaben = 3
    loeschenSelect = 4
    speicherungAusgabenMitZusatz = 5
    AusgabeSelektiv = 6


def custom_split(text):
    stack = []
    result = []
    start = 0
    for i, char in enumerate(text):
        if char in "({[":
            stack.append(char)
        elif char in ")}]":
            if stack:
                stack.pop()
        elif char.isspace() and not stack:
            result.append(text[start:i])
            start = i + 1
    if start < len(text):
        result.append(text[start:])
    return result


def custom_split2(input_string, delimiter):
    result = []
    temp = ""
    stack = []
    for char in input_string:
        if char in "({[":
            stack.append("(")
            temp += char
        elif char in ")}]":
            if stack and stack[-1] in "({[":
                stack.pop()
                temp += char
            else:
                temp += char
        elif char == delimiter and not stack:
            result.append(temp)
            temp = ""
        else:
            temp += char
    if temp:
        result.append(temp)
    return result


def verkuerze_dict(dictionary: dict) -> dict:
    dict2: dict = {}
    for key, value in dictionary.items():
        if value not in dict2.values():
            dict2[key] = value
    return dict2


@dataclass
class PromptLanguageBundle:
    architecture: RetaArchitecture
    i18n: object
    prompt_runtime: PromptRuntimeBundle
    completion_runtime: CompletionRuntimeBundle
    not_parameter_values: List[str]
    gebrochen_erlaubte_zahlen: Set[int]
    wahl15: Dict[str, str]
    wahl16: Dict[str, str]
    short_command_letters: Set[str] = field(default_factory=set)

    def __post_init__(self) -> None:
        if not self.short_command_letters:
            self.short_command_letters = {
                self.i18n.befehle2[var]
                for var in self.i18n.befehle2.keys()
                if len(var) == 1
            }

    @property
    def befehle(self) -> List[str]:
        return list(self.completion_runtime.befehle)

    @property
    def befehle2(self) -> Set[str]:
        return set(self.completion_runtime.befehle2)

    def snapshot(self) -> Dict[str, object]:
        return {
            "class": type(self).__name__,
            "not_parameter_values_len": len(self.not_parameter_values),
            "gebrochen_erlaubte_zahlen_len": len(self.gebrochen_erlaubte_zahlen),
            "wahl15_len": len(self.wahl15),
            "wahl16_len": len(self.wahl16),
            "short_command_letters": sorted(self.short_command_letters),
        }

    @staticmethod
    def str_as_generator_to_numset(text: str) -> Optional[Set[int]]:
        try:
            if text[0] == "(" and text[-1] == ")":
                text = "[" + text[1:-1] + "]"
            if (text[0] == "[" and text[-1] == "]") or (text[0] == "{" and text[-1] == "}"):
                try:
                    result = eval(text)
                    result = set(result)
                    if type(result) is set and all((type(a) is int for a in result)):
                        return result
                except Exception:
                    return None
        except Exception:
            return None
        return None

    def is_zeilenangabe_between_kommas(self, text: str) -> bool:
        generated1 = self.str_as_generator_to_numset(text)
        generated2 = self.str_as_generator_to_numset(text[1:]) if len(text) > 1 else None
        return (
            self.architecture.inputs.row_ranges.is_integer_range_token(text)
            or generated1 is not None
            or generated2 is not None
        )

    def is_zeilenbruch_between_kommas(self, text: str) -> bool:
        return self.architecture.inputs.row_ranges.is_fraction_range_token(text)

    def isReTaParameter(self, text: str) -> bool:
        return (
            len(text) > 0
            and text[0] == "-"
            and not self._is_zeilenbruch_or_ganzzahl_angabe(text)
            and text.split("=")[0] in [str(c).split("=")[0] for c in self.not_parameter_values]
        )

    def _is_zeilenbruch_or_ganzzahl_angabe(self, text: str) -> bool:
        checks = []
        for token in self.architecture.inputs.row_ranges.split_comma_list(text):
            checks.append(
                self.is_zeilenbruch_between_kommas(token)
                or self.is_zeilenangabe_between_kommas(token)
            )
        return all(checks)

    def is15or16command(self, text: str) -> bool:
        if text[:3] == "15_":
            if text[3:] == "":
                return True
            if text[3:] in self.wahl15:
                return True
        if text[:3] == "16_":
            if text[3:] == "":
                return True
            if text[:5] == "16_15":
                if text[5:] == "":
                    return True
                if text[5] == "_" and text[6:] in self.wahl15:
                    return True
            if text[3:] in self.wahl16:
                return True
        return False

    def verifyBruchNganzZahlCommaList(
        self,
        bruchAndGanzZahlEtwaKorrekterBereich,
        bruchBereichsAngabe,
        bruchBereichsAngaben,
        bruchRange,
        bruchRanges,
        commaListe,
        zahlenAngaben_,
    ):
        _bruchAndGanzZahlEtwaKorrekterBereich = []
        _bruchBereichsAngaben = []
        _bruchRanges = []
        _zahlenAngaben_ = []
        _etwaAllTrue = []

        for etwaBruch in self.architecture.inputs.row_ranges.split_comma_list(commaListe):
            (
                bruchAndGanzZahlEtwaKorrekterBereich1,
                bruchBereichsAngaben1,
                bruchRanges1,
                zahlenAngaben_1,
                etwaAllTrue1,
            ) = self.verifyBruchNganzZahlBetweenCommas(
                bruchAndGanzZahlEtwaKorrekterBereich,
                bruchBereichsAngabe,
                bruchBereichsAngaben,
                bruchRange,
                bruchRanges,
                etwaBruch,
                zahlenAngaben_,
            )
            _bruchAndGanzZahlEtwaKorrekterBereich += [bruchAndGanzZahlEtwaKorrekterBereich1]
            _bruchBereichsAngaben += [bruchBereichsAngaben1]
            _bruchRanges += [bruchRanges1]
            _zahlenAngaben_ += [zahlenAngaben_1]
            _etwaAllTrue += [etwaAllTrue1]
        return (
            _bruchAndGanzZahlEtwaKorrekterBereich,
            _bruchBereichsAngaben,
            _bruchRanges,
            _zahlenAngaben_,
            all(_bruchAndGanzZahlEtwaKorrekterBereich),
        )

    def verifyBruchNganzZahlBetweenCommas(
        self,
        bruchAndGanzZahlEtwaKorrekterBereich,
        bruchBereichsAngabe,
        bruchBereichsAngaben,
        bruchRange,
        bruchRanges,
        etwaBruch,
        zahlenAngaben_,
    ):
        isBruch = self.is_zeilenangabe_between_kommas(bruchBereichsAngabe)
        isGanzZahl = self.is_zeilenangabe_between_kommas(etwaBruch)
        if isBruch != isGanzZahl:
            bruchAndGanzZahlEtwaKorrekterBereich += [True]
            if isBruch:
                bruchRanges += [bruchRange]
                bruchBereichsAngaben += [bruchBereichsAngabe]
            elif isGanzZahl:
                zahlenAngaben_ += [etwaBruch]
        else:
            bruchAndGanzZahlEtwaKorrekterBereich += [False]
        return (
            bruchAndGanzZahlEtwaKorrekterBereich,
            bruchBereichsAngaben,
            bruchRanges,
            zahlenAngaben_,
            all(bruchAndGanzZahlEtwaKorrekterBereich),
        )

    def stextFromKleinKleinKleinBefehl(self, promptMode2, stext, textDazu):
        stext2 = []
        ifKurzKurz = False
        xtext = " ".join(stext)
        stext2 = custom_split(xtext)
        stext3 = []
        del xtext
        for kkk, s_ in enumerate(tuple(deepcopy(stext2))):
            s_ = s_.strip(",")
            s_m = s_
            textDazu = []
            if not self.is15or16command(s_) and s_ not in self.befehle and stext[0] != "reta":
                nn: Optional[int] = 0
                for iii, s_3 in enumerate(s_[::-1]):
                    if s_3.isdecimal() or (
                        s_3 in (")", "]", "}")
                        and "reta" not in s_
                        and any(("(" in at or "[" in at or "{" for at in s_))
                    ):
                        nn = iii
                        break
                if nn > 0:
                    s_b = s_[-nn:] + s_[:-nn]
                else:
                    s_b = s_
                n: Optional[int] = None
                for ii, s_3 in enumerate(s_b):
                    if s_3.isdecimal() or (
                        s_3 in ("(", "[", "{")
                        and "reta" not in s_b
                        and any((")" in at or "]" in at or "}" for at in s_b))
                    ):
                        n = ii
                        break
                try:
                    if s_b[int(n) - 1] == "-":
                        n -= 1
                except Exception:
                    pass

                if n is not None:
                    (
                        bruchAndGanzZahlEtwaKorrekterBereich,
                        bruchBereichsAngaben,
                        bruchRanges,
                        zahlenAngaben__Z,
                        fullBlockIsZahlenbereichAndBruch_Z,
                    ) = self.verifyBruchNganzZahlCommaList(
                        [],
                        "",
                        [],
                        [],
                        [],
                        s_b[n:],
                        [],
                    )

                    if fullBlockIsZahlenbereichAndBruch_Z:
                        s_ = s_b
                        if not (
                            (s_[0] == "(" and s_[-1] == ")")
                            or (s_[0] == "[" and s_[-1] == "]")
                            or (s_[0] == "{" and s_[-1] == "}")
                        ):
                            buchst = set(s_[:n]) & self.short_command_letters
                        else:
                            buchst = set()
                        setTextLenIs1 = (
                            len(
                                set(stext)
                                - {
                                    self.i18n.befehle2["e"],
                                    self.i18n.befehle2[
                                        "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
                                    ],
                                }
                            )
                            == 1
                        )

                        if (len(buchst) != len(s_[:n]) or len(buchst) == 0) and not (
                            setTextLenIs1 and fullBlockIsZahlenbereichAndBruch_Z
                        ):
                            s_ = s_m
                        else:
                            ifKurzKurz = True
                            if n == len(buchst):
                                buchst2: list = [
                                    a
                                    if a != self.i18n.befehle2["p"]
                                    else self.i18n.befehle2["mulpri"]
                                    for a in buchst
                                ]
                                textDazu += buchst2 + [str(s_[n:])]
                            if (
                                setTextLenIs1
                                and len(buchst) == 0
                                and promptMode2 != PromptModus.AusgabeSelektiv
                            ):
                                textDazu += [
                                    self.i18n.befehle2["mulpri"],
                                    self.i18n.befehle2["a"],
                                    self.i18n.befehle2["t"],
                                    self.i18n.befehle2["w"],
                                    self.i18n.befehle2[
                                        "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
                                    ],
                                ]
                                if any(("/" in _s_ for _s_ in stext)):
                                    textDazu += [
                                        self.i18n.befehle2["u"],
                                        self.i18n.befehle2["B"],
                                        self.i18n.befehle2["G"],
                                        self.i18n.befehle2["E"],
                                        self.i18n.befehle2["groesse"],
                                    ]
                                if "-" + self.i18n.retaPrompt.retaPromptParameter["e"] in sys.argv:
                                    textDazu += [
                                        "-" + self.i18n.mainParaCmds["ausgabe"],
                                        "--" + self.i18n.ausgabeParas["keineueberschriften"],
                                    ]
            else:
                if self.i18n.befehle2["ee"] == s_:
                    textDazu += [
                        "-" + self.i18n.mainParaCmds["ausgabe"],
                        "--" + self.i18n.ausgabeParas["keineueberschriften"],
                    ]
                else:
                    textDazu += [s_]

            if len(textDazu) > 0:
                stext3 += textDazu
            else:
                stext3 += [str(s_)]
        for jjj, _s_ in enumerate(copy(stext3)):
            if len(_s_) > 0 and _s_[0] == "(" and _s_[-1] == ")":
                stext3[jjj] = "[" + stext3[jjj][1:-1] + "]"
        if stext[0] not in [
            "reta",
            self.i18n.befehle2["shell"],
            self.i18n.befehle2["python"],
        ]:
            stext = stext3
        return ifKurzKurz, stext


_PROMPT_LANGUAGE_CACHE: MutableMapping[Tuple[str, str], PromptLanguageBundle] = {}


def bootstrap_prompt_language(
    repo_root: Optional[Path] = None,
    architecture: Optional[RetaArchitecture] = None,
    i18n=None,
    force_rebuild: bool = False,
) -> PromptLanguageBundle:
    architecture = architecture or RetaArchitecture.bootstrap(repo_root)
    if i18n is None:
        import i18n.words_runtime as i18n  # noqa: WPS433

    cache_key = (
        str(architecture.repo_root.resolve()),
        str(getattr(i18n, "__name__", type(i18n).__name__)),
    )
    if force_rebuild or cache_key not in _PROMPT_LANGUAGE_CACHE:
        prompt_runtime = bootstrap_prompt_runtime(
            repo_root=architecture.repo_root,
            architecture=architecture,
            i18n=i18n,
        )
        completion_runtime = bootstrap_completion_runtime(
            repo_root=architecture.repo_root,
            architecture=architecture,
            i18n=i18n,
        )
        _PROMPT_LANGUAGE_CACHE[cache_key] = PromptLanguageBundle(
            architecture=architecture,
            i18n=i18n,
            prompt_runtime=prompt_runtime,
            completion_runtime=completion_runtime,
            not_parameter_values=list(prompt_runtime.vocabulary.not_parameter_values),
            gebrochen_erlaubte_zahlen=set(prompt_runtime.vocabulary.gebrochen_erlaubte_zahlen),
            wahl15=dict(i18n.wahl15),
            wahl16=dict(i18n.wahl16),
        )
    return _PROMPT_LANGUAGE_CACHE[cache_key]


def _default_prompt_language() -> PromptLanguageBundle:
    return bootstrap_prompt_language()


def isReTaParameter(text: str) -> bool:
    return _default_prompt_language().isReTaParameter(text)


def is15or16command(text: str) -> bool:
    return _default_prompt_language().is15or16command(text)


def stextFromKleinKleinKleinBefehl(promptMode2, stext, textDazu):
    return _default_prompt_language().stextFromKleinKleinKleinBefehl(promptMode2, stext, textDazu)


def verifyBruchNganzZahlCommaList(
    bruchAndGanzZahlEtwaKorrekterBereich,
    bruchBereichsAngabe,
    bruchBereichsAngaben,
    bruchRange,
    bruchRanges,
    commaListe,
    zahlenAngaben_,
):
    return _default_prompt_language().verifyBruchNganzZahlCommaList(
        bruchAndGanzZahlEtwaKorrekterBereich,
        bruchBereichsAngabe,
        bruchBereichsAngaben,
        bruchRange,
        bruchRanges,
        commaListe,
        zahlenAngaben_,
    )


def verifyBruchNganzZahlBetweenCommas(
    bruchAndGanzZahlEtwaKorrekterBereich,
    bruchBereichsAngabe,
    bruchBereichsAngaben,
    bruchRange,
    bruchRanges,
    etwaBruch,
    zahlenAngaben_,
):
    return _default_prompt_language().verifyBruchNganzZahlBetweenCommas(
        bruchAndGanzZahlEtwaKorrekterBereich,
        bruchBereichsAngabe,
        bruchBereichsAngaben,
        bruchRange,
        bruchRanges,
        etwaBruch,
        zahlenAngaben_,
    )
