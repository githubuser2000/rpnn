use std::collections::{BTreeMap, BTreeSet, HashMap};

pub type Table = Vec<Vec<String>>;
pub type RowSet = BTreeSet<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ST {
    SternPolygon,
    Galaxie,
    GleichfoermigesPolygon,
    Universum,
}

#[derive(Debug, Clone, Default)]
pub struct Tables {
    pub generated_spalten_parameter_tags: HashMap<usize, BTreeSet<ST>>,
    pub generated_spalten_parameter: BTreeMap<usize, String>,
    pub spalten_vanilla_amount: usize,
    pub last_line_number: usize,
    pub data_dict: HashMap<usize, HashMap<usize, String>>,
    pub html_output_yes: bool,
    pub bbcode_output_yes: bool,
    pub hoechste_zeile_1024: usize,
}

#[derive(Debug, Clone, Default)]
pub struct ParametersMain {
    pub bedeutung0: String,
    pub procontra0: String,
    pub grundstrukturen0: String,
    pub unter0: String,
}

#[derive(Debug, Clone, Default)]
pub struct HtmlParameters {
    pub generated_befehle: BTreeSet<String>,
    pub source_label: String,
    pub heading: String,
}

#[derive(Debug, Clone, Default)]
pub struct PrimUniverseConfig {
    pub generated_befehle: BTreeSet<String>,
    pub para_text_namen: BTreeMap<usize, Vec<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct MetaKonkretContext {
    pub couples: Vec<usize>,
    pub generated_befehle: BTreeSet<String>,
    pub html: HtmlParameters,
}

pub fn concat1_row_prim_universe2(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
    _config: &PrimUniverseConfig,
) {
    todo!("Python-Funktion concat1RowPrimUniverse2 noch nach Rust portieren");
}

pub fn spalte_meta_konkret_theorie_abstrakt_etc_1(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
    _ctx: &MetaKonkretContext,
) {
    todo!("Python-Funktion spalteMetaKontretTheorieAbstrakt_etc_1 noch nach Rust portieren");
}

pub fn spalte_meta_konkret_abstrakt_is_ganzzahlig(
    _zahl: f64,
    _spalten_wahl: bool,
) -> bool {
    todo!("Python-Funktion spalteMetaKonkretAbstrakt_isGanzZahlig noch nach Rust portieren");
}

pub fn spalte_meta_konkret_theorie_abstrakt_etc(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
    _ctx: &MetaKonkretContext,
) {
    todo!("Python-Funktion spalteMetaKontretTheorieAbstrakt_etc noch nach Rust portieren");
}

pub fn spalte_meta_konkret_theorie_abstrakt_set_html_parameters(
    _table: &mut Table,
    _tables: &mut Tables,
    _html: &HtmlParameters,
) {
    todo!("Python-Funktion spalteMetaKonkretTheorieAbstrakt_SetHtmlParameters noch nach Rust portieren");
}

pub fn spalte_meta_konkret_theorie_abstrakt_main_part(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
    _ctx: &MetaKonkretContext,
) {
    todo!("Python-Funktion spalteMetaKonkretTheorieAbstrakt_mainPart noch nach Rust portieren");
}

pub fn spalte_meta_konkret_theorie_abstrakt_vorwort_behandlung_wie_vorwort_meta(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
    _ctx: &MetaKonkretContext,
) {
    todo!("Python-Funktion spalteMetaKonkretTheorieAbstrakt_VorwortBehandlungWieVorwortMeta noch nach Rust portieren");
}

pub fn spalte_meta_konkret_theorie_abstrakt_main_part_inserting_text(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
    _ctx: &MetaKonkretContext,
) {
    todo!("Python-Funktion spalteMetaKonkretTheorieAbstrakt_mainPart_InsertingText noch nach Rust portieren");
}

pub fn spalte_meta_konkret_theorie_abstrakt_get_gebr_rat_univ_strukturalie(
    _zahl: usize,
    _table: &Table,
    _tables: &Tables,
    _ctx: &MetaKonkretContext,
) -> String {
    todo!("Python-Funktion spalteMetaKonkretTheorieAbstrakt_getGebrRatUnivStrukturalie noch nach Rust portieren");
}

pub fn spalte_meta_konkret_abstrakt_ueberschriften_und_tags(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
    _ctx: &MetaKonkretContext,
) {
    todo!("Python-Funktion spalteMetaKonkretAbstrakt_UeberschriftenUndTags noch nach Rust portieren");
}

pub fn spalte_fuer_gegen_innen_aussen_seitlich_prim(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
) {
    todo!("Python-Funktion spalteFuerGegenInnenAussenSeitlichPrim noch nach Rust portieren");
}

pub fn create_spalte_gestirn(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
) {
    todo!("Python-Funktion createSpalteGestirn noch nach Rust portieren");
}

pub fn generate_missing_eigenschaften_family_columns(
    _table: &mut Table,
    _rows_as_numbers: &mut RowSet,
    _tables: &mut Tables,
    _requested_aliases: &BTreeSet<String>,
) {
    todo!("Allgemeine Eigenschaften-Generierung fehlt noch nach Rust portieren");
}
