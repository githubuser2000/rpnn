use crate::shared::exact_i18n::I18nExact;

#[derive(Clone, Debug)]
pub enum PyNode {
    NoneValue,
    Dict(Vec<(String, PyNode)>),
}

pub fn cmp_before(value: &(String, PyNode)) -> (bool, String) {
    let value = &value.0;
    let mut is_number: bool = true;
    let to_sort: String;
    if value.contains("/") {
        let a = value.split('/').last().unwrap_or("");
        if a.chars().all(|c| c.is_ascii_digit()) {
            to_sort = a.to_string();
        } else {
            is_number = false;
            to_sort = value.to_string();
        }
    } else if value.chars().all(|c| c.is_ascii_digit()) {
        to_sort = value.to_string();
    } else {
        is_number = false;
        to_sort = value.to_string();
    }
    (is_number, to_sort)
}

pub fn cmpx(erster: &(String, PyNode), zweiter: &(String, PyNode)) -> std::cmp::Ordering {
    let (is_number1, value1) = cmp_before(erster);
    let (is_number2, value2) = cmp_before(zweiter);

    if is_number1 && is_number2 {
        let value1: i64 = value1.parse().unwrap_or(0);
        let value2: i64 = value2.parse().unwrap_or(0);
        if value1 == value2 {
            if erster.0.contains("/") {
                std::cmp::Ordering::Greater
            } else if zweiter.0.contains("/") {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        } else if value1 < value2 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    } else if is_number1 && !is_number2 {
        std::cmp::Ordering::Greater
    } else if !is_number1 && is_number2 {
        std::cmp::Ordering::Less
    } else if value1 < value2 {
        std::cmp::Ordering::Less
    } else if value1 > value2 {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

pub fn sorted(mut d: Vec<(String, PyNode)>) -> Vec<(String, PyNode)> {
    d.sort_by(cmpx);
    d
}

pub fn merge_dicts(dict1: Vec<(String, PyNode)>, dict2: Vec<(String, PyNode)>) -> Vec<(String, PyNode)> {
    let mut out = dict1.clone();
    for (key2, value2) in dict2 {
        let mut found = false;
        for (key1, value1) in out.iter_mut() {
            if *key1 == key2 {
                found = true;
                match (value1.clone(), value2.clone()) {
                    (PyNode::Dict(left), PyNode::Dict(right)) => {
                        *value1 = PyNode::Dict(merge_dicts(left, right));
                    }
                    _ => {}
                }
            }
        }
        if !found {
            out.push((key2, value2));
        }
    }
    sorted(out)
}

pub fn traverseHierarchy(liste: &[String], mut thing: Vec<(String, PyNode)>, listenIndex: usize, value: &str) -> Vec<(String, PyNode)> {
    let knoten = liste[listenIndex].replace("pro", "/");

    if listenIndex == 0 {
        for x in value.split(',') {
            thing.push((x.to_string(), PyNode::NoneValue));
        }
        thing = sorted(thing);
    }

    thing = sorted(vec![(knoten.clone(), PyNode::Dict(thing))]);

    if liste.len() > listenIndex + 1 {
        let inner = match thing[0].1.clone() {
            PyNode::Dict(v) => v,
            PyNode::NoneValue => vec![],
        };
        thing = traverseHierarchy(liste, inner, listenIndex + 1, value);
        thing = sorted(vec![(knoten, PyNode::Dict(thing))]);
    }

    thing
}

pub fn myprint(d: Vec<(String, PyNode)>, tiefe: usize, blank: bool, grund_name: &str, out: &mut String) {
    let iter: Vec<(String, PyNode)> = if tiefe < 2 { d } else { let mut r = d; r.reverse(); r };

    for (k, v) in iter {
        let bereich_len = match &v {
            PyNode::Dict(inner) => inner.len() > 1 || tiefe < 2,
            PyNode::NoneValue => tiefe < 2,
        };
        let listen_vergleich = match &v {
            PyNode::Dict(inner) => (inner.iter().any(|(_, vv)| !matches!(vv, PyNode::NoneValue)) && inner.len() > 1) || tiefe < 2,
            PyNode::NoneValue => tiefe < 2,
        };

        if bereich_len {
            out.push_str("<div style=\"white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);\" >");
        }

        if matches!(v, PyNode::NoneValue) {
            out.push_str("<input type=\"checkbox\"");
            if blank {
                out.push_str(&format!(
                    " class=\"ordGru\" onchange=\"toggleP2(this,-10,'✗{},{}');\" id=\"ordGru{}\" value=\"{}\"",
                    grund_name, k, k, k
                ));
            }
            out.push_str(">");
        }

        if matches!(v, PyNode::NoneValue) || listen_vergleich {
            if matches!(v, PyNode::NoneValue) {
                out.push_str(&format!("<label id=\"ordGruB{}\">{}</label> ", k, k.replace('_', " ")));
            } else {
                out.push_str(&format!("{} ", k));
            }
        }

        if matches!(v, PyNode::NoneValue) {
            out.push_str("</input>");
        }

        if let PyNode::Dict(inner) = v {
            myprint(inner, tiefe + 1, blank, grund_name, out);
        }

        if bereich_len {
            out.push_str("</div>");
        }
    }
}

pub fn grundstruk_html_from_i18n(i18n: &I18nExact, blank: bool) -> String {
    let mut wahlNeu: Vec<(String, PyNode)> = sorted(vec![]);
    for (key0, value) in i18n.wahl15Words.iter() {
        let key = format!("_{}", key0.trim_start_matches('_'));
        let mut liste: Vec<String> = key.split('_').filter(|x| !x.is_empty()).map(|x| x.to_string()).collect();
        let mut thing: Vec<(String, PyNode)> = sorted(vec![]);
        if !liste.is_empty() {
            liste.reverse();
            thing = traverseHierarchy(&liste, thing, 0, value);
            wahlNeu = merge_dicts(thing, wahlNeu);
        }
    }

    let mut wahlNeu2: Vec<(String, PyNode)> = vec![("15".to_string(), PyNode::Dict(sorted(wahlNeu.clone())))];
    if let Some((_, PyNode::Dict(inner))) = wahlNeu.iter().find(|(k, _)| k == "15").cloned() {
        wahlNeu2 = merge_dicts(wahlNeu2, sorted(inner));
    }

    let mut out = String::new();
    out.push_str(&format!(
        "<div style=\"white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);\" {}>",
        if blank { "id='grundstrukturenDiv'" } else { "" }
    ));
    myprint(wahlNeu2, 0, blank, &i18n.grundstrukturen_name, &mut out);
    out.push_str("</div>");
    out
}

pub const PYTHON_SOURCE__GRUNDSTRUKHTML: &str = r#"#!/usr/bin/env pypy3
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
"#;
