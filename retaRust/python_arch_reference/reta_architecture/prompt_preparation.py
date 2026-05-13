from __future__ import annotations

import re
from copy import copy
from dataclasses import dataclass
from pathlib import Path

from .runtime_compat import BereichToNumbers2, i18n, isZeilenAngabe, teiler

from .prompt_execution import vorherVonAusschnittOderZaehlung
from .prompt_language import PromptModus, custom_split, stextFromKleinKleinKleinBefehl
from .prompt_session import PromptTextState


i18nRP = i18n.retaPrompt
promptSession = None
retaProgram = None
befehleBeenden = set()


def configure_prompt_preparation(*, prompt_runtime=None, prompt_session=None, i18n_obj=None) -> None:
    """Wire legacy-compatible prompt preparation helpers to explicit runtime bundles."""
    global retaProgram, promptSession, befehleBeenden, i18nRP
    source_i18n = i18n_obj or getattr(prompt_session, "i18n", i18n)
    i18nRP = source_i18n.retaPrompt
    if prompt_runtime is not None:
        retaProgram = prompt_runtime.program
    if prompt_session is not None:
        promptSession = prompt_session
    befehleBeenden = set(source_i18n.retaPrompt.befehleBeenden)


@dataclass(frozen=True)
class PromptPreparationBundle:
    """Morphism layer from prompt text/tokens to normalized Reta command inputs."""

    command_rotator: object
    regex_rewriter: object
    output_preparer: object

    def snapshot(self):
        return {
            "class": type(self).__name__,
            "command_rotator": getattr(self.command_rotator, "__name__", str(self.command_rotator)),
            "regex_rewriter": getattr(self.regex_rewriter, "__name__", str(self.regex_rewriter)),
            "output_preparer": getattr(self.output_preparer, "__name__", str(self.output_preparer)),
            "cached_parameter_value_domains": {key: len(value) for key, value in spaltenParaNvalueS.items()},
            "beenden_commands_len": len(befehleBeenden),
        }

    def rotate_where_reta_command(self, *args, **kwargs):
        return self.command_rotator(*args, **kwargs)

    def regex_replace(self, *args, **kwargs):
        return self.regex_rewriter(*args, **kwargs)

    def prepare_grosse_ausgabe(self, *args, **kwargs):
        return self.output_preparer(*args, **kwargs)


def bootstrap_prompt_preparation(
    *,
    architecture=None,
    repo_root: Path | None = None,
    i18n=None,
    prompt_runtime=None,
    prompt_session=None,
    force_rebuild: bool = False,
) -> PromptPreparationBundle:
    if repo_root is None:
        repo_root = Path(__file__).resolve().parent.parent
    if prompt_runtime is None:
        if architecture is not None and hasattr(architecture, "bootstrap_prompt_runtime"):
            prompt_runtime = architecture.bootstrap_prompt_runtime(i18n=i18n, force_rebuild=force_rebuild)
        else:
            from .prompt_runtime import bootstrap_prompt_runtime

            prompt_runtime = bootstrap_prompt_runtime(repo_root=repo_root, i18n=i18n, force_rebuild=force_rebuild)
    if prompt_session is None:
        if architecture is not None and hasattr(architecture, "bootstrap_prompt_session"):
            prompt_session = architecture.bootstrap_prompt_session(i18n=i18n, force_rebuild=force_rebuild)
        else:
            from .prompt_session import bootstrap_prompt_session

            prompt_session = bootstrap_prompt_session(repo_root=repo_root, i18n=i18n, force_rebuild=force_rebuild)
    configure_prompt_preparation(
        prompt_runtime=prompt_runtime,
        prompt_session=prompt_session,
        i18n_obj=i18n,
    )
    return PromptPreparationBundle(
        command_rotator=verdreheWoReTaBefehl,
        regex_rewriter=regExReplace,
        output_preparer=promptVorbereitungGrosseAusgabe,
    )


def verdreheWoReTaBefehl(text1: str, text2: str, text3: str, PromptMode: PromptModus):
    if text2[:4] == "reta" and text1[:4] != "reta" and len(text3) > 0:
        return text2, text1, custom_split(text2)
    return text1, text2, text3

spaltenParaNvalueS: dict = {"zeilen": {},"spalten": {}, "ausgabe": {}, "kombination": {}}

def regExReplace(Txt) -> list:
    if not any(("r\"" in a or "*" in a for a in Txt.menge)):
        return Txt.liste
    ifReta: bool = True if Txt.liste[:1] == ["reta"] else False
    neueListe: list = []
    foundParas4value: list = []
    i: int = -1
    regexAufgeloest = False
    changedAtAll = False
    def lastRetaHauptPara() -> str:
        for el in reversed(neueListe):
            if el[:1] == "-" and el[:2] != "--":
                try:
                    return el[1:]
                except:
                    return ""
        return ""
    def allEqSignAbarbeitung(foundParas4value, hauptCmd,onlyGen = False, eqThing=""):
        spaltenParaNvalue: dict = {}
        newTokens: list = []
        if hauptCmd == i18n.hauptForNeben["spalten"]:
            if len(spaltenParaNvalueS["spalten"]) == 0:
                for liste1 in retaProgram.dataDict[0].values():
                    for liste2 in liste1:
                        for liste3 in liste2:
                            try:
                                spaltenParaNvalue[liste3[0]] |= {liste3[1]}
                            except KeyError:
                                spaltenParaNvalue[liste3[0]] = {liste3[1]}
                spaltenParaNvalueS["spalten"] = spaltenParaNvalue
            else:
                spaltenParaNvalue = spaltenParaNvalueS["spalten"]
        elif hauptCmd == i18n.hauptForNeben["zeilen"]:
            if len(spaltenParaNvalueS["zeilen"]) == 0:
                spaltenParaNvalue = {zeilenPara: {''} for zeilenPara in i18n.haupt2neben[i18n.hauptForNeben["zeilen"]]}
                spaltenParaNvalue[i18n.zeilenParas["zeit"]] = {i18n.zeilenParas["gestern"], i18n.zeilenParas["heute"], i18n.zeilenParas["morgen"]}
                spaltenParaNvalue[i18n.zeilenParas["typ"]] = {i18n.zeilenParas["mond"],
                                                          i18n.zeilenParas["sonne"],
                                                          i18n.zeilenParas["planet"],
                                                          i18n.zeilenParas["schwarzesonne"],
                                                          i18n.zeilenParas["SonneMitMondanteil"]}
                spaltenParaNvalue[i18n.zeilenParas["primzahlen"]] = {i18n.zeilenParas["aussenerste"], i18n.zeilenParas["innenerste"], i18n.zeilenParas["innenalle"], i18n.zeilenParas["aussenalle"]}
                spaltenParaNvalueS["zeilen"] = spaltenParaNvalue
            else:
                spaltenParaNvalue = spaltenParaNvalueS["zeilen"]
        elif hauptCmd == i18n.hauptForNeben["kombination"]:
            if len(spaltenParaNvalueS["kombination"]) == 0:
                spaltenParaNvalue = {i18n.kombiMainParas["galaxie"]: {text for tupel in i18n.kombiParaNdataMatrix.values() for text in tupel}, i18n.kombiMainParas["universum"]: {text for tupel in i18n.kombiParaNdataMatrix2.values() for text in tupel}}
                spaltenParaNvalueS["kombination"] = spaltenParaNvalue
            else:
                spaltenParaNvalue = spaltenParaNvalueS["kombination"]
        elif hauptCmd == i18n.hauptForNeben["ausgabe"]:
            if len(spaltenParaNvalueS["ausgabe"]) == 0:
                spaltenParaNvalue = {ausgabePara: {''} for ausgabePara in i18n.haupt2neben[i18n.hauptForNeben["ausgabe"]]}
                spaltenParaNvalue[i18n.ausgabeParas["art"]] = set(i18n.ausgabeArt.keys())
                eqAusgabeParas = i18n.nested.artWort
                spaltenParaNvalueS["ausgabe"] = spaltenParaNvalue
            else:
                spaltenParaNvalue = spaltenParaNvalueS["ausgabe"]
        if onlyGen:
            return
        if i == 0:
            for para4value in spaltenParaNvalue.keys():
                if any(re.findall(regex, para4value)) or any(re.findall(regex, para4value+"=")):
                    try:
                        foundParas4value += [para4value]
                    except NameError:
                        foundParas4value: list = [para4value]
        elif i == 1:
            if len(eqThing) > 0 and not all([spaltenParaNvalue[a]=={''} for a in foundParas4value]):
                found2: dict = {}
                for found in foundParas4value:
                    passend = [a for a in list(set(spaltenParaNvalue[found]) | set(eqThing)) if len(a) > 1]
                    if len(passend) > 0:
                        found2[found] = passend
                for key, values in found2.items():
                    try:
                        if eqThing in spaltenParaNvalue[key]:
                            newTokens += ["".join(("--",key, "=", eqThing))]
                    except KeyError:
                        pass
            else:
                for para4value in foundParas4value:
                    try:
                        if spaltenParaNvalue[para4value] == {''}:
                            if len(eqThing) > 0:
                                    newTokens += ["".join(("--",para4value, "=", eqThing))]
                            elif all([spaltenParaNvalue[a]=={''} for a in foundParas4value]):
                                newTokens += ["--"+para4value]
                        else:
                            for values4para in spaltenParaNvalue[para4value]:
                                if any(re.findall(regex, values4para)) or any(re.findall(regex, "="+values4para)):
                                    newTokens += ["".join(("--",para4value, "=", values4para))]
                    except KeyError:
                        pass
                foundParas4value = []
        elif i == -1:
            for para4value in [para4value for para4value, value in spaltenParaNvalue.items() if value == {''}]:
                if any(re.findall(regex, para4value)) or any(re.findall(regex, "--"+para4value)):
                    newTokens += ["--"+para4value]
            for haupt in i18n.hauptForNeben.values():
                if any(re.findall(r""+regex, haupt)) or any(re.findall(r""+regex, "-"+haupt)):
                    newTokens += ["-"+haupt]

        regexAufgeloest = True
        return newTokens
    def findregEx(regex, foundParas4value: list = [], eqThing="") -> list:
        def immerHauptParaAbarbeitung(newTokens):
            for haupt in i18n.hauptForNeben.values():
                if (any(re.findall(r""+regex, haupt)) or any(re.findall(r""+regex, "-"+haupt))) and "-"+haupt not in newTokens:
                    newTokens += ["-"+haupt]
            regexAufgeloest = True

        allResultTokens: list = []
        newTokens: list = []
        if ifReta:
            if len(neueListe) > 0:
                if hauptCmd in (i18n.hauptForNeben["kombination"],i18n.hauptForNeben["zeilen"], i18n.hauptForNeben["spalten"], i18n.hauptForNeben["ausgabe"]):
                    newTokens = allEqSignAbarbeitung(foundParas4value, hauptCmd, False, eqThing)
                if len(foundParas4value) == 0 and len(newTokens) == 0:
                    immerHauptParaAbarbeitung(newTokens)
            else:
                return []
        else:
            for el in (cmd for cmd in i18n.befehle2.values() if len(cmd) > 1):
                if any(re.findall(regex, el)):
                    newTokens += [el]
        return newTokens

    for listenToken in Txt.liste:
        eqThings2 = listenToken.split("=")
        hauptCmd = lastRetaHauptPara()
        if len(eqThings2) > 2:
            eqThings2 = [eqThings2[0]] + ["=".join(eqThings2[1:])]
        if len(eqThings2) == 2:
            eqThings: list = [""]
            flag = False
            for i, eqThing7 in enumerate(eqThings2):
                eqThings3 = []
                for eqThing in eqThing7.split(",") if i == 1 else [eqThing7]:
                    if eqThing == "*" or (eqThing in ("--*","--") and i == 0):
                        eqThing = "r\"(.*)\""
                    if eqThing[:2] == "r\"" and eqThing[-1] == "\"":
                        regex = r""+eqThing[2:-1]
                        eqThings3 += findregEx(regex, foundParas4value)
                        flag = True
                        changedAtAll = True
                    else:
                        if flag:
                            eqThings += findregEx(None, foundParas4value, eqThing)
                        else:
                            if "=" not in eqThings[-1]:
                                eqThings[-1] += eqThing + "="
                            else:
                                eqThings[-1] += eqThing
                            foundParas4value += [eqThing[2:]]
                if len(eqThings3) > 0:
                    eqThings += eqThings3
            foundParas4value = []
            neueListe += [" ".join(eqThings)]
        elif listenToken[:2] == "r\"" and listenToken[-1] == "\"":
            regex = r""+listenToken[2:-1]
            i = -1
            neueListe += findregEx(regex)
        else:
            neueListe += [listenToken]
    if changedAtAll:
        neueNeueListe: list = []
        eqThings1: list = []
        eqThings2 = []
        def aufloesen(neueNeueListe, eqThings1, eqThings2, eqWo, ifEnde) -> tuple:
            if len(eqThings1) > 1:
                neueNeueListe += [eqThings1[:-1][0]]
                bis = eqThings2
                neueNeueListe[-1] += ",".join((a for a in bis if len(a) != 0))
            elif len(eqThings1) == 1:
                neueNeueListe += [eqThings1[0]+eqThings2[0]]
            if eqWo == 0:
                eqThings1, eqThings2 = [], []
            else:
                if ifEnde:
                    neueNeueListe += [a+b for a, b in zip(eqThings1, eqThings2)]
                    eqThings1 = []
                    eqThings2 = []

            return neueNeueListe, eqThings1, eqThings2

        neueListe=(" ".join(neueListe)).split()
        for i, n in enumerate(neueListe):
            ifEnde = i+1 == len(neueListe)
            eqWo = n.find("=")+1 # sucht nur nach ErstVorkommen!
            if eqWo != 0:
                if len(eqThings1) == 0:
                    if ifEnde:
                        neueNeueListe += [n]
                    else:
                        eqThings1 += [n[:eqWo]]
                        eqThings2 += [n[eqWo:]]
                elif all((n[:eqWo] == a for a in eqThings1)):
                    eqThings1 += [n[:eqWo]]
                    eqThings2 += [n[eqWo:]]
                    if ifEnde:
                        neueNeueListe += [eqThings1[:-1][0]]
                        neueNeueListe[-1] += ",".join((a for a in eqThings2 if len(a) != 0))
                else:
                    neueNeueListe, eqThings1, eqThings2 = aufloesen(neueNeueListe, eqThings1, eqThings2, eqWo, ifEnde)
                    eqThings1 = [n[:eqWo]]
                    eqThings2 = [n[eqWo:]]
            else:
                neueNeueListe, eqThings1, eqThings2 = aufloesen(neueNeueListe, eqThings1, eqThings2, eqWo, ifEnde)
                neueNeueListe += [n]
        neueListe = neueNeueListe
    if not ifReta and regexAufgeloest:
        print(" ".join(neueListe))
    #exit()
    return neueListe

def promptVorbereitungGrosseAusgabe(
    platzhalter, promptMode, promptMode2, promptModeLast, text, textDazu0
):
    Txt = promptSession.new_text_state(text)
    Txt.platzhalter = platzhalter
    ketten = []
    # AusgabeSelektiv = 5
    ifKurzKurz = False
    if len(Txt.liste) > 0:
        textDazu: list = []
        s_2: list
        ifKurzKurz, Txt.liste = stextFromKleinKleinKleinBefehl(
            promptMode2, Txt.liste, textDazu
        )
    if Txt.liste is not None:
        nstextnum: list = []
        for astext in Txt.liste:
            if astext.isdecimal():
                nstextnum += [int(astext)]
        if len(nstextnum) > 0:
            maxNum = max(nstextnum)
        else:
            maxNum = 1024
    zahlenBereichNeu: map = {}
    zahlenBereichNeu1: map = {}
    for swort in Txt.liste:
        try:
            zahlenBereichNeu1[bool(isZeilenAngabe(swort))] += [swort]
        except KeyError:
            zahlenBereichNeu1[bool(isZeilenAngabe(swort))] = [swort]
    for key, value in zahlenBereichNeu1.items():
        zahlenBereichNeu[key] = ",".join(value)

    zahlenBereichMatch = tuple(zahlenBereichNeu.keys())
    if (
        promptMode2 == PromptModus.AusgabeSelektiv
        and promptModeLast == PromptModus.normal
    ):
        Txt.liste = textDazu0 + Txt.liste
    if (
        promptMode == PromptModus.normal
        and len(Txt.platzhalter) > 1
        and Txt.platzhalter[:4] == "reta"
        and any(zahlenBereichMatch)
        and zahlenBereichMatch.count(True) == 1
    ):
        zeilenn = False
        woerterToDel = []
        for i, wort in enumerate(Txt.liste):
            if len(wort) > 1 and wort[0] == "-" and wort[1] != "-":
                zeilenn = False
            if zeilenn is True or wort == zahlenBereichNeu[True]:
                woerterToDel += [i]
            if wort == "-" + i18n.hauptForNeben["zeilen"]:
                zeilenn = True
                woerterToDel += [i]
        stextDict = {i: swort for i, swort in enumerate(Txt.liste)}
        for todel in woerterToDel:
            del stextDict[todel]
        Txt.liste = list(stextDict.values())

        if len({i18n.befehle2["w"], i18n.befehle2["teiler"]} & Txt.menge) > 0:
            BereichMenge = BereichToNumbers2(zahlenBereichNeu[True], False, 0)
            BereichMengeNeu = teiler(",".join([str(b) for b in BereichMenge]))[1]
            zahlenBereichNeu[True] = ""
            for a in BereichMengeNeu:
                zahlenBereichNeu[True] += str(a) + ","
            zahlenBereichNeu[True] = zahlenBereichNeu[True][:-1]

            try:
                tx = Txt.liste
                tx.remove(i18n.befehle2["w"])
                Txt.liste = tx
            except:
                pass
            try:
                tx = Txt.liste
                tx.remove(i18n.befehle2["teiler"])
                Txt.liste = tx
            except:
                pass

        if len({i18n.befehle2["v"], i18n.befehle2["vielfache"]} & Txt.menge) == 0:
            Txt.liste += [
                "".join(("-", i18n.hauptForNeben["zeilen"])),
                vorherVonAusschnittOderZaehlung(Txt, zahlenBereichNeu[True]),
            ]

        else:
            Txt.liste += [
                "".join(("-", i18n.hauptForNeben["zeilen"])),
                "".join(("--", i18n.zeilenParas["vielfachevonzahlen"], "="))
                + zahlenBereichNeu[True],
            ]
            try:
                tx = Txt.liste
                tx.remove(i18n.befehle2["v"])
                Txt.liste = tx
            except:
                pass
            try:
                tx = Txt.liste
                tx.remove(i18n.befehle2["vielfache"])
                Txt.liste = tx
            except:
                pass
    IsPureOnlyReTaCmd: bool = len(Txt.liste) > 0 and Txt.liste[0] == "reta"
    brueche = []
    zahlenAngaben_ = []
    zahlenAngabenC = ""
    if Txt.hasWithoutABC(set(befehleBeenden)):
        Txt.liste = [tuple(befehleBeenden)[0]]
        exit()
    replacements = i18nRP.replacements
    if len(Txt.liste) > 0 and Txt.liste[0] not in [
        "reta",
        i18n.befehle2["shell"],
        i18n.befehle2["python"],
        i18n.befehle2["abstand"],
    ]:
        listeNeu: list = []
        for token in Txt.liste:
            try:
                listeNeu += [replacements[token]]
            except KeyError:
                listeNeu += [token]
        Txt.liste = listeNeu
    if Txt.liste[:1] != ["reta"]:
        Txt.liste = list(Txt.menge)
    Txt.liste = regExReplace(Txt)
    return (
        IsPureOnlyReTaCmd,
        brueche,
        zahlenAngabenC,
        ketten,
        maxNum,
        Txt.liste,
        zahlenAngaben_,
        ifKurzKurz,
    )

