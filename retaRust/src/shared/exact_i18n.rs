use indexmap::IndexMap;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PyAtom {
    Int(i64),
    Str(String),
    Bool(bool),
    Tuple(Vec<PyAtom>),
    NoneValue,
}

#[derive(Clone, Debug)]
pub struct StoreParameterEntry {
    pub parameterMainNames: Vec<String>,
    pub parameterNames: Vec<String>,
    pub datas: Vec<Vec<PyAtom>>,
}

#[derive(Clone, Debug)]
pub struct I18nExact {
    pub paraNdataMatrix: Vec<StoreParameterEntry>,
    pub kombiParaNdataMatrix: IndexMap<i64, Vec<String>>,
    pub kombiParaNdataMatrix2: IndexMap<i64, Vec<String>>,
    pub grundstrukturen_name0: String,
    pub wahl15: Vec<(String, String)>,
}

impl I18nExact {
    pub fn from_python_evaluated_shapes_subset() -> Self {
        let mut paraNdataMatrix: Vec<StoreParameterEntry> = vec![];
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Wichtigste".to_string(), "wichtigste".to_string()],
    datas: vec![
        vec![PyAtom::Int(10), PyAtom::Int(4), PyAtom::Int(5), PyAtom::Int(8)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Mensch-zu-Tier".to_string(), "menschtier".to_string(), "tiermensch".to_string()],
    datas: vec![
        vec![PyAtom::Int(314)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Superkräfte".to_string(), "Superkraefte".to_string()],
    datas: vec![
        vec![PyAtom::Int(444), PyAtom::Int(494), PyAtom::Int(496), PyAtom::Int(503)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Superkräfte".to_string(), "Superkraefte".to_string()],
    datas: vec![
        vec![PyAtom::Int(444), PyAtom::Int(494), PyAtom::Int(496)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Evolution_vs_Design_intelligent".to_string()],
    datas: vec![
        vec![PyAtom::Int(519)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Evolution_vs_Design_intelligent".to_string()],
    datas: vec![
        vec![PyAtom::Int(519)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Superkräfte".to_string(), "Superkraefte".to_string()],
    datas: vec![
        vec![PyAtom::Int(444), PyAtom::Int(494), PyAtom::Int(496)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Formationen".to_string()],
    datas: vec![
        vec![PyAtom::Int(461)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Ansichten_Standpunkte_(18_17)".to_string(), "ansichten".to_string()],
    datas: vec![
        vec![PyAtom::Int(240), PyAtom::Int(346)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["(politische)_Richtungen_(7)".to_string(), "richtungen".to_string(), "politische".to_string()],
    datas: vec![
        vec![PyAtom::Int(235)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Wirklichkeiten_(10)".to_string(), "wirklichkeit".to_string(), "wirklichkeiten".to_string()],
    datas: vec![
        vec![PyAtom::Int(233), PyAtom::Int(265), PyAtom::Int(268), PyAtom::Int(322), PyAtom::Int(420)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Meta-Systeme_(12)".to_string(), "metasysteme".to_string(), "metasystem".to_string(), "meta-systeme".to_string(), "meta-system".to_string()],
    datas: vec![
        vec![PyAtom::Int(232), PyAtom::Int(288), PyAtom::Int(334), PyAtom::Int(410), PyAtom::Int(411), PyAtom::Int(483), PyAtom::Int(497), PyAtom::Int(498), PyAtom::Int(499), PyAtom::Int(79), PyAtom::Int(80)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Intelligenz".to_string(), "intelligenz".to_string()],
    datas: vec![
        vec![PyAtom::Int(214)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Gleichheit_Freiheit_Ordnung".to_string(), "gleichheit".to_string(), "freiheit".to_string(), "gleichheit".to_string()],
    datas: vec![
        vec![PyAtom::Int(132), PyAtom::Int(324), PyAtom::Int(328), PyAtom::Int(331), PyAtom::Int(335), PyAtom::Int(497), PyAtom::Int(498), PyAtom::Int(499), PyAtom::Int(79), PyAtom::Int(80)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Komplexität".to_string(), "komplexität".to_string(), "komplexitaet".to_string()],
    datas: vec![
        vec![PyAtom::Int(213)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Mechanismen".to_string(), "mechanismen".to_string(), "mechanismus".to_string()],
    datas: vec![
        vec![PyAtom::Int(107)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Zweitwichtigste".to_string(), "zweitwichtigste".to_string()],
    datas: vec![
        vec![PyAtom::Int(183), PyAtom::Int(19), PyAtom::Int(65)],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(10)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Drittwichtigste".to_string(), "drittwichtigste".to_string()],
    datas: vec![
        vec![PyAtom::Int(64)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Motive_Sternpolygone".to_string(), "viertwichtigste".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Str("primMotivStern".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste2']".to_string()],
    parameterNames: vec!["Wichtigste".to_string(), "wichtigstes".to_string()],
    datas: vec![
        vec![PyAtom::Int(0), PyAtom::Int(1), PyAtom::Int(2), PyAtom::Int(207), PyAtom::Int(36), PyAtom::Int(37)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste2']".to_string()],
    parameterNames: vec!["Zweitwichtigste".to_string(), "zweitwichtigste".to_string()],
    datas: vec![
        vec![PyAtom::Int(30)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["Halbierung".to_string(), "halbierung".to_string(), "halbierungen".to_string()],
    datas: vec![
        vec![PyAtom::Int(86)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Religions-Gründer-Typ".to_string(), "religionsgründertyp".to_string(), "prophet".to_string(), "archon".to_string(), "religionsgruendertyp".to_string()],
    datas: vec![
        vec![PyAtom::Int(503), PyAtom::Int(72)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Satan_Teufel".to_string()],
    datas: vec![
        vec![PyAtom::Int(495)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Satan_Teufel".to_string()],
    datas: vec![
        vec![PyAtom::Int(495)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Hinduismus".to_string(), "hinduismus".to_string()],
    datas: vec![
        vec![PyAtom::Int(217)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Sternpolygon".to_string(), "sternpolygon".to_string()],
    datas: vec![
        vec![PyAtom::Int(0), PyAtom::Int(36), PyAtom::Int(6)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["der_Tierkreiszeichen".to_string(), "dertierkreiszeichen".to_string(), "babylon".to_string()],
    datas: vec![
        vec![PyAtom::Int(0), PyAtom::Int(207), PyAtom::Int(36), PyAtom::Int(477), PyAtom::Int(478)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Sternpolygon_vs_gleichförmiges".to_string(), "vergleich".to_string(), "sternpolygonvsgleichfoermiges".to_string(), "vergleichnvs1divn".to_string()],
    datas: vec![
        vec![PyAtom::Int(87)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Messias".to_string(), "messias".to_string(), "heptagramm".to_string(), "hund".to_string(), "messiase".to_string(), "messiasse".to_string()],
    datas: vec![
        vec![PyAtom::Int(503), PyAtom::Int(7)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["gleichförmiges_Polygon".to_string(), "gleichförmigespolygon".to_string(), "gleichfoermigespolygon".to_string(), "nichtsternpolygon".to_string(), "polygon".to_string()],
    datas: vec![
        vec![PyAtom::Int(16), PyAtom::Int(37)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Vertreter_höherer_Konzepte".to_string(), "vertreterhoehererkonzepte".to_string(), "galaxien".to_string(), "galaxie".to_string(), "schwarzesonne".to_string(), "schwarzesonnen".to_string(), "universum".to_string(), "universen".to_string(), "kreis".to_string(), "kreise".to_string(), "kugel".to_string(), "kugeln".to_string()],
    datas: vec![
        vec![PyAtom::Int(23)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Lebewesen_Galaxie_am_Besten".to_string()],
    datas: vec![
        vec![PyAtom::Int(470), PyAtom::Int(471), PyAtom::Int(473)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Offenbarung_des_Johannes".to_string(), "offenbarung".to_string(), "offenbarungdesjohannes".to_string(), "johannes".to_string(), "bibel".to_string(), "offenbarungjohannes".to_string()],
    datas: vec![
        vec![PyAtom::Int(90)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["Teilchen-Meta-Physik".to_string(), "addition".to_string(), "identitaet".to_string(), "Identität".to_string()],
    datas: vec![
        vec![PyAtom::Int(219), PyAtom::Int(223), PyAtom::Int(307), PyAtom::Int(308), PyAtom::Int(333), PyAtom::Int(387), PyAtom::Int(388), PyAtom::Int(406)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Hochzüchten".to_string(), "hochzüchten".to_string(), "hochzuechten".to_string()],
    datas: vec![
        vec![PyAtom::Int(318), PyAtom::Int(319)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Teilchen_anderes_Universum".to_string()],
    datas: vec![
        vec![PyAtom::Int(512)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Teilchen_anderes_Universum".to_string()],
    datas: vec![
        vec![PyAtom::Int(512)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Teilchen_anderes_Universum".to_string()],
    datas: vec![
        vec![PyAtom::Int(512)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Zusammenhang_Gehirn_Kosmos_Universum".to_string()],
    datas: vec![
        vec![PyAtom::Int(489)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Zahlenarten".to_string()],
    datas: vec![
        vec![PyAtom::Int(462)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Bestrafung".to_string()],
    datas: vec![
        vec![PyAtom::Int(463)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bestrafung".to_string()],
    datas: vec![
        vec![PyAtom::Int(463)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["weniger_am_Menschen".to_string()],
    datas: vec![
        vec![PyAtom::Int(464)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Erlösung".to_string(), "Erloesung".to_string()],
    datas: vec![
        vec![PyAtom::Int(465)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Erlösung".to_string(), "Erloesung".to_string()],
    datas: vec![
        vec![PyAtom::Int(465)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gewalt".to_string()],
    datas: vec![
        vec![PyAtom::Int(466)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Gewalt".to_string()],
    datas: vec![
        vec![PyAtom::Int(466), PyAtom::Int(479)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Farben".to_string()],
    datas: vec![
        vec![PyAtom::Int(444)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["künstliches_Leben_(15)".to_string(), "künstlichesleben".to_string(), "grosseki".to_string()],
    datas: vec![
        vec![PyAtom::Int(409)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Software-Lizenzen_akademische_Grade".to_string(), "softwarelizenz".to_string(), "akademischeGrade".to_string()],
    datas: vec![
        vec![PyAtom::Int(422)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Strategie_Taktik_(15m8)".to_string(), "strategie".to_string(), "taktik".to_string()],
    datas: vec![
        vec![PyAtom::Int(385)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Universelles_Verhältnis_gleicher_Zahlen".to_string(), "verhaeltnisgleicherzahl".to_string()],
    datas: vec![
        vec![PyAtom::Int(383)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["universelles_Recht".to_string(), "recht".to_string(), "jura".to_string()],
    datas: vec![
        vec![PyAtom::Int(34), PyAtom::Int(382), PyAtom::Int(65)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["sowas_wie_Kombinieren_Verknüpfen".to_string(), "kombinierenetc".to_string()],
    datas: vec![
        vec![PyAtom::Int(320)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Hochzüchten".to_string(), "hochzüchten".to_string(), "hochzuechten".to_string()],
    datas: vec![
        vec![PyAtom::Int(318), PyAtom::Int(319)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Teilchen-Meta-Physik".to_string()],
    datas: vec![
        vec![PyAtom::Int(219), PyAtom::Int(308)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Universelle_(15)".to_string()],
    datas: vec![
        vec![PyAtom::Int(219), PyAtom::Int(308)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["Wirklichkeiten_(10)".to_string(), "wirklichkeit".to_string(), "wirklichkeiten".to_string()],
    datas: vec![
        vec![PyAtom::Int(420)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Galaktische_(14)".to_string()],
    datas: vec![
        vec![PyAtom::Int(406)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Multiverselle_(16)".to_string()],
    datas: vec![
        vec![PyAtom::Int(388), PyAtom::Int(418)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["die_Tugendsortierung_(13_mit_14)".to_string()],
    datas: vec![
        vec![PyAtom::Int(411)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["die_Galaxie_Unterbereiche_(13)".to_string()],
    datas: vec![
        vec![PyAtom::Int(223), PyAtom::Int(307), PyAtom::Int(412)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Gute_die_Richtung_(7)".to_string()],
    datas: vec![
        vec![PyAtom::Int(333)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["Raum_und_Dimensionen_(8)".to_string()],
    datas: vec![
        vec![PyAtom::Int(387)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["keine_Nur-Paradigma-Religionen".to_string(), "metaparadigmareligion".to_string()],
    datas: vec![
        vec![PyAtom::Int(190), PyAtom::Int(191), PyAtom::Int(196)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Kugeln_Kreise".to_string(), "kugelnkreise".to_string(), "kugeln".to_string(), "kreise".to_string()],
    datas: vec![
        vec![PyAtom::Int(145), PyAtom::Int(77)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Raumzeit_Anordnung_mathematisch_universell".to_string()],
    datas: vec![
        vec![PyAtom::Int(472)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Multiversalien_(16)".to_string(), "multiversalien".to_string()],
    datas: vec![
        vec![PyAtom::Int(389)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Meta-Physik-Teilchen_(1)".to_string(), "teilchen".to_string()],
    datas: vec![
        vec![PyAtom::Int(388)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Kugeln_Kreise".to_string(), "kugelnkreise".to_string(), "kugeln".to_string(), "kreise".to_string()],
    datas: vec![
        vec![PyAtom::Int(145), PyAtom::Int(77)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["chinesisches_Horoskop".to_string(), "chinesischeshoroskop".to_string(), "china".to_string()],
    datas: vec![
        vec![PyAtom::Int(91)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["babylonische_Tierkreiszeichen".to_string(), "tierkreiszeichen".to_string(), "babylon".to_string()],
    datas: vec![
        vec![PyAtom::Int(1), PyAtom::Int(2)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Thomasevangelium".to_string(), "thomasevangelium".to_string(), "thomas".to_string()],
    datas: vec![
        vec![PyAtom::Int(0), PyAtom::Int(3), PyAtom::Int(303)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Netzwerk".to_string(), "netzwerk".to_string()],
    datas: vec![
        vec![PyAtom::Int(417), PyAtom::Int(436)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Kontroverse_(51)".to_string(), "kontroverse".to_string()],
    datas: vec![
        vec![PyAtom::Int(421)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["mathematisches_Design_(32)".to_string(), "mathematischesdesign".to_string()],
    datas: vec![
        vec![PyAtom::Int(419)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()],
    datas: vec![
        vec![PyAtom::Int(84)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()],
    datas: vec![
        vec![PyAtom::Int(84)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Transzendentalien_innen_außen".to_string(), "innenaussenstrukur".to_string(), "strukturalieninnenaußen".to_string(), "strukturalieninnenaussen".to_string(), "innenaußenstrukur".to_string(), "transzendentalieninnenaußen".to_string(), "transzendentalieninnenaussen".to_string()],
    datas: vec![
        vec![PyAtom::Int(149)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Modallogik".to_string(), "modallogik".to_string()],
    datas: vec![
        vec![PyAtom::Int(148)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["5".to_string(), "fünf".to_string(), "fünfer".to_string(), "fünferstruktur".to_string(), "fuenf".to_string(), "fuenfer".to_string(), "fuenferstruktur".to_string()],
    datas: vec![
        vec![PyAtom::Int(96)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["9".to_string(), "neun".to_string(), "neuner".to_string(), "neunerstruktur".to_string()],
    datas: vec![
        vec![PyAtom::Int(94)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["3".to_string(), "drei".to_string(), "dreier".to_string(), "dreierstruktur".to_string()],
    datas: vec![
        vec![PyAtom::Int(315), PyAtom::Int(316), PyAtom::Int(92), PyAtom::Int(93)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["Licht".to_string(), "licht".to_string()],
    datas: vec![
        vec![PyAtom::Int(20), PyAtom::Int(27), PyAtom::Int(313)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Strukturgrösse".to_string(), "strukturgroesse".to_string(), "größe".to_string(), "groesse".to_string(), "gross".to_string(), "strukturgroesse".to_string(), "strukturgroeße".to_string(), "strukturgrösse".to_string(), "strukturgröße".to_string()],
    datas: vec![
        vec![PyAtom::Int(197), PyAtom::Int(21), PyAtom::Int(4), PyAtom::Int(425), PyAtom::Int(54)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["Strukturgrösse".to_string(), "strukturgroesse".to_string(), "größe".to_string(), "groesse".to_string(), "gross".to_string(), "strukturgroesse".to_string(), "strukturgroeße".to_string(), "strukturgrösse".to_string(), "strukturgröße".to_string()],
    datas: vec![
        vec![PyAtom::Int(197), PyAtom::Int(21), PyAtom::Int(4), PyAtom::Int(425), PyAtom::Int(54)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["Organisationen".to_string(), "organisationen".to_string(), "organisation".to_string()],
    datas: vec![
        vec![PyAtom::Int(30), PyAtom::Int(425), PyAtom::Int(82)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["politische_Systeme".to_string(), "politischesysteme".to_string(), "politik".to_string()],
    datas: vec![
        vec![PyAtom::Int(83)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["meta".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(2), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["konkret".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(2), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Theorie".to_string(), "theorie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(3), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Praxis".to_string(), "praxis".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(3), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Management".to_string(), "management".to_string(), "stau".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(4), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["verändernd".to_string(), "veraendernd".to_string(), "fluss".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(4), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["ganzheitlich".to_string(), "mathematisch_diskret".to_string(), "diskret".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(5), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["darüber_hinausgehend".to_string(), "hinausgehend".to_string(), "kontinuierlich".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(5), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Universum_Strukturalien_Transzendentalien".to_string(), "universum".to_string(), "strukturalie".to_string(), "strukturalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(5)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Richtung_als_Richtung".to_string(), "richtungrichtung".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::NoneValue])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Galaxieabsicht".to_string(), "absichtgalaxie".to_string(), "absicht".to_string(), "motive".to_string(), "motiv".to_string(), "absichten".to_string(), "galaxie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(10)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Absicht_Reziproke_Galaxie".to_string(), "absichtgalaxiereziproke".to_string(), "absichtreziproke".to_string(), "motivereziproke".to_string(), "motivreziproke".to_string(), "absichtenreziproke".to_string(), "galaxiereziproke".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(42)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Universum_Reziproke".to_string(), "universumreziproke".to_string(), "strukturaliereziproke".to_string(), "strukturalienreziproke".to_string(), "transzendentalienreziproke".to_string(), "transzendentaliereziproke".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(131)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Dagegen-Gegentranszendentalie".to_string(), "dagegengegentranszendentalie".to_string(), "dagegengegentranszendentalien".to_string(), "dagegengegenstrukturalien".to_string(), "dagegengegenstrukturalie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(138)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["neutrale_Gegentranszendentalie".to_string(), "neutralegegentranszendentalie".to_string(), "neutralegegentranszendentalien".to_string(), "neutralegegenstrukturalien".to_string(), "neutralegegenstrukturalie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(202)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Unternehmung_Geschäft".to_string(), "unternehmen".to_string(), "unternehmung".to_string(), "geschaeft".to_string(), "geschäft".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(6), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["wertvoll".to_string(), "wert".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(6), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Beherrschen".to_string(), "regieren".to_string(), "beherrschen".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(7), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Richtung".to_string(), "richtung".to_string(), "gut".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyAtom::Tuple(vec![PyAtom::Int(7), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()],
    datas: vec![
        vec![PyAtom::Int(84)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Gegentranszendentalien".to_string(), "gegentranszendentalien".to_string(), "gegentranszendentalie".to_string(), "gegenstrukturalien".to_string(), "gegenalien".to_string(), "gegenuniversalien".to_string()],
    datas: vec![
        vec![PyAtom::Int(138), PyAtom::Int(202)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Systemsachen".to_string(), "systemsachen".to_string()],
    datas: vec![
        vec![PyAtom::Int(150)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Transzendentalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string(), "strukturalien".to_string(), "alien".to_string(), "universalien".to_string()],
    datas: vec![
        vec![PyAtom::Int(198), PyAtom::Int(390), PyAtom::Int(5), PyAtom::Int(54), PyAtom::Int(55)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Reziproke_von_Transzendentalien".to_string(), "transzendentalienreziproke".to_string(), "transzendentaliereziproke".to_string(), "strukturalienreziproke".to_string(), "alienreziproke".to_string(), "universalienreziproke".to_string()],
    datas: vec![
        vec![PyAtom::Int(131), PyAtom::Int(201)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Netzwerk".to_string(), "netzwerk".to_string()],
    datas: vec![
        vec![PyAtom::Int(25), PyAtom::Int(386), PyAtom::Int(390), PyAtom::Int(55)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["warum_Transzendentalie_=_Strukturgroesse_=_Charakter".to_string(), "warumtranszendentaliezustrukturgroesseundcharakter".to_string()],
    datas: vec![
        vec![PyAtom::Int(165), PyAtom::Int(4), PyAtom::Int(5), PyAtom::Int(54)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Kategorie".to_string(), "kategorie".to_string()],
    datas: vec![
        vec![PyAtom::Int(204), PyAtom::Int(205), PyAtom::Int(281)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Raum-Missionen".to_string(), "weltall".to_string()],
    datas: vec![
        vec![PyAtom::Int(218)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Programmier-Paradigmen".to_string(), "programmierparadigmen".to_string()],
    datas: vec![
        vec![PyAtom::Int(351)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Raum-Missionen".to_string(), "weltall".to_string()],
    datas: vec![
        vec![PyAtom::Int(218)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Geist__(15)".to_string(), "geist".to_string()],
    datas: vec![
        vec![PyAtom::Int(242), PyAtom::Int(426)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});

        let mut kombiParaNdataMatrix: IndexMap<i64, Vec<String>> = IndexMap::new();
        kombiParaNdataMatrix.insert(1, vec!["Lebewesen".to_string(), "tiere".to_string(), "tier".to_string(), "lebewesen".to_string()]);
        kombiParaNdataMatrix.insert(2, vec!["Berufe".to_string(), "berufe".to_string(), "beruf".to_string()]);
        kombiParaNdataMatrix.insert(3, vec!["Kreativität_und_Intelligenz".to_string(), "kreativität".to_string(), "intelligenz".to_string(), "kreativitaet".to_string()]);
        kombiParaNdataMatrix.insert(4, vec!["Liebe".to_string(), "liebe".to_string()]);
        kombiParaNdataMatrix.insert(7, vec!["Männer".to_string(), "männer".to_string(), "maenner".to_string(), "frauen".to_string()]);
        kombiParaNdataMatrix.insert(8, vec!["Persönlichkeit_evolutionär_erwerben".to_string(), "evolution".to_string(), "erwerben".to_string(), "persoenlichkeit".to_string(), "persönlichkeit".to_string()]);
        kombiParaNdataMatrix.insert(9, vec!["Religion".to_string(), "religion".to_string(), "religionen".to_string()]);
        kombiParaNdataMatrix.insert(10, vec!["Motive_Ziele".to_string(), "motivation".to_string(), "ziele".to_string(), "ziel".to_string(), "motive".to_string()]);
        kombiParaNdataMatrix.insert(12, vec!["Emotionen".to_string(), "emotionen".to_string(), "gefuehle".to_string(), "emotion".to_string(), "gefühl".to_string(), "gefühle".to_string()]);
        kombiParaNdataMatrix.insert(13, vec!["Personen".to_string(), "personen".to_string(), "berühmtheiten".to_string(), "beruehmtheiten".to_string()]);
        kombiParaNdataMatrix.insert(16, vec!["Wirtschaftssysteme".to_string(), "wirtschaftssystem".to_string(), "wirtschaftssysteme".to_string(), "kombinierteswirtschaftssystem".to_string(), "kombiniertewirtschaftssysteme".to_string()]);
        kombiParaNdataMatrix.insert(17, vec!["Eigentum_und_Besitz".to_string()]);

        let mut kombiParaNdataMatrix2: IndexMap<i64, Vec<String>> = IndexMap::new();
        kombiParaNdataMatrix2.insert(1, vec!["Lebewesen".to_string(), "tiere".to_string(), "tier".to_string(), "lebewesen".to_string()]);
        kombiParaNdataMatrix2.insert(2, vec!["Berufe".to_string(), "berufe".to_string(), "beruf".to_string()]);
        kombiParaNdataMatrix2.insert(5, vec!["Transzendentalien_Strukturalien".to_string(), "transzendenz".to_string(), "transzendentalien".to_string(), "strukturalien".to_string(), "alien".to_string()]);
        kombiParaNdataMatrix2.insert(6, vec!["Primzahlkreuz".to_string(), "leibnitz".to_string(), "primzahlkreuz".to_string()]);
        kombiParaNdataMatrix2.insert(8, vec!["Persönlichkeit_evolutionär_erwerben".to_string(), "evolution".to_string(), "erwerben".to_string(), "persoenlichkeit".to_string(), "persönlichkeit".to_string()]);
        kombiParaNdataMatrix2.insert(9, vec!["Religion".to_string(), "religion".to_string(), "religionen".to_string()]);
        kombiParaNdataMatrix2.insert(10, vec!["Motive_Ziele".to_string(), "motivation".to_string(), "motive".to_string(), "ziele".to_string(), "ziel".to_string()]);
        kombiParaNdataMatrix2.insert(11, vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()]);
        kombiParaNdataMatrix2.insert(13, vec!["Personen".to_string(), "personen".to_string(), "berühmtheiten".to_string(), "beruehmtheiten".to_string()]);
        kombiParaNdataMatrix2.insert(14, vec!["Mechanismen_der_Zuechtung".to_string(), "mechanismen".to_string(), "wesen".to_string(), "zuechten".to_string(), "züchten".to_string()]);
        kombiParaNdataMatrix2.insert(15, vec!["Gegentranszendentalien".to_string(), "gegentranszendentalien".to_string(), "gegenstrukturalien".to_string()]);
        kombiParaNdataMatrix2.insert(17, vec!["Maschinen".to_string(), "maschinen".to_string(), "geräte".to_string(), "geraete".to_string()]);
        kombiParaNdataMatrix2.insert(18, vec!["Geist".to_string(), "geist".to_string()]);
        kombiParaNdataMatrix2.insert(19, vec!["Bewusstsein".to_string(), "bewusstsein".to_string()]);

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
            paraNdataMatrix,
            kombiParaNdataMatrix,
            kombiParaNdataMatrix2,
            grundstrukturen_name0: "grundstrukturen".to_string(),
            wahl15,
        }
    }
}

pub const PYTHON_SOURCE__WORDS_PARA_NDATA_MATRIX: &str = r#"[
    (
        ParametersMain.wichtigste,
        (
            _("Wichtigste"),
            _("wichtigste"),
        ),
        {10, 5, 4, 8},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Mensch-zu-Tier"),
            _("menschtier"),
            _("tiermensch"),
        ),
        {314},
    ),
    (
        ParametersMain.religionen,
        (
            _("Superkräfte"),
            _("Superkraefte"),
        ),
        {444, 494, 496, 503},
    ),
    (
        ParametersMain.galaxie,
        (
            _("Superkräfte"),
            _("Superkraefte"),
        ),
        {444, 494, 496},
    ),

    (
        ParametersMain.universum,
        (
            _("Evolution_vs_Design_intelligent"),
        ),
        {519},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Evolution_vs_Design_intelligent"),
        ),
        {519},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Superkräfte"),
            _("Superkraefte"),
        ),
        {444, 494, 496},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Formationen"),
        ),
        {461},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Ansichten_Standpunkte_(18_17)"),
            _("ansichten"),
        ),
        {240, 346},
    ),
    (
        ParametersMain.menschliches,
        (
            _("(politische)_Richtungen_(7)"),
            _("richtungen"),
            _("politische"),
        ),
        {235},
    ),
    (
        ParametersMain.planet,
        (
            _("Wirklichkeiten_(10)"),
            _("wirklichkeit"),
            _("wirklichkeiten"),
        ),
        {233, 265, 268, 322, 420},
    ),
    (
        ParametersMain.planet,
        (
            _("Meta-Systeme_(12)"),
            _("metasysteme"),
            _("metasystem"),
            _("meta-systeme"),
            _("meta-system"),
        ),
        {232, 288, 334, 410, 411, 483, 79, 80, 497, 498, 499},
    ),
    (
        ParametersMain.planet,
        (_("Intelligenz"), _("intelligenz")),
        {214},
    ),
    (
        ParametersMain.planet,
        (
            _("Gleichheit_Freiheit_Ordnung"),
            _("gleichheit"),
            _(freiheitGleichheit[0]),
            _(freiheitGleichheit[1]),
        ),
        {132, 324, 328, 79, 80, 331, 335, 497, 498, 499},
    ),
    (
        ParametersMain.planet,
        (
            _("Komplexität"),
            _("komplexität"),
            _("komplexitaet"),
        ),
        {213},
    ),
    (
        ParametersMain.planet,
        (
            _("Mechanismen"),
            _("mechanismen"),
            _("mechanismus"),
        ),
        {107},
    ),
    (
        ParametersMain.wichtigste,
        (
            _("Zweitwichtigste"),
            _("zweitwichtigste"),
        ),
        {19, 65, 183},
        set(),
        set(),
        set(),
        {(10,)},
    ),
    (
        ParametersMain.wichtigste,
        (
            _("Drittwichtigste"),
            _("drittwichtigste"),
        ),
        {64},
    ),
    (
        ParametersMain.wichtigste,
        (
            _("Motive_Sternpolygone"),
            _("viertwichtigste"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primMotivStern"},
    ),
    (
        ParametersMain.wichtigste2,
        (_("Wichtigste"), _("wichtigstes")),
        {0, 1, 2, 36, 37, 207},
    ),
    (
        ParametersMain.wichtigste2,
        (
            _("Zweitwichtigste"),
            _("zweitwichtigste"),
        ),
        {30},
    ),
    (
        ParametersMain.operationen,
        (
            _("Halbierung"),
            _("halbierung"),
            _("halbierungen"),
        ),
        {86},
    ),
    (
        ParametersMain.religionen,
        (
            _("Religions-Gründer-Typ"),
            _("religionsgründertyp"),
            _("prophet"),
            _("archon"),
            _("religionsgruendertyp"),
        ),
        {72, 503},
    ),
    (
        ParametersMain.religionen,
        (_("Satan_Teufel"),),
        {495},
    ),
    (
        ParametersMain.menschliches,
        (_("Satan_Teufel"),),
        {495},
    ),
    (
        ParametersMain.religionen,
        (_("Hinduismus"), _("hinduismus")),
        {217},
    ),
    (
        ParametersMain.religionen,
        (_("Sternpolygon"), _("sternpolygon")),
        {0, 6, 36},
    ),
    (
        ParametersMain.religionen,
        (
            _("der_Tierkreiszeichen"),
            _("dertierkreiszeichen"),
            _("babylon"),
        ),
        {0, 36, 207, 477, 478},
    ),
    (
        ParametersMain.religionen,
        (
            _("Sternpolygon_vs_gleichförmiges"),
            _("vergleich"),
            _("sternpolygonvsgleichfoermiges"),
            _("vergleichnvs1divn"),
        ),
        {87},
    ),
    (
        ParametersMain.religionen,
        (
            _("Messias"),
            _("messias"),
            _("heptagramm"),
            _("hund"),
            _("messiase"),
            _("messiasse"),
        ),
        {7, 503},
    ),
    (
        ParametersMain.religionen,
        (
            _("gleichförmiges_Polygon"),
            _("gleichförmigespolygon"),
            _("gleichfoermigespolygon"),
            _("nichtsternpolygon"),
            _("polygon"),
        ),
        {16, 37},
    ),
    (
        ParametersMain.religionen,
        (
            _("Vertreter_höherer_Konzepte"),
            _("vertreterhoehererkonzepte"),
            _("galaxien"),
            _("galaxie"),
            _("schwarzesonne"),
            _("schwarzesonnen"),
            _("universum"),
            _("universen"),
            _("kreis"),
            _("kreise"),
            _("kugel"),
            _("kugeln"),
        ),
        {23},
    ),
    (
        ParametersMain.galaxie,
        (
            _("Lebewesen_Galaxie_am_Besten"),
        ),
        {470, 471, 473},
    ),
    (
        ParametersMain.galaxie,
        (
            _("Offenbarung_des_Johannes"),
            _("offenbarung"),
            _("offenbarungdesjohannes"),
            _("johannes"),
            _("bibel"),
            _("offenbarungjohannes"),
        ),
        {90},
    ),
    (
        ParametersMain.inkrementieren,
        (
            _("Teilchen-Meta-Physik"),
            _("addition"),
            _("identitaet"),
            _("Identität"),
        ),
        {219, 223, 307, 308, 333, 387, 388, 406},
    ),
    (
        ParametersMain.galaxie,
        (
            _("Hochzüchten"),
            _("hochzüchten"),
            _("hochzuechten"),
        ),
        {318, 319},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Teilchen_anderes_Universum"),
        ),
        {512,},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Teilchen_anderes_Universum"),
        ),
        {512,},
    ),
    (
        ParametersMain.universum,
        (
            _("Teilchen_anderes_Universum"),
        ),
        {512,},
    ),
    (
        ParametersMain.universum,
        (
            _("Zusammenhang_Gehirn_Kosmos_Universum"),
        ),
        {489,},
    ),
    (
        ParametersMain.universum,
        (
            _("Zahlenarten"),
        ),
        {462,},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Bestrafung"),
        ),
        {463,},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Bestrafung"),
        ),
        {463,},
    ),
    (
        ParametersMain.menschliches,
        (
            _("weniger_am_Menschen"),
        ),
        {464,},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Erlösung"),
            _("Erloesung"),
        ),
        {465,},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Erlösung"),
            _("Erloesung"),
        ),
        {465,},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Gewalt"),
        ),
        {466},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Gewalt"),
        ),
        {466, 479},
    ),
    (
        ParametersMain.universum,
        (
            _("Farben"),
        ),
        {444,},
    ),
    (
        ParametersMain.universum,
        (
            _("künstliches_Leben_(15)"),
            _("künstlichesleben"),
            _("grosseki"),
        ),
        {409},
    ),
    (
        ParametersMain.universum,
        (
            _("Software-Lizenzen_akademische_Grade"),
            _("softwarelizenz"),
            _("akademischeGrade"),
        ),
        {422},
    ),
    (
        ParametersMain.universum,
        (_("Strategie_Taktik_(15m8)"), _("strategie"), _("taktik")),
        {385},
    ),
    (
        ParametersMain.universum,
        (_("Universelles_Verhältnis_gleicher_Zahlen"), verhaeltnisgleicherzahlWort),
        {383},
    ),
    (
        ParametersMain.universum,
        (
            _("universelles_Recht"),
            _("recht"),
            _("jura"),
        ),
        {382, 34, 65},
    ),
    (
        ParametersMain.universum,
        (
            _("sowas_wie_Kombinieren_Verknüpfen"),
            _("kombinierenetc"),
        ),
        {320},
    ),
    (
        ParametersMain.universum,
        (
            _("Hochzüchten"),
            _("hochzüchten"),
            _("hochzuechten"),
        ),
        {318, 319},
    ),
    (
        ParametersMain.universum,
        (_("Teilchen-Meta-Physik"),),
        {219, 308},
    ),
    (
        ParametersMain.teilchen,
        (_("das_Universelle_(15)"),),
        {219, 308},
    ),
    (
        ParametersMain.teilchen,
        (_("Wirklichkeiten_(10)"), _("wirklichkeit"), _("wirklichkeiten")),
        {420},
    ),
    (
        ParametersMain.teilchen,
        (_("das_Galaktische_(14)"),),
        {406},
    ),
    (
        ParametersMain.teilchen,
        (_("das_Multiverselle_(16)"),),
        {388, 418},
    ),
    (
        ParametersMain.teilchen,
        (_("die_Tugendsortierung_(13_mit_14)"),),
        {411},
    ),
    (
        ParametersMain.teilchen,
        (_("die_Galaxie_Unterbereiche_(13)"),),
        {223, 307, 412},
    ),
    (
        ParametersMain.teilchen,
        (_("das_Gute_die_Richtung_(7)"),),
        {333},
    ),
    (
        ParametersMain.teilchen,
        (_("Raum_und_Dimensionen_(8)"),),
        {387},
    ),
    (
        ParametersMain.universum,
        (
            _("keine_Nur-Paradigma-Religionen"),
            _("metaparadigmareligion"),
        ),
        {190, 191, 196},
    ),
    (
        ParametersMain.universum,
        (
            _("Kugeln_Kreise"),
            _("kugelnkreise"),
            kugelnKreise[0],
            kugelnKreise[1],
        ),
        {77, 145},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Raumzeit_Anordnung_mathematisch_universell"),
        ),
        {472},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Multiversalien_(16)"),
            _("multiversalien"),
        ),
        {389},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Meta-Physik-Teilchen_(1)"),
            _("teilchen"),
        ),
        {388},
    ),
    (
        ParametersMain.galaxie,
        (
            _("Kugeln_Kreise"),
            _("kugelnkreise"),
            _("kugeln"),
            _("kreise"),
        ),
        {77, 145},
    ),
    (
        ParametersMain.galaxie,
        (
            _("chinesisches_Horoskop"),
            _("chinesischeshoroskop"),
            _("china"),
        ),
        {91},
    ),
    (
        ParametersMain.galaxie,
        (
            _("babylonische_Tierkreiszeichen"),
            _("tierkreiszeichen"),
            _("babylon"),
        ),
        {1, 2},
    ),
    (
        ParametersMain.galaxie,
        (_("Thomasevangelium"), _("thomasevangelium"), thomasWort),
        {0, 3, 303},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Netzwerk"),
            _("netzwerk"),
        ),
        {417, 436},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Kontroverse_(51)"),
            _("kontroverse"),
        ),
        {421},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("mathematisches_Design_(32)"),
            _("mathematischesdesign"),
        ),
        {419},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("analytische_Ontologie"),
            _("analytischeontologie"),
            _("ontologie"),
        ),
        {84},
    ),
    (
        ParametersMain.galaxie,
        (
            _("analytische_Ontologie"),
            _("analytischeontologie"),
            _("ontologie"),
        ),
        {84},
    ),
    (
        ParametersMain.galaxie,
        (
            _("Transzendentalien_innen_außen"),
            _("innenaussenstrukur"),
            _("strukturalieninnenaußen"),
            _("strukturalieninnenaussen"),
            _("innenaußenstrukur"),
            _("transzendentalieninnenaußen"),
            _("transzendentalieninnenaussen"),
        ),
        {149},
    ),
    (
        ParametersMain.galaxie,
        (
            _("Modallogik"),
            _("modallogik"),
        ),
        {148},
    ),
    (
        ParametersMain.operationen,
        (
            _("5"),
            _("fünf"),
            _("fünfer"),
            _("fünferstruktur"),
            _("fuenf"),
            _("fuenfer"),
            _("fuenferstruktur"),
        ),
        {96},
    ),
    (
        ParametersMain.operationen,
        (
            _("9"),
            _("neun"),
            _("neuner"),
            _("neunerstruktur"),
        ),
        {94},
    ),
    (
        ParametersMain.operationen,
        (
            _("3"),
            _("drei"),
            _("dreier"),
            _("dreierstruktur"),
        ),
        {92, 93, 315, 316},
    ),
    (
        ParametersMain.strukturgroesse,
        (
            _("Licht"),
            _("licht"),
        ),
        {20, 27, 313},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Strukturgrösse"),
            ParametersMain.strukturgroesse[0],
            _("größe"),
            _("groesse"),
            _("gross"),
            _("strukturgroesse"),
            _("strukturgroeße"),
            _("strukturgrösse"),
            _("strukturgröße"),
        ),
        {4, 21, 54, 197, 425},
    ),
    (
        ParametersMain.strukturgroesse,
        (
            _("Strukturgrösse"),
            ParametersMain.strukturgroesse[0],
            _("größe"),
            _("groesse"),
            _("gross"),
            _("strukturgroesse"),
            _("strukturgroeße"),
            _("strukturgrösse"),
            _("strukturgröße"),
        ),
        {4, 21, 54, 197, 425},
    ),
    (
        ParametersMain.strukturgroesse,
        (
            _("Organisationen"),
            _("organisationen"),
            organisationWort,
        ),
        {30, 82, 425},
    ),
    (
        ParametersMain.strukturgroesse,
        (
            _("politische_Systeme"),
            _("politischesysteme"),
            _("politik"),
        ),
        {83},
    ),
    (
        ParametersMain.universummetakonkret,
        (_("meta"),),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                2,
                0,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (_("konkret"),),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                2,
                1,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (_("Theorie"), _("theorie")),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                3,
                0,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (_("Praxis"), _("praxis")),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                3,
                1,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (
            _("Management"),
            _("management"),
            _("stau"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                4,
                0,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (
            _("verändernd"),
            _("veraendernd"),
            _("fluss"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                4,
                1,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (
            _("ganzheitlich"),
            _("mathematisch_diskret"),
            _("diskret"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                5,
                0,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (
            _("darüber_hinausgehend"),
            _("hinausgehend"),
            _("kontinuierlich"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                5,
                1,
            ),
        },
    ),
    (
        ParametersMain.primzahlwirkung,
        (
            _("Universum_Strukturalien_Transzendentalien"),
            _("universum"),
            _("strukturalie"),
            _("strukturalien"),
            transzendentalienWort,
            _("transzendentalie"),
        ),
        set(),
        set(),
        set(),
        set(),
        {(5,)},
    ),
    (
        ParametersMain.primzahlwirkung,
        (
            _("Richtung_als_Richtung"),
            _("richtungrichtung"),
        ),
        set(),
        set(),
        set(),
        set(),
        {(None,)},
    ),
    (
        ParametersMain.primzahlwirkung,
        (
            GalaxieabsichtWort,
            _("absichtgalaxie"),
            _("absicht"),
            _("motive"),
            _("motiv"),
            _("absichten"),
            _("galaxie"),
        ),
        set(),
        set(),
        set(),
        set(),
        {(10,)},
    ),
    (
        ParametersMain.primzahlwirkung,
        (
            _("Absicht_Reziproke_Galaxie"),
            _("absichtgalaxiereziproke"),
            _("absichtreziproke"),
            _("motivereziproke"),
            _("motivreziproke"),
            _("absichtenreziproke"),
            _("galaxiereziproke"),
        ),
        set(),
        set(),
        set(),
        set(),
        {(42,)},
    ),
    (
        ParametersMain.primzahlwirkung,
        (
            _("Universum_Reziproke"),
            _("universumreziproke"),
            _("strukturaliereziproke"),
            _("strukturalienreziproke"),
            _("transzendentalienreziproke"),
            transzendentaliereziprokeWort,
        ),
        set(),
        set(),
        set(),
        set(),
        {(131,)},
    ),
    (
        ParametersMain.primzahlwirkung,
        (
            _("Dagegen-Gegentranszendentalie"),
            _("dagegengegentranszendentalie"),
            _("dagegengegentranszendentalien"),
            _("dagegengegenstrukturalien"),
            _("dagegengegenstrukturalie"),
        ),
        set(),
        set(),
        set(),
        set(),
        {(138,)},
    ),
    (
        ParametersMain.primzahlwirkung,
        (
            _("neutrale_Gegentranszendentalie"),
            _("neutralegegentranszendentalie"),
            _("neutralegegentranszendentalien"),
            _("neutralegegenstrukturalien"),
            _("neutralegegenstrukturalie"),
        ),
        set(),
        set(),
        set(),
        set(),
        {(202,)},
    ),
    (
        ParametersMain.universummetakonkret,
        (
            _("Unternehmung_Geschäft"),
            _("unternehmen"),
            _("unternehmung"),
            _("geschaeft"),
            _("geschäft"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                6,
                0,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (_("wertvoll"), _("wert")),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                6,
                1,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (
            _("Beherrschen"),
            _("regieren"),
            _("beherrschen"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                7,
                0,
            ),
        },
    ),
    (
        ParametersMain.universummetakonkret,
        (
            _("Richtung"),
            _("richtung"),
            _("gut"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {
            (
                7,
                1,
            ),
        },
    ),
    (
        ParametersMain.universum,
        (
            _("analytische_Ontologie"),
            _("analytischeontologie"),
            _("ontologie"),
        ),
        {84},
    ),
    (
        ParametersMain.universum,
        (
            _("Gegentranszendentalien"),
            _("gegentranszendentalien"),
            _("gegentranszendentalie"),
            _("gegenstrukturalien"),
            _("gegenalien"),
            _("gegenuniversalien"),
        ),
        {138, 202},
    ),
    (
        ParametersMain.universum,
        (_("Systemsachen"), _("systemsachen")),
        {
            150,
        },
    ),
    (
        ParametersMain.universum,
        (
            _("Transzendentalien"),
            _("transzendentalien"),
            _("transzendentalie"),
            _("strukturalien"),
            _("alien"),
            _("universalien"),
        ),
        {5, 54, 55, 198, 390},
    ),
    (
        ParametersMain.universum,
        (
            _("Reziproke_von_Transzendentalien"),
            _("transzendentalienreziproke"),
            _("transzendentaliereziproke"),
            _("strukturalienreziproke"),
            _("alienreziproke"),
            _("universalienreziproke"),
        ),
        {131, 201},
    ),
    (
        ParametersMain.universum,
        (_("Netzwerk"), netzwerkWort),
        {25, 55, 386, 390},
    ),
    (
        ParametersMain.universum,
        (
            _("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"),
            _("warumtranszendentaliezustrukturgroesseundcharakter"),
        ),
        {4, 54, 5, 165},
    ),
    (
        ParametersMain.universum,
        (_("Kategorie"), _("kategorie")),
        {204, 205, 281},
    ),
    (
        ParametersMain.universum,
        (_("Raum-Missionen"), _("weltall")),
        {218},
    ),
    (
        ParametersMain.universum,
        (
            _("Programmier-Paradigmen"),
            _("programmierparadigmen"),
        ),
        {351},
    ),
    (
        ParametersMain.galaxie,
        (_("Raum-Missionen"), _("weltall")),
        {218},
    ),
    (
        ParametersMain.universum,
        (_("Geist__(15)"), geistWort),
        {242, 426},
    ),
    (
        ParametersMain.universum,
        (
            _("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"),
            _("warumtranszendentaliegleichkomplexitaet"),
        ),
        {65, 5, 166},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Model_of_Hierarchical_Complexity"),
            _("modelofhierarchicalcomplexity"),
            _("komplex"),
            _("komplexität"),
            _("komplexitaet"),
            _("complexity"),
            _("model"),
            _("abstraktion"),
        ),
        {65, 75, 203, 483},
    ),
    (
        ParametersMain.universum,
        (
            _("Model_of_Hierarchical_Complexity"),
            _("modelofhierarchicalcomplexity"),
            komplexWort,
            _("komplexität"),
            _("komplexitaet"),
            _("complexity"),
            _("model"),
            _("abstraktion"),
        ),
        {65, 75, 203},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Model_of_Hierarchical_Complexity"),
            _("modelofhierarchicalcomplexity"),
            komplexWort,
            _("komplexität"),
            _("komplexitaet"),
            _("complexity"),
            _("model"),
            _("abstraktion"),
        ),
        {65, 75, 203},
    ),
    (
        ParametersMain.operationen,
        (
            _("2"),
            _("zwei"),
            _("gerade"),
            _("ungerade"),
            _("alternierung"),
            _("alternierend"),
            _("zweierstruktur"),
        ),
        {78, 79, 80, 331, 497, 498, 499},
    ),
    (
        ParametersMain.operationen,
        (
            _("Multiplikation"),
            _("multiplikation"),
        ),
        {158},
    ),
    (
        ParametersMain.operationen,
        (
            _("4"),
            _("vier"),
            _("viererstruktur"),
            _("viererabfolgen"),
        ),
        {76, 77, 81, 104, 145},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Gesellschaftsschicht"),
            _("klasse"),
            _("klassen"),
        ),
        {241},
    ),
    (
        ParametersMain.menschliches,
        (_("Moral"), _("moral"), _("warummoral")),
        {215, 216},
        {(216, 221)},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Fachgebiete"),
            _("fachgebiete"),
            _("fachbereiche"),
            _("themen"),
        ),
        {183},
    ),
    (
        ParametersMain.wirtschaft,
        (
            _("Fachgebiete"),
            _("fachgebiete"),
            _("fachbereiche"),
            _("themen"),
        ),
        {183},
    ),
    (
        ParametersMain.wirtschaft,
        (
            _("Pflanzen"),
            _("pflanzen"),
        ),
        {113},
    ),
    (
        ParametersMain.wirtschaft,
        (
            _("Maschinen"),
            _("maschinen"),
            _("maschine"),
            _("gerät"),
            _("geräte"),
            _("geraete"),
            _("geraet"),
        ),
        {89},
    ),
    (
        ParametersMain.wirtschaft,
        (
            _("Organisationsform"),
            _("organisationsform"),
            _("organisationsart"),
            _("firma"),
            _("verein"),
        ),
        {99},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("System"),
            _("system"),
        ),
        {
            69,
            70,
            440,
            455,
            476,
            513,
        },
    ),
    (
        ParametersMain.wirtschaft,
        (
            _("System"),
            _("system"),
        ),
        {
            69,
            70,
            440,
            455,
            476,
            513,
        },
    ),
    (
        ParametersMain.wirtschaft,
        (
            _("Erklärung"),
            _("erklärung"),
            _("erklaerung"),
        ),
        {71},
    ),
    (
        ParametersMain.wirtschaft,
        (
            _("BWL"),
            _("bwl"),
        ),
        {109},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Sinn_des_Lebens"),
            _("sinndeslebens"),
            _("lebenssinn"),
            _("sinn"),
            _("sinnsuche"),
        ),
        {88, 189},
        {(181, 182)},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Intelligenzprobleme"),
            _("intelligenzprobleme"),
            _("intelligenzmaengel"),
            _("intelligenzmängel"),
        ),
        {147},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Denkweise_von_Lebewesen"),
            _("lebewesendenkweise"),
            _("denkweise"),
        ),
        {146},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Gegentranszendentalien"),
            _("gegentranszendentalien"),
            _("gegenstrukturalien"),
        ),
        {138, 139, 202},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Gleichheit_Freiheit"),
            _("gleichheitfreiheit"),
            _("ungleichheit"),
            _("dominieren"),
            _("gleichheit"),
            _("freiheit"),
        ),
        {132, 328, 331, 335},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Gefühle"),
            _("emotionen"),
            _("gefuehle"),
            emotionWort,
            _("gefühl"),
            _("gefuehl"),
        ),
        {105, 230, 243, 283, 284, 285, 286, 305},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Egoismus"),
            _("egoismus"),
            _("altruismus"),
            _("selbstlosigkeit"),
        ),
        {136},
        {(66, 67)},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Wirkung"),
            _("wirkung"),
        ),
        {135},
    ),
    (
        ParametersMain.menschliches,
        (
            _("INCELs"),
            _("incel"),
            _("incels"),
        ),
        {68},
    ),
    (
        ParametersMain.menschliches,
        (
            _("irrationale_Zahlen_durch_Wurzelbildung"),
            _("irrationalezahlendurchwurzelbildung"),
            _("ausgangslage"),
        ),
        {73},
    ),
    (
        ParametersMain.menschliches,
        (
            _("dominierendes_Geschlecht"),
            _("dominierendesgeschlecht"),
            _("maennlich"),
            _("männlich"),
            _("weiblich"),
        ),
        {51},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Liebe"),
            _("liebe"),
            _("ethik"),
        ),
        {8, 9, 28, 208, 330},
        {(121, 122)},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Glaube_Erkenntnis"),
            _("glauben"),
            _("erkenntnis"),
            _("glaube"),
        ),
        {59},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Angreifbarkeit"),
            _("angreifbarkeit"),
            _("angreifbar"),
        ),
        {58, 57},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
            _("Transzendentalien"),
            _("transzendentalien"),
            _("transzendentalie"),
            _("strukturalien"),
            _("alien"),
            _("universalien"),
            _("meta-paradigmen"),
        ),
        {5, 229, 131},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)"),
            _("Transzendentalien"),
            _("transzendentalien"),
            _("transzendentalie"),
            _("strukturalien"),
            _("alien"),
            _("universalien"),
            _("meta-paradigmen"),
        ),
        {5, 229, 131},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Bedingung_und_Auslöser_(1/3)"),
            _("bedingung"),
            _("bedingungen"),
            _("auslöser"),
            _("ausloeser"),
        ),
        {338},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Lebensbereiche_Problemklassen_(28)"),
            _("lebensbereiche"),
            _("lebensfelder"),
            _("problemklassen"),
        ),
        {405, 415, 416},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Maßnahmen_(39)"),
            _("massnahmen"),
        ),
        {384},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)"),
            _("relativreziprokuniversell"),
        ),
        {350},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("universeller_Komperativ_(18→15)"),
            _("universellerkomperativ"),
        ),
        {349},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Existenzialien_(3)"),
            _("existenzialien"),
        ),
        {348},
    ),
    (
        ParametersMain.grundstrukturen,
        (_("Extremalien_(19)"), _("extremalien")),
        {347, 352},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Erwartungshaltungen_(26)"),
            _("erwartungen"),
            _("erwartungshaltungen"),
        ),
        {344},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Leidenschaften_(21)"),
            _("leidenschaft"),
            _("leidenschaften"),
        ),
        {343},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("relativer_Zeit-Betrag_(15_10_4_18_6)"),
            _("relativerzeitbetrag"),
        ),
        {339},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Zahlenvergleich_(15_18_6)"),
            _("zahlenvergleich"),
        ),
        {340},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Bestrebungen(1/5)"),
            _("bestrebung"),
            _("bestrebungen"),
        ),
        {332, 414},
    ),
    (
        ParametersMain.grundstrukturen,
        (_("Prinzipien(1/8)"), _("prinzipien")),
        {329, 378},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Attraktionen_(36)"),
            _("attraktionen"),
        ),
        {311},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Optimierung_(10)"),
            _("optimierung"),
        ),
        {310},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Themen_(6)"),
            _("themen"),
            _("thema"),
        ),
        {309},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Bedeutung_(10)"),
            _("bedeutung"),
        ),
        {306},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Reziprokes"),
            _("reziproke"),
            _("reziprokes"),
        ),
        {
            42,
            131,
            204,
            231,
            273,
            257,
            284,
            285,
            257,
            204,
            205,
            281,
            326,
            327,
            328,
            329,
            330,
            331,
            332,
            334,
            335,
            338,
            416,
        },
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Achtung_(4)"),
            _("achtung"),
            _("achten"),
        ),
        {270, 393},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Zeit_(4)_als_Wirklichkeit"),
            _("zeit"),
        ),
        {266, 267},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Absicht_16_ist_zu_genügen"),
            _("absicht16"),
        ),
        {312},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Absicht_17_ist_zu_meinen"),
            _("absicht17"),
        ),
        {263},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Absicht_6_ist_Vorteilsmaximierung"),
            _("absicht6"),
        ),
        {262},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Absicht_7_ist_Selbstlosigkeit"),
            _("absicht7"),
        ),
        {261},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Bewusstheit_statt_Bewusstsein_(1)"),
            _("bewusstheit"),
        ),
        {282},
    ),
    (
        ParametersMain.grundstrukturen,
        (_("Verhalten_(11)"), _("verhalten")),
        {301, 302, 413},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Energie_und_universelle_Eigenschaften_(30)"),
            _("energie"),
            _("universelleeigenschaften"),
            _("lebensenergie"),
        ),
        {287, 293},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Garben_und_Verhalten_nachfühlen(31)"),
            _("garben"),
            _("verhaltenfuehlen"),
            _("verhaltenfühlen"),
        ),
        {295},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            Primzahlkreuz_pro_contra_strs_Fkt[1],
            _("nachvollziehen"),
        ),
        {242, 297},
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primzahlkreuzprocontra"},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Empathie_(37)"),
            _("empathie"),
            _("mitgefuehl"),
        ),
        {294},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Absicht_1/6_ist_Reinigung_und_Klarheit"),
            _("absicht1/6"),
            _("absicht1pro6"),
        ),
        {298},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("innere_Werte_1/6_der_Reinigung_und_Klarheit"),
            _("innerewerte"),
        ),
        {398, 399, 400, 401},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Absicht_10_ist_Wirklichkeit_erkennen"),
            _("absicht10"),
        ),
        {260},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Wohlbefinden_(7mit6)"),
            _("wohlbefinden"),
        ),
        {427, 428},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Geist_(15)"),
            geistWort,
            _("bewusstsein"),
        ),
        {229, 231, 242, 273, 297, 304, 426},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Geist_(15)"),
            geistWort,
            _("bewusstsein"),
        ),
        {229, 231, 242, 273, 297, 304, 426},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Reflexe_(3)"),
            _("reflex"),
            _("reflexe"),
        ),
        {256},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Lust_(9)"),
            _("lust"),
            _("einheiten"),
        ),
        {255, 391},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Paradigmen_sind_Absichten_(13)"),
            _("paradigmen"),
            _("absichten"),
        ),
        {10, 42, 410, 411, 493, 494 },
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Wirklichkeiten_Wahrheit_Wahrnehmung_(10)"),
            _("wirklichkeit"),
            _("wirklichkeiten"),
            _("wahrheit"),
            _("wahrnehmung"),
        ),
        {233, 265, 268, 322, 342, 480},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Rechnen"),
            _("rechnen"),
        ),
        {
            404,
        },
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Stimmungen_Kombinationen_(14)"),
            _("stimmung"),
            _("stimmungen"),
            _("kombination"),
            _("kombinationen"),
        ),
        {33, 290, 296, 325, 326, 327, 402, 403, 406, 407, 408, 430, 492},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Struktur-Wissenschaften_(10)"),
        ),
        {438,},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Muster-Wissenschaften_(20)"),
        ),
        {439, 484},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Anführer_Arten_(7)"),
        ),
         {429, 455, 481, 482, 490, 497, 498, 499, 502, 509},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Klassen_(20)"),
            _("klasse"),
            _("klassen"),
        ),
        {241, 289, 394, 395, 485, 516},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Ordnung_und_Filterung_12_und_1pro12"),
            _("ordnen"),
            _("ordnenundfiltern"),
            _("filtern"),
        ),
        {132, 328, 331, 335},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Meta-Systeme_(12)"),
            _("metasysteme"),
            _("metasystem"),
            _("meta-systeme"),
            _("meta-system"),
            _("menge"),
            _("mengen"),
        ),
        {232, 288, 334, 410, 411, 483, 79, 80, 497, 498, 499},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Absicht_1/8"),
            _("absicht1pro8"),
            _("absicht1/8"),
        ),
        {272, 379},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Ziele_(19)"),
            _("ziele"),
            _("maxima"),
            _("höhenvorstellungen"),
        ),
        {271},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Konkreta_und_Focus_(2)"),
            _("konkreta"),
            _("focus"),
            _("fokus"),
        ),
        {250, 269, 253},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Gefühle_(7)"),
            _("gefuehle"),
            _("emotionen"),
            emotionWort,
            _("gefühle"),
        ),
        {29, 243, 283, 284, 285, 286, 305},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("abhängige_Verbundenheit_(90)"),
            _("abhaengigkeit"),
            _("abhängigkeit"),
        ),
        {357},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Karte_Filter_und_Unterscheidung_(1/12)"),
            _("karte"),
            _("filter"),
            _("unterscheidung"),
        ),
        {377},
    ),
    (
        ParametersMain.grundstrukturen,
        (_("Fundament_(1/19)"), _("fundament")),
        {356},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Gedanken_sind_Positionen_(17)"),
            _("positionen"),
            _("gedanken"),
        ),
        {249, 317, 323},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Funktionen_Vorstellungen_(16)"),
            _("vorstellungen"),
            _("vorstellung"),
            _("funktionen"),
        ),
        {345, 264, 388, 418},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Sollen_Frage_Vorgehensweise_(1/13)"),
            _("sollen"),
            _("frage"),
            _("vorgehensweise"),
        ),
        {353, 354},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Ansichten_Standpunkte_(18_17)"),
            _("ansichten"),
        ),
        {240, 346},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Verbundenheiten_(18)"),
            _("verbundenheiten"),
        ),
        {252, 299, 300, 336},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Absicht_13_ist_Helfen"),
            _("absicht13"),
            _("helfen"),
        ),
        {370},
    ),
    (
        ParametersMain.grundstrukturen,
        (_("Liebe_(7)"), _("liebe")),
        {8, 9, 28, 208, 221, 330},
        {(121, 122)},
    ),
    (
        ParametersMain.grundstrukturen,
        (_("Koalitionen_(10)"), _("koalitionen")),
        {321},
    ),
    (
        ParametersMain.grundstrukturen,
        (_("gegen_5"),),
        {24},
    ),
    (
        ParametersMain.grundstrukturen,
        (_("Impulse_(5)"), _("impulse")),
        {251, 253, 257, 341},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Triebe_und_Bedürfnisse_(6)"),
            _("trieb"),
            _("triebe"),
            _("bedürfnis"),
            _("bedürfnisse"),
            _("werte"),
        ),
        {254, 392, 396, 397, 423},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Taetigkeiten"),
            _("tätigkeiten"),
            _("taetigkeiten"),
        ),
        {424},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Reflektion_und_Kategorien_(1/15)"),
            _("reflektion"),
            _("kategorien"),
        ),
        {204, 205, 281},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Modus_und_Sein_(8)"),
            _("zustaende"),
            _("zustände"),
            _("modus"),
            _("modi"),
            _("sein"),
        ),
        {234, 337, 385, 387, 491},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Motive"),
            _("motive"),
            motivationWort,
            _("motiv"),
            _("absicht"),
            _("absichten"),
        ),
        {10, 18, 42, 167, 168, 149, 229, 230},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Gedanken_sind_Positionen_(17)"),
            _("positionen"),
            _("gedanken"),
        ),
        {249, 276},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Bewusstsein_und_Wahrnehmung"),
            _("bewusstsein"),
            _("wahrnehmung"),
        ),
        {265, 229, 231, 281, 304, 342},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Errungenschaften"),
            _("errungenschaften"),
            _("ziele"),
            _("erhalten"),
        ),
        {11, 257, 251},
    ),
    (
        ParametersMain.menschliches,
        (
            _("evolutionär_erwerben_und_Intelligenz_Kreativität"),
            _("evolutionärerwerbenundintelligenz"),
            _("intelligenz"),
            _("erwerben"),
            _("erlernen"),
            _("lernen"),
            _("evolutionaer"),
            _("evolutionär"),
            _("kreativität"),
            _("kreativitaet"),
            _("kreativ"),
        ),
        {12, 47, 27, 13, 32},
    ),
    (
        ParametersMain.menschliches,
        (
            _("brauchen"),
            _("benoetigen"),
            _("benötigen"),
            _("notwendig"),
        ),
        {13, 14},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Krankheit"),
            _("krankheit"),
            _("krankheiten"),
            _("pathologisch"),
            _("pathologie"),
            _("psychiatrisch"),
        ),
        {24},
    ),
    (
        ParametersMain.menschliches,
        (
            _("alpha_beta"),
            _("alphabeta"),
            _("alpha"),
            _("beta"),
            _("omega"),
            _("sigma"),
        ),
        {46},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Anführer"),
            _("anfuehrer"),
            _("chef"),
        ),
        {29, 170, 429, 455, 490, 502, 509},
    ),
    (
        ParametersMain.grundstrukturen,
        (
            _("Biologischer_Baum_(15)"),
        ),
        {500},
    ),
    (
        ParametersMain.multiversum,
        (
            _("Biologischer_Baum_(16_->_5)"),
        ),
        {500},
    ),
    (
        ParametersMain.universum,
        (
            _("Biologischer_Baum_(15)"),
        ),
        {500},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Biologischer_Baum_(15)"),
        ),
        {500},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Manipulation"),
            _("manipulation"),
        ),
        {153},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Berufe"),
            _("berufe"),
            _("beruf"),
        ),
        {30},
    ),
    (
        ParametersMain.menschliches,
        (
            _("Lösungen"),
            _("lösungen"),
            _("loesungen"),
            _("loesung"),
            _("lösungen"),
        ),
        {31},
    ),
    (ParametersMain.menschliches, (_("Musik"), _("musik")), {33}),
    (
        ParametersMain.procontra,
        (
            _("ergibt_Sinn"),
            _("ergibtsinn"),
            _("machtsinn"),
            _("sinn"),
        ),
        {140},
    ),
    (
        ParametersMain.procontra,
        (
            _("Veränderung"),
            _("veraenderung"),
            _("veraendern"),
            _("veränderung"),
            _("verändern"),
        ),
        {142},
    ),
    (
        ParametersMain.procontra,
        (
            _("bändigen_kontrollieren"),
            _("baendigenkontrollieren"),
            _("kontrollieren"),
            _("baendigen"),
            _("bändigen"),
        ),
        {143},
    ),
    (
        ParametersMain.procontra,
        (
            _("vereinen"),
            _("einheit"),
        ),
        {144},
    ),
    (
        ParametersMain.procontra,
        (
            _("Vorteile"),
            _("vorteile"),
            _("veraenderungnutzen"),
        ),
        {141},
    ),
    (
        ParametersMain.procontra,
        (
            _("Gegenspieler"),
            _("gegenspieler"),
            _("antagonist"),
        ),
        {137},
    ),
    (
        ParametersMain.procontra,
        (_("nervig"),),
        {120},
    ),
    (
        ParametersMain.procontra,
        (
            _("pro_nutzen"),
            _("pronutzen"),
        ),
        {117},
    ),
    (
        ParametersMain.procontra,
        (
            _("Gegenposition"),
            _("gegenposition"),
        ),
        {116},
    ),
    (
        ParametersMain.procontra,
        (
            _("Hilfe_erhalten"),
            _("hilfeerhalten"),
        ),
        {114},
    ),
    (
        ParametersMain.procontra,
        (
            _("Helfen"),
            _("helfen"),
            _("hilfe"),
        ),
        {115},
    ),
    (
        ParametersMain.procontra,
        (
            _("Pro"),
            _("pro"),
            _("dafür"),
            _("dafuer"),
        ),
        {17, 48},
    ),
    (
        ParametersMain.procontra,
        (
            _("nicht_miteinander_auskommen"),
            _("nichtauskommen"),
        ),
        {123},
    ),
    (
        ParametersMain.procontra,
        (
            _("nicht_dagegen"),
            _("nichtdagegen"),
        ),
        {124},
    ),
    (
        ParametersMain.procontra,
        (
            _("kein_Gegenteil"),
            _("keingegenteil"),
        ),
        {125},
    ),
    (
        ParametersMain.procontra,
        (
            _("nicht_dafür"),
            _("nichtdafuer"),
        ),
        {126},
    ),
    (
        ParametersMain.procontra,
        (
            _("Hilfe_nicht_gebrauchen"),
            _("hilfenichtgebrauchen"),
        ),
        {127},
    ),
    (
        ParametersMain.procontra,
        (
            _("nicht_helfen_können"),
            _("nichthelfenkoennen"),
        ),
        {128},
    ),
    (
        ParametersMain.procontra,
        (
            _("nicht_abgeneigt"),
            _("nichtabgeneigt"),
        ),
        {129},
    ),
    (
        ParametersMain.procontra,
        (_("unmotivierbar"),),
        {130},
    ),
    (
        ParametersMain.procontra,
        (
            _("contra"),
            _("dagegen"),
        ),
        {15, 26},
    ),
    (
        ParametersMain.procontra,
        (
            _("Gegenteil"),
            _("gegenteil"),
        ),
        {100, 101, 222},
    ),
    (
        ParametersMain.procontra,
        (
            _("Harmonie"),
            _("harmonie"),
        ),
        {102, 103},
    ),
    (ParametersMain.licht, (), {20, 27, 313}),
    (
        ParametersMain.procontra,
        (
            Primzahlkreuz_pro_contra_strs_Fkt[0],
            _("primzahlkreuz"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primzahlkreuzprocontra"},
    ),
    (
        ParametersMain.bedeutung,
        (
            Primzahlkreuz_pro_contra_strs_Fkt[0],
            primzahlkreuzWort,
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primzahlkreuzprocontra"},
    ),
    (
        ParametersMain.bedeutung,
        (
            _("in_ReTa"),
            _("inreta"),
        ),
        {209, 210, 474, 475},
    ),
    (
        ParametersMain.bedeutung,
        (
            _("Vorzeichen"),
            _("vorzeichen"),
        ),
        {118, 119},
    ),
    (
        ParametersMain.bedeutung,
        (
            _("Primzahlen"),
            _("primzahlen"),
            _("vielfache"),
            _("vielfacher"),
        ),
        {19},
    ),
    (
        ParametersMain.bedeutung,
        (
            _("Anwendung_der_Sonnen_und_Monde"),
            _("anwendungdersonnenundmonde"),
            _("anwendungdersonnen"),
            _("anwendungenfuermonde"),
        ),
        {22},
    ),
    (
        ParametersMain.bedeutung,
        (
            _("Zählungen"),
            _("zählungen"),
            _("zaehlung"),
            _("zaehlungen"),
            _("zählung"),
        ),
        {25, 45, 169, 188, 386, 390},
    ),
    (
        ParametersMain.bedeutung,
        (
            _("Jura"),
            _("jura"),
            _("gesetzeslehre"),
            _("recht"),
        ),
        {34},
    ),
    (
        ParametersMain.bedeutung,
        (
            _("Vollkommenheit_des_Geistes"),
            _("vollkommenheit"),
            geistWort,
        ),
        {35},
    ),
    (
        ParametersMain.bedeutung,
        (
            _("Gestirn"),
            gestirnWort,
            _("mond"),
            _("sonne"),
            _("planet"),
        ),
        {64, 154},
        set(),
        set(),
        set(),
    ),
    (
        ParametersMain.bedeutung,
        (
            _("Konjunktiv_Wurzelbildung"),
            _("konjunktiv"),
            _("wurzel"),
        ),
        {106},
    ),
    (
        ParametersMain.bedeutung,
        (
            _("Mechanismen_der_Züchtung"),
            _("mechanismen"),
            _("wesen"),
            _("zuechtung"),
            _("züchtung"),
            _("züchten"),
            _("zuechten"),
        ),
        {107, 108, 109},
    ),
    (
        ParametersMain.gebrochengalaxie,
        {str(a) for a in range(2, gebrochenSpaltenMaximumPlus1)},
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {str(a) for a in range(2, gebrochenSpaltenMaximumPlus1)},
    ),
    (
        ParametersMain.gebrochenuniversum,
        {str(a) for a in range(2, gebrochenSpaltenMaximumPlus1)},
        set(),
        set(),
        set(),
        set(),
        set(),
        {str(a) for a in range(2, gebrochenSpaltenMaximumPlus1)},
    ),
    (
        ParametersMain.gebrochenemotion,
        {str(a) for a in range(2, gebrochenSpaltenMaximumPlus1)},
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {str(a) for a in range(2, gebrochenSpaltenMaximumPlus1)},
    ),
    (
        ParametersMain.gebrochengroesse,
        {str(a) for a in range(2, gebrochenSpaltenMaximumPlus1)},
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {str(a) for a in range(2, gebrochenSpaltenMaximumPlus1)},
    ),
    (ParametersMain.symbole, (_("Religionen"),), {36, 37}),
    (ParametersMain.symbole,         (
            _("Drei"),
        ),
        {452, 460}
    ),
    (ParametersMain.symbole,         (
            _("Vier"),
        ),
        {453,}
    ),
    (ParametersMain.symbole,         (
            _("Fünf"),
            _("Fuenf"),
        ),
        {454,}
    ),
    (ParametersMain.symbole,         (
            _("Sechs"),
        ),
        {457,}
    ),
    (ParametersMain.symbole,         (
            _("Sieben"),
        ),
        {457,}
    ),
    (ParametersMain.symbole,         (
            _("Acht"),
        ),
        {458,}
    ),
    (ParametersMain.symbole,         (
            _("Neun"),
        ),
        {459,}
    ),
    (ParametersMain.symbole,         (
            _("Zehn"),
        ),
        {456,}
    ),
    (ParametersMain.symbole,         (
            _("Zwölf"),
            _("Zwoelf"),
        ),
        {456,}
    ),

    # (
    #    ParametersMain.Multiplikationen,
    #    allowedPrimNumbersForCommand,
    #    set(),
    #    set(),
    #    (
    #        lambda: {  # nur noch ein Platzhalter
    #            None,
    #        },
    #    ),
    # ),
    (
        ParametersMain.konzept,
        (
            _("Weisheit_etc"),
            _("weisheit"),
            _("metaweisheit"),
            _("meta-weisheit"),
            _("idiot"),
            _("weise"),
            _("optimal"),
            _("optimum"),
        ),
        {112},
        {(40, 41)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Dein_Recht_bekommen"),
            _("rechte"),
            _("recht"),
            _("selbstgerecht"),
        ),
        set(),
        {(291, 292)},
    ),
    (
        ParametersMain.konzept,
        (
            _("unterlegen_überlegen"),
            _("unterlegen"),
            _("ueberlegen"),
        ),
        set(),
        {(380, 381)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Ehrlichkeit_und_Streit"),
            _("streit"),
            _("ehrlichkeit"),
        ),
        set(),
        {(375, 376)},
    ),
    (
        ParametersMain.konzept2,
        (_("Würdig"), _("wuerdig"), _("würdig")),
        set(),
        {(373, 374)},
    ),
    (
        ParametersMain.konzept2,
        (
            _("Regel_vs_Ausnahme"),
            _("regel"),
            _("ausnahme"),
        ),
        set(),
        {(371, 372)},
    ),
    (
        ParametersMain.konzept2,
        (
            _("Filterart_Widrigkeit"),
            _("filterart"),
            _("widrigkeit"),
        ),
        {331, 335},
    ),
    (
        ParametersMain.konzept2,
        (
            _("Werte"),
            _("werte"),
        ),
        set(),
        {(360, 361)},
    ),
    (
        ParametersMain.konzept2,
        (
            _("Gutartigkeits-Egoismus"),
            _("position"),
            _("gutesreziprok"),
        ),
        set(),
        {(362, 363)},
    ),
    (
        ParametersMain.konzept2,
        (
            _("Reflektieren_Erkenntnis-Erkennen"),
            _("reflektieren"),
            _("erkenntnis"),
        ),
        set(),
        {(364, 365)},
    ),
    (
        ParametersMain.konzept2,
        (
            _("Vertrauen_wollen"),
            _("vertrauenwollen"),
        ),
        set(),
        {(366, 367)},
    ),
    (
        ParametersMain.konzept,
        (
            _("einklinken_vertrauen_anprangern"),
            _("einklinken"),
            _("vertrauenerhalten"),
            _("anprangern"),
        ),
        set(),
        {(368, 369)},
    ),
    (
        ParametersMain.konzept2,
        (
            _("Ausrichten_Einrichten"),
            _("einrichten"),
            _("ausrichten"),
        ),
        set(),
        {(358, 359)},
    ),
    (
        ParametersMain.konzept2,
        (
            _("Toleranz_Respekt_Akzeptanz_Willkommen"),
            _("toleranz"),
            _("respekt"),
            _("akzeptanz"),
            _("willkommen"),
        ),
        set(),
        # {(359, 360)},
        {(62, 63)},
    ),
    (
        ParametersMain.konzept,
        (_("familiebrauchen"),),
        set(),
        {(279, 280)},
    ),
    (
        ParametersMain.konzept,
        (_("ego"), _("bescheiden")),
        set(),
        {(277, 278)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Selbstsucht_Ichsucht_etc"),
            _("selbstsucht"),
            _("ichsucht"),
        ),
        set(),
        {(274, 275)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Forschen_Erfinden_Einklinken"),
            _("wissenschaft"),
            _("forschen"),
            _("einklinken"),
            _("erfinden"),
        ),
        set(),
        {(258, 259)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Kooperation_vs_Arsch"),
            _("arschloch"),
            _("kooperation"),
            _("arsch"),
        ),
        set(),
        {(245, 246)},
    ),
    (
        ParametersMain.konzept,
        (_("Liebe_usw"), _("liebe"), _("zuneigung")),
        set(),
        {(247, 248)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Selbstlosigkeit_Ichlosigkeit_etc"),
            _("selbstlos"),
            _("ichlos"),
        ),
        set(),
        {(238, 239)},
    ),
    (
        ParametersMain.konzept,
        (
            _("variationsreich_eintönig"),
            _("eintönig"),
            _("eintoenig"),
            _("variationsreich"),
        ),
        set(),
        {(236, 237)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Zuneigung_Abneigung"),
            _("abgeneigt"),
            _("zugewandt"),
            _("reserviert"),
            _("zugeneigt"),
        ),
        set(),
        {(199, 200)},
    ),
    (
        ParametersMain.menschliches,
        (
            _("ehrlich_vs_höflich"),
            _("ehrlich"),
            _("höflich"),
            _("hoeflich"),
        ),
        set(),
        {(224, 225)},
    ),
    # (
    #    ParametersMain.konzept,
    #    (_("delegieren"), _("ansammlung")),
    #    set(),
    #    {(227, 228)},
    # ),
    (
        ParametersMain.konzept,
        (
            _("ehrlich_vs_höflich"),
            _("ehrlich"),
            _("höflich"),
            _("hoeflich"),
        ),
        set(),
        {(224, 225)},
    ),
    (
        ParametersMain.konzept,
        (_("Tragweite"), _("tragweite")),
        set(),
        {(211, 212)},
    ),
    (
        ParametersMain.konzept,
        (_("wertvoll"), _("wertlos")),
        set(),
        {(186, 187)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Götter_Propheten_Familien_Freunde"),
            _("familiaer"),
            _("goettlich"),
            _("freunde"),
            _("propheten"),
        ),
        set(),
        {(184, 185)},
    ),
    (
        ParametersMain.konzept,
        (
            _("sanft_vs_hart"),
            _("sanft"),
            _("hart"),
        ),
        set(),
        {(159, 160), (161, 162)},
    ),
    (
        ParametersMain.konzept,
        (
            _("vereinen_vs_verbinden"),
            _("vereinenverbinden"),
            _("vereinen"),
            _("verbinden"),
            _("einheit"),
            _("verbindung"),
        ),
        set(),
        {(133, 134)},
    ),
    (
        ParametersMain.konzept,
        (
            _("ähnlich"),
            _("aehnlich"),
        ),
        {220},
    ),
    (
        ParametersMain.konzept,
        (
            _("gut_böse_lieb_schlecht"),
            _("gut"),
            _("böse"),
            _("boese"),
            _("lieb"),
            _("schlecht"),
        ),
        {52, 53},
        {(38, 39)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Sinn_und_Zweck_des_Lebens"),
            _("sinn"),
            _("zweck"),
            _("bedeutung"),
        ),
        {88, 189},
        {(181, 182)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Zeit_vs_Raum"),
            _("zeit"),
            _("raum"),
            _("zeitlich"),
            _("räumlich"),
        ),
        set(),
        {(49, 50)},
    ),
    (
        ParametersMain.konzept,
        (
            _("egalitär_vs_autoritär"),
            _("egalitaerautoritaer"),
            _("egalitaer"),
            _("autoritaer"),
            _("egalitär"),
            _("autoritär"),
        ),
        set(),
        {(163, 164)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Meinungen_und_Ruf"),
            _("meinungen"),
            _("anderemenschen"),
            _("ruf"),
        ),
        set(),
        {(60, 61)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Meinungsintelligenz"),
            _("meinungsintelligenz"),
            _("ursprungsintelligenz"),
        ),
        set(),
        {(151, 152)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Sittlichkeit"),
            _("sittlichkeit"),
            _("annaehrerung"),
        ),
        set(),
        {(179, 180)},
    ),
    (
        ParametersMain.konzept,
        (_("Führung"), _("führung"), _("fuehrung")),
        set(),
        {(173, 174)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Durchleuchten"),
            _("durchleuchten"),
            _("erleuchten"),
        ),
        set(),
        {(177, 178)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Fördern_Sensiblisieren_und_Gedeihen"),
            _("foerdern"),
            _("fördern"),
            _("begrenzen"),
            _("sensibilisieren"),
            _("gedeihen"),
            _("verderben"),
        ),
        set(),
        {(175, 176)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Überheblichkeit"),
            _("überheblich"),
            _("ueberheblichkeit"),
            _("ueberheblich"),
            _("überheblichkeit"),
        ),
        set(),
        {(171, 172)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Polung_der_Liebe"),
            _("liebepolung"),
        ),
        set(),
        {(121, 122)},
    ),
    (
        ParametersMain.konzept,
        (
            _("Egoismus_vs_Altruismus"),
            _("egoismus"),
            _("altruismus"),
            _("egoist"),
            _("altruist"),
        ),
        {136},
        {(66, 67)},
    ),
    (
        ParametersMain.konzept,
        (_("kausal"), _("geltung"), _("genese")),
        set(),
        {(110, 111)},
    ),
    (
        ParametersMain.konzept,
        (_("Gleichheit"), _("gleich")),
        set(),
        {(192, 193)},
    ),
    (
        ParametersMain.konzept,
        (_("Überleben"), _("ueberleben")),
        set(),
        {(194, 195)},
    ),
    (ParametersMain.inkrementieren, set(), {43, 54, 74, 95}),
    (ParametersMain.inkrementieren, (_("um1"),), {155}),
    (ParametersMain.inkrementieren, (_("um2"),), {156}),
    (ParametersMain.inkrementieren, (_("um3"),), {157}),
    (
        ParametersMain.inkrementieren,
        (
            _("warum_Transzendentalie_=_Strukturgroesse_=_Charakter"),
            _("warumtranszendentaliezustrukturgroesseundcharakter"),
        ),
        {4, 54, 5, 165},
    ),
    (
        ParametersMain.inkrementieren,
        (
            _("warum_Transzendentalie_=_Komplexität_von_Michael_Commons"),
            _("warumtranszendentaliegleichkomplexitaet"),
        ),
        {65, 5, 166},
    ),
    (
        ParametersMain.primvielfache,
        (_("Rahmen-Bedingungen"), _("rahmen")),
        {226},
    ),
    (
        ParametersMain.primvielfache,
        (
            _("Motive_gleichförmige_Polygone"),
            _("motivgleichfoermig"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primMotivGleichf"},
    ),
    (
        ParametersMain.primvielfache,
        (
            _("Struktur_gleichförmige_Polygone"),
            _("strukturgleichfoermig"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primStrukGleichf"},
    ),
    (
        ParametersMain.primvielfache,
        (
            _("Motive_Sternpolygone"),
            _("motivstern"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primMotivStern"},
    ),
    (
        ParametersMain.primvielfache,
        (
            _("Struktur_Sternpolygone"),
            _("strukturstern"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primStrukStern"},
    ),
    (
        ParametersMain.primvielfache,
        (
            _("Motiv_Sternpolygon_gebrochen-rational"),
            _("motivgebrstern"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primMotivSternGebr"},
    ),
    (
        ParametersMain.primvielfache,
        (
            _("Struktur_Sternpolyon_gebrochen-rational"),
            _("strukgebrstern"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primStrukSternGebr"},
    ),
    (
        ParametersMain.primvielfache,
        (
            _("Motiv_gleichförmige_Polygone_gebrochen-rational"),
            _("motivgebrgleichf"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primMotivGleichfGebr"},
    ),
    (
        ParametersMain.primvielfache,
        (
            _("Struktur_gleichförmige_Polygone_gebrochen-rational"),
            _("strukgebrgleichf"),
        ),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"primStrukGleichfGebr"},
    ),
    (
        ParametersMain.primvielfache,
        (_("beschrieben"),),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        set(),
        {"PrimCSV"},
    ),
    (
        ParametersMain.kontinuum,
        ("Q", "q", _("Siebzehn"),),
        {431, 432, 433, 434, 437, 441, 442, 443, 445, 450, 467, 468, 469, 487, 488},
    ),
    (
        ParametersMain.kontinuum,
        ("i", "I", _("Neun"),),
        {517},
    ),
    (
        ParametersMain.kontinuum,
        ("G", "g", _("Sieben"),),
        {518},
    ),
    (
        ParametersMain.kontinuum,
        ("J", "j", _("Zehn"),),
        {514},
    ),
    (
        ParametersMain.kontinuum,
        ("k", "K", _("Elf"),),
        {515},
    ),
    (
        ParametersMain.kontinuum,
        ("E", "e", _("Fünf"),),
        {511},
    ),
    (
        ParametersMain.kontinuum,
        ("L", "l", _("Zwölf"),),
        {506},
    ),
    (
        ParametersMain.kontinuum,
        ("Y", "y", _("Fünfundzwanzig"),),
        {507, 510},
    ),
    (
        ParametersMain.kontinuum,
        (_("Kontinuen"), "F", "f", "Sechs"),
        {508},
    ),
    (
        ParametersMain.kontinuum,
        ("F", "f", "Sechs", _("Kontinuen")),
        {508},
    ),
    (
        ParametersMain.kontinuum,
        ("O", "o", _("Fünfzehn"),),
        {5},
    ),
    (
        ParametersMain.kontinuum,
        ("H" , "h", _("Acht"),),
        {491},
    ),
    (
        ParametersMain.kontinuum,
        ("N", "n", _("Vierzehn"),),
        {492},
    ),
    (
        ParametersMain.kontinuum,
        ("M", "m", _("Dreizehn"),),
        {493},
    ),
    (
        ParametersMain.kontinuum,
        ("T", "t", _("Zwanzig"),),
        {486},
    ),
    (
        ParametersMain.multiversum,
        ("P", "p", _("Sechszehn"),),
        {435},
    ),
    (
        ParametersMain.kontinuum,
        ("P5", "p5", _("Sechszehn->Fünf"),),
        {501},
    ),
    (
        ParametersMain.multiversum,
        ("P5", "p5", _("Sechszehn->Fünf"),),
        {501},
    ),
    (
        ParametersMain.kontinuum,
        ("P", "p", _("Sechszehn"),),
        {435},
    ),
    (
        ParametersMain.kontinuum,
        ("X", "x", _("Vierundzwanzig"),),
        {25, 55, 436, 25, 386},
    ),
    (
        ParametersMain.kontinuum,
        ("S", "s", _("Neunzehn"),),
        {504},
    ),
    (
        ParametersMain.kontinuum,
        ("R", "r", _("Achtzehn"),),
        {451, 436},
    ),
    (
        ParametersMain.kontinuum,
        ("A", "a", _("Eins") ),
        {446,},
    ),
    (
        ParametersMain.kontinuum,
        ("B", "b", _("Zwei")),
        {447,},
    ),
    (
        ParametersMain.kontinuum,
        ("C", "c", _("Drei") ),
        {448,},
    ),
    (
        ParametersMain.kontinuum,
        ("D", "d", _("Vier")),
        {449,},
    ),
]"#;
pub const PYTHON_SOURCE__WORDS_KOMBI_MATRIX_1: &str = r#"OrderedDict(
    {
        1: (
            _("Lebewesen"),
            _("tiere"),
            _("tier"),
            _("lebewesen"),
        ),
        2: (_("Berufe"), _("berufe"), _("beruf")),
        3: (
            _("Kreativität_und_Intelligenz"),
            _("kreativität"),
            _("intelligenz"),
            _("kreativitaet"),
        ),
        4: (
            _("Liebe"),
            _("liebe"),
        ),
        7: (
            _("Männer"),
            _("männer"),
            _("maenner"),
            _("frauen"),
        ),
        8: (
            _("Persönlichkeit_evolutionär_erwerben"),
            _("evolution"),
            _("erwerben"),
            _("persoenlichkeit"),
            _("persönlichkeit"),
        ),
        9: (
            _("Religion"),
            _("religion"),
            _("religionen"),
        ),
        10: (
            _("Motive_Ziele"),
            _("motivation"),
            _("ziele"),
            _("ziel"),
            _("motive"),
        ),
        12: (
            _("Emotionen"),
            _("emotionen"),
            _("gefuehle"),
            emotionWort,
            _("gefühl"),
            _("gefühle"),
        ),
        13: (
            _("Personen"),
            _("personen"),
            _("berühmtheiten"),
            _("beruehmtheiten"),
        ),
        16: (
            _("Wirtschaftssysteme"),
            _("wirtschaftssystem"),
            _("wirtschaftssysteme"),
            _("kombinierteswirtschaftssystem"),
            _("kombiniertewirtschaftssysteme"),
        ),
        17: (_("Eigentum_und_Besitz"),),
    }
)"#;
pub const PYTHON_SOURCE__WORDS_KOMBI_MATRIX_2: &str = r#"OrderedDict(
    {
        1: (
            _("Lebewesen"),
            _("tiere"),
            _("tier"),
            _("lebewesen"),
        ),
        2: (_("Berufe"), _("berufe"), _("beruf")),
        # 3: (
        #    _("Kreativität_und_Intelligenz"),
        #    _("kreativität"),
        #    _("intelligenz"),
        #    _("kreativitaet"),
        # ),
        # 4: (
        #    _("Liebe"),
        #    _("liebe"),
        # ),
        5: (
            _("Transzendentalien_Strukturalien"),
            _("transzendenz"),
            _("transzendentalien"),
            _("strukturalien"),
            _("alien"),
        ),
        6: (
            _("Primzahlkreuz"),
            _("leibnitz"),
            _("primzahlkreuz"),
        ),
        # 7: (
        #    _("Männer"),
        #    _("männer"),
        #    _("maenner"),
        #    _("frauen"),
        # ),
        8: (
            _("Persönlichkeit_evolutionär_erwerben"),
            _("evolution"),
            _("erwerben"),
            _("persoenlichkeit"),
            _("persönlichkeit"),
        ),
        9: (
            _("Religion"),
            _("religion"),
            _("religionen"),
        ),
        10: (
            _("Motive_Ziele"),
            _("motivation"),
            _("motive"),
            _("ziele"),
            _("ziel"),
        ),
        11: (
            _("analytische_Ontologie"),
            _("analytischeontologie"),
            _("ontologie"),
        ),
        # 12: (
        #    _("Emotionen"),
        #    _("emotionen"),
        #    _("gefuehle"),
        #    _("gefühle"),
        #    emotionWort,
        #    _("gefühl"),
        #    _("gefühle"),
        # ),
        13: (_("Personen"), _("personen"), _("berühmtheiten"), _("beruehmtheiten")),
        14: (
            _("Mechanismen_der_Zuechtung"),
            _("mechanismen"),
            _("wesen"),
            _("zuechten"),
            _("züchten"),
        ),
        15: (
            _("Gegentranszendentalien"),
            _("gegentranszendentalien"),
            _("gegenstrukturalien"),
        ),
        # 16: (
        #    _("Wirtschaftssysteme"),
        #    _("wirtschaftssystem"),
        #    _("wirtschaftssysteme"),
        #    _("kombinierteswirtschaftssystem"),
        #    _("kombiniertewirtschaftssysteme"),
        # ),
        17: (
            _("Maschinen"),
            _("maschinen"),
            _("geräte"),
            _("geraete"),
        ),
        18: (_("Geist"), geistWort),
        19: (_("Bewusstsein"), _("bewusstsein")),
    }
)"#;
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
pub const PYTHON_SOURCE__LIBRETAPROMPT: &str = r#"import os
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
"#;
