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
    let (target, header_display) = match unter {
        StandardUnterId::Eigenschaft(spec) => return resolve_eigenschaft(req, spec),
        StandardUnterId::PythonSubcategory(sub) => {
            let ober = match &req {
                SpaltenAnfrage::Standard { domain, .. } => domain_cli_name(*domain)?,
                _ => return None,
            };
            let mut cols: Vec<u16> = python_source_of_truth::exact_columns_for_pair(ober, &sub)
                .into_iter()
                .map(|n| n as u16 + 1)
                .collect();
            if cols.is_empty() {
                cols = python_source_of_truth::fuzzy_columns_for_pair(ober, &sub)
                    .into_iter()
                    .map(|n| n as u16 + 1)
                    .collect();
            }
            let target = match cols.len() {
                0 => return None,
                1 => ColumnTarget::DirectColumn(cols[0]),
                _ => ColumnTarget::DirectColumns(cols),
            };
            (target, sub)
        }
        StandardUnterId::Gewalt => (ColumnTarget::DirectColumn(496), "Gewalt".to_string()),
        StandardUnterId::Politische => (ColumnTarget::DirectColumn(497), "politische".to_string()),
        StandardUnterId::Richtungen => (ColumnTarget::DirectColumn(498), "Richtungen".to_string()),
        StandardUnterId::Formationen => {
            (ColumnTarget::DirectColumn(499), "Formationen".to_string())
        }
        StandardUnterId::Klasse => (ColumnTarget::DirectColumn(242), "Klasse".to_string()),
        StandardUnterId::Hoelle => (ColumnTarget::DirectColumn(496), "Hölle".to_string()),
        StandardUnterId::Liebe => (ColumnTarget::DirectColumn(14), "Liebe".to_string()),
        StandardUnterId::Geist => (ColumnTarget::DirectColumn(15), "Geist".to_string()),
        StandardUnterId::SymboleReligion => {
            (ColumnTarget::DirectColumn(700), "Symbole Religion".to_string())
        }
        StandardUnterId::Primzahlkreuz => (
            ColumnTarget::Generator(GeneratorSpec {
                art: GeneratorArt::Primzahlkreuz,
                parameter: GeneratorParameter::Keine,
            }),
            "Primzahlkreuz".to_string(),
        ),
    };

    Some(CanonicalColumnSpec {
        request: req,
        target,
        header_display,
        aliases_for_report: vec![],
    })
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

fn domain_cli_name(domain: crate::domain::ids::domain_id::DomainId) -> Option<&'static str> {
    use crate::domain::ids::domain_id::DomainId;
    match domain {
        DomainId::Menschliches => Some("Menschliches"),
        DomainId::Religion => Some("Religion"),
        DomainId::Galaxie => Some("Galaxie"),
        DomainId::Universum => Some("Universum"),
        DomainId::Grundstrukturen => Some("Grundstrukturen"),
        DomainId::Kontinuum => Some("Kontinuum"),
        DomainId::Multiversum => Some("Multiversum"),
        DomainId::Planet10Oder12 => Some("Planet"),
        DomainId::MetaKonkret => Some("MetaKonkret"),
        _ => None,
    }
}
