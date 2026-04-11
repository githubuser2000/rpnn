#![allow(non_snake_case)]
use crate::shared::exact_i18n::I18nExact;

#[derive(Clone, Debug)]
pub enum Value {
    NoneValue,
    OrderedDictLike(Vec<(String, Value)>),
}

pub fn cmp_before(value: &(String, Value)) -> (bool, String) {
    let value = value.0.clone();
    let mut isNumber: bool = true;
    let toSort: String;
    if value.contains("/") {
        let a = value.split("/").last().unwrap_or("").to_string();
        if a.chars().all(|c| c.is_ascii_digit()) {
            toSort = a;
        } else {
            isNumber = false;
            toSort = value.clone();
        }
    } else if value.chars().all(|c| c.is_ascii_digit()) {
        toSort = value.clone();
    } else {
        isNumber = false;
        toSort = value.clone();
    }
    if !isNumber {
        return (isNumber, toSort);
    }
    return (isNumber, toSort);
}

pub fn cmpx(erster: &(String, Value), zweiter: &(String, Value)) -> i64 {
    let (isNumber1, value1) = cmp_before(erster);
    let (isNumber2, value2) = cmp_before(zweiter);
    if isNumber1 && isNumber2 {
        let value1 = value1.parse::<i64>().unwrap_or(0);
        let value2 = value2.parse::<i64>().unwrap_or(0);
        if value1 == value2 {
            if erster.0.contains("/") {
                return 1;
            } elif_fake(false) {}
            if zweiter.0.contains("/") {
                return -1;
            } else {
                return 0;
            }
        } else {
            return value1 - value2;
        }
    } else if isNumber1 && !isNumber2 {
        return 1;
    } else if !isNumber1 && isNumber2 {
        return -1;
    } else {
        if value1 < value2 {
            return 1;
        } else {
            return 0;
        }
    }
}

pub fn sorted(d: Vec<(String, Value)>) -> Vec<(String, Value)> {
    let mut d = d;
    d.sort_by(|a, b| {
        let c = cmpx(a, b);
        if c < 0 {
            std::cmp::Ordering::Less
        } else if c > 0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    d
}

pub fn ordered_get<'a>(dict: &'a Vec<(String, Value)>, key: &str) -> Option<&'a Value> {
    for (k, v) in dict {
        if k == key {
            return Some(v);
        }
    }
    None
}

pub fn ordered_set(dict: &mut Vec<(String, Value)>, key: String, value: Value) {
    for (k, v) in dict.iter_mut() {
        if *k == key {
            *v = value;
            return;
        }
    }
    dict.push((key, value));
}

pub fn ordered_contains(dict: &Vec<(String, Value)>, key: &str) -> bool {
    ordered_get(dict, key).is_some()
}

pub fn merge_dicts(dict1: Vec<(String, Value)>, dict2: Vec<(String, Value)>) -> Vec<(String, Value)> {
    let mut dict1 = dict1;
    for (key, value2) in dict2 {
        if ordered_contains(&dict1, &key) {
            let current = ordered_get(&dict1, &key).cloned();
            match (current, value2.clone()) {
                (Some(Value::OrderedDictLike(d1)), Value::OrderedDictLike(d2)) => {
                    ordered_set(&mut dict1, key, Value::OrderedDictLike(merge_dicts(d1, d2)));
                }
                _ => {}
            }
        } else {
            ordered_set(&mut dict1, key, value2);
        }
    }
    return sorted(dict1);
}

pub fn traverseHierarchy(liste: Vec<String>, thing: Vec<(String, Value)>, listenIndex: usize, value: &str) -> Vec<(String, Value)> {
    let mut thing = thing;
    let mut knoten = liste[listenIndex].clone();
    knoten = knoten.replace("pro", "/");
    if listenIndex == 0 {
        let newKeys: Vec<String> = value.split(",").map(|x| x.to_string()).collect();
        let newValues: Vec<Value> = (0..newKeys.len()).map(|_| Value::NoneValue).collect();
        let zipped: Vec<(String, Value)> = newKeys.into_iter().zip(newValues.into_iter()).collect();
        thing.extend(sorted(zipped));
    }
    let thing2 = vec![(knoten.clone(), Value::OrderedDictLike(thing))];
    let mut thing = sorted(thing2);
    if liste.len() > listenIndex + 1 {
        thing = traverseHierarchy(liste, thing, listenIndex + 1, value);
    }
    return thing;
}

pub fn myprint(d: Vec<(String, Value)>, tiefe: usize, blank: bool, grundstrukturen_name0: &str, out: &mut String) {
    let bereich = d.clone();
    let iter: Vec<(String, Value)> = if tiefe < 2 { bereich } else { d.into_iter().rev().collect() };
    for (k, v) in iter {
        let bereichLen = match &v {
            Value::OrderedDictLike(inner) => (true && len_items(inner) > 1) || tiefe < 2,
            Value::NoneValue => tiefe < 2,
        };
        let listenVergleich = match &v {
            Value::OrderedDictLike(inner) =>
                (inner.iter().any(|(_, vValue)| !matches!(vValue, Value::NoneValue)) && len_items(inner) > 1) || tiefe < 2,
            Value::NoneValue => tiefe < 2,
        };
        if bereichLen {
            out.push_str("".join());
            out.push_str(
                "".to_string()
                .add("<div style="")
                .add("white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);" ")
                .add(">")
                .as_str()
            );
        }
        if matches!(v, Value::NoneValue) {
            out.push_str(
                "".to_string()
                .add("<input type="checkbox"")
                .add(
                    if blank {
                        "".to_string()
                        .add(" class="ordGru" onchange="toggleP2(this,-10,")
                        .add("'")
                        .add("✗")
                        .add(grundstrukturen_name0)
                        .add(",")
                        .add(&k)
                        .add("');\"")
                        .add(" id="ordGru")
                        .add(&k)
                        .add("" value="")
                        .add(&k)
                        .add(""")
                    } else {
                        "".to_string()
                    }.as_str()
                )
                .add(">")
                .as_str()
            );
        }

        if matches!(v, Value::NoneValue) || listenVergleich {
            let kkk =
                if matches!(v, Value::NoneValue) {
                    "".to_string()
                    .add("<label id="ordGruB")
                    .add(&k)
                    .add("">")
                    .add(&k.replace("_", " "))
                    .add("</label>")
                } else {
                    k.clone()
                };
            out.push_str(format!("{} ", kkk).as_str());
        }
        if matches!(v, Value::NoneValue) {
            out.push_str("</input>");
        }
        if let Value::OrderedDictLike(inner) = v {
            myprint(inner, tiefe + 1, blank, grundstrukturen_name0, out);
        }
        if bereichLen {
            out.push_str("</div>");
        }
    }
}

pub fn len_items(inner: &Vec<(String, Value)>) -> usize {
    inner.len()
}

trait FakeJoin {
    fn join(self) -> String;
}
impl FakeJoin for &str {
    fn join(self) -> String {
        String::new()
    }
}

trait StringAdd {
    fn add(self, s: &str) -> String;
}
impl StringAdd for String {
    fn add(self, s: &str) -> String {
        self + s
    }
}

pub fn elif_fake(_x: bool) -> bool { false }

pub fn grundstruk_html_from_i18n(i18n: &I18nExact, blank: bool) -> String {
    let mut wahlNeu: Vec<(String, Value)> = sorted(vec![]);

    let mut liste0: Vec<String>;
    for (key, value) in i18n.wahl15.iter() {
        let key = "_".to_string() + key;
        liste0 = key.split("_").filter(|x| !x.is_empty()).map(|x| x.to_string()).collect();
        let mut thing: Vec<(String, Value)> = sorted(vec![]);
        if len0(&liste0) > 0 {
            let reversed_liste: Vec<String> = liste0.into_iter().rev().collect();
            thing = traverseHierarchy(reversed_liste, thing, 0, value);
            wahlNeu = merge_dicts(thing, wahlNeu);
        }
    }

    let mut wahlNeu2: Vec<(String, Value)> = vec![];
    wahlNeu2.push(("15".to_string(), Value::OrderedDictLike(sorted(wahlNeu.clone()))));
    let inner15 = match ordered_get(&wahlNeu, "15").cloned() {
        Some(Value::OrderedDictLike(x)) => x,
        _ => vec![],
    };
    wahlNeu2 = merge_dicts(wahlNeu2, sorted(inner15));

    let mut out = String::new();
    out.push_str(
        "".to_string()
        .add("<div style="")
        .add(if blank && false { "display:none;" } else { "" })
        .add("white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);" ")
        .add(if blank { "id='grundstrukturenDiv'" } else { "" })
        .add(">")
        .as_str()
    );
    myprint(wahlNeu2, 0, blank, &i18n.grundstrukturen_name0, &mut out);
    out.push_str("</div>
");
    out
}

pub fn len0<T>(x: &Vec<T>) -> usize { x.len() }

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
