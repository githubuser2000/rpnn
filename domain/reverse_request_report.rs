use std::collections::BTreeSet;

use crate::cli::TextBereich;
use crate::domain::categories::{KategorieMap, OberkategorieKey, UnterkategorieName};
use crate::domain::exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};
use crate::reta_ausgabe::OutputSyntax;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnfragePair {
    pub ober: OberkategorieKey,
    pub unter: UnterkategorieName,
}

impl AnfragePair {
    pub fn new(ober: impl Into<String>, unter: impl Into<String>) -> Self {
        let ober_raw: String = ober.into();
        let unter_raw: String = unter.into();
        Self {
            ober: OberkategorieKey::from_raw(&ober_raw),
            unter: UnterkategorieName::new(unter_raw),
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

fn collect_exact_and_partial_direct_pairs(
    kategorie_map: &KategorieMap,
    visible_columns: &BTreeSet<u32>,
    report: &mut ReverseRequestReport,
) {
    for haupt in &kategorie_map.hauptkategorien {
        for unter in &haupt.unterkategorien {
            let pair = AnfragePair {
                ober: haupt.key.clone(),
                unter: unter.name.clone(),
            };
            let cols: BTreeSet<u32> = unter.spaltennummern.iter().copied().collect();

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
    let has =
        |needle: &str| generated_befehle.iter().any(|g| normalize_key(g) == normalize_key(needle));

    if has("primzahlkreuzprocontra") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Universum", "Primzahlkreuz"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Bedeutung", "Primzahlkreuz"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Pro_Contra", "Primzahlkreuz"));
    }

    if has("lovepolygon") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Menschliches", "Liebe"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Grundstrukturen", "Liebe"));
    }

    if has("gleichheitfreiheit") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Planet", "Gleichheit"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Menschliches", "Gleichheit"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Grundstrukturen", "Gleichheit"));
    }

    if has("geistemotionenergiematerietopologie") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Universum", "Geist"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Multiversum", "Geist"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Grundstrukturen", "Geist"));
    }

    if has("primcreativitytype") || has("mondexponzierenlogarithmustyp") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Wichtigstes_zum_verstehen", "Gestirn"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Bedeutung", "Gestirn"));
    }

    if has("vervielfachezeile") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Wichtigstes_zum_verstehen", "Primzahlen"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Bedeutung", "Primzahlen"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("Galaxie", "Primzahlen"));
    }

    if has("primmotgleichf") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("primvielfache", "motivgleichfoermig"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("multiplikationen", "motivgleichfoermig"));
    }
    if has("primstrukgleichf") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("primvielfache", "strukturgleichfoermig"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("multiplikationen", "strukturgleichfoermig"));
    }
    if has("primmotivstern") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("primvielfache", "motivstern"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("multiplikationen", "motivstern"));
    }
    if has("primstrukturstern") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("primvielfache", "strukturstern"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("multiplikationen", "strukturstern"));
    }
    if has("primmotivsterngebr") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("primvielfache", "motivgebrstern"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("multiplikationen", "motivgebrstern"));
    }
    if has("primstruktursterngebr") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("primvielfache", "strukgebrstern"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("multiplikationen", "strukgebrstern"));
    }
    if has("primmotgleichfgebr") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("primvielfache", "motivgebrgleichf"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("multiplikationen", "motivgebrgleichf"));
    }
    if has("primstrukgleichfgebr") {
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("primvielfache", "strukgebrgleichf"));
        report.insert(TrefferArt::WeiterePassende, AnfragePair::new("multiplikationen", "strukgebrgleichf"));
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
