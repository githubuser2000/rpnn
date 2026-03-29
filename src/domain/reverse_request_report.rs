use std::collections::BTreeSet;

use crate::cli::TextBereich;
use crate::domain::categories::KategorieMap;
use crate::domain::exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};
use crate::domain::python_source_of_truth::{generated_pairs_for_command, kombination_name_for_index, multiplication_pairs_for_command};
use crate::domain::indices::ColumnNumber;
use crate::domain::spalten_anfrage::SpaltenAnfrage;
use crate::reta_ausgabe::OutputSyntax;

use crate::domain::model::spalten_anfrage::CanonicalColumnSpec;

pub fn spec_to_report_line(spec: &CanonicalColumnSpec) -> String {
    spec.header_display.clone()
}

pub fn request_to_report_line(req: &SpaltenAnfrage) -> String {
    format!("{req:?}")
}

type AnfragePair = SpaltenAnfrage;

fn normalize_key(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}

fn request(ober: &str, unter: &str) -> SpaltenAnfrage {
    SpaltenAnfrage::parse(ober, unter).unwrap_or_else(|_| SpaltenAnfrage::Unknown {
        ober: ober.to_string(),
        unter: unter.to_string(),
    })
}

fn collect_visible_columns(bereich: &TextBereich) -> BTreeSet<u32> {
    let mut visible = BTreeSet::<u32>::new();

    for &(a, b) in &bereich.spalten_bereiche {
        if a == 0 || b == 0 || a > b {
            continue;
        }
        for n in a..=b {
            visible.insert(ColumnNumber::from(n).one_based() as u32);
        }
    }

    for &n in &bereich.spaltenreihenfolgeundnurdiese {
        if n > 0 {
            visible.insert(ColumnNumber::from(n).one_based() as u32);
        }
    }

    for &n in &bereich.exact_visible_columns {
        if n > 0 {
            visible.insert(ColumnNumber::from(n).one_based() as u32);
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
            let pair = request(haupt.name.as_str(), unter.name.as_str());
            let cols: BTreeSet<u32> = unter.spaltennummern.as_slice().iter().copied().collect();

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
        out.insert(request("gebrochen-rational_Galaxie_n/m", &n.to_string()));
    }
    for n in &bereich.pypy_compat.gebrochenuniversum {
        out.insert(request("gebrochen-rational_Universum_n/m", &n.to_string()));
    }
    for n in &bereich.pypy_compat.gebrochenemotion {
        out.insert(request("gebrochen-rational_Gefühle_n/m", &n.to_string()));
    }
    for n in &bereich.pypy_compat.gebrochengroesse {
        out.insert(request("gebrochen-rational_Strukturgroesse_n/m", &n.to_string()));
    }
}

fn collect_kombi_pairs(bereich: &TextBereich, out: &mut BTreeSet<AnfragePair>) {
    for idx in &bereich.pypy_compat.kombi_galaxie {
        if let Some(name) = kombination_name_for_index("KombinationGalaxie", *idx) {
            out.insert(request("KombinationGalaxie", name));
        }
    }

    for idx in &bereich.pypy_compat.kombi_universum {
        if let Some(name) = kombination_name_for_index("KombinationUniversum", *idx) {
            out.insert(request("KombinationUniversum", name));
        }
    }
}

fn collect_generated_pairs(generated_befehle: &BTreeSet<String>, out: &mut BTreeSet<AnfragePair>) {
    for generated in generated_befehle {
        for (ober, unter) in generated_pairs_for_command(generated) {
            out.insert(request(&ober, &unter));
        }
        for (ober, unter) in multiplication_pairs_for_command(generated) {
            out.insert(request(&ober, &unter));
        }
    }
}

fn collect_exact_bridge_pairs(bereich: &TextBereich, out: &mut BTreeSet<AnfragePair>) {
    if !bereich.exact_meta_konkret_specs.is_empty() {
        for (aliases, _pair) in META_KONKRET_MAPPINGS {
            if let Some(first) = aliases.first() {
                out.insert(request("universummetakonkret", first));
            }
        }
    }

    if !bereich.exact_modal_pairs.is_empty() {
        for (aliases, _cols, maybe_pair) in EIGENSCHAFT_MAPPINGS {
            if maybe_pair.is_some() {
                if let Some(first) = aliases.first() {
                    out.insert(request("Eigenschaften_n", first));
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

    collect_exact_and_partial_direct_pairs(kategorie_map, &visible_columns, &mut exact_pairs, &mut non_exact_pairs);
    collect_fraction_pairs(bereich, &mut non_exact_pairs);
    collect_kombi_pairs(bereich, &mut non_exact_pairs);
    collect_generated_pairs(generated_befehle, &mut non_exact_pairs);
    collect_exact_bridge_pairs(bereich, &mut non_exact_pairs);

    for ex in &exact_pairs {
        non_exact_pairs.remove(ex);
    }

    if exact_pairs.is_empty() && non_exact_pairs.is_empty() {
        return;
    }

    println!();
    println!("══════════════════════════════════════════════");
    println!("Exakt äquivalente Spalten-Auswahl:");
    println!("══════════════════════════════════════════════");
    if exact_pairs.is_empty() {
        println!("  (keine)");
    } else {
        for pair in &exact_pairs {
            println!("  {}", pair.to_cli());
        }
    }

    println!();
    println!("══════════════════════════════════════════════");
    println!("Weitere passende, aber nicht exakt äquivalente Auswahl:");
    println!("══════════════════════════════════════════════");
    if non_exact_pairs.is_empty() {
        println!("  (keine)");
    } else {
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
