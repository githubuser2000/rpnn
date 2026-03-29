use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::{DomainId, GebrochenRationalArt, GeneratorArt, KombinationsArt};
use crate::domain::model::spalten_anfrage as canonical;
use crate::domain::spalten_anfrage as legacy;

pub fn bridge_cli_selection(ober: &str, unter: &str) -> Option<canonical::SpaltenAnfrage> {
    let legacy = legacy::SpaltenAnfrage::parse(ober, unter).ok()?;
    bridge_legacy_request(&legacy)
}

pub fn bridge_legacy_request(request: &legacy::SpaltenAnfrage) -> Option<canonical::SpaltenAnfrage> {
    use canonical::{EigenschaftRequest, EigenschaftsFamilie, GeneratorParameter, KombiUnterId, SpaltenAnfrage, StandardUnterId};
    use legacy::{MenschlichesUnter, ReligionUnter, StandardAnfrage, StandardOberkategorie, UniversumUnter};

    let mapped = match request {
        legacy::SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(unter)) => {
            let unter = match unter {
                MenschlichesUnter::Liebe => StandardUnterId::Liebe,
                MenschlichesUnter::Gleichheit => StandardUnterId::PythonSubcategory("Gleichheit".to_string()),
                MenschlichesUnter::Hoelle => StandardUnterId::Hoelle,
                MenschlichesUnter::Klasse => StandardUnterId::Klasse,
                MenschlichesUnter::Gewalt => StandardUnterId::Gewalt,
                MenschlichesUnter::Politische => StandardUnterId::Politische,
                MenschlichesUnter::Richtungen => StandardUnterId::Richtungen,
                MenschlichesUnter::Formationen => StandardUnterId::Formationen,
                MenschlichesUnter::Motive => StandardUnterId::PythonSubcategory("Motive".to_string()),
                MenschlichesUnter::Sonstige(name) => StandardUnterId::PythonSubcategory(name.clone()),
            };
            SpaltenAnfrage::Standard {
                domain: DomainId::Menschliches,
                unter,
            }
        }
        legacy::SpaltenAnfrage::Standard(StandardAnfrage::Universum(unter)) => {
            let unter = match unter {
                UniversumUnter::Geist => StandardUnterId::Geist,
                UniversumUnter::Primzahlkreuz => StandardUnterId::Primzahlkreuz,
                UniversumUnter::Sonstige(name) => StandardUnterId::PythonSubcategory(name.clone()),
            };
            SpaltenAnfrage::Standard {
                domain: DomainId::Universum,
                unter,
            }
        }
        legacy::SpaltenAnfrage::Standard(StandardAnfrage::Religion(unter)) => {
            let unter = match unter {
                ReligionUnter::Religion => StandardUnterId::SymboleReligion,
                ReligionUnter::Ethik => StandardUnterId::PythonSubcategory("Ethik".to_string()),
                ReligionUnter::Sonstige(name) => StandardUnterId::PythonSubcategory(name.clone()),
            };
            SpaltenAnfrage::Standard {
                domain: DomainId::Religion,
                unter,
            }
        }
        legacy::SpaltenAnfrage::Standard(StandardAnfrage::Sonstige { ober, unter }) => {
            match ober {
                StandardOberkategorie::Menschliches => SpaltenAnfrage::Standard {
                    domain: DomainId::Menschliches,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::Universum => SpaltenAnfrage::Standard {
                    domain: DomainId::Universum,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::Religion => SpaltenAnfrage::Standard {
                    domain: DomainId::Religion,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::Planet => SpaltenAnfrage::Standard {
                    domain: DomainId::Planet10Oder12,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::Galaxie => SpaltenAnfrage::Standard {
                    domain: DomainId::Galaxie,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::Multiversum => SpaltenAnfrage::Standard {
                    domain: DomainId::Multiversum,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::Grundstrukturen => SpaltenAnfrage::Standard {
                    domain: DomainId::Grundstrukturen,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::Bedeutung => SpaltenAnfrage::Standard {
                    domain: DomainId::SonstigePythonDecl,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::ProContra => SpaltenAnfrage::Standard {
                    domain: DomainId::SonstigePythonDecl,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::WichtigstesZumVerstehen => SpaltenAnfrage::Standard {
                    domain: DomainId::SonstigePythonDecl,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::UniversumMetaKonkret => SpaltenAnfrage::Standard {
                    domain: DomainId::MetaKonkret,
                    unter: StandardUnterId::PythonSubcategory(unter.clone()),
                },
                StandardOberkategorie::EigenschaftenN => {
                    let key = EigenschaftKeyId::from_alias(unter)?;
                    SpaltenAnfrage::Standard {
                        domain: DomainId::EigenschaftenN,
                        unter: StandardUnterId::Eigenschaft(EigenschaftRequest {
                            familie: EigenschaftsFamilie::N,
                            key,
                        }),
                    }
                }
                StandardOberkategorie::Eigenschaften1ProN => {
                    let key = EigenschaftKeyId::from_alias(unter)?;
                    SpaltenAnfrage::Standard {
                        domain: DomainId::Eigenschaften1ProN,
                        unter: StandardUnterId::Eigenschaft(EigenschaftRequest {
                            familie: EigenschaftsFamilie::EinsDurchN,
                            key,
                        }),
                    }
                }
                StandardOberkategorie::Sonstige(_) => return None,
            }
        }
        legacy::SpaltenAnfrage::KombinationGalaxie { unter } => SpaltenAnfrage::Kombination {
            art: KombinationsArt::Galaxie,
            unter: map_kombi_unter(unter)?,
        },
        legacy::SpaltenAnfrage::KombinationUniversum { unter } => SpaltenAnfrage::Kombination {
            art: KombinationsArt::Universum,
            unter: map_kombi_unter(unter)?,
        },
        legacy::SpaltenAnfrage::GebrochenRationalGalaxie { unter } => SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Galaxie,
            index: unter.parse().ok()?,
        },
        legacy::SpaltenAnfrage::GebrochenRationalUniversum { unter } => {
            SpaltenAnfrage::GebrochenRational {
                art: GebrochenRationalArt::Universum,
                index: unter.parse().ok()?,
            }
        }
        legacy::SpaltenAnfrage::GebrochenRationalGefuehle { unter } => {
            SpaltenAnfrage::GebrochenRational {
                art: GebrochenRationalArt::Gefuehle,
                index: unter.parse().ok()?,
            }
        }
        legacy::SpaltenAnfrage::GebrochenRationalStrukturgroesse { unter } => {
            SpaltenAnfrage::GebrochenRational {
                art: GebrochenRationalArt::Strukturgroesse,
                index: unter.parse().ok()?,
            }
        }
        legacy::SpaltenAnfrage::Primvielfache { unter } => SpaltenAnfrage::Generator {
            art: GeneratorArt::Primvielfache,
            parameter: GeneratorParameter::Text(unter.clone()),
        },
        legacy::SpaltenAnfrage::Multiplikationen { unter } => SpaltenAnfrage::Generator {
            art: GeneratorArt::Multiplikationen,
            parameter: GeneratorParameter::Text(unter.clone()),
        },
        legacy::SpaltenAnfrage::Unknown { .. } => return None,
    };

    Some(mapped)
}

pub fn bridge_raw_selection(selection: &legacy::SpaltenAnfrage) -> Option<canonical::SpaltenAnfrage> {
    bridge_legacy_request(selection)
}

pub fn bridge_raw_pairs<I, O, U>(pairs: I) -> Vec<canonical::SpaltenAnfrage>
where
    I: IntoIterator<Item = (O, U)>,
    O: AsRef<str>,
    U: AsRef<str>,
{
    pairs
        .into_iter()
        .filter_map(|(ober, unter)| bridge_cli_selection(ober.as_ref(), unter.as_ref()))
        .collect()
}

fn map_kombi_unter(value: &str) -> Option<canonical::KombiUnterId> {
    match value.trim() {
        "tiere" => Some(canonical::KombiUnterId::Tiere),
        "berufe" => Some(canonical::KombiUnterId::Berufe),
        "religion" => Some(canonical::KombiUnterId::Religion),
        "politik" => Some(canonical::KombiUnterId::Politik),
        _ => None,
    }
}
