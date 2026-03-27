use crate::domain::ids::domain_id::{DomainId, GebrochenRationalArt, KombinationsArt};
use crate::domain::model::spalten_anfrage as canonical;
use crate::domain::spalten_anfrage::{
    MenschlichesUnter, ReligionUnter, SpaltenAnfrage as LegacyRequest, StandardAnfrage,
    StandardOberkategorie, UniversumUnter,
};

pub fn bridge_request(req: &LegacyRequest) -> Option<canonical::SpaltenAnfrage> {
    match req {
        LegacyRequest::Standard(StandardAnfrage::Menschliches(unter)) => Some(canonical::SpaltenAnfrage::Standard {
            domain: DomainId::Menschliches,
            unter: map_menschliches(unter)?,
        }),
        LegacyRequest::Standard(StandardAnfrage::Universum(unter)) => Some(canonical::SpaltenAnfrage::Standard {
            domain: DomainId::Universum,
            unter: map_universum(unter)?,
        }),
        LegacyRequest::Standard(StandardAnfrage::Religion(unter)) => Some(canonical::SpaltenAnfrage::Standard {
            domain: DomainId::Religion,
            unter: map_religion(unter)?,
        }),
        LegacyRequest::Standard(StandardAnfrage::Sonstige { ober, unter }) => Some(canonical::SpaltenAnfrage::Standard {
            domain: map_ober(ober)?,
            unter: map_standard_unter_fallback(unter)?,
        }),
        LegacyRequest::KombinationGalaxie { unter } => Some(canonical::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Galaxie,
            unter: map_kombi_unter(unter),
        }),
        LegacyRequest::KombinationUniversum { unter } => Some(canonical::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Universum,
            unter: map_kombi_unter(unter),
        }),
        LegacyRequest::GebrochenRationalGalaxie { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Galaxie,
            index: unter.trim().parse().ok()?,
        }),
        LegacyRequest::GebrochenRationalUniversum { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Universum,
            index: unter.trim().parse().ok()?,
        }),
        LegacyRequest::GebrochenRationalGefuehle { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Gefuehle,
            index: unter.trim().parse().ok()?,
        }),
        LegacyRequest::GebrochenRationalStrukturgroesse { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Strukturgroesse,
            index: unter.trim().parse().ok()?,
        }),
        LegacyRequest::Primvielfache { unter } => Some(canonical::SpaltenAnfrage::Generator {
            art: crate::domain::ids::domain_id::GeneratorArt::Primvielfache,
            parameter: canonical::GeneratorParameter::Text(unter.clone()),
        }),
        LegacyRequest::Multiplikationen { unter } => Some(canonical::SpaltenAnfrage::Generator {
            art: crate::domain::ids::domain_id::GeneratorArt::Multiplikationen,
            parameter: canonical::GeneratorParameter::Text(unter.clone()),
        }),
        LegacyRequest::Unknown { .. } => None,
    }
}

fn map_ober(ober: &StandardOberkategorie) -> Option<DomainId> {
    Some(match ober {
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
        StandardOberkategorie::Sonstige(_) => return None,
    })
}

fn map_menschliches(unter: &MenschlichesUnter) -> Option<canonical::StandardUnterId> {
    Some(match unter {
        MenschlichesUnter::Gewalt => canonical::StandardUnterId::Gewalt,
        MenschlichesUnter::Politische => canonical::StandardUnterId::Politische,
        MenschlichesUnter::Richtungen => canonical::StandardUnterId::Richtungen,
        MenschlichesUnter::Formationen => canonical::StandardUnterId::Formationen,
        MenschlichesUnter::Klasse => canonical::StandardUnterId::Klasse,
        MenschlichesUnter::Hoelle => canonical::StandardUnterId::Hoelle,
        MenschlichesUnter::Liebe => canonical::StandardUnterId::Liebe,
        MenschlichesUnter::Gleichheit => return None,
        MenschlichesUnter::Motive => return None,
        MenschlichesUnter::Sonstige(_) => return None,
    })
}

fn map_universum(unter: &UniversumUnter) -> Option<canonical::StandardUnterId> {
    Some(match unter {
        UniversumUnter::Geist => canonical::StandardUnterId::Geist,
        UniversumUnter::Primzahlkreuz => canonical::StandardUnterId::Primzahlkreuz,
        UniversumUnter::Sonstige(_) => return None,
    })
}

fn map_religion(unter: &ReligionUnter) -> Option<canonical::StandardUnterId> {
    Some(match unter {
        ReligionUnter::Religion => canonical::StandardUnterId::SymboleReligion,
        ReligionUnter::Ethik => return None,
        ReligionUnter::Sonstige(_) => return None,
    })
}

fn map_standard_unter_fallback(unter: &str) -> Option<canonical::StandardUnterId> {
    match normalize(unter).as_str() {
        "gewalt" => Some(canonical::StandardUnterId::Gewalt),
        "politische" => Some(canonical::StandardUnterId::Politische),
        "richtungen" => Some(canonical::StandardUnterId::Richtungen),
        "formationen" => Some(canonical::StandardUnterId::Formationen),
        "klasse" => Some(canonical::StandardUnterId::Klasse),
        "hoelle" | "hölle" => Some(canonical::StandardUnterId::Hoelle),
        "liebe" => Some(canonical::StandardUnterId::Liebe),
        "geist" => Some(canonical::StandardUnterId::Geist),
        "religion" | "symbolereligion" => Some(canonical::StandardUnterId::SymboleReligion),
        "primzahlkreuz" => Some(canonical::StandardUnterId::Primzahlkreuz),
        _ => None,
    }
}

fn map_kombi_unter(unter: &str) -> canonical::KombiUnterId {
    match normalize(unter).as_str() {
        "tiere" => canonical::KombiUnterId::Tiere,
        "berufe" => canonical::KombiUnterId::Berufe,
        "religion" => canonical::KombiUnterId::Religion,
        "politik" => canonical::KombiUnterId::Politik,
        _ => canonical::KombiUnterId::Unbekannt,
    }
}

fn normalize(s: &str) -> String {
    s.trim().to_lowercase().replace('_', "").replace('-', "").replace(' ', "")
}
