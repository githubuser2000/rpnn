use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainId {
    Menschliches,
    Religion,
    Galaxie,
    Universum,
    Grundstrukturen,
    Kontinuum,
    Multiversum,
    Planet10Oder12,

    EigenschaftenN,
    Eigenschaften1ProN,

    GebrochenRational(GebrochenRationalArt),
    Kombination(KombinationsArt),
    Generator(GeneratorArt),

    MetaKonkret,

    /// Übergangsanker für noch nicht sauber typisierte Python-Deklarationen.
    SonstigePythonDecl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GebrochenRationalArt {
    Galaxie,
    Universum,
    Gefuehle,
    Strukturgroesse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KombinationsArt {
    Galaxie,
    Universum,
    Gefuehle,
    Strukturgroesse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratorArt {
    Primzahlkreuz,
    Multiplikationen,
    Primvielfache,
    MetaKonkret,
}

impl fmt::Display for GebrochenRationalArt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GebrochenRationalArt::Galaxie => write!(f, "gebrochen-rational_Galaxie_n/m"),
            GebrochenRationalArt::Universum => write!(f, "gebrochen-rational_Universum_n/m"),
            GebrochenRationalArt::Gefuehle => write!(f, "gebrochen-rational_Gefuehle_n/m"),
            GebrochenRationalArt::Strukturgroesse => {
                write!(f, "gebrochen-rational_Strukturgroesse_n/m")
            }
        }
    }
}

impl fmt::Display for KombinationsArt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KombinationsArt::Galaxie => write!(f, "KombinationGalaxie"),
            KombinationsArt::Universum => write!(f, "KombinationUniversum"),
            KombinationsArt::Gefuehle => write!(f, "KombinationGefuehle"),
            KombinationsArt::Strukturgroesse => write!(f, "KombinationStrukturgroesse"),
        }
    }
}

impl fmt::Display for GeneratorArt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeneratorArt::Primzahlkreuz => write!(f, "Primzahlkreuz"),
            GeneratorArt::Multiplikationen => write!(f, "Multiplikationen"),
            GeneratorArt::Primvielfache => write!(f, "Primvielfache"),
            GeneratorArt::MetaKonkret => write!(f, "MetaKonkret"),
        }
    }
}
