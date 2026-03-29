use crate::domain::ids::domain_id::{
    DomainId, GebrochenRationalArt, GeneratorArt, KombinationsArt,
};
use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::model::spalten_anfrage::{
    EigenschaftRequest, EigenschaftsFamilie, GeneratorParameter, KombiUnterId,
    SpaltenAnfrage, StandardUnterId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnknownOberkategorie(String),
    UnknownUnterkategorie { ober: String, unter: String },
    InvalidGebrochenRationalIndex { ober: String, unter: String },
}

fn normalize_cli_token(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss")
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
        .replace('„', "")
        .replace('“', "")
        .replace('"', "")
}

fn matches_alias(input: &str, aliases: &[&str]) -> bool {
    let n = normalize_cli_token(input);
    aliases.iter().any(|a| normalize_cli_token(a) == n)
}

fn parse_eigenschaft_request(unter: &str, familie: EigenschaftsFamilie) -> Option<EigenschaftRequest> {
    EigenschaftKeyId::from_alias(unter).map(|key| EigenschaftRequest { familie, key })
}

pub fn parse_oberkategorie(input: &str) -> Result<DomainId, ParseError> {
    if matches_alias(input, &["Menschliches"]) {
        return Ok(DomainId::Menschliches);
    }
    if matches_alias(input, &["Religion", "Religionen"]) {
        return Ok(DomainId::Religion);
    }
    if matches_alias(input, &["Galaxie", "Galaxien"]) {
        return Ok(DomainId::Galaxie);
    }
    if matches_alias(input, &["Universum"]) {
        return Ok(DomainId::Universum);
    }
    if matches_alias(input, &["Grundstrukturen"]) {
        return Ok(DomainId::Grundstrukturen);
    }
    if matches_alias(input, &["Kontinuum"]) {
        return Ok(DomainId::Kontinuum);
    }
    if matches_alias(input, &["Multiversum"]) {
        return Ok(DomainId::Multiversum);
    }
    if matches_alias(input, &["Planet_(10_und_oder_12)", "Planet"]) {
        return Ok(DomainId::Planet10Oder12);
    }

    if matches_alias(input, &["Eigenschaft", "Eigenschaften", "konzept", "konzepte"]) {
        return Ok(DomainId::Eigenschaften);
    }
    if matches_alias(input, &["Eigenschaften_n", "konzept1", "konzepte1"]) {
        return Ok(DomainId::EigenschaftenN);
    }
    if matches_alias(input, &["Eigenschaften_1/n", "konzept2", "konzepte2"]) {
        return Ok(DomainId::Eigenschaften1ProN);
    }

    if matches_alias(input, &["gebrochen-rational_Galaxie_n/m", "gebrochengalaxie"]) {
        return Ok(DomainId::GebrochenRational(GebrochenRationalArt::Galaxie));
    }
    if matches_alias(input, &["gebrochen-rational_Universum_n/m", "gebrochenuniversum"]) {
        return Ok(DomainId::GebrochenRational(GebrochenRationalArt::Universum));
    }
    if matches_alias(
        input,
        &["gebrochen-rational_Gefuehle_n/m", "gebrochenemotion", "gebrochengemotion"],
    ) {
        return Ok(DomainId::GebrochenRational(GebrochenRationalArt::Gefuehle));
    }
    if matches_alias(
        input,
        &["gebrochen-rational_Strukturgroesse_n/m", "gebrochengroesse"],
    ) {
        return Ok(DomainId::GebrochenRational(
            GebrochenRationalArt::Strukturgroesse,
        ));
    }

    if matches_alias(input, &["KombinationGalaxie"]) {
        return Ok(DomainId::Kombination(KombinationsArt::Galaxie));
    }
    if matches_alias(input, &["KombinationUniversum"]) {
        return Ok(DomainId::Kombination(KombinationsArt::Universum));
    }
    if matches_alias(input, &["KombinationGefuehle"]) {
        return Ok(DomainId::Kombination(KombinationsArt::Gefuehle));
    }
    if matches_alias(input, &["KombinationStrukturgroesse"]) {
        return Ok(DomainId::Kombination(KombinationsArt::Strukturgroesse));
    }

    if matches_alias(input, &["Primzahlkreuz"]) {
        return Ok(DomainId::Generator(GeneratorArt::Primzahlkreuz));
    }
    if matches_alias(input, &["Multiplikationen"]) {
        return Ok(DomainId::Generator(GeneratorArt::Multiplikationen));
    }
    if matches_alias(input, &["Primvielfache", "primvielfache"]) {
        return Ok(DomainId::Generator(GeneratorArt::Primvielfache));
    }
    if matches_alias(input, &["MetaKonkret", "Universum_Metakonkret"]) {
        return Ok(DomainId::MetaKonkret);
    }

    Err(ParseError::UnknownOberkategorie(input.to_string()))
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
    let u = normalize_cli_token(unter);

    let parsed = match u.as_str() {
        "gewalt" => StandardUnterId::Gewalt,
        "politische" => StandardUnterId::Politische,
        "richtungen" => StandardUnterId::Richtungen,
        "formationen" => StandardUnterId::Formationen,
        "klasse" => StandardUnterId::Klasse,
        "hölle" | "hoelle" => StandardUnterId::Hoelle,
        "liebe" => StandardUnterId::Liebe,
        "geist" => StandardUnterId::Geist,
        "religion" | "symbole religion" | "symbolereligion" => StandardUnterId::SymboleReligion,
        "primzahlkreuz" => StandardUnterId::Primzahlkreuz,
        _ => {
            return Ok(StandardUnterId::PythonSubcategory(unter.trim().to_string()));
        }
    };

    Ok(parsed)
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
    let u = normalize_cli_token(unter);

    let parsed = match u.as_str() {
        "tiere" => KombiUnterId::Tiere,
        "berufe" => KombiUnterId::Berufe,
        "religion" => KombiUnterId::Religion,
        "politik" => KombiUnterId::Politik,
        _ => {
            return Err(ParseError::UnknownUnterkategorie {
                ober: ober.to_string(),
                unter: unter.to_string(),
            });
        }
    };

    Ok(parsed)
}
