use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::{
    DomainId, GebrochenRationalArt, GeneratorArt, KombinationsArt,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpaltenAnfrage {
    Standard {
        domain: DomainId,
        unter: StandardUnterId,
    },
    GebrochenRational {
        art: GebrochenRationalArt,
        index: u16,
    },
    Kombination {
        art: KombinationsArt,
        unter: KombiUnterId,
    },
    Generator {
        art: GeneratorArt,
        parameter: GeneratorParameter,
    },
    DirektSpalten {
        ids: Vec<u16>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StandardUnterId {
    Gewalt,
    Politische,
    Richtungen,
    Formationen,
    Klasse,
    Hoelle,
    Liebe,
    Geist,
    SymboleReligion,

    Eigenschaft(EigenschaftRequest),

    Primzahlkreuz,
    PythonSubcategory(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EigenschaftsFamilie {
    Generisch,
    N,
    EinsDurchN,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EigenschaftRequest {
    pub familie: EigenschaftsFamilie,
    pub key: EigenschaftKeyId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KombiUnterId {
    Tiere,
    Berufe,
    Religion,
    Politik,
    Unbekannt,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeneratorParameter {
    Keine,
    Text(String),
    Zahl(u16),
    TextListe(Vec<String>),
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Col1(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Col0(pub usize);

impl Col1 {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn try_from_usize(value: usize) -> Result<Self, &'static str> {
        if value == 0 || value > u16::MAX as usize {
            return Err("column index must be >= 1");
        }
        Ok(Self(value as u16))
    }

    pub fn get(self) -> u16 {
        self.0
    }

    pub fn to_zero_based(self) -> Col0 {
        debug_assert!(self.0 >= 1);
        Col0((self.0 - 1) as usize)
    }
}

impl Col0 {
    pub fn get(self) -> usize {
        self.0
    }

    pub fn to_one_based(self) -> Result<Col1, &'static str> {
        let value = self.0.checked_add(1).ok_or("column index overflow")?;
        Col1::try_from_usize(value)
    }
}

impl TryFrom<usize> for Col1 {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from_usize(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedTarget {
    SourceColumns(Vec<Col1>),
    GeneratedEigenschaft {
        familie: EigenschaftsFamilie,
        key: EigenschaftKeyId,
        required_sources: Vec<Col1>,
    },
    Generator {
        befehle: Vec<String>,
        required_sources: Vec<Col1>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnTarget {
    DirectColumn(u16),
    DirectColumns(Vec<u16>),
    Pair(u16, u16),
    Generator(GeneratorSpec),
    Combination(CombinationSpec),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorSpec {
    pub art: GeneratorArt,
    pub parameter: GeneratorParameter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombinationSpec {
    pub art: KombinationsArt,
    pub unter: KombiUnterId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalColumnSpec {
    pub request: SpaltenAnfrage,
    pub target: ColumnTarget,
    pub header_display: String,
    pub aliases_for_report: Vec<String>,
}


impl EigenschaftsFamilie {
    pub fn oberkategorie_cli_name(&self) -> &'static str {
        match self {
            Self::Generisch => "Eigenschaften",
            Self::N => "Eigenschaften_n",
            Self::EinsDurchN => "Eigenschaften_1/n",
        }
    }
}

impl StandardUnterId {
    pub fn cli_unterkategorie_name(&self) -> String {
        match self {
            Self::Gewalt => "Gewalt".to_string(),
            Self::Politische => "politische".to_string(),
            Self::Richtungen => "Richtungen".to_string(),
            Self::Formationen => "Formationen".to_string(),
            Self::Klasse => "Klasse".to_string(),
            Self::Hoelle => "Hölle".to_string(),
            Self::Liebe => "Liebe".to_string(),
            Self::Geist => "Geist".to_string(),
            Self::SymboleReligion => "Religion".to_string(),
            Self::Eigenschaft(req) => req.key.canonical_name().to_string(),
            Self::Primzahlkreuz => "Primzahlkreuz".to_string(),
            Self::PythonSubcategory(name) => name.clone(),
        }
    }
}

impl SpaltenAnfrage {
    pub fn to_cli_pair(&self) -> Option<(String, String)> {
        match self {
            Self::Standard { domain, unter } => {
                let ober = match unter {
                    StandardUnterId::Eigenschaft(req) => req.familie.oberkategorie_cli_name().to_string(),
                    _ => match domain {
                        DomainId::Menschliches => "Menschliches".to_string(),
                        DomainId::Religion => "Religion".to_string(),
                        DomainId::Galaxie => "Galaxie".to_string(),
                        DomainId::Universum => "Universum".to_string(),
                        DomainId::Grundstrukturen => "Grundstrukturen".to_string(),
                        DomainId::Kontinuum => "Kontinuum".to_string(),
                        DomainId::Multiversum => "Multiversum".to_string(),
                        DomainId::Planet10Oder12 => "Planet".to_string(),
                        DomainId::Eigenschaften => "Eigenschaften".to_string(),
                        DomainId::EigenschaftenN => "Eigenschaften_n".to_string(),
                        DomainId::Eigenschaften1ProN => "Eigenschaften_1/n".to_string(),
                        DomainId::MetaKonkret => "MetaKonkret".to_string(),
                        DomainId::GebrochenRational(_) | DomainId::Kombination(_) | DomainId::Generator(_) | DomainId::SonstigePythonDecl => return None,
                    },
                };
                Some((ober, unter.cli_unterkategorie_name()))
            }
            Self::GebrochenRational { art, index } => Some((art.to_string(), index.to_string())),
            Self::Kombination { art, unter } => {
                let unter = match unter {
                    KombiUnterId::Tiere => "tiere",
                    KombiUnterId::Berufe => "berufe",
                    KombiUnterId::Religion => "religion",
                    KombiUnterId::Politik => "politik",
                    KombiUnterId::Unbekannt => return None,
                };
                Some((art.to_string(), unter.to_string()))
            }
            Self::Generator { art, parameter } => {
                let unter = match parameter {
                    GeneratorParameter::Keine => String::new(),
                    GeneratorParameter::Text(s) => s.clone(),
                    GeneratorParameter::Zahl(n) => n.to_string(),
                    GeneratorParameter::TextListe(items) => items.join(","),
                };
                Some((art.to_string(), unter))
            }
            Self::DirektSpalten { ids } => Some((
                "DirektSpalten".to_string(),
                ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(","),
            )),
        }
    }
}
