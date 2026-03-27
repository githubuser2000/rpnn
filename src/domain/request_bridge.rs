use crate::domain::ids::domain_id::{DomainId, GebrochenRationalArt, GeneratorArt, KombinationsArt};
use crate::domain::model::spalten_anfrage as model;
use crate::domain::spalten_anfrage::{
    MenschlichesUnter, ReligionUnter, SpaltenAnfrage, StandardAnfrage, StandardOberkategorie,
    UniversumUnter,
};

pub fn to_canonical_request(request: &SpaltenAnfrage) -> Option<model::SpaltenAnfrage> {
    match request {
        SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(unter)) => Some(model::SpaltenAnfrage::Standard {
            domain: DomainId::Menschliches,
            unter: map_menschliches(unter)?,
        }),
        SpaltenAnfrage::Standard(StandardAnfrage::Universum(unter)) => Some(model::SpaltenAnfrage::Standard {
            domain: DomainId::Universum,
            unter: map_universum(unter)?,
        }),
        SpaltenAnfrage::Standard(StandardAnfrage::Religion(unter)) => Some(model::SpaltenAnfrage::Standard {
            domain: DomainId::Religion,
            unter: map_religion(unter)?,
        }),
        SpaltenAnfrage::Standard(StandardAnfrage::Sonstige { ober, unter }) => Some(model::SpaltenAnfrage::Standard {
            domain: map_ober(ober),
            unter: map_standard_sonstige(unter)?,
        }),
        SpaltenAnfrage::KombinationGalaxie { unter } => Some(model::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Galaxie,
            unter: map_kombi_unter(unter),
        }),
        SpaltenAnfrage::KombinationUniversum { unter } => Some(model::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Universum,
            unter: map_kombi_unter(unter),
        }),
        SpaltenAnfrage::GebrochenRationalGalaxie { unter } => Some(model::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Galaxie, index: unter.parse().ok()?,
        }),
        SpaltenAnfrage::GebrochenRationalUniversum { unter } => Some(model::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Universum, index: unter.parse().ok()?,
        }),
        SpaltenAnfrage::GebrochenRationalGefuehle { unter } => Some(model::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Gefuehle, index: unter.parse().ok()?,
        }),
        SpaltenAnfrage::GebrochenRationalStrukturgroesse { unter } => Some(model::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Strukturgroesse, index: unter.parse().ok()?,
        }),
        SpaltenAnfrage::Primvielfache { unter } => Some(model::SpaltenAnfrage::Generator {
            art: GeneratorArt::Primvielfache, parameter: model::GeneratorParameter::Text(unter.clone()),
        }),
        SpaltenAnfrage::Multiplikationen { unter } => Some(model::SpaltenAnfrage::Generator {
            art: GeneratorArt::Multiplikationen, parameter: model::GeneratorParameter::Text(unter.clone()),
        }),
        SpaltenAnfrage::Unknown { .. } => None,
    }
}

fn map_ober(ober: &StandardOberkategorie) -> DomainId {
    match ober {
        StandardOberkategorie::Menschliches => DomainId::Menschliches,
        StandardOberkategorie::Universum => DomainId::Universum,
        StandardOberkategorie::Religion => DomainId::Religion,
        StandardOberkategorie::Planet => DomainId::Planet10Oder12,
        StandardOberkategorie::Galaxie => DomainId::Galaxie,
        StandardOberkategorie::Multiversum => DomainId::Multiversum,
        StandardOberkategorie::Grundstrukturen => DomainId::Grundstrukturen,
        StandardOberkategorie::Bedeutung => DomainId::SonstigePythonDecl,
        StandardOberkategorie::ProContra => DomainId::SonstigePythonDecl,
        StandardOberkategorie::WichtigstesZumVerstehen => DomainId::SonstigePythonDecl,
        StandardOberkategorie::EigenschaftenN => DomainId::EigenschaftenN,
        StandardOberkategorie::Eigenschaften1ProN => DomainId::Eigenschaften1ProN,
        StandardOberkategorie::UniversumMetaKonkret => DomainId::MetaKonkret,
        StandardOberkategorie::Sonstige(_) => DomainId::SonstigePythonDecl,
    }
}

fn map_standard_sonstige(unter: &str) -> Option<model::StandardUnterId> {
    match unter.trim().to_lowercase().replace('_', " ").as_str() {
        "würdig" | "wuerdig" => Some(model::StandardUnterId::Wuerdig),
        "regel vs ausnahme" => Some(model::StandardUnterId::RegelVsAusnahme),
        "filterart widrigkeit" => Some(model::StandardUnterId::FilterartWidrigkeit),
        "werte" => Some(model::StandardUnterId::Werte),
        "gutartigkeits-egoismus" | "gutartigkeits egoismus" => Some(model::StandardUnterId::GutartigkeitsEgoismus),
        "reflektieren erkenntnis-erkennen" | "reflektieren erkenntnis erkennen" => Some(model::StandardUnterId::ReflektierenErkenntnisErkennen),
        "vertrauen wollen" => Some(model::StandardUnterId::VertrauenWollen),
        "ausrichten einrichten" => Some(model::StandardUnterId::AusrichtenEinrichten),
        "toleranz respekt akzeptanz willkommen" => Some(model::StandardUnterId::ToleranzRespektAkzeptanzWillkommen),
        _ => None,
    }
}

fn map_menschliches(unter: &MenschlichesUnter) -> Option<model::StandardUnterId> {
    Some(match unter {
        MenschlichesUnter::Gewalt => model::StandardUnterId::Gewalt,
        MenschlichesUnter::Politische => model::StandardUnterId::Politische,
        MenschlichesUnter::Richtungen => model::StandardUnterId::Richtungen,
        MenschlichesUnter::Formationen => model::StandardUnterId::Formationen,
        MenschlichesUnter::Klasse => model::StandardUnterId::Klasse,
        MenschlichesUnter::Hoelle => model::StandardUnterId::Hoelle,
        MenschlichesUnter::Liebe => model::StandardUnterId::Liebe,
        MenschlichesUnter::Sonstige(_) => return None,
        MenschlichesUnter::Gleichheit => return None,
        MenschlichesUnter::Motive => return None,
    })
}
fn map_universum(unter: &UniversumUnter) -> Option<model::StandardUnterId> {
    Some(match unter {
        UniversumUnter::Geist => model::StandardUnterId::Geist,
        UniversumUnter::Primzahlkreuz => model::StandardUnterId::Primzahlkreuz,
        UniversumUnter::Sonstige(_) => return None,
    })
}
fn map_religion(unter: &ReligionUnter) -> Option<model::StandardUnterId> {
    Some(match unter {
        ReligionUnter::Religion => model::StandardUnterId::SymboleReligion,
        ReligionUnter::Ethik => model::StandardUnterId::Liebe,
        ReligionUnter::Sonstige(_) => return None,
    })
}
fn map_kombi_unter(unter: &str) -> model::KombiUnterId {
    match unter.trim().to_lowercase().as_str() {
        "tiere" => model::KombiUnterId::Tiere,
        "berufe" => model::KombiUnterId::Berufe,
        "religion" => model::KombiUnterId::Religion,
        "politik" => model::KombiUnterId::Politik,
        _ => model::KombiUnterId::Unbekannt,
    }
}
