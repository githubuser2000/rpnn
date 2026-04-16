// center.rs
// AUTO-TRANSPILED FROM libs/center.py
// DO NOT EDIT MANUALLY

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use std::cell::RefCell;

use crate::py_runtime::*;

// ------------------------------------------------------------
// Module init (entspricht Top-Level-Ausführung in Python)
// ------------------------------------------------------------

pub fn center_module(env: &mut PyEnv) {

    // #!/usr/bin/env python3
    // # -*- coding: utf-8 -*-

    // import math
    env.globals.insert("math".into(), PyValue::Dict(BTreeMap::new()));

    // import os
    env.globals.insert("os".into(), PyValue::Dict(BTreeMap::new()));

    // import platform
    env.globals.insert("platform".into(), PyValue::Dict(BTreeMap::new()));

    // import pprint
    env.globals.insert("pprint".into(), PyValue::Dict(BTreeMap::new()));

    // import re
    env.globals.insert("re".into(), PyValue::Dict(BTreeMap::new()));

    // import sys
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
        PyValue::Dict(BTreeMap::new())
    );
    env.globals.insert("i18n".into(), PyValue::Dict(i18n_mod));

    // try:
    //     from collections import Callable
    // except ImportError:
    //     pass
    env.globals.insert(
        "Callable".into(),
        PyValue::Str("Callable".into())
    );

    // ------------------------------------------------------------
    // Regex pattern
    // kpattern = r",(?![^\[\]\{\}\(\)]*[\]\}\)])"
    // ------------------------------------------------------------

    env.globals.insert(
        "kpattern".into(),
        PyValue::Str(",(?![^\\[\\]\\{\\}\\(\\)]*[\\]\\}\\)])".into())
    );

    // ------------------------------------------------------------
    // Primzahlkreuz_pro_contra_strs
    // ------------------------------------------------------------

    // Python:
    // Primzahlkreuz_pro_contra_strs =
    //   i18n.Primzahlkreuz_pro_contra_strs_Dict[
    //      ("Primzahlkreuz_pro_contra",
    //       "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)")
    //   ]

    let prim_key = PyValue::List(vec![
        PyValue::Str("Primzahlkreuz_pro_contra".into()),
        PyValue::Str(
            "nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)".into()
        ),
    ]);

    let prim_map = match env.globals.get("i18n") {
        Some(PyValue::Dict(d)) => {
            d.get("Primzahlkreuz_pro_contra_strs_Dict")
                .cloned()
                .unwrap_or(PyValue::Dict(BTreeMap::new()))
        }
        _ => PyValue::Dict(BTreeMap::new()),
    };

    env.globals.insert(
        "Primzahlkreuz_pro_contra_strs".into(),
        prim_map
    );

    // ------------------------------------------------------------
    // Aliases
    // ------------------------------------------------------------

    // s = str
    env.globals.insert("s".into(), PyValue::Str("str".into()));

    // t = tuple
    env.globals.insert("t".into(), PyValue::Str("tuple".into()));

    // l = list
    env.globals.insert("l".into(), PyValue::Str("list".into()));

    // d = dict
    env.globals.insert("d".into(), PyValue::Str("dict".into()));

    // s2 = set
    env.globals.insert("s2".into(), PyValue::Str("set".into()));

    // ------------------------------------------------------------
    // from rich.console import Console
    // from rich.markdown import Markdown
    // ------------------------------------------------------------

    let mut rich_console = BTreeMap::new();
    rich_console.insert(
        "Console".into(),
        PyValue::Function(Rc::new(|_| PyValue::Dict(BTreeMap::new())))
    );
    env.globals.insert("Console".into(), PyValue::Dict(rich_console));

    let mut rich_md = BTreeMap::new();
    rich_md.insert(
        "Markdown".into(),
        PyValue::Function(Rc::new(|_| PyValue::Dict(BTreeMap::new())))
    );
    env.globals.insert("Markdown".into(), PyValue::Dict(rich_md));

    // ------------------------------------------------------------
    // >>> Ab hier beginnen Funktionsdefinitionen (primCreativity, …)
    // ------------------------------------------------------------
}
