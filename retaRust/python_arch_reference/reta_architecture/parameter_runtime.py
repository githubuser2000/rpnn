# -*- coding: utf-8 -*-
"""Shell-/CLI-Parameterlaufzeit für Reta.

Diese Schicht enthält die alte Spalten-, Zeilen-, Breiten- und
Obergrenzen-Auswertung, aber außerhalb des Programmkerns. Sie ist absichtlich
Legacy-kompatibel: Die Funktionen erwarten weiterhin das Program-Objekt als
``self`` und schreiben in dessen Tabellen-/Architekturzustand.
"""
from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable, Optional, Union

try:
    from orderedset import OrderedSet
except Exception:  # pragma: no cover - Legacy-Fallback
    OrderedSet = set

from .universal import normalize_column_buckets

BereichToNumbers2 = None
i18n = None
i18nR = None
cliout = None
getTextWrapThings = None
shellRowsAmount = 0


def _ensure_runtime_imports() -> None:
    """Import legacy runtime modules lazily to avoid center <-> architecture cycles."""

    global BereichToNumbers2, i18n, i18nR, cliout, getTextWrapThings, shellRowsAmount
    if i18n is not None:
        return
    from .runtime_compat import (
        BereichToNumbers2 as _BereichToNumbers2,
        cliout as _cliout,
        getTextWrapThings as _getTextWrapThings,
        i18n as _i18n,
    )

    BereichToNumbers2 = _BereichToNumbers2
    i18n = _i18n
    i18nR = _i18n.retapy
    cliout = _cliout
    getTextWrapThings = _getTextWrapThings
    shellRowsAmount = _getTextWrapThings()[0]


@dataclass(frozen=True)
class ParameterRuntimeBundle:
    """Inspektions-Bundle für die Shell-/CLI-Parameterlaufzeit."""

    column_function: str = "produce_all_spalten_numbers"
    width_function: str = "apply_width_parameter"
    parse_function: str = "parameters_to_commands_and_numbers"
    upper_limit_argument_function: str = "upper_limit_values_for_argument"
    upper_limit_aggregate_function: str = "upper_limit_from_arguments"
    upper_limit_apply_function: str = "apply_upper_limit_argument"

    def snapshot(self) -> dict:
        return {
            "class": type(self).__name__,
            "column_function": self.column_function,
            "width_function": self.width_function,
            "parse_function": self.parse_function,
            "upper_limit_argument_function": self.upper_limit_argument_function,
            "upper_limit_aggregate_function": self.upper_limit_aggregate_function,
            "upper_limit_apply_function": self.upper_limit_apply_function,
        }


def bootstrap_parameter_runtime() -> ParameterRuntimeBundle:
    return ParameterRuntimeBundle()


def produce_all_spalten_numbers(self, neg=""):
    _ensure_runtime_imports()
    global shellRowsAmount

    # x("ANFANG metakonkret", self.paraDict[(cmd[:eq], "konkret")])

    def resultingSpaltenFromTuple(
        tupl: tuple, neg, paraValue=None, befehlName=None
    ) -> tuple:
        # x("tupl", tupl)
        for i, eineSpaltenArtmitSpaltenNummern in enumerate(tupl):
            """
            Die Variable self.tables.spalteGestirn braucht man gar nicht mehr !!!
            """
            # x(
            #    "eineSpaltenArtmitSpaltenNummernWW",
            #    [i, eineSpaltenArtmitSpaltenNummern],
            # )
            if (
                type(eineSpaltenArtmitSpaltenNummern) in [list, tuple]
                and len(eineSpaltenArtmitSpaltenNummern) > 0
            ):
                if type(eineSpaltenArtmitSpaltenNummern[0]) is bool:
                    eineSpaltenArtmitSpaltenNummern = set(
                        eineSpaltenArtmitSpaltenNummern
                    )
                elif type(eineSpaltenArtmitSpaltenNummern[0]) in [tuple, list]:
                    eineSpaltenArtmitSpaltenNummern = set(
                        eineSpaltenArtmitSpaltenNummern[0]
                    )
            # x(
            #    "if ",
            #    [
            #        type(eineSpaltenArtmitSpaltenNummern),
            #        [
            #            list,
            #            tuple,
            #            # set,
            #        ],
            #        befehlName,
            #        i18n.gebrochenUniGalEinzeln,
            #        {b for a in i18n.gebrochenUniGal.values() for b in a},
            #    ],
            # )
            if i == 2 and (
                type(eineSpaltenArtmitSpaltenNummern)
                in [
                    list,
                    tuple,
                    # set,
                ]
                or befehlName in i18n.gebrochenUniGalEinzeln
            ):
                gebrBefehleDict: dict = {
                    type(self).ParametersMain.Multiplikationen[0]: 2,
                    type(self).ParametersMain.gebrochenuniversum[0]: 5,
                    type(self).ParametersMain.gebrochenuniversum[1]: 5,
                    type(self).ParametersMain.gebrochengalaxie[0]: 6,
                    type(self).ParametersMain.gebrochengalaxie[1]: 6,
                    type(self).ParametersMain.gebrochenemotion[0]: 9,
                    type(self).ParametersMain.gebrochenemotion[1]: 9,
                    type(self).ParametersMain.gebrochengroesse[0]: 10,
                    type(self).ParametersMain.gebrochengroesse[1]: 10,
                }
                # x("bli 1", befehlName)
                # x("bli 2", gebrBefehleDict[befehlName])
                # x("bli 3", paraValue)
                # x(
                #    "bli 4",
                #    type(self).lambdaPrimGalax(paraValue)
                #    if befehlName == type(self).ParametersMain.Multiplikationen[0]
                #    else type(self).lambdaGebrUnivUndGalax(paraValue),
                # )
                self.spaltenArtenKey_SpaltennummernValue[
                    len(neg), gebrBefehleDict[befehlName]
                ] |= (
                    type(self).lambdaPrimGalax(paraValue)
                    if befehlName == type(self).ParametersMain.Multiplikationen[0]
                    else type(self).lambdaGebrUnivUndGalax(paraValue)
                )
            elif (
                paraValue == i18nR.beschriebenWort
                and befehlName in type(self).ParametersMain.primvielfache
            ):
                self.spaltenArtenKey_SpaltennummernValue[(len(neg), 2)] |= {2}
            # elif i not in (5, 6, 9, 10):
            else:
                try:
                    # x(
                    #    "dazu_T",
                    #    [
                    #        i,
                    #        self.spaltenArtenKey_SpaltennummernValue[(len(neg), i)],
                    #        eineSpaltenArtmitSpaltenNummern,
                    #        neg,
                    #    ],
                    # )
                    self.spaltenArtenKey_SpaltennummernValue[
                        (len(neg), i)
                    ] |= eineSpaltenArtmitSpaltenNummern
                except TypeError:
                    pass
        return self.spaltenArtenKey_SpaltennummernValue

    def spalten_removeDoublesNthenRemoveOneFromAnother():
        self.spaltenArtenKey_SpaltennummernValue = normalize_column_buckets(
            self.spaltenArtenKey_SpaltennummernValue
        )

    self.mainParaCmds: dict = {
        i18n.mainParaCmds["zeilen"]: 0,
        i18n.mainParaCmds["spalten"]: 1,
        i18n.mainParaCmds[tuple(i18n.tableHandling.parameterName.keys())[0]]: 2,
        i18n.mainParaCmds["ausgabe"]: 3,
        i18n.mainParaCmds["debug"]: None,
        i18n.mainParaCmds["h"]: None,
        i18n.mainParaCmds["help"]: None,
    }
    lastMainCmd: int = -1
    for cmd in self.argv[1:]:
        if len(cmd) > 1 and cmd[0] == "-" and cmd[1] != "-":
            if cmd[1:] in self.mainParaCmds.keys():
                lastMainCmd = self.mainParaCmds[cmd[1:]]
            elif cmd[1:] in (i18nR.nichtsWort, "nichts", "nothing"):
                pass
            elif (
                cmd[: len(i18n.sprachenParameterWort)] == i18n.sprachenParameterWort
                and cmd[len(i18n.sprachenParameterWort) :] in i18n.sprachen.keys()
            ):
                pass
            elif (
                cmd[: len(i18n.sprachenParameterWort)] == i18n.sprachenParameterWort
                and cmd[len(i18n.sprachenParameterWort) :]
                not in i18n.sprachen.keys()
            ):
                print(i18n.wrongLangSentence)
                exit()
            elif len(neg) == 0:
                # else:
                cliout(
                    i18nR.cliout1Saetze[0]
                    + cmd
                    + i18nR.cliout1Saetze[1]
                    + i18nR.cliout1Saetze[2]
                    + str(", -".join(list(self.mainParaCmds.keys())))
                )
        elif cmd[:2] == "--":
            if lastMainCmd == self.mainParaCmds[i18n.mainParaCmds["spalten"]]:
                cmd = cmd[2:]
                eq = cmd.find("=")
                if self.breiteBreitenSysArgvPara(cmd, neg):
                    pass
                elif cmd == i18nR.keineNumWort and len(neg) == 0:
                    self.tables.nummeriere = False
                elif eq != -1:
                    for oneOfThingsAfterEqSign in cmd[eq + 1 :].split(","):
                        if (
                            len(oneOfThingsAfterEqSign) > 0
                            and oneOfThingsAfterEqSign[0] == "-"
                        ):
                            oneOfThingsAfterEqSign = oneOfThingsAfterEqSign[1:]
                            yes1 = True if neg == "-" else False
                        else:
                            yes1 = True if len(neg) == 0 else False
                        if yes1:
                            try:
                                # x(
                                #    "tupleQ4_5",
                                #    [
                                #        self.paraDict[
                                #            (cmd[:eq], oneOfThingsAfterEqSign)
                                #        ][5],
                                #        oneOfThingsAfterEqSign,
                                #        cmd[:eq],
                                #    ],
                                # )
                                resultingSpaltenFromTuple(
                                    self.paraDict[
                                        (cmd[:eq], oneOfThingsAfterEqSign)
                                    ],
                                    neg,
                                    oneOfThingsAfterEqSign,
                                    befehlName=cmd[:eq],
                                )
                            except KeyError:
                                nebenParameters: list = []
                                nebenparameterWerte: list = []
                                for value in self.paraDict.keys():
                                    nebenParameters += [value[0]]
                                    nebenparameterWerte += [value[1]]

                                if cmd[:eq] in nebenParameters:
                                    possibleNebenparameterWert: list = []
                                    for nebenParameter, nebenparameterWert in zip(
                                        nebenParameters,
                                        nebenparameterWerte,
                                    ):
                                        if nebenParameter == cmd[:eq]:
                                            possibleNebenparameterWert += [
                                                nebenparameterWert
                                            ]

                                    cliout(
                                        i18nR.cliout2Saetze[0]
                                        + cmd[:eq]
                                        + i18nR.cliout2Saetze[1]
                                        + oneOfThingsAfterEqSign
                                        + (
                                            (i18nR.cliout2Saetze[2])
                                            + (
                                                ",".join(possibleNebenparameterWert)
                                                + '"'
                                            )
                                            if (
                                                len(possibleNebenparameterWert) > 0
                                                and not all(
                                                    [
                                                        p == ""
                                                        for p in possibleNebenparameterWert
                                                    ]
                                                )
                                            )
                                            else i18nR.cliout2Saetze[3]
                                        )
                                    )
                                else:
                                    cliout(
                                        i18nR.cliout3Saetze[0]
                                        + cmd[:eq]
                                        + i18nR.cliout3Saetze[1]
                                        + oneOfThingsAfterEqSign
                                        + i18nR.cliout3Saetze[2]
                                        + i18nR.cliout3Saetze[3]
                                        + i18nR.cliout3Saetze[4]
                                        + i18nR.cliout3Saetze[5]
                                        + str(
                                            ", --".join(
                                                tuple(
                                                    OrderedSet(
                                                        key[0]
                                                        for key in self.paraDict.keys()
                                                    )
                                                )
                                            )
                                        )
                                        + i18nR.cliout3Saetze[6]
                                        + i18nR.cliout3Saetze[7]
                                        + str(
                                            ",".join(
                                                tuple(
                                                    OrderedSet(
                                                        key[1]
                                                        for key in self.paraDict.keys()
                                                    )
                                                )
                                            )
                                        )
                                    )

                else:
                    try:
                        if len(cmd) > 0 and (cmd[-1] == "-" and neg == "-") != (
                            len(neg) == 0 and cmd[-1] != "-"
                        ):
                            if len(cmd) > 0 and cmd[-1] == "-" and len(neg) > 0:
                                cmd = cmd[:-1]

                            # x("tupleP4_5", self.paraDict[(cmd, "")][5])
                            resultingSpaltenFromTuple(
                                self.paraDict[(cmd, "")], neg, befehlName=cmd
                            )

                    except KeyError:
                        cliout(
                            i18nR.cliout4Saetze[0]
                            + cmd
                            + i18nR.cliout4Saetze[1]
                            + i18nR.cliout4Saetze[2]
                            + i18nR.cliout4Saetze[3]
                            + i18nR.cliout4Saetze[4]
                            + str(
                                ", --".join(
                                    tuple(
                                        OrderedSet(
                                            key[0] for key in self.paraDict.keys()
                                        )
                                    )
                                )
                            )
                            + i18nR.cliout4Saetze[5]
                        )

            elif (
                lastMainCmd
                == self.mainParaCmds[self.tables.getCombis.parameterName]
            ):
                galWort = "--" + i18n.kombiMainParas["galaxie"] + "="
                uniWort = "--" + i18n.kombiMainParas["universum"] + "="

                if cmd[: len(galWort)] == galWort or cmd[: len(uniWort)] == uniWort:
                    for oneKombiSpalte in cmd[cmd.find("=") + 1 :].split(","):
                        if len(oneKombiSpalte) > 0 and oneKombiSpalte[0] == "-":
                            oneKombiSpalte = oneKombiSpalte[1:]
                            yes1 = True if neg == "-" else False
                        else:
                            yes1 = True if len(neg) == 0 else False
                        if yes1:
                            try:
                                resultingSpaltenFromTuple(
                                    (
                                        OrderedSet(),
                                        OrderedSet(),
                                        OrderedSet(),
                                        {
                                            self.kombiReverseDict[oneKombiSpalte],
                                        }
                                        if cmd.find("=") == len(galWort) - 1
                                        else OrderedSet(),
                                        OrderedSet(),
                                        OrderedSet(),
                                        OrderedSet(),
                                        OrderedSet(),
                                        {
                                            self.kombiReverseDict2[oneKombiSpalte],
                                        }
                                        if cmd.find("=") == len(uniWort) - 1
                                        else OrderedSet(),
                                    ),
                                    neg,
                                    befehlName="kombinationen",
                                )
                            except KeyError:
                                cliout(
                                    i18nR.cliout5Saetze[0]
                                    + oneKombiSpalte
                                    + i18nR.cliout5Saetze[1]
                                    + cmd[: cmd.find("=") + 1]
                                    + " "
                                    + (
                                        str(
                                            tuple(
                                                [
                                                    element
                                                    for row in i18n.kombiParaNdataMatrix.values()
                                                    for element in row
                                                ]
                                            )
                                        )[1:-1]
                                        if cmd[: cmd.find("=")] == galWort[:-1]
                                        else str(
                                            tuple(
                                                [
                                                    element
                                                    for row in i18n.kombiParaNdataMatrix2.values()
                                                    for element in row
                                                ]
                                            )
                                        )[1:-1]
                                        if cmd[: cmd.find("=")] == uniWort[:-1]
                                        else ""
                                    )
                                )

                elif neg == "":
                    cliout(i18nR.cliout6Satz + str(cmd))
            elif lastMainCmd not in self.mainParaCmds.values():
                cliout(
                    i18nR.cliout7Saetze[0]
                    + i18nR.cliout7Saetze[1]
                    + cmd
                    + i18nR.cliout7Saetze[2]
                    + " -".join(self.mainParaCmds)
                )
    breiteIstNull = "".join(("--", i18n.ausgabeParas["breite"], "=0"))
    if breiteIstNull in self.argv:
        self.breiteBreitenSysArgvPara(breiteIstNull[2:], "")
    if len(neg) == 0:
        self.produceAllSpaltenNumbers("-")
        spalten_removeDoublesNthenRemoveOneFromAnother()

def apply_width_parameter(self, cmd, neg) -> bool:
    _ensure_runtime_imports()
    global shellRowsAmount
    # alxp(
    #    "shellRowsAmount war in reta.py wegen dem Setzen der Breite auf {} gesetzt".format(
    #        shellRowsAmount
    #    )
    # )

    paraBreite = i18n.ausgabeParas["breite"] + "="
    paraBreiteN = i18n.ausgabeParas["breiten"] + "="
    if cmd[: len(paraBreite)] == paraBreite:
        shellRowsAmount, _, _, _ = getTextWrapThings()
        if self.breiteHasBeenOnceZero:
            shellRowsAmount = 0
            self.tables.textWidth = 0
            self.breiteORbreiten = True
            return True
        if cmd[len(paraBreite) :].isdecimal():
            breite = abs(int(cmd[len(paraBreite) :]))
            if breite == 0:
                self.breiteHasBeenOnceZero = True
                shellRowsAmount = 0
            elif shellRowsAmount > 7 and breite > shellRowsAmount - 7:
                breite = shellRowsAmount - 7
            try:
                self.tables.textWidth = (
                    breite
                    if breite > self.tables.textWidth
                    else self.tables.textWidth
                )
            except:
                self.tables.textWidth = breite
            self.breiteORbreiten = True
        return True
    elif cmd[: len(paraBreiteN)] == paraBreiteN and len(neg) == 0:
        self.tables.breitenn = []
        for breite in cmd[len(paraBreiteN) :].split(","):
            if breite.isdecimal():
                self.tables.breitenn += [int(breite)]
                self.breiteORbreiten = True
        return True
    return False

def parameters_to_commands_and_numbers(
    self, argv, neg=""
) -> Iterable[Union[set, set, set, list]]:
    _ensure_runtime_imports()
    """Parameter in der Shell werden hier vorverarbeitet.
    Die Paraemter führen dazu, dass Variablen gesetzt werden, z.B.
    eine Menge die als Befehl kodiert, welche Zeilen und eine die kodiert
    welche Spaltennummer ausgegeben werden sollen.
    Außerdem welche extra Tabellen geladen werden sollen.

    return paramLines, rowsAsNumbers, rowsOfcombi

    @type  argv: list
    @param argv: Programmparamenter
    @type  neg: str
    @param neg: MinusZeichen davor ?
    @rtype: set, set, set
    @return: Zeilen, Spalten, Spalten anderer Tabellen
    """
    global infoLog, shellRowsAmount  # , puniverseprims
    if len(argv) == 1 and neg == "":
        cliout(i18nR.cliout8SatzVersucheParaH)
    spaltenreihenfolgeundnurdiese: tuple = ()
    puniverseprims_only: set = OrderedSet()
    rowsAsNumbers: set = set()
    paramLines: set = OrderedSet()
    self.bigParamaeter: list = []
    self._Program__willBeOverwritten_rowsOfcombi: set = OrderedSet()
    generRows = OrderedSet()
    for arg in argv[1:]:
        if len(arg) > 0 and arg[0] == "-":
            if (
                len(arg) > 1
                and arg[1] == "-"
                and len(self.bigParamaeter) > 0
                and self.bigParamaeter[-1] == i18n.mainParaCmds["zeilen"]
            ):
                if (
                    arg[2 : i18n.zeilenParasLen["alles"] + 2]
                    == i18n.zeilenParas["alles"]
                    and len(neg) == 0
                ):
                    paramLines.add("all")
                    self.obZeilenBereicheAngegeben = True
                if (
                    arg[2 : 2 + i18n.zeilenParasLen["alles"]]
                    == i18n.zeilenParas["alles"]
                    and len(neg) != 0
                ):
                    pass
                elif (
                    arg[2 : i18n.zeilenParasLen["zeit"] + 3]
                    == i18n.zeilenParas["zeit"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    for subpara in arg[3 + i18n.zeilenParasLen["zeit"] :].split(
                        ","
                    ):
                        if neg + i18n.zeilenParas["heute"] == subpara:
                            paramLines.add("=")
                        elif neg + i18n.zeilenParas["gestern"] == subpara:
                            paramLines.add("<")
                        elif neg + i18n.zeilenParas["morgen"] == subpara:
                            paramLines.add(">")
                elif (
                    arg[2 : 3 + i18n.zeilenParasLen["zaehlung"]]
                    == i18n.zeilenParas["zaehlung"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    if neg == "":
                        paramLines |= (
                            self.tables.getPrepare.parametersCmdWithSomeBereich(
                                arg[3 + i18n.zeilenParasLen["zaehlung"] :],
                                "n",
                                "",
                                True,
                            )
                        )
                elif (
                    arg[2 : 3 + i18n.zeilenParasLen["hoehemaximal"]]
                    == i18n.zeilenParas["hoehemaximal"] + "="
                ):
                    if arg[3 + i18n.zeilenParasLen["hoehemaximal"] :].isdecimal():
                        self.tables.textHeight = abs(int(arg[15:]))
                elif (
                    arg[2 : 3 + i18n.zeilenParasLen["typ"]]
                    == i18n.zeilenParas["typ"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    for word in arg[3 + i18n.zeilenParasLen["typ"] :].split(","):
                        if word == neg + i18n.zeilenParas["sonne"]:
                            paramLines.add("sonne")
                        elif word == neg + i18n.zeilenParas["schwarzesonne"]:
                            paramLines.add("schwarzesonne")
                        elif word == neg + i18n.zeilenParas["planet"]:
                            paramLines.add("planet")
                        elif word == neg + i18n.zeilenParas["mond"]:
                            paramLines.add("mond")
                        elif word == neg + i18n.zeilenParas["SonneMitMondanteil"]:
                            paramLines.add("SonneMitMondanteil")
                elif (
                    arg[2 : 3 + i18n.zeilenParasLen["primzahlen"]]
                    == i18n.zeilenParas["primzahlen"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    for word in arg[3 + i18n.zeilenParasLen["primzahlen"] :].split(
                        ","
                    ):
                        if word == neg + i18n.zeilenParas["aussenerste"]:
                            paramLines.add("aussenerste")
                        elif word == neg + i18n.zeilenParas["innenerste"]:
                            paramLines.add("innenerste")
                        elif word == neg + i18n.zeilenParas["aussenalle"]:
                            paramLines.add("aussenalle")
                        elif word == neg + i18n.zeilenParas["innenalle"]:
                            paramLines.add("innenalle")
                elif (
                    arg[2 : 3 + i18n.zeilenParasLen["potenzenvonzahlen"]]
                    == i18n.zeilenParas["potenzenvonzahlen"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    if neg == "" or True:
                        angabe = arg[3 + i18n.zeilenParasLen["potenzenvonzahlen"] :]
                        paramLines |= (
                            self.tables.getPrepare.parametersCmdWithSomeBereich(
                                angabe, "^", neg, keineNegBeruecksichtigung=False
                            )
                        )
                elif (
                    arg[2 : 3 + i18n.zeilenParasLen["vielfachevonzahlen"]]
                    == i18n.zeilenParas["vielfachevonzahlen"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    if neg == "":
                        paramLines |= (
                            self.tables.getPrepare.parametersCmdWithSomeBereich(
                                arg[
                                    3 + i18n.zeilenParasLen["vielfachevonzahlen"] :
                                ],
                                "b",
                                neg,
                                keineNegBeruecksichtigung=True,
                            )
                        )
                elif (
                    arg[2 : 3 + i18n.zeilenParasLen["primzahlvielfache"]]
                    == i18n.zeilenParas["primzahlvielfache"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    if neg == "":
                        zahlenMenge = BereichToNumbers2(
                            arg[3 + i18n.zeilenParasLen["primzahlvielfache"] :]
                        )
                        for zahl in zahlenMenge:
                            paramLines.add(str(zahl) + "p")
                elif self.oberesMaximum(arg):
                    pass
                elif (
                    arg[2 : 2 + i18n.zeilenParasLen["invertieren"]]
                    == i18n.zeilenParas["invertieren"]
                ):
                    self.obZeilenBereicheAngegeben = True
                    if neg == "":
                        paramLines |= (
                            self.tables.getPrepare.parametersCmdWithSomeBereich(
                                "1", "i", neg, keineNegBeruecksichtigung=True
                            )
                        )
                elif (
                    arg[2 : 2 + i18n.zeilenParasLen["vorhervonausschnittteiler"]]
                    == i18n.zeilenParas["vorhervonausschnittteiler"]
                ):
                    self.obZeilenBereicheAngegeben = True
                    if neg == "":
                        paramLines |= (
                            self.tables.getPrepare.parametersCmdWithSomeBereich(
                                "1", "w", neg, keineNegBeruecksichtigung=True
                            )
                        )
                elif (
                    arg[2 : 3 + i18n.zeilenParasLen["vorhervonausschnitt"]]
                    == i18n.zeilenParas["vorhervonausschnitt"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    if neg == "":
                        paramLines |= (
                            self.tables.getPrepare.parametersCmdWithSomeBereich(
                                arg[
                                    3 + i18n.zeilenParasLen["vorhervonausschnitt"] :
                                ],
                                "a",
                                neg,
                                keineNegBeruecksichtigung=True,
                            )
                        )
                elif (
                    arg[
                        2 : 3
                        + i18n.zeilenParasLen["nachtraeglichneuabzaehlungvielfache"]
                    ]
                    == i18n.zeilenParas["nachtraeglichneuabzaehlungvielfache"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    paramLines |= (
                        self.tables.getPrepare.parametersCmdWithSomeBereich(
                            arg[
                                3
                                + i18n.zeilenParasLen[
                                    "nachtraeglichneuabzaehlungvielfache"
                                ] :
                            ],
                            "y",
                            neg,
                        )
                    )
                elif (
                    arg[2 : 3 + i18n.zeilenParasLen["nachtraeglichneuabzaehlung"]]
                    == i18n.zeilenParas["nachtraeglichneuabzaehlung"] + "="
                ):
                    self.obZeilenBereicheAngegeben = True
                    paramLines |= (
                        self.tables.getPrepare.parametersCmdWithSomeBereich(
                            arg[
                                3
                                + i18n.zeilenParasLen[
                                    "nachtraeglichneuabzaehlung"
                                ] :
                            ],
                            "z",
                            neg,
                        )
                    )
                elif len(neg) > 0:
                    from LibRetaPrompt import zeilenParas

                    cliout(
                        i18nR.cliout9Saetze[0]
                        + arg
                        + i18nR.cliout9Saetze[1]
                        + self.bigParamaeter[-1]
                        + i18nR.cliout9Saetze[2]
                        + i18nR.cliout9Saetze[3]
                        + ", ".join(zeilenParas)
                    )
            elif (
                len(arg) > 1
                and arg[1] == "-"
                and len(self.bigParamaeter) > 0
                and self.bigParamaeter[-1] == i18n.mainParaCmds["ausgabe"]
            ):  # unteres Kommando
                if self.breiteBreitenSysArgvPara(arg[2:], neg):
                    pass
                elif (
                    arg[2 : 2 + i18n.ausgabeParasLen["keineueberschriften"]]
                    == i18n.ausgabeParas["keineueberschriften"]
                ):
                    self.tables.keineUeberschriften = True
                elif (
                    arg[2 : 2 + i18n.ausgabeParasLen["keinenummerierung"]]
                    == i18n.ausgabeParas["keinenummerierung"]
                ):
                    self.tables.nummeriere = False
                elif (
                    arg[2 : 2 + i18n.ausgabeParasLen["keineleereninhalte"]]
                    == i18n.ausgabeParas["keineleereninhalte"]
                ):
                    self.keineleereninhalte = True
                    self.tables.keineleereninhalte = True
                elif (
                    arg[
                        2 : 3
                        + i18n.ausgabeParasLen["spaltenreihenfolgeundnurdiese"]
                    ]
                    == i18n.ausgabeParas["spaltenreihenfolgeundnurdiese"] + "="
                ):
                    spaltenreihenfolgeundnurdiese = tuple(
                        BereichToNumbers2(
                            arg[
                                3
                                + i18n.ausgabeParasLen[
                                    "spaltenreihenfolgeundnurdiese"
                                ] :
                            ]
                        )
                    )
                elif (
                    arg[2 : i18n.ausgabeParasLen["art"] + 3]
                    == i18n.ausgabeParas["art"] + "="
                ):
                    outputtype = arg[(arg.find("=") + 1) :]
                    self.apply_output_mode(outputtype)
                elif (
                    arg[2:]
                    in [i18n.ausgabeParas["nocolor"], i18n.ausgabeParas["justtext"]]
                    and neg == ""
                ):
                    self.tables.getOut.color = False
                elif (
                    arg[2:]
                    in [
                        i18n.ausgabeParas["endlessscreen"],
                        i18n.ausgabeParas["endless"],
                        i18n.ausgabeParas["dontwrap"],
                        i18n.ausgabeParas["onetable"],
                    ]
                    and neg == ""
                ):
                    self.tables.getOut.oneTable = True
                elif len(neg) == 0:
                    cliout(
                        i18nR.cliout10Saetze[0]
                        + arg
                        + i18nR.cliout10Saetze[1]
                        + self.bigParamaeter[-1]
                        + i18nR.cliout10Saetze[2]
                    )
            else:  # oberes Kommando
                if arg[1:] in [
                    i18n.hauptForNeben["zeilen"],
                    i18n.hauptForNeben["spalten"],
                    i18n.hauptForNeben["kombination"],
                    i18n.hauptForNeben["ausgabe"],
                ]:
                    self.bigParamaeter += [arg[1:]]
                elif arg[1:] in [i18n.hauptForNeben["debug"]]:
                    self.propInfoLog = True
                elif (
                    arg[1:] in [i18n.hauptForNeben["h"], i18n.hauptForNeben["help"]]
                    and neg == ""
                ):
                    self.helpPage()

    if not self.tables.getOut.oneTable:
        shellRowsAmount, _, _, _ = getTextWrapThings()

        self.tables.textWidth = (
            self.tables.textWidth
            if shellRowsAmount > self.tables.textWidth + 7 or shellRowsAmount <= 0
            else shellRowsAmount - 7
        )
    self.tables.ifZeilenSetted = self.obZeilenBereicheAngegeben
    return (
        paramLines,
        rowsAsNumbers,
        self._Program__willBeOverwritten_rowsOfcombi,
        spaltenreihenfolgeundnurdiese,
        puniverseprims_only,
        generRows,
    )

def upper_limit_values_for_argument(self, arg) -> tuple:
    _ensure_runtime_imports()
    werte: list = []
    if (
        arg[2 : 3 + i18n.zeilenParasLen["oberesmaximum"]]
        == i18n.zeilenParas["oberesmaximum"] + "="
        and arg[3 + i18n.zeilenParasLen["oberesmaximum"] :].isdecimal()
    ):
        werte = [int(arg[3 + i18n.zeilenParasLen["oberesmaximum"] :])]
        return werte, True
    elif (
        arg[2 : 3 + i18n.zeilenParasLen["vorhervonausschnitt"]]
        == i18n.zeilenParas["vorhervonausschnitt"] + "="
    ):
        werteList: list = [
            a + 1
            for a in BereichToNumbers2(
                arg[3 + i18n.zeilenParasLen["vorhervonausschnitt"] :], False, 0
            )
        ]
        werte = [max(w, 1024) for w in werteList]
        return werte, False
    else:
        return werte, False

def upper_limit_from_arguments(self, argv2) -> Optional[int]:
    _ensure_runtime_imports()
    try:
        werte: list = [self.tables.hoechsteZeile[1024]]
    except:
        werte: list = []
    for arg in argv2:
        werte += self.oberesMaximumArg(arg)[0]

    return max(werte) if len(werte) > 0 else None

def apply_upper_limit_argument(self, arg) -> bool:
    _ensure_runtime_imports()
    liste, wahrheitswert = self.oberesMaximumArg(arg)
    if len(liste) == 0 or not wahrheitswert:
        return False
    max_ = max(liste + [self.tables.hoechsteZeile[1024]])
    self.tables.hoechsteZeile = max_
    return True
