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
pub struct I18nSubset {
    pub paraNdataMatrix: Vec<StoreParameterEntry>,
    pub kombiParaNdataMatrix: IndexMap<i64, Vec<String>>,
    pub kombiParaNdataMatrix2: IndexMap<i64, Vec<String>>,
}

impl I18nSubset {
    pub fn new() -> Self {
        let mut paraNdataMatrix: Vec<StoreParameterEntry> = vec![];
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Wichtigste".to_string(), "wichtigste".to_string()],
    datas: vec![
        vec![PyAtom::Int(10), PyAtom::Int(4), PyAtom::Int(5), PyAtom::Int(8)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Mensch-zu-Tier".to_string(), "menschtier".to_string(), "tiermensch".to_string()],
    datas: vec![
        vec![PyAtom::Int(314)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Superkräfte".to_string(), "Superkraefte".to_string()],
    datas: vec![
        vec![PyAtom::Int(444), PyAtom::Int(494), PyAtom::Int(496), PyAtom::Int(503)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Superkräfte".to_string(), "Superkraefte".to_string()],
    datas: vec![
        vec![PyAtom::Int(444), PyAtom::Int(494), PyAtom::Int(496)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Evolution_vs_Design_intelligent".to_string()],
    datas: vec![
        vec![PyAtom::Int(519)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Evolution_vs_Design_intelligent".to_string()],
    datas: vec![
        vec![PyAtom::Int(519)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Superkräfte".to_string(), "Superkraefte".to_string()],
    datas: vec![
        vec![PyAtom::Int(444), PyAtom::Int(494), PyAtom::Int(496)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Formationen".to_string()],
    datas: vec![
        vec![PyAtom::Int(461)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Ansichten_Standpunkte_(18_17)".to_string(), "ansichten".to_string()],
    datas: vec![
        vec![PyAtom::Int(240), PyAtom::Int(346)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["(politische)_Richtungen_(7)".to_string(), "richtungen".to_string(), "politische".to_string()],
    datas: vec![
        vec![PyAtom::Int(235)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Wirklichkeiten_(10)".to_string(), "wirklichkeit".to_string(), "wirklichkeiten".to_string()],
    datas: vec![
        vec![PyAtom::Int(233), PyAtom::Int(265), PyAtom::Int(268), PyAtom::Int(322), PyAtom::Int(420)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Meta-Systeme_(12)".to_string(), "metasysteme".to_string(), "metasystem".to_string(), "meta-systeme".to_string(), "meta-system".to_string()],
    datas: vec![
        vec![PyAtom::Int(232), PyAtom::Int(288), PyAtom::Int(334), PyAtom::Int(410), PyAtom::Int(411), PyAtom::Int(483), PyAtom::Int(497), PyAtom::Int(498), PyAtom::Int(499), PyAtom::Int(79), PyAtom::Int(80)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Intelligenz".to_string(), "intelligenz".to_string()],
    datas: vec![
        vec![PyAtom::Int(214)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Gleichheit_Freiheit_Ordnung".to_string(), "gleichheit".to_string(), "freiheit".to_string(), "gleichheit".to_string()],
    datas: vec![
        vec![PyAtom::Int(132), PyAtom::Int(324), PyAtom::Int(328), PyAtom::Int(331), PyAtom::Int(335), PyAtom::Int(497), PyAtom::Int(498), PyAtom::Int(499), PyAtom::Int(79), PyAtom::Int(80)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Komplexität".to_string(), "komplexität".to_string(), "komplexitaet".to_string()],
    datas: vec![
        vec![PyAtom::Int(213)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['planet']".to_string()],
    parameterNames: vec!["Mechanismen".to_string(), "mechanismen".to_string(), "mechanismus".to_string()],
    datas: vec![
        vec![PyAtom::Int(107)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Zweitwichtigste".to_string(), "zweitwichtigste".to_string()],
    datas: vec![
        vec![PyAtom::Int(183), PyAtom::Int(19), PyAtom::Int(65)],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(10)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Drittwichtigste".to_string(), "drittwichtigste".to_string()],
    datas: vec![
        vec![PyAtom::Int(64)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste']".to_string()],
    parameterNames: vec!["Motive_Sternpolygone".to_string(), "viertwichtigste".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primMotivStern".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste2']".to_string()],
    parameterNames: vec!["Wichtigste".to_string(), "wichtigstes".to_string()],
    datas: vec![
        vec![PyAtom::Int(0), PyAtom::Int(1), PyAtom::Int(2), PyAtom::Int(207), PyAtom::Int(36), PyAtom::Int(37)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wichtigste2']".to_string()],
    parameterNames: vec!["Zweitwichtigste".to_string(), "zweitwichtigste".to_string()],
    datas: vec![
        vec![PyAtom::Int(30)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["Halbierung".to_string(), "halbierung".to_string(), "halbierungen".to_string()],
    datas: vec![
        vec![PyAtom::Int(86)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Religions-Gründer-Typ".to_string(), "religionsgründertyp".to_string(), "prophet".to_string(), "archon".to_string(), "religionsgruendertyp".to_string()],
    datas: vec![
        vec![PyAtom::Int(503), PyAtom::Int(72)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Satan_Teufel".to_string()],
    datas: vec![
        vec![PyAtom::Int(495)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Satan_Teufel".to_string()],
    datas: vec![
        vec![PyAtom::Int(495)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Hinduismus".to_string(), "hinduismus".to_string()],
    datas: vec![
        vec![PyAtom::Int(217)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Sternpolygon".to_string(), "sternpolygon".to_string()],
    datas: vec![
        vec![PyAtom::Int(0), PyAtom::Int(36), PyAtom::Int(6)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["der_Tierkreiszeichen".to_string(), "dertierkreiszeichen".to_string(), "babylon".to_string()],
    datas: vec![
        vec![PyAtom::Int(0), PyAtom::Int(207), PyAtom::Int(36), PyAtom::Int(477), PyAtom::Int(478)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Sternpolygon_vs_gleichförmiges".to_string(), "vergleich".to_string(), "sternpolygonvsgleichfoermiges".to_string(), "vergleichnvs1divn".to_string()],
    datas: vec![
        vec![PyAtom::Int(87)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Messias".to_string(), "messias".to_string(), "heptagramm".to_string(), "hund".to_string(), "messiase".to_string(), "messiasse".to_string()],
    datas: vec![
        vec![PyAtom::Int(503), PyAtom::Int(7)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["gleichförmiges_Polygon".to_string(), "gleichförmigespolygon".to_string(), "gleichfoermigespolygon".to_string(), "nichtsternpolygon".to_string(), "polygon".to_string()],
    datas: vec![
        vec![PyAtom::Int(16), PyAtom::Int(37)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['religionen']".to_string()],
    parameterNames: vec!["Vertreter_höherer_Konzepte".to_string(), "vertreterhoehererkonzepte".to_string(), "galaxien".to_string(), "galaxie".to_string(), "schwarzesonne".to_string(), "schwarzesonnen".to_string(), "universum".to_string(), "universen".to_string(), "kreis".to_string(), "kreise".to_string(), "kugel".to_string(), "kugeln".to_string()],
    datas: vec![
        vec![PyAtom::Int(23)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Lebewesen_Galaxie_am_Besten".to_string()],
    datas: vec![
        vec![PyAtom::Int(470), PyAtom::Int(471), PyAtom::Int(473)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Offenbarung_des_Johannes".to_string(), "offenbarung".to_string(), "offenbarungdesjohannes".to_string(), "johannes".to_string(), "bibel".to_string(), "offenbarungjohannes".to_string()],
    datas: vec![
        vec![PyAtom::Int(90)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["Teilchen-Meta-Physik".to_string(), "addition".to_string(), "identitaet".to_string(), "Identität".to_string()],
    datas: vec![
        vec![PyAtom::Int(219), PyAtom::Int(223), PyAtom::Int(307), PyAtom::Int(308), PyAtom::Int(333), PyAtom::Int(387), PyAtom::Int(388), PyAtom::Int(406)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Hochzüchten".to_string(), "hochzüchten".to_string(), "hochzuechten".to_string()],
    datas: vec![
        vec![PyAtom::Int(318), PyAtom::Int(319)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Teilchen_anderes_Universum".to_string()],
    datas: vec![
        vec![PyAtom::Int(512)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Teilchen_anderes_Universum".to_string()],
    datas: vec![
        vec![PyAtom::Int(512)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Teilchen_anderes_Universum".to_string()],
    datas: vec![
        vec![PyAtom::Int(512)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Zusammenhang_Gehirn_Kosmos_Universum".to_string()],
    datas: vec![
        vec![PyAtom::Int(489)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Zahlenarten".to_string()],
    datas: vec![
        vec![PyAtom::Int(462)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Bestrafung".to_string()],
    datas: vec![
        vec![PyAtom::Int(463)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bestrafung".to_string()],
    datas: vec![
        vec![PyAtom::Int(463)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["weniger_am_Menschen".to_string()],
    datas: vec![
        vec![PyAtom::Int(464)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Erlösung".to_string(), "Erloesung".to_string()],
    datas: vec![
        vec![PyAtom::Int(465)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Erlösung".to_string(), "Erloesung".to_string()],
    datas: vec![
        vec![PyAtom::Int(465)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gewalt".to_string()],
    datas: vec![
        vec![PyAtom::Int(466)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Gewalt".to_string()],
    datas: vec![
        vec![PyAtom::Int(466), PyAtom::Int(479)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Farben".to_string()],
    datas: vec![
        vec![PyAtom::Int(444)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["künstliches_Leben_(15)".to_string(), "künstlichesleben".to_string(), "grosseki".to_string()],
    datas: vec![
        vec![PyAtom::Int(409)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Software-Lizenzen_akademische_Grade".to_string(), "softwarelizenz".to_string(), "akademischeGrade".to_string()],
    datas: vec![
        vec![PyAtom::Int(422)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Strategie_Taktik_(15m8)".to_string(), "strategie".to_string(), "taktik".to_string()],
    datas: vec![
        vec![PyAtom::Int(385)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Universelles_Verhältnis_gleicher_Zahlen".to_string(), "verhaeltnisgleicherzahl".to_string()],
    datas: vec![
        vec![PyAtom::Int(383)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["universelles_Recht".to_string(), "recht".to_string(), "jura".to_string()],
    datas: vec![
        vec![PyAtom::Int(34), PyAtom::Int(382), PyAtom::Int(65)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["sowas_wie_Kombinieren_Verknüpfen".to_string(), "kombinierenetc".to_string()],
    datas: vec![
        vec![PyAtom::Int(320)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Hochzüchten".to_string(), "hochzüchten".to_string(), "hochzuechten".to_string()],
    datas: vec![
        vec![PyAtom::Int(318), PyAtom::Int(319)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Teilchen-Meta-Physik".to_string()],
    datas: vec![
        vec![PyAtom::Int(219), PyAtom::Int(308)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Universelle_(15)".to_string()],
    datas: vec![
        vec![PyAtom::Int(219), PyAtom::Int(308)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["Wirklichkeiten_(10)".to_string(), "wirklichkeit".to_string(), "wirklichkeiten".to_string()],
    datas: vec![
        vec![PyAtom::Int(420)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Galaktische_(14)".to_string()],
    datas: vec![
        vec![PyAtom::Int(406)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Multiverselle_(16)".to_string()],
    datas: vec![
        vec![PyAtom::Int(388), PyAtom::Int(418)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["die_Tugendsortierung_(13_mit_14)".to_string()],
    datas: vec![
        vec![PyAtom::Int(411)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["die_Galaxie_Unterbereiche_(13)".to_string()],
    datas: vec![
        vec![PyAtom::Int(223), PyAtom::Int(307), PyAtom::Int(412)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["das_Gute_die_Richtung_(7)".to_string()],
    datas: vec![
        vec![PyAtom::Int(333)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['teilchen']".to_string()],
    parameterNames: vec!["Raum_und_Dimensionen_(8)".to_string()],
    datas: vec![
        vec![PyAtom::Int(387)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["keine_Nur-Paradigma-Religionen".to_string(), "metaparadigmareligion".to_string()],
    datas: vec![
        vec![PyAtom::Int(190), PyAtom::Int(191), PyAtom::Int(196)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Kugeln_Kreise".to_string(), "kugelnkreise".to_string(), "kugeln".to_string(), "kreise".to_string()],
    datas: vec![
        vec![PyAtom::Int(145), PyAtom::Int(77)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Raumzeit_Anordnung_mathematisch_universell".to_string()],
    datas: vec![
        vec![PyAtom::Int(472)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Multiversalien_(16)".to_string(), "multiversalien".to_string()],
    datas: vec![
        vec![PyAtom::Int(389)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Meta-Physik-Teilchen_(1)".to_string(), "teilchen".to_string()],
    datas: vec![
        vec![PyAtom::Int(388)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Kugeln_Kreise".to_string(), "kugelnkreise".to_string(), "kugeln".to_string(), "kreise".to_string()],
    datas: vec![
        vec![PyAtom::Int(145), PyAtom::Int(77)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["chinesisches_Horoskop".to_string(), "chinesischeshoroskop".to_string(), "china".to_string()],
    datas: vec![
        vec![PyAtom::Int(91)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["babylonische_Tierkreiszeichen".to_string(), "tierkreiszeichen".to_string(), "babylon".to_string()],
    datas: vec![
        vec![PyAtom::Int(1), PyAtom::Int(2)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Thomasevangelium".to_string(), "thomasevangelium".to_string(), "thomas".to_string()],
    datas: vec![
        vec![PyAtom::Int(0), PyAtom::Int(3), PyAtom::Int(303)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Netzwerk".to_string(), "netzwerk".to_string()],
    datas: vec![
        vec![PyAtom::Int(417), PyAtom::Int(436)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Kontroverse_(51)".to_string(), "kontroverse".to_string()],
    datas: vec![
        vec![PyAtom::Int(421)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["mathematisches_Design_(32)".to_string(), "mathematischesdesign".to_string()],
    datas: vec![
        vec![PyAtom::Int(419)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()],
    datas: vec![
        vec![PyAtom::Int(84)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()],
    datas: vec![
        vec![PyAtom::Int(84)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Transzendentalien_innen_außen".to_string(), "innenaussenstrukur".to_string(), "strukturalieninnenaußen".to_string(), "strukturalieninnenaussen".to_string(), "innenaußenstrukur".to_string(), "transzendentalieninnenaußen".to_string(), "transzendentalieninnenaussen".to_string()],
    datas: vec![
        vec![PyAtom::Int(149)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Modallogik".to_string(), "modallogik".to_string()],
    datas: vec![
        vec![PyAtom::Int(148)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["5".to_string(), "fünf".to_string(), "fünfer".to_string(), "fünferstruktur".to_string(), "fuenf".to_string(), "fuenfer".to_string(), "fuenferstruktur".to_string()],
    datas: vec![
        vec![PyAtom::Int(96)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["9".to_string(), "neun".to_string(), "neuner".to_string(), "neunerstruktur".to_string()],
    datas: vec![
        vec![PyAtom::Int(94)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["3".to_string(), "drei".to_string(), "dreier".to_string(), "dreierstruktur".to_string()],
    datas: vec![
        vec![PyAtom::Int(315), PyAtom::Int(316), PyAtom::Int(92), PyAtom::Int(93)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["Licht".to_string(), "licht".to_string()],
    datas: vec![
        vec![PyAtom::Int(20), PyAtom::Int(27), PyAtom::Int(313)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Strukturgrösse".to_string(), "strukturgroesse".to_string(), "größe".to_string(), "groesse".to_string(), "gross".to_string(), "strukturgroesse".to_string(), "strukturgroeße".to_string(), "strukturgrösse".to_string(), "strukturgröße".to_string()],
    datas: vec![
        vec![PyAtom::Int(197), PyAtom::Int(21), PyAtom::Int(4), PyAtom::Int(425), PyAtom::Int(54)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["Strukturgrösse".to_string(), "strukturgroesse".to_string(), "größe".to_string(), "groesse".to_string(), "gross".to_string(), "strukturgroesse".to_string(), "strukturgroeße".to_string(), "strukturgrösse".to_string(), "strukturgröße".to_string()],
    datas: vec![
        vec![PyAtom::Int(197), PyAtom::Int(21), PyAtom::Int(4), PyAtom::Int(425), PyAtom::Int(54)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["Organisationen".to_string(), "organisationen".to_string(), "organisation".to_string()],
    datas: vec![
        vec![PyAtom::Int(30), PyAtom::Int(425), PyAtom::Int(82)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['strukturgroesse']".to_string()],
    parameterNames: vec!["politische_Systeme".to_string(), "politischesysteme".to_string(), "politik".to_string()],
    datas: vec![
        vec![PyAtom::Int(83)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["meta".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(2), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["konkret".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(2), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Theorie".to_string(), "theorie".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(3), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Praxis".to_string(), "praxis".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(3), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Management".to_string(), "management".to_string(), "stau".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(4), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["verändernd".to_string(), "veraendernd".to_string(), "fluss".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(4), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["ganzheitlich".to_string(), "mathematisch_diskret".to_string(), "diskret".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(5), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["darüber_hinausgehend".to_string(), "hinausgehend".to_string(), "kontinuierlich".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(5), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Universum_Strukturalien_Transzendentalien".to_string(), "universum".to_string(), "strukturalie".to_string(), "strukturalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(5)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Richtung_als_Richtung".to_string(), "richtungrichtung".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::NoneValue])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Galaxieabsicht".to_string(), "absichtgalaxie".to_string(), "absicht".to_string(), "motive".to_string(), "motiv".to_string(), "absichten".to_string(), "galaxie".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(10)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Absicht_Reziproke_Galaxie".to_string(), "absichtgalaxiereziproke".to_string(), "absichtreziproke".to_string(), "motivereziproke".to_string(), "motivreziproke".to_string(), "absichtenreziproke".to_string(), "galaxiereziproke".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(42)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Universum_Reziproke".to_string(), "universumreziproke".to_string(), "strukturaliereziproke".to_string(), "strukturalienreziproke".to_string(), "transzendentalienreziproke".to_string(), "transzendentaliereziproke".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(131)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["Dagegen-Gegentranszendentalie".to_string(), "dagegengegentranszendentalie".to_string(), "dagegengegentranszendentalien".to_string(), "dagegengegenstrukturalien".to_string(), "dagegengegenstrukturalie".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(138)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primzahlwirkung']".to_string()],
    parameterNames: vec!["neutrale_Gegentranszendentalie".to_string(), "neutralegegentranszendentalie".to_string(), "neutralegegentranszendentalien".to_string(), "neutralegegenstrukturalien".to_string(), "neutralegegenstrukturalie".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(202)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Unternehmung_Geschäft".to_string(), "unternehmen".to_string(), "unternehmung".to_string(), "geschaeft".to_string(), "geschäft".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(6), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["wertvoll".to_string(), "wert".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(6), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Beherrschen".to_string(), "regieren".to_string(), "beherrschen".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(7), PyAtom::Int(0)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universummetakonkret']".to_string()],
    parameterNames: vec!["Richtung".to_string(), "richtung".to_string(), "gut".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(7), PyAtom::Int(1)])]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["analytische_Ontologie".to_string(), "analytischeontologie".to_string(), "ontologie".to_string()],
    datas: vec![
        vec![PyAtom::Int(84)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Gegentranszendentalien".to_string(), "gegentranszendentalien".to_string(), "gegentranszendentalie".to_string(), "gegenstrukturalien".to_string(), "gegenalien".to_string(), "gegenuniversalien".to_string()],
    datas: vec![
        vec![PyAtom::Int(138), PyAtom::Int(202)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Systemsachen".to_string(), "systemsachen".to_string()],
    datas: vec![
        vec![PyAtom::Int(150)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Transzendentalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string(), "strukturalien".to_string(), "alien".to_string(), "universalien".to_string()],
    datas: vec![
        vec![PyAtom::Int(198), PyAtom::Int(390), PyAtom::Int(5), PyAtom::Int(54), PyAtom::Int(55)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Reziproke_von_Transzendentalien".to_string(), "transzendentalienreziproke".to_string(), "transzendentaliereziproke".to_string(), "strukturalienreziproke".to_string(), "alienreziproke".to_string(), "universalienreziproke".to_string()],
    datas: vec![
        vec![PyAtom::Int(131), PyAtom::Int(201)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Netzwerk".to_string(), "netzwerk".to_string()],
    datas: vec![
        vec![PyAtom::Int(25), PyAtom::Int(386), PyAtom::Int(390), PyAtom::Int(55)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["warum_Transzendentalie_=_Strukturgroesse_=_Charakter".to_string(), "warumtranszendentaliezustrukturgroesseundcharakter".to_string()],
    datas: vec![
        vec![PyAtom::Int(165), PyAtom::Int(4), PyAtom::Int(5), PyAtom::Int(54)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Kategorie".to_string(), "kategorie".to_string()],
    datas: vec![
        vec![PyAtom::Int(204), PyAtom::Int(205), PyAtom::Int(281)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Raum-Missionen".to_string(), "weltall".to_string()],
    datas: vec![
        vec![PyAtom::Int(218)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Programmier-Paradigmen".to_string(), "programmierparadigmen".to_string()],
    datas: vec![
        vec![PyAtom::Int(351)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['galaxie']".to_string()],
    parameterNames: vec!["Raum-Missionen".to_string(), "weltall".to_string()],
    datas: vec![
        vec![PyAtom::Int(218)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Geist__(15)".to_string(), "geist".to_string()],
    datas: vec![
        vec![PyAtom::Int(242), PyAtom::Int(426)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["warum_Transzendentalie_=_Komplexität_von_Michael_Commons".to_string(), "warumtranszendentaliegleichkomplexitaet".to_string()],
    datas: vec![
        vec![PyAtom::Int(166), PyAtom::Int(5), PyAtom::Int(65)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Model_of_Hierarchical_Complexity".to_string(), "modelofhierarchicalcomplexity".to_string(), "komplex".to_string(), "komplexität".to_string(), "komplexitaet".to_string(), "complexity".to_string(), "model".to_string(), "abstraktion".to_string()],
    datas: vec![
        vec![PyAtom::Int(203), PyAtom::Int(483), PyAtom::Int(65), PyAtom::Int(75)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Model_of_Hierarchical_Complexity".to_string(), "modelofhierarchicalcomplexity".to_string(), "komplex".to_string(), "komplexität".to_string(), "komplexitaet".to_string(), "complexity".to_string(), "model".to_string(), "abstraktion".to_string()],
    datas: vec![
        vec![PyAtom::Int(203), PyAtom::Int(65), PyAtom::Int(75)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Model_of_Hierarchical_Complexity".to_string(), "modelofhierarchicalcomplexity".to_string(), "komplex".to_string(), "komplexität".to_string(), "komplexitaet".to_string(), "complexity".to_string(), "model".to_string(), "abstraktion".to_string()],
    datas: vec![
        vec![PyAtom::Int(203), PyAtom::Int(65), PyAtom::Int(75)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["2".to_string(), "zwei".to_string(), "gerade".to_string(), "ungerade".to_string(), "alternierung".to_string(), "alternierend".to_string(), "zweierstruktur".to_string()],
    datas: vec![
        vec![PyAtom::Int(331), PyAtom::Int(497), PyAtom::Int(498), PyAtom::Int(499), PyAtom::Int(78), PyAtom::Int(79), PyAtom::Int(80)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["Multiplikation".to_string(), "multiplikation".to_string()],
    datas: vec![
        vec![PyAtom::Int(158)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['operationen']".to_string()],
    parameterNames: vec!["4".to_string(), "vier".to_string(), "viererstruktur".to_string(), "viererabfolgen".to_string()],
    datas: vec![
        vec![PyAtom::Int(104), PyAtom::Int(145), PyAtom::Int(76), PyAtom::Int(77), PyAtom::Int(81)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gesellschaftsschicht".to_string(), "klasse".to_string(), "klassen".to_string()],
    datas: vec![
        vec![PyAtom::Int(241)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Moral".to_string(), "moral".to_string(), "warummoral".to_string()],
    datas: vec![
        vec![PyAtom::Int(215), PyAtom::Int(216)],
        vec![PyAtom::Tuple(vec![PyAtom::Int(216), PyAtom::Int(221)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Fachgebiete".to_string(), "fachgebiete".to_string(), "fachbereiche".to_string(), "themen".to_string()],
    datas: vec![
        vec![PyAtom::Int(183)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Fachgebiete".to_string(), "fachgebiete".to_string(), "fachbereiche".to_string(), "themen".to_string()],
    datas: vec![
        vec![PyAtom::Int(183)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Pflanzen".to_string(), "pflanzen".to_string()],
    datas: vec![
        vec![PyAtom::Int(113)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Maschinen".to_string(), "maschinen".to_string(), "maschine".to_string(), "gerät".to_string(), "geräte".to_string(), "geraete".to_string(), "geraet".to_string()],
    datas: vec![
        vec![PyAtom::Int(89)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Organisationsform".to_string(), "organisationsform".to_string(), "organisationsart".to_string(), "firma".to_string(), "verein".to_string()],
    datas: vec![
        vec![PyAtom::Int(99)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["System".to_string(), "system".to_string()],
    datas: vec![
        vec![PyAtom::Int(440), PyAtom::Int(455), PyAtom::Int(476), PyAtom::Int(513), PyAtom::Int(69), PyAtom::Int(70)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["System".to_string(), "system".to_string()],
    datas: vec![
        vec![PyAtom::Int(440), PyAtom::Int(455), PyAtom::Int(476), PyAtom::Int(513), PyAtom::Int(69), PyAtom::Int(70)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["Erklärung".to_string(), "erklärung".to_string(), "erklaerung".to_string()],
    datas: vec![
        vec![PyAtom::Int(71)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['wirtschaft']".to_string()],
    parameterNames: vec!["BWL".to_string(), "bwl".to_string()],
    datas: vec![
        vec![PyAtom::Int(109)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Sinn_des_Lebens".to_string(), "sinndeslebens".to_string(), "lebenssinn".to_string(), "sinn".to_string(), "sinnsuche".to_string()],
    datas: vec![
        vec![PyAtom::Int(189), PyAtom::Int(88)],
        vec![PyAtom::Tuple(vec![PyAtom::Int(181), PyAtom::Int(182)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Intelligenzprobleme".to_string(), "intelligenzprobleme".to_string(), "intelligenzmaengel".to_string(), "intelligenzmängel".to_string()],
    datas: vec![
        vec![PyAtom::Int(147)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Denkweise_von_Lebewesen".to_string(), "lebewesendenkweise".to_string(), "denkweise".to_string()],
    datas: vec![
        vec![PyAtom::Int(146)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gegentranszendentalien".to_string(), "gegentranszendentalien".to_string(), "gegenstrukturalien".to_string()],
    datas: vec![
        vec![PyAtom::Int(138), PyAtom::Int(139), PyAtom::Int(202)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gleichheit_Freiheit".to_string(), "gleichheitfreiheit".to_string(), "ungleichheit".to_string(), "dominieren".to_string(), "gleichheit".to_string(), "freiheit".to_string()],
    datas: vec![
        vec![PyAtom::Int(132), PyAtom::Int(328), PyAtom::Int(331), PyAtom::Int(335)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gefühle".to_string(), "emotionen".to_string(), "gefuehle".to_string(), "emotion".to_string(), "gefühl".to_string(), "gefuehl".to_string()],
    datas: vec![
        vec![PyAtom::Int(105), PyAtom::Int(230), PyAtom::Int(243), PyAtom::Int(283), PyAtom::Int(284), PyAtom::Int(285), PyAtom::Int(286), PyAtom::Int(305)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Egoismus".to_string(), "egoismus".to_string(), "altruismus".to_string(), "selbstlosigkeit".to_string()],
    datas: vec![
        vec![PyAtom::Int(136)],
        vec![PyAtom::Tuple(vec![PyAtom::Int(66), PyAtom::Int(67)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Wirkung".to_string(), "wirkung".to_string()],
    datas: vec![
        vec![PyAtom::Int(135)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["INCELs".to_string(), "incel".to_string(), "incels".to_string()],
    datas: vec![
        vec![PyAtom::Int(68)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["irrationale_Zahlen_durch_Wurzelbildung".to_string(), "irrationalezahlendurchwurzelbildung".to_string(), "ausgangslage".to_string()],
    datas: vec![
        vec![PyAtom::Int(73)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["dominierendes_Geschlecht".to_string(), "dominierendesgeschlecht".to_string(), "maennlich".to_string(), "männlich".to_string(), "weiblich".to_string()],
    datas: vec![
        vec![PyAtom::Int(51)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Liebe".to_string(), "liebe".to_string(), "ethik".to_string()],
    datas: vec![
        vec![PyAtom::Int(208), PyAtom::Int(28), PyAtom::Int(330), PyAtom::Int(8), PyAtom::Int(9)],
        vec![PyAtom::Tuple(vec![PyAtom::Int(121), PyAtom::Int(122)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Glaube_Erkenntnis".to_string(), "glauben".to_string(), "erkenntnis".to_string(), "glaube".to_string()],
    datas: vec![
        vec![PyAtom::Int(59)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Angreifbarkeit".to_string(), "angreifbarkeit".to_string(), "angreifbar".to_string()],
    datas: vec![
        vec![PyAtom::Int(57), PyAtom::Int(58)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)".to_string(), "Transzendentalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string(), "strukturalien".to_string(), "alien".to_string(), "universalien".to_string(), "meta-paradigmen".to_string()],
    datas: vec![
        vec![PyAtom::Int(131), PyAtom::Int(229), PyAtom::Int(5)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)".to_string(), "Transzendentalien".to_string(), "transzendentalien".to_string(), "transzendentalie".to_string(), "strukturalien".to_string(), "alien".to_string(), "universalien".to_string(), "meta-paradigmen".to_string()],
    datas: vec![
        vec![PyAtom::Int(131), PyAtom::Int(229), PyAtom::Int(5)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bedingung_und_Auslöser_(1/3)".to_string(), "bedingung".to_string(), "bedingungen".to_string(), "auslöser".to_string(), "ausloeser".to_string()],
    datas: vec![
        vec![PyAtom::Int(338)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Lebensbereiche_Problemklassen_(28)".to_string(), "lebensbereiche".to_string(), "lebensfelder".to_string(), "problemklassen".to_string()],
    datas: vec![
        vec![PyAtom::Int(405), PyAtom::Int(415), PyAtom::Int(416)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Maßnahmen_(39)".to_string(), "massnahmen".to_string()],
    datas: vec![
        vec![PyAtom::Int(384)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Relation_zueinander_reziprok_Universellen_(18→n_vs._1/n)".to_string(), "relativreziprokuniversell".to_string()],
    datas: vec![
        vec![PyAtom::Int(350)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["universeller_Komperativ_(18→15)".to_string(), "universellerkomperativ".to_string()],
    datas: vec![
        vec![PyAtom::Int(349)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Existenzialien_(3)".to_string(), "existenzialien".to_string()],
    datas: vec![
        vec![PyAtom::Int(348)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Extremalien_(19)".to_string(), "extremalien".to_string()],
    datas: vec![
        vec![PyAtom::Int(347), PyAtom::Int(352)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Erwartungshaltungen_(26)".to_string(), "erwartungen".to_string(), "erwartungshaltungen".to_string()],
    datas: vec![
        vec![PyAtom::Int(344)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Leidenschaften_(21)".to_string(), "leidenschaft".to_string(), "leidenschaften".to_string()],
    datas: vec![
        vec![PyAtom::Int(343)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["relativer_Zeit-Betrag_(15_10_4_18_6)".to_string(), "relativerzeitbetrag".to_string()],
    datas: vec![
        vec![PyAtom::Int(339)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Zahlenvergleich_(15_18_6)".to_string(), "zahlenvergleich".to_string()],
    datas: vec![
        vec![PyAtom::Int(340)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bestrebungen(1/5)".to_string(), "bestrebung".to_string(), "bestrebungen".to_string()],
    datas: vec![
        vec![PyAtom::Int(332), PyAtom::Int(414)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Prinzipien(1/8)".to_string(), "prinzipien".to_string()],
    datas: vec![
        vec![PyAtom::Int(329), PyAtom::Int(378)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Attraktionen_(36)".to_string(), "attraktionen".to_string()],
    datas: vec![
        vec![PyAtom::Int(311)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Optimierung_(10)".to_string(), "optimierung".to_string()],
    datas: vec![
        vec![PyAtom::Int(310)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Themen_(6)".to_string(), "themen".to_string(), "thema".to_string()],
    datas: vec![
        vec![PyAtom::Int(309)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bedeutung_(10)".to_string(), "bedeutung".to_string()],
    datas: vec![
        vec![PyAtom::Int(306)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Reziprokes".to_string(), "reziproke".to_string(), "reziprokes".to_string()],
    datas: vec![
        vec![PyAtom::Int(131), PyAtom::Int(204), PyAtom::Int(205), PyAtom::Int(231), PyAtom::Int(257), PyAtom::Int(273), PyAtom::Int(281), PyAtom::Int(284), PyAtom::Int(285), PyAtom::Int(326), PyAtom::Int(327), PyAtom::Int(328), PyAtom::Int(329), PyAtom::Int(330), PyAtom::Int(331), PyAtom::Int(332), PyAtom::Int(334), PyAtom::Int(335), PyAtom::Int(338), PyAtom::Int(416), PyAtom::Int(42)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Achtung_(4)".to_string(), "achtung".to_string(), "achten".to_string()],
    datas: vec![
        vec![PyAtom::Int(270), PyAtom::Int(393)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Zeit_(4)_als_Wirklichkeit".to_string(), "zeit".to_string()],
    datas: vec![
        vec![PyAtom::Int(266), PyAtom::Int(267)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_16_ist_zu_genügen".to_string(), "absicht16".to_string()],
    datas: vec![
        vec![PyAtom::Int(312)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_17_ist_zu_meinen".to_string(), "absicht17".to_string()],
    datas: vec![
        vec![PyAtom::Int(263)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_6_ist_Vorteilsmaximierung".to_string(), "absicht6".to_string()],
    datas: vec![
        vec![PyAtom::Int(262)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_7_ist_Selbstlosigkeit".to_string(), "absicht7".to_string()],
    datas: vec![
        vec![PyAtom::Int(261)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Bewusstheit_statt_Bewusstsein_(1)".to_string(), "bewusstheit".to_string()],
    datas: vec![
        vec![PyAtom::Int(282)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Verhalten_(11)".to_string(), "verhalten".to_string()],
    datas: vec![
        vec![PyAtom::Int(301), PyAtom::Int(302), PyAtom::Int(413)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Energie_und_universelle_Eigenschaften_(30)".to_string(), "energie".to_string(), "universelleeigenschaften".to_string(), "lebensenergie".to_string()],
    datas: vec![
        vec![PyAtom::Int(287), PyAtom::Int(293)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Garben_und_Verhalten_nachfühlen(31)".to_string(), "garben".to_string(), "verhaltenfuehlen".to_string(), "verhaltenfühlen".to_string()],
    datas: vec![
        vec![PyAtom::Int(295)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["nachvollziehen_emotional_oder_geistig_durch_Primzahl-Kreuz-Algorithmus_(15)".to_string(), "nachvollziehen".to_string()],
    datas: vec![
        vec![PyAtom::Int(242), PyAtom::Int(297)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primzahlkreuzprocontra".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Empathie_(37)".to_string(), "empathie".to_string(), "mitgefuehl".to_string()],
    datas: vec![
        vec![PyAtom::Int(294)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_1/6_ist_Reinigung_und_Klarheit".to_string(), "absicht1/6".to_string(), "absicht1pro6".to_string()],
    datas: vec![
        vec![PyAtom::Int(298)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["innere_Werte_1/6_der_Reinigung_und_Klarheit".to_string(), "innerewerte".to_string()],
    datas: vec![
        vec![PyAtom::Int(398), PyAtom::Int(399), PyAtom::Int(400), PyAtom::Int(401)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_10_ist_Wirklichkeit_erkennen".to_string(), "absicht10".to_string()],
    datas: vec![
        vec![PyAtom::Int(260)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Wohlbefinden_(7mit6)".to_string(), "wohlbefinden".to_string()],
    datas: vec![
        vec![PyAtom::Int(427), PyAtom::Int(428)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Geist_(15)".to_string(), "geist".to_string(), "bewusstsein".to_string()],
    datas: vec![
        vec![PyAtom::Int(229), PyAtom::Int(231), PyAtom::Int(242), PyAtom::Int(273), PyAtom::Int(297), PyAtom::Int(304), PyAtom::Int(426)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Geist_(15)".to_string(), "geist".to_string(), "bewusstsein".to_string()],
    datas: vec![
        vec![PyAtom::Int(229), PyAtom::Int(231), PyAtom::Int(242), PyAtom::Int(273), PyAtom::Int(297), PyAtom::Int(304), PyAtom::Int(426)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Reflexe_(3)".to_string(), "reflex".to_string(), "reflexe".to_string()],
    datas: vec![
        vec![PyAtom::Int(256)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Lust_(9)".to_string(), "lust".to_string(), "einheiten".to_string()],
    datas: vec![
        vec![PyAtom::Int(255), PyAtom::Int(391)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Paradigmen_sind_Absichten_(13)".to_string(), "paradigmen".to_string(), "absichten".to_string()],
    datas: vec![
        vec![PyAtom::Int(10), PyAtom::Int(410), PyAtom::Int(411), PyAtom::Int(42), PyAtom::Int(493), PyAtom::Int(494)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Wirklichkeiten_Wahrheit_Wahrnehmung_(10)".to_string(), "wirklichkeit".to_string(), "wirklichkeiten".to_string(), "wahrheit".to_string(), "wahrnehmung".to_string()],
    datas: vec![
        vec![PyAtom::Int(233), PyAtom::Int(265), PyAtom::Int(268), PyAtom::Int(322), PyAtom::Int(342), PyAtom::Int(480)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Rechnen".to_string(), "rechnen".to_string()],
    datas: vec![
        vec![PyAtom::Int(404)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Stimmungen_Kombinationen_(14)".to_string(), "stimmung".to_string(), "stimmungen".to_string(), "kombination".to_string(), "kombinationen".to_string()],
    datas: vec![
        vec![PyAtom::Int(290), PyAtom::Int(296), PyAtom::Int(325), PyAtom::Int(326), PyAtom::Int(327), PyAtom::Int(33), PyAtom::Int(402), PyAtom::Int(403), PyAtom::Int(406), PyAtom::Int(407), PyAtom::Int(408), PyAtom::Int(430), PyAtom::Int(492)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Struktur-Wissenschaften_(10)".to_string()],
    datas: vec![
        vec![PyAtom::Int(438)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Muster-Wissenschaften_(20)".to_string()],
    datas: vec![
        vec![PyAtom::Int(439), PyAtom::Int(484)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Anführer_Arten_(7)".to_string()],
    datas: vec![
        vec![PyAtom::Int(429), PyAtom::Int(455), PyAtom::Int(481), PyAtom::Int(482), PyAtom::Int(490), PyAtom::Int(497), PyAtom::Int(498), PyAtom::Int(499), PyAtom::Int(502), PyAtom::Int(509)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Klassen_(20)".to_string(), "klasse".to_string(), "klassen".to_string()],
    datas: vec![
        vec![PyAtom::Int(241), PyAtom::Int(289), PyAtom::Int(394), PyAtom::Int(395), PyAtom::Int(485), PyAtom::Int(516)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Ordnung_und_Filterung_12_und_1pro12".to_string(), "ordnen".to_string(), "ordnenundfiltern".to_string(), "filtern".to_string()],
    datas: vec![
        vec![PyAtom::Int(132), PyAtom::Int(328), PyAtom::Int(331), PyAtom::Int(335)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Meta-Systeme_(12)".to_string(), "metasysteme".to_string(), "metasystem".to_string(), "meta-systeme".to_string(), "meta-system".to_string(), "menge".to_string(), "mengen".to_string()],
    datas: vec![
        vec![PyAtom::Int(232), PyAtom::Int(288), PyAtom::Int(334), PyAtom::Int(410), PyAtom::Int(411), PyAtom::Int(483), PyAtom::Int(497), PyAtom::Int(498), PyAtom::Int(499), PyAtom::Int(79), PyAtom::Int(80)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_1/8".to_string(), "absicht1pro8".to_string(), "absicht1/8".to_string()],
    datas: vec![
        vec![PyAtom::Int(272), PyAtom::Int(379)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Ziele_(19)".to_string(), "ziele".to_string(), "maxima".to_string(), "höhenvorstellungen".to_string()],
    datas: vec![
        vec![PyAtom::Int(271)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Konkreta_und_Focus_(2)".to_string(), "konkreta".to_string(), "focus".to_string(), "fokus".to_string()],
    datas: vec![
        vec![PyAtom::Int(250), PyAtom::Int(253), PyAtom::Int(269)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Gefühle_(7)".to_string(), "gefuehle".to_string(), "emotionen".to_string(), "emotion".to_string(), "gefühle".to_string()],
    datas: vec![
        vec![PyAtom::Int(243), PyAtom::Int(283), PyAtom::Int(284), PyAtom::Int(285), PyAtom::Int(286), PyAtom::Int(29), PyAtom::Int(305)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["abhängige_Verbundenheit_(90)".to_string(), "abhaengigkeit".to_string(), "abhängigkeit".to_string()],
    datas: vec![
        vec![PyAtom::Int(357)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Karte_Filter_und_Unterscheidung_(1/12)".to_string(), "karte".to_string(), "filter".to_string(), "unterscheidung".to_string()],
    datas: vec![
        vec![PyAtom::Int(377)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Fundament_(1/19)".to_string(), "fundament".to_string()],
    datas: vec![
        vec![PyAtom::Int(356)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Gedanken_sind_Positionen_(17)".to_string(), "positionen".to_string(), "gedanken".to_string()],
    datas: vec![
        vec![PyAtom::Int(249), PyAtom::Int(317), PyAtom::Int(323)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Funktionen_Vorstellungen_(16)".to_string(), "vorstellungen".to_string(), "vorstellung".to_string(), "funktionen".to_string()],
    datas: vec![
        vec![PyAtom::Int(264), PyAtom::Int(345), PyAtom::Int(388), PyAtom::Int(418)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Sollen_Frage_Vorgehensweise_(1/13)".to_string(), "sollen".to_string(), "frage".to_string(), "vorgehensweise".to_string()],
    datas: vec![
        vec![PyAtom::Int(353), PyAtom::Int(354)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Ansichten_Standpunkte_(18_17)".to_string(), "ansichten".to_string()],
    datas: vec![
        vec![PyAtom::Int(240), PyAtom::Int(346)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Verbundenheiten_(18)".to_string(), "verbundenheiten".to_string()],
    datas: vec![
        vec![PyAtom::Int(252), PyAtom::Int(299), PyAtom::Int(300), PyAtom::Int(336)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Absicht_13_ist_Helfen".to_string(), "absicht13".to_string(), "helfen".to_string()],
    datas: vec![
        vec![PyAtom::Int(370)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Liebe_(7)".to_string(), "liebe".to_string()],
    datas: vec![
        vec![PyAtom::Int(208), PyAtom::Int(221), PyAtom::Int(28), PyAtom::Int(330), PyAtom::Int(8), PyAtom::Int(9)],
        vec![PyAtom::Tuple(vec![PyAtom::Int(121), PyAtom::Int(122)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Koalitionen_(10)".to_string(), "koalitionen".to_string()],
    datas: vec![
        vec![PyAtom::Int(321)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["gegen_5".to_string()],
    datas: vec![
        vec![PyAtom::Int(24)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Impulse_(5)".to_string(), "impulse".to_string()],
    datas: vec![
        vec![PyAtom::Int(251), PyAtom::Int(253), PyAtom::Int(257), PyAtom::Int(341)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Triebe_und_Bedürfnisse_(6)".to_string(), "trieb".to_string(), "triebe".to_string(), "bedürfnis".to_string(), "bedürfnisse".to_string(), "werte".to_string()],
    datas: vec![
        vec![PyAtom::Int(254), PyAtom::Int(392), PyAtom::Int(396), PyAtom::Int(397), PyAtom::Int(423)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Taetigkeiten".to_string(), "tätigkeiten".to_string(), "taetigkeiten".to_string()],
    datas: vec![
        vec![PyAtom::Int(424)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Reflektion_und_Kategorien_(1/15)".to_string(), "reflektion".to_string(), "kategorien".to_string()],
    datas: vec![
        vec![PyAtom::Int(204), PyAtom::Int(205), PyAtom::Int(281)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Modus_und_Sein_(8)".to_string(), "zustaende".to_string(), "zustände".to_string(), "modus".to_string(), "modi".to_string(), "sein".to_string()],
    datas: vec![
        vec![PyAtom::Int(234), PyAtom::Int(337), PyAtom::Int(385), PyAtom::Int(387), PyAtom::Int(491)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Motive".to_string(), "motive".to_string(), "motivation".to_string(), "motiv".to_string(), "absicht".to_string(), "absichten".to_string()],
    datas: vec![
        vec![PyAtom::Int(10), PyAtom::Int(149), PyAtom::Int(167), PyAtom::Int(168), PyAtom::Int(18), PyAtom::Int(229), PyAtom::Int(230), PyAtom::Int(42)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Gedanken_sind_Positionen_(17)".to_string(), "positionen".to_string(), "gedanken".to_string()],
    datas: vec![
        vec![PyAtom::Int(249), PyAtom::Int(276)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Bewusstsein_und_Wahrnehmung".to_string(), "bewusstsein".to_string(), "wahrnehmung".to_string()],
    datas: vec![
        vec![PyAtom::Int(229), PyAtom::Int(231), PyAtom::Int(265), PyAtom::Int(281), PyAtom::Int(304), PyAtom::Int(342)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Errungenschaften".to_string(), "errungenschaften".to_string(), "ziele".to_string(), "erhalten".to_string()],
    datas: vec![
        vec![PyAtom::Int(11), PyAtom::Int(251), PyAtom::Int(257)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["evolutionär_erwerben_und_Intelligenz_Kreativität".to_string(), "evolutionärerwerbenundintelligenz".to_string(), "intelligenz".to_string(), "erwerben".to_string(), "erlernen".to_string(), "lernen".to_string(), "evolutionaer".to_string(), "evolutionär".to_string(), "kreativität".to_string(), "kreativitaet".to_string(), "kreativ".to_string()],
    datas: vec![
        vec![PyAtom::Int(12), PyAtom::Int(13), PyAtom::Int(27), PyAtom::Int(32), PyAtom::Int(47)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["brauchen".to_string(), "benoetigen".to_string(), "benötigen".to_string(), "notwendig".to_string()],
    datas: vec![
        vec![PyAtom::Int(13), PyAtom::Int(14)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Krankheit".to_string(), "krankheit".to_string(), "krankheiten".to_string(), "pathologisch".to_string(), "pathologie".to_string(), "psychiatrisch".to_string()],
    datas: vec![
        vec![PyAtom::Int(24)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["alpha_beta".to_string(), "alphabeta".to_string(), "alpha".to_string(), "beta".to_string(), "omega".to_string(), "sigma".to_string()],
    datas: vec![
        vec![PyAtom::Int(46)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Anführer".to_string(), "anfuehrer".to_string(), "chef".to_string()],
    datas: vec![
        vec![PyAtom::Int(170), PyAtom::Int(29), PyAtom::Int(429), PyAtom::Int(455), PyAtom::Int(490), PyAtom::Int(502), PyAtom::Int(509)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['grundstrukturen']".to_string()],
    parameterNames: vec!["Biologischer_Baum_(15)".to_string()],
    datas: vec![
        vec![PyAtom::Int(500)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["Biologischer_Baum_(16_->_5)".to_string()],
    datas: vec![
        vec![PyAtom::Int(500)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['universum']".to_string()],
    parameterNames: vec!["Biologischer_Baum_(15)".to_string()],
    datas: vec![
        vec![PyAtom::Int(500)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Biologischer_Baum_(15)".to_string()],
    datas: vec![
        vec![PyAtom::Int(500)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Manipulation".to_string(), "manipulation".to_string()],
    datas: vec![
        vec![PyAtom::Int(153)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Berufe".to_string(), "berufe".to_string(), "beruf".to_string()],
    datas: vec![
        vec![PyAtom::Int(30)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Lösungen".to_string(), "lösungen".to_string(), "loesungen".to_string(), "loesung".to_string(), "lösungen".to_string()],
    datas: vec![
        vec![PyAtom::Int(31)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["Musik".to_string(), "musik".to_string()],
    datas: vec![
        vec![PyAtom::Int(33)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["ergibt_Sinn".to_string(), "ergibtsinn".to_string(), "machtsinn".to_string(), "sinn".to_string()],
    datas: vec![
        vec![PyAtom::Int(140)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Veränderung".to_string(), "veraenderung".to_string(), "veraendern".to_string(), "veränderung".to_string(), "verändern".to_string()],
    datas: vec![
        vec![PyAtom::Int(142)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["bändigen_kontrollieren".to_string(), "baendigenkontrollieren".to_string(), "kontrollieren".to_string(), "baendigen".to_string(), "bändigen".to_string()],
    datas: vec![
        vec![PyAtom::Int(143)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["vereinen".to_string(), "einheit".to_string()],
    datas: vec![
        vec![PyAtom::Int(144)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Vorteile".to_string(), "vorteile".to_string(), "veraenderungnutzen".to_string()],
    datas: vec![
        vec![PyAtom::Int(141)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Gegenspieler".to_string(), "gegenspieler".to_string(), "antagonist".to_string()],
    datas: vec![
        vec![PyAtom::Int(137)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nervig".to_string()],
    datas: vec![
        vec![PyAtom::Int(120)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["pro_nutzen".to_string(), "pronutzen".to_string()],
    datas: vec![
        vec![PyAtom::Int(117)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Gegenposition".to_string(), "gegenposition".to_string()],
    datas: vec![
        vec![PyAtom::Int(116)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Hilfe_erhalten".to_string(), "hilfeerhalten".to_string()],
    datas: vec![
        vec![PyAtom::Int(114)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Helfen".to_string(), "helfen".to_string(), "hilfe".to_string()],
    datas: vec![
        vec![PyAtom::Int(115)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Pro".to_string(), "pro".to_string(), "dafür".to_string(), "dafuer".to_string()],
    datas: vec![
        vec![PyAtom::Int(17), PyAtom::Int(48)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_miteinander_auskommen".to_string(), "nichtauskommen".to_string()],
    datas: vec![
        vec![PyAtom::Int(123)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_dagegen".to_string(), "nichtdagegen".to_string()],
    datas: vec![
        vec![PyAtom::Int(124)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["kein_Gegenteil".to_string(), "keingegenteil".to_string()],
    datas: vec![
        vec![PyAtom::Int(125)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_dafür".to_string(), "nichtdafuer".to_string()],
    datas: vec![
        vec![PyAtom::Int(126)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Hilfe_nicht_gebrauchen".to_string(), "hilfenichtgebrauchen".to_string()],
    datas: vec![
        vec![PyAtom::Int(127)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_helfen_können".to_string(), "nichthelfenkoennen".to_string()],
    datas: vec![
        vec![PyAtom::Int(128)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["nicht_abgeneigt".to_string(), "nichtabgeneigt".to_string()],
    datas: vec![
        vec![PyAtom::Int(129)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["unmotivierbar".to_string()],
    datas: vec![
        vec![PyAtom::Int(130)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["contra".to_string(), "dagegen".to_string()],
    datas: vec![
        vec![PyAtom::Int(15), PyAtom::Int(26)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Gegenteil".to_string(), "gegenteil".to_string()],
    datas: vec![
        vec![PyAtom::Int(100), PyAtom::Int(101), PyAtom::Int(222)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Harmonie".to_string(), "harmonie".to_string()],
    datas: vec![
        vec![PyAtom::Int(102), PyAtom::Int(103)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['licht']".to_string()],
    parameterNames: vec![],
    datas: vec![
        vec![PyAtom::Int(20), PyAtom::Int(27), PyAtom::Int(313)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['procontra']".to_string()],
    parameterNames: vec!["Primzahlkreuz_pro_contra".to_string(), "primzahlkreuz".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primzahlkreuzprocontra".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Primzahlkreuz_pro_contra".to_string(), "primzahlkreuz".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primzahlkreuzprocontra".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["in_ReTa".to_string(), "inreta".to_string()],
    datas: vec![
        vec![PyAtom::Int(209), PyAtom::Int(210), PyAtom::Int(474), PyAtom::Int(475)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Vorzeichen".to_string(), "vorzeichen".to_string()],
    datas: vec![
        vec![PyAtom::Int(118), PyAtom::Int(119)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Primzahlen".to_string(), "primzahlen".to_string(), "vielfache".to_string(), "vielfacher".to_string()],
    datas: vec![
        vec![PyAtom::Int(19)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Anwendung_der_Sonnen_und_Monde".to_string(), "anwendungdersonnenundmonde".to_string(), "anwendungdersonnen".to_string(), "anwendungenfuermonde".to_string()],
    datas: vec![
        vec![PyAtom::Int(22)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Zählungen".to_string(), "zählungen".to_string(), "zaehlung".to_string(), "zaehlungen".to_string(), "zählung".to_string()],
    datas: vec![
        vec![PyAtom::Int(169), PyAtom::Int(188), PyAtom::Int(25), PyAtom::Int(386), PyAtom::Int(390), PyAtom::Int(45)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Jura".to_string(), "jura".to_string(), "gesetzeslehre".to_string(), "recht".to_string()],
    datas: vec![
        vec![PyAtom::Int(34)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Vollkommenheit_des_Geistes".to_string(), "vollkommenheit".to_string(), "geist".to_string()],
    datas: vec![
        vec![PyAtom::Int(35)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Gestirn".to_string(), "gestirn".to_string(), "mond".to_string(), "sonne".to_string(), "planet".to_string()],
    datas: vec![
        vec![PyAtom::Int(154), PyAtom::Int(64)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Konjunktiv_Wurzelbildung".to_string(), "konjunktiv".to_string(), "wurzel".to_string()],
    datas: vec![
        vec![PyAtom::Int(106)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['bedeutung']".to_string()],
    parameterNames: vec!["Mechanismen_der_Züchtung".to_string(), "mechanismen".to_string(), "wesen".to_string(), "zuechtung".to_string(), "züchtung".to_string(), "züchten".to_string(), "zuechten".to_string()],
    datas: vec![
        vec![PyAtom::Int(107), PyAtom::Int(108), PyAtom::Int(109)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['gebrochengalaxie']".to_string()],
    parameterNames: vec!["14".to_string(), "10".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "21".to_string(), "15".to_string(), "9".to_string(), "5".to_string(), "6".to_string(), "13".to_string(), "12".to_string(), "7".to_string(), "22".to_string(), "19".to_string(), "23".to_string(), "4".to_string(), "11".to_string(), "2".to_string(), "3".to_string(), "20".to_string(), "8".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("10".to_string()), PyAtom::Str("11".to_string()), PyAtom::Str("12".to_string()), PyAtom::Str("13".to_string()), PyAtom::Str("14".to_string()), PyAtom::Str("15".to_string()), PyAtom::Str("16".to_string()), PyAtom::Str("17".to_string()), PyAtom::Str("18".to_string()), PyAtom::Str("19".to_string()), PyAtom::Str("2".to_string()), PyAtom::Str("20".to_string()), PyAtom::Str("21".to_string()), PyAtom::Str("22".to_string()), PyAtom::Str("23".to_string()), PyAtom::Str("3".to_string()), PyAtom::Str("4".to_string()), PyAtom::Str("5".to_string()), PyAtom::Str("6".to_string()), PyAtom::Str("7".to_string()), PyAtom::Str("8".to_string()), PyAtom::Str("9".to_string())],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['gebrochenuniversum']".to_string()],
    parameterNames: vec!["14".to_string(), "10".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "21".to_string(), "15".to_string(), "9".to_string(), "5".to_string(), "6".to_string(), "13".to_string(), "12".to_string(), "7".to_string(), "22".to_string(), "19".to_string(), "23".to_string(), "4".to_string(), "11".to_string(), "2".to_string(), "3".to_string(), "20".to_string(), "8".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("10".to_string()), PyAtom::Str("11".to_string()), PyAtom::Str("12".to_string()), PyAtom::Str("13".to_string()), PyAtom::Str("14".to_string()), PyAtom::Str("15".to_string()), PyAtom::Str("16".to_string()), PyAtom::Str("17".to_string()), PyAtom::Str("18".to_string()), PyAtom::Str("19".to_string()), PyAtom::Str("2".to_string()), PyAtom::Str("20".to_string()), PyAtom::Str("21".to_string()), PyAtom::Str("22".to_string()), PyAtom::Str("23".to_string()), PyAtom::Str("3".to_string()), PyAtom::Str("4".to_string()), PyAtom::Str("5".to_string()), PyAtom::Str("6".to_string()), PyAtom::Str("7".to_string()), PyAtom::Str("8".to_string()), PyAtom::Str("9".to_string())],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['gebrochenemotion']".to_string()],
    parameterNames: vec!["14".to_string(), "10".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "21".to_string(), "15".to_string(), "9".to_string(), "5".to_string(), "6".to_string(), "13".to_string(), "12".to_string(), "7".to_string(), "22".to_string(), "19".to_string(), "23".to_string(), "4".to_string(), "11".to_string(), "2".to_string(), "3".to_string(), "20".to_string(), "8".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("10".to_string()), PyAtom::Str("11".to_string()), PyAtom::Str("12".to_string()), PyAtom::Str("13".to_string()), PyAtom::Str("14".to_string()), PyAtom::Str("15".to_string()), PyAtom::Str("16".to_string()), PyAtom::Str("17".to_string()), PyAtom::Str("18".to_string()), PyAtom::Str("19".to_string()), PyAtom::Str("2".to_string()), PyAtom::Str("20".to_string()), PyAtom::Str("21".to_string()), PyAtom::Str("22".to_string()), PyAtom::Str("23".to_string()), PyAtom::Str("3".to_string()), PyAtom::Str("4".to_string()), PyAtom::Str("5".to_string()), PyAtom::Str("6".to_string()), PyAtom::Str("7".to_string()), PyAtom::Str("8".to_string()), PyAtom::Str("9".to_string())],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['gebrochengroesse']".to_string()],
    parameterNames: vec!["14".to_string(), "10".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), "21".to_string(), "15".to_string(), "9".to_string(), "5".to_string(), "6".to_string(), "13".to_string(), "12".to_string(), "7".to_string(), "22".to_string(), "19".to_string(), "23".to_string(), "4".to_string(), "11".to_string(), "2".to_string(), "3".to_string(), "20".to_string(), "8".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("10".to_string()), PyAtom::Str("11".to_string()), PyAtom::Str("12".to_string()), PyAtom::Str("13".to_string()), PyAtom::Str("14".to_string()), PyAtom::Str("15".to_string()), PyAtom::Str("16".to_string()), PyAtom::Str("17".to_string()), PyAtom::Str("18".to_string()), PyAtom::Str("19".to_string()), PyAtom::Str("2".to_string()), PyAtom::Str("20".to_string()), PyAtom::Str("21".to_string()), PyAtom::Str("22".to_string()), PyAtom::Str("23".to_string()), PyAtom::Str("3".to_string()), PyAtom::Str("4".to_string()), PyAtom::Str("5".to_string()), PyAtom::Str("6".to_string()), PyAtom::Str("7".to_string()), PyAtom::Str("8".to_string()), PyAtom::Str("9".to_string())],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Religionen".to_string()],
    datas: vec![
        vec![PyAtom::Int(36), PyAtom::Int(37)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Drei".to_string()],
    datas: vec![
        vec![PyAtom::Int(452), PyAtom::Int(460)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Vier".to_string()],
    datas: vec![
        vec![PyAtom::Int(453)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Fünf".to_string(), "Fuenf".to_string()],
    datas: vec![
        vec![PyAtom::Int(454)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Sechs".to_string()],
    datas: vec![
        vec![PyAtom::Int(457)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Sieben".to_string()],
    datas: vec![
        vec![PyAtom::Int(457)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Acht".to_string()],
    datas: vec![
        vec![PyAtom::Int(458)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Neun".to_string()],
    datas: vec![
        vec![PyAtom::Int(459)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Zehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(456)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['symbole']".to_string()],
    parameterNames: vec!["Zwölf".to_string(), "Zwoelf".to_string()],
    datas: vec![
        vec![PyAtom::Int(456)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Weisheit_etc".to_string(), "weisheit".to_string(), "metaweisheit".to_string(), "meta-weisheit".to_string(), "idiot".to_string(), "weise".to_string(), "optimal".to_string(), "optimum".to_string()],
    datas: vec![
        vec![PyAtom::Int(112)],
        vec![PyAtom::Tuple(vec![PyAtom::Int(40), PyAtom::Int(41)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Dein_Recht_bekommen".to_string(), "rechte".to_string(), "recht".to_string(), "selbstgerecht".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(291), PyAtom::Int(292)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["unterlegen_überlegen".to_string(), "unterlegen".to_string(), "ueberlegen".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(380), PyAtom::Int(381)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Ehrlichkeit_und_Streit".to_string(), "streit".to_string(), "ehrlichkeit".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(375), PyAtom::Int(376)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Würdig".to_string(), "wuerdig".to_string(), "würdig".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(373), PyAtom::Int(374)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Regel_vs_Ausnahme".to_string(), "regel".to_string(), "ausnahme".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(371), PyAtom::Int(372)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Filterart_Widrigkeit".to_string(), "filterart".to_string(), "widrigkeit".to_string()],
    datas: vec![
        vec![PyAtom::Int(331), PyAtom::Int(335)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Werte".to_string(), "werte".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(360), PyAtom::Int(361)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Gutartigkeits-Egoismus".to_string(), "position".to_string(), "gutesreziprok".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(362), PyAtom::Int(363)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Reflektieren_Erkenntnis-Erkennen".to_string(), "reflektieren".to_string(), "erkenntnis".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(364), PyAtom::Int(365)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Vertrauen_wollen".to_string(), "vertrauenwollen".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(366), PyAtom::Int(367)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["einklinken_vertrauen_anprangern".to_string(), "einklinken".to_string(), "vertrauenerhalten".to_string(), "anprangern".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(368), PyAtom::Int(369)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Ausrichten_Einrichten".to_string(), "einrichten".to_string(), "ausrichten".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(358), PyAtom::Int(359)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept2']".to_string()],
    parameterNames: vec!["Toleranz_Respekt_Akzeptanz_Willkommen".to_string(), "toleranz".to_string(), "respekt".to_string(), "akzeptanz".to_string(), "willkommen".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(62), PyAtom::Int(63)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["familiebrauchen".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(279), PyAtom::Int(280)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["ego".to_string(), "bescheiden".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(277), PyAtom::Int(278)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Selbstsucht_Ichsucht_etc".to_string(), "selbstsucht".to_string(), "ichsucht".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(274), PyAtom::Int(275)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Forschen_Erfinden_Einklinken".to_string(), "wissenschaft".to_string(), "forschen".to_string(), "einklinken".to_string(), "erfinden".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(258), PyAtom::Int(259)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Kooperation_vs_Arsch".to_string(), "arschloch".to_string(), "kooperation".to_string(), "arsch".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(245), PyAtom::Int(246)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Liebe_usw".to_string(), "liebe".to_string(), "zuneigung".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(247), PyAtom::Int(248)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Selbstlosigkeit_Ichlosigkeit_etc".to_string(), "selbstlos".to_string(), "ichlos".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(238), PyAtom::Int(239)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["variationsreich_eintönig".to_string(), "eintönig".to_string(), "eintoenig".to_string(), "variationsreich".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(236), PyAtom::Int(237)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Zuneigung_Abneigung".to_string(), "abgeneigt".to_string(), "zugewandt".to_string(), "reserviert".to_string(), "zugeneigt".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(199), PyAtom::Int(200)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['menschliches']".to_string()],
    parameterNames: vec!["ehrlich_vs_höflich".to_string(), "ehrlich".to_string(), "höflich".to_string(), "hoeflich".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(224), PyAtom::Int(225)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["ehrlich_vs_höflich".to_string(), "ehrlich".to_string(), "höflich".to_string(), "hoeflich".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(224), PyAtom::Int(225)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Tragweite".to_string(), "tragweite".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(211), PyAtom::Int(212)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["wertvoll".to_string(), "wertlos".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(186), PyAtom::Int(187)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Götter_Propheten_Familien_Freunde".to_string(), "familiaer".to_string(), "goettlich".to_string(), "freunde".to_string(), "propheten".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(184), PyAtom::Int(185)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["sanft_vs_hart".to_string(), "sanft".to_string(), "hart".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(159), PyAtom::Int(160)]), PyAtom::Tuple(vec![PyAtom::Int(161), PyAtom::Int(162)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["vereinen_vs_verbinden".to_string(), "vereinenverbinden".to_string(), "vereinen".to_string(), "verbinden".to_string(), "einheit".to_string(), "verbindung".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(133), PyAtom::Int(134)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["ähnlich".to_string(), "aehnlich".to_string()],
    datas: vec![
        vec![PyAtom::Int(220)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["gut_böse_lieb_schlecht".to_string(), "gut".to_string(), "böse".to_string(), "boese".to_string(), "lieb".to_string(), "schlecht".to_string()],
    datas: vec![
        vec![PyAtom::Int(52), PyAtom::Int(53)],
        vec![PyAtom::Tuple(vec![PyAtom::Int(38), PyAtom::Int(39)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Sinn_und_Zweck_des_Lebens".to_string(), "sinn".to_string(), "zweck".to_string(), "bedeutung".to_string()],
    datas: vec![
        vec![PyAtom::Int(189), PyAtom::Int(88)],
        vec![PyAtom::Tuple(vec![PyAtom::Int(181), PyAtom::Int(182)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Zeit_vs_Raum".to_string(), "zeit".to_string(), "raum".to_string(), "zeitlich".to_string(), "räumlich".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(49), PyAtom::Int(50)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["egalitär_vs_autoritär".to_string(), "egalitaerautoritaer".to_string(), "egalitaer".to_string(), "autoritaer".to_string(), "egalitär".to_string(), "autoritär".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(163), PyAtom::Int(164)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Meinungen_und_Ruf".to_string(), "meinungen".to_string(), "anderemenschen".to_string(), "ruf".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(60), PyAtom::Int(61)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Meinungsintelligenz".to_string(), "meinungsintelligenz".to_string(), "ursprungsintelligenz".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(151), PyAtom::Int(152)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Sittlichkeit".to_string(), "sittlichkeit".to_string(), "annaehrerung".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(179), PyAtom::Int(180)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Führung".to_string(), "führung".to_string(), "fuehrung".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(173), PyAtom::Int(174)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Durchleuchten".to_string(), "durchleuchten".to_string(), "erleuchten".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(177), PyAtom::Int(178)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Fördern_Sensiblisieren_und_Gedeihen".to_string(), "foerdern".to_string(), "fördern".to_string(), "begrenzen".to_string(), "sensibilisieren".to_string(), "gedeihen".to_string(), "verderben".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(175), PyAtom::Int(176)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Überheblichkeit".to_string(), "überheblich".to_string(), "ueberheblichkeit".to_string(), "ueberheblich".to_string(), "überheblichkeit".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(171), PyAtom::Int(172)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Polung_der_Liebe".to_string(), "liebepolung".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(121), PyAtom::Int(122)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Egoismus_vs_Altruismus".to_string(), "egoismus".to_string(), "altruismus".to_string(), "egoist".to_string(), "altruist".to_string()],
    datas: vec![
        vec![PyAtom::Int(136)],
        vec![PyAtom::Tuple(vec![PyAtom::Int(66), PyAtom::Int(67)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["kausal".to_string(), "geltung".to_string(), "genese".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(110), PyAtom::Int(111)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Gleichheit".to_string(), "gleich".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(192), PyAtom::Int(193)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['konzept']".to_string()],
    parameterNames: vec!["Überleben".to_string(), "ueberleben".to_string()],
    datas: vec![
        vec![],
        vec![PyAtom::Tuple(vec![PyAtom::Int(194), PyAtom::Int(195)])],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec![],
    datas: vec![
        vec![PyAtom::Int(43), PyAtom::Int(54), PyAtom::Int(74), PyAtom::Int(95)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["um1".to_string()],
    datas: vec![
        vec![PyAtom::Int(155)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["um2".to_string()],
    datas: vec![
        vec![PyAtom::Int(156)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["um3".to_string()],
    datas: vec![
        vec![PyAtom::Int(157)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["warum_Transzendentalie_=_Strukturgroesse_=_Charakter".to_string(), "warumtranszendentaliezustrukturgroesseundcharakter".to_string()],
    datas: vec![
        vec![PyAtom::Int(165), PyAtom::Int(4), PyAtom::Int(5), PyAtom::Int(54)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['inkrementieren']".to_string()],
    parameterNames: vec!["warum_Transzendentalie_=_Komplexität_von_Michael_Commons".to_string(), "warumtranszendentaliegleichkomplexitaet".to_string()],
    datas: vec![
        vec![PyAtom::Int(166), PyAtom::Int(5), PyAtom::Int(65)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Rahmen-Bedingungen".to_string(), "rahmen".to_string()],
    datas: vec![
        vec![PyAtom::Int(226)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Motive_gleichförmige_Polygone".to_string(), "motivgleichfoermig".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primMotivGleichf".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Struktur_gleichförmige_Polygone".to_string(), "strukturgleichfoermig".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primStrukGleichf".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Motive_Sternpolygone".to_string(), "motivstern".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primMotivStern".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Struktur_Sternpolygone".to_string(), "strukturstern".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primStrukStern".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Motiv_Sternpolygon_gebrochen-rational".to_string(), "motivgebrstern".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primMotivSternGebr".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Struktur_Sternpolyon_gebrochen-rational".to_string(), "strukgebrstern".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primStrukSternGebr".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Motiv_gleichförmige_Polygone_gebrochen-rational".to_string(), "motivgebrgleichf".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primMotivGleichfGebr".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["Struktur_gleichförmige_Polygone_gebrochen-rational".to_string(), "strukgebrgleichf".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("primStrukGleichfGebr".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['primvielfache']".to_string()],
    parameterNames: vec!["beschrieben".to_string()],
    datas: vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![PyAtom::Str("PrimCSV".to_string())],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["Q".to_string(), "q".to_string(), "Siebzehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(431), PyAtom::Int(432), PyAtom::Int(433), PyAtom::Int(434), PyAtom::Int(437), PyAtom::Int(441), PyAtom::Int(442), PyAtom::Int(443), PyAtom::Int(445), PyAtom::Int(450), PyAtom::Int(467), PyAtom::Int(468), PyAtom::Int(469), PyAtom::Int(487), PyAtom::Int(488)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["i".to_string(), "I".to_string(), "Neun".to_string()],
    datas: vec![
        vec![PyAtom::Int(517)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["G".to_string(), "g".to_string(), "Sieben".to_string()],
    datas: vec![
        vec![PyAtom::Int(518)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["J".to_string(), "j".to_string(), "Zehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(514)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["k".to_string(), "K".to_string(), "Elf".to_string()],
    datas: vec![
        vec![PyAtom::Int(515)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["E".to_string(), "e".to_string(), "Fünf".to_string()],
    datas: vec![
        vec![PyAtom::Int(511)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["L".to_string(), "l".to_string(), "Zwölf".to_string()],
    datas: vec![
        vec![PyAtom::Int(506)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["Y".to_string(), "y".to_string(), "Fünfundzwanzig".to_string()],
    datas: vec![
        vec![PyAtom::Int(507), PyAtom::Int(510)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["Kontinuen".to_string(), "F".to_string(), "f".to_string(), "Sechs".to_string()],
    datas: vec![
        vec![PyAtom::Int(508)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["F".to_string(), "f".to_string(), "Sechs".to_string(), "Kontinuen".to_string()],
    datas: vec![
        vec![PyAtom::Int(508)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["O".to_string(), "o".to_string(), "Fünfzehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(5)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["H".to_string(), "h".to_string(), "Acht".to_string()],
    datas: vec![
        vec![PyAtom::Int(491)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["N".to_string(), "n".to_string(), "Vierzehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(492)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["M".to_string(), "m".to_string(), "Dreizehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(493)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["T".to_string(), "t".to_string(), "Zwanzig".to_string()],
    datas: vec![
        vec![PyAtom::Int(486)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["P".to_string(), "p".to_string(), "Sechszehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(435)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["P5".to_string(), "p5".to_string(), "Sechszehn->Fünf".to_string()],
    datas: vec![
        vec![PyAtom::Int(501)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['multiversum']".to_string()],
    parameterNames: vec!["P5".to_string(), "p5".to_string(), "Sechszehn->Fünf".to_string()],
    datas: vec![
        vec![PyAtom::Int(501)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["P".to_string(), "p".to_string(), "Sechszehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(435)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["X".to_string(), "x".to_string(), "Vierundzwanzig".to_string()],
    datas: vec![
        vec![PyAtom::Int(25), PyAtom::Int(386), PyAtom::Int(436), PyAtom::Int(55)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["S".to_string(), "s".to_string(), "Neunzehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(504)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["R".to_string(), "r".to_string(), "Achtzehn".to_string()],
    datas: vec![
        vec![PyAtom::Int(436), PyAtom::Int(451)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["A".to_string(), "a".to_string(), "Eins".to_string()],
    datas: vec![
        vec![PyAtom::Int(446)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["B".to_string(), "b".to_string(), "Zwei".to_string()],
    datas: vec![
        vec![PyAtom::Int(447)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["C".to_string(), "c".to_string(), "Drei".to_string()],
    datas: vec![
        vec![PyAtom::Int(448)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
    ],
});
        paraNdataMatrix.push(StoreParameterEntry {
    parameterMainNames: vec!["['kontinuum']".to_string()],
    parameterNames: vec!["D".to_string(), "d".to_string(), "Vier".to_string()],
    datas: vec![
        vec![PyAtom::Int(449)],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![]
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
