use crate::domain::eigenschaften::EigenschaftKeyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OberToken {
    Menschliches,
    Religion,
    Galaxie,
    Universum,
    Grundstrukturen,
    Kontinuum,
    Multiversum,
    Planet10Oder12,

    Eigenschaften,
    EigenschaftenN,
    Eigenschaften1ProN,

    GebrochenRationalGalaxie,
    GebrochenRationalUniversum,
    GebrochenRationalGefuehle,
    GebrochenRationalStrukturgroesse,

    KombinationGalaxie,
    KombinationUniversum,
    KombinationGefuehle,
    KombinationStrukturgroesse,

    Primzahlkreuz,
    Multiplikationen,
    Primvielfache,

    MetaKonkret,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StandardUnterToken {
    Gewalt,
    Politische,
    Richtungen,
    Formationen,
    Klasse,
    Hoelle,
    Liebe,
    Geist,
    SymboleReligion,
    Primzahlkreuz,
    Eigenschaft(EigenschaftKeyId),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KombiUnterToken {
    Tiere,
    Berufe,
    Religion,
    Politik,
    Unbekannt,
    Text(String),
}

fn fold(s: &str) -> String {
    s.trim().to_lowercase()
}

impl OberToken {
    pub fn parse(input: &str) -> Option<Self> {
        let s = fold(input);
        Some(match s.as_str() {
            "menschliches" => Self::Menschliches,
            "religion" | "religionen" => Self::Religion,
            "galaxie" | "galaxien" | "alteschriften" | "kreis" | "kreise" => Self::Galaxie,
            "universum" => Self::Universum,
            "grundstrukturen" => Self::Grundstrukturen,
            "kontinuum" => Self::Kontinuum,
            "multiversum" => Self::Multiversum,
            "planet" | "planet_(10_und_oder_12)" => Self::Planet10Oder12,

            "eigenschaften" => Self::Eigenschaften,
            "eigenschaften_n" => Self::EigenschaftenN,
            "eigenschaften_1/n" => Self::Eigenschaften1ProN,

            "gebrochen-rational_galaxie_n/m" => Self::GebrochenRationalGalaxie,
            "gebrochen-rational_universum_n/m" => Self::GebrochenRationalUniversum,
            "gebrochen-rational_gefuehle_n/m" => Self::GebrochenRationalGefuehle,
            "gebrochen-rational_strukturgroesse_n/m" => Self::GebrochenRationalStrukturgroesse,

            "kombinationgalaxie" => Self::KombinationGalaxie,
            "kombinationuniversum" => Self::KombinationUniversum,
            "kombinationgefuehle" => Self::KombinationGefuehle,
            "kombinationstrukturgroesse" => Self::KombinationStrukturgroesse,

            "primzahlkreuz" => Self::Primzahlkreuz,
            "multiplikationen" => Self::Multiplikationen,
            "primvielfache" => Self::Primvielfache,

            "metakonkret" => Self::MetaKonkret,
            _ => return None,
        })
    }
}

impl StandardUnterToken {
    pub fn parse(input: &str) -> Self {
        let s = input.trim();

        if let Some(key) = EigenschaftKeyId::from_alias(s) {
            return Self::Eigenschaft(key);
        }

        match fold(s).as_str() {
            "gewalt" => Self::Gewalt,
            "politische" => Self::Politische,
            "richtungen" => Self::Richtungen,
            "formationen" => Self::Formationen,
            "klasse" => Self::Klasse,
            "hölle" | "hoelle" => Self::Hoelle,
            "liebe" => Self::Liebe,
            "geist" => Self::Geist,
            "religion" => Self::SymboleReligion,
            "primzahlkreuz" => Self::Primzahlkreuz,
            _ => Self::Text(s.to_string()),
        }
    }
}

impl KombiUnterToken {
    pub fn parse(input: &str) -> Self {
        let s = input.trim();
        match fold(s).as_str() {
            "tiere" => Self::Tiere,
            "berufe" => Self::Berufe,
            "religion" => Self::Religion,
            "politik" => Self::Politik,
            "unbekannt" => Self::Unbekannt,
            _ => Self::Text(s.to_string()),
        }
    }
}
