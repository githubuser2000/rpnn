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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    Wuerdig,
    RegelVsAusnahme,
    FilterartWidrigkeit,
    Werte,
    GutartigkeitsEgoismus,
    ReflektierenErkenntnisErkennen,
    VertrauenWollen,
    AusrichtenEinrichten,
    ToleranzRespektAkzeptanzWillkommen,

    Primzahlkreuz,
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
