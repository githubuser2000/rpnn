#!/usr/bin/env pypy3
# -*- coding: utf-8 -*-
import json
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "libs"))
from collections import OrderedDict
from copy import deepcopy
from functools import cmp_to_key
from pprint import pprint

from LibRetaPrompt import wahl15

import i18n.words as i18n


def cmp_before(value):
    value = value[0]
    isNumber: bool = True
    if "/" in value:
        a = value.split("/")[-1]
        if a.isdecimal():
            toSort = a
        else:
            isNumber = False
    elif value.isdecimal():
        toSort = value
    else:
        isNumber = False
    if not isNumber:
        toSort = value
        # print("value: " + str(toSort))
    return isNumber, toSort


def cmpx(erster, zweiter):
    isNumber1, value1 = cmp_before(erster)
    isNumber2, value2 = cmp_before(zweiter)
    if isNumber1 and isNumber2:
        value1 = int(value1)
        value2 = int(value2)
        if value1 == value2:
            if "/" in erster[0]:
                return 1
            elif "/" in zweiter[0]:
                return -1
            else:
                return 0
        else:
            return value1 - value2
    # else:
    #    print(str(isNumber1) + "-" + str(isNumber2))
    #    print(str(erster) + "-" + str(zweiter))
    elif isNumber1 and not isNumber2:
        return 1
    elif not isNumber1 and isNumber2:
        # print(str(erster) + "+" + str(zweiter))
        return -1
    else:
        return value1 < value2


def merge_dicts(dict1, dict2):
    for key in dict2:
        if (
            key in dict1
            and isinstance(dict1[key], OrderedDict)
            and isinstance(dict2[key], OrderedDict)
        ):
            merge_dicts(dict1[key], dict2[key])
        else:
            if key in dict1:
                if isinstance(dict2[key], OrderedDict) and not isinstance(
                    dict1[key], OrderedDict
                ):
                    # print(str(type(dict1[key])))
                    dict1[key] = OrderedDict(
                        sorted(
                            OrderedDict({dict1[key]: None}).update(dict2[key]),
                            key=cmp_to_key(cmpx),
                        )
                    )
            else:
                dict1[key] = dict2[key]

    return OrderedDict(sorted(dict1.items(), key=cmp_to_key(cmpx)))


def traverseHierarchy(liste, thing, listenIndex, value):
    # print(listenIndex)
    # print(liste[listenIndex:])
    # print(tuple(reversed(liste[listenIndex:])))
    knoten = liste[listenIndex]
    knoten = knoten.replace("pro", "/")
    # print(liste)
    # print(knoten)
    # print(thing.keys())
    # if "relativer_Zeit-Betrag_(15_10_4_18_6)" == value:
    #    print(liste)
    #    print(listenIndex)
    if listenIndex == 0:
        thing: dict
        newKeys = value.split(",")
        newValues = [None] * len(newKeys)
        thing.update(OrderedDict(sorted(zip(newKeys, newValues), key=cmp_to_key(cmpx))))
        # if "relativer_Zeit-Betrag_(15_10_4_18_6)" == value:
        #    print(thing)
    thing = OrderedDict(sorted({knoten: thing}.items(), key=cmp_to_key(cmpx)))
    if len(liste) > listenIndex + 1:
        # print("SDASDFGGFGFSGSDFG")
        thing = traverseHierarchy(liste, thing, listenIndex + 1, value)
        # print(thing[knoten])
    return thing


wahlNeu: dict = OrderedDict(sorted({}.items(), key=cmp_to_key(cmpx)))

liste: list
for key, value in wahl15.items():
    key = "_" + key
    liste = key.split("_")
    liste = list(filter(None, liste))
    thing: dict = OrderedDict(sorted({}.items(), key=cmp_to_key(cmpx)))
    if len(liste) > 0:
        thing = traverseHierarchy(tuple(reversed(liste)), thing, 0, value)
        wahlNeu = merge_dicts(thing, wahlNeu)


wahlNeu2: OrderedDict = OrderedDict(sorted({}, key=cmp_to_key(cmpx)))
wahlNeu2["15"] = OrderedDict(sorted(wahlNeu.items(), key=cmp_to_key(cmpx)))
wahlNeu2 = merge_dicts(
    wahlNeu2, OrderedDict(sorted(wahlNeu["15"].items(), key=cmp_to_key(cmpx)))
)


# pprint(json.dumps(wahlNeu2))
# print("-------------------")
# pprint(wahlNeu2)


# print("<br>BLAAAAAAAAAAAAAAAAA<br>")

blank = len(sys.argv) > 1 and sys.argv[1] == "blank"


def myprint(d, tiefe):
    global blank
    bereich = d.items()
    for k, v in bereich if tiefe < 2 else reversed(bereich):
        bereichLen = (v is not None and len(v.items()) > 1) or tiefe < 2
        listenVergleich = (
            v is not None
            and any([vValue is not None for vKey, vValue in v.items()])
            and len(v.items()) > 1
            or tiefe < 2
        )
        if bereichLen:
            print(
                "".join(
                    (
                        '<div style="',
                        'white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);" ',
                        ">",
                    )
                ),
                end="",
            )
        if v is None:
            print(
                "".join(
                    (
                        '<input type="checkbox"',
                        (
                            "".join(
                                (
                                    ' class="ordGru" onchange="toggleP2(this,-10,',
                                    "'",
                                    "✗",
                                    i18n.ParametersMain.grundstrukturen[0],
                                    ",",
                                    k,
                                    "');\"",
                                    ' id="ordGru',
                                    k,
                                    '" value="',
                                    k,
                                    '"',
                                ),
                            )
                            if blank
                            else ""
                        ),
                        ">",
                    )
                ),
                end="",
            )

        if v is None or listenVergleich:
            if v is None:
                kkk = "".join(
                    ('<label id="ordGruB', k, '">', k.replace("_", " "), "</label>")
                )
            else:
                kkk = k
            print("{0} ".format(kkk), end="")
        if v is None:
            print("</input>", end="")
        if v is not None:
            myprint(v, tiefe + 1)
        if bereichLen:
            print("</div>", end="")


print(
    "".join(
        (
            '<div style="',
            ("display:none;" if blank and False else ""),
            'white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);" ',
            ("id='grundstrukturenDiv'" if blank else ""),
            ">",
        )
    ),
    end="",
)

myprint(wahlNeu2, 0)
print("</div>")
