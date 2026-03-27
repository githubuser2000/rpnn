use crate::domain::ids::domain_id::{DomainId, GebrochenRationalArt, GeneratorArt, KombinationsArt};
use crate::domain::model::spalten_anfrage as canonical;
use crate::domain::spalten_anfrage as legacy;
use crate::domain::spalten_anfrage::{MenschlichesUnter, ReligionUnter, SpaltenAnfrage, StandardAnfrage, StandardOberkategorie, UniversumUnter};

pub fn bridge_request(req: &SpaltenAnfrage) -> Option<canonical::SpaltenAnfrage> {
    match req {
        SpaltenAnfrage::Standard(standard) => bridge_standard(standard),
        SpaltenAnfrage::KombinationGalaxie { unter } => Some(canonical::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Galaxie,
            unter: map_kombi_unter(unter),
        }),
        SpaltenAnfrage::KombinationUniversum { unter } => Some(canonical::SpaltenAnfrage::Kombination {
            art: KombinationsArt::Universum,
            unter: map_kombi_unter(unter),
        }),
        SpaltenAnfrage::GebrochenRationalGalaxie { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Galaxie,
            index: unter.trim().parse().ok()?,
        }),
        SpaltenAnfrage::GebrochenRationalUniversum { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Universum,
            index: unter.trim().parse().ok()?,
        }),
        SpaltenAnfrage::GebrochenRationalGefuehle { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Gefuehle,
            index: unter.trim().parse().ok()?,
        }),
        SpaltenAnfrage::GebrochenRationalStrukturgroesse { unter } => Some(canonical::SpaltenAnfrage::GebrochenRational {
            art: GebrochenRationalArt::Strukturgroesse,
            index: unter.trim().parse().ok()?,
        }),
        SpaltenAnfrage::Primvielfache { unter } => Some(canonical::SpaltenAnfrage::Generator {
            art: GeneratorArt::Primvielfache,
            parameter: canonical::GeneratorParameter::Text(unter.clone()),
        }),
        SpaltenAnfrage::Multiplikationen { unter } => Some(canonical::SpaltenAnfrage::Generator {
            art: GeneratorArt::Multiplikationen,
            parameter: canonical::GeneratorParameter::Text(unter.clone()),
        }),
        SpaltenAnfrage::Unknown { .. } => None,
    }
}

fn bridge_standard(standard: &StandardAnfrage) -> Option<canonical::SpaltenAnfrage> {
    match standard {
        StandardAnfrage::Menschliches(unter) => Some(canonical::SpaltenAnfrage::Standard {
            domain: DomainId::Menschliches,
            unter: map_menschliches(unter)?,
        }),
        StandardAnfrage::Universum(unter) => Some(canonical::SpaltenAnfrage::Standard {
            domain: DomainId::Universum,
            unter: map_universum(unter)?,
        }),
        StandardAnfrage::Religion(unter) => Some(canonical::SpaltenAnfrage::Standard {
            domain: DomainId::Religion,
            unter: map_religion(unter)?,
        }),
        StandardAnfrage::Sonstige { ober, unter } => bridge_sonstige(ober, unter),
    }
}

fn bridge_sonstige(ober: &StandardOberkategorie, unter: &str) -> Option<canonical::SpaltenAnfrage> {
    let domain = match ober {
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
    };

    Some(canonical::SpaltenAnfrage::Standard {
        domain,
        unter: map_standard_unter_fallback(unter)?,
    })
}

fn map_menschliches(unter: &MenschlichesUnter) -> Option<canonical::StandardUnterId> {
    match unter {
        MenschlichesUnter::Gewalt => Some(canonical::StandardUnterId::Gewalt),
        MenschlichesUnter::Politische => Some(canonical::StandardUnterId::Politische),
        MenschlichesUnter::Richtungen => Some(canonical::StandardUnterId::Richtungen),
        MenschlichesUnter::Formationen => Some(canonical::StandardUnterId::Formationen),
        MenschlichesUnter::Klasse => Some(canonical::StandardUnterId::Klasse),
        MenschlichesUnter::Hoelle => Some(canonical::StandardUnterId::Hoelle),
        MenschlichesUnter::Liebe => Some(canonical::StandardUnterId::Liebe),
        MenschlichesUnter::Gleichheit => None,
        MenschlichesUnter::Motive => None,
        MenschlichesUnter::Sonstige(s) => map_standard_unter_fallback(s),
    }
}

fn map_universum(unter: &UniversumUnter) -> Option<canonical::StandardUnterId> {
    match unter {
        UniversumUnter::Geist => Some(canonical::StandardUnterId::Geist),
        UniversumUnter::Primzahlkreuz => Some(canonical::StandardUnterId::Primzahlkreuz),
        UniversumUnter::Sonstige(s) => map_standard_unter_fallback(s),
    }
}

fn map_religion(unter: &ReligionUnter) -> Option<canonical::StandardUnterId> {
    match unter {
        ReligionUnter::Religion => Some(canonical::StandardUnterId::SymboleReligion),
        ReligionUnter::Ethik => None,
        ReligionUnter::Sonstige(s) => map_standard_unter_fallback(s),
    }
}

fn map_kombi_unter(input: &str) -> canonical::KombiUnterId {
    match normalize_key(input).as_str() {
        "tiere" => canonical::KombiUnterId::Tiere,
        "berufe" => canonical::KombiUnterId::Berufe,
        "religion" => canonical::KombiUnterId::Religion,
        "politik" => canonical::KombiUnterId::Politik,
        _ => canonical::KombiUnterId::Unbekannt,
    }
}

fn map_standard_unter_fallback(input: &str) -> Option<canonical::StandardUnterId> {
    match normalize_key(input).as_str() {
        "gewalt" => Some(canonical::StandardUnterId::Gewalt),
        "politische" => Some(canonical::StandardUnterId::Politische),
        "richtungen" => Some(canonical::StandardUnterId::Richtungen),
        "formationen" => Some(canonical::StandardUnterId::Formationen),
        "klasse" => Some(canonical::StandardUnterId::Klasse),
        "hoelle" => Some(canonical::StandardUnterId::Hoelle),
        "liebe" => Some(canonical::StandardUnterId::Liebe),
        "geist" => Some(canonical::StandardUnterId::Geist),
        "religion" | "symbolereligion" | "symbolereligionen" => Some(canonical::StandardUnterId::SymboleReligion),
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

fn normalize_key(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss")
}
