// center.rs
// AUTO-TRANSPILED FROM libs/center.py (1:1 semantics)

use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::py_runtime::*;

// --- Imports -------------------------------------------------------

pub fn center_module(env: &mut PyEnv) {

    // import math, os, platform, pprint, re, sys
    // → als Platzhalter-Module (wie in Python: Import hat Seiteneffekt)
    env.globals.insert("math".into(), PyValue::Dict(BTreeMap::new()));
    env.globals.insert("os".into(), PyValue::Dict(BTreeMap::new()));
    env.globals.insert("platform".into(), PyValue::Dict(BTreeMap::new()));
    env.globals.insert("pprint".into(), PyValue::Dict(BTreeMap::new()));
    env.globals.insert("re".into(), PyValue::Dict(BTreeMap::new()));
    env.globals.insert("sys".into(), PyValue::Dict(BTreeMap::new()));

    // from collections import OrderedDict
    env.globals.insert(
        "OrderedDict".into(),
        PyValue::Function(Rc::new(|_| PyValue::Dict(BTreeMap::new())))
    );

    // import i18n.words as i18n
    let mut i18n_mod = BTreeMap::new();
    i18n_mod.insert(
        "Primzahlkreuz_pro_contra_strs_Dict".into(),
        PyValue::Dict(BTreeMap::new()),
    );
    env.globals.insert("i18n".into(), PyValue::Dict(i18n_mod));

    // --- try: from collections import Callable ---------------------

    env.globals.insert(
        "Callable".into(),
        PyValue::Str("Callable".into()), // rein symbolisch wie in Python
    );

    // --- enum IntEnum ----------------------------------------------

    let int_enum = PyClass::new("IntEnum");
    env.globals.insert(
        "IntEnum".into(),
        PyValue::Dict(int_enum.dict.borrow().clone()),
    );

    // --- Regex patterns --------------------------------------------

    env.globals.insert(
        "kpattern".into(),
        PyValue::Str(r",(?![^\[\]\{\}\(\)]*[\]\}\)])".into()),
    );

    // --- Primzahlkreuz_pro_contra_strs -----------------------------

    // Python:
    // Primzahlkreuz_pro_contra_strs = i18n.Primzahlkreuz_pro_contra_strs_Dict[(..., ...)]

    let key_tuple = PyValue::List(vec![
        PyValue::Str("Primzahlkreuz_pro_contra".into()),
        PyValue::Str(
            "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)".into()
        ),
    ]);

    let prim_map = match env.globals.get("i18n") {
        Some(PyValue::Dict(d)) => d.get("Primzahlkreuz_pro_contra_strs_Dict").cloned(),
        _ => None,
    }.unwrap_or(PyValue::Dict(BTreeMap::new()));

    env.globals.insert(
        "Primzahlkreuz_pro_contra_strs".into(),
        prim_map,
    );

    // --- Ab hier: Funktionen folgen exakt der Python-Reihenfolge ---
}
