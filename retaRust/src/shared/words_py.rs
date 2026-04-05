use indexmap::IndexMap;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PyValue {
    Int(i64),
    Str(String),
    Bool(bool),
    Tuple(Vec<PyValue>),
    NoneValue,
}

#[derive(Clone, Debug)]
pub struct StoreParameterEntry {
    pub parameterMainNames: Vec<String>,
    pub parameterNames: Vec<String>,
    pub datas: Vec<Vec<PyValue>>,
}

#[derive(Clone, Debug)]
pub struct Words {
    pub paraNdataMatrix: Vec<StoreParameterEntry>,
    pub kombiParaNdataMatrix: IndexMap<i64, Vec<String>>,
    pub kombiParaNdataMatrix2: IndexMap<i64, Vec<String>>,
}

impl Words {
    pub fn new() -> Self {
        let mut paraNdataMatrix: Vec<StoreParameterEntry> = vec![];
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Wichtigste".to_string(), "wichtigste".to_string()],
    datas: vec![
        vec![PyValue::Int(10), PyValue::Int(4), PyValue::Int(5), PyValue::Int(8)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Mensch-zu-Tier".to_string(), "menschtier".to_string(), "tiermensch".to_string()],
    datas: vec![
        vec![PyValue::Int(314)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Superkräfte".to_string(), "Superkraefte".to_string()],
    datas: vec![
        vec![PyValue::Int(444), PyValue::Int(494), PyValue::Int(496), PyValue::Int(503)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Superkräfte".to_string(), "Superkraefte".to_string()],
    datas: vec![
        vec![PyValue::Int(444), PyValue::Int(494), PyValue::Int(496)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Evolution_vs_Design_intelligent".to_string()],
    datas: vec![
        vec![PyValue::Int(519)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Evolution_vs_Design_intelligent".to_string()],
    datas: vec![
        vec![PyValue::Int(519)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Superkräfte".to_string(), "Superkraefte".to_string()],
    datas: vec![
        vec![PyValue::Int(444), PyValue::Int(494), PyValue::Int(496)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Formationen".to_string()],
    datas: vec![
        vec![PyValue::Int(461)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Ansichten_Standpunkte_(18_17)".to_string(), "ansichten".to_string()],
    datas: vec![
        vec![PyValue::Int(240), PyValue::Int(346)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["(politische)_Richtungen_(7)".to_string(), "richtungen".to_string(), "politische".to_string()],
    datas: vec![
        vec![PyValue::Int(235)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Wirklichkeiten_(10)".to_string(), "wirklichkeit".to_string(), "wirklichkeiten".to_string()],
    datas: vec![
        vec![PyValue::Int(233), PyValue::Int(265), PyValue::Int(268), PyValue::Int(322), PyValue::Int(420)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Meta-Systeme_(12)".to_string(), "metasysteme".to_string(), "metasystem".to_string(), "meta-systeme".to_string(), "meta-system".to_string()],
    datas: vec![
        vec![PyValue::Int(232), PyValue::Int(288), PyValue::Int(334), PyValue::Int(410), PyValue::Int(411), PyValue::Int(483), PyValue::Int(497), PyValue::Int(498), PyValue::Int(499), PyValue::Int(79), PyValue::Int(80)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Intelligenz".to_string(), "intelligenz".to_string()],
    datas: vec![
        vec![PyValue::Int(214)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Gleichheit_Freiheit_Ordnung".to_string(), "gleichheit".to_string(), "freiheit".to_string(), "gleichheit".to_string()],
    datas: vec![
        vec![PyValue::Int(132), PyValue::Int(324), PyValue::Int(328), PyValue::Int(331), PyValue::Int(335), PyValue::Int(497), PyValue::Int(498), PyValue::Int(499), PyValue::Int(79), PyValue::Int(80)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Komplexität".to_string(), "komplexität".to_string(), "komplexitaet".to_string()],
    datas: vec![
        vec![PyValue::Int(213)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Mechanismen".to_string(), "mechanismen".to_string(), "mechanismus".to_string()],
    datas: vec![
        vec![PyValue::Int(107)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Zweitwichtigste".to_string(), "zweitwichtigste".to_string()],
    datas: vec![
        vec![PyValue::Int(183), PyValue::Int(19), PyValue::Int(65)],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(10)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Drittwichtigste".to_string(), "drittwichtigste".to_string()],
    datas: vec![
        vec![PyValue::Int(64)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Motive_Sternpolygone".to_string(), "viertwichtigste".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primMotivStern".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste2']".to_string()],
    parameterNames: vec!["Wichtigste".to_string(), "wichtigstes".to_string()],
    datas: vec![
        vec![PyValue::Int(0), PyValue::Int(1), PyValue::Int(2), PyValue::Int(207), PyValue::Int(36), PyValue::Int(37)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste2']".to_string()],
    parameterNames: vec!["Zweitwichtigste".to_string(), "zweitwichtigste".to_string()],
    datas: vec![
        vec![PyValue::Int(30)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["Halbierung".to_string(), "halbierung".to_string(), "halbierungen".to_string()],
    datas: vec![
        vec![PyValue::Int(86)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Religions-Gründer-Typ".to_string(), "religionsgründertyp".to_string(), "prophet".to_string(), "archon".to_string(), "religionsgruendertyp".to_string()],
    datas: vec![
        vec![PyValue::Int(503), PyValue::Int(72)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Satan_Teufel".to_string()],
    datas: vec![
        vec![PyValue::Int(495)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Satan_Teufel".to_string()],
    datas: vec![
        vec![PyValue::Int(495)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Hinduismus".to_string(), "hinduismus".to_string()],
    datas: vec![
        vec![PyValue::Int(217)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Sternpolygon".to_string(), "sternpolygon".to_string()],
    datas: vec![
        vec![PyValue::Int(0), PyValue::Int(36), PyValue::Int(6)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["der_Tierkreiszeichen".to_string(), "dertierkreiszeichen".to_string(), "babylon".to_string()],
    datas: vec![
        vec![PyValue::Int(0), PyValue::Int(207), PyValue::Int(36), PyValue::Int(477), PyValue::Int(478)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Sternpolygon_vs_gleichförmiges".to_string(), "vergleich".to_string(), "sternpolygonvsgleichfoermiges".to_string(), "vergleichnvs1divn".to_string()],
    datas: vec![
        vec![PyValue::Int(87)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Messias".to_string(), "messias".to_string(), "heptagramm".to_string(), "hund".to_string(), "messiase".to_string(), "messiasse".to_string()],
    datas: vec![
        vec![PyValue::Int(503), PyValue::Int(7)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["gleichförmiges_Polygon".to_string(), "gleichförmigespolygon".to_string(), "gleichfoermigespolygon".to_string(), "nichtsternpolygon".to_string(), "polygon".to_string()],
    datas: vec![
        vec![PyValue::Int(16), PyValue::Int(37)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Vertreter_höherer_Konzepte".to_string(), "vertreterhoehererkonzepte".to_string(), "galaxien".to_string(), "galaxie".to_string(), "schwarzesonne".to_string(), "schwarzesonnen".to_string(), "universum".to_string(), "universen".to_string(), "kreis".to_string(), "kreise".to_string(), "kugel".to_string(), "kugeln".to_string()],
    datas: vec![
        vec![PyValue::Int(23)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Lebewesen_Galaxie_am_Besten".to_string()],
    datas: vec![
        vec![PyValue::Int(470), PyValue::Int(471), PyValue::Int(473)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Offenbarung_des_Johannes".to_string(), "offenbarung".to_string(), "offenbarungdesjohannes".to_string(), "johannes".to_string(), "bibel".to_string(), "offenbarungjohannes".to_string()],
    datas: vec![
        vec![PyValue::Int(90)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["Teilchen-Meta-Physik".to_string(), "addition".to_string(), "identitaet".to_string(), "Identität".to_string()],
    datas: vec![
        vec![PyValue::Int(219), PyValue::Int(223), PyValue::Int(307), PyValue::Int(308), PyValue::Int(333), PyValue::Int(387), PyValue::Int(388), PyValue::Int(406)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Hochzüchten".to_string(), "hochzüchten".to_string(), "hochzuechten".to_string()],
    datas: vec![
        vec![PyValue::Int(318), PyValue::Int(319)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Teilchen_anderes_Universum".to_string()],
    datas: vec![
        vec![PyValue::Int(512)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Teilchen_anderes_Universum".to_string()],
    datas: vec![
        vec![PyValue::Int(512)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Teilchen_anderes_Universum".to_string()],
    datas: vec![
        vec![PyValue::Int(512)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Zusammenhang_Gehirn_Kosmos_Universum".to_string()],
    datas: vec![
        vec![PyValue::Int(489)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Zahlenarten".to_string()],
    datas: vec![
        vec![PyValue::Int(462)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Bestrafung".to_string()],
    datas: vec![
        vec![PyValue::Int(463)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bestrafung".to_string()],
    datas: vec![
        vec![PyValue::Int(463)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["weniger_am_Menschen".to_string()],
    datas: vec![
        vec![PyValue::Int(464)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Erlösung".to_string(), "Erloesung".to_string()],
    datas: vec![
        vec![PyValue::Int(465)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Erlösung".to_string(), "Erloesung".to_string()],
    datas: vec![
        vec![PyValue::Int(465)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gewalt".to_string()],
    datas: vec![
        vec![PyValue::Int(466)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Gewalt".to_string()],
    datas: vec![
        vec![PyValue::Int(466), PyValue::Int(479)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Farben".to_string()],
    datas: vec![
        vec![PyValue::Int(444)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["künstliches_Leben_(15)".to_string(), "künstlichesleben".to_string(), "grosseki".to_string()],
    datas: vec![
        vec![PyValue::Int(409)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Software-Lizenzen_akademische_Grade".to_string(), "softwarelizenz".to_string(), "akademischeGrade".to_string()],
    datas: vec![
        vec![PyValue::Int(422)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Strategie_Taktik_(15m8)".to_string(), "strategie".to_string(), "taktik".to_string()],
    datas: vec![
        vec![PyValue::Int(385)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Universelles_Verhältnis_gleicher_Zahlen".to_string(), "verhaeltnisgleicherzahl".to_string()],
    datas: vec![
        vec![PyValue::Int(383)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["universelles_Recht".to_string(), "recht".to_string(), "jura".to_string()],
    datas: vec![
        vec![PyValue::Int(34), PyValue::Int(382), PyValue::Int(65)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["sowas_wie_Kombinieren_Verknüpfen".to_string(), "kombinierenetc".to_string()],
    datas: vec![
        vec![PyValue::Int(320)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Hochzüchten".to_string(), "hochzüchten".to_string(), "hochzuechten".to_string()],
    datas: vec![
        vec![PyValue::Int(318), PyValue::Int(319)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Teilchen-Meta-Physik".to_string()],
    datas: vec![
        vec![PyValue::Int(219), PyValue::Int(308)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Universelle_(15)".to_string()],
    datas: vec![
        vec![PyValue::Int(219), PyValue::Int(308)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["Wirklichkeiten_(10)".to_string(), "wirklichkeit".to_string(), "wirklichkeiten".to_string()],
    datas: vec![
        vec![PyValue::Int(420)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Galaktische_(14)".to_string()],
    datas: vec![
        vec![PyValue::Int(406)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Multiverselle_(16)".to_string()],
    datas: vec![
        vec![PyValue::Int(388), PyValue::Int(418)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["die_Tugendsortierung_(13_mit_14)".to_string()],
    datas: vec![
        vec![PyValue::Int(411)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["die_Galaxie_Unterbereiche_(13)".to_string()],
    datas: vec![
        vec![PyValue::Int(223), PyValue::Int(307), PyValue::Int(412)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Gute_die_Richtung_(7)".to_string()],
    datas: vec![
        vec![PyValue::Int(333)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["Raum_und_Dimensionen_(8)".to_string()],
    datas: vec![
        vec![PyValue::Int(387)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["keine_Nur-Paradigma-Religionen".to_string(), "metaparadigmareligion".to_string()],
    datas: vec![
        vec![PyValue::Int(190), PyValue::Int(191), PyValue::Int(196)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Kugeln_Kreise".to_string(), "kugelnkreise".to_string(), "kugeln".to_string(), "kreise".to_string()],
    datas: vec![
        vec![PyValue::Int(145), PyValue::Int(77)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Raumzeit_Anordnung_mathematisch_universell".to_string()],
    datas: vec![
        vec![PyValue::Int(472)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Multiversalien_(16)".to_string(), "multiversalien".to_string()],
    datas: vec![
        vec![PyValue::Int(389)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Meta-Physik-Teilchen_(1)".to_string(), "teilchen".to_string()],
    datas: vec![
        vec![PyValue::Int(388)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Kugeln_Kreise".to_string(), "kugelnkreise".to_string(), "kugeln".to_string(), "kreise".to_string()],
    datas: vec![
        vec![PyValue::Int(145), PyValue::Int(77)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["chinesisches_Horoskop".to_string(), "chinesischeshoroskop".to_string(), "china".to_string()],
    datas: vec![
        vec![PyValue::Int(91)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["babylonische_Tierkreiszeichen".to_string(), "tierkreiszeichen".to_string(), "babylon".to_string()],
    datas: vec![
        vec![PyValue::Int(1), PyValue::Int(2)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Thomasevangelium".to_string(), "thomasevangelium".to_string(), "thomas".to_string()],
    datas: vec![
        vec![PyValue::Int(0), PyValue::Int(3), PyValue::Int(303)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Netzwerk".to_string(), "netzwerk".to_string()],
    datas: vec![
        vec![PyValue::Int(417), PyValue::Int(436)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Kontroverse_(51)".to_string(), "kontroverse".to_string()],
    datas: vec![
        vec![PyValue::Int(421)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["mathematisches_Design_(32)".to_string(), "mathematischesdesign".to_string()],
    datas: vec![
        vec![PyValue::Int(419)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()],
    datas: vec![
        vec![PyValue::Int(84)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()],
    datas: vec![
        vec![PyValue::Int(84)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Transzendentalien_innen_außen".to_string(), "innenaussenstrukur".to_string(), "strukturalieninnenaußen".to_string(), "strukturalieninnenaussen".to_string(), "innenaußenstrukur".to_string(), "transzendentalieninnenaußen".to_string(), "transzendentalieninnenaussen".to_string()],
    datas: vec![
        vec![PyValue::Int(149)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Modallogik".to_string(), "modallogik".to_string()],
    datas: vec![
        vec![PyValue::Int(148)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["5".to_string(), "fünf".to_string(), "fünfer".to_string(), "fünferstruktur".to_string(), "fuenf".to_string(), "fuenfer".to_string(), "fuenferstruktur".to_string()],
    datas: vec![
        vec![PyValue::Int(96)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["9".to_string(), "neun".to_string(), "neuner".to_string(), "neunerstruktur".to_string()],
    datas: vec![
        vec![PyValue::Int(94)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["3".to_string(), "drei".to_string(), "dreier".to_string(), "dreierstruktur".to_string()],
    datas: vec![
        vec![PyValue::Int(315), PyValue::Int(316), PyValue::Int(92), PyValue::Int(93)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["Licht".to_string(), "licht".to_string()],
    datas: vec![
        vec![PyValue::Int(20), PyValue::Int(27), PyValue::Int(313)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Strukturgrösse".to_string(), "strukturgroesse".to_string(), "größe".to_string(), "groesse".to_string(), "gross".to_string(), "strukturgroesse".to_string(), "strukturgroeße".to_string(), "strukturgrösse".to_string(), "strukturgröße".to_string()],
    datas: vec![
        vec![PyValue::Int(197), PyValue::Int(21), PyValue::Int(4), PyValue::Int(425), PyValue::Int(54)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["Strukturgrösse".to_string(), "strukturgroesse".to_string(), "größe".to_string(), "groesse".to_string(), "gross".to_string(), "strukturgroesse".to_string(), "strukturgroeße".to_string(), "strukturgrösse".to_string(), "strukturgröße".to_string()],
    datas: vec![
        vec![PyValue::Int(197), PyValue::Int(21), PyValue::Int(4), PyValue::Int(425), PyValue::Int(54)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["Organisationen".to_string(), "organisationen".to_string(), "organisation".to_string()],
    datas: vec![
        vec![PyValue::Int(30), PyValue::Int(425), PyValue::Int(82)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["politische_Systeme".to_string(), "politischesysteme".to_string(), "politik".to_string()],
    datas: vec![
        vec![PyValue::Int(83)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["meta".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(2), PyValue::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["konkret".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(2), PyValue::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Theorie".to_string(), "theorie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(3), PyValue::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Praxis".to_string(), "praxis".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(3), PyValue::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Management".to_string(), "management".to_string(), "stau".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(4), PyValue::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["verändernd".to_string(), "veraendernd".to_string(), "fluss".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(4), PyValue::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["ganzheitlich".to_string(), "mathematisch_diskret".to_string(), "diskret".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(5), PyValue::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["darüber_hinausgehend".to_string(), "hinausgehend".to_string(), "kontinuierlich".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(5), PyValue::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Universum_Strukturalien_Transzendentalien".to_string(), "universum".to_string(), "strukturalie".to_string(), "strukturalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(5)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Richtung_als_Richtung".to_string(), "richtungrichtung".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::NoneValue])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Galaxieabsicht".to_string(), "absichtgalaxie".to_string(), "absicht".to_string(), "motive".to_string(), "motiv".to_string(), "absichten".to_string(), "galaxie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(10)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Absicht_Reziproke_Galaxie".to_string(), "absichtgalaxiereziproke".to_string(), "absichtreziproke".to_string(), "motivereziproke".to_string(), "motivreziproke".to_string(), "absichtenreziproke".to_string(), "galaxiereziproke".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(42)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Universum_Reziproke".to_string(), "universumreziproke".to_string(), "strukturaliereziproke".to_string(), "strukturalienreziproke".to_string(), "transzendentalienreziproke".to_string(), "transzendentaliereziproke".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(131)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Dagegen-Gegentranszendentalie".to_string(), "dagegengegentranszendentalie".to_string(), "dagegengegentranszendentalien".to_string(), "dagegengegenstrukturalien".to_string(), "dagegengegenstrukturalie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(138)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["neutrale_Gegentranszendentalie".to_string(), "neutralegegentranszendentalie".to_string(), "neutralegegentranszendentalien".to_string(), "neutralegegenstrukturalien".to_string(), "neutralegegenstrukturalie".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(202)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Unternehmung_Geschäft".to_string(), "unternehmen".to_string(), "unternehmung".to_string(), "geschaeft".to_string(), "geschäft".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(6), PyValue::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["wertvoll".to_string(), "wert".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(6), PyValue::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Beherrschen".to_string(), "regieren".to_string(), "beherrschen".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(7), PyValue::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Richtung".to_string(), "richtung".to_string(), "gut".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(7), PyValue::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()],
    datas: vec![
        vec![PyValue::Int(84)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Gegentranszendentalien".to_string(), "gegentranszendentalien".to_string(), "gegentranszendentalie".to_string(), "gegenstrukturalien".to_string(), "gegenalien".to_string(), "gegenuniversalien".to_string()],
    datas: vec![
        vec![PyValue::Int(138), PyValue::Int(202)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Systemsachen".to_string(), "systemsachen".to_string()],
    datas: vec![
        vec![PyValue::Int(150)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Transzendentalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string(), "strukturalien".to_string(), "alien".to_string(), "universalien".to_string()],
    datas: vec![
        vec![PyValue::Int(198), PyValue::Int(390), PyValue::Int(5), PyValue::Int(54), PyValue::Int(55)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Reziproke_von_Transzendentalien".to_string(), "transzendentalienreziproke".to_string(), "transzendentaliereziproke".to_string(), "strukturalienreziproke".to_string(), "alienreziproke".to_string(), "universalienreziproke".to_string()],
    datas: vec![
        vec![PyValue::Int(131), PyValue::Int(201)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Netzwerk".to_string(), "netzwerk".to_string()],
    datas: vec![
        vec![PyValue::Int(25), PyValue::Int(386), PyValue::Int(390), PyValue::Int(55)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["warum_Transzendentalie_=_Strukturgroesse_=_Charakter".to_string(), "warumtranszendentaliezustrukturgroesseundcharakter".to_string()],
    datas: vec![
        vec![PyValue::Int(165), PyValue::Int(4), PyValue::Int(5), PyValue::Int(54)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Kategorie".to_string(), "kategorie".to_string()],
    datas: vec![
        vec![PyValue::Int(204), PyValue::Int(205), PyValue::Int(281)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Raum-Missionen".to_string(), "weltall".to_string()],
    datas: vec![
        vec![PyValue::Int(218)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Programmier-Paradigmen".to_string(), "programmierparadigmen".to_string()],
    datas: vec![
        vec![PyValue::Int(351)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Raum-Missionen".to_string(), "weltall".to_string()],
    datas: vec![
        vec![PyValue::Int(218)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Geist__(15)".to_string(), "geist".to_string()],
    datas: vec![
        vec![PyValue::Int(242), PyValue::Int(426)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["warum_Transzendentalie_=_Komplexität_von_Michael_Commons".to_string(), "warumtranszendentaliegleichkomplexitaet".to_string()],
    datas: vec![
        vec![PyValue::Int(166), PyValue::Int(5), PyValue::Int(65)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Model_of_Hierarchical_Complexity".to_string(), "modelofhierarchicalcomplexity".to_string(), "komplex".to_string(), "komplexität".to_string(), "komplexitaet".to_string(), "complexity".to_string(), "model".to_string(), "abstraktion".to_string()],
    datas: vec![
        vec![PyValue::Int(203), PyValue::Int(483), PyValue::Int(65), PyValue::Int(75)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Model_of_Hierarchical_Complexity".to_string(), "modelofhierarchicalcomplexity".to_string(), "komplex".to_string(), "komplexität".to_string(), "komplexitaet".to_string(), "complexity".to_string(), "model".to_string(), "abstraktion".to_string()],
    datas: vec![
        vec![PyValue::Int(203), PyValue::Int(65), PyValue::Int(75)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Model_of_Hierarchical_Complexity".to_string(), "modelofhierarchicalcomplexity".to_string(), "komplex".to_string(), "komplexität".to_string(), "komplexitaet".to_string(), "complexity".to_string(), "model".to_string(), "abstraktion".to_string()],
    datas: vec![
        vec![PyValue::Int(203), PyValue::Int(65), PyValue::Int(75)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["2".to_string(), "zwei".to_string(), "gerade".to_string(), "ungerade".to_string(), "alternierung".to_string(), "alternierend".to_string(), "zweierstruktur".to_string()],
    datas: vec![
        vec![PyValue::Int(331), PyValue::Int(497), PyValue::Int(498), PyValue::Int(499), PyValue::Int(78), PyValue::Int(79), PyValue::Int(80)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["Multiplikation".to_string(), "multiplikation".to_string()],
    datas: vec![
        vec![PyValue::Int(158)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["4".to_string(), "vier".to_string(), "viererstruktur".to_string(), "viererabfolgen".to_string()],
    datas: vec![
        vec![PyValue::Int(104), PyValue::Int(145), PyValue::Int(76), PyValue::Int(77), PyValue::Int(81)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gesellschaftsschicht".to_string(), "klasse".to_string(), "klassen".to_string()],
    datas: vec![
        vec![PyValue::Int(241)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Moral".to_string(), "moral".to_string(), "warummoral".to_string()],
    datas: vec![
        vec![PyValue::Int(215), PyValue::Int(216)],\n        vec![PyValue::Tuple(vec![PyValue::Int(216), PyValue::Int(221)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Fachgebiete".to_string(), "fachgebiete".to_string(), "fachbereiche".to_string(), "themen".to_string()],
    datas: vec![
        vec![PyValue::Int(183)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Fachgebiete".to_string(), "fachgebiete".to_string(), "fachbereiche".to_string(), "themen".to_string()],
    datas: vec![
        vec![PyValue::Int(183)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Pflanzen".to_string(), "pflanzen".to_string()],
    datas: vec![
        vec![PyValue::Int(113)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Maschinen".to_string(), "maschinen".to_string(), "maschine".to_string(), "gerät".to_string(), "geräte".to_string(), "geraete".to_string(), "geraet".to_string()],
    datas: vec![
        vec![PyValue::Int(89)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Organisationsform".to_string(), "organisationsform".to_string(), "organisationsart".to_string(), "firma".to_string(), "verein".to_string()],
    datas: vec![
        vec![PyValue::Int(99)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["System".to_string(), "system".to_string()],
    datas: vec![
        vec![PyValue::Int(440), PyValue::Int(455), PyValue::Int(476), PyValue::Int(513), PyValue::Int(69), PyValue::Int(70)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["System".to_string(), "system".to_string()],
    datas: vec![
        vec![PyValue::Int(440), PyValue::Int(455), PyValue::Int(476), PyValue::Int(513), PyValue::Int(69), PyValue::Int(70)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Erklärung".to_string(), "erklärung".to_string(), "erklaerung".to_string()],
    datas: vec![
        vec![PyValue::Int(71)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["BWL".to_string(), "bwl".to_string()],
    datas: vec![
        vec![PyValue::Int(109)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Sinn_des_Lebens".to_string(), "sinndeslebens".to_string(), "lebenssinn".to_string(), "sinn".to_string(), "sinnsuche".to_string()],
    datas: vec![
        vec![PyValue::Int(189), PyValue::Int(88)],\n        vec![PyValue::Tuple(vec![PyValue::Int(181), PyValue::Int(182)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Intelligenzprobleme".to_string(), "intelligenzprobleme".to_string(), "intelligenzmaengel".to_string(), "intelligenzmängel".to_string()],
    datas: vec![
        vec![PyValue::Int(147)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Denkweise_von_Lebewesen".to_string(), "lebewesendenkweise".to_string(), "denkweise".to_string()],
    datas: vec![
        vec![PyValue::Int(146)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gegentranszendentalien".to_string(), "gegentranszendentalien".to_string(), "gegenstrukturalien".to_string()],
    datas: vec![
        vec![PyValue::Int(138), PyValue::Int(139), PyValue::Int(202)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gleichheit_Freiheit".to_string(), "gleichheitfreiheit".to_string(), "ungleichheit".to_string(), "dominieren".to_string(), "gleichheit".to_string(), "freiheit".to_string()],
    datas: vec![
        vec![PyValue::Int(132), PyValue::Int(328), PyValue::Int(331), PyValue::Int(335)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gefühle".to_string(), "emotionen".to_string(), "gefuehle".to_string(), "emotion".to_string(), "gefühl".to_string(), "gefuehl".to_string()],
    datas: vec![
        vec![PyValue::Int(105), PyValue::Int(230), PyValue::Int(243), PyValue::Int(283), PyValue::Int(284), PyValue::Int(285), PyValue::Int(286), PyValue::Int(305)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Egoismus".to_string(), "egoismus".to_string(), "altruismus".to_string(), "selbstlosigkeit".to_string()],
    datas: vec![
        vec![PyValue::Int(136)],\n        vec![PyValue::Tuple(vec![PyValue::Int(66), PyValue::Int(67)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Wirkung".to_string(), "wirkung".to_string()],
    datas: vec![
        vec![PyValue::Int(135)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["INCELs".to_string(), "incel".to_string(), "incels".to_string()],
    datas: vec![
        vec![PyValue::Int(68)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["irrationale_Zahlen_durch_Wurzelbildung".to_string(), "irrationalezahlendurchwurzelbildung".to_string(), "ausgangslage".to_string()],
    datas: vec![
        vec![PyValue::Int(73)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["dominierendes_Geschlecht".to_string(), "dominierendesgeschlecht".to_string(), "maennlich".to_string(), "männlich".to_string(), "weiblich".to_string()],
    datas: vec![
        vec![PyValue::Int(51)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Liebe".to_string(), "liebe".to_string(), "ethik".to_string()],
    datas: vec![
        vec![PyValue::Int(208), PyValue::Int(28), PyValue::Int(330), PyValue::Int(8), PyValue::Int(9)],\n        vec![PyValue::Tuple(vec![PyValue::Int(121), PyValue::Int(122)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Glaube_Erkenntnis".to_string(), "glauben".to_string(), "erkenntnis".to_string(), "glaube".to_string()],
    datas: vec![
        vec![PyValue::Int(59)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Angreifbarkeit".to_string(), "angreifbarkeit".to_string(), "angreifbar".to_string()],
    datas: vec![
        vec![PyValue::Int(57), PyValue::Int(58)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)".to_string(), "Transzendentalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string(), "strukturalien".to_string(), "alien".to_string(), "universalien".to_string(), "meta-paradigmen".to_string()],
    datas: vec![
        vec![PyValue::Int(131), PyValue::Int(229), PyValue::Int(5)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)".to_string(), "Transzendentalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string(), "strukturalien".to_string(), "alien".to_string(), "universalien".to_string(), "meta-paradigmen".to_string()],
    datas: vec![
        vec![PyValue::Int(131), PyValue::Int(229), PyValue::Int(5)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bedingung_und_Auslöser_(1/3)".to_string(), "bedingung".to_string(), "bedingungen".to_string(), "auslöser".to_string(), "ausloeser".to_string()],
    datas: vec![
        vec![PyValue::Int(338)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Lebensbereiche_Problemklassen_(28)".to_string(), "lebensbereiche".to_string(), "lebensfelder".to_string(), "problemklassen".to_string()],
    datas: vec![
        vec![PyValue::Int(405), PyValue::Int(415), PyValue::Int(416)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Maßnahmen_(39)".to_string(), "massnahmen".to_string()],
    datas: vec![
        vec![PyValue::Int(384)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)".to_string(), "relativreziprokuniversell".to_string()],
    datas: vec![
        vec![PyValue::Int(350)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["universeller_Komperativ_(18→15)".to_string(), "universellerkomperativ".to_string()],
    datas: vec![
        vec![PyValue::Int(349)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Existenzialien_(3)".to_string(), "existenzialien".to_string()],
    datas: vec![
        vec![PyValue::Int(348)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Extremalien_(19)".to_string(), "extremalien".to_string()],
    datas: vec![
        vec![PyValue::Int(347), PyValue::Int(352)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Erwartungshaltungen_(26)".to_string(), "erwartungen".to_string(), "erwartungshaltungen".to_string()],
    datas: vec![
        vec![PyValue::Int(344)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Leidenschaften_(21)".to_string(), "leidenschaft".to_string(), "leidenschaften".to_string()],
    datas: vec![
        vec![PyValue::Int(343)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["relativer_Zeit-Betrag_(15_10_4_18_6)".to_string(), "relativerzeitbetrag".to_string()],
    datas: vec![
        vec![PyValue::Int(339)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Zahlenvergleich_(15_18_6)".to_string(), "zahlenvergleich".to_string()],
    datas: vec![
        vec![PyValue::Int(340)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bestrebungen(1/5)".to_string(), "bestrebung".to_string(), "bestrebungen".to_string()],
    datas: vec![
        vec![PyValue::Int(332), PyValue::Int(414)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Prinzipien(1/8)".to_string(), "prinzipien".to_string()],
    datas: vec![
        vec![PyValue::Int(329), PyValue::Int(378)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Attraktionen_(36)".to_string(), "attraktionen".to_string()],
    datas: vec![
        vec![PyValue::Int(311)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Optimierung_(10)".to_string(), "optimierung".to_string()],
    datas: vec![
        vec![PyValue::Int(310)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Themen_(6)".to_string(), "themen".to_string(), "thema".to_string()],
    datas: vec![
        vec![PyValue::Int(309)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bedeutung_(10)".to_string(), "bedeutung".to_string()],
    datas: vec![
        vec![PyValue::Int(306)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Reziprokes".to_string(), "reziproke".to_string(), "reziprokes".to_string()],
    datas: vec![
        vec![PyValue::Int(131), PyValue::Int(204), PyValue::Int(205), PyValue::Int(231), PyValue::Int(257), PyValue::Int(273), PyValue::Int(281), PyValue::Int(284), PyValue::Int(285), PyValue::Int(326), PyValue::Int(327), PyValue::Int(328), PyValue::Int(329), PyValue::Int(330), PyValue::Int(331), PyValue::Int(332), PyValue::Int(334), PyValue::Int(335), PyValue::Int(338), PyValue::Int(416), PyValue::Int(42)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Achtung_(4)".to_string(), "achtung".to_string(), "achten".to_string()],
    datas: vec![
        vec![PyValue::Int(270), PyValue::Int(393)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Zeit_(4)_als_Wirklichkeit".to_string(), "zeit".to_string()],
    datas: vec![
        vec![PyValue::Int(266), PyValue::Int(267)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_16_ist_zu_genügen".to_string(), "absicht16".to_string()],
    datas: vec![
        vec![PyValue::Int(312)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_17_ist_zu_meinen".to_string(), "absicht17".to_string()],
    datas: vec![
        vec![PyValue::Int(263)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_6_ist_Vorteilsmaximierung".to_string(), "absicht6".to_string()],
    datas: vec![
        vec![PyValue::Int(262)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_7_ist_Selbstlosigkeit".to_string(), "absicht7".to_string()],
    datas: vec![
        vec![PyValue::Int(261)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bewusstheit_statt_Bewusstsein_(1)".to_string(), "bewusstheit".to_string()],
    datas: vec![
        vec![PyValue::Int(282)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Verhalten_(11)".to_string(), "verhalten".to_string()],
    datas: vec![
        vec![PyValue::Int(301), PyValue::Int(302), PyValue::Int(413)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Energie_und_universelle_Eigenschaften_(30)".to_string(), "energie".to_string(), "universelleeigenschaften".to_string(), "lebensenergie".to_string()],
    datas: vec![
        vec![PyValue::Int(287), PyValue::Int(293)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Garben_und_Verhalten_nachfühlen(31)".to_string(), "garben".to_string(), "verhaltenfuehlen".to_string(), "verhaltenfühlen".to_string()],
    datas: vec![
        vec![PyValue::Int(295)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)".to_string(), "nachvollziehen".to_string()],
    datas: vec![
        vec![PyValue::Int(242), PyValue::Int(297)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primzahlkreuzprocontra".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Empathie_(37)".to_string(), "empathie".to_string(), "mitgefuehl".to_string()],
    datas: vec![
        vec![PyValue::Int(294)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_1/6_ist_Reinigung_und_Klarheit".to_string(), "absicht1/6".to_string(), "absicht1pro6".to_string()],
    datas: vec![
        vec![PyValue::Int(298)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["innere_Werte_1/6_der_Reinigung_und_Klarheit".to_string(), "innerewerte".to_string()],
    datas: vec![
        vec![PyValue::Int(398), PyValue::Int(399), PyValue::Int(400), PyValue::Int(401)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_10_ist_Wirklichkeit_erkennen".to_string(), "absicht10".to_string()],
    datas: vec![
        vec![PyValue::Int(260)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Wohlbefinden_(7mit6)".to_string(), "wohlbefinden".to_string()],
    datas: vec![
        vec![PyValue::Int(427), PyValue::Int(428)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Geist_(15)".to_string(), "geist".to_string(), "bewusstsein".to_string()],
    datas: vec![
        vec![PyValue::Int(229), PyValue::Int(231), PyValue::Int(242), PyValue::Int(273), PyValue::Int(297), PyValue::Int(304), PyValue::Int(426)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Geist_(15)".to_string(), "geist".to_string(), "bewusstsein".to_string()],
    datas: vec![
        vec![PyValue::Int(229), PyValue::Int(231), PyValue::Int(242), PyValue::Int(273), PyValue::Int(297), PyValue::Int(304), PyValue::Int(426)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Reflexe_(3)".to_string(), "reflex".to_string(), "reflexe".to_string()],
    datas: vec![
        vec![PyValue::Int(256)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Lust_(9)".to_string(), "lust".to_string(), "einheiten".to_string()],
    datas: vec![
        vec![PyValue::Int(255), PyValue::Int(391)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Paradigmen_sind_Absichten_(13)".to_string(), "paradigmen".to_string(), "absichten".to_string()],
    datas: vec![
        vec![PyValue::Int(10), PyValue::Int(410), PyValue::Int(411), PyValue::Int(42), PyValue::Int(493), PyValue::Int(494)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Wirklichkeiten_Wahrheit_Wahrnehmung_(10)".to_string(), "wirklichkeit".to_string(), "wirklichkeiten".to_string(), "wahrheit".to_string(), "wahrnehmung".to_string()],
    datas: vec![
        vec![PyValue::Int(233), PyValue::Int(265), PyValue::Int(268), PyValue::Int(322), PyValue::Int(342), PyValue::Int(480)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Rechnen".to_string(), "rechnen".to_string()],
    datas: vec![
        vec![PyValue::Int(404)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Stimmungen_Kombinationen_(14)".to_string(), "stimmung".to_string(), "stimmungen".to_string(), "kombination".to_string(), "kombinationen".to_string()],
    datas: vec![
        vec![PyValue::Int(290), PyValue::Int(296), PyValue::Int(325), PyValue::Int(326), PyValue::Int(327), PyValue::Int(33), PyValue::Int(402), PyValue::Int(403), PyValue::Int(406), PyValue::Int(407), PyValue::Int(408), PyValue::Int(430), PyValue::Int(492)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Struktur-Wissenschaften_(10)".to_string()],
    datas: vec![
        vec![PyValue::Int(438)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Muster-Wissenschaften_(20)".to_string()],
    datas: vec![
        vec![PyValue::Int(439), PyValue::Int(484)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Anführer_Arten_(7)".to_string()],
    datas: vec![
        vec![PyValue::Int(429), PyValue::Int(455), PyValue::Int(481), PyValue::Int(482), PyValue::Int(490), PyValue::Int(497), PyValue::Int(498), PyValue::Int(499), PyValue::Int(502), PyValue::Int(509)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Klassen_(20)".to_string(), "klasse".to_string(), "klassen".to_string()],
    datas: vec![
        vec![PyValue::Int(241), PyValue::Int(289), PyValue::Int(394), PyValue::Int(395), PyValue::Int(485), PyValue::Int(516)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Ordnung_und_Filterung_12_und_1pro12".to_string(), "ordnen".to_string(), "ordnenundfiltern".to_string(), "filtern".to_string()],
    datas: vec![
        vec![PyValue::Int(132), PyValue::Int(328), PyValue::Int(331), PyValue::Int(335)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Meta-Systeme_(12)".to_string(), "metasysteme".to_string(), "metasystem".to_string(), "meta-systeme".to_string(), "meta-system".to_string(), "menge".to_string(), "mengen".to_string()],
    datas: vec![
        vec![PyValue::Int(232), PyValue::Int(288), PyValue::Int(334), PyValue::Int(410), PyValue::Int(411), PyValue::Int(483), PyValue::Int(497), PyValue::Int(498), PyValue::Int(499), PyValue::Int(79), PyValue::Int(80)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_1/8".to_string(), "absicht1pro8".to_string(), "absicht1/8".to_string()],
    datas: vec![
        vec![PyValue::Int(272), PyValue::Int(379)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Ziele_(19)".to_string(), "ziele".to_string(), "maxima".to_string(), "höhenvorstellungen".to_string()],
    datas: vec![
        vec![PyValue::Int(271)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Konkreta_und_Focus_(2)".to_string(), "konkreta".to_string(), "focus".to_string(), "fokus".to_string()],
    datas: vec![
        vec![PyValue::Int(250), PyValue::Int(253), PyValue::Int(269)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Gefühle_(7)".to_string(), "gefuehle".to_string(), "emotionen".to_string(), "emotion".to_string(), "gefühle".to_string()],
    datas: vec![
        vec![PyValue::Int(243), PyValue::Int(283), PyValue::Int(284), PyValue::Int(285), PyValue::Int(286), PyValue::Int(29), PyValue::Int(305)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["abhängige_Verbundenheit_(90)".to_string(), "abhaengigkeit".to_string(), "abhängigkeit".to_string()],
    datas: vec![
        vec![PyValue::Int(357)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Karte_Filter_und_Unterscheidung_(1/12)".to_string(), "karte".to_string(), "filter".to_string(), "unterscheidung".to_string()],
    datas: vec![
        vec![PyValue::Int(377)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Fundament_(1/19)".to_string(), "fundament".to_string()],
    datas: vec![
        vec![PyValue::Int(356)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Gedanken_sind_Positionen_(17)".to_string(), "positionen".to_string(), "gedanken".to_string()],
    datas: vec![
        vec![PyValue::Int(249), PyValue::Int(317), PyValue::Int(323)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Funktionen_Vorstellungen_(16)".to_string(), "vorstellungen".to_string(), "vorstellung".to_string(), "funktionen".to_string()],
    datas: vec![
        vec![PyValue::Int(264), PyValue::Int(345), PyValue::Int(388), PyValue::Int(418)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Sollen_Frage_Vorgehensweise_(1/13)".to_string(), "sollen".to_string(), "frage".to_string(), "vorgehensweise".to_string()],
    datas: vec![
        vec![PyValue::Int(353), PyValue::Int(354)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Ansichten_Standpunkte_(18_17)".to_string(), "ansichten".to_string()],
    datas: vec![
        vec![PyValue::Int(240), PyValue::Int(346)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Verbundenheiten_(18)".to_string(), "verbundenheiten".to_string()],
    datas: vec![
        vec![PyValue::Int(252), PyValue::Int(299), PyValue::Int(300), PyValue::Int(336)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_13_ist_Helfen".to_string(), "absicht13".to_string(), "helfen".to_string()],
    datas: vec![
        vec![PyValue::Int(370)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Liebe_(7)".to_string(), "liebe".to_string()],
    datas: vec![
        vec![PyValue::Int(208), PyValue::Int(221), PyValue::Int(28), PyValue::Int(330), PyValue::Int(8), PyValue::Int(9)],\n        vec![PyValue::Tuple(vec![PyValue::Int(121), PyValue::Int(122)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Koalitionen_(10)".to_string(), "koalitionen".to_string()],
    datas: vec![
        vec![PyValue::Int(321)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["gegen_5".to_string()],
    datas: vec![
        vec![PyValue::Int(24)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Impulse_(5)".to_string(), "impulse".to_string()],
    datas: vec![
        vec![PyValue::Int(251), PyValue::Int(253), PyValue::Int(257), PyValue::Int(341)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Triebe_und_Bedürfnisse_(6)".to_string(), "trieb".to_string(), "triebe".to_string(), "bedürfnis".to_string(), "bedürfnisse".to_string(), "werte".to_string()],
    datas: vec![
        vec![PyValue::Int(254), PyValue::Int(392), PyValue::Int(396), PyValue::Int(397), PyValue::Int(423)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Taetigkeiten".to_string(), "tätigkeiten".to_string(), "taetigkeiten".to_string()],
    datas: vec![
        vec![PyValue::Int(424)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Reflektion_und_Kategorien_(1/15)".to_string(), "reflektion".to_string(), "kategorien".to_string()],
    datas: vec![
        vec![PyValue::Int(204), PyValue::Int(205), PyValue::Int(281)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Modus_und_Sein_(8)".to_string(), "zustaende".to_string(), "zustände".to_string(), "modus".to_string(), "modi".to_string(), "sein".to_string()],
    datas: vec![
        vec![PyValue::Int(234), PyValue::Int(337), PyValue::Int(385), PyValue::Int(387), PyValue::Int(491)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Motive".to_string(), "motive".to_string(), "motivation".to_string(), "motiv".to_string(), "absicht".to_string(), "absichten".to_string()],
    datas: vec![
        vec![PyValue::Int(10), PyValue::Int(149), PyValue::Int(167), PyValue::Int(168), PyValue::Int(18), PyValue::Int(229), PyValue::Int(230), PyValue::Int(42)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gedanken_sind_Positionen_(17)".to_string(), "positionen".to_string(), "gedanken".to_string()],
    datas: vec![
        vec![PyValue::Int(249), PyValue::Int(276)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Bewusstsein_und_Wahrnehmung".to_string(), "bewusstsein".to_string(), "wahrnehmung".to_string()],
    datas: vec![
        vec![PyValue::Int(229), PyValue::Int(231), PyValue::Int(265), PyValue::Int(281), PyValue::Int(304), PyValue::Int(342)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Errungenschaften".to_string(), "errungenschaften".to_string(), "ziele".to_string(), "erhalten".to_string()],
    datas: vec![
        vec![PyValue::Int(11), PyValue::Int(251), PyValue::Int(257)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["evolutionär_erwerben_und_Intelligenz_Kreativität".to_string(), "evolutionärerwerbenundintelligenz".to_string(), "intelligenz".to_string(), "erwerben".to_string(), "erlernen".to_string(), "lernen".to_string(), "evolutionaer".to_string(), "evolutionär".to_string(), "kreativität".to_string(), "kreativitaet".to_string(), "kreativ".to_string()],
    datas: vec![
        vec![PyValue::Int(12), PyValue::Int(13), PyValue::Int(27), PyValue::Int(32), PyValue::Int(47)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["brauchen".to_string(), "benoetigen".to_string(), "benötigen".to_string(), "notwendig".to_string()],
    datas: vec![
        vec![PyValue::Int(13), PyValue::Int(14)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Krankheit".to_string(), "krankheit".to_string(), "krankheiten".to_string(), "pathologisch".to_string(), "pathologie".to_string(), "psychiatrisch".to_string()],
    datas: vec![
        vec![PyValue::Int(24)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["alpha_beta".to_string(), "alphabeta".to_string(), "alpha".to_string(), "beta".to_string(), "omega".to_string(), "sigma".to_string()],
    datas: vec![
        vec![PyValue::Int(46)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Anführer".to_string(), "anfuehrer".to_string(), "chef".to_string()],
    datas: vec![
        vec![PyValue::Int(170), PyValue::Int(29), PyValue::Int(429), PyValue::Int(455), PyValue::Int(490), PyValue::Int(502), PyValue::Int(509)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Biologischer_Baum_(15)".to_string()],
    datas: vec![
        vec![PyValue::Int(500)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Biologischer_Baum_(16_->_5)".to_string()],
    datas: vec![
        vec![PyValue::Int(500)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Biologischer_Baum_(15)".to_string()],
    datas: vec![
        vec![PyValue::Int(500)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Biologischer_Baum_(15)".to_string()],
    datas: vec![
        vec![PyValue::Int(500)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Manipulation".to_string(), "manipulation".to_string()],
    datas: vec![
        vec![PyValue::Int(153)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Berufe".to_string(), "berufe".to_string(), "beruf".to_string()],
    datas: vec![
        vec![PyValue::Int(30)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Lösungen".to_string(), "lösungen".to_string(), "loesungen".to_string(), "loesung".to_string(), "lösungen".to_string()],
    datas: vec![
        vec![PyValue::Int(31)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Musik".to_string(), "musik".to_string()],
    datas: vec![
        vec![PyValue::Int(33)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["ergibt_Sinn".to_string(), "ergibtsinn".to_string(), "machtsinn".to_string(), "sinn".to_string()],
    datas: vec![
        vec![PyValue::Int(140)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Veränderung".to_string(), "veraenderung".to_string(), "veraendern".to_string(), "veränderung".to_string(), "verändern".to_string()],
    datas: vec![
        vec![PyValue::Int(142)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["bändigen_kontrollieren".to_string(), "baendigenkontrollieren".to_string(), "kontrollieren".to_string(), "baendigen".to_string(), "bändigen".to_string()],
    datas: vec![
        vec![PyValue::Int(143)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["vereinen".to_string(), "einheit".to_string()],
    datas: vec![
        vec![PyValue::Int(144)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Vorteile".to_string(), "vorteile".to_string(), "veraenderungnutzen".to_string()],
    datas: vec![
        vec![PyValue::Int(141)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Gegenspieler".to_string(), "gegenspieler".to_string(), "antagonist".to_string()],
    datas: vec![
        vec![PyValue::Int(137)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nervig".to_string()],
    datas: vec![
        vec![PyValue::Int(120)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["pro_nutzen".to_string(), "pronutzen".to_string()],
    datas: vec![
        vec![PyValue::Int(117)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Gegenposition".to_string(), "gegenposition".to_string()],
    datas: vec![
        vec![PyValue::Int(116)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Hilfe_erhalten".to_string(), "hilfeerhalten".to_string()],
    datas: vec![
        vec![PyValue::Int(114)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Helfen".to_string(), "helfen".to_string(), "hilfe".to_string()],
    datas: vec![
        vec![PyValue::Int(115)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Pro".to_string(), "pro".to_string(), "dafür".to_string(), "dafuer".to_string()],
    datas: vec![
        vec![PyValue::Int(17), PyValue::Int(48)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_miteinander_auskommen".to_string(), "nichtauskommen".to_string()],
    datas: vec![
        vec![PyValue::Int(123)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_dagegen".to_string(), "nichtdagegen".to_string()],
    datas: vec![
        vec![PyValue::Int(124)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["kein_Gegenteil".to_string(), "keingegenteil".to_string()],
    datas: vec![
        vec![PyValue::Int(125)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_dafür".to_string(), "nichtdafuer".to_string()],
    datas: vec![
        vec![PyValue::Int(126)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Hilfe_nicht_gebrauchen".to_string(), "hilfenichtgebrauchen".to_string()],
    datas: vec![
        vec![PyValue::Int(127)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_helfen_können".to_string(), "nichthelfenkoennen".to_string()],
    datas: vec![
        vec![PyValue::Int(128)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_abgeneigt".to_string(), "nichtabgeneigt".to_string()],
    datas: vec![
        vec![PyValue::Int(129)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["unmotivierbar".to_string()],
    datas: vec![
        vec![PyValue::Int(130)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["contra".to_string(), "dagegen".to_string()],
    datas: vec![
        vec![PyValue::Int(15), PyValue::Int(26)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Gegenteil".to_string(), "gegenteil".to_string()],
    datas: vec![
        vec![PyValue::Int(100), PyValue::Int(101), PyValue::Int(222)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Harmonie".to_string(), "harmonie".to_string()],
    datas: vec![
        vec![PyValue::Int(102), PyValue::Int(103)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['licht']".to_string()],
    parameterNames: vec![],
    datas: vec![
        vec![PyValue::Int(20), PyValue::Int(27), PyValue::Int(313)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Primzahlkreuz_pro_contra".to_string(), "primzahlkreuz".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primzahlkreuzprocontra".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Primzahlkreuz_pro_contra".to_string(), "primzahlkreuz".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primzahlkreuzprocontra".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["in_ReTa".to_string(), "inreta".to_string()],
    datas: vec![
        vec![PyValue::Int(209), PyValue::Int(210), PyValue::Int(474), PyValue::Int(475)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Vorzeichen".to_string(), "vorzeichen".to_string()],
    datas: vec![
        vec![PyValue::Int(118), PyValue::Int(119)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Primzahlen".to_string(), "primzahlen".to_string(), "vielfache".to_string(), "vielfacher".to_string()],
    datas: vec![
        vec![PyValue::Int(19)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Anwendung_der_Sonnen_und_Monde".to_string(), "anwendungdersonnenundmonde".to_string(), "anwendungdersonnen".to_string(), "anwendungenfuermonde".to_string()],
    datas: vec![
        vec![PyValue::Int(22)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Zählungen".to_string(), "zählungen".to_string(), "zaehlung".to_string(), "zaehlungen".to_string(), "zählung".to_string()],
    datas: vec![
        vec![PyValue::Int(169), PyValue::Int(188), PyValue::Int(25), PyValue::Int(386), PyValue::Int(390), PyValue::Int(45)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Jura".to_string(), "jura".to_string(), "gesetzeslehre".to_string(), "recht".to_string()],
    datas: vec![
        vec![PyValue::Int(34)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Vollkommenheit_des_Geistes".to_string(), "vollkommenheit".to_string(), "geist".to_string()],
    datas: vec![
        vec![PyValue::Int(35)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Gestirn".to_string(), "gestirn".to_string(), "mond".to_string(), "sonne".to_string(), "planet".to_string()],
    datas: vec![
        vec![PyValue::Int(154), PyValue::Int(64)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Konjunktiv_Wurzelbildung".to_string(), "konjunktiv".to_string(), "wurzel".to_string()],
    datas: vec![
        vec![PyValue::Int(106)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Mechanismen_der_Züchtung".to_string(), "mechanismen".to_string(), "wesen".to_string(), "zuechtung".to_string(), "züchtung".to_string(), "züchten".to_string(), "zuechten".to_string()],
    datas: vec![
        vec![PyValue::Int(107), PyValue::Int(108), PyValue::Int(109)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['gebrochengalaxie']".to_string()],
    parameterNames: vec!["14".to_string(), "10".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "21".to_string(), "15".to_string(), "9".to_string(), "5".to_string(), "6".to_string(), "13".to_string(), "12".to_string(), "7".to_string(), "22".to_string(), "19".to_string(), "23".to_string(), "4".to_string(), "11".to_string(), "2".to_string(), "3".to_string(), "20".to_string(), "8".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("10".to_string()), PyValue::Str("11".to_string()), PyValue::Str("12".to_string()), PyValue::Str("13".to_string()), PyValue::Str("14".to_string()), PyValue::Str("15".to_string()), PyValue::Str("16".to_string()), PyValue::Str("17".to_string()), PyValue::Str("18".to_string()), PyValue::Str("19".to_string()), PyValue::Str("2".to_string()), PyValue::Str("20".to_string()), PyValue::Str("21".to_string()), PyValue::Str("22".to_string()), PyValue::Str("23".to_string()), PyValue::Str("3".to_string()), PyValue::Str("4".to_string()), PyValue::Str("5".to_string()), PyValue::Str("6".to_string()), PyValue::Str("7".to_string()), PyValue::Str("8".to_string()), PyValue::Str("9".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['gebrochenuniversum']".to_string()],
    parameterNames: vec!["14".to_string(), "10".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "21".to_string(), "15".to_string(), "9".to_string(), "5".to_string(), "6".to_string(), "13".to_string(), "12".to_string(), "7".to_string(), "22".to_string(), "19".to_string(), "23".to_string(), "4".to_string(), "11".to_string(), "2".to_string(), "3".to_string(), "20".to_string(), "8".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("10".to_string()), PyValue::Str("11".to_string()), PyValue::Str("12".to_string()), PyValue::Str("13".to_string()), PyValue::Str("14".to_string()), PyValue::Str("15".to_string()), PyValue::Str("16".to_string()), PyValue::Str("17".to_string()), PyValue::Str("18".to_string()), PyValue::Str("19".to_string()), PyValue::Str("2".to_string()), PyValue::Str("20".to_string()), PyValue::Str("21".to_string()), PyValue::Str("22".to_string()), PyValue::Str("23".to_string()), PyValue::Str("3".to_string()), PyValue::Str("4".to_string()), PyValue::Str("5".to_string()), PyValue::Str("6".to_string()), PyValue::Str("7".to_string()), PyValue::Str("8".to_string()), PyValue::Str("9".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['gebrochenemotion']".to_string()],
    parameterNames: vec!["14".to_string(), "10".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "21".to_string(), "15".to_string(), "9".to_string(), "5".to_string(), "6".to_string(), "13".to_string(), "12".to_string(), "7".to_string(), "22".to_string(), "19".to_string(), "23".to_string(), "4".to_string(), "11".to_string(), "2".to_string(), "3".to_string(), "20".to_string(), "8".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("10".to_string()), PyValue::Str("11".to_string()), PyValue::Str("12".to_string()), PyValue::Str("13".to_string()), PyValue::Str("14".to_string()), PyValue::Str("15".to_string()), PyValue::Str("16".to_string()), PyValue::Str("17".to_string()), PyValue::Str("18".to_string()), PyValue::Str("19".to_string()), PyValue::Str("2".to_string()), PyValue::Str("20".to_string()), PyValue::Str("21".to_string()), PyValue::Str("22".to_string()), PyValue::Str("23".to_string()), PyValue::Str("3".to_string()), PyValue::Str("4".to_string()), PyValue::Str("5".to_string()), PyValue::Str("6".to_string()), PyValue::Str("7".to_string()), PyValue::Str("8".to_string()), PyValue::Str("9".to_string())],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['gebrochengroesse']".to_string()],
    parameterNames: vec!["14".to_string(), "10".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "21".to_string(), "15".to_string(), "9".to_string(), "5".to_string(), "6".to_string(), "13".to_string(), "12".to_string(), "7".to_string(), "22".to_string(), "19".to_string(), "23".to_string(), "4".to_string(), "11".to_string(), "2".to_string(), "3".to_string(), "20".to_string(), "8".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("10".to_string()), PyValue::Str("11".to_string()), PyValue::Str("12".to_string()), PyValue::Str("13".to_string()), PyValue::Str("14".to_string()), PyValue::Str("15".to_string()), PyValue::Str("16".to_string()), PyValue::Str("17".to_string()), PyValue::Str("18".to_string()), PyValue::Str("19".to_string()), PyValue::Str("2".to_string()), PyValue::Str("20".to_string()), PyValue::Str("21".to_string()), PyValue::Str("22".to_string()), PyValue::Str("23".to_string()), PyValue::Str("3".to_string()), PyValue::Str("4".to_string()), PyValue::Str("5".to_string()), PyValue::Str("6".to_string()), PyValue::Str("7".to_string()), PyValue::Str("8".to_string()), PyValue::Str("9".to_string())],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Religionen".to_string()],
    datas: vec![
        vec![PyValue::Int(36), PyValue::Int(37)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Drei".to_string()],
    datas: vec![
        vec![PyValue::Int(452), PyValue::Int(460)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Vier".to_string()],
    datas: vec![
        vec![PyValue::Int(453)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Fünf".to_string(), "Fuenf".to_string()],
    datas: vec![
        vec![PyValue::Int(454)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Sechs".to_string()],
    datas: vec![
        vec![PyValue::Int(457)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Sieben".to_string()],
    datas: vec![
        vec![PyValue::Int(457)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Acht".to_string()],
    datas: vec![
        vec![PyValue::Int(458)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Neun".to_string()],
    datas: vec![
        vec![PyValue::Int(459)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Zehn".to_string()],
    datas: vec![
        vec![PyValue::Int(456)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Zwölf".to_string(), "Zwoelf".to_string()],
    datas: vec![
        vec![PyValue::Int(456)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Weisheit_etc".to_string(), "weisheit".to_string(), "metaweisheit".to_string(), "meta-weisheit".to_string(), "idiot".to_string(), "weise".to_string(), "optimal".to_string(), "optimum".to_string()],
    datas: vec![
        vec![PyValue::Int(112)],\n        vec![PyValue::Tuple(vec![PyValue::Int(40), PyValue::Int(41)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Dein_Recht_bekommen".to_string(), "rechte".to_string(), "recht".to_string(), "selbstgerecht".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(291), PyValue::Int(292)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["unterlegen_überlegen".to_string(), "unterlegen".to_string(), "ueberlegen".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(380), PyValue::Int(381)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Ehrlichkeit_und_Streit".to_string(), "streit".to_string(), "ehrlichkeit".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(375), PyValue::Int(376)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Würdig".to_string(), "wuerdig".to_string(), "würdig".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(373), PyValue::Int(374)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Regel_vs_Ausnahme".to_string(), "regel".to_string(), "ausnahme".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(371), PyValue::Int(372)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Filterart_Widrigkeit".to_string(), "filterart".to_string(), "widrigkeit".to_string()],
    datas: vec![
        vec![PyValue::Int(331), PyValue::Int(335)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Werte".to_string(), "werte".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(360), PyValue::Int(361)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Gutartigkeits-Egoismus".to_string(), "position".to_string(), "gutesreziprok".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(362), PyValue::Int(363)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Reflektieren_Erkenntnis-Erkennen".to_string(), "reflektieren".to_string(), "erkenntnis".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(364), PyValue::Int(365)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Vertrauen_wollen".to_string(), "vertrauenwollen".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(366), PyValue::Int(367)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["einklinken_vertrauen_anprangern".to_string(), "einklinken".to_string(), "vertrauenerhalten".to_string(), "anprangern".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(368), PyValue::Int(369)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Ausrichten_Einrichten".to_string(), "einrichten".to_string(), "ausrichten".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(358), PyValue::Int(359)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Toleranz_Respekt_Akzeptanz_Willkommen".to_string(), "toleranz".to_string(), "respekt".to_string(), "akzeptanz".to_string(), "willkommen".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(62), PyValue::Int(63)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["familiebrauchen".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(279), PyValue::Int(280)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["ego".to_string(), "bescheiden".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(277), PyValue::Int(278)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Selbstsucht_Ichsucht_etc".to_string(), "selbstsucht".to_string(), "ichsucht".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(274), PyValue::Int(275)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Forschen_Erfinden_Einklinken".to_string(), "wissenschaft".to_string(), "forschen".to_string(), "einklinken".to_string(), "erfinden".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(258), PyValue::Int(259)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Kooperation_vs_Arsch".to_string(), "arschloch".to_string(), "kooperation".to_string(), "arsch".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(245), PyValue::Int(246)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Liebe_usw".to_string(), "liebe".to_string(), "zuneigung".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(247), PyValue::Int(248)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Selbstlosigkeit_Ichlosigkeit_etc".to_string(), "selbstlos".to_string(), "ichlos".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(238), PyValue::Int(239)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["variationsreich_eintönig".to_string(), "eintönig".to_string(), "eintoenig".to_string(), "variationsreich".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(236), PyValue::Int(237)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Zuneigung_Abneigung".to_string(), "abgeneigt".to_string(), "zugewandt".to_string(), "reserviert".to_string(), "zugeneigt".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(199), PyValue::Int(200)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["ehrlich_vs_höflich".to_string(), "ehrlich".to_string(), "höflich".to_string(), "hoeflich".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(224), PyValue::Int(225)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["ehrlich_vs_höflich".to_string(), "ehrlich".to_string(), "höflich".to_string(), "hoeflich".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(224), PyValue::Int(225)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Tragweite".to_string(), "tragweite".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(211), PyValue::Int(212)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["wertvoll".to_string(), "wertlos".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(186), PyValue::Int(187)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Götter_Propheten_Familien_Freunde".to_string(), "familiaer".to_string(), "goettlich".to_string(), "freunde".to_string(), "propheten".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(184), PyValue::Int(185)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["sanft_vs_hart".to_string(), "sanft".to_string(), "hart".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(159), PyValue::Int(160)]), PyValue::Tuple(vec![PyValue::Int(161), PyValue::Int(162)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["vereinen_vs_verbinden".to_string(), "vereinenverbinden".to_string(), "vereinen".to_string(), "verbinden".to_string(), "einheit".to_string(), "verbindung".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(133), PyValue::Int(134)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["ähnlich".to_string(), "aehnlich".to_string()],
    datas: vec![
        vec![PyValue::Int(220)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["gut_böse_lieb_schlecht".to_string(), "gut".to_string(), "böse".to_string(), "boese".to_string(), "lieb".to_string(), "schlecht".to_string()],
    datas: vec![
        vec![PyValue::Int(52), PyValue::Int(53)],\n        vec![PyValue::Tuple(vec![PyValue::Int(38), PyValue::Int(39)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Sinn_und_Zweck_des_Lebens".to_string(), "sinn".to_string(), "zweck".to_string(), "bedeutung".to_string()],
    datas: vec![
        vec![PyValue::Int(189), PyValue::Int(88)],\n        vec![PyValue::Tuple(vec![PyValue::Int(181), PyValue::Int(182)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Zeit_vs_Raum".to_string(), "zeit".to_string(), "raum".to_string(), "zeitlich".to_string(), "räumlich".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(49), PyValue::Int(50)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["egalitär_vs_autoritär".to_string(), "egalitaerautoritaer".to_string(), "egalitaer".to_string(), "autoritaer".to_string(), "egalitär".to_string(), "autoritär".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(163), PyValue::Int(164)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Meinungen_und_Ruf".to_string(), "meinungen".to_string(), "anderemenschen".to_string(), "ruf".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(60), PyValue::Int(61)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Meinungsintelligenz".to_string(), "meinungsintelligenz".to_string(), "ursprungsintelligenz".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(151), PyValue::Int(152)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Sittlichkeit".to_string(), "sittlichkeit".to_string(), "annaehrerung".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(179), PyValue::Int(180)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Führung".to_string(), "führung".to_string(), "fuehrung".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(173), PyValue::Int(174)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Durchleuchten".to_string(), "durchleuchten".to_string(), "erleuchten".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(177), PyValue::Int(178)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Fördern_Sensiblisieren_und_Gedeihen".to_string(), "foerdern".to_string(), "fördern".to_string(), "begrenzen".to_string(), "sensibilisieren".to_string(), "gedeihen".to_string(), "verderben".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(175), PyValue::Int(176)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Überheblichkeit".to_string(), "überheblich".to_string(), "ueberheblichkeit".to_string(), "ueberheblich".to_string(), "überheblichkeit".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(171), PyValue::Int(172)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Polung_der_Liebe".to_string(), "liebepolung".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(121), PyValue::Int(122)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Egoismus_vs_Altruismus".to_string(), "egoismus".to_string(), "altruismus".to_string(), "egoist".to_string(), "altruist".to_string()],
    datas: vec![
        vec![PyValue::Int(136)],\n        vec![PyValue::Tuple(vec![PyValue::Int(66), PyValue::Int(67)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["kausal".to_string(), "geltung".to_string(), "genese".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(110), PyValue::Int(111)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Gleichheit".to_string(), "gleich".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(192), PyValue::Int(193)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Überleben".to_string(), "ueberleben".to_string()],
    datas: vec![
        vec![],\n        vec![PyValue::Tuple(vec![PyValue::Int(194), PyValue::Int(195)])],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec![],
    datas: vec![
        vec![PyValue::Int(43), PyValue::Int(54), PyValue::Int(74), PyValue::Int(95)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["um1".to_string()],
    datas: vec![
        vec![PyValue::Int(155)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["um2".to_string()],
    datas: vec![
        vec![PyValue::Int(156)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["um3".to_string()],
    datas: vec![
        vec![PyValue::Int(157)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["warum_Transzendentalie_=_Strukturgroesse_=_Charakter".to_string(), "warumtranszendentaliezustrukturgroesseundcharakter".to_string()],
    datas: vec![
        vec![PyValue::Int(165), PyValue::Int(4), PyValue::Int(5), PyValue::Int(54)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["warum_Transzendentalie_=_Komplexität_von_Michael_Commons".to_string(), "warumtranszendentaliegleichkomplexitaet".to_string()],
    datas: vec![
        vec![PyValue::Int(166), PyValue::Int(5), PyValue::Int(65)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Rahmen-Bedingungen".to_string(), "rahmen".to_string()],
    datas: vec![
        vec![PyValue::Int(226)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Motive_gleichförmige_Polygone".to_string(), "motivgleichfoermig".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primMotivGleichf".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Struktur_gleichförmige_Polygone".to_string(), "strukturgleichfoermig".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primStrukGleichf".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Motive_Sternpolygone".to_string(), "motivstern".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primMotivStern".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Struktur_Sternpolygone".to_string(), "strukturstern".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primStrukStern".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Motiv_Sternpolygon_gebrochen-rational".to_string(), "motivgebrstern".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primMotivSternGebr".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Struktur_Sternpolyon_gebrochen-rational".to_string(), "strukgebrstern".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primStrukSternGebr".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Motiv_gleichförmige_Polygone_gebrochen-rational".to_string(), "motivgebrgleichf".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primMotivGleichfGebr".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Struktur_gleichförmige_Polygone_gebrochen-rational".to_string(), "strukgebrgleichf".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("primStrukGleichfGebr".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["beschrieben".to_string()],
    datas: vec![
        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![PyValue::Str("PrimCSV".to_string())],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["Q".to_string(), "q".to_string(), "Siebzehn".to_string()],
    datas: vec![
        vec![PyValue::Int(431), PyValue::Int(432), PyValue::Int(433), PyValue::Int(434), PyValue::Int(437), PyValue::Int(441), PyValue::Int(442), PyValue::Int(443), PyValue::Int(445), PyValue::Int(450), PyValue::Int(467), PyValue::Int(468), PyValue::Int(469), PyValue::Int(487), PyValue::Int(488)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["i".to_string(), "I".to_string(), "Neun".to_string()],
    datas: vec![
        vec![PyValue::Int(517)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["G".to_string(), "g".to_string(), "Sieben".to_string()],
    datas: vec![
        vec![PyValue::Int(518)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["J".to_string(), "j".to_string(), "Zehn".to_string()],
    datas: vec![
        vec![PyValue::Int(514)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["k".to_string(), "K".to_string(), "Elf".to_string()],
    datas: vec![
        vec![PyValue::Int(515)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["E".to_string(), "e".to_string(), "Fünf".to_string()],
    datas: vec![
        vec![PyValue::Int(511)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["L".to_string(), "l".to_string(), "Zwölf".to_string()],
    datas: vec![
        vec![PyValue::Int(506)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["Y".to_string(), "y".to_string(), "Fünfundzwanzig".to_string()],
    datas: vec![
        vec![PyValue::Int(507), PyValue::Int(510)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["Kontinuen".to_string(), "F".to_string(), "f".to_string(), "Sechs".to_string()],
    datas: vec![
        vec![PyValue::Int(508)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["F".to_string(), "f".to_string(), "Sechs".to_string(), "Kontinuen".to_string()],
    datas: vec![
        vec![PyValue::Int(508)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["O".to_string(), "o".to_string(), "Fünfzehn".to_string()],
    datas: vec![
        vec![PyValue::Int(5)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["H".to_string(), "h".to_string(), "Acht".to_string()],
    datas: vec![
        vec![PyValue::Int(491)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["N".to_string(), "n".to_string(), "Vierzehn".to_string()],
    datas: vec![
        vec![PyValue::Int(492)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["M".to_string(), "m".to_string(), "Dreizehn".to_string()],
    datas: vec![
        vec![PyValue::Int(493)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["T".to_string(), "t".to_string(), "Zwanzig".to_string()],
    datas: vec![
        vec![PyValue::Int(486)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["P".to_string(), "p".to_string(), "Sechszehn".to_string()],
    datas: vec![
        vec![PyValue::Int(435)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["P5".to_string(), "p5".to_string(), "Sechszehn->Fünf".to_string()],
    datas: vec![
        vec![PyValue::Int(501)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["P5".to_string(), "p5".to_string(), "Sechszehn->Fünf".to_string()],
    datas: vec![
        vec![PyValue::Int(501)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["P".to_string(), "p".to_string(), "Sechszehn".to_string()],
    datas: vec![
        vec![PyValue::Int(435)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["X".to_string(), "x".to_string(), "Vierundzwanzig".to_string()],
    datas: vec![
        vec![PyValue::Int(25), PyValue::Int(386), PyValue::Int(436), PyValue::Int(55)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["S".to_string(), "s".to_string(), "Neunzehn".to_string()],
    datas: vec![
        vec![PyValue::Int(504)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["R".to_string(), "r".to_string(), "Achtzehn".to_string()],
    datas: vec![
        vec![PyValue::Int(436), PyValue::Int(451)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["A".to_string(), "a".to_string(), "Eins".to_string()],
    datas: vec![
        vec![PyValue::Int(446)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["B".to_string(), "b".to_string(), "Zwei".to_string()],
    datas: vec![
        vec![PyValue::Int(447)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["C".to_string(), "c".to_string(), "Drei".to_string()],
    datas: vec![
        vec![PyValue::Int(448)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["D".to_string(), "d".to_string(), "Vier".to_string()],
    datas: vec![
        vec![PyValue::Int(449)],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![],\n        vec![]
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

        Self {
            paraNdataMatrix,
            kombiParaNdataMatrix,
            kombiParaNdataMatrix2,
        }
    }
}
