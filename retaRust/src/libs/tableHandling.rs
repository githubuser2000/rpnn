#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

/*
PY-ARCHITEKTURKNOTEN FÜR libs/tableHandling.py

Diese Datei ist absichtlich keine minimierte Hilfsschicht mehr, sondern eine
py-nahe Kompatibilitätsfassade über den bereits portierten shared-Kern.

Ziel:
- `Tables` als zentraler Python-Knoten bleibt erhalten.
- `getPrepare`, `getOut`, `getCombis`, `getConcat`, `getMainTable` bleiben erhalten.
- bereits sauber portierte shared-Logik wird wiederverwendet.
- dort, wo der Rust-Kern die Python-Details schon zusammengezogen hat,
  wird die Python-Architektur als Fassade rekonstruiert statt doppelt zu bauen.

Wichtig:
- Diese Datei ist bewusst größer als ein bloßer Adapter. Der vorige 12-KB-Stand
  war für diesen Knoten zu klein und zu unvollständig.
- Gleichzeitig wird nicht blind die Python-Datei als String eingefroren. Stattdessen
  wird die aktive Rust-Transpilation an die Python-Klassenstruktur angelehnt.
*/

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};

use indexmap::IndexMap;

use crate::shared::lib4tables_enum_py::ST;
use crate::shared::reta_program_types::{dedup_preserve_order_i64, Program};

pub const PYTHON_SOURCE__TABLE_HANDLING: &str = include_str!("../../python_reference/tableHandling.py");

pub type TxtSink = Rc<RefCell<Vec<String>>>;

#[derive(Debug, Clone, Default)]
pub struct BreakoutException;

impl fmt::Display for BreakoutException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BreakoutException")
    }
}

impl std::error::Error for BreakoutException {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
enum OutputSyntaxKind {
    #[default]
    Shell,
    Nichts,
    Markdown,
    BbCode,
    Html,
    Csv,
    Emacs,
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct OutputSyntax {
    kind: OutputSyntaxKind,
}

impl OutputSyntax {
    pub fn shell() -> Self {
        Self {
            kind: OutputSyntaxKind::Shell,
        }
    }

    pub fn nichts() -> Self {
        Self {
            kind: OutputSyntaxKind::Nichts,
        }
    }

    pub fn markdown() -> Self {
        Self {
            kind: OutputSyntaxKind::Markdown,
        }
    }

    pub fn bbcode() -> Self {
        Self {
            kind: OutputSyntaxKind::BbCode,
        }
    }

    pub fn html() -> Self {
        Self {
            kind: OutputSyntaxKind::Html,
        }
    }

    pub fn csv() -> Self {
        Self {
            kind: OutputSyntaxKind::Csv,
        }
    }

    pub fn emacs() -> Self {
        Self {
            kind: OutputSyntaxKind::Emacs,
        }
    }

    fn from_program_value(value: &str) -> Self {
        match value {
            "nichts" => Self::nichts(),
            "markdown" => Self::markdown(),
            "bbcode" => Self::bbcode(),
            "html" => Self::html(),
            "csv" => Self::csv(),
            "emacs" => Self::emacs(),
            _ => Self::shell(),
        }
    }

    fn as_program_value(&self) -> &'static str {
        match self.kind {
            OutputSyntaxKind::Shell => "shell",
            OutputSyntaxKind::Nichts => "nichts",
            OutputSyntaxKind::Markdown => "markdown",
            OutputSyntaxKind::BbCode => "bbcode",
            OutputSyntaxKind::Html => "html",
            OutputSyntaxKind::Csv => "csv",
            OutputSyntaxKind::Emacs => "emacs",
        }
    }

    fn html_like(&self) -> bool {
        matches!(self.kind, OutputSyntaxKind::Html | OutputSyntaxKind::BbCode)
    }
}

#[derive(Clone, Debug, Default)]
pub struct NichtsSyntax;
#[derive(Clone, Debug, Default)]
pub struct markdownSyntax;
#[derive(Clone, Debug, Default)]
pub struct bbCodeSyntax;
#[derive(Clone, Debug, Default)]
pub struct htmlSyntax;
#[derive(Clone, Debug, Default)]
pub struct csvSyntax;
#[derive(Clone, Debug, Default)]
pub struct emacsSyntax;

impl From<NichtsSyntax> for OutputSyntax {
    fn from(_: NichtsSyntax) -> Self {
        OutputSyntax::nichts()
    }
}
impl From<markdownSyntax> for OutputSyntax {
    fn from(_: markdownSyntax) -> Self {
        OutputSyntax::markdown()
    }
}
impl From<bbCodeSyntax> for OutputSyntax {
    fn from(_: bbCodeSyntax) -> Self {
        OutputSyntax::bbcode()
    }
}
impl From<htmlSyntax> for OutputSyntax {
    fn from(_: htmlSyntax) -> Self {
        OutputSyntax::html()
    }
}
impl From<csvSyntax> for OutputSyntax {
    fn from(_: csvSyntax) -> Self {
        OutputSyntax::csv()
    }
}
impl From<emacsSyntax> for OutputSyntax {
    fn from(_: emacsSyntax) -> Self {
        OutputSyntax::emacs()
    }
}

pub static shellRowsAmount: AtomicI64 = AtomicI64::new(0);

pub fn setShellRowsAmount(value: i64) {
    shellRowsAmount.store(value.max(0), Ordering::Relaxed);
}

pub fn getShellRowsAmount() -> i64 {
    shellRowsAmount.load(Ordering::Relaxed)
}

type SharedTablesState = Rc<RefCell<TablesState>>;

#[derive(Clone, Debug)]
struct TablesState {
    program: Program,
    hoechsteZeile: BTreeMap<i64, i64>,
    textHeight: i64,
    rowNumDisplay2rowNumOrig: IndexMap<i64, i64>,
    religionNumbers: Vec<i64>,
    ifprimmultis: bool,
    concat_ones: Vec<i64>,
    sumOfAllCombiRowsAmount: usize,
}

impl TablesState {
    fn new(hoechstZeil: Option<i64>) -> Self {
        let mut program = Program::new(vec!["reta".to_string()]);
        let hz = hoechstZeil.unwrap_or(1024);
        program.hoechsteZeile = hz;
        program.textWidth = 21;
        program.nummeriere = true;
        program.outType = "shell".to_string();
        program.shellRowsAmount = getShellRowsAmount();
        Self {
            program,
            hoechsteZeile: BTreeMap::from([(1024, hz), (114, hoechstZeil.unwrap_or(163))]),
            textHeight: 0,
            rowNumDisplay2rowNumOrig: IndexMap::new(),
            religionNumbers: vec![],
            ifprimmultis: false,
            concat_ones: vec![],
            sumOfAllCombiRowsAmount: 0,
        }
    }
}

fn detected_shell_width(program: &Program) -> i64 {
    let global = getShellRowsAmount();
    if global > 0 {
        global
    } else if program.shellWidth > 0 {
        program.shellWidth
    } else if program.shellRowsAmount > 0 {
        program.shellRowsAmount
    } else {
        0
    }
}

fn normalize_text_width(program: &Program, requested: i64) -> i64 {
    let shell_width = detected_shell_width(program);
    let out_type = OutputSyntax::from_program_value(&program.outType);
    let zero_allowed = requested == 0 && (out_type.html_like() || program.oneTable);
    if (shell_width > requested + 7 || shell_width == 0) && (requested != 0 || zero_allowed) {
        requested
    } else if shell_width > 7 {
        shell_width - 7
    } else {
        requested.max(0)
    }
}

fn normalize_breiten(program: &Program, values: Vec<i64>) -> Vec<i64> {
    let shell_width = detected_shell_width(program);
    values
        .into_iter()
        .map(|value| {
            if shell_width > value + 7 || shell_width == 0 {
                value
            } else if shell_width > 7 {
                shell_width - 7
            } else {
                value.max(0)
            }
        })
        .collect()
}

fn clamp_cell_height(table: Vec<Vec<String>>, text_height: i64) -> Vec<Vec<String>> {
    if text_height <= 0 {
        return table;
    }
    let keep = text_height as usize;
    table
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| {
                    let parts: Vec<&str> = cell.split('\n').take(keep).collect();
                    parts.join("\n")
                })
                .collect()
        })
        .collect()
}

fn sync_program_runtime_fields(state: &mut TablesState) {
    state.program.hoechsteZeile = *state.hoechsteZeile.get(&1024).unwrap_or(&state.program.hoechsteZeile);
    state.program.shellRowsAmount = getShellRowsAmount();
}

fn update_program_with_relitable_and_rows<R, F>(
    state: &SharedTablesState,
    relitable: Vec<Vec<String>>,
    rowsAsNumbers: Vec<i64>,
    f: F,
) -> (Vec<Vec<String>>, Vec<i64>, R)
where
    F: FnOnce(&mut Program, &mut Vec<i64>) -> R,
{
    let mut guard = state.borrow_mut();
    sync_program_runtime_fields(&mut guard);
    guard.program.relitable = relitable;
    let mut rows = rowsAsNumbers;
    let extra = f(&mut guard.program, &mut rows);
    let relitable_out = guard.program.relitable.clone();
    (relitable_out, rows, extra)
}

#[derive(Clone, Debug)]
pub struct Tables {
    state: SharedTablesState,
    pub getPrepare: TablesPrepare,
    pub getCombis: TablesCombi,
    pub getConcat: TablesConcat,
    pub getOut: TablesOutput,
    pub getMainTable: TablesMaintable,
}

impl Tables {
    pub fn new(hoechstZeil: Option<i64>, Txt: Option<TxtSink>) -> Self {
        let state = Rc::new(RefCell::new(TablesState::new(hoechstZeil)));
        let getPrepare = TablesPrepare::new(state.clone());
        let getCombis = TablesCombi::new(state.clone());
        let getConcat = TablesConcat::new(state.clone());
        let getOut = TablesOutput::new(state.clone(), Txt);
        let getMainTable = TablesMaintable::new(state.clone());
        let tables = Self {
            state,
            getPrepare,
            getCombis,
            getConcat,
            getOut,
            getMainTable,
        };
        tables.set_textHeight(0);
        tables.set_textWidth(21);
        tables.set_nummeriere(true);
        tables.set_breitenn(vec![]);
        tables
    }

    pub fn NichtsOutputYes(&self) -> bool {
        self.outType() == OutputSyntax::nichts()
    }

    pub fn markdownOutputYes(&self) -> bool {
        self.outType() == OutputSyntax::markdown()
    }

    pub fn bbcodeOutputYes(&self) -> bool {
        self.outType() == OutputSyntax::bbcode()
    }

    pub fn htmlOutputYes(&self) -> bool {
        self.outType() == OutputSyntax::html()
    }

    pub fn outType(&self) -> OutputSyntax {
        self.getOut.outType()
    }

    pub fn set_outType<T: Into<OutputSyntax>>(&self, value: T) {
        self.getOut.set_outType(value.into());
    }

    pub fn hoechsteZeile(&self) -> BTreeMap<i64, i64> {
        self.state.borrow().hoechsteZeile.clone()
    }

    pub fn set_hoechsteZeile(&self, value: i64) {
        let mut state = self.state.borrow_mut();
        state.hoechsteZeile = BTreeMap::from([(1024, value), (114, value)]);
        state.program.hoechsteZeile = value;
    }

    pub fn generRows(&self) -> Vec<i64> {
        self.state.borrow().program.generRows.clone()
    }

    pub fn set_generRows(&self, value: Vec<i64>) {
        self.state.borrow_mut().program.generRows = dedup_preserve_order_i64(value);
    }

    pub fn ifPrimMultis(&self) -> bool {
        self.state.borrow().ifprimmultis
    }

    pub fn set_ifPrimMultis(&self, value: bool) {
        self.state.borrow_mut().ifprimmultis = value;
    }

    pub fn ifZeilenSetted(&self) -> bool {
        self.state.borrow().program.ifZeilenSetted
    }

    pub fn set_ifZeilenSetted(&self, value: bool) {
        self.state.borrow_mut().program.ifZeilenSetted = value;
    }

    pub fn gebrUnivSet(&self) -> Vec<i64> {
        self.state.borrow().program.puniverseprims.clone()
    }

    pub fn breitenn(&self) -> Vec<i64> {
        self.state.borrow().program.breiten.clone()
    }

    pub fn set_breitenn(&self, value: Vec<i64>) {
        let mut state = self.state.borrow_mut();
        state.program.breiten = normalize_breiten(&state.program, value);
    }

    pub fn nummeriere(&self) -> bool {
        self.state.borrow().program.nummeriere
    }

    pub fn set_nummeriere(&self, value: bool) {
        self.state.borrow_mut().program.nummeriere = value;
    }

    pub fn textHeight(&self) -> i64 {
        self.state.borrow().textHeight
    }

    pub fn set_textHeight(&self, value: i64) {
        self.state.borrow_mut().textHeight = value.max(0);
    }

    pub fn textWidth(&self) -> i64 {
        self.state.borrow().program.textWidth
    }

    pub fn set_textWidth(&self, value: i64) {
        let mut state = self.state.borrow_mut();
        state.program.textWidth = normalize_text_width(&state.program, value.max(0));
    }

    pub fn rowNumDisplay2rowNumOrig(&self) -> IndexMap<i64, i64> {
        self.state.borrow().rowNumDisplay2rowNumOrig.clone()
    }

    pub fn religionNumbers(&self) -> Vec<i64> {
        self.state.borrow().religionNumbers.clone()
    }

    pub fn set_religionNumbers(&self, value: Vec<i64>) {
        self.state.borrow_mut().religionNumbers = value;
    }

    pub fn SpaltenVanillaAmount(&self) -> i64 {
        self.state.borrow().program.SpaltenVanillaAmount
    }

    pub fn set_SpaltenVanillaAmount(&self, value: i64) {
        self.state.borrow_mut().program.SpaltenVanillaAmount = value;
    }

    pub fn generatedSpaltenParameter(&self) -> Vec<String> {
        self.state.borrow().program.generatedSpaltenParameter.clone()
    }

    pub fn generatedSpaltenParameter_Tags(&self) -> BTreeMap<i64, BTreeSet<ST>> {
        self.state.borrow().program.generatedSpaltenParameter_Tags.clone()
    }

    pub fn fillBoth<T: Clone>(mut liste1: Vec<T>, mut liste2: Vec<T>, filler: T) -> (Vec<T>, Vec<T>) {
        while liste1.len() < liste2.len() {
            liste1.push(filler.clone());
        }
        while liste2.len() < liste1.len() {
            liste2.push(filler.clone());
        }
        (liste1, liste2)
    }

    pub fn tableReducedInLinesByTypeSet<T: Clone>(&self, table: Vec<T>, linesAllowed: BTreeSet<usize>) -> Vec<T> {
        table
            .into_iter()
            .enumerate()
            .filter_map(|(i, line)| if linesAllowed.contains(&i) { Some(line) } else { None })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct TablesPrepare {
    state: SharedTablesState,
}

impl TablesPrepare {
    fn new(state: SharedTablesState) -> Self {
        Self { state }
    }

    pub fn ifprimmultis(&self) -> bool {
        self.state.borrow().ifprimmultis
    }

    pub fn set_ifprimmultis(&self, value: bool) {
        self.state.borrow_mut().ifprimmultis = value;
    }

    pub fn ifZeilenSetted(&self) -> bool {
        self.state.borrow().program.ifZeilenSetted
    }

    pub fn set_ifZeilenSetted(&self, value: bool) {
        self.state.borrow_mut().program.ifZeilenSetted = value;
    }

    pub fn breitenn(&self) -> Vec<i64> {
        self.state.borrow().program.breiten.clone()
    }

    pub fn set_breitenn(&self, value: Vec<i64>) {
        let mut state = self.state.borrow_mut();
        state.program.breiten = normalize_breiten(&state.program, value);
    }

    pub fn nummerierung(&self) -> bool {
        self.state.borrow().program.nummeriere
    }

    pub fn set_nummerierung(&self, value: bool) {
        self.state.borrow_mut().program.nummeriere = value;
    }

    pub fn textWidth(&self) -> i64 {
        self.state.borrow().program.textWidth
    }

    pub fn set_textWidth(&self, value: i64) {
        let mut state = self.state.borrow_mut();
        state.program.textWidth = normalize_text_width(&state.program, value.max(0));
    }

    pub fn rowsAsNumbers(&self) -> Vec<i64> {
        self.state.borrow().program.rowsAsNumbers.clone()
    }

    pub fn set_rowsAsNumbers(&self, value: Vec<i64>) {
        self.state.borrow_mut().program.rowsAsNumbers = dedup_preserve_order_i64(value);
    }

    pub fn deleteDoublesInSets(&self, pos: Vec<String>, neg: Vec<String>) -> (Vec<String>, Vec<String>) {
        self.state.borrow().program.deleteDoublesInSets_py(pos, neg)
    }

    pub fn parametersCmdWithSomeBereich(
        &self,
        txt: &str,
        suffix: &str,
        neg: &str,
        keineNegBeruecksichtigung: bool,
    ) -> Vec<String> {
        self.state
            .borrow()
            .program
            .parametersCmdWithSomeBereich_py(txt, suffix, neg, keineNegBeruecksichtigung)
    }

    pub fn prepare4out_beforeForLoop_SpaltenZeilenBestimmen(
        &self,
        relitable: Vec<Vec<String>>,
        paramLines: Vec<String>,
        paramLinesNot: Vec<String>,
    ) -> (Vec<String>, i64, Vec<Vec<String>>, i64, Vec<i64>) {
        let mut state = self.state.borrow_mut();
        sync_program_runtime_fields(&mut state);
        state.program.relitable = relitable.clone();
        let rows_as_numbers = state.program.rowsAsNumbers.clone();
        let (finallyDisplayLines, newTable, numlen, rowsRange, _old2new) =
            state
                .program
                .prepare4out_py(paramLines, paramLinesNot, relitable.clone(), rows_as_numbers);
        let headingsAmount = relitable.first().map(|row| row.len()).unwrap_or(0) as i64;
        (finallyDisplayLines, headingsAmount, newTable, numlen, rowsRange)
    }

    pub fn prepare4out(
        &self,
        paramLines: Vec<String>,
        paramLinesNot: Vec<String>,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
        _gebrSpalten: Option<BTreeMap<String, Vec<i64>>>,
        _primSpalten: Option<Vec<i64>>,
        _sumOfAllCombiRowsAmount: Option<usize>,
        _reliTableLenUntilNow: Option<usize>,
        _kombiCSVNumber: Option<i64>,
    ) -> (Vec<String>, Vec<Vec<String>>, i64, Vec<i64>, Vec<i64>) {
        let mut state = self.state.borrow_mut();
        sync_program_runtime_fields(&mut state);
        state.program.relitable = relitable.clone();
        state.program.rowsAsNumbers = dedup_preserve_order_i64(rowsAsNumbers.clone());
        state
            .program
            .prepare4out_py(paramLines, paramLinesNot, relitable, rowsAsNumbers)
    }
}

#[derive(Clone, Debug)]
pub struct TablesOutput {
    state: SharedTablesState,
    resultingTable: Rc<RefCell<Vec<String>>>,
    Txt: Option<TxtSink>,
    color: Rc<RefCell<bool>>,
}

impl TablesOutput {
    fn new(state: SharedTablesState, Txt: Option<TxtSink>) -> Self {
        Self {
            state,
            resultingTable: Rc::new(RefCell::new(vec![])),
            Txt,
            color: Rc::new(RefCell::new(true)),
        }
    }

    pub fn outType(&self) -> OutputSyntax {
        let state = self.state.borrow();
        OutputSyntax::from_program_value(&state.program.outType)
    }

    pub fn set_outType(&self, value: OutputSyntax) {
        self.state.borrow_mut().program.outType = value.as_program_value().to_string();
    }

    pub fn color(&self) -> bool {
        *self.color.borrow()
    }

    pub fn set_color(&self, value: bool) {
        *self.color.borrow_mut() = value;
        self.state.borrow_mut().program.nocolor = !value;
    }

    pub fn oneTable(&self) -> bool {
        self.state.borrow().program.oneTable
    }

    pub fn set_oneTable(&self, value: bool) {
        self.state.borrow_mut().program.oneTable = value;
    }

    pub fn breitenn(&self) -> Vec<i64> {
        self.state.borrow().program.breiten.clone()
    }

    pub fn set_breitenn(&self, value: Vec<i64>) {
        let mut state = self.state.borrow_mut();
        state.program.breiten = normalize_breiten(&state.program, value);
    }

    pub fn nummeriere(&self) -> bool {
        self.state.borrow().program.nummeriere
    }

    pub fn set_nummeriere(&self, value: bool) {
        self.state.borrow_mut().program.nummeriere = value;
    }

    pub fn textHeight(&self) -> i64 {
        self.state.borrow().textHeight
    }

    pub fn set_textHeight(&self, value: i64) {
        self.state.borrow_mut().textHeight = value.max(0);
    }

    pub fn textWidth(&self) -> i64 {
        self.state.borrow().program.textWidth
    }

    pub fn set_textWidth(&self, value: i64) {
        let mut state = self.state.borrow_mut();
        state.program.textWidth = normalize_text_width(&state.program, value.max(0));
    }

    pub fn rowsAsNumbers(&self) -> Vec<i64> {
        self.state.borrow().program.rowsAsNumbers.clone()
    }

    pub fn set_rowsAsNumbers(&self, value: Vec<i64>) {
        self.state.borrow_mut().program.rowsAsNumbers = dedup_preserve_order_i64(value);
    }

    pub fn resultingTable(&self) -> Vec<String> {
        self.resultingTable.borrow().clone()
    }

    pub fn onlyThatColumns(&self, table: Vec<Vec<String>>, onlyThatColumns: Vec<i64>) -> Vec<Vec<String>> {
        self.state.borrow().program.onlyThatColumns_py(table, onlyThatColumns)
    }

    pub fn cliOut(
        &self,
        finallyDisplayLinesSet: Vec<String>,
        newTable: Vec<Vec<String>>,
        numlen: i64,
        rowsRange: Vec<i64>,
    ) -> Vec<Vec<String>> {
        let clipped_table = clamp_cell_height(newTable, self.textHeight());
        let (result, lines) = {
            let mut state = self.state.borrow_mut();
            sync_program_runtime_fields(&mut state);
            state.program.nocolor = !self.color();
            let result = state
                .program
                .cliOut_py(finallyDisplayLinesSet, clipped_table, numlen, rowsRange);
            let lines = state.program.finallyDisplayLines.clone();
            (result, lines)
        };
        *self.resultingTable.borrow_mut() = lines.clone();
        if let Some(txt) = &self.Txt {
            *txt.borrow_mut() = lines;
        }
        result
    }

    pub fn cliout2(&self, text: impl Into<String>) {
        let text = text.into();
        self.resultingTable.borrow_mut().push(text.clone());
        if let Some(txt) = &self.Txt {
            txt.borrow_mut().push(text);
        }
    }

    pub fn colorize(&self, text: &str, num: i64, rest: bool) -> String {
        let nocolor = !self.color();
        Program::styled_shell_text_py(text, Some(num), num == 0, rest, nocolor)
    }
}

#[derive(Clone, Debug, Default)]
pub struct PreparedTableJoin {
    pub chosen_lines: BTreeMap<i64, BTreeSet<i64>>,
    pub prepared_kombi_table: Vec<Vec<String>>,
}

pub type MainToSubRelation = (IndexMap<i64, i64>, IndexMap<i64, i64>);

#[derive(Clone, Debug)]
pub struct TablesCombi {
    state: SharedTablesState,
}

impl TablesCombi {
    fn new(state: SharedTablesState) -> Self {
        Self { state }
    }

    pub fn rowsOfcombi(&self) -> Vec<i64> {
        self.state.borrow().program.rowsOfcombi2.clone()
    }

    pub fn set_rowsOfcombi(&self, value: Vec<i64>) {
        self.state.borrow_mut().program.rowsOfcombi2 = dedup_preserve_order_i64(value);
    }

    pub fn sumOfAllCombiRowsAmount(&self) -> usize {
        self.state.borrow().sumOfAllCombiRowsAmount
    }

    pub fn prepareTableJoin(
        &self,
        ChosenKombiLines: BTreeMap<i64, BTreeSet<i64>>,
        newTable_kombi_1: Vec<Vec<String>>,
    ) -> PreparedTableJoin {
        PreparedTableJoin {
            chosen_lines: ChosenKombiLines,
            prepared_kombi_table: newTable_kombi_1,
        }
    }

    pub fn prepare_kombi(
        &self,
        _finallyDisplayLines_kombi_1: Vec<String>,
        _kombiTable: Vec<Vec<String>>,
        paramLines: Vec<String>,
        displayingZeilen: Vec<String>,
        kombiTable_Kombis: Vec<Vec<i64>>,
    ) -> BTreeMap<i64, BTreeSet<i64>> {
        let has_kombi_trigger = paramLines.iter().any(|condition| condition == "ka" || condition == "ka2");
        if !has_kombi_trigger {
            return BTreeMap::new();
        }
        let displaying: BTreeSet<i64> = displayingZeilen
            .iter()
            .filter_map(|value| value.trim().parse::<i64>().ok())
            .collect();

        let mut chosen: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
        for (kombiLineNumber, kombiLine) in kombiTable_Kombis.iter().enumerate() {
            for kombiNumber in kombiLine {
                if displaying.contains(kombiNumber) {
                    chosen
                        .entry(*kombiNumber)
                        .or_default()
                        .insert((kombiLineNumber + 1) as i64);
                }
            }
        }
        chosen
    }

    pub fn prepare4out_kombi_table(&self, kombiTable: Vec<Vec<String>>, rowsOfcombi: Vec<i64>) -> Vec<Vec<String>> {
        let selected_cols = dedup_preserve_order_i64(rowsOfcombi);
        let state = self.state.borrow();
        kombiTable
            .into_iter()
            .map(|line| {
                let mut new2Lines: Vec<String> = vec![];
                let mut rowToDisplay: usize = 0;
                for t in selected_cols.iter().copied() {
                    let idx = t as usize;
                    let cell = line.get(idx).cloned().unwrap_or_default();
                    rowToDisplay += 1;
                    let certaintextwidth = if rowToDisplay >= 1 && rowToDisplay - 1 < state.program.breiten.len() {
                        state.program.breiten[rowToDisplay - 1]
                    } else {
                        state.program.textWidth
                    };
                    let into = if certaintextwidth <= 0 {
                        vec![cell.trim().to_string()]
                    } else {
                        Program::wrap_text_py(cell.trim(), certaintextwidth as usize)
                    };
                    new2Lines.push(into.join("\n"));
                }
                new2Lines
            })
            .collect()
    }

    pub fn removeOneNumber(&self, hinein: Vec<String>, colNum: i64) -> Vec<String> {
        let state = self.state.borrow();
        let text_width = state.program.textWidth.max(0) as usize;
        let mut text = hinein.join("");
        while text.ends_with('-') {
            text.pop();
        }

        let open_pos = match text.find('(') {
            Some(v) => v,
            None => {
                return if text_width > 0 {
                    text.split('\n').map(|s| s.to_string()).collect::<Vec<String>>()
                } else {
                    vec![text.replace('\n', "; ")]
                };
            }
        };
        let close_rel = match text[open_pos..].find(") ") {
            Some(v) => v,
            None => {
                return if text_width > 0 {
                    text.split('\n').map(|s| s.to_string()).collect::<Vec<String>>()
                } else {
                    vec![text.replace('\n', "; ")]
                };
            }
        };
        let close_pos = open_pos + close_rel;
        let inside = &text[(open_pos + 1)..close_pos];
        let target_plain = colNum.to_string();
        let target_paren = format!("({})", colNum);
        let kept_parts: Vec<&str> = inside
            .split('|')
            .filter(|part| {
                let p = part.trim();
                !(p == target_plain || p == target_paren)
            })
            .collect();
        let rebuilt_inside = kept_parts.join("|");
        let mut rebuilt = String::new();
        rebuilt.push_str(&text[..open_pos + 1]);
        rebuilt.push_str(&rebuilt_inside);
        rebuilt.push_str(&text[close_pos..]);
        let rebuilt = rebuilt
            .replace("(|", "(")
            .replace("|)", ")")
            .replace("||", "|");
        if text_width > 0 {
            rebuilt.split('\n').map(|s| s.to_string()).collect::<Vec<String>>()
        } else {
            vec![rebuilt.replace('\n', "; ")]
        }
    }

    pub fn tableJoin(
        &self,
        mut mainTable: Vec<Vec<String>>,
        prepared: PreparedTableJoin,
        maintable2subtable_Relation: MainToSubRelation,
        old2newTable: Vec<i64>,
        rowsOfcombi: Vec<i64>,
        finallyDisplayLines: Vec<String>,
        paramLines: Vec<String>,
        csvFileName: &str,
        output_column_origins: Vec<i64>,
    ) -> Vec<Vec<String>> {
        let kind = if csvFileName.contains("meta") { "ka2" } else { "ka" };
        if !paramLines.iter().any(|p| p == kind) {
            return mainTable;
        }

        let mut output_to_preparedcol: BTreeMap<usize, usize> = BTreeMap::new();
        for (out_idx, orig_col) in output_column_origins.iter().copied().enumerate() {
            if let Some(sub_idx) = maintable2subtable_Relation.0.get(&orig_col) {
                let wanted_csv_col = *sub_idx + 1;
                if let Some(pos) = rowsOfcombi.iter().position(|v| *v == wanted_csv_col) {
                    output_to_preparedcol.insert(out_idx, pos);
                }
            }
        }
        if output_to_preparedcol.is_empty() {
            return mainTable;
        }

        let state = self.state.borrow();
        let oneLinePerLine = state.program.outType == "html" || state.program.outType == "bbcode";
        let remove_number_now =
            ((state.program.textWidth == 0 && state.program.oneTable)
                || state.program.outType == "html"
                || state.program.outType == "bbcode")
                && state.program.breiten.is_empty();
        drop(state);

        for (display_row_idx, original_row_no) in old2newTable.iter().copied().enumerate() {
            if display_row_idx >= mainTable.len() {
                continue;
            }
            let original_row_label_matches = finallyDisplayLines
                .get(display_row_idx)
                .and_then(|value| value.trim().parse::<i64>().ok())
                .map(|value| value == original_row_no)
                .unwrap_or(false);
            if !original_row_label_matches && original_row_no != 0 {
                continue;
            }
            let Some(kombi_line_numbers) = prepared.chosen_lines.get(&original_row_no) else {
                continue;
            };

            for (out_col_idx, prepared_col_idx) in output_to_preparedcol.iter() {
                if *out_col_idx >= mainTable[display_row_idx].len() {
                    continue;
                }

                let mut teile: Vec<String> = vec![];
                for kombi_line_no in kombi_line_numbers.iter().copied() {
                    let src_row_idx = kombi_line_no as usize;
                    if src_row_idx >= prepared.prepared_kombi_table.len() {
                        continue;
                    }
                    let raw_prepared = prepared.prepared_kombi_table[src_row_idx]
                        .get(*prepared_col_idx)
                        .cloned()
                        .unwrap_or_default();
                    if raw_prepared.trim().is_empty() {
                        continue;
                    }
                    let block = if remove_number_now {
                        let raw_lines: Vec<String> = raw_prepared.split('\n').map(|s| s.to_string()).collect();
                        self.removeOneNumber(raw_lines, original_row_no).join("\n")
                    } else {
                        raw_prepared
                    };
                    if !block.trim().is_empty() {
                        teile.push(block);
                    }
                }

                if teile.is_empty() {
                    continue;
                }

                let merged = {
                    let state = self.state.borrow();
                    if oneLinePerLine {
                        if state.program.outType == "html" {
                            format!(
                                "<ul>{}</ul>",
                                teile
                                    .into_iter()
                                    .map(|t| format!("<li>{}</li>", t))
                                    .collect::<Vec<_>>()
                                    .join("")
                            )
                        } else if state.program.outType == "bbcode" {
                            format!(
                                "[list]{}[/list]",
                                teile
                                    .into_iter()
                                    .map(|t| format!("[*]{}", t))
                                    .collect::<Vec<_>>()
                                    .join("")
                            )
                        } else {
                            teile.join("\n")
                        }
                    } else if state.program.textWidth == 0 && state.program.oneTable {
                        teile.join(" | ")
                    } else {
                        teile.join("\n")
                    }
                };

                if mainTable[display_row_idx][*out_col_idx].is_empty() {
                    mainTable[display_row_idx][*out_col_idx] = merged;
                } else if !mainTable[display_row_idx][*out_col_idx].contains(&merged) {
                    let state = self.state.borrow();
                    if oneLinePerLine || (state.program.textWidth == 0 && state.program.oneTable) {
                        mainTable[display_row_idx][*out_col_idx].push_str(" | ");
                    } else {
                        mainTable[display_row_idx][*out_col_idx].push('\n');
                    }
                    mainTable[display_row_idx][*out_col_idx].push_str(&merged);
                }
            }
        }
        mainTable
    }

    pub fn readKombiCsv(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
        rowsOfcombi: Vec<i64>,
        csvFileName: &str,
    ) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<i64>>, MainToSubRelation) {
        let mut state = self.state.borrow_mut();
        sync_program_runtime_fields(&mut state);
        state.program.relitable = relitable;
        state.sumOfAllCombiRowsAmount += rowsOfcombi.len();
        let mut rowsAsNumbers = dedup_preserve_order_i64(rowsAsNumbers);

        let rows = match state.program.load_csv_rows_semicolon_exact_path(csvFileName) {
            Ok(rows) => rows,
            Err(_) => {
                return (
                    vec![vec![]],
                    state.program.relitable.clone(),
                    vec![vec![]],
                    (IndexMap::new(), IndexMap::new()),
                )
            }
        };

        let headingsAmount = state.program.relitable.first().map(|row| row.len()).unwrap_or(0) as i64;
        let mut maintable2subtable_Relation: MainToSubRelation = (IndexMap::new(), IndexMap::new());
        let mut kombiTable: Vec<Vec<String>> = vec![];
        let mut kombiTable_Kombis: Vec<Vec<i64>> = vec![];
        let mut maxlen = 0usize;

        for (z, row) in rows.into_iter().enumerate() {
            let mut col = row.clone();
            if let Some(first) = col.first().cloned() {
                for i in 1..col.len() {
                    if !col[i].trim().is_empty() && !first.trim().is_empty() {
                        col[i] = format!("({}) {} ({})", first, col[i], first);
                    }
                }
            }
            maxlen = maxlen.max(col.len());
            if z > 0 && !col.is_empty() && !col[0].trim().is_empty() {
                let mut parsed: Vec<i64> = vec![];
                for num in col[0].split('|') {
                    Self::kombiNumbersCorrectTestAndSet(num, &mut parsed);
                }
                kombiTable_Kombis.push(parsed);
            }
            kombiTable.push(col);
        }
        if maxlen > 0 {
            for row in kombiTable.iter_mut() {
                while row.len() < maxlen {
                    row.push(String::new());
                }
            }
        }

        if !state.program.relitable.is_empty() && !kombiTable.is_empty() {
            let added_cols = maxlen.saturating_sub(1);
            let header_animcol = &kombiTable[0];
            for t in 0..added_cols {
                let new_main_idx = state.program.relitable[0].len() as i64;
                let sub_idx = t as i64;
                maintable2subtable_Relation.0.insert(new_main_idx, sub_idx);
                maintable2subtable_Relation.1.insert(sub_idx, new_main_idx);
                let heading = header_animcol.get(t + 1).cloned().unwrap_or_default();
                state.program.relitable[0].push(heading.clone());
                if !heading.is_empty() {
                    state.program.generatedSpaltenParameter.push(heading);
                }
                for i in 1..state.program.relitable.len() {
                    state.program.relitable[i].push(String::new());
                }
            }
            for a in rowsOfcombi.iter().copied() {
                let u = headingsAmount + a - 1;
                if !rowsAsNumbers.contains(&u) {
                    rowsAsNumbers.push(u);
                }
            }
        }
        state.program.rowsAsNumbers = rowsAsNumbers.clone();
        (
            kombiTable,
            state.program.relitable.clone(),
            kombiTable_Kombis,
            maintable2subtable_Relation,
        )
    }

    pub fn kombiNumbersCorrectTestAndSet(input: &str, target: &mut Vec<i64>) {
        let num = input.trim();
        if num.len() > 2 && num.starts_with('(') && num.ends_with(')') {
            Self::kombiNumbersCorrectTestAndSet(&num[1..num.len() - 1], target);
            return;
        }
        if !num.is_empty()
            && (num.chars().all(|c| c.is_ascii_digit())
                || ((num.starts_with('+') || num.starts_with('-'))
                    && num[1..].chars().all(|c| c.is_ascii_digit())))
        {
            if let Ok(v) = num.parse::<i64>() {
                target.push(v.abs());
            }
            return;
        }
        if num.len() > 2 && num.contains('/') {
            if let Some((left, right)) = num.split_once('/') {
                Self::kombiNumbersCorrectTestAndSet(left, target);
                Self::kombiNumbersCorrectTestAndSet(right, target);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct TablesConcat {
    state: SharedTablesState,
}

impl TablesConcat {
    fn new(state: SharedTablesState) -> Self {
        Self { state }
    }

    pub fn ones(&self) -> Vec<i64> {
        self.state.borrow().concat_ones.clone()
    }

    pub fn set_ones(&self, ones: Vec<i64>) {
        self.state.borrow_mut().concat_ones = dedup_preserve_order_i64(ones);
    }

    pub fn readConcatCsv(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
        concatTableSelection: Vec<i64>,
        concatTable: i64,
    ) -> (Vec<Vec<String>>, Vec<i64>, Vec<i64>) {
        update_program_with_relitable_and_rows(&self.state, relitable, rowsAsNumbers, |program, rows| {
            program.readConcatCsv(rows, concatTableSelection, concatTable)
        })
    }

    pub fn concatVervielfacheZeile(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.concatVervielfacheZeile(rows),
        );
        (relitable, rows)
    }

    pub fn concatModallogik(
        &self,
        relitable: Vec<Vec<String>>,
        generRows: Vec<i64>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        self.state.borrow_mut().program.generRows = dedup_preserve_order_i64(generRows);
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.concatModallogik(rows),
        );
        (relitable, rows)
    }

    pub fn concatPrimCreativityType(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.concatPrimCreativityType(rows),
        );
        (relitable, rows)
    }

    pub fn concatGleichheitFreiheitDominieren(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.concatGleichheitFreiheitDominieren(rows),
        );
        (relitable, rows)
    }

    pub fn concatGeistEmotionEnergieMaterieTopologie(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.concatGeistEmotionEnergieMaterieTopologie(rows),
        );
        (relitable, rows)
    }

    pub fn concatMondExponzierenLogarithmusTyp(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.concatMondExponzierenLogarithmusTyp(rows),
        );
        (relitable, rows)
    }

    pub fn concat1RowPrimUniverse2(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
        _generated2_keys: Vec<i64>,
        _paraTextNamen: BTreeMap<i64, Vec<Vec<String>>>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.concat1RowPrimUniverse2(rows),
        );
        (relitable, rows)
    }

    pub fn concat1PrimzahlkreuzProContra(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
        _generated2_keys: Vec<i64>,
        _ParametersMain: BTreeMap<String, Vec<String>>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.concat1PrimzahlkreuzProContra(rows),
        );
        (relitable, rows)
    }

    pub fn concatLovePolygon(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.concatLovePolygon(rows),
        );
        (relitable, rows)
    }

    pub fn spalteFuerGegenInnenAussenSeitlichPrim(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.spalteFuerGegenInnenAussenSeitlichPrim(rows),
        );
        (relitable, rows)
    }

    pub fn spalteMetaKontretTheorieAbstrakt_etc_1(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
        _couplesX: Vec<(i64, i64)>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.spalteMetaKontretTheorieAbstrakt_etc_1(rows),
        );
        (relitable, rows)
    }
}

#[derive(Clone, Debug)]
pub struct TablesMaintable {
    state: SharedTablesState,
}

impl TablesMaintable {
    fn new(state: SharedTablesState) -> Self {
        Self { state }
    }

    pub fn createSpalteGestirn(
        &self,
        relitable: Vec<Vec<String>>,
        rowsAsNumbers: Vec<i64>,
    ) -> (Vec<Vec<String>>, Vec<i64>) {
        let (relitable, rows, _unit) = update_program_with_relitable_and_rows(
            &self.state,
            relitable,
            rowsAsNumbers,
            |program, rows| program.createSpalteGestirn(rows),
        );
        (relitable, rows)
    }
}
