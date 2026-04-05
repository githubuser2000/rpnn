use indexmap::{IndexMap, IndexSet};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PyAtom {
    Int(i64),
    Str(String),
    Bool(bool),
    Tuple(Vec<PyAtom>),
    NoneValue,
}

#[derive(Clone, Debug)]
pub enum PyCollection {
    Tuple(Vec<PyCollection>),
    List(Vec<PyCollection>),
    Set(BTreeSet<PyAtom>),
    Dict(BTreeMap<String, PyCollection>),
    Atom(PyAtom),
    Unknown,
}

impl PyCollection {
    pub fn len(&self) -> usize {
        match self {
            PyCollection::Tuple(v) | PyCollection::List(v) => v.len(),
            PyCollection::Set(v) => v.len(),
            PyCollection::Dict(v) => v.len(),
            _ => 0,
        }
    }

    pub fn is_list_or_tuple(&self) -> bool {
        matches!(self, PyCollection::Tuple(_) | PyCollection::List(_))
    }

    pub fn to_set_if_first_bool_or_nested(&self) -> Option<BTreeSet<PyAtom>> {
        match self {
            PyCollection::Tuple(v) | PyCollection::List(v) => {
                if v.is_empty() {
                    return None;
                }
                match &v[0] {
                    PyCollection::Atom(PyAtom::Bool(_)) => {
                        let mut s = BTreeSet::new();
                        for x in v {
                            if let PyCollection::Atom(a) = x {
                                s.insert(a.clone());
                            }
                        }
                        Some(s)
                    }
                    PyCollection::Tuple(inner) | PyCollection::List(inner) => {
                        let mut s = BTreeSet::new();
                        for x in inner {
                            if let PyCollection::Atom(a) = x {
                                s.insert(a.clone());
                            }
                        }
                        Some(s)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn as_set(&self) -> Option<BTreeSet<PyAtom>> {
        match self {
            PyCollection::Set(s) => Some(s.clone()),
            PyCollection::Atom(a) => {
                let mut s = BTreeSet::new();
                s.insert(a.clone());
                Some(s)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TablesPlaceholder {
    pub nummeriere: bool,
    pub textWidth: i64,
    pub breitenn: Vec<i64>,
    pub spalten_parameter_name: String,
    pub kombi_parameter_name: String,
    pub ausgabe_breite: String,
}

impl TablesPlaceholder {
    pub fn new() -> Self {
        Self {
            nummeriere: true,
            textWidth: 0,
            breitenn: vec![],
            spalten_parameter_name: "spalten".to_string(),
            kombi_parameter_name: "kombi".to_string(),
            ausgabe_breite: "breite".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct I18nPlaceholder {
    pub main_para_cmds: IndexMap<String, Option<i64>>,
    pub sprachen_parameter_wort: String,
    pub wrong_lang_sentence: String,
    pub sprachen: IndexSet<String>,
    pub nichts_wort: String,
    pub keine_num_wort: String,
    pub beschrieben_wort: String,
    pub cliout2Saetze: Vec<String>,
    pub cliout3Saetze: Vec<String>,
    pub cliout4Saetze: Vec<String>,
    pub cliout5Saetze: Vec<String>,
    pub cliout6Satz: String,
    pub cliout7Saetze: Vec<String>,
    pub kombi_main_paras: IndexMap<String, String>,
}

impl I18nPlaceholder {
    pub fn demo() -> Self {
        let mut main_para_cmds = IndexMap::new();
        main_para_cmds.insert("zeilen".to_string(), Some(0));
        main_para_cmds.insert("spalten".to_string(), Some(1));
        main_para_cmds.insert("kombi".to_string(), Some(2));
        main_para_cmds.insert("ausgabe".to_string(), Some(3));
        main_para_cmds.insert("debug".to_string(), None);
        main_para_cmds.insert("h".to_string(), None);
        main_para_cmds.insert("help".to_string(), None);

        let mut sprachen = IndexSet::new();
        sprachen.insert("de".to_string());
        sprachen.insert("en".to_string());

        let mut kombi_main_paras = IndexMap::new();
        kombi_main_paras.insert("galaxie".to_string(), "galaxie".to_string());
        kombi_main_paras.insert("universum".to_string(), "universum".to_string());

        Self {
            main_para_cmds,
            sprachen_parameter_wort: "sprache=".to_string(),
            wrong_lang_sentence: "wrong language".to_string(),
            sprachen,
            nichts_wort: "nichts".to_string(),
            keine_num_wort: "keinenummerierung".to_string(),
            beschrieben_wort: "beschrieben".to_string(),
            cliout2Saetze: vec![
                "Nebenparameter '".to_string(),
                "' kennt Wert '".to_string(),
                "'; mögliche Werte: \"".to_string(),
                "'; keine nichtleeren Werte vorhanden".to_string(),
            ],
            cliout3Saetze: vec![
                "Nebenparameter '".to_string(),
                "' und Wert '".to_string(),
                "' unbekannt. Bekannte Nebenparameter: ".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                " | bekannte Werte: ".to_string(),
                "".to_string(),
            ],
            cliout4Saetze: vec![
                "Parameter '".to_string(),
                "' unbekannt. Erlaubt sind: ".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            cliout5Saetze: vec![
                "Kombi-Spalte '".to_string(),
                "' unbekannt für ".to_string(),
            ],
            cliout6Satz: "Kombi-Kommando unbekannt: ".to_string(),
            cliout7Saetze: vec![
                "Hauptparameter fehlt. ".to_string(),
                "Erhalten: ".to_string(),
                " | Erlaubt: -".to_string(),
            ],
            kombi_main_paras,
        }
    }
}

pub fn cliout(text: String) {
    eprintln!("{}", text);
}
