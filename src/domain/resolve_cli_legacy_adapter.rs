use std::collections::BTreeSet;

use crate::domain::categories::KategorieMap;
use crate::domain::exact_generator_bridge::resolve_exact_generator;
use crate::domain::ids::domain_id::GeneratorArt;
use crate::domain::model::spalten_anfrage::ColumnTarget;
use crate::domain::python_source_of_truth::{exact_columns_for_pair, is_strict_generated_pair};
use crate::domain::request_bridge::bridge_cli_selection;
use crate::domain::request_pipeline::RawSelectionRequest;
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
    kategorie_map: &KategorieMap,
    ober: &str,
    unter: &str,
) -> Result<LegacyResolvedSelection, Box<dyn std::error::Error>> {
    if let Some(canonical) = bridge_cli_selection(ober, unter).and_then(resolve_request) {
        let mut out = LegacyResolvedSelection::default();
        apply_canonical_spec(&mut out, canonical.target);
        dedup_legacy_selection(&mut out);
        return Ok(out);
    }

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

    if let Some(non_legacy) = resolve_via_non_legacy_exact(kategorie_map, ober, unter)? {
        return Ok(non_legacy);
    }

    resolve_via_legacy_pipeline(kategorie_map, ober, unter)
}

fn resolve_via_non_legacy_exact(
    kategorie_map: &KategorieMap,
    ober: &str,
    unter: &str,
) -> Result<Option<LegacyResolvedSelection>, Box<dyn std::error::Error>> {
    let mut out = LegacyResolvedSelection::default();

    let mut direct_columns: Vec<u16> = Vec::new();

    direct_columns.extend(
        kategorie_map
            .finde_spaltennummern_fuer_kategorien(ober, unter)
            .into_iter()
            .map(|n| u16::try_from(n).map_err(|_| format!("Spaltenindex {} passt nicht in u16", n)))
            .collect::<Result<Vec<u16>, _>>()?,
    );

    if !is_strict_generated_pair(ober, unter) {
        direct_columns.extend(
            exact_columns_for_pair(ober, unter)
                .into_iter()
                .map(|n| u16::try_from(n).map_err(|_| format!("Spaltenindex {} passt nicht in u16", n)))
                .collect::<Result<Vec<u16>, _>>()?,
        );
    }

    direct_columns.sort_unstable();
    direct_columns.dedup();
    out.direct_columns = direct_columns;

    if let Some(inference) = kategorie_map.infer_generated_pair(ober, unter) {
        out.generated_befehle.extend(inference.generated_befehle);
        out.required_columns = inference
            .required_columns
            .into_iter()
            .map(|n| u16::try_from(n).map_err(|_| format!("Spaltenindex {} passt nicht in u16", n)))
            .collect::<Result<Vec<u16>, _>>()?;

        let inferred_direct_columns = inference
            .direct_columns
            .into_iter()
            .map(|n| u16::try_from(n).map_err(|_| format!("Spaltenindex {} passt nicht in u16", n)))
            .collect::<Result<Vec<u16>, _>>()?;
        out.direct_columns.extend(inferred_direct_columns);
    }

    dedup_legacy_selection(&mut out);

    if out.direct_columns.is_empty()
        && out.required_columns.is_empty()
        && out.exact_direct_columns.is_empty()
        && out.exact_modal_pairs.is_empty()
        && out.exact_meta_konkret_specs.is_empty()
        && out.generated_befehle.is_empty()
    {
        return Ok(None);
    }

    Ok(Some(out))
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
        .resolve(kategorie_map).map_err(to_boxed_request_pipeline_error)?;

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



fn canonical_generator_command(art: GeneratorArt) -> &'static str {
    match art {
        GeneratorArt::Primzahlkreuz => "primzahlkreuzprocontra",
        GeneratorArt::Multiplikationen => "multiplikationen",
        GeneratorArt::Primvielfache => "primvielfache",
        GeneratorArt::MetaKonkret => "metakonkret",
    }
}

fn apply_canonical_spec(sel: &mut LegacyResolvedSelection, target: ColumnTarget) {
    match target {
        ColumnTarget::DirectColumn(col) => sel.direct_columns.push(col),
        ColumnTarget::DirectColumns(cols) => sel.direct_columns.extend(cols),
        ColumnTarget::Pair(a, b) => sel.exact_modal_pairs.push((usize::from(a), usize::from(b))),
        ColumnTarget::Generator(generator_spec) => {
            sel.generated_befehle
                .insert(canonical_generator_command(generator_spec.art).to_string());
        }
        ColumnTarget::Combination(_) => {}
    }
}
