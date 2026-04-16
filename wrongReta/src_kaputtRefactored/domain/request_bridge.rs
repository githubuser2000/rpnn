use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::{DomainId, GebrochenRationalArt, GeneratorArt, KombinationsArt};
use crate::domain::model::spalten_anfrage as canonical;
use crate::domain::model::spalten_anfrage::{EigenschaftRequest, EigenschaftsFamilie, GeneratorParameter, KombiUnterId, StandardUnterId};
use crate::domain::spalten_anfrage as legacy;

fn canonical_standard(domain: DomainId, unter: StandardUnterId) -> canonical::SpaltenAnfrage {
    canonical::SpaltenAnfrage::Standard { domain, unter }
}

fn canonical_python(domain: DomainId, unter: impl Into<String>) -> canonical::SpaltenAnfrage {
    canonical_standard(domain, StandardUnterId::PythonSubcategory(unter.into()))
}

fn map_eigenschaft(
    familie: EigenschaftsFamilie,
    unter: &str,
    fallback_domain: DomainId,
) -> canonical::SpaltenAnfrage {
    if let Some(key) = EigenschaftKeyId::from_alias(unter) {
        return canonical_standard(
            fallback_domain,
            StandardUnterId::Eigenschaft(EigenschaftRequest { familie, key }),
        );
    }

    canonical_python(fallback_domain, unter)
}

fn map_menschliches(unter: &legacy::MenschlichesUnter) -> canonical::SpaltenAnfrage {
    let mapped = match unter {
        legacy::MenschlichesUnter::Liebe => StandardUnterId::Liebe,
        legacy::MenschlichesUnter::Gleichheit => StandardUnterId::PythonSubcategory("Gleichheit".to_string()),
        legacy::MenschlichesUnter::Hoelle => StandardUnterId::Hoelle,
        legacy::MenschlichesUnter::Klasse => StandardUnterId::Klasse,
        legacy::MenschlichesUnter::Gewalt => StandardUnterId::Gewalt,
        legacy::MenschlichesUnter::Politische => StandardUnterId::Politische,
        legacy::MenschlichesUnter::Richtungen => StandardUnterId::Richtungen,
        legacy::MenschlichesUnter::Formationen => StandardUnterId::Formationen,
        legacy::MenschlichesUnter::Motive => StandardUnterId::PythonSubcategory("Motive".to_string()),
        legacy::MenschlichesUnter::Sonstige(name) => StandardUnterId::PythonSubcategory(name.clone()),
    };
    canonical_standard(DomainId::Menschliches, mapped)
}

fn map_universum(unter: &legacy::UniversumUnter) -> canonical::SpaltenAnfrage {
    let mapped = match unter {
        legacy::UniversumUnter::Geist => StandardUnterId::Geist,
        legacy::UniversumUnter::Primzahlkreuz => StandardUnterId::Primzahlkreuz,
        legacy::UniversumUnter::Sonstige(name) => StandardUnterId::PythonSubcategory(name.clone()),
    };
    canonical_standard(DomainId::Universum, mapped)
}

fn map_religion(unter: &legacy::ReligionUnter) -> canonical::SpaltenAnfrage {
    let mapped = match unter {
        legacy::ReligionUnter::Religion => StandardUnterId::SymboleReligion,
        legacy::ReligionUnter::Ethik => StandardUnterId::PythonSubcategory("Ethik".to_string()),
        legacy::ReligionUnter::Sonstige(name) => StandardUnterId::PythonSubcategory(name.clone()),
    };
    canonical_standard(DomainId::Religion, mapped)
}

fn map_standard_sonstige(
    ober: &legacy::StandardOberkategorie,
    unter: &str,
) -> canonical::SpaltenAnfrage {
    match ober {
        legacy::StandardOberkategorie::Planet => canonical_python(DomainId::Planet10Oder12, unter),
        legacy::StandardOberkategorie::Galaxie => canonical_python(DomainId::Galaxie, unter),
        legacy::StandardOberkategorie::Multiversum => canonical_python(DomainId::Multiversum, unter),
        legacy::StandardOberkategorie::Grundstrukturen => canonical_python(DomainId::Grundstrukturen, unter),
        legacy::StandardOberkategorie::Bedeutung => canonical_python(DomainId::SonstigePythonDecl, unter),
        legacy::StandardOberkategorie::ProContra => canonical_python(DomainId::SonstigePythonDecl, unter),
        legacy::StandardOberkategorie::WichtigstesZumVerstehen => canonical_python(DomainId::SonstigePythonDecl, unter),
        legacy::StandardOberkategorie::UniversumMetaKonkret => canonical::SpaltenAnfrage::Generator {
            art: GeneratorArt::MetaKonkret,
            parameter: GeneratorParameter::Text(unter.to_string()),
        },
        legacy::StandardOberkategorie::EigenschaftenN => {
            map_eigenschaft(EigenschaftsFamilie::N, unter, DomainId::EigenschaftenN)
        }
        legacy::StandardOberkategorie::Eigenschaften1ProN => {
            map_eigenschaft(EigenschaftsFamilie::EinsDurchN, unter, DomainId::Eigenschaften1ProN)
        }
        legacy::StandardOberkategorie::Sonstige(name) => canonical_python(
            if name == "Eigenschaften" { DomainId::Eigenschaften } else { DomainId::SonstigePythonDecl },
            unter,
        ),
        legacy::StandardOberkategorie::Menschliches
        | legacy::StandardOberkategorie::Universum
        | legacy::StandardOberkategorie::Religion => canonical_python(DomainId::SonstigePythonDecl, unter),
    }
}

pub fn bridge_cli_selection(ober: &str, unter: &str) -> Option<canonical::SpaltenAnfrage> {
    let legacy = legacy::SpaltenAnfrage::parse(ober, unter).ok()?;
    bridge_legacy_request(&legacy)
}

pub fn bridge_legacy_request(request: &legacy::SpaltenAnfrage) -> Option<canonical::SpaltenAnfrage> {
    let mapped = match request {
        legacy::SpaltenAnfrage::Standard(legacy::StandardAnfrage::Menschliches(unter)) => {
            map_menschliches(unter)
        }
        legacy::SpaltenAnfrage::Standard(legacy::StandardAnfrage::Universum(unter)) => {
            map_universum(unter)
        }
        legacy::SpaltenAnfrage::Standard(legacy::StandardAnfrage::Religion(unter)) => {
            map_religion(unter)
        }
        legacy::SpaltenAnfrage::Standard(legacy::StandardAnfrage::Sonstige { ober, unter }) => {
            map_standard_sonstige(ober, unter)
        }
        legacy::SpaltenAnfrage::KombinationGalaxie { unter } => canonical::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Galaxie,
            unter: match unter.as_str() {
                "tiere" => KombiUnterId::Tiere,
                "berufe" => KombiUnterId::Berufe,
                "religion" => KombiUnterId::Religion,
                "politik" => KombiUnterId::Politik,
                _ => KombiUnterId::Unbekannt,
            },
        },
        legacy::SpaltenAnfrage::KombinationUniversum { unter } => canonical::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Universum,
            unter: match unter.as_str() {
                "tiere" => KombiUnterId::Tiere,
                "berufe" => KombiUnterId::Berufe,
                "religion" => KombiUnterId::Religion,
                "politik" => KombiUnterId::Politik,
                _ => KombiUnterId::Unbekannt,
            },
        },
        legacy::SpaltenAnfrage::GebrochenRationalGalaxie { unter } => canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Galaxie,
            index: unter.parse().ok()?,
        },
        legacy::SpaltenAnfrage::GebrochenRationalUniversum { unter } => canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Universum,
            index: unter.parse().ok()?,
        },
        legacy::SpaltenAnfrage::GebrochenRationalGefuehle { unter } => canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Gefuehle,
            index: unter.parse().ok()?,
        },
        legacy::SpaltenAnfrage::GebrochenRationalStrukturgroesse { unter } => canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Strukturgroesse,
            index: unter.parse().ok()?,
        },
        legacy::SpaltenAnfrage::Primvielfache { unter } => canonical::SpaltenAnfrage::Generator {
            art: GeneratorArt::Primvielfache,
            parameter: GeneratorParameter::Text(unter.clone()),
        },
        legacy::SpaltenAnfrage::Multiplikationen { unter } => canonical::SpaltenAnfrage::Generator {
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
