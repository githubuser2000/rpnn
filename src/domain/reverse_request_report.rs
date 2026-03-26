use std::collections::BTreeSet;

use crate::cli::TextBereich;
use crate::domain::categories::KategorieMap;
use crate::domain::exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};
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
    exact_out: &mut BTreeSet<AnfragePair>,
    partial_out: &mut BTreeSet<AnfragePair>,
) {
    for haupt in &kategorie_map.hauptkategorien {
        for unter in &haupt.unterkategorien {
            let pair = AnfragePair::new(haupt.name.clone(), unter.name.clone());
            let cols: BTreeSet<u32> = unter.spaltennummern.iter().copied().collect();

            if cols.is_empty() {
                continue;
            }

            if cols == *visible_columns {
                exact_out.insert(pair);
            } else if cols.iter().any(|c| visible_columns.contains(c)) {
                partial_out.insert(pair);
            }
        }
    }
}

fn collect_fraction_pairs(bereich: &TextBereich, out: &mut BTreeSet<AnfragePair>) {
    for n in &bereich.pypy_compat.gebrochengalaxie {
        out.insert(AnfragePair::new("gebrochen-rational_Galaxie_n/m", n.to_string()));
    }
    for n in &bereich.pypy_compat.gebrochenuniversum {
        out.insert(AnfragePair::new("gebrochen-rational_Universum_n/m", n.to_string()));
    }
    for n in &bereich.pypy_compat.gebrochenemotion {
        out.insert(AnfragePair::new("gebrochen-rational_Gefühle_n/m", n.to_string()));
    }
    for n in &bereich.pypy_compat.gebrochengroesse {
        out.insert(AnfragePair::new("gebrochen-rational_Strukturgroesse_n/m", n.to_string()));
    }
}

fn collect_kombi_pairs(bereich: &TextBereich, out: &mut BTreeSet<AnfragePair>) {
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
            out.insert(AnfragePair::new("KombinationGalaxie", name));
        }
    }

    for idx in &bereich.pypy_compat.kombi_universum {
        if let Some(name) = universum_name(*idx) {
            out.insert(AnfragePair::new("KombinationUniversum", name));
        }
    }
}

fn collect_generated_pairs(generated_befehle: &BTreeSet<String>, out: &mut BTreeSet<AnfragePair>) {
    let has =
        |needle: &str| generated_befehle.iter().any(|g| normalize_key(g) == normalize_key(needle));

    if has("primzahlkreuzprocontra") {
        out.insert(AnfragePair::new("Universum", "Primzahlkreuz"));
        out.insert(AnfragePair::new("Bedeutung", "Primzahlkreuz"));
        out.insert(AnfragePair::new("Pro_Contra", "Primzahlkreuz"));
    }

    if has("lovepolygon") {
        out.insert(AnfragePair::new("Menschliches", "Liebe"));
        out.insert(AnfragePair::new("Grundstrukturen", "Liebe"));
    }

    if has("gleichheitfreiheit") {
        out.insert(AnfragePair::new("Planet", "Gleichheit"));
        out.insert(AnfragePair::new("Menschliches", "Gleichheit"));
        out.insert(AnfragePair::new("Grundstrukturen", "Gleichheit"));
    }

    if has("geistemotionenergiematerietopologie") {
        out.insert(AnfragePair::new("Universum", "Geist"));
        out.insert(AnfragePair::new("Multiversum", "Geist"));
        out.insert(AnfragePair::new("Grundstrukturen", "Geist"));
    }

    if has("primcreativitytype") || has("mondexponzierenlogarithmustyp") {
        out.insert(AnfragePair::new("Wichtigstes_zum_verstehen", "Gestirn"));
        out.insert(AnfragePair::new("Bedeutung", "Gestirn"));
    }

    if has("vervielfachezeile") {
        out.insert(AnfragePair::new("Wichtigstes_zum_verstehen", "Primzahlen"));
        out.insert(AnfragePair::new("Bedeutung", "Primzahlen"));
        out.insert(AnfragePair::new("Galaxie", "Primzahlen"));
    }

    if has("primmotgleichf") {
        out.insert(AnfragePair::new("primvielfache", "motivgleichfoermig"));
        out.insert(AnfragePair::new("multiplikationen", "motivgleichfoermig"));
    }
    if has("primstrukgleichf") {
        out.insert(AnfragePair::new("primvielfache", "strukturgleichfoermig"));
        out.insert(AnfragePair::new("multiplikationen", "strukturgleichfoermig"));
    }
    if has("primmotivstern") {
        out.insert(AnfragePair::new("primvielfache", "motivstern"));
        out.insert(AnfragePair::new("multiplikationen", "motivstern"));
    }
    if has("primstrukturstern") {
        out.insert(AnfragePair::new("primvielfache", "strukturstern"));
        out.insert(AnfragePair::new("multiplikationen", "strukturstern"));
    }
    if has("primmotivsterngebr") {
        out.insert(AnfragePair::new("primvielfache", "motivgebrstern"));
        out.insert(AnfragePair::new("multiplikationen", "motivgebrstern"));
    }
    if has("primstruktursterngebr") {
        out.insert(AnfragePair::new("primvielfache", "strukgebrstern"));
        out.insert(AnfragePair::new("multiplikationen", "strukgebrstern"));
    }
    if has("primmotgleichfgebr") {
        out.insert(AnfragePair::new("primvielfache", "motivgebrgleichf"));
        out.insert(AnfragePair::new("multiplikationen", "motivgebrgleichf"));
    }
    if has("primstrukgleichfgebr") {
        out.insert(AnfragePair::new("primvielfache", "strukgebrgleichf"));
        out.insert(AnfragePair::new("multiplikationen", "strukgebrgleichf"));
    }
}

fn collect_exact_bridge_pairs(bereich: &TextBereich, out: &mut BTreeSet<AnfragePair>) {
    if !bereich.exact_meta_konkret_specs.is_empty() {
        for (aliases, _pair) in META_KONKRET_MAPPINGS {
            if let Some(first) = aliases.first() {
                out.insert(AnfragePair::new("universummetakonkret", *first));
            }
        }
    }

    if !bereich.exact_modal_pairs.is_empty() {
        for (aliases, _cols, maybe_pair) in EIGENSCHAFT_MAPPINGS {
            if maybe_pair.is_some() {
                if let Some(first) = aliases.first() {
                    out.insert(AnfragePair::new("Eigenschaften_n", *first));
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

    let mut exact_pairs = BTreeSet::<AnfragePair>::new();
    let mut non_exact_pairs = BTreeSet::<AnfragePair>::new();

    collect_exact_and_partial_direct_pairs(
        kategorie_map,
        &visible_columns,
        &mut exact_pairs,
        &mut non_exact_pairs,
    );
    collect_fraction_pairs(bereich, &mut non_exact_pairs);
    collect_kombi_pairs(bereich, &mut non_exact_pairs);
    collect_generated_pairs(generated_befehle, &mut non_exact_pairs);
    collect_exact_bridge_pairs(bereich, &mut non_exact_pairs);

    for pair in &exact_pairs {
        non_exact_pairs.remove(pair);
    }

    if exact_pairs.is_empty() && non_exact_pairs.is_empty() {
        return;
    }

    println!();

    if !exact_pairs.is_empty() {
        println!("══════════════════════════════════════════════");
        println!("Exakt äquivalente Spalten-Auswahl:");
        println!("══════════════════════════════════════════════");
        for pair in &exact_pairs {
            println!("  {}", pair.to_cli());
        }
    }

    if !non_exact_pairs.is_empty() {
        if !exact_pairs.is_empty() {
            println!();
        }
        println!("══════════════════════════════════════════════");
        println!("Weitere passende, aber nicht exakt äquivalente Auswahl:");
        println!("══════════════════════════════════════════════");
        for pair in &non_exact_pairs {
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
