use std::collections::BTreeSet;

use crate::cli::TextBereich;
use crate::domain::categories::{KategorieMap, KategorieProvider, OberkategorieEntry, UnterkategorieEntry};
use crate::domain::exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};
use crate::domain::indices::ColumnNumber;
use crate::reta_ausgabe::OutputSyntax;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnfragePair {
    pub ober: String,
    pub unter: String,
}

impl AnfragePair {
    pub fn new(ober: impl Into<String>, unter: impl Into<String>) -> Self {
        Self {
            ober: ober.into(),
            unter: unter.into(),
        }
    }

    pub fn to_cli(&self) -> String {
        format!("--spaltenname {} {}", self.ober, self.unter)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrefferArt {
    Exakt,
    WeiterePassende,
}

#[derive(Debug, Clone, Default)]
pub struct ReverseRequestReport {
    pub exakte_auswahl: BTreeSet<AnfragePair>,
    pub weitere_passende_auswahl: BTreeSet<AnfragePair>,
}

impl ReverseRequestReport {
    pub fn is_empty(&self) -> bool {
        self.exakte_auswahl.is_empty() && self.weitere_passende_auswahl.is_empty()
    }

    pub fn insert(&mut self, art: TrefferArt, pair: AnfragePair) {
        match art {
            TrefferArt::Exakt => {
                self.weitere_passende_auswahl.remove(&pair);
                self.exakte_auswahl.insert(pair);
            }
            TrefferArt::WeiterePassende => {
                if !self.exakte_auswahl.contains(&pair) {
                    self.weitere_passende_auswahl.insert(pair);
                }
            }
        }
    }
}

pub trait CliRenderable {
    fn to_cli(&self) -> String;
}

impl CliRenderable for AnfragePair {
    fn to_cli(&self) -> String {
        self.to_cli()
    }
}

fn normalize_key(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}

fn collect_visible_columns(bereich: &TextBereich) -> BTreeSet<ColumnNumber> {
    let mut visible = BTreeSet::<ColumnNumber>::new();

    for &(a, b) in &bereich.spalten_bereiche {
        if a == 0 || b == 0 || a > b {
            continue;
        }
        for n in a..=b {
            visible.insert(ColumnNumber(n as u32));
        }
    }

    for &n in &bereich.spaltenreihenfolgeundnurdiese {
        if n > 0 {
            visible.insert(ColumnNumber(n as u32));
        }
    }

    for &n in &bereich.exact_visible_columns {
        if n > 0 {
            visible.insert(ColumnNumber(n as u32));
        }
    }

    visible
}

fn collect_exact_and_partial_direct_pairs<P>(
    provider: &P,
    visible_columns: &BTreeSet<ColumnNumber>,
    report: &mut ReverseRequestReport,
) where
    P: KategorieProvider,
{
    for haupt in provider.hauptkategorien() {
        for unter in haupt.unterkategorien() {
            let pair = AnfragePair::new(
                haupt.ober_name().as_str().to_string(),
                unter.unter_name().as_str().to_string(),
            );
            let cols: BTreeSet<ColumnNumber> = unter
                .spaltennummern()
                .iter()
                .copied()
                .map(ColumnNumber)
                .collect();

            if cols.is_empty() {
                continue;
            }

            if cols == *visible_columns {
                report.insert(TrefferArt::Exakt, pair);
            } else if cols.iter().any(|c| visible_columns.contains(c)) {
                report.insert(TrefferArt::WeiterePassende, pair);
            }
        }
    }
}

fn collect_fraction_pairs(bereich: &TextBereich, report: &mut ReverseRequestReport) {
    for n in &bereich.pypy_compat.gebrochengalaxie {
        report.insert(
            TrefferArt::WeiterePassende,
            AnfragePair::new("gebrochen-rational_Galaxie_n/m", n.to_string()),
        );
    }
    for n in &bereich.pypy_compat.gebrochenuniversum {
        report.insert(
            TrefferArt::WeiterePassende,
            AnfragePair::new("gebrochen-rational_Universum_n/m", n.to_string()),
        );
    }
    for n in &bereich.pypy_compat.gebrochenemotion {
        report.insert(
            TrefferArt::WeiterePassende,
            AnfragePair::new("gebrochen-rational_Gefühle_n/m", n.to_string()),
        );
    }
    for n in &bereich.pypy_compat.gebrochengroesse {
        report.insert(
            TrefferArt::WeiterePassende,
            AnfragePair::new("gebrochen-rational_Strukturgroesse_n/m", n.to_string()),
        );
    }
}

fn collect_kombi_pairs(bereich: &TextBereich, report: &mut ReverseRequestReport) {
    fn galaxie_name(idx: usize) -> Option<&'static str> {
        match idx {
            1 => Some("tiere"),
            2 => Some("berufe"),
            3 => Some("kreativität"),
            4 => Some("liebe"),
            7 => Some("männer"),
            8 => Some("persönlichkeit"),
            9 => Some("religion"),
            10 => Some("motive"),
            12 => Some("emotionen"),
            13 => Some("personen"),
            16 => Some("wirtschaftssysteme"),
            17 => Some("eigentum"),
            _ => None,
        }
    }

    fn universum_name(idx: usize) -> Option<&'static str> {
        match idx {
            1 => Some("tiere"),
            2 => Some("berufe"),
            5 => Some("transzendentalien"),
            6 => Some("primzahlkreuz"),
            8 => Some("persönlichkeit"),
            9 => Some("religion"),
            10 => Some("motive"),
            11 => Some("ontologie"),
            13 => Some("personen"),
            14 => Some("mechanismen"),
            15 => Some("gegentranszendentalien"),
            17 => Some("maschinen"),
            18 => Some("geist"),
            19 => Some("bewusstsein"),
            _ => None,
        }
    }

    for idx in &bereich.pypy_compat.kombi_galaxie {
        if let Some(name) = galaxie_name(*idx) {
            report.insert(
                TrefferArt::WeiterePassende,
                AnfragePair::new("KombinationGalaxie", name),
            );
        }
    }

    for idx in &bereich.pypy_compat.kombi_universum {
        if let Some(name) = universum_name(*idx) {
            report.insert(
                TrefferArt::WeiterePassende,
                AnfragePair::new("KombinationUniversum", name),
            );
        }
    }
}

fn collect_generated_pairs(generated_befehle: &BTreeSet<String>, report: &mut ReverseRequestReport) {
    let has = |needle: &str| generated_befehle.iter().any(|g| normalize_key(g) == normalize_key(needle));

    let mut add = |ober: &str, unter: &str| {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new(ober, unter));
    };

    if has("primzahlkreuzprocontra") {
        add("Universum", "Primzahlkreuz");
        add("Bedeutung", "Primzahlkreuz");
        add("Pro_Contra", "Primzahlkreuz");
    }
    if has("lovepolygon") {
        add("Menschliches", "Liebe");
        add("Grundstrukturen", "Liebe");
    }
    if has("gleichheitfreiheit") {
        add("Planet", "Gleichheit");
        add("Menschliches", "Gleichheit");
        add("Grundstrukturen", "Gleichheit");
    }
    if has("geistemotionenergiematerietopologie") {
        add("Universum", "Geist");
        add("Multiversum", "Geist");
        add("Grundstrukturen", "Geist");
    }
    if has("primcreativitytype") || has("mondexponzierenlogarithmustyp") {
        add("Wichtigstes_zum_verstehen", "Gestirn");
        add("Bedeutung", "Gestirn");
    }
    if has("vervielfachezeile") {
        add("Wichtigstes_zum_verstehen", "Primzahlen");
        add("Bedeutung", "Primzahlen");
        add("Galaxie", "Primzahlen");
    }
    for (needle, unter) in [
        ("primmotgleichf", "motivgleichfoermig"),
        ("primstrukgleichf", "strukturgleichfoermig"),
        ("primmotivstern", "motivstern"),
        ("primstrukturstern", "strukturstern"),
        ("primmotivsterngebr", "motivgebrstern"),
        ("primstruktursterngebr", "strukgebrstern"),
        ("primmotgleichfgebr", "motivgebrgleichf"),
        ("primstrukgleichfgebr", "strukgebrgleichf"),
    ] {
        if has(needle) {
            add("primvielfache", unter);
            add("multiplikationen", unter);
        }
    }
}

fn collect_exact_bridge_pairs(bereich: &TextBereich, report: &mut ReverseRequestReport) {
    if !bereich.exact_meta_konkret_specs.is_empty() {
        for (aliases, _pair) in META_KONKRET_MAPPINGS {
            if let Some(first) = aliases.first() {
                report.insert(
                    TrefferArt::WeiterePassende,
                    AnfragePair::new("universummetakonkret", *first),
                );
            }
        }
    }

    if !bereich.exact_modal_pairs.is_empty() {
        for (aliases, _cols, maybe_pair) in EIGENSCHAFT_MAPPINGS {
            if maybe_pair.is_some() {
                if let Some(first) = aliases.first() {
                    report.insert(
                        TrefferArt::WeiterePassende,
                        AnfragePair::new("Eigenschaften_n", *first),
                    );
                }
            }
        }
    }
}

pub fn print_reverse_request_pairs(
    kategorie_map: &KategorieMap,
    bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
) {
    if matches!(bereich.output_syntax, OutputSyntax::HTML) {
        return;
    }

    let visible_columns = collect_visible_columns(bereich);
    if visible_columns.is_empty()
        && generated_befehle.is_empty()
        && bereich.exact_meta_konkret_specs.is_empty()
        && bereich.exact_modal_pairs.is_empty()
        && bereich.pypy_compat.gebrochengalaxie.is_empty()
        && bereich.pypy_compat.gebrochenuniversum.is_empty()
        && bereich.pypy_compat.gebrochenemotion.is_empty()
        && bereich.pypy_compat.gebrochengroesse.is_empty()
        && bereich.pypy_compat.kombi_galaxie.is_empty()
        && bereich.pypy_compat.kombi_universum.is_empty()
    {
        return;
    }

    let mut report = ReverseRequestReport::default();
    collect_exact_and_partial_direct_pairs(kategorie_map, &visible_columns, &mut report);
    collect_fraction_pairs(bereich, &mut report);
    collect_kombi_pairs(bereich, &mut report);
    collect_generated_pairs(generated_befehle, &mut report);
    collect_exact_bridge_pairs(bereich, &mut report);

    if report.is_empty() {
        return;
    }

    println!();
    if !report.exakte_auswahl.is_empty() {
        println!("══════════════════════════════════════════════");
        println!("Exakt äquivalente Spalten-Auswahl:");
        println!("══════════════════════════════════════════════");
        for pair in &report.exakte_auswahl {
            println!("  {}", pair.to_cli());
        }
    }
    if !report.weitere_passende_auswahl.is_empty() {
        if !report.exakte_auswahl.is_empty() {
            println!();
        }
        println!("══════════════════════════════════════════════");
        println!("Weitere passende, aber nicht exakt äquivalente Auswahl:");
        println!("══════════════════════════════════════════════");
        for pair in &report.weitere_passende_auswahl {
            println!("  {}", pair.to_cli());
        }
    }
}

pub fn print_reverse_request_pairs_dual(
    kategorie_map: &KategorieMap,
    bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
) {
    print_reverse_request_pairs(kategorie_map, bereich, generated_befehle);
}
