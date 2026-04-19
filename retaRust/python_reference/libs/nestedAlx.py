"""
Nestedcompleter for completion of hierarchical data structures.
"""
import sys

sys.path.insert(0, "libs")
import difflib
from enum import Enum
from typing import Any, Dict, Iterable, Mapping, Optional, Set, Union

from center import i18n
from LibRetaPrompt import (
    PromptModus,
    ausgabeArt,
    ausgabeParas,
    befehle,
    befehle2,
    hauptForNeben,
    hauptForNebenSet,
    isZeilenBruchOrGanzZahlAngabe,
    kombiMainParas,
    mainParas,
    reta,
    retaProgram,
    spalten,
    spaltenDict,
    stextFromKleinKleinKleinBefehl,
    zeilenParas,
    zeilenTypen,
    zeilenTypenB,
    zeilenZeit,
)

# from baseAlx import WordCompleter
# from completionAlx import Completion
from prompt_toolkit.completion import (
    CompleteEvent,
    Completer,
    Completion,
    FuzzyWordCompleter,
)
from prompt_toolkit.document import Document

# from prompt_toolkit.completion.word_completer import WordCompleter
from word_completerAlx import WordCompleter

__all__ = ["NestedCompleter"]
hundert = list((str(n) for n in range(100)))
hundert2 = list((str(n) for n in range(10, 100)))

# NestedDict = Mapping[str, Union['NestedDict', Set[str], None, Completer]]
NestedDict = Mapping[str, Union[Any, Set[str], None, Completer]]

ifRetaAnfang = False


def sort_key(key):
    if not key.startswith("1") and not any((key[:3] == a[:3] for a in i18n.EIGS_N_R)):
        if key in (
            i18n.befehle2["absicht"],
            i18n.befehle2["hilfe"],
            i18n.befehle2["kurzbefehle"],
        ) and len(key) != 1:
            return (0, key)
        elif key in (
            i18n.befehle2["universum"],
            i18n.befehle2["thomas"],
            i18n.befehle2["befehle"],
            i18n.befehle2["groesse"],
        ) and len(key) != 1:
            return (1, key)
        elif key in (
            i18n.befehle2["reta"],
            i18n.befehle2["bewusstsein"],
            i18n.befehle2["geist"],
            i18n.befehle2["emotion"],
            i18n.befehle2["impulse"],
        ) and len(key) != 1:
            return (2, key)
        elif key in (
            i18n.befehle2["loggen"],
            i18n.befehle2["nichtloggen"],
            i18n.befehle2["exit"],
            i18n.befehle2["quit"],
        ) and len(key) != 1:
            return (3, key)
        elif len({key} & set(i18n.befehle2.values())) > 0 and not (key[:3] not in ("15_","16_",i18n.EIGS_N_R[1][:3]),i18n.EIGS_N_R[0][:3]) and len(key) != 1:
            return (4, key)
        elif key in i18n.befehle and len(key) != 1:
            return (5, key)
        elif not key.startswith("1") and len(key) != 1:
            return (6, key)
        elif len(key.strip()) == 1:
            return (7, key)
        elif key[:3] == "15_" or key == "15":
            return (8, key)
        elif key[:3] == "16_" or key == "16":
            return (9, key)
        elif key.startswith("1") and len(key) != 1:
            if key.startswith("15"):
                return (10, key)
            elif key.startswith("16"):
                return (11, key)
            else:
                return (11, key)
        elif any((key[:3] == a[:3] for a in i18n.EIGS_N_R)):
            return (19, key)
        else:
            return (13, key)
    elif any((key[:3] == a[:3] for a in i18n.EIGS_N_R)):
        return (19, key)
    else:
        return (14, key)


befehle2 = set(sorted(befehle2, key=lambda item: sort_key(item)))
befehle = list(sorted(befehle, key=lambda item: sort_key(item)))
befehle2List = list(sorted(befehle2, key=lambda item: sort_key(item)))


class ComplSitua(Enum):
    hauptPara = 0
    zeilenPara = 1
    value = 3
    neitherNor = 4
    retaAnfang = 5
    unbekannt = 6
    spaltenPara = 7
    komiPara = 8
    kombiMetaPara = 9
    ausgabePara = 10
    spaltenValPara = 11
    zeilenValPara = 12
    kombiValPara = 13
    ausgabeValPara = 14
    befehleNichtReta = 15


class NestedCompleter(Completer):
    """
    Completer which wraps around several other completers, and calls any the
    one that corresponds with the first word of the input.

    By combining multiple `NestedCompleter` instances, we can achieve multiple
    hierarchical levels of autocompletion. This is useful when `WordCompleter`
    is not sufficient.

    If you need multiple levels, check out the `from_nested_dict` classmethod.
    """

    def __init__(
        self,
        options: Dict[str, Optional[Completer]],
        optionsStandard: Dict[str, Optional[Completer]],
        situation: ComplSitua,
        lastString: str,
        optionsTypes: Dict[str, Optional[ComplSitua]],
        ignore_case: bool = True,
    ) -> None:
        self.options2 = optionsStandard
        self.options1 = options
        self.options = {**options, **optionsStandard}
        self.ignore_case = ignore_case
        self.ExOptions: dict = {}
        self.ifGleichheitszeichen = False
        self.optionsPark: Dict[str, Optional[Completer]] = {}
        self.situationsTyp: Optional[ComplSitua] = situation
        self.lastString: str = lastString
        self.optionsTypes: Dict[str, Optional[ComplSitua]] = optionsTypes
        self.spaltenParaWort: Optional[str] = "  "
        self.kombiParaWort: Optional[str] = "  "
        self.ausgabeParaWort: Optional[str] = "  "
        self.zeilenParaWort: Optional[str] = "  "
        self.nebenParaWort: Optional[str] = "  "
        self.lastCommands: set = set()

    def optionsSync(
        self,
    ):
        self.options = {**self.options1, **self.options2}

    def __repr__(self) -> str:
        return "NestedCompleter(%r, ignore_case=%r)" % (self.options, self.ignore_case)

    completers: set = set()

    def __eq__(self, obj) -> bool:
        return self.options.keys() == obj.options.keys()

    def __hash__(self):
        return hash(str(self.options.keys()))

    def matchTextAlx(
        self, first_term: str, trennzeichen: str = " "
    ) -> Optional[Completer]:
        result = None
        for i in range(len(first_term), -1, -1):
            result = self.options.get(first_term[:i])
            if result is not None:
                break
        if result is None:
            result = NestedCompleter({}, {}, self.situationsTyp, first_term, {})
            self.__setOptions(result, first_term, trennzeichen)
        return result

    def __setOptions(
        self, completer: "NestedCompleter", first_term: str, trennzeichen: str
    ):
        global ifRetaAnfang
        gleich = trennzeichen == "=" and self.situationsTyp in (
            ComplSitua.spaltenPara,
            ComplSitua.zeilenPara,
            ComplSitua.komiPara,
            ComplSitua.ausgabePara,
        )
        komma = trennzeichen == "," and self.situationsTyp in (
            ComplSitua.spaltenValPara,
            ComplSitua.zeilenValPara,
            ComplSitua.kombiValPara,
            ComplSitua.ausgabeValPara,
        )
        self.lastCommands |= {first_term}
        # print([first_term in hauptForNeben, completer.lastString in hauptForNeben])
        if trennzeichen == " ":
            if self.situationsTyp == ComplSitua.retaAnfang and first_term == "reta":
                ifRetaAnfang = True
                completer.options = {key: None for key in hauptForNeben}
                completer.optionsTypes = {
                    key: ComplSitua.hauptPara for key in hauptForNeben
                }
                completer.lastString = first_term
                completer.situationsTyp = ComplSitua.hauptPara
            elif (
                self.situationsTyp
                in (
                    ComplSitua.retaAnfang,
                    ComplSitua.befehleNichtReta,
                )
                and first_term not in hauptForNeben
            ):
                if (
                    (
                        len(self.lastCommands & befehle2) > 0
                        and any(
                            [
                                isZeilenBruchOrGanzZahlAngabe(txt)
                                for txt in self.lastCommands
                            ]
                        )
                    )
                    or stextFromKleinKleinKleinBefehl(
                        PromptModus.normal, list(self.lastCommands), []
                    )[0]
                    or not ifRetaAnfang
                ):
                    liste = befehle2List + hauptForNeben
                else:
                    liste = befehle2List
                    ifRetaAnfang = False
                completer.options = {key: None for key in liste}
                completer.optionsTypes = {
                    key: ComplSitua.befehleNichtReta for key in liste
                }
                completer.lastString = first_term
                completer.situationsTyp = ComplSitua.befehleNichtReta

            else:
                if len({first_term, self.nebenParaWort} & hauptForNebenSet) > 0:
                    if "-" + i18n.hauptForNeben["zeilen"] == first_term:
                        var1, var2 = self.paraZeilen(completer)
                    elif "-" + i18n.hauptForNeben["spalten"] == first_term:
                        var1, var2 = self.paraSpalten(completer)
                    elif "-" + i18n.hauptForNeben["ausgabe"] == first_term:
                        var1, var2 = self.paraAusgabe(completer)
                    elif "-" + i18n.hauptForNeben["kombination"] == first_term:
                        var1, var2 = self.paraKombination(completer)
                    elif "-" + i18n.hauptForNeben["zeilen"] == self.nebenParaWort:
                        var1, var2 = self.paraZeilen(completer)
                    elif "-" + i18n.hauptForNeben["spalten"] == self.nebenParaWort:
                        var1, var2 = self.paraSpalten(completer)
                    elif "-" + i18n.hauptForNeben["ausgabe"] == self.nebenParaWort:
                        var1, var2 = self.paraAusgabe(completer)
                    elif "-" + i18n.hauptForNeben["kombination"] == self.nebenParaWort:
                        var1, var2 = self.paraKombination(completer)

                    if not ifRetaAnfang:
                        try:
                            var1 += befehle2List
                        except:
                            var1 = []
                    try:
                        var1
                    except UnboundLocalError:
                        var1 = []
                    completer.options = {key: None for key in var1}
                    completer.optionsTypes = {key: var2 for key in var1}
                    completer.lastString = first_term

                    completer.nebenParaWort = (
                        first_term
                        if first_term in hauptForNeben
                        else self.nebenParaWort
                    )
                    if first_term not in hauptForNeben:
                        completer.options = {
                            **completer.options,
                            **{key: None for key in hauptForNeben},
                        }
                        completer.optionsTypes = {
                            **completer.optionsTypes,
                            **{key: ComplSitua.hauptPara for key in hauptForNeben},
                        }
        elif gleich or komma:
            if "-" + i18n.hauptForNeben["spalten"] == first_term:
                var2, var3, var4 = self.gleichKommaSpalten(
                    completer, first_term, gleich, komma
                )
            elif "-" + i18n.hauptForNeben["zeilen"] == first_term:
                var2, var3, var4 = self.gleichKommaZeilen(
                    completer, first_term, gleich, komma
                )
            elif "-" + i18n.hauptForNeben["kombination"] == first_term:
                var2, var3, var4 = self.gleichKommaKombi(
                    completer, first_term, gleich, komma
                )
            elif "-" + i18n.hauptForNeben["ausgabe"] == first_term:
                var2, var3, var4 = self.gleichKommaAusg(
                    completer, first_term, gleich, komma
                )
            elif "-" + i18n.hauptForNeben["spalten"] == self.nebenParaWort:
                var2, var3, var4 = self.gleichKommaSpalten(
                    completer, first_term, gleich, komma
                )
            elif "-" + i18n.hauptForNeben["zeilen"] == self.nebenParaWort:
                var2, var3, var4 = self.gleichKommaZeilen(
                    completer, first_term, gleich, komma
                )
            elif "-" + i18n.hauptForNeben["kombination"] == self.nebenParaWort:
                var2, var3, var4 = self.gleichKommaKombi(
                    completer, first_term, gleich, komma
                )
            elif "-" + i18n.hauptForNeben["ausgabe"] == self.nebenParaWort:
                var2, var3, var4 = self.gleichKommaAusg(
                    completer, first_term, gleich, komma
                )
            else:
                var3 = ""
                var4 = {}
            suchWort = (
                first_term[2:]
                if gleich
                else var3[2:]
                if komma and var3 is not None
                else 0
            )
            try:
                var1 = var4[suchWort]
            except KeyError:
                var1 = difflib.get_close_matches(suchWort, var4.keys())
            completer.options = {key: None for key in var1}
            completer.optionsTypes = {key: var2 for key in var1}
            completer.lastString = first_term
            completer.nebenParaWort = self.nebenParaWort

    def gleichKommaKombi(self, completer, first_term, gleich, komma):
        completer.kombiParaWort = (
            first_term if gleich else self.kombiParaWort if komma else None
        )
        a = [
                item
                for sublist in retaProgram.kombiParaNdataMatrix.values()
                for item in sublist
            ]
        b = [
                item
                for sublist in retaProgram.kombiParaNdataMatrix2.values()
                for item in sublist
            ]
        var4 = {
            i18n.kombiMainParas["galaxie"]: a,
            i18n.kombiMainParas["universum"]: b,
            "*": a+b
        }
        var2 = ComplSitua.kombiValPara
        var3 = self.kombiParaWort
        completer.situationsTyp = ComplSitua.kombiValPara
        return var2, var3, var4

    def gleichKommaZeilen(self, completer, first_term, gleich, komma):
        completer.zeilenParaWort = (
            first_term if gleich else self.zeilenParaWort if komma else None
        )
        var4 = {key: hundert for key in i18n.zeilenParas if "--"+key+"=" in zeilenParas}
        var4.update({key: [""] for key in i18n.zeilenParas if "--"+key+"=" not in zeilenParas})
        var4[i18n.zeilenParas["typ"]] = zeilenTypen + ["-" + t for t in zeilenTypen]
        var4[i18n.zeilenParas["primzahlen"]] = zeilenTypenB + [
            "-" + t for t in zeilenTypenB
        ]
        var4[i18n.zeilenParas["zeit"]] = zeilenZeit + ["-" + t for t in zeilenZeit]
        var4.update({"*": var4[i18n.zeilenParas["typ"]] + var4[i18n.zeilenParas["primzahlen"]] + var4[i18n.zeilenParas["zeit"]]})
        #var4.setdefault(hundert)

        var2 = ComplSitua.zeilenPara
        var3 = self.zeilenParaWort
        completer.situationsTyp = ComplSitua.zeilenValPara
        return var2, var3, var4

    def gleichKommaAusg(self, completer, first_term, gleich, komma):
        completer.ausgabeParaWort = (
            first_term if gleich else self.ausgabeParaWort if komma else None
        )
        var4 = {"*": ausgabeArt,
                i18n.ausgabeParas["breite"]: hundert2,
                i18n.ausgabeParas["breiten"]: hundert2}
        var4[i18n.ausgabeParas["art"]] = ausgabeArt
        #var4.setdefault(hundert)
        var2 = ComplSitua.ausgabeValPara
        var3 = self.ausgabeParaWort
        completer.situationsTyp = ComplSitua.ausgabeValPara
        return var2, var3, var4

    def gleichKommaSpalten(self, completer, first_term, gleich, komma):
        completer.spaltenParaWort = (
            first_term if gleich else self.spaltenParaWort if komma else None
        )
        var4 = spaltenDict
        var4.update({i18n.ausgabeParas["breite"]: hundert2,
                i18n.ausgabeParas["breiten"]: hundert2})
        var2 = ComplSitua.spaltenValPara
        var3 = self.spaltenParaWort
        completer.situationsTyp = ComplSitua.spaltenValPara
        return var2, var3, var4

    def paraKombination(self, completer):
        var1 = kombiMainParas
        var2 = ComplSitua.kombiValPara
        completer.situationsTyp = ComplSitua.komiPara
        return var1, var2

    def paraAusgabe(self, completer):
        var1 = ausgabeParas
        var2 = ComplSitua.ausgabeValPara
        completer.situationsTyp = ComplSitua.ausgabePara
        return var1, var2

    def paraSpalten(self, completer):
        var1 = spalten
        var2 = ComplSitua.spaltenValPara
        completer.situationsTyp = ComplSitua.spaltenPara
        return var1, var2

    def paraZeilen(self, completer):
        var1 = zeilenParas
        var2 = ComplSitua.zeilenValPara
        completer.situationsTyp = ComplSitua.zeilenPara
        return var1, var2

    def get_completions(
        self, document: Document, complete_event: CompleteEvent
    ) -> Iterable[Completion]:
        # Split document.
        text = document.text_before_cursor.lstrip()
        stripped_len = len(document.text_before_cursor) - len(text)

        # If there is a space, check for the first term, and use a
        # subcompleter.
        gleich: bool = "=" in text
        komma: bool = "," in text
        if " " in text:
            first_term = text.split()[0]
            # completer = self.options.get(first_term)
            completer = self.matchTextAlx(first_term)

            # If we have a sub completer, use this for the completions.
            if completer is not None:
                # if self.ifGleichheitszeichen:
                #    completer.options = completer.optionsPark
                # self.ifGleichheitszeichen = False
                # if "=" not in text and len(completer.ExOptions) != 0:
                #    for key, val in completer.ExOptions.items():
                #        completer.options[key] = val
                remaining_text = text[len(first_term) :].lstrip()
                move_cursor = len(text) - len(remaining_text) + stripped_len

                new_document = Document(
                    remaining_text,
                    cursor_position=document.cursor_position - move_cursor,
                )

                for c in completer.get_completions(new_document, complete_event):
                    yield c

        elif gleich or komma:
            text = str(text)
            first_term = text.split("=" if gleich else ",")[0]
            # completer = self.options.get(first_term)
            completer = self.matchTextAlx(first_term, "=" if gleich else ",")

            # If we have a sub completer, use this for the completions.
            if completer is not None:
                # self.ifGleichheitszeichen = True
                # completer.optionsPark = completer.options
                # completer.options = completer.options2
                # ES SIND EINFACH ZU VIELE, D.H.: ANDERS LÖSEN!
                # ES GIBT AB SOFORT 2 options DATENSTRUKTUREN!
                # for notParaVal in self.notParameterValues:
                #    if notParaVal in completer.options:
                #        completer.ExOptions[notParaVal] = completer.options.pop(
                #            notParaVal, None
                #        ||)
                remaining_text = text[len(first_term) + 1 :].lstrip()
                move_cursor = len(text) - len(remaining_text) + stripped_len

                new_document = Document(
                    remaining_text,
                    cursor_position=document.cursor_position - move_cursor,
                )

                for c in completer.get_completions(new_document, complete_event):
                    yield c

        # No space in the input: behave exactly like `WordCompleter`.
        else:
            # completer = WordCompleter(
            #    list(self.options.keys()), ignore_case=self.ignore_case
            # )
            completer = FuzzyWordCompleter(list(self.options.keys()))

            document._text = document._text
            if self.ifGleichheitszeichen:
                completer.options = completer.optionsPark
            self.ifGleichheitszeichen = False
            for c in completer.get_completions(document, complete_event):
                yield c
