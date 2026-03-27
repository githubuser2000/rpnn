use std::fmt;

use crate::domain::errors::ParseSpaltenAnfrageError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StandardOberkategorie {
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
    EigenschaftenN,
    Eigenschaften1ProN,
    UniversumMetaKonkret,
    Sonstige(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenschlichesUnter {
    Liebe,
    Gleichheit,
    Hoelle,
    Klasse,
    Gewalt,
    Politische,
    Richtungen,
    Formationen,
    Motive,
    Sonstige(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UniversumUnter {
    Geist,
    Primzahlkreuz,
    Sonstige(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReligionUnter {
    Religion,
    Ethik,
    Sonstige(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StandardAnfrage {
    Menschliches(MenschlichesUnter),
    Universum(UniversumUnter),
    Religion(ReligionUnter),
    Sonstige {
        ober: StandardOberkategorie,
        unter: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpaltenAnfrage {
    Standard(StandardAnfrage),
    KombinationGalaxie { unter: String },
    KombinationUniversum { unter: String },
    GebrochenRationalGalaxie { unter: String },
    GebrochenRationalUniversum { unter: String },
    GebrochenRationalGefuehle { unter: String },
    GebrochenRationalStrukturgroesse { unter: String },
    Primvielfache { unter: String },
    Multiplikationen { unter: String },
    Unknown { ober: String, unter: String },
}

impl StandardOberkategorie {
    pub fn parse(input: &str) -> Self {
        match normalize_key(input).as_str() {
            "menschliches" => Self::Menschliches,
            "universum" => Self::Universum,
            "religion" | "religionen" => Self::Religion,
            "planet" | "planet10undoder12" | "planet10oder12" => Self::Planet,
            "galaxie" => Self::Galaxie,
            "multiversum" => Self::Multiversum,
            "grundstrukturen" => Self::Grundstrukturen,
            "bedeutung" => Self::Bedeutung,
            "procontra" => Self::ProContra,
            "wichtigsteszumverstehen" => Self::WichtigstesZumVerstehen,
            "eigenschaftenn" | "eigenschaftenn1" | "eigenschaftennn" | "konzept1" | "konzepte1" => {
                Self::EigenschaftenN
            }
            "eigenschaften1n" | "konzept2" | "konzepte2" => Self::Eigenschaften1ProN,
            "universummetakonkret" | "metakonkret" => Self::UniversumMetaKonkret,
            _ => Self::Sonstige(input.trim().to_string()),
        }
    }

    pub fn as_cli_str(&self) -> &str {
        match self {
            Self::Menschliches => "Menschliches",
            Self::Universum => "Universum",
            Self::Religion => "Religion",
            Self::Planet => "Planet",
            Self::Galaxie => "Galaxie",
            Self::Multiversum => "Multiversum",
            Self::Grundstrukturen => "Grundstrukturen",
            Self::Bedeutung => "Bedeutung",
            Self::ProContra => "Pro_Contra",
            Self::WichtigstesZumVerstehen => "Wichtigstes_zum_verstehen",
            Self::EigenschaftenN => "Eigenschaften_n",
            Self::Eigenschaften1ProN => "Eigenschaften_1/n",
            Self::UniversumMetaKonkret => "universummetakonkret",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

impl MenschlichesUnter {
    pub fn parse(input: &str) -> Self {
        match normalize_key(input).as_str() {
            "liebe" | "ethik" => Self::Liebe,
            "gleichheit" => Self::Gleichheit,
            "hoelle" | "hölle" => Self::Hoelle,
            "klasse" => Self::Klasse,
            "gewalt" => Self::Gewalt,
            "politische" => Self::Politische,
            "richtungen" => Self::Richtungen,
            "formationen" => Self::Formationen,
            "motive" => Self::Motive,
            _ => Self::Sonstige(input.trim().to_string()),
        }
    }

    pub fn as_cli_str(&self) -> &str {
        match self {
            Self::Liebe => "Liebe",
            Self::Gleichheit => "Gleichheit",
            Self::Hoelle => "Hölle",
            Self::Klasse => "Klasse",
            Self::Gewalt => "Gewalt",
            Self::Politische => "politische",
            Self::Richtungen => "Richtungen",
            Self::Formationen => "Formationen",
            Self::Motive => "Motive",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

impl UniversumUnter {
    pub fn parse(input: &str) -> Self {
        match normalize_key(input).as_str() {
            "geist" => Self::Geist,
            "primzahlkreuz" | "primzahlkreuzprocontra" => Self::Primzahlkreuz,
            _ => Self::Sonstige(input.trim().to_string()),
        }
    }

    pub fn as_cli_str(&self) -> &str {
        match self {
            Self::Geist => "Geist",
            Self::Primzahlkreuz => "Primzahlkreuz",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

impl ReligionUnter {
    pub fn parse(input: &str) -> Self {
        match normalize_key(input).as_str() {
            "religion" | "religionen" => Self::Religion,
            "ethik" => Self::Ethik,
            _ => Self::Sonstige(input.trim().to_string()),
        }
    }

    pub fn as_cli_str(&self) -> &str {
        match self {
            Self::Religion => "Religion",
            Self::Ethik => "Ethik",
            Self::Sonstige(s) => s.as_str(),
        }
    }
}

impl SpaltenAnfrage {
    pub fn parse(ober: &str, unter: &str) -> Result<Self, ParseSpaltenAnfrageError> {
        let unter = unter.trim();
        if unter.is_empty() {
            return Err(ParseSpaltenAnfrageError::EmptyUnterkategorie);
        }

        let ober_norm = normalize_key(ober);
        let unter = unter.to_string();

        let parsed = match ober_norm.as_str() {
            "kombinationgalaxie" => Self::KombinationGalaxie { unter },
            "kombinationuniversum" => Self::KombinationUniversum { unter },
            "gebrochenrationalgalaxienm" => Self::GebrochenRationalGalaxie { unter },
            "gebrochenrationaluniversumnm" => Self::GebrochenRationalUniversum { unter },
            "gebrochenrationalgefuehlenm" | "gebrochenrationalgefuhlenm" => {
                Self::GebrochenRationalGefuehle { unter }
            }
            "gebrochenrationalstrukturgroessenm" => Self::GebrochenRationalStrukturgroesse { unter },
            "primvielfache" => Self::Primvielfache { unter },
            "multiplikationen" => Self::Multiplikationen { unter },
            _ => {
                let standard = StandardOberkategorie::parse(ober);
                match standard {
                    StandardOberkategorie::Menschliches => {
                        Self::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::parse(&unter)))
                    }
                    StandardOberkategorie::Universum => {
                        Self::Standard(StandardAnfrage::Universum(UniversumUnter::parse(&unter)))
                    }
                    StandardOberkategorie::Religion => {
                        Self::Standard(StandardAnfrage::Religion(ReligionUnter::parse(&unter)))
                    }
                    StandardOberkategorie::Sonstige(_) => Self::Unknown {
                        ober: ober.to_string(),
                        unter,
                    },
                    known => Self::Standard(StandardAnfrage::Sonstige { ober: known, unter }),
                }
            }
        };

        Ok(parsed)
    }

    pub fn to_cli(&self) -> String {
        match self {
            Self::Standard(StandardAnfrage::Menschliches(unter)) => {
                format!("--spaltenname Menschliches {}", unter.as_cli_str())
            }
            Self::Standard(StandardAnfrage::Universum(unter)) => {
                format!("--spaltenname Universum {}", unter.as_cli_str())
            }
            Self::Standard(StandardAnfrage::Religion(unter)) => {
                format!("--spaltenname Religion {}", unter.as_cli_str())
            }
            Self::Standard(StandardAnfrage::Sonstige { ober, unter }) => {
                format!("--spaltenname {} {}", ober.as_cli_str(), unter)
            }
            Self::KombinationGalaxie { unter } => {
                format!("--spaltenname KombinationGalaxie {}", unter)
            }
            Self::KombinationUniversum { unter } => {
                format!("--spaltenname KombinationUniversum {}", unter)
            }
            Self::GebrochenRationalGalaxie { unter } => {
                format!("--spaltenname gebrochen-rational_Galaxie_n/m {}", unter)
            }
            Self::GebrochenRationalUniversum { unter } => {
                format!("--spaltenname gebrochen-rational_Universum_n/m {}", unter)
            }
            Self::GebrochenRationalGefuehle { unter } => {
                format!("--spaltenname gebrochen-rational_Gefuehle_n/m {}", unter)
            }
            Self::GebrochenRationalStrukturgroesse { unter } => {
                format!("--spaltenname gebrochen-rational_Strukturgroesse_n/m {}", unter)
            }
            Self::Primvielfache { unter } => {
                format!("--spaltenname primvielfache {}", unter)
            }
            Self::Multiplikationen { unter } => {
                format!("--spaltenname multiplikationen {}", unter)
            }
            Self::Unknown { ober, unter } => {
                format!("--spaltenname {} {}", ober, unter)
            }
        }
    }
}

impl fmt::Display for SpaltenAnfrage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_cli())
    }
}

fn normalize_key(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}


impl SpaltenAnfrage {
    pub fn ober_unter_cli_pair(&self) -> (String, String) {
        match self {
            Self::Standard(StandardAnfrage::Menschliches(unter)) => {
                ("Menschliches".to_string(), unter.as_cli_str().to_string())
            }
            Self::Standard(StandardAnfrage::Universum(unter)) => {
                ("Universum".to_string(), unter.as_cli_str().to_string())
            }
            Self::Standard(StandardAnfrage::Religion(unter)) => {
                ("Religion".to_string(), unter.as_cli_str().to_string())
            }
            Self::Standard(StandardAnfrage::Sonstige { ober, unter }) => {
                (ober.as_cli_str().to_string(), unter.clone())
            }
            Self::KombinationGalaxie { unter } => ("KombinationGalaxie".to_string(), unter.clone()),
            Self::KombinationUniversum { unter } => ("KombinationUniversum".to_string(), unter.clone()),
            Self::GebrochenRationalGalaxie { unter } => ("gebrochen-rational_Galaxie_n/m".to_string(), unter.clone()),
            Self::GebrochenRationalUniversum { unter } => ("gebrochen-rational_Universum_n/m".to_string(), unter.clone()),
            Self::GebrochenRationalGefuehle { unter } => ("gebrochen-rational_Gefuehle_n/m".to_string(), unter.clone()),
            Self::GebrochenRationalStrukturgroesse { unter } => ("gebrochen-rational_Strukturgroesse_n/m".to_string(), unter.clone()),
            Self::Primvielfache { unter } => ("primvielfache".to_string(), unter.clone()),
            Self::Multiplikationen { unter } => ("multiplikationen".to_string(), unter.clone()),
            Self::Unknown { ober, unter } => (ober.clone(), unter.clone()),
        }
    }

    pub fn ober_normalized(&self) -> String {
        let (ober, _) = self.ober_unter_cli_pair();
        normalize_key(&ober)
    }

    pub fn unter_normalized(&self) -> String {
        let (_, unter) = self.ober_unter_cli_pair();
        normalize_key(&unter)
    }

    pub fn generated_befehle_hint(&self) -> Vec<String> {
        let ober_n = self.ober_normalized();
        let unter_n = self.unter_normalized();
        let mut out = Vec::<String>::new();

        let is_bedeutung = matches!(ober_n.as_str(), "bedeutung" | "wichtigsteszumverstehen" | "wichtigsteverstehen" | "wichtigste");
        let is_procontra = matches!(ober_n.as_str(), "procontra" | "dagegendafuer");
        let is_universum = matches!(ober_n.as_str(), "universum" | "multiversum" | "grundstrukturen");
        let is_planet = matches!(ober_n.as_str(), "planet" | "planet10undoder12" | "planet10oder12");
        let is_menschliches = ober_n == "menschliches";
        let is_galaxie = matches!(ober_n.as_str(), "galaxie" | "alteschriften" | "kreis" | "galaxien" | "kreise");
        let is_prim_generated_group = matches!(self, Self::Primvielfache { .. } | Self::Multiplikationen { .. });

        let has_any = |s: &str, vals: &[&str]| vals.iter().any(|v| s == normalize_key(v));

        if (is_bedeutung || is_procontra || is_universum)
            && matches!(unter_n.as_str(), "primzahlkreuzprocontra" | "primzahlkreuz") {
            out.push("primzahlkreuzprocontra".to_string());
        }
        if (is_menschliches || ober_n == "grundstrukturen") && has_any(&unter_n, &["liebe", "ethik"]) {
            out.push("lovepolygon".to_string());
        }
        if (is_planet || is_menschliches || ober_n == "grundstrukturen")
            && has_any(&unter_n, &["gleichheit", "freiheit", "dominieren", "ordnung", "ordnen", "ordnenundfiltern", "filterung", "ungleichheit"]) {
            out.push("gleichheitfreiheit".to_string());
        }
        if is_universum && has_any(&unter_n, &["geist", "bewusstsein", "emotion", "emotionen", "gefuehl", "gefuehle", "gefühl", "gefühle", "energie", "materie", "topologie"]) {
            out.push("geistemotionenergiematerietopologie".to_string());
        }
        if is_bedeutung && has_any(&unter_n, &["gestirn", "mond", "sonne", "planet", "evolution", "intelligenz", "kreativ", "kreativitaet", "kreativität", "lernen", "erwerben"]) {
            out.push("primcreativitytype".to_string());
            out.push("mondexponzierenlogarithmustyp".to_string());
        }
        if (is_bedeutung || is_galaxie)
            && has_any(&unter_n, &["primzahlen", "vielfache", "vielfacher", "multis", "multiplikationen", "offenbarung", "offenbarungjohannes"]) {
            out.push("vervielfachezeile".to_string());
        }
        if has_any(&unter_n, &["modallogik", "modal", "modus", "modi", "sein", "zustaende", "zustände"]) || has_any(&ober_n, &["modallogik", "modal", "modus", "modi", "sein", "zustaende", "zustände"]) {
            out.push("modallogik".to_string());
        }
        if is_prim_generated_group {
            if has_any(&unter_n, &["motivgleichfoermig", "motivgleichförmig", "motivegleichfoermigepolygone", "motivegleichförmige polygone"]) { out.push("primmotivgleichf".to_string()); }
            if has_any(&unter_n, &["strukturgleichfoermig", "strukturgleichförmig", "strukturgleichfoermigepolygone", "strukturgleichförmige polygone"]) { out.push("primstrukgleichf".to_string()); }
            if has_any(&unter_n, &["motivstern", "motivesternpolygone", "motivesternpolygon"]) { out.push("primmotivstern".to_string()); }
            if has_any(&unter_n, &["strukturstern", "struktursternpolygone", "struktursternpolygon"]) { out.push("primstrukstern".to_string()); }
            if has_any(&unter_n, &["motivgebrstern", "motivsternpolygongebrochenrational", "motivsternpolygongebrochen-rational"]) { out.push("primmotivsterngebr".to_string()); }
            if has_any(&unter_n, &["strukgebrstern", "struktursternpolyongebrochenrational", "struktursternpolygongebrochen-rational"]) { out.push("primstruksterngebr".to_string()); }
            if has_any(&unter_n, &["motivgebrgleichf", "motivgleichfoermigepolygonegebrochenrational", "motivgleichförmigepolygonegebrochen-rational"]) { out.push("primmotivgleichfgebr".to_string()); }
            if has_any(&unter_n, &["strukgebrgleichf", "strukturgleichfoermigepolygonegebrochenrational", "strukturgleichförmigepolygonegebrochen-rational"]) { out.push("primstrukgleichfgebr".to_string()); }
        }

        out.sort();
        out.dedup();
        out
    }

    pub fn parameters_main_hint(&self) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
        let (ober, unter) = self.ober_unter_cli_pair();
        let ober_n = normalize_key(&ober);
        let unter_n = normalize_key(&unter);

        let bedeutung0 = if matches!(ober_n.as_str(), "bedeutung" | "wichtigsteszumverstehen" | "wichtigsteverstehen" | "wichtigste") {
            Some(ober_n.clone())
        } else { None };
        let procontra0 = if ober_n == "procontra" { Some(ober_n.clone()) } else { None };
        let grundstrukturen0 = if matches!(ober_n.as_str(), "universum" | "multiversum" | "grundstrukturen") {
            Some(ober_n.clone())
        } else { None };
        let unter0 = if unter_n.is_empty() { None } else { Some(unter) };
        (bedeutung0, procontra0, grundstrukturen0, unter0)
    }
}
