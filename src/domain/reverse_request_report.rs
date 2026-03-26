use std::collections::BTreeSet;

use crate::cli::TextBereich;
use crate::domain::categories::{KategorieMap, KategorieProvider, OberkategorieEntry, UnterkategorieEntry};
use crate::domain::exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};
use crate::reta_ausgabe::OutputSyntax;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnfragePair {
    pub ober: String,
    pub unter: String,
}

pub trait CliRenderable {
    fn to_cli(&self) -> String;
}

impl AnfragePair {
    pub fn new(ober: impl Into<String>, unter: impl Into<String>) -> Self {
        Self {
            ober: ober.into(),
            unter: unter.into(),
        }
    }
}

impl CliRenderable for AnfragePair {
    fn to_cli(&self) -> String {
        format!("--spaltenname {} {}", self.ober, self.unter)
    }
}

#[derive(Debug, Default)]
struct ReverseRequestReport {
    exact_pairs: BTreeSet<AnfragePair>,
    non_exact_pairs: BTreeSet<AnfragePair>,
}

trait ReverseRequestSink {
    fn insert_exact(&mut self, pair: AnfragePair);
    fn insert_partial(&mut self, pair: AnfragePair);
}

impl ReverseRequestSink for ReverseRequestReport {
    fn insert_exact(&mut self, pair: AnfragePair) {
        self.non_exact_pairs.remove(&pair);
        self.exact_pairs.insert(pair);
    }

    fn insert_partial(&mut self, pair: AnfragePair) {
        if !self.exact_pairs.contains(&pair) {
            self.non_exact_pairs.insert(pair);
        }
    }
}

impl ReverseRequestReport {
    fn is_empty(&self) -> bool {
        self.exact_pairs.is_empty() && self.non_exact_pairs.is_empty()
    }
}

trait ReverseRequestCollector {
    fn collect(&self, sink: &mut dyn ReverseRequestSink);
}

fn normalize_key(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}

fn collect_visible_columns(bereich: &TextBereich) -> BTreeSet<u32> {
    let mut visible = BTreeSet::<u32>::new();

    for &(a, b) in &bereich.spalten_bereiche {
        if a == 0 || b == 0 || a > b {
            continue;
        }
        for n in a..=b {
            visible.insert(n as u32);
        }
    }

    for &n in &bereich.spaltenreihenfolgeundnurdiese {
        if n > 0 {
            visible.insert(n as u32);
        }
    }

    for &n in &bereich.exact_visible_columns {
        if n > 0 {
            visible.insert(n as u32);
        }
    }

    visible
}

struct DirectPairCollector<'a> {
    kategorie_map: &'a KategorieMap,
    visible_columns: &'a BTreeSet<u32>,
}

impl<'a> ReverseRequestCollector for DirectPairCollector<'a> {
    fn collect(&self, sink: &mut dyn ReverseRequestSink) {
        for haupt in self.kategorie_map.hauptkategorien() {
            for unter in haupt.unterkategorien() {
                let pair = AnfragePair::new(haupt.ober_name(), unter.unter_name());
                let cols: BTreeSet<u32> = unter.column_numbers().iter().copied().collect();

                if cols.is_empty() {
                    continue;
                }

                if cols == *self.visible_columns {
                    sink.insert_exact(pair);
                } else if cols.iter().any(|c| self.visible_columns.contains(c)) {
                    sink.insert_partial(pair);
                }
            }
        }
    }
}

struct FractionPairCollector<'a> {
    bereich: &'a TextBereich,
}

impl<'a> ReverseRequestCollector for FractionPairCollector<'a> {
    fn collect(&self, sink: &mut dyn ReverseRequestSink) {
        for n in &self.bereich.pypy_compat.gebrochengalaxie {
            sink.insert_partial(AnfragePair::new("gebrochen-rational_Galaxie_n/m", n.to_string()));
        }
        for n in &self.bereich.pypy_compat.gebrochenuniversum {
            sink.insert_partial(AnfragePair::new("gebrochen-rational_Universum_n/m", n.to_string()));
        }
        for n in &self.bereich.pypy_compat.gebrochenemotion {
            sink.insert_partial(AnfragePair::new("gebrochen-rational_Gefühle_n/m", n.to_string()));
        }
        for n in &self.bereich.pypy_compat.gebrochengroesse {
            sink.insert_partial(AnfragePair::new("gebrochen-rational_Strukturgroesse_n/m", n.to_string()));
        }
    }
}

struct KombiPairCollector<'a> {
    bereich: &'a TextBereich,
}

impl<'a> KombiPairCollector<'a> {
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
}

impl<'a> ReverseRequestCollector for KombiPairCollector<'a> {
    fn collect(&self, sink: &mut dyn ReverseRequestSink) {
        for idx in &self.bereich.pypy_compat.kombi_galaxie {
            if let Some(name) = Self::galaxie_name(*idx) {
                sink.insert_partial(AnfragePair::new("KombinationGalaxie", name));
            }
        }

        for idx in &self.bereich.pypy_compat.kombi_universum {
            if let Some(name) = Self::universum_name(*idx) {
                sink.insert_partial(AnfragePair::new("KombinationUniversum", name));
            }
        }
    }
}

struct GeneratedPairCollector<'a> {
    generated_befehle: &'a BTreeSet<String>,
}

impl<'a> GeneratedPairCollector<'a> {
    fn has(&self, needle: &str) -> bool {
        self.generated_befehle
            .iter()
            .any(|g| normalize_key(g) == normalize_key(needle))
    }
}

impl<'a> ReverseRequestCollector for GeneratedPairCollector<'a> {
    fn collect(&self, sink: &mut dyn ReverseRequestSink) {
        if self.has("primzahlkreuzprocontra") {
            sink.insert_partial(AnfragePair::new("Universum", "Primzahlkreuz"));
            sink.insert_partial(AnfragePair::new("Bedeutung", "Primzahlkreuz"));
            sink.insert_partial(AnfragePair::new("Pro_Contra", "Primzahlkreuz"));
        }

        if self.has("lovepolygon") {
            sink.insert_partial(AnfragePair::new("Menschliches", "Liebe"));
            sink.insert_partial(AnfragePair::new("Grundstrukturen", "Liebe"));
        }

        if self.has("gleichheitfreiheit") {
            sink.insert_partial(AnfragePair::new("Planet", "Gleichheit"));
            sink.insert_partial(AnfragePair::new("Menschliches", "Gleichheit"));
            sink.insert_partial(AnfragePair::new("Grundstrukturen", "Gleichheit"));
        }

        if self.has("geistemotionenergiematerietopologie") {
            sink.insert_partial(AnfragePair::new("Universum", "Geist"));
            sink.insert_partial(AnfragePair::new("Multiversum", "Geist"));
            sink.insert_partial(AnfragePair::new("Grundstrukturen", "Geist"));
        }

        if self.has("primcreativitytype") || self.has("mondexponzierenlogarithmustyp") {
            sink.insert_partial(AnfragePair::new("Wichtigstes_zum_verstehen", "Gestirn"));
            sink.insert_partial(AnfragePair::new("Bedeutung", "Gestirn"));
        }

        if self.has("vervielfachezeile") {
            sink.insert_partial(AnfragePair::new("Wichtigstes_zum_verstehen", "Primzahlen"));
            sink.insert_partial(AnfragePair::new("Bedeutung", "Primzahlen"));
            sink.insert_partial(AnfragePair::new("Galaxie", "Primzahlen"));
        }

        if self.has("primmotgleichf") {
            sink.insert_partial(AnfragePair::new("primvielfache", "motivgleichfoermig"));
            sink.insert_partial(AnfragePair::new("multiplikationen", "motivgleichfoermig"));
        }
        if self.has("primstrukgleichf") {
            sink.insert_partial(AnfragePair::new("primvielfache", "strukturgleichfoermig"));
            sink.insert_partial(AnfragePair::new("multiplikationen", "strukturgleichfoermig"));
        }
        if self.has("primmotivstern") {
            sink.insert_partial(AnfragePair::new("primvielfache", "motivstern"));
            sink.insert_partial(AnfragePair::new("multiplikationen", "motivstern"));
        }
        if self.has("primstrukturstern") {
            sink.insert_partial(AnfragePair::new("primvielfache", "strukturstern"));
            sink.insert_partial(AnfragePair::new("multiplikationen", "strukturstern"));
        }
        if self.has("primmotivsterngebr") {
            sink.insert_partial(AnfragePair::new("primvielfache", "motivgebrstern"));
            sink.insert_partial(AnfragePair::new("multiplikationen", "motivgebrstern"));
        }
        if self.has("primstruktursterngebr") {
            sink.insert_partial(AnfragePair::new("primvielfache", "strukgebrstern"));
            sink.insert_partial(AnfragePair::new("multiplikationen", "strukgebrstern"));
        }
        if self.has("primmotgleichfgebr") {
            sink.insert_partial(AnfragePair::new("primvielfache", "motivgebrgleichf"));
            sink.insert_partial(AnfragePair::new("multiplikationen", "motivgebrgleichf"));
        }
        if self.has("primstrukgleichfgebr") {
            sink.insert_partial(AnfragePair::new("primvielfache", "strukgebrgleichf"));
            sink.insert_partial(AnfragePair::new("multiplikationen", "strukgebrgleichf"));
        }
    }
}

struct ExactBridgeCollector<'a> {
    bereich: &'a TextBereich,
}

impl<'a> ReverseRequestCollector for ExactBridgeCollector<'a> {
    fn collect(&self, sink: &mut dyn ReverseRequestSink) {
        if !self.bereich.exact_meta_konkret_specs.is_empty() {
            for (aliases, _pair) in META_KONKRET_MAPPINGS {
                if let Some(first) = aliases.first() {
                    sink.insert_partial(AnfragePair::new("universummetakonkret", *first));
                }
            }
        }

        if !self.bereich.exact_modal_pairs.is_empty() {
            for (aliases, _cols, maybe_pair) in EIGENSCHAFT_MAPPINGS {
                if maybe_pair.is_some() {
                    if let Some(first) = aliases.first() {
                        sink.insert_partial(AnfragePair::new("Eigenschaften_n", *first));
                    }
                }
            }
        }
    }
}

fn print_pair_section(title: &str, pairs: &BTreeSet<AnfragePair>) {
    println!("══════════════════════════════════════════════");
    println!("{title}");
    println!("══════════════════════════════════════════════");
    for pair in pairs {
        println!("  {}", pair.to_cli());
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

    let direct = DirectPairCollector {
        kategorie_map,
        visible_columns: &visible_columns,
    };
    let fraction = FractionPairCollector { bereich };
    let kombi = KombiPairCollector { bereich };
    let generated = GeneratedPairCollector { generated_befehle };
    let bridges = ExactBridgeCollector { bereich };

    let collectors: [&dyn ReverseRequestCollector; 5] =
        [&direct, &fraction, &kombi, &generated, &bridges];

    let mut report = ReverseRequestReport::default();
    for collector in collectors {
        collector.collect(&mut report);
    }

    if report.is_empty() {
        return;
    }

    println!();

    if !report.exact_pairs.is_empty() {
        print_pair_section("Exakt äquivalente Spalten-Auswahl:", &report.exact_pairs);
    }

    if !report.non_exact_pairs.is_empty() {
        if !report.exact_pairs.is_empty() {
            println!();
        }
        print_pair_section(
            "Weitere passende, aber nicht exakt äquivalente Auswahl:",
            &report.non_exact_pairs,
        );
    }
}

pub fn print_reverse_request_pairs_dual(
    kategorie_map: &KategorieMap,
    bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
) {
    print_reverse_request_pairs(kategorie_map, bereich, generated_befehle);
}
