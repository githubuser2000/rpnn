import os
import re
import sys
from copy import copy, deepcopy
from enum import Enum
from fractions import Fraction
from typing import Optional

import reta

from center import (
    BereichToNumbers2,
    Primzahlkreuz_pro_contra_strs,
    i18n,
    isZeilenAngabe,
    isZeilenAngabe_betweenKommas,
    isZeilenBruchAngabe,
    isZeilenBruchOrGanzZahlAngabe,
    kpattern,
    x,
)

wahl15 = i18n.wahl15
wahl16 = i18n.wahl16

# retaProgram = reta.Program([sys.argv[0], "-" + i18n.retapy.nichtsWort])
retaProgram = reta.Program([sys.argv[0], "-" + i18n.retapy.nichtsWort])
mainParas = ["-" + a for a in retaProgram.mainParaCmds]
spalten = ["--" + a[0] + "=" for a in retaProgram.paraDict.keys()] + ["--="]
eigsN, eigsR = [], []
for pp in retaProgram.paraDict.keys():
    if pp[0] == i18n.konzeptE["konzept"]:
        eigsN += [pp[1]]
    elif pp[0] == i18n.konzeptE["konzept2"]:
        eigsR += [pp[1]]


def custom_split(text):
    stack = []
    result = []
    start = 0  # Start index for each substring
    for i, char in enumerate(text):
        if char in "({[":
            stack.append(char)

        elif char in ")}]":
            if stack:
                stack.pop()
        elif char.isspace() and not stack:
            result.append(text[start:i])
            start = i + 1  # Skip the whitespace character for the next substring
    # Append the last substring if it exists
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


class PromptModus(Enum):
    normal = 0
    speichern = 1
    loeschenStart = 2
    speicherungAusgaben = 3
    loeschenSelect = 4
    speicherungAusgabenMitZusatz = 5
    AusgabeSelektiv = 6


#
#
# DAS SOLLTE ICH BESSER ALLES ORDENTLICH IN RETA.PY PACKEN, STATT ES HIER AUSZUSCHREIBEN, WEIL SONST DOPPELT!
# D.H. spezielle DatenTypen dafür in Reta.py anlegen!
# DAS GEHT SCHNELL, FLEIßARBEIT, WEIL KAUM BUGGEFAHR
#
# startpunkt: dict = {}
spaltenDict = {}
for tupel in retaProgram.paraNdataMatrix:
    for haupt in tupel[0]:
        try:
            spaltenDict[haupt] += list(tupel[1])
        except KeyError:
            spaltenDict[haupt] = list(tupel[1])
        try:
            spaltenDict["*"] += list(tupel[1])
        except KeyError:
            spaltenDict["*"] = list(tupel[1])


spalten += [
    "--" + i18n.ausgabeParas["breite"] + "=",
    "--" + i18n.ausgabeParas["breiten"] + "=",
    "--" + i18n.ausgabeParas["keinenummerierung"],
    "--*=",
]

zeilenTypen = [
    i18n.zeilenParas["sonne"],
    i18n.zeilenParas["mond"],
    i18n.zeilenParas["planet"],
    i18n.zeilenParas["schwarzesonne"],
    i18n.zeilenParas["SonneMitMondanteil"],
    "*",
]
zeilenZeit = [
    i18n.zeilenParas["heute"],
    i18n.zeilenParas["gestern"],
    i18n.zeilenParas["morgen"],
    "*",
]

zeilenTypenB = [
    i18n.zeilenParas["aussenerste"],
    i18n.zeilenParas["innenerste"],
    i18n.zeilenParas["aussenalle"],
    i18n.zeilenParas["innenalle"],
    "*",
]

ausgabeParas = [
    "--" + s + ("=" if l else "")
    for s, l in zip(i18n.ausgabeParas.values(), i18n.ausgabeParasEqSign.values())
] + ["--*="]
# ausgabeParas = [
#    "--nocolor",
#    "--justtext",
#    "--art=",
#    "--onetable",
#    "--spaltenreihenfolgeundnurdiese=",
#    "--endlessscreen",
#    "--endless",
#    "--dontwrap",
#    "--breite=",
#    "--breiten=",
#    "--keineleereninhalte",
#    "--keinenummerierung",
#    "--keineueberschriften",
# ]
kombiMainParas = [
    "--" + i18n.kombiMainParas["galaxie"] + "=",
    "--" + i18n.kombiMainParas["universum"] + "=",
    "--*=",
]
zeilenParas = [
    # "--"+i18n.zeilenParas["nichts"]+"",
    "--" + i18n.zeilenParas["zeit"] + "=",
    "--" + i18n.zeilenParas["zaehlung"] + "=",
    "--" + i18n.zeilenParas["vorhervonausschnitt"] + "=",
    "--" + i18n.zeilenParas["vorhervonausschnittteiler"],
    "--" + i18n.zeilenParas["primzahlvielfache"] + "=",
    "--" + i18n.zeilenParas["nachtraeglichneuabzaehlung"] + "=",
    "--" + i18n.zeilenParas["nachtraeglichneuabzaehlungvielfache"] + "=",
    "--" + i18n.zeilenParas["alles"],
    "--" + i18n.zeilenParas["potenzenvonzahlen"] + "=",
    "--" + i18n.zeilenParas["typ"] + "=",
    "--" + i18n.zeilenParas["vielfachevonzahlen"] + "=",
    "--" + i18n.zeilenParas["oberesmaximum"] + "=",
    "--" + i18n.zeilenParas["primzahlen"] + "=",
    "--" + i18n.zeilenParas["invertieren"],
    "--*=",
]
hauptForNeben = ["-" + s for s in set(i18n.hauptForNeben.values()) - {i18n.mainParaCmds["debug"]}]
# hauptForNeben = ("-zeilen", "-spalten", "-kombination", "-ausgabe", "-h", "-help")

notParameterValues = ausgabeParas + zeilenParas + kombiMainParas + spalten + mainParas
hauptForNebenSet = set(hauptForNeben)

ausgabeArt = list(i18n.ausgabeArt.values())
# ausgabeArt = ["bbcode", "html", "csv", "shell", "markdown", "emacs"]

# wahl15 = {
#    #    "_": "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15)",
#    "_15": "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,"
#    + Primzahlkreuz_pro_contra_strs[1],
#    "_2": "Konkreta_und_Focus_(2)",
#    "_5": "Impulse_(5)",
#    "_7": "Gefühle_(7)",
#    "_8": "Modus_und_Sein_(8)",
#    "_10": "Wirklichkeiten_Wahrheit_Wahrnehmung_(10)",
#    "_12": "Meta-Systeme_(12),Ordnung_und_Filterung_12_und_1pro12",
#    "_13": "Paradigmen_sind_Absichten_(13)",
#    "_17": "Gedanken_sind_Positionen_(17)",
#    "_18": "Verbundenheiten_(18)",
#    "_6": "Triebe_und_Bedürfnisse_(6)",
#    "_9": "Lust_(9)",
#    "_3": "Reflexe_(3),Existenzialien_(3)",
#    "_13_6": "Absicht_6_ist_Vorteilsmaximierung",
#    "_13_7": "Absicht_7_ist_Selbstlosigkeit",
#    "_13_10": "Absicht_10_ist_Wirklichkeit_erkennen",
#    "_13_17": "Absicht_17_ist_zu_meinen",
#    "_10_4": "Zeit_(4)_als_Wirklichkeit",
#    "_16": "Funktionen_Vorstellungen_(16)",
#    "_4": "Achtung_(4)",
#    "_13_1pro8": "Absicht_1/8",
#    "_13_1pro6": "Absicht_1/6_ist_Reinigung_und_Klarheit",
#    "_1pro15": "Reflektion_und_Kategorien_(1/15)",
#    "_1": "Regungen_(1)",
#    "_30": "Energie_und_universelle_Eigenschaften_(30)",
#    "_14": "Stimmungen_Kombinationen_(14)",
#    "_20": "Klassen_(20)",
#    "_37": "Empathie_(37)",
#    "_31": "Garben_und_Verhalten_nachfühlen(31)",
#    "_11": "Verhalten_(11)",
#    "_5_10": "Bedeutung_(10)",
#    "_17_6": "Themen_(6)",
#    "_17_6_10mit4": "Optimierung_(10)",
#    "_36": "Attraktionen_(36)",
#    "_13_16": "Absicht_16_ist_zu_genügen",
#    "_18_7": "Liebe_(7)",
#    "_18_10": "Koalitionen_(10)",
#    "_18_17": "Ansichten_Standpunkte_(18_17)",
#    "_1pro8": "Prinzipien(1/8)",
#    "_1pro5": "Bestrebungen(1/5)",
#    "_1pro3": "Bedingung_und_Auslöser_(1/3)",
#    "_10_4_18_6": "relativer_Zeit-Betrag_(15_10_4_18_6)",
#    "_18_6": "Zahlenvergleich_(15_18_6)",
#    "_21": "Leidenschaften_(21)",
#    "_26": "Erwartungshaltungen_(26)",
#    "_19": "Extremalien_(19),Ziele_(19)",
#    "_18_15": "universeller_Komperativ_(18→15)",
#    "_18_15_n-vs-1pron": "Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)",
#    "_1pro13": "Sollen_Frage_Vorgehensweise_(1/13)",
#    "_1pro19": "Fundament_(1/19)",
#    "_90": "abhängige_Verbundenheit_(90)",
#    "_13_13": "Absicht_13_ist_Helfen",
#    "_1pro12": "Karte_Filter_und_Unterscheidung_(1/12)",
# }

zumVergleich = []
gebrochenErlaubteZahlen: set = set()
for a in reta.Program(["reta", "-" + i18n.mainParaCmds["zeilen"]]).paraNdataMatrix:
    for b in a[1]:
        zumVergleich += [b]
        if len(set(a[0]) & i18n.gebrochenUniGalEinzeln) > 0:
            gebrochenErlaubteZahlen |= {int(b)}
gebrochenErlaubteZahlen -= {max(gebrochenErlaubteZahlen)}

flagX = False
for a in wahl15.values():
    for b in re.split(kpattern, a):
        try:
            assert b in zumVergleich
        except:
            print(b)
            flagX = True
if flagX:
    print()
    print("assert fehlgeschlagen")
    exit()

befehle = i18n.befehle
# print(sys.argv[0].split(os.sep)[-1])
# if sys.argv[0].split(os.sep)[-1] == "rpl":
#    befehle += ["englisch", "english"]
# befehle = ["15" + a for a in wahl15.keys()] + [
#    "mond",
#    "reta",
#    "absicht",
#    "motiv",
#    "thomas",
#    "universum",
#    "motive",
#    "absichten",
#    "vielfache",
#    "einzeln",
#    "multis",
#    "modulo",
#    "prim",
#    "primfaktorzerlegung",
#    "procontra",
#    "prim24",
#    "primfaktorzerlegungModulo24",
#    "help",
#    "hilfe",
#    "abc",
#    "abcd",
#    "alles",
#    "a",
#    "u",
#    "befehle",
#    "t",
#    "richtung",
#    "r",
#    "v",
#    "h",
#    "p",
#    "mo",
#    "mu",
#    "primzahlkreuz",
#    "ende",
#    "exit",
#    "quit",
#    "q",
#    ":q",
#    "shell",
#    "s",
#    "math",
#    "loggen",
#    "nichtloggen",
#    "mulpri",
#    "python",
#    "w",
#    "teiler",
#    "BefehlSpeichernDanach",
#    "S",
#    "BefehlSpeicherungLöschen",
#    "l",
#    "BefehlSpeicherungAusgeben",
#    "o",
#    # "BefehlsSpeicherungsModusAus",
#    # "x",
#    "BefehlSpeichernDavor",
#    "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar",
#    "abstand",
# ]
befehle += [i18n.EIGS_N_R[0] + a for a in eigsN] + [i18n.EIGS_N_R[1] + a for a in eigsR]

befehle2: set = set(befehle) - {"reta"}


def isReTaParameter(t: str):
    return (
        len(t) > 0
        and t[0] == "-"
        and not isZeilenBruchOrGanzZahlAngabe(t)
        and t.split("=")[0] in [str(c).split("=")[0] for c in notParameterValues]
    )


def is15or16command(text: str) -> bool:
    if text[:3] == "15_":
        if text[3:] == "":
            return True
        if text[3:] in wahl15:
            return True
    if text[:3] == "16_":
        if text[3:] == "":
            return True
        if text[:5] == "16_15":
            if text[5:] == "":
                return True
            if text[5] == "_" and text[6:] in wahl15:
                return True
        if text[3:] in wahl16:
            return True
    return False


def stextFromKleinKleinKleinBefehl(promptMode2, stext, textDazu):
    stext2 = []
    ifKurzKurz = False
    xtext = " ".join(stext)
    # if isZeilenAngabe(xtext):
    #    stext = [",".join(str(B) for B in BereichToNumbers2(xtext))]
    stext2 = custom_split(xtext)
    stext3 = []
    del xtext
    for kkk, s_ in enumerate(tuple(deepcopy(stext2))):
        s_ = s_.strip(",")
        s_m = s_
        textDazu = []
        if not is15or16command(s_) and s_ not in befehle and stext[0] != "reta":
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
            except:
                pass

            if n is not None:
                (
                    bruchAndGanzZahlEtwaKorrekterBereich,
                    bruchBereichsAngaben,
                    bruchRanges,
                    zahlenAngaben__Z,
                    fullBlockIsZahlenbereichAndBruch_Z,
                ) = verifyBruchNganzZahlCommaList(
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
                        buchst = set(s_[:n]) & {
                            i18n.befehle2[var]
                            for var in i18n.befehle2.keys()
                            if len(var) == 1
                        }
                    else:
                        buchst = set()
                    setTextLenIs1 = (
                        len(
                            set(stext)
                            - {
                                i18n.befehle2["e"],
                                i18n.befehle2[
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
                        # erst hier passiert wirklich etwas
                        if n == len(buchst):
                            buchst2: list = [
                                a
                                if a != i18n.befehle2["p"]
                                else i18n.befehle2["mulpri"]
                                for a in buchst
                            ]
                            textDazu += buchst2 + [str(s_[n:])]
                        if (
                            setTextLenIs1
                            and len(buchst) == 0
                            and promptMode2 != PromptModus.AusgabeSelektiv
                        ):
                            textDazu += [
                                i18n.befehle2["mulpri"],
                                i18n.befehle2["a"],
                                i18n.befehle2["t"],
                                i18n.befehle2["w"],
                                i18n.befehle2[
                                    "keineEinZeichenZeilenPlusKeineAusgabeWelcherBefehlEsWar"
                                ],
                            ]
                            if any(("/" in _s_ for _s_ in stext)):
                                textDazu += [
                                    i18n.befehle2["u"],
                                    i18n.befehle2["B"],
                                    i18n.befehle2["G"],
                                    i18n.befehle2["E"],
                                    i18n.befehle2["groesse"],
                                ]
                            if (
                                "-" + i18n.retaPrompt.retaPromptParameter["e"]
                                in sys.argv
                            ):
                                textDazu += [
                                    "-" + i18n.mainParaCmds["ausgabe"],
                                    "--" + i18n.ausgabeParas["keineueberschriften"],
                                ]
        else:
            if i18n.befehle2["ee"] == s_:
                textDazu += [
                    "-" + i18n.mainParaCmds["ausgabe"],
                    "--" + i18n.ausgabeParas["keineueberschriften"],
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
        i18n.befehle2["shell"],
        i18n.befehle2["python"],
    ]:
        stext = stext3
    # x("stext", stext)
    # print(stext)
    return ifKurzKurz, stext


def verifyBruchNganzZahlCommaList(
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

    for etwaBruch in re.split(kpattern, commaListe):
        (
            bruchAndGanzZahlEtwaKorrekterBereich1,
            bruchBereichsAngaben1,
            bruchRanges1,
            zahlenAngaben_1,
            etwaAllTrue1,
        ) = verifyBruchNganzZahlBetweenCommas(
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
    bruchAndGanzZahlEtwaKorrekterBereich,
    bruchBereichsAngabe,
    bruchBereichsAngaben,
    bruchRange,
    bruchRanges,
    etwaBruch,
    zahlenAngaben_,
):
    isBruch, isGanzZahl = isZeilenAngabe_betweenKommas(
        bruchBereichsAngabe
    ), isZeilenAngabe_betweenKommas(etwaBruch)
    if isBruch != isGanzZahl:
        bruchAndGanzZahlEtwaKorrekterBereich += [True]
        if isBruch:
            bruchRanges += [bruchRange]
            bruchBereichsAngaben += [bruchBereichsAngabe]
        elif isGanzZahl:
            zahlenAngaben_ += [etwaBruch]
    else:
        bruchAndGanzZahlEtwaKorrekterBereich += [False]
    # if isZeilenAngabe_betweenKommas(etwaBruch):
    #    zahlenAngaben_ += [etwaBruch]
    return (
        bruchAndGanzZahlEtwaKorrekterBereich,
        bruchBereichsAngaben,
        bruchRanges,
        zahlenAngaben_,
        all(bruchAndGanzZahlEtwaKorrekterBereich),
    )


# def getFromZahlenBereichBruchAndZahlenbereich(a, brueche, zahlenAngaben_):
#    ifAllTrue = []
#    first = True
#    for innerKomma in a.split(","):
#        bruch = [bruch for bruch in innerKomma.split("/")]
#        isBruch_ = [bruch1.isdecimal() for bruch1 in bruch] == [True, True]
#        if first:
#            isZahlenangabe_ = isZeilenAngabe(innerKomma)
#        else:
#            isZahlenangabe_ = isZeilenAngabe_betweenKommas(innerKomma)
#        if isBruch_ or isZahlenangabe_:
#            ifAllTrue += [True]
#            if isBruch_:
#                brueche += [bruch]
#            if isZahlenangabe_:
#                zahlenAngaben_ += [innerKomma]
#        else:
#            ifAllTrue += [True]
#        first = False
#    return brueche, zahlenAngaben_, all(ifAllTrue)
def verkuerze_dict(dictionary: dict) -> dict:
    dict2: dict = {}
    for key, value in dictionary.items():
        if value not in dict2.values():
            dict2[key] = value
    return dict2
