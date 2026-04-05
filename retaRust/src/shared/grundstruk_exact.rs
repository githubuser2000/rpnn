
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PyVal {
    NoneValue,
    OrderedDictLike(OrderedDictLike),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderedDictLike {
    pub items: Vec<(String, PyVal)>,
}

impl OrderedDictLike {
    pub fn new() -> Self { Self { items: vec![] } }
    pub fn items(&self) -> Vec<(String, PyVal)> { self.items.clone() }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn contains_key(&self, key: &str) -> bool { self.items.iter().any(|(k, _)| k == key) }
    pub fn get(&self, key: &str) -> Option<PyVal> {
        for (k, v) in &self.items {
            if k == key { return Some(v.clone()); }
        }
        None
    }
    pub fn set(&mut self, key: String, value: PyVal) {
        for (k, v) in self.items.iter_mut() {
            if *k == key {
                *v = value;
                return;
            }
        }
        self.items.push((key, value));
    }
    pub fn update_like_python(&mut self, other: OrderedDictLike) {
        for (k, v) in other.items {
            self.set(k, v);
        }
    }
}

#[derive(Clone, Debug)]
pub struct I18nLike {
    pub wahl15: Vec<(String, String)>,
    pub grundstrukturen0: String,
}

impl I18nLike {
    pub fn new() -> Self {
        let mut wahl15: Vec<(String, String)> = vec![];
wahl15.push(("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)".to_string(), "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity),,nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)".to_string()));
        wahl15.push(("Konkreta_und_Focus_(2)".to_string(), "Konkreta_und_Focus_(2)".to_string()));
        wahl15.push(("Impulse_(5)".to_string(), "Impulse_(5)".to_string()));
        wahl15.push(("Gefühle_(7)".to_string(), "Gefühle_(7)".to_string()));
        wahl15.push(("Modus_und_Sein_(8)".to_string(), "Modus_und_Sein_(8)".to_string()));
        wahl15.push(("Wirklichkeiten_Wahrheit_Wahrnehmung_(10)".to_string(), "Wirklichkeiten_Wahrheit_Wahrnehmung_(10)".to_string()));
        wahl15.push(("Meta-Systeme_(12),Ordnung_und_Filterung_12_und_1pro12".to_string(), "Meta-Systeme_(12),Ordnung_und_Filterung_12_und_1pro12".to_string()));
        wahl15.push(("Paradigmen_sind_Absichten_(13)".to_string(), "Paradigmen_sind_Absichten_(13)".to_string()));
        wahl15.push(("Gedanken_sind_Positionen_(17)".to_string(), "Gedanken_sind_Positionen_(17)".to_string()));
        wahl15.push(("Verbundenheiten_(18)".to_string(), "Verbundenheiten_(18)".to_string()));
        wahl15.push(("Triebe_und_Bedürfnisse_(6)".to_string(), "Triebe_und_Bedürfnisse_(6)".to_string()));
        wahl15.push(("Lust_(9)".to_string(), "Lust_(9)".to_string()));
        wahl15.push(("Reflexe_(3),Existenzialien_(3)".to_string(), "Reflexe_(3),Existenzialien_(3)".to_string()));
        wahl15.push(("Absicht_6_ist_Vorteilsmaximierung".to_string(), "Absicht_6_ist_Vorteilsmaximierung".to_string()));
        wahl15.push(("Absicht_7_ist_Selbstlosigkeit".to_string(), "Absicht_7_ist_Selbstlosigkeit".to_string()));
        wahl15.push(("Absicht_10_ist_Wirklichkeit_erkennen".to_string(), "Absicht_10_ist_Wirklichkeit_erkennen".to_string()));
        wahl15.push(("Absicht_17_ist_zu_meinen".to_string(), "Absicht_17_ist_zu_meinen".to_string()));
        wahl15.push(("Zeit_(4)_als_Wirklichkeit".to_string(), "Zeit_(4)_als_Wirklichkeit".to_string()));
        wahl15.push(("Funktionen_Vorstellungen_(16)".to_string(), "Funktionen_Vorstellungen_(16)".to_string()));
        wahl15.push(("Achtung_(4)".to_string(), "Achtung_(4)".to_string()));
        wahl15.push(("Absicht_1/8".to_string(), "Absicht_1/8".to_string()));
        wahl15.push(("Absicht_1/6_ist_Reinigung_und_Klarheit".to_string(), "Absicht_1/6_ist_Reinigung_und_Klarheit".to_string()));
        wahl15.push(("Reflektion_und_Kategorien_(1/15)".to_string(), "Reflektion_und_Kategorien_(1/15)".to_string()));
        wahl15.push(("Bewusstheit_statt_Bewusstsein_(1)".to_string(), "Bewusstheit_statt_Bewusstsein_(1)".to_string()));
        wahl15.push(("Energie_und_universelle_Eigenschaften_(30)".to_string(), "Energie_und_universelle_Eigenschaften_(30)".to_string()));
        wahl15.push(("Stimmungen_Kombinationen_(14)".to_string(), "Stimmungen_Kombinationen_(14)".to_string()));
        wahl15.push(("Klassen_(20)".to_string(), "Klassen_(20)".to_string()));
        wahl15.push(("Empathie_(37)".to_string(), "Empathie_(37)".to_string()));
        wahl15.push(("Garben_und_Verhalten_nachfühlen(31)".to_string(), "Garben_und_Verhalten_nachfühlen(31)".to_string()));
        wahl15.push(("Verhalten_(11)".to_string(), "Verhalten_(11)".to_string()));
        wahl15.push(("Bedeutung_(10)".to_string(), "Bedeutung_(10)".to_string()));
        wahl15.push(("Themen_(6)".to_string(), "Themen_(6)".to_string()));
        wahl15.push(("Optimierung_(10)".to_string(), "Optimierung_(10)".to_string()));
        wahl15.push(("Attraktionen_(36)".to_string(), "Attraktionen_(36)".to_string()));
        wahl15.push(("Absicht_16_ist_zu_genügen".to_string(), "Absicht_16_ist_zu_genügen".to_string()));
        wahl15.push(("Liebe_(7)".to_string(), "Liebe_(7)".to_string()));
        wahl15.push(("Koalitionen_(10)".to_string(), "Koalitionen_(10)".to_string()));
        wahl15.push(("Ansichten_Standpunkte_(18_17)".to_string(), "Ansichten_Standpunkte_(18_17)".to_string()));
        wahl15.push(("Prinzipien(1/8)".to_string(), "Prinzipien(1/8)".to_string()));
        wahl15.push(("Bestrebungen(1/5)".to_string(), "Bestrebungen(1/5)".to_string()));
        wahl15.push(("Bedingung_und_Auslöser_(1/3)".to_string(), "Bedingung_und_Auslöser_(1/3)".to_string()));
        wahl15.push(("relativer_Zeit-Betrag_(15_10_4_18_6)".to_string(), "relativer_Zeit-Betrag_(15_10_4_18_6)".to_string()));
        wahl15.push(("Zahlenvergleich_(15_18_6)".to_string(), "Zahlenvergleich_(15_18_6)".to_string()));
        wahl15.push(("Leidenschaften_(21)".to_string(), "Leidenschaften_(21)".to_string()));
        wahl15.push(("Erwartungshaltungen_(26)".to_string(), "Erwartungshaltungen_(26)".to_string()));
        wahl15.push(("Extremalien_(19),Ziele_(19)".to_string(), "Extremalien_(19),Ziele_(19)".to_string()));
        wahl15.push(("universeller_Komperativ_(18→15)".to_string(), "universeller_Komperativ_(18→15)".to_string()));
        wahl15.push(("Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)".to_string(), "Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)".to_string()));
        wahl15.push(("Sollen_Frage_Vorgehensweise_(1/13)".to_string(), "Sollen_Frage_Vorgehensweise_(1/13)".to_string()));
        wahl15.push(("Fundament_(1/19)".to_string(), "Fundament_(1/19)".to_string()));
        wahl15.push(("abhängige_Verbundenheit_(90)".to_string(), "abhängige_Verbundenheit_(90)".to_string()));
        wahl15.push(("Absicht_13_ist_Helfen".to_string(), "Absicht_13_ist_Helfen".to_string()));
        wahl15.push(("Karte_Filter_und_Unterscheidung_(1/12)".to_string(), "Karte_Filter_und_Unterscheidung_(1/12)".to_string()));
        wahl15.push(("Maßnahmen_39".to_string(), "Maßnahmen_(39)".to_string()));
        Self {
            wahl15,
            grundstrukturen0: "grundstrukturen".to_string(),
        }
    }
}

pub fn cmp_before(value: &(String, PyVal)) -> (bool, String) {
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
        let toSort = value;
        return (isNumber, toSort);
    }
    (isNumber, toSort)
}

pub fn cmpx(erster: &(String, PyVal), zweiter: &(String, PyVal)) -> i64 {
    let (isNumber1, value1) = cmp_before(erster);
    let (isNumber2, value2) = cmp_before(zweiter);
    if isNumber1 && isNumber2 {
        let value1 = value1.parse::<i64>().unwrap_or(0);
        let value2 = value2.parse::<i64>().unwrap_or(0);
        if value1 == value2 {
            if erster.0.contains("/") {
                return 1;
            } else if zweiter.0.contains("/") {
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
    } else if value1 < value2 {
        return 1;
    } else {
        return 0;
    }
}

pub fn sorted(items: Vec<(String, PyVal)>) -> Vec<(String, PyVal)> {
    let mut indexed: Vec<(usize, (String, PyVal))> = items.into_iter().enumerate().collect();
    indexed.sort_by(|a, b| {
        let c = cmpx(&a.1, &b.1);
        if c < 0 {
            std::cmp::Ordering::Less
        } else if c > 0 {
            std::cmp::Ordering::Greater
        } else {
            a.0.cmp(&b.0)
        }
    });
    indexed.into_iter().map(|(_, item)| item).collect()
}

pub fn od_from_items(items: Vec<(String, PyVal)>) -> OrderedDictLike {
    OrderedDictLike { items }
}

pub fn merge_dicts(dict1: OrderedDictLike, dict2: OrderedDictLike) -> OrderedDictLike {
    let mut dict1 = dict1;
    for (key, dict2_value) in dict2.items() {
        if dict1.contains_key(&key)
            && matches!(dict1.get(&key), Some(PyVal::OrderedDictLike(_)))
            && matches!(dict2_value, PyVal::OrderedDictLike(_))
        {
            if let Some(PyVal::OrderedDictLike(left)) = dict1.get(&key) {
                if let PyVal::OrderedDictLike(right) = dict2_value.clone() {
                    let merged = merge_dicts(left, right);
                    dict1.set(key, PyVal::OrderedDictLike(merged));
                }
            }
        } else if dict1.contains_key(&key)
            && matches!(dict2_value, PyVal::OrderedDictLike(_))
            && !matches!(dict1.get(&key), Some(PyVal::OrderedDictLike(_)))
        {
            if let Some(existing) = dict1.get(&key) {
                if let PyVal::OrderedDictLike(right) = dict2_value {
                    let mut temp = OrderedDictLike::new();
                    temp.set(key.clone(), existing);
                    temp.update_like_python(right);
                    dict1.set(key, PyVal::OrderedDictLike(od_from_items(sorted(temp.items()))));
                }
            }
        } else if !dict1.contains_key(&key) {
            dict1.set(key, dict2_value);
        }
    }
    od_from_items(sorted(dict1.items()))
}

pub fn traverseHierarchy(liste: Vec<String>, thing: OrderedDictLike, listenIndex: usize, value: &str) -> OrderedDictLike {
    let mut thing = thing;
    let mut knoten = liste[listenIndex].clone();
    knoten = knoten.replace("pro", "/");
    if listenIndex == 0 {
        let newKeys = value.split(",").map(|x| x.to_string()).collect::<Vec<_>>();
        let newValues = vec![PyVal::NoneValue; newKeys.len()];
        let zipped = newKeys.into_iter().zip(newValues.into_iter()).collect::<Vec<_>>();
        let mut items = thing.items();
        items.extend(sorted(zipped));
        thing = od_from_items(items);
    }
    let thing = od_from_items(sorted(vec![(knoten.clone(), PyVal::OrderedDictLike(thing))]));
    if liste.len() > listenIndex + 1 {
        return traverseHierarchy(liste, thing, listenIndex + 1, value);
    }
    thing
}

pub fn myprint(d: OrderedDictLike, tiefe: usize, blank: bool, i18n: &I18nLike, out: &mut String) {
    let bereich = d.items();
    let iter: Vec<(String, PyVal)> = if tiefe < 2 { bereich } else { let mut x = d.items(); x.reverse(); x };
    for (k, v) in iter {
        let bereichLen = match &v {
            PyVal::OrderedDictLike(inner) => (inner.items().len() > 1) || tiefe < 2,
            PyVal::NoneValue => tiefe < 2,
        };
        let listenVergleich = match &v {
            PyVal::OrderedDictLike(inner) =>
                (
                    inner.items().iter().any(|(_, vValue)| !matches!(vValue, PyVal::NoneValue))
                    && inner.items().len() > 1
                ) || tiefe < 2,
            PyVal::NoneValue => tiefe < 2,
        };
        if bereichLen {
            out.push_str("<div style=\"white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);\" >");
        }
        if matches!(v, PyVal::NoneValue) {
            out.push_str("<input type=\"checkbox\"");
            if blank {
                out.push_str(" class=\"ordGru\" onchange=\"toggleP2(this,-10,'✗");
                out.push_str(&i18n.grundstrukturen0);
                out.push_str(",");
                out.push_str(&k);
                out.push_str("');\" id=\"ordGru");
                out.push_str(&k);
                out.push_str("\" value=\"");
                out.push_str(&k);
                out.push_str("\"");
            }
            out.push_str(">");
        }
        if matches!(v, PyVal::NoneValue) || listenVergleich {
            if matches!(v, PyVal::NoneValue) {
                out.push_str("<label id=\"ordGruB");
                out.push_str(&k);
                out.push_str("\">");
                out.push_str(&k.replace("_", " "));
                out.push_str("</label> ");
            } else {
                out.push_str(&k);
                out.push_str(" ");
            }
        }
        if matches!(v, PyVal::NoneValue) {
            out.push_str("</input>");
        }
        if let PyVal::OrderedDictLike(inner) = v {
            myprint(inner, tiefe + 1, blank, i18n, out);
        }
        if bereichLen {
            out.push_str("</div>");
        }
    }
}

pub fn grundstruk_html_from_i18n(i18n: &I18nLike, blank: bool) -> String {
    let mut wahlNeu = od_from_items(sorted(vec![]));
    let mut liste: Vec<String>;
    for (key, value) in &i18n.wahl15 {
        let key = "_".to_string() + key;
        liste = key.split("_").filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<Vec<_>>();
        let mut thing = od_from_items(sorted(vec![]));
        if !liste.is_empty() {
            thing = traverseHierarchy(liste.into_iter().rev().collect::<Vec<_>>(), thing, 0, value);
            wahlNeu = merge_dicts(thing, wahlNeu);
        }
    }

    let mut wahlNeu2 = od_from_items(sorted(vec![]));
    wahlNeu2.set("15".to_string(), PyVal::OrderedDictLike(od_from_items(sorted(wahlNeu.items()))));
    let inner15 = match wahlNeu.get("15") {
        Some(PyVal::OrderedDictLike(x)) => x,
        _ => od_from_items(vec![]),
    };
    wahlNeu2 = merge_dicts(wahlNeu2, od_from_items(sorted(inner15.items())));

    let mut out = String::new();
    out.push_str("<div style=\"");
    if blank && false {
        out.push_str("display:none;");
    }
    out.push_str("white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);\" ");
    if blank {
        out.push_str("id='grundstrukturenDiv'");
    }
    out.push_str(">");
    myprint(wahlNeu2, 0, blank, i18n, &mut out);
    out.push_str("</div>\n");
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
pub const PYTHON_SOURCE__WAHL15WORDS: &str = r#"wahl15Words: dict = {
    "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15),Geist_(15),Model_of_Hierarchical_Complexity,"
    + Primzahlkreuz_pro_contra_strs[1]: ",".join(
        (
            _("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
            _("Geist_(15)"),
            _("Model_of_Hierarchical_Complexity),"),
            Primzahlkreuz_pro_contra_strs_Fkt[1],
        ),
    ),
    "Konkreta_und_Focus_(2)": _("Konkreta_und_Focus_(2)"),
    "Impulse_(5)": _("Impulse_(5)"),
    "Gefühle_(7)": _("Gefühle_(7)"),
    "Modus_und_Sein_(8)": _("Modus_und_Sein_(8)"),
    "Wirklichkeiten_Wahrheit_Wahrnehmung_(10)": _(
        "Wirklichkeiten_Wahrheit_Wahrnehmung_(10)"
    ),
    "Meta-Systeme_(12),Ordnung_und_Filterung_12_und_1pro12": ",".join(
        (("Meta-Systeme_(12)"), _("Ordnung_und_Filterung_12_und_1pro12"))
    ),
    "Paradigmen_sind_Absichten_(13)": _("Paradigmen_sind_Absichten_(13)"),
    "Gedanken_sind_Positionen_(17)": _("Gedanken_sind_Positionen_(17)"),
    "Verbundenheiten_(18)": _("Verbundenheiten_(18)"),
    "Triebe_und_Bedürfnisse_(6)": _("Triebe_und_Bedürfnisse_(6)"),
    "Lust_(9)": _("Lust_(9)"),
    "Reflexe_(3),Existenzialien_(3)": ",".join(
        (_("Reflexe_(3)"), _("Existenzialien_(3)"))
    ),
    "Absicht_6_ist_Vorteilsmaximierung": _("Absicht_6_ist_Vorteilsmaximierung"),
    "Absicht_7_ist_Selbstlosigkeit": _("Absicht_7_ist_Selbstlosigkeit"),
    "Absicht_10_ist_Wirklichkeit_erkennen": _("Absicht_10_ist_Wirklichkeit_erkennen"),
    "Absicht_17_ist_zu_meinen": _("Absicht_17_ist_zu_meinen"),
    "Zeit_(4)_als_Wirklichkeit": _("Zeit_(4)_als_Wirklichkeit"),
    "Funktionen_Vorstellungen_(16)": _("Funktionen_Vorstellungen_(16)"),
    "Achtung_(4)": _("Achtung_(4)"),
    "Absicht_1/8": _("Absicht_1/8"),
    "Absicht_1/6_ist_Reinigung_und_Klarheit": _(
        "Absicht_1/6_ist_Reinigung_und_Klarheit"
    ),
    "Reflektion_und_Kategorien_(1/15)": _("Reflektion_und_Kategorien_(1/15)"),
    "Bewusstheit_statt_Bewusstsein_(1)": _("Bewusstheit_statt_Bewusstsein_(1)"),
    "Energie_und_universelle_Eigenschaften_(30)": _(
        "Energie_und_universelle_Eigenschaften_(30)"
    ),
    "Stimmungen_Kombinationen_(14)": _("Stimmungen_Kombinationen_(14)"),
    "Klassen_(20)": _("Klassen_(20)"),
    "Empathie_(37)": _("Empathie_(37)"),
    "Garben_und_Verhalten_nachfühlen(31)": _("Garben_und_Verhalten_nachfühlen(31)"),
    "Verhalten_(11)": _("Verhalten_(11)"),
    "Bedeutung_(10)": _("Bedeutung_(10)"),
    "Themen_(6)": _("Themen_(6)"),
    "Optimierung_(10)": _("Optimierung_(10)"),
    "Attraktionen_(36)": _("Attraktionen_(36)"),
    "Absicht_16_ist_zu_genügen": _("Absicht_16_ist_zu_genügen"),
    "Liebe_(7)": _("Liebe_(7)"),
    "Koalitionen_(10)": _("Koalitionen_(10)"),
    "Ansichten_Standpunkte_(18_17)": _("Ansichten_Standpunkte_(18_17)"),
    "Prinzipien(1/8)": _("Prinzipien(1/8)"),
    "Bestrebungen(1/5)": _("Bestrebungen(1/5)"),
    "Bedingung_und_Auslöser_(1/3)": _("Bedingung_und_Auslöser_(1/3)"),
    "relativer_Zeit-Betrag_(15_10_4_18_6)": _("relativer_Zeit-Betrag_(15_10_4_18_6)"),
    "Zahlenvergleich_(15_18_6)": _("Zahlenvergleich_(15_18_6)"),
    "Leidenschaften_(21)": _("Leidenschaften_(21)"),
    "Erwartungshaltungen_(26)": _("Erwartungshaltungen_(26)"),
    "Extremalien_(19),Ziele_(19)": ",".join((_("Extremalien_(19)"), _("Ziele_(19)"))),
    "universeller_Komperativ_(18→15)": _("universeller_Komperativ_(18→15)"),
    "Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)": _(
        "Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)"
    ),
    "Sollen_Frage_Vorgehensweise_(1/13)": _("Sollen_Frage_Vorgehensweise_(1/13)"),
    "Fundament_(1/19)": _("Fundament_(1/19)"),
    "abhängige_Verbundenheit_(90)": _("abhängige_Verbundenheit_(90)"),
    "Absicht_13_ist_Helfen": _("Absicht_13_ist_Helfen"),
    "Karte_Filter_und_Unterscheidung_(1/12)": _(
        "Karte_Filter_und_Unterscheidung_(1/12)"
    ),
    "Maßnahmen_39": _("Maßnahmen_(39)"),
}"#;
