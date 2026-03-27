use crate::domain::ids::domain_id::{DomainId, GebrochenRationalArt, GeneratorArt, KombinationsArt};
use crate::domain::model::spalten_anfrage as canonical;
use crate::domain::spalten_anfrage as legacy;

pub fn to_canonical_request(request: &legacy::SpaltenAnfrage) -> Option<canonical::SpaltenAnfrage> {
    match request {
        legacy::SpaltenAnfrage::Standard(legacy::StandardAnfrage::Menschliches(unter)) => Some(canonical::SpaltenAnfrage::Standard {
            domain: DomainId::Menschliches,
            unter: map_menschliches(unter)?,
        }),
        legacy::SpaltenAnfrage::Standard(legacy::StandardAnfrage::Universum(unter)) => Some(canonical::SpaltenAnfrage::Standard {
            domain: DomainId::Universum,
            unter: map_universum(unter)?,
        }),
        legacy::SpaltenAnfrage::Standard(legacy::StandardAnfrage::Religion(unter)) => Some(canonical::SpaltenAnfrage::Standard {
            domain: DomainId::Religion,
            unter: map_religion(unter)?,
        }),
        legacy::SpaltenAnfrage::Standard(legacy::StandardAnfrage::Sonstige { ober, unter }) => Some(canonical::SpaltenAnfrage::Standard {
            domain: map_standard_ober(ober)?,
            unter: map_standard_unter_fallback(unter)?,
        }),
        legacy::SpaltenAnfrage::GebrochenRationalGalaxie { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Galaxie,
            index: unter.trim().parse().ok()?,
        }),
        legacy::SpaltenAnfrage::GebrochenRationalUniversum { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Universum,
            index: unter.trim().parse().ok()?,
        }),
        legacy::SpaltenAnfrage::GebrochenRationalGefuehle { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Gefuehle,
            index: unter.trim().parse().ok()?,
        }),
        legacy::SpaltenAnfrage::GebrochenRationalStrukturgroesse { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Strukturgroesse,
            index: unter.trim().parse().ok()?,
        }),
        legacy::SpaltenAnfrage::KombinationGalaxie { unter } => Some(canonical::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Galaxie,
            unter: map_kombi_unter(unter),
        }),
        legacy::SpaltenAnfrage::KombinationUniversum { unter } => Some(canonical::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Universum,
            unter: map_kombi_unter(unter),
        }),
        legacy::SpaltenAnfrage::Primvielfache { unter } => Some(canonical::SpaltenAnfrage::Generator {
            art: GeneratorArt::Primvielfache,
            parameter: canonical::GeneratorParameter::Text(unter.clone()),
        }),
        legacy::SpaltenAnfrage::Multiplikationen { unter } => Some(canonical::SpaltenAnfrage::Generator {
            art: GeneratorArt::Multiplikationen,
            parameter: canonical::GeneratorParameter::Text(unter.clone()),
        }),
        legacy::SpaltenAnfrage::Unknown { .. } => None,
    }
}

fn map_standard_ober(ober: &legacy::StandardOberkategorie) -> Option<DomainId> {
    match ober {
        legacy::StandardOberkategorie::Menschliches => Some(DomainId::Menschliches),
        legacy::StandardOberkategorie::Universum => Some(DomainId::Universum),
        legacy::StandardOberkategorie::Religion => Some(DomainId::Religion),
        legacy::StandardOberkategorie::Planet => Some(DomainId::Planet10Oder12),
        legacy::StandardOberkategorie::Galaxie => Some(DomainId::Galaxie),
        legacy::StandardOberkategorie::Multiversum => Some(DomainId::Multiversum),
        legacy::StandardOberkategorie::Grundstrukturen => Some(DomainId::Grundstrukturen),
        legacy::StandardOberkategorie::EigenschaftenN => Some(DomainId::EigenschaftenN),
        legacy::StandardOberkategorie::Eigenschaften1ProN => Some(DomainId::Eigenschaften1ProN),
        _ => None,
    }
}

fn map_menschliches(unter: &legacy::MenschlichesUnter) -> Option<canonical::StandardUnterId> {
    match unter {
        legacy::MenschlichesUnter::Gewalt => Some(canonical::StandardUnterId::Gewalt),
        legacy::MenschlichesUnter::Politische => Some(canonical::StandardUnterId::Politische),
        legacy::MenschlichesUnter::Richtungen => Some(canonical::StandardUnterId::Richtungen),
        legacy::MenschlichesUnter::Formationen => Some(canonical::StandardUnterId::Formationen),
        legacy::MenschlichesUnter::Klasse => Some(canonical::StandardUnterId::Klasse),
        legacy::MenschlichesUnter::Hoelle => Some(canonical::StandardUnterId::Hoelle),
        legacy::MenschlichesUnter::Liebe => Some(canonical::StandardUnterId::Liebe),
        legacy::MenschlichesUnter::Gleichheit => None,
        legacy::MenschlichesUnter::Motive => None,
        legacy::MenschlichesUnter::Sonstige(s) => map_standard_unter_fallback(s),
    }
}

fn map_universum(unter: &legacy::UniversumUnter) -> Option<canonical::StandardUnterId> {
    match unter {
        legacy::UniversumUnter::Geist => Some(canonical::StandardUnterId::Geist),
        legacy::UniversumUnter::Primzahlkreuz => Some(canonical::StandardUnterId::Primzahlkreuz),
        legacy::UniversumUnter::Sonstige(s) => map_standard_unter_fallback(s),
    }
}

fn map_religion(unter: &legacy::ReligionUnter) -> Option<canonical::StandardUnterId> {
    match unter {
        legacy::ReligionUnter::Religion => Some(canonical::StandardUnterId::SymboleReligion),
        legacy::ReligionUnter::Ethik => Some(canonical::StandardUnterId::Liebe),
        legacy::ReligionUnter::Sonstige(s) => map_standard_unter_fallback(s),
    }
}

fn map_standard_unter_fallback(s: &str) -> Option<canonical::StandardUnterId> {
    let key = legacy::normalize_key(s);
    match key.as_str() {
        "gewalt" => Some(canonical::StandardUnterId::Gewalt),
        "politische" => Some(canonical::StandardUnterId::Politische),
        "richtungen" => Some(canonical::StandardUnterId::Richtungen),
        "formationen" => Some(canonical::StandardUnterId::Formationen),
        "klasse" => Some(canonical::StandardUnterId::Klasse),
        "hoelle" => Some(canonical::StandardUnterId::Hoelle),
        "liebe" => Some(canonical::StandardUnterId::Liebe),
        "geist" => Some(canonical::StandardUnterId::Geist),
        "religion" | "symbolereligion" => Some(canonical::StandardUnterId::SymboleReligion),
        "primzahlkreuz" => Some(canonical::StandardUnterId::Primzahlkreuz),
        "wuerdig" => Some(canonical::StandardUnterId::Wuerdig),
        "regelvsausnahme" => Some(canonical::StandardUnterId::RegelVsAusnahme),
        "filterartwidrigkeit" => Some(canonical::StandardUnterId::FilterartWidrigkeit),
        "werte" => Some(canonical::StandardUnterId::Werte),
        "gutartigkeitsegoismus" => Some(canonical::StandardUnterId::GutartigkeitsEgoismus),
        "reflektierenerkenntniserkennen" => Some(canonical::StandardUnterId::ReflektierenErkenntnisErkennen),
        "vertrauenwollen" => Some(canonical::StandardUnterId::VertrauenWollen),
        "ausrichteneinrichten" => Some(canonical::StandardUnterId::AusrichtenEinrichten),
        "toleranzrespektakzeptanzwillkommen" => Some(canonical::StandardUnterId::ToleranzRespektAkzeptanzWillkommen),
        _ => None,
    }
}

fn map_kombi_unter(s: &str) -> canonical::KombiUnterId {
    match legacy::normalize_key(s).as_str() {
        "tiere" => canonical::KombiUnterId::Tiere,
        "berufe" => canonical::KombiUnterId::Berufe,
        "religion" => canonical::KombiUnterId::Religion,
        "politik" => canonical::KombiUnterId::Politik,
        _ => canonical::KombiUnterId::Unbekannt,
    }
}
