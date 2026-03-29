use crate::domain::python_source_of_truth;
use crate::domain::ids::domain_id::{GebrochenRationalArt, GeneratorArt, KombinationsArt};
use crate::domain::model::spalten_anfrage::{
    CanonicalColumnSpec, ColumnTarget, CombinationSpec, EigenschaftRequest, GeneratorParameter, GeneratorSpec,
    KombiUnterId, SpaltenAnfrage, StandardUnterId,
};

pub fn resolve_request(req: SpaltenAnfrage) -> Option<CanonicalColumnSpec> {
    match &req {
        SpaltenAnfrage::Standard { unter, .. } => resolve_standard(req.clone(), unter.clone()),
        SpaltenAnfrage::GebrochenRational { art, index } => {
            resolve_gebrochen_rational(req.clone(), *art, *index)
        }
        SpaltenAnfrage::Kombination { art, unter } => {
            resolve_kombination(req.clone(), *art, *unter)
        }
        SpaltenAnfrage::Generator { art, parameter } => {
            resolve_generator(req.clone(), *art, parameter.clone())
        }
        SpaltenAnfrage::DirektSpalten { ids } => {
            let ids_cloned = ids.clone();
            Some(CanonicalColumnSpec {
                request: req.clone(),
                target: ColumnTarget::DirectColumns(ids_cloned.clone()),
                header_display: format!("DirektSpalten {:?}", ids_cloned),
                aliases_for_report: vec![],
            })
        },
    }
}

fn resolve_standard(req: SpaltenAnfrage, unter: StandardUnterId) -> Option<CanonicalColumnSpec> {
    if let StandardUnterId::Eigenschaft(spec) = unter.clone() {
        return resolve_eigenschaft(req, spec);
    }

    if let StandardUnterId::Primzahlkreuz = unter {
        return Some(CanonicalColumnSpec {
            request: req,
            target: ColumnTarget::Generator(GeneratorSpec {
                art: GeneratorArt::Primzahlkreuz,
                parameter: GeneratorParameter::Keine,
            }),
            header_display: "Primzahlkreuz".to_string(),
            aliases_for_report: vec![],
        });
    }

    if let Some((ober, unter_name)) = req.to_cli_pair() {
        let cols = python_source_of_truth::exact_all_direct_columns_for_pair(&ober, &unter_name);

        let header_display = unter_name.clone();
        let target = match cols.len() {
            0 => return None,
            1 => ColumnTarget::DirectColumn(cols[0] as u16),
            _ => ColumnTarget::DirectColumns(cols.into_iter().map(|n| n as u16).collect()),
        };

        return Some(CanonicalColumnSpec {
            request: req,
            target,
            header_display,
            aliases_for_report: vec![],
        });
    }

    None
}

fn resolve_eigenschaft(req: SpaltenAnfrage, spec: EigenschaftRequest) -> Option<CanonicalColumnSpec> {
    let key = spec.key;
    let target = if let Some((left, right)) = key.maybe_pair() {
        ColumnTarget::Pair((left as u16) + 1, (right as u16) + 1)
    } else if key.direct_columns().len() == 1 {
        ColumnTarget::DirectColumn((key.direct_columns()[0] as u16) + 1)
    } else if !key.direct_columns().is_empty() {
        ColumnTarget::DirectColumns(key.direct_columns().iter().map(|n| (*n as u16) + 1).collect())
    } else {
        return None;
    };

    Some(CanonicalColumnSpec {
        request: req,
        target,
        header_display: key.canonical_name().to_string(),
        aliases_for_report: key.aliases().iter().map(|s| (*s).to_string()).collect(),
    })
}

fn resolve_gebrochen_rational(
    req: SpaltenAnfrage,
    art: GebrochenRationalArt,
    index: u16,
) -> Option<CanonicalColumnSpec> {
    let header_display = match art {
        GebrochenRationalArt::Galaxie => format!("gebrochen-rational_Galaxie_n/m {index}"),
        GebrochenRationalArt::Universum => format!("gebrochen-rational_Universum_n/m {index}"),
        GebrochenRationalArt::Gefuehle => format!("gebrochen-rational_Gefuehle_n/m {index}"),
        GebrochenRationalArt::Strukturgroesse => {
            format!("gebrochen-rational_Strukturgroesse_n/m {index}")
        }
    };

    Some(CanonicalColumnSpec {
        request: req,
        target: ColumnTarget::DirectColumn(index),
        header_display,
        aliases_for_report: vec![],
    })
}

fn resolve_kombination(
    req: SpaltenAnfrage,
    art: KombinationsArt,
    unter: KombiUnterId,
) -> Option<CanonicalColumnSpec> {
    Some(CanonicalColumnSpec {
        request: req,
        target: ColumnTarget::Combination(CombinationSpec { art, unter }),
        header_display: format!("{art:?} {unter:?}"),
        aliases_for_report: vec![],
    })
}

fn resolve_generator(
    req: SpaltenAnfrage,
    art: GeneratorArt,
    parameter: GeneratorParameter,
) -> Option<CanonicalColumnSpec> {
    let header_display = match art {
        GeneratorArt::Primzahlkreuz => "Primzahlkreuz",
        GeneratorArt::Multiplikationen => "Multiplikationen",
        GeneratorArt::Primvielfache => "Primvielfache",
        GeneratorArt::MetaKonkret => "MetaKonkret",
    }
    .to_string();

    Some(CanonicalColumnSpec {
        request: req,
        target: ColumnTarget::Generator(GeneratorSpec { art, parameter }),
        header_display,
        aliases_for_report: vec![],
    })
}
