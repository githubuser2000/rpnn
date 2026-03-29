use crate::domain::ids::domain_id::{DomainId, GebrochenRationalArt, GeneratorArt, KombinationsArt};
use crate::domain::python_source_of_truth::{self, is_strict_generated_pair};
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
    let domain = match &req {
        SpaltenAnfrage::Standard { domain, .. } => *domain,
        _ => return None,
    };

    let (target, header_display) = match unter {
        StandardUnterId::Eigenschaft(spec) => return resolve_eigenschaft(req, spec),
        StandardUnterId::PythonSubcategory(sub) => {
            let ober = match domain {
                DomainId::Menschliches => "Menschliches",
                DomainId::Religion => "Religion",
                DomainId::Galaxie => "Galaxie",
                DomainId::Universum => "Universum",
                DomainId::Grundstrukturen => "Grundstrukturen",
                DomainId::Kontinuum => "Kontinuum",
                DomainId::Multiversum => "Multiversum",
                DomainId::Planet10Oder12 => "Planet",
                DomainId::Eigenschaften => "Eigenschaften",
                DomainId::EigenschaftenN => "Eigenschaften_n",
                DomainId::Eigenschaften1ProN => "Eigenschaften_1/n",
                DomainId::MetaKonkret => "MetaKonkret",
                DomainId::GebrochenRational(_) | DomainId::Kombination(_) | DomainId::Generator(_) | DomainId::SonstigePythonDecl => return None,
            };
            if is_strict_generated_pair(ober, &sub) {
                return Some(CanonicalColumnSpec {
                    request: req,
                    target: ColumnTarget::Generator(GeneratorSpec {
                        art: GeneratorArt::Primzahlkreuz,
                        parameter: GeneratorParameter::Keine,
                    }),
                    header_display: sub,
                    aliases_for_report: vec![],
                });
            }
            let mut exact: Vec<u16> = python_source_of_truth::exact_all_direct_columns_for_pair(&ober, &sub)
                .into_iter()
                .map(|n| (n as u16) + 1)
                .collect();
            exact.sort();
            exact.dedup();
            let target = match exact.len() {
                0 => return None,
                1 => ColumnTarget::DirectColumn(exact[0]),
                _ => ColumnTarget::DirectColumns(exact),
            };
            (target, sub)
        }

        // Platzhalter/erste Brücke – diese IDs später gegen Python-Wahrheit austauschen
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
        StandardUnterId::Primzahlkreuz => {
            let ober = match domain {
                DomainId::Menschliches => "Menschliches",
                DomainId::Religion => "Religion",
                DomainId::Galaxie => "Galaxie",
                DomainId::Universum => "Universum",
                DomainId::Grundstrukturen => "Grundstrukturen",
                DomainId::Kontinuum => "Kontinuum",
                DomainId::Multiversum => "Multiversum",
                DomainId::Planet10Oder12 => "Planet",
                DomainId::Eigenschaften => "Eigenschaften",
                DomainId::EigenschaftenN => "Eigenschaften_n",
                DomainId::Eigenschaften1ProN => "Eigenschaften_1/n",
                DomainId::MetaKonkret => "MetaKonkret",
                DomainId::GebrochenRational(_) | DomainId::Kombination(_) | DomainId::Generator(_) | DomainId::SonstigePythonDecl => return None,
            };
            if is_strict_generated_pair(ober, "Primzahlkreuz") {
                (
                    ColumnTarget::Generator(GeneratorSpec {
                        art: GeneratorArt::Primzahlkreuz,
                        parameter: GeneratorParameter::Keine,
                    }),
                    "Primzahlkreuz".to_string(),
                )
            } else {
                let mut exact: Vec<u16> = python_source_of_truth::exact_columns_for_pair(ober, "Primzahlkreuz")
                    .into_iter().map(|n| (n as u16) + 1).collect();
                exact.sort();
                exact.dedup();
                let target = match exact.len() {
                    0 => ColumnTarget::Generator(GeneratorSpec {
                        art: GeneratorArt::Primzahlkreuz,
                        parameter: GeneratorParameter::Keine,
                    }),
                    1 => ColumnTarget::DirectColumn(exact[0]),
                    _ => ColumnTarget::DirectColumns(exact),
                };
                (target, "Primzahlkreuz".to_string())
            }
        },
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
