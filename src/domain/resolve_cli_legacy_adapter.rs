use std::collections::BTreeSet;

use crate::domain::categories::{resolve_via_categories, KategorieMap};
use crate::domain::exact_generator_bridge::resolve_exact_generator;
use crate::domain::model::spalten_anfrage::{
    CanonicalColumnSpec, ColumnTarget, SpaltenAnfrage,
};
use crate::domain::parser::cli_alias_parser::{parse_spalten_anfrage, ParseError};
use crate::domain::resolver::request_resolver::resolve_request;

#[derive(Debug, Clone, Default)]
pub struct LegacyResolvedSelection {
    pub direct_columns: Vec<u16>,
    pub required_columns: Vec<u16>,
    pub exact_direct_columns: Vec<usize>,
    pub exact_modal_pairs: Vec<(usize, usize)>,
    pub exact_meta_konkret_specs: Vec<(usize, usize)>,
    pub generated_befehle: BTreeSet<String>,
}

pub fn resolve_cli_selection(
    _kategorie_map: &KategorieMap,
    ober: &str,
    unter: &str,
) -> Result<LegacyResolvedSelection, Box<dyn std::error::Error>> {
    let req = parse_spalten_anfrage(ober, unter).map_err(to_boxed_error)?;

    let spec = resolve_any(&req).ok_or_else(|| {
        format!(
            "Konnte Spaltenanfrage nicht auflösen: ober='{}', unter='{}'",
            ober, unter
        )
    })?;

    Ok(spec_to_legacy_selection(&spec))
}

fn resolve_any(req: &SpaltenAnfrage) -> Option<CanonicalColumnSpec> {
    resolve_request(req.clone())
        .or_else(|| resolve_via_categories(req))
        .or_else(|| resolve_exact_generator(req))
}

fn spec_to_legacy_selection(spec: &CanonicalColumnSpec) -> LegacyResolvedSelection {
    let mut out = LegacyResolvedSelection::default();

    match &spec.target {
        ColumnTarget::DirectColumn(id) => {
            out.direct_columns.push(*id);
            out.required_columns.push(*id);
        }
        ColumnTarget::DirectColumns(ids) => {
            out.direct_columns.extend(ids.iter().copied());
            out.required_columns.extend(ids.iter().copied());
        }
        ColumnTarget::Pair(left, right) => {
            out.exact_modal_pairs.push((*left as usize, *right as usize));
        }
        ColumnTarget::Generator(generator_spec) => {
            out.generated_befehle.insert(generator_art_to_legacy_name(&generator_spec.art));
            match &generator_spec.parameter {
                crate::domain::model::spalten_anfrage::GeneratorParameter::Keine => {}
                crate::domain::model::spalten_anfrage::GeneratorParameter::Text(s) => {
                    if !s.trim().is_empty() {
                        out.generated_befehle.insert(s.trim().to_string());
                    }
                }
                crate::domain::model::spalten_anfrage::GeneratorParameter::Zahl(n) => {
                    out.generated_befehle.insert(n.to_string());
                }
                crate::domain::model::spalten_anfrage::GeneratorParameter::TextListe(xs) => {
                    out.generated_befehle.extend(
                        xs.iter()
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .map(ToOwned::to_owned),
                    );
                }
            }
        }
        ColumnTarget::Combination(_comb) => {}
    }

    dedup_legacy_selection(&mut out);
    out
}

fn generator_art_to_legacy_name(art: &crate::domain::ids::domain_id::GeneratorArt) -> String {
    match art {
        crate::domain::ids::domain_id::GeneratorArt::Primzahlkreuz => "primzahlkreuz".to_string(),
        crate::domain::ids::domain_id::GeneratorArt::Multiplikationen => {
            "multiplikationen".to_string()
        }
        crate::domain::ids::domain_id::GeneratorArt::Primvielfache => "primvielfache".to_string(),
        crate::domain::ids::domain_id::GeneratorArt::MetaKonkret => "metakonkret".to_string(),
    }
}

fn dedup_legacy_selection(sel: &mut LegacyResolvedSelection) {
    sel.direct_columns.sort_unstable();
    sel.direct_columns.dedup();

    sel.required_columns.sort_unstable();
    sel.required_columns.dedup();

    sel.exact_direct_columns.sort_unstable();
    sel.exact_direct_columns.dedup();

    sel.exact_modal_pairs.sort_unstable();
    sel.exact_modal_pairs.dedup();

    sel.exact_meta_konkret_specs.sort_unstable();
    sel.exact_meta_konkret_specs.dedup();
}

fn to_boxed_error(err: ParseError) -> Box<dyn std::error::Error> {
    Box::<dyn std::error::Error>::from(format_parse_error(err))
}

fn format_parse_error(err: ParseError) -> String {
    match err {
        ParseError::UnknownOberkategorie(ober) => {
            format!("Unbekannte Oberkategorie: {}", ober)
        }
        ParseError::UnknownUnterkategorie { ober, unter } => {
            format!(
                "Unbekannte Unterkategorie für Oberkategorie '{}': {}",
                ober, unter
            )
        }
        ParseError::InvalidGebrochenRationalIndex { ober, unter } => {
            format!("Ungültiger n/m-Index für '{}': '{}'", ober, unter)
        }
    }
}
