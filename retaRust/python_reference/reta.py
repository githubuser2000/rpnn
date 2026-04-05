#!/usr/bin/env pypy3
# -*- coding: utf-8 -*-
# import numpy as np
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "libs"))
import html
import json
import platform
import re
from collections import OrderedDict, namedtuple
from itertools import zip_longest
from typing import Any, NamedTuple, Optional, Tuple, Union

try:
    from orderedset import OrderedSet
except:
    OrderedSet = set

from center import (
    BereichToNumbers2,
    Primzahlkreuz_pro_contra_strs,
    i18n,
    retaHilfe,
    nPmEnum,
)
from tableHandling import (
    Enum,
    Iterable,
    Multiplikationen,
    OutputSyntax,
    NichtsSyntax,
    Tables,
    Union,
    alxp,
    bbCodeSyntax,
    cliout,
    copy,
    csv,
    csvSyntax,
    deepcopy,
    emacsSyntax,
    getTextWrapThings,
    htmlSyntax,
    infoLog,
    markdownSyntax,
    math,
    os,
    output,
    primCreativity,
    re,
    setShellRowsAmount,
    shellRowsAmount,
    sys,
    x,
)

gebrochenSpaltenMaximumPlus1 = i18n.gebrochenSpaltenMaximumPlus1

csvFileNames = i18n.csvFileNames
i18nR = i18n.retapy


def render_color(tag_name, value, options, parent, context):
    return '<span style="color:%s;">%s</span>' % (tag_name, value)


class Program:
    def produceAllSpaltenNumbers(self, neg=""):
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
                        Program.ParametersMain.Multiplikationen[0]: 2,
                        Program.ParametersMain.gebrochenuniversum[0]: 5,
                        Program.ParametersMain.gebrochenuniversum[1]: 5,
                        Program.ParametersMain.gebrochengalaxie[0]: 6,
                        Program.ParametersMain.gebrochengalaxie[1]: 6,
                        Program.ParametersMain.gebrochenemotion[0]: 9,
                        Program.ParametersMain.gebrochenemotion[1]: 9,
                        Program.ParametersMain.gebrochengroesse[0]: 10,
                        Program.ParametersMain.gebrochengroesse[1]: 10,
                    }
                    # x("bli 1", befehlName)
                    # x("bli 2", gebrBefehleDict[befehlName])
                    # x("bli 3", paraValue)
                    # x(
                    #    "bli 4",
                    #    Program.lambdaPrimGalax(paraValue)
                    #    if befehlName == Program.ParametersMain.Multiplikationen[0]
                    #    else Program.lambdaGebrUnivUndGalax(paraValue),
                    # )
                    self.spaltenArtenKey_SpaltennummernValue[
                        len(neg), gebrBefehleDict[befehlName]
                    ] |= (
                        Program.lambdaPrimGalax(paraValue)
                        if befehlName == Program.ParametersMain.Multiplikationen[0]
                        else Program.lambdaGebrUnivUndGalax(paraValue)
                    )
                elif (
                    paraValue == i18nR.beschriebenWort
                    and befehlName in Program.ParametersMain.primvielfache
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
            for el2Type in range(
                int(len(self.spaltenArtenKey_SpaltennummernValue) / 2)
            ):
                self.spaltenArtenKey_SpaltennummernValue[(0, el2Type)] -= (
                    self.spaltenArtenKey_SpaltennummernValue[(0, el2Type)]
                    & self.spaltenArtenKey_SpaltennummernValue[(1, el2Type)]
                )
            for el2Type in range(
                int(len(self.spaltenArtenKey_SpaltennummernValue) / 2)
            ):
                self.spaltenArtenKey_SpaltennummernValue[
                    (0, el2Type)
                ] -= self.spaltenArtenKey_SpaltennummernValue.pop((1, el2Type))

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

    def breiteBreitenSysArgvPara(self, cmd, neg) -> bool:
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

    def storeParamtersForColumns(self):
        # global puniverseprims
        def intoParameterDatatype(
            parameterMainNames: tuple, parameterNames: tuple, datas: tuple
        ) -> tuple:
            """
            ALLE PARAMETER DIESER FUNKTION SIND EIGENTLICH NUR EIN JEWEILIGES ELEMENT VON
            paraNdataMatrix
            ZUSAMMEN

            Speichert einen Parameter mit seinem DatenSet
            in 2 Datenstrukturen (die beides kombinieren 2x2)
            Diese werden jedoch nur zurück gegeben und nicht in der Klasse gespeichert.
            @return: alle Hauptparamter| alle Nebenparamter zu nur einem
            Hauptparameter ergibt Mengen an Spalten | enthält alle Haup- und
            Nebenparameter keys sind Spalten der Tabelle
            """
            paraMainDict = {}
            for name in parameterMainNames:
                paraMainDict[name] = parameterNames
            paraDict = {}
            for name1 in parameterMainNames:
                for name2 in parameterNames:
                    paraDict[(name1, name2)] = datas
                if len(parameterNames) == 0:
                    paraDict[(name1, "")] = datas
            dataDicts: tuple = ({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})

            # datas sind nicht die Haupt-und-Neben-Parameter, sondern alles das diese enthalten und meinen können
            # ein datas Datensatz sind alle sets, die ein Haupt-Neben-Parameter Zusammenhang enthalten kann an sets
            for i, d in enumerate(datas):
                for spaltenNummerOderEtc in d:
                    # spaltenNummerOderEtc ist hier also eine Zahl von einem set, die z.B. eine Spaltennummer meinen kann
                    into = []
                    parameterMainNamePerLoop = []
                    case: int = None

                    # das mit 2 Schleifen nur deshalb, damit immer alle Haupt- und Neben-Parameter in die Liste rein kommen
                    for parameterMainName in parameterMainNames:
                        for parameterName in (
                            parameterNames if len(parameterNames) > 0 else ("",)
                        ):
                            # i ist die Nummer welches Set es ist
                            if i == 4 and (
                                type(spaltenNummerOderEtc) is bool
                                or (
                                    type(spaltenNummerOderEtc) in [tuple, list]
                                    and len(spaltenNummerOderEtc) > 0
                                    and type(spaltenNummerOderEtc[0]) is bool
                                )
                            ):
                                case = 1
                                into += [
                                    (
                                        parameterMainName,
                                        parameterName,
                                    )
                                ]
                            elif i in (
                                5,
                                6,
                                9,
                                10,
                            ):  # and type(spaltenNummerOderEtc) is set:
                                case = 2
                                into += [[(parameterMainName, parameterName)]]
                                parameterMainNamePerLoop += [parameterName]
                            elif i == 2 and callable(spaltenNummerOderEtc):
                                case = 2
                                parameterMainNamePerLoop += [parameterName]
                                into += [[(parameterMainName, parameterName)]]
                            elif i == 4 and (
                                type(spaltenNummerOderEtc) in (list, tuple)
                            ):
                                case = 4
                                into += [(parameterMainName, parameterName)]
                            elif i == 4 and (type(spaltenNummerOderEtc) in (set,)):
                                case = 4
                                into += [(parameterMainName, parameterName)]
                                spaltenNummerOderEtc = spaltenNummerOderEtc.pop()
                            else:
                                case = 3
                                try:
                                    into += [(parameterMainName, parameterName)]
                                except KeyError:
                                    into = [(parameterMainName, parameterName)]

                    index1 = i if case != 1 else 3
                    index2a = (
                        spaltenNummerOderEtc
                        if case == 3
                        else (
                            spaltenNummerOderEtc
                            if case == 4
                            else ("bool", 0)
                            if case == 1
                            else tuple(
                                (
                                    int(para)
                                    if para.isdecimal()
                                    else para
                                    if len(parameterNames) > 0
                                    else None
                                    for para in parameterMainNamePerLoop
                                )
                            )
                            if case == 2
                            else None
                        )
                    )
                    intoA = into if case == 2 else (into,)
                    for index2, into2 in zip_longest(
                        index2a if case == 2 else (index2a,), intoA, fillvalue=into
                    ):
                        try:
                            # x("index1", index1)
                            # x("index1", d)
                            dataDicts[index1][index2] += (
                                (into2,)
                                if into2 not in dataDicts[index1][index2]
                                else ()
                            )
                        except KeyError:
                            dataDicts[index1][index2] = (into2,)
            return paraMainDict, paraDict, dataDicts

        def mergeParameterDicts(
            paraMainDict1: dict,
            paraDict1: dict,
            dataDicts1: list,
            paraMainDict2: dict,
            paraDict2: dict,
            dataDicts2: list,
        ) -> tuple:
            """Merged die beiden 2x2 Datenstrukturen und speichert diese
            in die Klasse und gibt sie dennoch auch mit return zurück
            @param paraMainDict: Hauptparameter in der Kommandozeile
            hat als Werte die Nebenparameter und keys sind die Hauptparamter
            @param paraDict: Nebenparamteter in der Kommandozeile
            hat als Werte die Spaltennummern dazugehörig
            @param dataDicts: die beiden Parameter sagen welche Spaltennummern es
            sein werden
            @return: Spaltennummer sagt welche Parameter es ingesamt dazu sind | die
            beiden Parameter sagen, welche Spalten es alle sind.

            *paraNdataMatrix*
            enthält die meisten Parameternamen mit den zugehörigen Spaltennummern mit Sonderdaten, weil einige Spalten generiert werden aus anderen

            u.a. daraus wird das *paraDict* und *dataDict* gebaut. Beides hat das Gleiche drin, nur das andere jeweils mit Key und Value vertauscht.
            Darin sind die Paramenternamen und csv Spaltennummern drin, die nicht verwechselt werden dürfen mit den dann real vorhandenen Spaltennummern, die nicht die gleichen als Zahl sind, wie die in der CSV-Datei.

            *self.tables.generatedSpaltenParameter*
            key ist Spaltennummer der Ausgabe, value ist ein Paar von 2 Strings über Überparametername und Unterparametername für den Klassenname für die Spalte des HTML-Tags.
            <em>
            Das beinhaltet das für alle Parameter und Ausgabe-Spalten-Nummern.</em>"""
            global gebrochenSpaltenMaximumPlus1

            paraMainDict1 = {**paraMainDict1, **paraMainDict2}
            paraDict1 = {**paraDict1, **paraDict2}
            dataDicts3 = deepcopy(dataDicts1)
            for i, (dict1, dict2) in enumerate(zip_longest(dataDicts1, dataDicts2)):
                if type(dict1) is dict and type(dict2) is dict:
                    if len(dataDicts3[i].keys()) == 0:
                        dataDicts3[i] = dataDicts2[i]
                    else:
                        for key1, value1 in dict1.items():
                            for key2, value2 in dict2.items():
                                if key2 == key1:
                                    dataDicts3[i][key1] += value2
                                elif key2 not in dataDicts3[i].keys():
                                    dataDicts3[i][key2] = value2
                elif type(dict1) is dict and dict2 is None:
                    dataDicts3[i] = dict1
                elif dict1 is None and type(dict2) is dict:
                    dataDicts3[i] = dict2
            return paraDict1, dataDicts3

        Program.ParametersMain: namedtuple = i18n.ParametersMain

        allowedPrimNumbersForCommand: tuple[str] = tuple(
            (
                str(num)
                for num in tuple(
                    OrderedSet(
                        (
                            num if primCreativity(num) == 1 else None
                            for num in range(2, 32)
                        )
                    )
                    - {None}
                )
            )
        )

        Program.lambdaGebrUnivUndGalax = lambda paraValues: {
            abs(int(chosen)) if chosen.isdecimal() else None
            for chosen in [value for value in (paraValues.split(","))]
        } - {None, 0, 1}

        Program.lambdaPrimGalax = lambda paraValues: {
            abs(int(chosen))
            if chosen.isdecimal() and primCreativity(abs(int(chosen))) == 1
            else None
            for chosen in [value for value in (paraValues.split(","))]
        } - {None, 0, 1}

        paraNdataMatrix: list[
            Tuple[Any, dict[str, str], set[int], Optional[set]]
        ] = i18n.paraNdataMatrix
        Program.paraNdataMatrix = paraNdataMatrix

        Program.kombiParaNdataMatrix: OrderedDict[
            int, tuple[str]
        ] = i18n.kombiParaNdataMatrix

        Program.kombiParaNdataMatrix2: OrderedDict[
            int, tuple[str]
        ] = i18n.kombiParaNdataMatrix2
        self.kombiReverseDict: dict = {}
        for key, value in Program.kombiParaNdataMatrix.items():
            for valuesInValuess in value:
                self.kombiReverseDict[valuesInValuess] = key

        self.kombiReverseDict2: dict = {}
        for key, value in Program.kombiParaNdataMatrix2.items():
            for valuesInValuess in value:
                self.kombiReverseDict2[valuesInValuess] = key

        allValues = [
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
            OrderedSet(),
        ]
        # x("paraNdataMatrix A4", paraNdataMatrix)
        # x("allValues 11 A", allValues[11])
        for possibleCommands in paraNdataMatrix:
            for commandValue, aAllValue in zip(possibleCommands[2:], allValues):
                try:
                    aAllValue |= commandValue
                except TypeError:
                    print("FEHLER")
                    print(commandValue)
                    raise ValueError
                    exit()

        self.AllSimpleCommandSpalten = set(allValues[0])
        if self.__invertAlles:
            allValues[0] = (
                set(range(max(allValues[0])))
                - set(allValues[0])
                - {a[0] for a in allValues[1]}
                - {a[1] for a in allValues[1]}
            )
        # x("allV1", allValues)
        # x("allValues 4b", allValues[4])
        # x("allValues 11 B", allValues[11])
        """
        Folgende Schleife ist eigentlich unnötig.
        Sie ist für bool Werte da, wenn Sachen generiert werden.
        Ich brauche aber gerade den bool wert gar nicht mehr, weil es in
        diesem Fall auch anders geht, aber für die Zukunft kann das hilfreich sein.
        Also lasse ich es mal stehen!

        for possibleCommands in paraNdataMatrix:
            for commandValue, aAllValue in zip(possibleCommands[6:], allValues[4:]):
                aAllValue += [commandValue]
        # allValues[1] = allValues[2]
        """
        allValues[2] = set((int(pNum) for pNum in allowedPrimNumbersForCommand))
        allValues[3] = set(Program.kombiParaNdataMatrix.keys())
        allValues[5] = set(range(2, gebrochenSpaltenMaximumPlus1))
        allValues[6] = set(range(2, gebrochenSpaltenMaximumPlus1))
        allValues[8] = set(Program.kombiParaNdataMatrix2.keys())
        allValues[9] = set(range(2, gebrochenSpaltenMaximumPlus1))
        allValues[10] = set(range(2, gebrochenSpaltenMaximumPlus1))
        if self.__invertAlles:
            for zahl in range(1, 11):
                allValues[zahl] = set()
        """
        self.paraDictGenerated = {}
        self.paraDictGenerated4htmlTags = {}
        for key, value in paraNdataMatrix4onlyGenerated.items():
            for firstParameter in value[0][1:]:
                for secondParameter in value[1][1:]:
                    self.paraDictGenerated[(firstParameter, secondParameter)] = key
            self.paraDictGenerated4htmlTags[(value[0][0], value[1][0])] = key
            allValues[7] |= {key}
        """

        paraNdataMatrix += [
            (
                Program.ParametersMain.alles,
                (),
                *allValues,
            )
        ]
        """
        Hier wird erreicht, dass beide Dictionaries stückweise aufgefüllt werden.
        Aus den 3 voran gegangen Datenstrukturen werden 2 Dicts gemacht.
        """
        self.paraMainDict, self.paraDict = {}, {}
        for parameterEntry in paraNdataMatrix:
            into = intoParameterDatatype(
                parameterEntry[0],
                parameterEntry[1],
                tuple(
                    parameterEntryElement
                    for parameterEntryElement in parameterEntry[2:]
                ),
            )
            self.paraDict, self.dataDict = mergeParameterDicts(
                self.paraMainDict,
                self.paraDict,
                self.dataDict,
                *into,
            )

        self.dataDict[3] = Program.kombiParaNdataMatrix
        self.dataDict[8] = Program.kombiParaNdataMatrix2

        # alxp(self.paraDictGenerated)
        # alxp("-|-|")
        # alxp(self.paraDictGenerated4htmlTags)
        # alxp("||-|")
        # alxp(self.paraDict)
        # alxp("--|-")
        # alxp(self.dataDict)
        # alxp("--||")
        self.tables.dataDict = self.dataDict

    def parametersToCommandsAndNumbers(
        self, argv, neg=""
    ) -> Iterable[Union[set, set, set, list]]:
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
        self.__willBeOverwritten_rowsOfcombi: set = OrderedSet()
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
                        breiteIstNull = "".join(
                            ("--", i18n.ausgabeParas["breite"], "=0")
                        )
                        outputtype = arg[(arg.find("=") + 1) :]
                        if outputtype == i18n.ausgabeArt["shell"]:
                            self.tables.outType = OutputSyntax()
                        elif outputtype == i18n.ausgabeArt["nichts"]:
                            self.tables.outType = NichtsSyntax()
                        elif outputtype == i18n.ausgabeArt["csv"]:
                            self.tables.outType = csvSyntax()
                            self.tables.getOut.oneTable = True
                            self.breiteBreitenSysArgvPara(breiteIstNull[2:], "")
                        elif outputtype == i18n.ausgabeArt["bbcode"]:
                            self.htmlOrBBcode = True
                            self.tables.outType = bbCodeSyntax()
                        elif outputtype == i18n.ausgabeArt["html"]:
                            self.tables.outType = htmlSyntax()
                            self.htmlOrBBcode = True
                        elif outputtype == i18n.ausgabeArt["emacs"]:
                            self.tables.getOut.oneTable = True
                            self.tables.outType = emacsSyntax()
                            self.breiteBreitenSysArgvPara(breiteIstNull[2:], "")
                        elif outputtype == i18n.ausgabeArt["markdown"]:
                            self.tables.outType = markdownSyntax()
                            self.tables.getOut.oneTable = True
                            self.breiteBreitenSysArgvPara(breiteIstNull[2:], "")
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
                        infoLog = True
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
            self.__willBeOverwritten_rowsOfcombi,
            spaltenreihenfolgeundnurdiese,
            puniverseprims_only,
            generRows,
        )

    def helpPage(self):
        global folder
        retaHilfe()

    def bringAllImportantBeginThings(self, argv) -> tuple:
        """Einlesen der ersten Tabelle "religion.csv" zu self.relitable
        aller anderen csv dateien
        Parameter werden in Befehle und Nummernlisten gewandelt
        csv Dateien werden angehangen an self.relitable


        @rtype: tuple(int,set,set,list,set,list,set,list,list)
        @return: Spaltenanzahl, Zeilen Ja, Zeilen Nein, Religionstabelle, Spalten, weitere Tabelle daneben, spalten weitere Tabelle, weitere Tabelle für wie sql-join, deren spalten
        """
        global folder, shellRowsAmount
        if "Brython" not in sys.version.split():
            place = os.path.join(
                os.getcwd(),
                os.path.dirname(__file__),
                "csv",
                os.path.basename(csvFileNames.religion),
            )
        else:
            exit()
            place = csvFileNames.religion
        with open(place, mode="r", encoding="utf-8") as csv_file:
            self.relitable: list = []
            # maxi: dict = {}
            # tabneu = np.chararray((len(self.relitable) + 1, self.tables.hoechsteZeile[1024] + 3), itemsize=5000, unicode = True)
            # self.relitable = np.chararray((len(self.relitable) + 1, self.tables.hoechsteZeile[1024] + 3), itemsize=5000, unicode = True)
            for i, col in enumerate(csv.reader(csv_file, delimiter=";")):
                if (
                    "".join(
                        ("--", i18n.ausgabeParas["art"], "=", i18n.ausgabeArt["bbcode"])
                    )
                    in self.argv
                ):
                    col = [
                        json.loads(ccc[1:-1])["bbcode"]
                        if ccc[:2] == "|{" and ccc[-2:] == "}|"
                        else ccc
                        for ccc in col
                    ]
                elif (
                    "".join(
                        ("--", i18n.ausgabeParas["art"], "=", i18n.ausgabeArt["html"])
                    )
                    in self.argv
                ):
                    col = [
                        json.loads(ccc[1:-1])["html"]
                        if ccc[:2] == "|{" and ccc[-2:] == "}|"
                        else html.escape(ccc, quote=True)
                        for ccc in col
                    ]
                else:
                    col = [
                        json.loads(ccc[1:-1])[""]
                        if ccc[:2] == "|{" and ccc[-2:] == "}|"
                        else ccc
                        for ccc in col
                    ]
                self.relitable += [col]
                if i == 0:
                    self.RowsLen = len(col)
            for egal in range(
                len(self.relitable) + 1, self.tables.hoechsteZeile[1024] + 2
            ):
                self.relitable += [[""] * len(self.relitable[0])]
        changeMotivesColumn = i18n.tomDecodedMotivesLang["kr"] if i18n.sprachen[i18n.sprachenWahl] == "kr" else i18n.tomDecodedMotivesLang["cn"] if i18n.sprachen[i18n.sprachenWahl] == "cn" else i18n.tomDecodedMotivesLang["vn"] if i18n.sprachen[i18n.sprachenWahl] == "vn" else ""
        if changeMotivesColumn not in ("", "de"):
            place = os.path.join(
                os.getcwd(),
                os.path.dirname(__file__),
                "csv",
                os.path.basename(changeMotivesColumn),
            )
            with open(place, mode="r", encoding="utf-8") as csv_file:
                for i, col in enumerate(csv.reader(csv_file, delimiter=";")):
                    try:
                        self.relitable[i][10] = col[0]
                    except IndexError:
                        pass
        self.htmlOrBBcode = False
        self.breiteORbreiten = False
        self.keineleereninhalte = False
        self.tables.keineleereninhalte = False
        (
            paramLines,
            self.rowsAsNumbers,
            self.rowsOfcombi,
            spaltenreihenfolgeundnurdiese,
            self.puniverseprims,
            self.generRows,
        ) = self.parametersToCommandsAndNumbers(argv)
        (
            paramLinesNot,
            self.rowsAsNumbersNot,
            self.rowsOfcombiNot,
            spaltenreihenfolgeundnurdieseNot,
            self.puniverseprimsNot,
            self.generRowsNot,
        ) = self.parametersToCommandsAndNumbers(argv, "-")
        self.dataDict: list = [{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}]
        self.spaltenTypeNaming: namedtuple = namedtuple(
            "SpaltenTyp",
            "ordinary generated1 concat1 kombi1 boolAndTupleSet1 gebroUni1 gebrGal1 generated2 kombi2 gebrEmo1 gebrGroe1 metakonkret ordinaryNot generate1dNot concat1Not kombi1Not boolAndTupleSet1Not gebroUni1Not gebrGal1Not generated2Not kombi2Not gebrEmo1Not gebrGroe1Not metakonkretNot",
        )
        self.spaltenTypeNaming = self.spaltenTypeNaming(
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),
            (0, 8),
            (0, 9),
            (0, 10),
            (0, 11),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 4),
            (1, 5),
            (1, 6),
            (1, 7),
            (1, 8),
            (1, 9),
            (1, 10),
            (1, 11),
        )
        self.spaltenArtenKey_SpaltennummernValue = {
            (0, 0): OrderedSet(),
            (0, 1): OrderedSet(),
            (0, 2): OrderedSet(),
            (0, 3): OrderedSet(),
            (0, 4): OrderedSet(),
            (0, 5): OrderedSet(),
            (0, 6): OrderedSet(),
            (0, 7): OrderedSet(),
            (0, 8): OrderedSet(),
            (0, 9): OrderedSet(),
            (0, 10): OrderedSet(),
            (0, 11): OrderedSet(),
            (1, 0): OrderedSet(),
            (1, 1): OrderedSet(),
            (1, 2): OrderedSet(),
            (1, 3): OrderedSet(),
            (1, 4): OrderedSet(),
            (1, 5): OrderedSet(),
            (1, 6): OrderedSet(),
            (1, 7): OrderedSet(),
            (1, 8): OrderedSet(),
            (1, 9): OrderedSet(),
            (1, 10): OrderedSet(),
            (1, 11): OrderedSet(),
        }
        # x(
        #    "onlyGeneratedA",
        #    self.spaltenArtenKey_SpaltennummernValue[
        #        self.spaltenTypeNaming.boolAndTupleSet1
        #    ],
        # )

        # x(
        #    "NEIN_X1",
        #    self.spaltenArtenKey_SpaltennummernValue[self.spaltenTypeNaming.gebrGal1],
        # )
        self.storeParamtersForColumns()
        # x(
        #    "onlyGeneratedB",
        #    self.spaltenArtenKey_SpaltennummernValue[
        #        self.spaltenTypeNaming.boolAndTupleSet1
        #    ],
        # )
        # x(
        #    "NEIN_X2",
        #    self.spaltenArtenKey_SpaltennummernValue[self.spaltenTypeNaming.gebrGal1],
        # )
        self.produceAllSpaltenNumbers()
        # x(
        #    "onlyGeneratedC",
        #    self.spaltenArtenKey_SpaltennummernValue[
        #        self.spaltenTypeNaming.boolAndTupleSet1
        #    ],
        # )
        # x(
        #    "NEIN_X3",
        #    self.spaltenArtenKey_SpaltennummernValue[self.spaltenTypeNaming.gebrGal1],
        # )
        if self.htmlOrBBcode and not self.breiteORbreiten:
            shellRowsAmount = 0
            self.tables.textWidth = 0

        paramLines, paramLinesNot = self.tables.getPrepare.deleteDoublesInSets(
            paramLines, paramLinesNot
        )
        self.rowsAsNumbers = self.spaltenArtenKey_SpaltennummernValue[
            self.spaltenTypeNaming.ordinary
        ]
        self.generRows = self.spaltenArtenKey_SpaltennummernValue[
            self.spaltenTypeNaming.generated1
        ]
        self.puniverseprims = self.spaltenArtenKey_SpaltennummernValue[
            self.spaltenTypeNaming.concat1
        ]
        self.rowsOfcombi = self.spaltenArtenKey_SpaltennummernValue[
            self.spaltenTypeNaming.kombi1
        ]
        self.rowsOfcombi2 = self.spaltenArtenKey_SpaltennummernValue[
            self.spaltenTypeNaming.kombi2
        ]
        self.onlyGenerated = self.spaltenArtenKey_SpaltennummernValue[
            self.spaltenTypeNaming.boolAndTupleSet1
        ]
        ones = []
        # x(
        #    "onlyGeneratedD",
        #    self.spaltenArtenKey_SpaltennummernValue[
        #        self.spaltenTypeNaming.boolAndTupleSet1
        #    ],
        # )
        for a in self.onlyGenerated:
            if len(a) == 1:
                ones += a
        self.tables.getConcat.ones = ones

        # for gebrUniva in self.gebrUni:
        #    self.tables.gebrUnivSet.add(gebrUniva)
        # for prims in self.puniverseprims:
        #    self.tables.primUniversePrimsSet.add(prims)

        if len(self.rowsOfcombi) > 0:
            paramLines.add("ka")
        if len(self.rowsOfcombi2) > 0:
            paramLines.add("ka2")
        self.tables.generRows = self.generRows
        self.tables.getPrepare.rowsAsNumbers = self.rowsAsNumbers
        self.tables.getOut.rowsAsNumbers = self.rowsAsNumbers

        self.tables.SpaltenVanillaAmount = len(self.rowsAsNumbers)

        CsvTheirsSpalten: dict = {}
        for i, input1 in enumerate(
            (
                self.puniverseprims,
                self.spaltenArtenKey_SpaltennummernValue[
                    self.spaltenTypeNaming.gebrGal1
                ],
                self.spaltenArtenKey_SpaltennummernValue[
                    self.spaltenTypeNaming.gebrGal1
                ],
                self.spaltenArtenKey_SpaltennummernValue[
                    self.spaltenTypeNaming.gebroUni1
                ],
                self.spaltenArtenKey_SpaltennummernValue[
                    self.spaltenTypeNaming.gebroUni1
                ],
                self.spaltenArtenKey_SpaltennummernValue[
                    self.spaltenTypeNaming.gebrEmo1
                ],
                self.spaltenArtenKey_SpaltennummernValue[
                    self.spaltenTypeNaming.gebrEmo1
                ],
                self.spaltenArtenKey_SpaltennummernValue[
                    self.spaltenTypeNaming.gebrGroe1
                ],
                self.spaltenArtenKey_SpaltennummernValue[
                    self.spaltenTypeNaming.gebrGroe1
                ],
            ),
            start=1,
        ):
            # x("nix1", [self.rowsAsNumbers, input1, i])
            (
                self.relitable,
                rowsAsNumbers,
                CsvTheirsSpalten[i],
            ) = self.tables.getConcat.readConcatCsv(
                self.relitable, self.rowsAsNumbers, input1, i
            )
            # x("nix2", [self.rowsAsNumbers, CsvTheirsSpalten[i]])
        primSpalten = CsvTheirsSpalten[1]
        gebr: dict = {}
        gebr["Gal"] = CsvTheirsSpalten[2]
        gebr["Gal2"] = CsvTheirsSpalten[3]
        gebr["Uni"] = CsvTheirsSpalten[4]
        gebr["Uni2"] = CsvTheirsSpalten[5]
        gebr["Emo"] = CsvTheirsSpalten[6]
        gebr["Emo2"] = CsvTheirsSpalten[7]
        gebr["Groe"] = CsvTheirsSpalten[8]
        gebr["Groe2"] = CsvTheirsSpalten[9]
        # x("gebR", gebr)

        (
            finallyDisplayLinesEarly,
            headingsAmountEarly,
            newerTableEarly,
            numlenEarly,
            rowsRangeEarly,
        ) = self.tables.getPrepare.prepare4out_beforeForLoop_SpaltenZeilenBestimmen(
            self.relitable, paramLines, paramLinesNot
        )
        zeilenliste = list(finallyDisplayLinesEarly)
        zeilenliste.sort()
        self.tables.lastLineNumber = zeilenliste[-1]

        (
            self.relitable,
            self.rowsAsNumbers,
        ) = self.tables.getConcat.concatVervielfacheZeile(
            self.relitable, self.rowsAsNumbers
        )
        self.relitable, self.rowsAsNumbers = self.tables.getConcat.concatModallogik(
            self.relitable, self.tables.generRows, self.rowsAsNumbers
        )
        (
            self.relitable,
            self.rowsAsNumbers,
        ) = self.tables.getConcat.concatPrimCreativityType(
            self.relitable, self.rowsAsNumbers
        )

        (
            self.relitable,
            self.rowsAsNumbers,
        ) = self.tables.getConcat.concatGleichheitFreiheitDominieren(
            self.relitable, self.rowsAsNumbers
        )

        (
            self.relitable,
            self.rowsAsNumbers,
        ) = self.tables.getConcat.concatGeistEmotionEnergieMaterieTopologie(
            self.relitable, self.rowsAsNumbers
        )

        (
            self.relitable,
            self.rowsAsNumbers,
        ) = self.tables.getConcat.concatMondExponzierenLogarithmusTyp(
            self.relitable, self.rowsAsNumbers
        )

        paraTextNamen = {}
        # x("NIXDA", self.spaltenArtenKey_SpaltennummernValue[(0, 7)])
        # x("NIXDA2", self.spaltenArtenKey_SpaltennummernValue[(0, 11)])
        for text in self.spaltenArtenKey_SpaltennummernValue[(0, 7)]:
            paraTextNamen[text] = [self.dataDict[7][text]]

        (
            self.relitable,
            self.rowsAsNumbers,
        ) = self.tables.getConcat.concat1RowPrimUniverse2(
            self.relitable,
            self.rowsAsNumbers,
            self.spaltenArtenKey_SpaltennummernValue[(0, 7)],
            paraTextNamen,
        )

        (
            self.relitable,
            self.rowsAsNumbers,
        ) = self.tables.getConcat.concat1PrimzahlkreuzProContra(
            self.relitable,
            self.rowsAsNumbers,
            self.spaltenArtenKey_SpaltennummernValue[(0, 7)],
            Program.ParametersMain,
        )

        (
            self.relitable,
            self.rowsAsNumbers,
        ) = self.tables.getConcat.concatLovePolygon(self.relitable, self.rowsAsNumbers)
        (
            self.relitable,
            rowsAsNumbers,
        ) = self.tables.getConcat.spalteFuerGegenInnenAussenSeitlichPrim(
            self.relitable, self.rowsAsNumbers
        )

        couplesX = list(self.spaltenArtenKey_SpaltennummernValue[(0, 11)])
        # x("couplesX", couplesX)

        (
            self.relitable,
            self.rowsAsNumbers,
        ) = self.tables.getConcat.spalteMetaKontretTheorieAbstrakt_etc_1(
            self.relitable, self.rowsAsNumbers, couplesX
        )

        self.tables.getMainTable.createSpalteGestirn(self.relitable, self.rowsAsNumbers)

        if len(self.rowsOfcombi) > 0:
            (
                animalsProfessionsTable,
                self.relitable,
                kombiTable_Kombis,
                maintable2subtable_Relation,
            ) = self.tables.getCombis.readKombiCsv(
                self.relitable,
                self.rowsAsNumbers,
                self.rowsOfcombi,
                csvFileNames.kombi13,
            )
        else:
            animalsProfessionsTable = []
            kombiTable_Kombis = []
            maintable2subtable_Relation = []

        if len(self.rowsOfcombi2) > 0:
            (
                animalsProfessionsTable2,
                self.relitable,
                kombiTable_Kombis2,
                maintable2subtable_Relation2,
            ) = self.tables.getCombis.readKombiCsv(
                self.relitable,
                self.rowsAsNumbers,
                self.rowsOfcombi2,
                csvFileNames.kombi15,
            )
        else:
            animalsProfessionsTable2 = []
            kombiTable_Kombis2 = []
            maintable2subtable_Relation2 = []

        return (
            self.RowsLen,
            paramLines,
            paramLinesNot,
            self.relitable,
            self.rowsAsNumbers,
            animalsProfessionsTable,
            self.rowsOfcombi,
            kombiTable_Kombis,
            maintable2subtable_Relation,
            spaltenreihenfolgeundnurdiese,
            primSpalten,
            gebr,
            animalsProfessionsTable2,
            kombiTable_Kombis2,
            maintable2subtable_Relation2,
        )

    def oberesMaximumArg(self, arg) -> tuple:
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

    def oberesMaximum2(self, argv2) -> Optional[int]:
        try:
            werte: list = [self.tables.hoechsteZeile[1024]]
        except:
            werte: list = []
        for arg in argv2:
            werte += self.oberesMaximumArg(arg)[0]

        return max(werte) if len(werte) > 0 else None

    def oberesMaximum(self, arg) -> bool:
        liste, wahrheitswert = self.oberesMaximumArg(arg)
        if len(liste) == 0 or not wahrheitswert:
            return False
        max_ = max(liste + [self.tables.hoechsteZeile[1024]])
        self.tables.hoechsteZeile = max_
        return True

    @property
    def propInfoLog(self) -> OutputSyntax:
        global Tables, infoLog
        return infoLog

    @propInfoLog.setter
    def propInfoLog(self, value: bool):
        global Tables, infoLog
        infoLog = value

    def __init__(
        self,
        argv=[],
        alternativeShellRowsAmount: Optional[int] = None,
        Txt=None,
        runAlles=True,
    ):
        global Tables, infoLog
        self.argv = [a.strip() for a in argv]
        self.allesParameters = 0
        self.tables = Tables(self.oberesMaximum2(argv[1:]), Txt)

        self.breiteHasBeenOnceZero: bool = False
        self.obZeilenBereicheAngegeben = False
        if platform.system() == "Windows":
            self.tables.getOut.color = False

        self.__runAlles = runAlles
        self.__invertAlles = False
        if runAlles:
            self.__resultingTable = self.workflowEverything(self.argv)

    def invertAlles(self):
        self.__invertAlles = True

    def run(self):
        if not self.__runAlles:
            self.__resultingTable = self.workflowEverything(self.argv)

    @property
    def resultingTable(self) -> list:
        return self.__resultingTable

    def workflowEverything(self, argv) -> list:
        global infoLog
        (
            self.RowsLen,
            paramLines,
            paramLinesNot,
            self.relitable,
            self.rowsAsNumbers,
            animalsProfessionsTable,
            self.rowsOfcombi,
            kombiTable_Kombis,
            maintable2subtable_Relation,
            spaltenreihenfolgeundnurdiese,
            primSpalten,
            gebr,
            animalsProfessionsTable2,
            kombiTable_Kombis2,
            maintable2subtable_Relation2,
        ) = self.bringAllImportantBeginThings(argv)

        # x("gebr", gebr)
        (
            finallyDisplayLines,
            newTable,
            numlen,
            rowsRange,
            old2newTable,
        ) = self.tables.getPrepare.prepare4out(
            paramLines,
            paramLinesNot,
            self.relitable,
            self.rowsAsNumbers,
            gebrSpalten=gebr,
            primSpalten=primSpalten,
        )

        if len(self.rowsOfcombi) > 0:
            newTable = self.combiTableWorkflow(
                animalsProfessionsTable,
                finallyDisplayLines,
                kombiTable_Kombis,
                maintable2subtable_Relation,
                newTable,
                old2newTable,
                paramLines,
                csvFileNames.kombi13,
            )

        if len(self.rowsOfcombi2) > 0:
            newTable = self.combiTableWorkflow(
                animalsProfessionsTable2,
                finallyDisplayLines,
                kombiTable_Kombis2,
                maintable2subtable_Relation2,
                newTable,
                old2newTable,
                paramLines,
                csvFileNames.kombi15,
            )

        newTable = self.tables.getOut.onlyThatColumns(
            newTable, spaltenreihenfolgeundnurdiese
        )
        self.newTable = newTable
        self.finallyDisplayLines = finallyDisplayLines
        self.rowsRange = rowsRange
        self.numlen = numlen

        return self.tables.getOut.cliOut(
            finallyDisplayLines, newTable, numlen, rowsRange
        )

    def combiTableWorkflow(
        self,
        animalsProfessionsTable,
        finallyDisplayLines,
        kombiTable_Kombis,
        maintable2subtable_Relation,
        newTable,
        old2newTable,
        paramLines,
        csvFileName,
    ):
        """alle  Schritte für kombi:
        1. lesen: KombiTable und relation, was von kombitable zu haupt gehört
                  und matrix mit zellen sind zahlen der kombinationen
                  d.h. 3 Sachen sind das Ergebnis
        2. prepare: die Zeilen, die infrage kommen für Kombi, d.h.:
                                key = haupttabellenzeilennummer
                                value = kombitabellenzeilennummer
        3. Zeilenumbruch machen, wie es bei der Haupt+Anzeige-Tabelle auch gemacht wurde
           prepare4out
        4. Vorbereiten des Joinens beider Tabellen direkt hier rein programmiert
           (Müsste ich unbedingt mal refactoren!)
        5. joinen
           Wenn ich hier jetzt alles joine, und aber nicht mehrere Zellen mache pro Kombitablezeile,
           d.h. nicht genauso viele Zeilen wie es der Kombitablezeilen entspricht,
           d.h. ich mache nur eine Zeile, in der ich alle kombitableteilen nur konkatteniere,
           dann ist das Ergebnis Mist in der Ausagbe, weil der Zeilenumbruch noch mal gemacht werden müsste,
           der jedoch bereits schon gemacht wurde.
           Der musste aber vorher gemacht werden, denn wenn man ihn jetzt machen würde,
           dann müsste man das eigentlich WIEDER mit der ganzen Tabelle tun!
           Also etwa alles völlig umprogrammieren?
        6. noch mal nur das ausgeben lassen, das nur ausgegeben werden soll
        7. letztendliche Ausagebe von allem!!
        """
        ChosenKombiLines = self.tables.getCombis.prepare_kombi(
            finallyDisplayLines,
            animalsProfessionsTable,
            paramLines,
            finallyDisplayLines,
            kombiTable_Kombis,
        )
        komb_rows = (
            self.rowsOfcombi
            if csvFileName == csvFileNames.kombi13
            else (self.rowsOfcombi2 if csvFileName == csvFileNames.kombi15 else None)
        )
        (
            finallyDisplayLines_kombi,
            newTable_kombi_1,
            lineLen_kombi_1,
            animalsProfessionsTable,
            old2newTableAnimalsProfessions,
        ) = self.tables.getPrepare.prepare4out(
            OrderedSet(),
            OrderedSet(),
            animalsProfessionsTable,
            komb_rows,
            {},
            self.tables.getCombis.sumOfAllCombiRowsAmount,
            reliTableLenUntilNow=len(newTable[0])
            - (
                len(self.rowsOfcombi) + len(self.rowsOfcombi2)
                if csvFileName == csvFileNames.kombi13
                else len(self.rowsOfcombi2)
                if csvFileName == csvFileNames.kombi15
                else None
            ),
            kombiCSVNumber=0
            if csvFileName == csvFileNames.kombi13
            else 1
            if csvFileName == csvFileNames.kombi15
            else None,
        )
        KombiTables = self.tables.getCombis.prepareTableJoin(
            ChosenKombiLines, newTable_kombi_1
        )
        newTable = self.tables.getCombis.tableJoin(
            newTable,
            KombiTables,
            maintable2subtable_Relation,
            old2newTable,
            komb_rows,
        )
        return newTable


if __name__ == "__main__":
    # try:
    Program(sys.argv)
    # except KeyboardInterrupt:
    #    sys.exit()
