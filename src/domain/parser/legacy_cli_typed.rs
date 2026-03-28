use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LegacyOberToken {
    Menschliches,
    Universum,
    Religion,
    Planet,
    Galaxie,
    Multiversum,
    Grundstrukturen,
    Bedeutung,
    ProContra,
    WichtigstesZumVerstehen,
    WichtigstesZumGedanklichEinordnen,
    Eigenschaften,
    EigenschaftenN,
    Eigenschaften1ProN,
    UniversumMetaKonkret,
    KombinationGalaxie,
    KombinationUniversum,
    GebrochenRationalGalaxie,
    GebrochenRationalUniversum,
    GebrochenRationalGefuehle,
    GebrochenRationalStrukturgroesse,
    Primvielfache,
    Multiplikationen,
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GeneratedCommandToken {
    PrimzahlkreuzProContra,
    LovePolygon,
    GleichheitFreiheit,
    GeistEmotionEnergieMaterieTopologie,
    PrimCreativityType,
    MondExponzierenLogarithmusTyp,
    VervielfacheZeile,
    PrimMotGleichf,
    PrimStrukGleichf,
    PrimMotivStern,
    PrimStrukturStern,
    PrimMotivSternGebr,
    PrimStrukturSternGebr,
    PrimMotGleichfGebr,
    PrimStrukGleichfGebr,
}

pub fn fold_cli_case(input: &str) -> String {
    input.trim().to_lowercase()
}

pub fn matches_any_alias(input: &str, aliases: &[&str]) -> bool {
    let wanted = fold_cli_case(input);
    aliases.iter().any(|alias| fold_cli_case(alias) == wanted)
}

impl LegacyOberToken {
    pub fn parse(input: &str) -> Self {
        if matches_any_alias(input, &["menschliches"]) {
            return Self::Menschliches;
        }
        if matches_any_alias(input, &["universum"]) {
            return Self::Universum;
        }
        if matches_any_alias(input, &["religion", "religionen"]) {
            return Self::Religion;
        }
        if matches_any_alias(input, &["planet", "planet_(10_und_oder_12)"]) {
            return Self::Planet;
        }
        if matches_any_alias(input, &["galaxie", "galaxien", "alteschriften", "kreis", "kreise"]) {
            return Self::Galaxie;
        }
        if matches_any_alias(input, &["multiversum"]) {
            return Self::Multiversum;
        }
        if matches_any_alias(input, &["grundstrukturen"]) {
            return Self::Grundstrukturen;
        }
        if matches_any_alias(input, &["bedeutung"]) {
            return Self::Bedeutung;
        }
        if matches_any_alias(input, &["pro_contra", "procontra"]) {
            return Self::ProContra;
        }
        if matches_any_alias(input, &["wichtigstes_zum_verstehen", "wichtigsteverstehen"]) {
            return Self::WichtigstesZumVerstehen;
        }
        if matches_any_alias(input, &["wichtigstes_zum_gedanklich_einordnen", "wichtigsteeinordnen"]) {
            return Self::WichtigstesZumGedanklichEinordnen;
        }
        if matches_any_alias(input, &["eigenschaft", "eigenschaften", "konzept", "konzepte"]) {
            return Self::Eigenschaften;
        }
        if matches_any_alias(input, &["eigenschaften_n", "konzept1", "konzepte1"]) {
            return Self::EigenschaftenN;
        }
        if matches_any_alias(input, &["eigenschaften_1/n", "konzept2", "konzepte2"]) {
            return Self::Eigenschaften1ProN;
        }
        if matches_any_alias(input, &["universummetakonkret", "universum_metakonkret", "metakonkret"]) {
            return Self::UniversumMetaKonkret;
        }
        if matches_any_alias(input, &["kombinationgalaxie"]) {
            return Self::KombinationGalaxie;
        }
        if matches_any_alias(input, &["kombinationuniversum"]) {
            return Self::KombinationUniversum;
        }
        if matches_any_alias(input, &["gebrochen-rational_galaxie_n/m", "gebrochengalaxie"]) {
            return Self::GebrochenRationalGalaxie;
        }
        if matches_any_alias(input, &["gebrochen-rational_universum_n/m", "gebrochenuniversum"]) {
            return Self::GebrochenRationalUniversum;
        }
        if matches_any_alias(input, &["gebrochen-rational_gefühle_n/m", "gebrochen-rational_gefuehle_n/m", "gebrochenemotion", "gebrochengemotion"]) {
            return Self::GebrochenRationalGefuehle;
        }
        if matches_any_alias(input, &["gebrochen-rational_strukturgroesse_n/m", "gebrochengroesse"]) {
            return Self::GebrochenRationalStrukturgroesse;
        }
        if matches_any_alias(input, &["primvielfache"]) {
            return Self::Primvielfache;
        }
        if matches_any_alias(input, &["multiplikationen"]) {
            return Self::Multiplikationen;
        }
        Self::Unknown(input.trim().to_string())
    }
}

impl GeneratedCommandToken {
    pub fn parse(input: &str) -> Option<Self> {
        if matches_any_alias(input, &["primzahlkreuzprocontra"]) {
            return Some(Self::PrimzahlkreuzProContra);
        }
        if matches_any_alias(input, &["lovepolygon"]) {
            return Some(Self::LovePolygon);
        }
        if matches_any_alias(input, &["gleichheitfreiheit"]) {
            return Some(Self::GleichheitFreiheit);
        }
        if matches_any_alias(input, &["geistemotionenergiematerietopologie"]) {
            return Some(Self::GeistEmotionEnergieMaterieTopologie);
        }
        if matches_any_alias(input, &["primcreativitytype"]) {
            return Some(Self::PrimCreativityType);
        }
        if matches_any_alias(input, &["mondexponzierenlogarithmustyp"]) {
            return Some(Self::MondExponzierenLogarithmusTyp);
        }
        if matches_any_alias(input, &["vervielfachezeile"]) {
            return Some(Self::VervielfacheZeile);
        }
        if matches_any_alias(input, &["primmotgleichf"]) {
            return Some(Self::PrimMotGleichf);
        }
        if matches_any_alias(input, &["primstrukgleichf"]) {
            return Some(Self::PrimStrukGleichf);
        }
        if matches_any_alias(input, &["primmotivstern"]) {
            return Some(Self::PrimMotivStern);
        }
        if matches_any_alias(input, &["primstrukturstern"]) {
            return Some(Self::PrimStrukturStern);
        }
        if matches_any_alias(input, &["primmotivsterngebr"]) {
            return Some(Self::PrimMotivSternGebr);
        }
        if matches_any_alias(input, &["primstruktursterngebr"]) {
            return Some(Self::PrimStrukturSternGebr);
        }
        if matches_any_alias(input, &["primmotgleichfgebr"]) {
            return Some(Self::PrimMotGleichfGebr);
        }
        if matches_any_alias(input, &["primstrukgleichfgebr"]) {
            return Some(Self::PrimStrukGleichfGebr);
        }
        None
    }
}

impl fmt::Display for LegacyOberToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Menschliches => f.write_str("Menschliches"),
            Self::Universum => f.write_str("Universum"),
            Self::Religion => f.write_str("Religion"),
            Self::Planet => f.write_str("Planet"),
            Self::Galaxie => f.write_str("Galaxie"),
            Self::Multiversum => f.write_str("Multiversum"),
            Self::Grundstrukturen => f.write_str("Grundstrukturen"),
            Self::Bedeutung => f.write_str("Bedeutung"),
            Self::ProContra => f.write_str("Pro_Contra"),
            Self::WichtigstesZumVerstehen => f.write_str("Wichtigstes_zum_verstehen"),
            Self::WichtigstesZumGedanklichEinordnen => f.write_str("Wichtigstes_zum_gedanklich_einordnen"),
            Self::Eigenschaften => f.write_str("Eigenschaften"),
            Self::EigenschaftenN => f.write_str("Eigenschaften_n"),
            Self::Eigenschaften1ProN => f.write_str("Eigenschaften_1/n"),
            Self::UniversumMetaKonkret => f.write_str("universummetakonkret"),
            Self::KombinationGalaxie => f.write_str("KombinationGalaxie"),
            Self::KombinationUniversum => f.write_str("KombinationUniversum"),
            Self::GebrochenRationalGalaxie => f.write_str("gebrochen-rational_Galaxie_n/m"),
            Self::GebrochenRationalUniversum => f.write_str("gebrochen-rational_Universum_n/m"),
            Self::GebrochenRationalGefuehle => f.write_str("gebrochen-rational_Gefuehle_n/m"),
            Self::GebrochenRationalStrukturgroesse => f.write_str("gebrochen-rational_Strukturgroesse_n/m"),
            Self::Primvielfache => f.write_str("Primvielfache"),
            Self::Multiplikationen => f.write_str("Multiplikationen"),
            Self::Unknown(value) => f.write_str(value),
        }
    }
}
