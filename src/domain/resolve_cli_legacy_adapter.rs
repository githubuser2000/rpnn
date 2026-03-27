use std::collections::BTreeSet;

use crate::domain::categories::KategorieMap;
use crate::domain::exact_generator_bridge::resolve_exact_generator;
use crate::domain::model::spalten_anfrage::{CanonicalColumnSpec, ColumnTarget};
use crate::domain::request_bridge::bridge_request;
use crate::domain::request_pipeline::RawSelectionRequest;
use crate::domain::resolver::request_resolver::resolve_request;
use crate::domain::spalten_anfrage::SpaltenAnfrage;

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
    kategorie_map: &KategorieMap,
    ober: &str,
    unter: &str,
) -> Result<LegacyResolvedSelection, Box<dyn std::error::Error>> {
    if let Some(exact) = resolve_exact_generator(ober, unter) {
        let mut out = LegacyResolvedSelection::default();

        out.exact_direct_columns.extend(exact.direct_columns.iter().copied());
        out.exact_modal_pairs.extend(exact.modal_pairs.iter().copied());
        out.exact_meta_konkret_specs
            .extend(exact.meta_konkret_specs.iter().copied());
        out.generated_befehle.extend(exact.generated_befehle.iter().cloned());

        dedup_legacy_selection(&mut out);
        return Ok(out);
    }

    if let Ok(parsed) = SpaltenAnfrage::parse(ober, unter) {
        if let Some(canonical) = bridge_request(&parsed) {
            if let Some(spec) = resolve_request(canonical) {
                let mut out = legacy_from_canonical(spec)?;
                // ergänze generische Kategorien-Map-Inferenz für benötigte Basis-Spalten
                if let Some(inference) = kategorie_map.infer_generated_pair(ober, unter) {
                    out.required_columns.extend(inference.required_columns.into_iter().filter_map(|n| u16::try_from(n).ok()));
                    out.direct_columns.extend(inference.direct_columns.into_iter().filter_map(|n| u16::try_from(n).ok()));
                    out.generated_befehle.extend(inference.generated_befehle);
                }
                dedup_legacy_selection(&mut out);
                return Ok(out);
            }
        }
    }

    resolve_via_legacy_pipeline(kategorie_map, ober, unter)
}

fn legacy_from_canonical(spec: CanonicalColumnSpec) -> Result<LegacyResolvedSelection, Box<dyn std::error::Error>> {
    let mut out = LegacyResolvedSelection::default();

    match spec.target {
        ColumnTarget::DirectColumn(col) => {
            out.direct_columns.push(col);
        }
        ColumnTarget::DirectColumns(cols) => {
            out.direct_columns.extend(cols);
        }
        ColumnTarget::Pair(a, b) => {
            out.exact_modal_pairs.push((usize::from(a), usize::from(b)));
            out.direct_columns.push(a.saturating_add(1));
            out.direct_columns.push(b.saturating_add(1));
            out.generated_befehle.insert("modallogik".to_string());
        }
        ColumnTarget::Generator(generator) => {
            out.generated_befehle
                .insert(generator.art.to_string().to_lowercase());
        }
        ColumnTarget::Combination(_combination) => {}
    }

    dedup_legacy_selection(&mut out);
    Ok(out)
}

fn resolve_via_legacy_pipeline(
    kategorie_map: &KategorieMap,
    ober: &str,
    unter: &str,
) -> Result<LegacyResolvedSelection, Box<dyn std::error::Error>> {
    let resolved = RawSelectionRequest::new(ober.to_string(), unter.to_string())
        .parse()
        .map_err(to_boxed_request_pipeline_error)?
        .expand(kategorie_map)
        .resolve(kategorie_map);

    let mut out = LegacyResolvedSelection::default();

    out.direct_columns = resolved
        .direct_columns
        .into_iter()
        .map(|n| u16::try_from(n).map_err(|_| format!("Spaltenindex {} passt nicht in u16", n)))
        .collect::<Result<Vec<u16>, _>>()?;

    out.required_columns = resolved
        .required_columns
        .into_iter()
        .map(|n| u16::try_from(n).map_err(|_| format!("Spaltenindex {} passt nicht in u16", n)))
        .collect::<Result<Vec<u16>, _>>()?;

    out.generated_befehle = resolved.generated_befehle;

    dedup_legacy_selection(&mut out);
    Ok(out)
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

fn to_boxed_request_pipeline_error(
    err: crate::domain::errors::RequestPipelineError,
) -> Box<dyn std::error::Error> {
    Box::<dyn std::error::Error>::from(err.to_string())
}
