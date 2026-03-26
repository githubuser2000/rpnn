use std::collections::BTreeSet;
use std::fmt;

use crate::cli::TextBereich;
use crate::domain::categories::{KategorieMap, OberkategorieKey, StandardOberkategorie};
use crate::domain::exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};
use crate::reta_ausgabe::OutputSyntax;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MenschlichesUnter {
    Liebe,
    Gleichheit,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UniversumUnter {
    Geist,
    Primzahlkreuz,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BedeutungUnter {
    Primzahlkreuz,
    Gestirn,
    Primzahlen,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProContraUnter {
    Primzahlkreuz,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GrundstrukturenUnter {
    Liebe,
    Gleichheit,
    Geist,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PlanetUnter {
    Gleichheit,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MultiversumUnter {
    Geist,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WichtigstesUnter {
    Gestirn,
    Primzahlen,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GalaxieUnter {
    Primzahlen,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GeneratorUnter {
    MotivGleichfoermig,
    StrukturGleichfoermig,
    MotivStern,
    StrukturStern,
    MotivGebrStern,
    StrukGebrStern,
    MotivGebrGleichf,
    StrukGebrGleichf,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KombiGalaxieUnter {
    Tiere,
    Berufe,
    Kreativitaet,
    Liebe,
    Maenner,
    Persoenlichkeit,
    Religion,
    Motive,
    Emotionen,
    Personen,
    Wirtschaftssysteme,
    Eigentum,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KombiUniversumUnter {
    Tiere,
    Berufe,
    Transzendentalien,
    Primzahlkreuz,
    Persoenlichkeit,
    Religion,
    Motive,
    Ontologie,
    Personen,
    Mechanismen,
    Gegentranszendentalien,
    Maschinen,
    Geist,
    Bewusstsein,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeUnter {
    UniversumMetaKonkret(String),
    EigenschaftenN(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpaltenAnfrage {
    Menschliches(MenschlichesUnter),
    Universum(UniversumUnter),
    Bedeutung(BedeutungUnter),
    ProContra(ProContraUnter),
    Grundstrukturen(GrundstrukturenUnter),
    Planet(PlanetUnter),
    Multiversum(MultiversumUnter),
    WichtigstesZumVerstehen(WichtigstesUnter),
    Galaxie(GalaxieUnter),
    KombinationGalaxie(KombiGalaxieUnter),
    KombinationUniversum(KombiUniversumUnter),
    GebrochenRationalGalaxie { n: String },
    GebrochenRationalUniversum { n: String },
    GebrochenRationalGefuehle { n: String },
    GebrochenRationalStrukturgroesse { n: String },
    Primvielfache(GeneratorUnter),
    Multiplikationen(GeneratorUnter),
    Bridge(BridgeUnter),
    Unknown { ober: String, unter: String },
}

macro_rules! impl_display_via_debug_name {
    ($t:ty, { $($variant:ident => $text:expr),* $(,)? }) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Self::$variant => f.write_str($text),)*
                    Self::Unknown(s) => f.write_str(s),
                }
            }
        }
    };
}

impl_display_via_debug_name!(MenschlichesUnter, {
    Liebe => "Liebe",
    Gleichheit => "Gleichheit"
});
impl_display_via_debug_name!(UniversumUnter, {
    Geist => "Geist",
    Primzahlkreuz => "Primzahlkreuz"
});
impl_display_via_debug_name!(BedeutungUnter, {
    Primzahlkreuz => "Primzahlkreuz",
    Gestirn => "Gestirn",
    Primzahlen => "Primzahlen"
});
impl_display_via_debug_name!(ProContraUnter, {
    Primzahlkreuz => "Primzahlkreuz"
});
impl_display_via_debug_name!(GrundstrukturenUnter, {
    Liebe => "Liebe",
    Gleichheit => "Gleichheit",
    Geist => "Geist"
});
impl_display_via_debug_name!(PlanetUnter, {
    Gleichheit => "Gleichheit"
});
impl_display_via_debug_name!(MultiversumUnter, {
    Geist => "Geist"
});
impl_display_via_debug_name!(WichtigstesUnter, {
    Gestirn => "Gestirn",
    Primzahlen => "Primzahlen"
});
impl_display_via_debug_name!(GalaxieUnter, {
    Primzahlen => "Primzahlen"
});
impl_display_via_debug_name!(GeneratorUnter, {
    MotivGleichfoermig => "motivgleichfoermig",
    StrukturGleichfoermig => "strukturgleichfoermig",
    MotivStern => "motivstern",
    StrukturStern => "strukturstern",
    MotivGebrStern => "motivgebrstern",
    StrukGebrStern => "strukgebrstern",
    MotivGebrGleichf => "motivgebrgleichf",
    StrukGebrGleichf => "strukgebrgleichf"
});
impl_display_via_debug_name!(KombiGalaxieUnter, {
    Tiere => "tiere",
    Berufe => "berufe",
    Kreativitaet => "kreativität",
    Liebe => "liebe",
    Maenner => "männer",
    Persoenlichkeit => "persönlichkeit",
    Religion => "religion",
    Motive => "motive",
    Emotionen => "emotionen",
    Personen => "personen",
    Wirtschaftssysteme => "wirtschaftssysteme",
    Eigentum => "eigentum"
});
impl_display_via_debug_name!(KombiUniversumUnter, {
    Tiere => "tiere",
    Berufe => "berufe",
    Transzendentalien => "transzendentalien",
    Primzahlkreuz => "primzahlkreuz",
    Persoenlichkeit => "persönlichkeit",
    Religion => "religion",
    Motive => "motive",
    Ontologie => "ontologie",
    Personen => "personen",
    Mechanismen => "mechanismen",
    Gegentranszendentalien => "gegentranszendentalien",
    Maschinen => "maschinen",
    Geist => "geist",
    Bewusstsein => "bewusstsein"
});

impl SpaltenAnfrage {
    pub fn from_strings(ober: impl Into<String>, unter: impl Into<String>) -> Self {
        let ober = ober.into();
        let unter = unter.into();
        let ober_n = normalize_key(&ober);
        let unter_n = normalize_key(&unter);

        match ober_n.as_str() {
            "menschliches" => Self::Menschliches(match unter_n.as_str() {
                "liebe" => MenschlichesUnter::Liebe,
                "gleichheit" => MenschlichesUnter::Gleichheit,
                _ => MenschlichesUnter::Unknown(unter),
            }),
            "universum" => Self::Universum(match unter_n.as_str() {
                "geist" => UniversumUnter::Geist,
                "primzahlkreuz" => UniversumUnter::Primzahlkreuz,
                _ => UniversumUnter::Unknown(unter),
            }),
            "bedeutung" => Self::Bedeutung(match unter_n.as_str() {
                "primzahlkreuz" => BedeutungUnter::Primzahlkreuz,
                "gestirn" => BedeutungUnter::Gestirn,
                "primzahlen" => BedeutungUnter::Primzahlen,
                _ => BedeutungUnter::Unknown(unter),
            }),
            "procontra" => Self::ProContra(match unter_n.as_str() {
                "primzahlkreuz" => ProContraUnter::Primzahlkreuz,
                _ => ProContraUnter::Unknown(unter),
            }),
            "grundstrukturen" => Self::Grundstrukturen(match unter_n.as_str() {
                "liebe" => GrundstrukturenUnter::Liebe,
                "gleichheit" => GrundstrukturenUnter::Gleichheit,
                "geist" => GrundstrukturenUnter::Geist,
                _ => GrundstrukturenUnter::Unknown(unter),
            }),
            "planet" => Self::Planet(match unter_n.as_str() {
                "gleichheit" => PlanetUnter::Gleichheit,
                _ => PlanetUnter::Unknown(unter),
            }),
            "multiversum" => Self::Multiversum(match unter_n.as_str() {
                "geist" => MultiversumUnter::Geist,
                _ => MultiversumUnter::Unknown(unter),
            }),
            "wichtigsteszumverstehen" => {
                Self::WichtigstesZumVerstehen(match unter_n.as_str() {
                    "gestirn" => WichtigstesUnter::Gestirn,
                    "primzahlen" => WichtigstesUnter::Primzahlen,
                    _ => WichtigstesUnter::Unknown(unter),
                })
            }
            "galaxie" => Self::Galaxie(match unter_n.as_str() {
                "primzahlen" => GalaxieUnter::Primzahlen,
                _ => GalaxieUnter::Unknown(unter),
            }),
            "kombinationgalaxie" => Self::KombinationGalaxie(match unter_n.as_str() {
                "tiere" => KombiGalaxieUnter::Tiere,
                "berufe" => KombiGalaxieUnter::Berufe,
                "kreativität" | "kreativitaet" => KombiGalaxieUnter::Kreativitaet,
                "liebe" => KombiGalaxieUnter::Liebe,
                "männer" | "maenner" => KombiGalaxieUnter::Maenner,
                "persönlichkeit" | "persoenlichkeit" => KombiGalaxieUnter::Persoenlichkeit,
                "religion" => KombiGalaxieUnter::Religion,
                "motive" => KombiGalaxieUnter::Motive,
                "emotionen" => KombiGalaxieUnter::Emotionen,
                "personen" => KombiGalaxieUnter::Personen,
                "wirtschaftssysteme" => KombiGalaxieUnter::Wirtschaftssysteme,
                "eigentum" => KombiGalaxieUnter::Eigentum,
                _ => KombiGalaxieUnter::Unknown(unter),
            }),
            "kombinationuniversum" => Self::KombinationUniversum(match unter_n.as_str() {
                "tiere" => KombiUniversumUnter::Tiere,
                "berufe" => KombiUniversumUnter::Berufe,
                "transzendentalien" => KombiUniversumUnter::Transzendentalien,
                "primzahlkreuz" => KombiUniversumUnter::Primzahlkreuz,
                "persönlichkeit" | "persoenlichkeit" => KombiUniversumUnter::Persoenlichkeit,
                "religion" => KombiUniversumUnter::Religion,
                "motive" => KombiUniversumUnter::Motive,
                "ontologie" => KombiUniversumUnter::Ontologie,
                "personen" => KombiUniversumUnter::Personen,
                "mechanismen" => KombiUniversumUnter::Mechanismen,
                "gegentranszendentalien" => KombiUniversumUnter::Gegentranszendentalien,
                "maschinen" => KombiUniversumUnter::Maschinen,
                "geist" => KombiUniversumUnter::Geist,
                "bewusstsein" => KombiUniversumUnter::Bewusstsein,
                _ => KombiUniversumUnter::Unknown(unter),
            }),
            "gebrochenrationalgalaxienm" => Self::GebrochenRationalGalaxie { n: unter },
            "gebrochenrationaluniversumnm" => Self::GebrochenRationalUniversum { n: unter },
            "gebrochenrationalgefuhlenm" => Self::GebrochenRationalGefuehle { n: unter },
            "gebrochenrationalstrukturgrossenm" | "gebrochenrationalstrukturgroessenm" => {
                Self::GebrochenRationalStrukturgroesse { n: unter }
            }
            "primvielfache" => Self::Primvielfache(parse_generator_unter(unter_n.as_str(), unter)),
            "multiplikationen" => Self::Multiplikationen(parse_generator_unter(unter_n.as_str(), unter)),
            "universummetakonkret" => Self::Bridge(BridgeUnter::UniversumMetaKonkret(unter)),
            "eigenschaftenn" => Self::Bridge(BridgeUnter::EigenschaftenN(unter)),
            _ => Self::Unknown { ober, unter },
        }
    }

    pub fn to_cli(&self) -> String {
        match self {
            Self::Menschliches(u) => format!("--spaltenname Menschliches {}", u),
            Self::Universum(u) => format!("--spaltenname Universum {}", u),
            Self::Bedeutung(u) => format!("--spaltenname Bedeutung {}", u),
            Self::ProContra(u) => format!("--spaltenname Pro_Contra {}", u),
            Self::Grundstrukturen(u) => format!("--spaltenname Grundstrukturen {}", u),
            Self::Planet(u) => format!("--spaltenname Planet {}", u),
            Self::Multiversum(u) => format!("--spaltenname Multiversum {}", u),
            Self::WichtigstesZumVerstehen(u) => {
                format!("--spaltenname Wichtigstes_zum_verstehen {}", u)
            }
            Self::Galaxie(u) => format!("--spaltenname Galaxie {}", u),
            Self::KombinationGalaxie(u) => format!("--spaltenname KombinationGalaxie {}", u),
            Self::KombinationUniversum(u) => {
                format!("--spaltenname KombinationUniversum {}", u)
            }
            Self::GebrochenRationalGalaxie { n } => {
                format!("--spaltenname gebrochen-rational_Galaxie_n/m {}", n)
            }
            Self::GebrochenRationalUniversum { n } => {
                format!("--spaltenname gebrochen-rational_Universum_n/m {}", n)
            }
            Self::GebrochenRationalGefuehle { n } => {
                format!("--spaltenname gebrochen-rational_Gefühle_n/m {}", n)
            }
            Self::GebrochenRationalStrukturgroesse { n } => {
                format!("--spaltenname gebrochen-rational_Strukturgroesse_n/m {}", n)
            }
            Self::Primvielfache(u) => format!("--spaltenname primvielfache {}", u),
            Self::Multiplikationen(u) => format!("--spaltenname multiplikationen {}", u),
            Self::Bridge(BridgeUnter::UniversumMetaKonkret(u)) => {
                format!("--spaltenname universummetakonkret {}", u)
            }
            Self::Bridge(BridgeUnter::EigenschaftenN(u)) => {
                format!("--spaltenname Eigenschaften_n {}", u)
            }
            Self::Unknown { ober, unter } => format!("--spaltenname {} {}", ober, unter),
        }
    }
}

fn parse_generator_unter(norm: &str, original: String) -> GeneratorUnter {
    match norm {
        "motivgleichfoermig" => GeneratorUnter::MotivGleichfoermig,
        "strukturgleichfoermig" => GeneratorUnter::StrukturGleichfoermig,
        "motivstern" => GeneratorUnter::MotivStern,
        "strukturstern" => GeneratorUnter::StrukturStern,
        "motivgebrstern" => GeneratorUnter::MotivGebrStern,
        "strukgebrstern" => GeneratorUnter::StrukGebrStern,
        "motivgebrgleichf" => GeneratorUnter::MotivGebrGleichf,
        "strukgebrgleichf" => GeneratorUnter::StrukGebrGleichf,
        _ => GeneratorUnter::Unknown(original),
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

#[derive(Debug, Clone, Default)]
pub struct ReverseRequestReport {
    pub exakte_auswahl: BTreeSet<SpaltenAnfrage>,
    pub weitere_passende_auswahl: BTreeSet<SpaltenAnfrage>,
}

impl ReverseRequestReport {
    pub fn is_empty(&self) -> bool {
        self.exakte_auswahl.is_empty() && self.weitere_passende_auswahl.is_empty()
    }

    pub fn insert_exact(&mut self, anfrage: SpaltenAnfrage) {
        self.weitere_passende_auswahl.remove(&anfrage);
        self.exakte_auswahl.insert(anfrage);
    }

    pub fn insert_partial(&mut self, anfrage: SpaltenAnfrage) {
        if !self.exakte_auswahl.contains(&anfrage) {
            self.weitere_passende_auswahl.insert(anfrage);
        }
    }
}

fn collect_visible_columns(bereich: &TextBereich) -> BTreeSet<u32> {
    let mut visible = BTreeSet::<u32>::new();

    for &(a, b) in &bereich.spalten_bereiche {
        if a == 0 || b == 0 || a > b {
            continue;
        }
        for n in a..=b {
            visible.insert(n as u32);
        }
    }

    for &n in &bereich.spaltenreihenfolgeundnurdiese {
        if n > 0 {
            visible.insert(n as u32);
        }
    }

    for &n in &bereich.exact_visible_columns {
        if n > 0 {
            visible.insert(n as u32);
        }
    }

    visible
}

fn collect_exact_and_partial_direct_pairs(
    kategorie_map: &KategorieMap,
    visible_columns: &BTreeSet<u32>,
    report: &mut ReverseRequestReport,
) {
    for haupt in &kategorie_map.hauptkategorien {
        for unter in &haupt.unterkategorien {
            let pair = match &haupt.key {
                OberkategorieKey::Standard(StandardOberkategorie::Menschliches)
                | OberkategorieKey::Standard(StandardOberkategorie::Universum)
                | OberkategorieKey::Standard(StandardOberkategorie::Bedeutung)
                | OberkategorieKey::Standard(StandardOberkategorie::ProContra)
                | OberkategorieKey::Standard(StandardOberkategorie::Grundstrukturen)
                | OberkategorieKey::Standard(StandardOberkategorie::Planet)
                | OberkategorieKey::Standard(StandardOberkategorie::Multiversum)
                | OberkategorieKey::Standard(StandardOberkategorie::WichtigstesZumVerstehen)
                | OberkategorieKey::Standard(StandardOberkategorie::Galaxie)
                | OberkategorieKey::Kombination(_)
                | OberkategorieKey::GebrochenRational(_)
                | OberkategorieKey::Generator(_)
                | OberkategorieKey::Standard(StandardOberkategorie::Religion)
                | OberkategorieKey::Standard(StandardOberkategorie::EigenschaftenN)
                | OberkategorieKey::Standard(StandardOberkategorie::UniversumMetaKonkret)
                | OberkategorieKey::Sonstige(_) => {
                    SpaltenAnfrage::from_strings(haupt.key.to_string(), unter.name.to_string())
                }
            };
            let cols: BTreeSet<u32> = unter.spaltennummern.iter().copied().collect();

            if cols.is_empty() {
                continue;
            }

            if cols == *visible_columns {
                report.insert_exact(pair);
            } else if cols.iter().any(|c| visible_columns.contains(c)) {
                report.insert_partial(pair);
            }
        }
    }
}

fn collect_fraction_pairs(bereich: &TextBereich, report: &mut ReverseRequestReport) {
    for n in &bereich.pypy_compat.gebrochengalaxie {
        report.insert_partial(SpaltenAnfrage::GebrochenRationalGalaxie { n: n.to_string() });
    }
    for n in &bereich.pypy_compat.gebrochenuniversum {
        report.insert_partial(SpaltenAnfrage::GebrochenRationalUniversum { n: n.to_string() });
    }
    for n in &bereich.pypy_compat.gebrochenemotion {
        report.insert_partial(SpaltenAnfrage::GebrochenRationalGefuehle { n: n.to_string() });
    }
    for n in &bereich.pypy_compat.gebrochengroesse {
        report.insert_partial(SpaltenAnfrage::GebrochenRationalStrukturgroesse { n: n.to_string() });
    }
}

fn collect_kombi_pairs(bereich: &TextBereich, report: &mut ReverseRequestReport) {
    fn galaxie_name(idx: usize) -> Option<KombiGalaxieUnter> {
        match idx {
            1 => Some(KombiGalaxieUnter::Tiere),
            2 => Some(KombiGalaxieUnter::Berufe),
            3 => Some(KombiGalaxieUnter::Kreativitaet),
            4 => Some(KombiGalaxieUnter::Liebe),
            7 => Some(KombiGalaxieUnter::Maenner),
            8 => Some(KombiGalaxieUnter::Persoenlichkeit),
            9 => Some(KombiGalaxieUnter::Religion),
            10 => Some(KombiGalaxieUnter::Motive),
            12 => Some(KombiGalaxieUnter::Emotionen),
            13 => Some(KombiGalaxieUnter::Personen),
            16 => Some(KombiGalaxieUnter::Wirtschaftssysteme),
            17 => Some(KombiGalaxieUnter::Eigentum),
            _ => None,
        }
    }

    fn universum_name(idx: usize) -> Option<KombiUniversumUnter> {
        match idx {
            1 => Some(KombiUniversumUnter::Tiere),
            2 => Some(KombiUniversumUnter::Berufe),
            5 => Some(KombiUniversumUnter::Transzendentalien),
            6 => Some(KombiUniversumUnter::Primzahlkreuz),
            8 => Some(KombiUniversumUnter::Persoenlichkeit),
            9 => Some(KombiUniversumUnter::Religion),
            10 => Some(KombiUniversumUnter::Motive),
            11 => Some(KombiUniversumUnter::Ontologie),
            13 => Some(KombiUniversumUnter::Personen),
            14 => Some(KombiUniversumUnter::Mechanismen),
            15 => Some(KombiUniversumUnter::Gegentranszendentalien),
            17 => Some(KombiUniversumUnter::Maschinen),
            18 => Some(KombiUniversumUnter::Geist),
            19 => Some(KombiUniversumUnter::Bewusstsein),
            _ => None,
        }
    }

    for idx in &bereich.pypy_compat.kombi_galaxie {
        if let Some(name) = galaxie_name(*idx) {
            report.insert_partial(SpaltenAnfrage::KombinationGalaxie(name));
        }
    }

    for idx in &bereich.pypy_compat.kombi_universum {
        if let Some(name) = universum_name(*idx) {
            report.insert_partial(SpaltenAnfrage::KombinationUniversum(name));
        }
    }
}

fn collect_generated_pairs(generated_befehle: &BTreeSet<String>, report: &mut ReverseRequestReport) {
    let has =
        |needle: &str| generated_befehle.iter().any(|g| normalize_key(g) == normalize_key(needle));

    if has("primzahlkreuzprocontra") {
        report.insert_partial(SpaltenAnfrage::Universum(UniversumUnter::Primzahlkreuz));
        report.insert_partial(SpaltenAnfrage::Bedeutung(BedeutungUnter::Primzahlkreuz));
        report.insert_partial(SpaltenAnfrage::ProContra(ProContraUnter::Primzahlkreuz));
    }

    if has("lovepolygon") {
        report.insert_partial(SpaltenAnfrage::Menschliches(MenschlichesUnter::Liebe));
        report.insert_partial(SpaltenAnfrage::Grundstrukturen(GrundstrukturenUnter::Liebe));
    }

    if has("gleichheitfreiheit") {
        report.insert_partial(SpaltenAnfrage::Planet(PlanetUnter::Gleichheit));
        report.insert_partial(SpaltenAnfrage::Menschliches(MenschlichesUnter::Gleichheit));
        report.insert_partial(SpaltenAnfrage::Grundstrukturen(GrundstrukturenUnter::Gleichheit));
    }

    if has("geistemotionenergiematerietopologie") {
        report.insert_partial(SpaltenAnfrage::Universum(UniversumUnter::Geist));
        report.insert_partial(SpaltenAnfrage::Multiversum(MultiversumUnter::Geist));
        report.insert_partial(SpaltenAnfrage::Grundstrukturen(GrundstrukturenUnter::Geist));
    }

    if has("primcreativitytype") || has("mondexponzierenlogarithmustyp") {
        report.insert_partial(SpaltenAnfrage::WichtigstesZumVerstehen(WichtigstesUnter::Gestirn));
        report.insert_partial(SpaltenAnfrage::Bedeutung(BedeutungUnter::Gestirn));
    }

    if has("vervielfachezeile") {
        report.insert_partial(SpaltenAnfrage::WichtigstesZumVerstehen(WichtigstesUnter::Primzahlen));
        report.insert_partial(SpaltenAnfrage::Bedeutung(BedeutungUnter::Primzahlen));
        report.insert_partial(SpaltenAnfrage::Galaxie(GalaxieUnter::Primzahlen));
    }

    if has("primmotgleichf") {
        report.insert_partial(SpaltenAnfrage::Primvielfache(GeneratorUnter::MotivGleichfoermig));
        report.insert_partial(SpaltenAnfrage::Multiplikationen(GeneratorUnter::MotivGleichfoermig));
    }
    if has("primstrukgleichf") {
        report.insert_partial(SpaltenAnfrage::Primvielfache(GeneratorUnter::StrukturGleichfoermig));
        report.insert_partial(SpaltenAnfrage::Multiplikationen(GeneratorUnter::StrukturGleichfoermig));
    }
    if has("primmotivstern") {
        report.insert_partial(SpaltenAnfrage::Primvielfache(GeneratorUnter::MotivStern));
        report.insert_partial(SpaltenAnfrage::Multiplikationen(GeneratorUnter::MotivStern));
    }
    if has("primstrukturstern") {
        report.insert_partial(SpaltenAnfrage::Primvielfache(GeneratorUnter::StrukturStern));
        report.insert_partial(SpaltenAnfrage::Multiplikationen(GeneratorUnter::StrukturStern));
    }
    if has("primmotivsterngebr") {
        report.insert_partial(SpaltenAnfrage::Primvielfache(GeneratorUnter::MotivGebrStern));
        report.insert_partial(SpaltenAnfrage::Multiplikationen(GeneratorUnter::MotivGebrStern));
    }
    if has("primstruktursterngebr") {
        report.insert_partial(SpaltenAnfrage::Primvielfache(GeneratorUnter::StrukGebrStern));
        report.insert_partial(SpaltenAnfrage::Multiplikationen(GeneratorUnter::StrukGebrStern));
    }
    if has("primmotgleichfgebr") {
        report.insert_partial(SpaltenAnfrage::Primvielfache(GeneratorUnter::MotivGebrGleichf));
        report.insert_partial(SpaltenAnfrage::Multiplikationen(GeneratorUnter::MotivGebrGleichf));
    }
    if has("primstrukgleichfgebr") {
        report.insert_partial(SpaltenAnfrage::Primvielfache(GeneratorUnter::StrukGebrGleichf));
        report.insert_partial(SpaltenAnfrage::Multiplikationen(GeneratorUnter::StrukGebrGleichf));
    }
}

fn collect_exact_bridge_pairs(bereich: &TextBereich, report: &mut ReverseRequestReport) {
    if !bereich.exact_meta_konkret_specs.is_empty() {
        for (aliases, _pair) in META_KONKRET_MAPPINGS {
            if let Some(first) = aliases.first() {
                report.insert_partial(SpaltenAnfrage::Bridge(BridgeUnter::UniversumMetaKonkret((*first).to_string())));
            }
        }
    }

    if !bereich.exact_modal_pairs.is_empty() {
        for (aliases, _cols, maybe_pair) in EIGENSCHAFT_MAPPINGS {
            if maybe_pair.is_some() {
                if let Some(first) = aliases.first() {
                    report.insert_partial(SpaltenAnfrage::Bridge(BridgeUnter::EigenschaftenN((*first).to_string())));
                }
            }
        }
    }
}

pub fn print_reverse_request_pairs(
    kategorie_map: &KategorieMap,
    bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
) {
    if matches!(bereich.output_syntax, OutputSyntax::HTML) {
        return;
    }

    let visible_columns = collect_visible_columns(bereich);
    if visible_columns.is_empty()
        && generated_befehle.is_empty()
        && bereich.exact_meta_konkret_specs.is_empty()
        && bereich.exact_modal_pairs.is_empty()
        && bereich.pypy_compat.gebrochengalaxie.is_empty()
        && bereich.pypy_compat.gebrochenuniversum.is_empty()
        && bereich.pypy_compat.gebrochenemotion.is_empty()
        && bereich.pypy_compat.gebrochengroesse.is_empty()
        && bereich.pypy_compat.kombi_galaxie.is_empty()
        && bereich.pypy_compat.kombi_universum.is_empty()
    {
        return;
    }

    let mut report = ReverseRequestReport::default();

    collect_exact_and_partial_direct_pairs(kategorie_map, &visible_columns, &mut report);
    collect_fraction_pairs(bereich, &mut report);
    collect_kombi_pairs(bereich, &mut report);
    collect_generated_pairs(generated_befehle, &mut report);
    collect_exact_bridge_pairs(bereich, &mut report);

    if report.is_empty() {
        return;
    }

    println!();

    if !report.exakte_auswahl.is_empty() {
        println!("══════════════════════════════════════════════");
        println!("Exakt äquivalente Spalten-Auswahl:");
        println!("══════════════════════════════════════════════");
        for pair in &report.exakte_auswahl {
            println!("  {}", pair.to_cli());
        }
    }

    if !report.weitere_passende_auswahl.is_empty() {
        if !report.exakte_auswahl.is_empty() {
            println!();
        }
        println!("══════════════════════════════════════════════");
        println!("Weitere passende, aber nicht exakt äquivalente Auswahl:");
        println!("══════════════════════════════════════════════");
        for pair in &report.weitere_passende_auswahl {
            println!("  {}", pair.to_cli());
        }
    }
}

pub fn print_reverse_request_pairs_dual(
    kategorie_map: &KategorieMap,
    bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
) {
    print_reverse_request_pairs(kategorie_map, bereich, generated_befehle);
}
