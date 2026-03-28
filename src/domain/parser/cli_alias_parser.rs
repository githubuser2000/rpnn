use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::{
    DomainId, GebrochenRationalArt, GeneratorArt, KombinationsArt,
};
use crate::domain::model::spalten_anfrage::{
    EigenschaftRequest, EigenschaftsFamilie, GeneratorParameter, KombiUnterId,
    SpaltenAnfrage, StandardUnterId,
};
use crate::domain::parser::cli_tokens::{KombiUnterToken, OberToken, StandardUnterToken};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnknownOberkategorie(String),
    UnknownUnterkategorie { ober: String, unter: String },
    InvalidGebrochenRationalIndex { ober: String, unter: String },
}

fn parse_eigenschaft_request(unter: &str, familie: EigenschaftsFamilie) -> Option<EigenschaftRequest> {
    EigenschaftKeyId::from_alias(unter).map(|key| EigenschaftRequest { familie, key })
}

pub fn parse_oberkategorie(input: &str) -> Result<DomainId, ParseError> {
    let token = OberToken::parse(input).ok_or_else(|| ParseError::UnknownOberkategorie(input.to_string()))?;

    Ok(match token {
        OberToken::Menschliches => DomainId::Menschliches,
        OberToken::Religion => DomainId::Religion,
        OberToken::Galaxie => DomainId::Galaxie,
        OberToken::Universum => DomainId::Universum,
        OberToken::Grundstrukturen => DomainId::Grundstrukturen,
        OberToken::Kontinuum => DomainId::Kontinuum,
        OberToken::Multiversum => DomainId::Multiversum,
        OberToken::Planet10Oder12 => DomainId::Planet10Oder12,
        OberToken::Eigenschaften => DomainId::Eigenschaften,
        OberToken::EigenschaftenN => DomainId::EigenschaftenN,
        OberToken::Eigenschaften1ProN => DomainId::Eigenschaften1ProN,
        OberToken::GebrochenRationalGalaxie => DomainId::GebrochenRational(GebrochenRationalArt::Galaxie),
        OberToken::GebrochenRationalUniversum => DomainId::GebrochenRational(GebrochenRationalArt::Universum),
        OberToken::GebrochenRationalGefuehle => DomainId::GebrochenRational(GebrochenRationalArt::Gefuehle),
        OberToken::GebrochenRationalStrukturgroesse => {
            DomainId::GebrochenRational(GebrochenRationalArt::Strukturgroesse)
        }
        OberToken::KombinationGalaxie => DomainId::Kombination(KombinationsArt::Galaxie),
        OberToken::KombinationUniversum => DomainId::Kombination(KombinationsArt::Universum),
        OberToken::KombinationGefuehle => DomainId::Kombination(KombinationsArt::Gefuehle),
        OberToken::KombinationStrukturgroesse => DomainId::Kombination(KombinationsArt::Strukturgroesse),
        OberToken::Primzahlkreuz => DomainId::Generator(GeneratorArt::Primzahlkreuz),
        OberToken::Multiplikationen => DomainId::Generator(GeneratorArt::Multiplikationen),
        OberToken::Primvielfache => DomainId::Generator(GeneratorArt::Primvielfache),
        OberToken::MetaKonkret => DomainId::MetaKonkret,
    })
}

pub fn parse_spalten_anfrage(ober: &str, unter: &str) -> Result<SpaltenAnfrage, ParseError> {
    let domain = parse_oberkategorie(ober)?;

    match domain {
        DomainId::Eigenschaften1ProN => Ok(SpaltenAnfrage::Standard {
            domain,
            unter: parse_eigenschaften_1_pro_n_unter(ober, unter)?,
        }),

        DomainId::EigenschaftenN => Ok(SpaltenAnfrage::Standard {
            domain,
            unter: parse_eigenschaften_n_unter(ober, unter)?,
        }),

        DomainId::Eigenschaften => Ok(SpaltenAnfrage::Standard {
            domain,
            unter: parse_eigenschaften_generisch_unter(ober, unter)?,
        }),

        DomainId::GebrochenRational(art) => Ok(SpaltenAnfrage::GebrochenRational {
            art,
            index: parse_u16_index(ober, unter)?,
        }),

        DomainId::Kombination(art) => Ok(SpaltenAnfrage::Kombination {
            art,
            unter: parse_kombi_unter(ober, unter)?,
        }),

        DomainId::Generator(art) => Ok(SpaltenAnfrage::Generator {
            art,
            parameter: GeneratorParameter::Text(unter.trim().to_string()),
        }),

        DomainId::Menschliches
        | DomainId::Religion
        | DomainId::Galaxie
        | DomainId::Universum
        | DomainId::Grundstrukturen
        | DomainId::Kontinuum
        | DomainId::Multiversum
        | DomainId::Planet10Oder12
        | DomainId::MetaKonkret
        | DomainId::SonstigePythonDecl => Ok(SpaltenAnfrage::Standard {
            domain,
            unter: parse_standard_unter(ober, unter)?,
        }),
    }
}

fn parse_u16_index(ober: &str, unter: &str) -> Result<u16, ParseError> {
    unter.trim().parse::<u16>().map_err(|_| ParseError::InvalidGebrochenRationalIndex {
        ober: ober.to_string(),
        unter: unter.to_string(),
    })
}

fn parse_standard_unter(ober: &str, unter: &str) -> Result<StandardUnterId, ParseError> {
    let token = StandardUnterToken::parse(unter).ok_or_else(|| ParseError::UnknownUnterkategorie {
        ober: ober.to_string(),
        unter: unter.to_string(),
    })?;

    Ok(match token {
        StandardUnterToken::Gewalt => StandardUnterId::Gewalt,
        StandardUnterToken::Politische => StandardUnterId::Politische,
        StandardUnterToken::Richtungen => StandardUnterId::Richtungen,
        StandardUnterToken::Formationen => StandardUnterId::Formationen,
        StandardUnterToken::Klasse => StandardUnterId::Klasse,
        StandardUnterToken::Hoelle => StandardUnterId::Hoelle,
        StandardUnterToken::Liebe => StandardUnterId::Liebe,
        StandardUnterToken::Geist => StandardUnterId::Geist,
        StandardUnterToken::Religion => StandardUnterId::SymboleReligion,
        StandardUnterToken::Primzahlkreuz => StandardUnterId::Primzahlkreuz,
    })
}

fn parse_eigenschaften_generisch_unter(
    ober: &str,
    unter: &str,
) -> Result<StandardUnterId, ParseError> {
    if let Some(req) = parse_eigenschaft_request(unter, EigenschaftsFamilie::Generisch) {
        return Ok(StandardUnterId::Eigenschaft(req));
    }
    parse_standard_unter(ober, unter)
}

fn parse_eigenschaften_n_unter(ober: &str, unter: &str) -> Result<StandardUnterId, ParseError> {
    if let Some(req) = parse_eigenschaft_request(unter, EigenschaftsFamilie::N) {
        return Ok(StandardUnterId::Eigenschaft(req));
    }
    parse_standard_unter(ober, unter)
}

fn parse_eigenschaften_1_pro_n_unter(
    ober: &str,
    unter: &str,
) -> Result<StandardUnterId, ParseError> {
    if let Some(req) = parse_eigenschaft_request(unter, EigenschaftsFamilie::EinsDurchN) {
        return Ok(StandardUnterId::Eigenschaft(req));
    }

    Err(ParseError::UnknownUnterkategorie {
        ober: ober.to_string(),
        unter: unter.to_string(),
    })
}

fn parse_kombi_unter(ober: &str, unter: &str) -> Result<KombiUnterId, ParseError> {
    let token = KombiUnterToken::parse(unter).ok_or_else(|| ParseError::UnknownUnterkategorie {
        ober: ober.to_string(),
        unter: unter.to_string(),
    })?;

    Ok(match token {
        KombiUnterToken::Tiere => KombiUnterId::Tiere,
        KombiUnterToken::Berufe => KombiUnterId::Berufe,
        KombiUnterToken::Religion => KombiUnterId::Religion,
        KombiUnterToken::Politik => KombiUnterId::Politik,
    })
}
