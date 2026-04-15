#![allow(non_snake_case)]

use indexmap::IndexMap;
use std::collections::{BTreeSet, BTreeMap};
use crate::shared::words_py::PyValue;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PairStr(pub String, pub String);

pub fn dedup_preserve_order_i64(input: Vec<i64>) -> Vec<i64> {
    let mut seen = BTreeSet::new();
    let mut out: Vec<i64> = Vec::new();
    for item in input {
        if !seen.contains(&item) {
            seen.insert(item);
            out.push(item);
        }
    }
    out
}


#[derive(Clone, Debug)]
pub struct SpaltenTyp {
    pub ordinary: (usize, usize),
    pub generated1: (usize, usize),
    pub concat1: (usize, usize),
    pub kombi1: (usize, usize),
    pub boolAndTupleSet1: (usize, usize),
    pub gebroUni1: (usize, usize),
    pub gebrGal1: (usize, usize),
    pub generated2: (usize, usize),
    pub kombi2: (usize, usize),
    pub gebrEmo1: (usize, usize),
    pub gebrGroe1: (usize, usize),
    pub metakonkret: (usize, usize),
    pub ordinaryNot: (usize, usize),
    pub generate1dNot: (usize, usize),
    pub concat1Not: (usize, usize),
    pub kombi1Not: (usize, usize),
    pub boolAndTupleSet1Not: (usize, usize),
    pub gebroUni1Not: (usize, usize),
    pub gebrGal1Not: (usize, usize),
    pub generated2Not: (usize, usize),
    pub kombi2Not: (usize, usize),
    pub gebrEmo1Not: (usize, usize),
    pub gebrGroe1Not: (usize, usize),
    pub metakonkretNot: (usize, usize),
}

impl Default for SpaltenTyp {
    fn default() -> Self {
        Self {
            ordinary: (0, 0),
            generated1: (0, 1),
            concat1: (0, 2),
            kombi1: (0, 3),
            boolAndTupleSet1: (0, 4),
            gebroUni1: (0, 5),
            gebrGal1: (0, 6),
            generated2: (0, 7),
            kombi2: (0, 8),
            gebrEmo1: (0, 9),
            gebrGroe1: (0, 10),
            metakonkret: (0, 11),
            ordinaryNot: (1, 0),
            generate1dNot: (1, 1),
            concat1Not: (1, 2),
            kombi1Not: (1, 3),
            boolAndTupleSet1Not: (1, 4),
            gebroUni1Not: (1, 5),
            gebrGal1Not: (1, 6),
            generated2Not: (1, 7),
            kombi2Not: (1, 8),
            gebrEmo1Not: (1, 9),
            gebrGroe1Not: (1, 10),
            metakonkretNot: (1, 11),
        }
    }
}


#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Generated2Selection {
    pub parameter_main_name: String,
    pub parameter_name: String,
    pub code: String,
}


#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GeneratorPairSelection {
    pub parameter_main_name: String,
    pub parameter_name: String,
    pub left: i64,
    pub right: i64,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub argv: Vec<String>,
    pub argvWithoutProgram: Vec<String>,
    pub paraMainDict: IndexMap<String, Vec<String>>,
    pub paraDict: IndexMap<(String, String), Vec<Vec<PyValue>>>,
    pub dataDicts: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    pub kombiReverseDict: IndexMap<String, i64>,
    pub kombiReverseDict2: IndexMap<String, i64>,
    pub newTable: bool,
    pub allImportantBeginThingsDone: bool,
    pub runDone: bool,
    pub hoechsteZeile: i64,
    pub tableGenerated: bool,
    pub relitable: Vec<Vec<String>>,
    pub RowsLen: i64,
    pub cliErrors: Vec<String>,
    pub mainParas: Vec<String>,
    pub sideParas: Vec<String>,
    pub allesParameters: i64,
    pub __runAlles: bool,
    pub __invertAlles: bool,
    pub __resultingTable: Vec<Vec<String>>,
    pub rowsAsNumbers: Vec<i64>,
    pub breite: i64,
    pub breiten: Vec<i64>,
    pub shellRowsAmount: i64,
    pub shellWidth: i64,
    pub finallyDisplayLines: Vec<String>,
    pub spaltenNumbers: Vec<i64>,
    pub ifPrint: bool,
    pub rowRange: Vec<i64>,
    pub ifZeilenSetToInf: bool,
    pub gebrRatMulStern: bool,
    pub tables: Vec<Vec<Vec<String>>>,
    pub numlen: i64,
    pub old2Rows: Vec<Vec<String>>,
    pub newerTable: Vec<Vec<String>>,
    pub finallyDisplayLinesByChunks: Vec<Vec<String>>,
    pub rowsOfcombi: Vec<Vec<String>>,
    pub oldRows: Vec<Vec<String>>,
    pub newerRows: Vec<Vec<String>>,
    pub oldTable: Vec<Vec<String>>,
    pub generatedSpaltenParameter: Vec<String>,
    pub generatedSpaltenParameter_Tags: BTreeMap<i64, Vec<String>>,
    pub allEquColumns: Vec<i64>,
    pub finallyDisplayTable: Vec<Vec<String>>,
    pub rowsRangeLen: i64,
    pub mainParaCmds: IndexMap<String, i64>,
    pub bigParamaeter: Vec<String>,
    pub __willBeOverwritten_rowsOfcombi: Vec<Vec<String>>,
    pub obZeilenBereicheAngegeben: bool,
    pub breiteHasBeenOnceZero: bool,
    pub breiteORbreiten: bool,
    pub spaltenreihenfolgeundnurdiese: Vec<i64>,
    pub puniverseprims: Vec<i64>,
    pub puniverseprimsNot: Vec<i64>,
    pub generRows: Vec<i64>,
    pub generRowsNot: Vec<i64>,
    pub rowsAsNumbersNot: Vec<i64>,
    pub rowsOfcombiNot: Vec<Vec<String>>,
    pub htmlOrBBcode: bool,
    pub keineleereninhalte: bool,
    pub keineUeberschriften: bool,
    pub nummeriere: bool,
    pub oneTable: bool,
    pub nocolor: bool,
    pub outType: String,
    pub textWidth: i64,
    pub ifZeilenSetted: bool,
    pub dataDict: Vec<IndexMap<String, Vec<Vec<PairStr>>>>,
    pub paraDictGenerated: IndexMap<(String, String), i64>,
    pub paraDictGenerated4htmlTags: IndexMap<(String, String), i64>,
    pub spaltenTypeNaming: SpaltenTyp,
    pub rowsOfcombi2: Vec<i64>,
    pub onlyGenerated: Vec<Vec<i64>>,
    pub getConcat_ones: Vec<i64>,
    pub SpaltenVanillaAmount: i64,
    pub CsvTheirsSpalten: IndexMap<i64, Vec<i64>>,
    pub generated1Pairs: Vec<(i64, i64)>,
    pub generated1Selections: Vec<GeneratorPairSelection>,
    pub generated2Codes: Vec<String>,
    pub generated2Selections: Vec<Generated2Selection>,
    pub boolAndTupleSet1Options: Vec<Option<i64>>,
    pub metakonkretPairs: Vec<(i64, i64)>,
    pub metakonkretSelections: Vec<GeneratorPairSelection>,
    pub spaltenArtenKey_SpaltennummernValue: IndexMap<(usize, usize), BTreeSet<i64>>,
    pub AllSimpleCommandSpalten: Vec<i64>,
    pub lastLineNumber: i64,
}

impl Program {
    pub fn new(argv: Vec<String>) -> Self {
        let argvWithoutProgram = if argv.len() > 1 { argv[1..].to_vec() } else { vec![] };
        let mut dataDicts = vec![];
        for _ in 0..12 {
            dataDicts.push(IndexMap::new());
        }
        Self {
            argv,
            argvWithoutProgram,
            paraMainDict: IndexMap::new(),
            paraDict: IndexMap::new(),
            dataDicts,
            kombiReverseDict: IndexMap::new(),
            kombiReverseDict2: IndexMap::new(),
            newTable: false,
            allImportantBeginThingsDone: false,
            runDone: false,
            hoechsteZeile: 0,
            tableGenerated: false,
            relitable: vec![],
            RowsLen: 0,
            cliErrors: vec![],
            mainParas: vec![
                "-zeilen".to_string(),
                "-spalten".to_string(),
                "-kombination".to_string(),
                "-ausgabe".to_string(),
                "-debug".to_string(),
                "-h".to_string(),
                "-help".to_string(),
            ],
            sideParas: vec![],
            allesParameters: 0,
            __runAlles: false,
            __invertAlles: false,
            __resultingTable: vec![],
            rowsAsNumbers: vec![],
            breite: 0,
            breiten: vec![],
            shellRowsAmount: 0,
            shellWidth: 0,
            finallyDisplayLines: vec![],
            spaltenNumbers: vec![],
            ifPrint: true,
            rowRange: vec![],
            ifZeilenSetToInf: false,
            gebrRatMulStern: false,
            tables: vec![],
            numlen: 0,
            old2Rows: vec![],
            newerTable: vec![],
            finallyDisplayLinesByChunks: vec![],
            rowsOfcombi: vec![],
            oldRows: vec![],
            newerRows: vec![],
            oldTable: vec![],
            generatedSpaltenParameter: vec![],
            generatedSpaltenParameter_Tags: BTreeMap::new(),
            allEquColumns: vec![],
            finallyDisplayTable: vec![],
            rowsRangeLen: 0,
            mainParaCmds: IndexMap::new(),
            bigParamaeter: vec![],
            __willBeOverwritten_rowsOfcombi: vec![],
            obZeilenBereicheAngegeben: false,
            breiteHasBeenOnceZero: false,
            breiteORbreiten: false,
            spaltenreihenfolgeundnurdiese: vec![],
            puniverseprims: vec![],
            puniverseprimsNot: vec![],
            generRows: vec![],
            generRowsNot: vec![],
            rowsAsNumbersNot: vec![],
            rowsOfcombiNot: vec![],
            htmlOrBBcode: false,
            keineleereninhalte: false,
            keineUeberschriften: false,
            nummeriere: true,
            oneTable: false,
            nocolor: false,
            outType: "shell".to_string(),
            textWidth: 0,
            ifZeilenSetted: false,
            dataDict: vec![],
            paraDictGenerated: IndexMap::new(),
            paraDictGenerated4htmlTags: IndexMap::new(),
            spaltenTypeNaming: SpaltenTyp::default(),
            rowsOfcombi2: vec![],
            onlyGenerated: vec![],
            getConcat_ones: vec![],
            SpaltenVanillaAmount: 0,
            CsvTheirsSpalten: IndexMap::new(),
            generated1Pairs: vec![],
            generated1Selections: vec![],
            generated2Codes: vec![],
            generated2Selections: vec![],
            boolAndTupleSet1Options: vec![],
            metakonkretPairs: vec![],
            metakonkretSelections: vec![],
            spaltenArtenKey_SpaltennummernValue: IndexMap::new(),
            AllSimpleCommandSpalten: vec![],
            lastLineNumber: 0,
        }
    }

}
