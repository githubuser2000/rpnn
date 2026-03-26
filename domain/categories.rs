use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::domain::python_source_of_truth::{self, PY_DECLS};

fn normalize_key(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnterkategorieName(pub String);

impl UnterkategorieName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn normalized(&self) -> String {
        normalize_key(&self.0)
    }

    pub fn matches_str(&self, other: &str) -> bool {
        self.normalized() == normalize_key(other)
    }
}

impl fmt::Display for UnterkategorieName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OberkategorieKey {
    Menschliches,
    Universum,
    Religion,
    Religionen,
    Galaxie,
    Planet,
    Bedeutung,
    ProContra,
    Grundstrukturen,
    Multiversum,
    WichtigstesZumVerstehen,
    KombinationGalaxie,
    KombinationUniversum,
    GebrochenRationalGalaxie,
    GebrochenRationalUniversum,
    GebrochenRationalStrukturgroesse,
    GebrochenRationalGefuehle,
    EigenschaftenN,
    UniversumMetaKonkret,
    Primvielfache,
    Multiplikationen,
    Unknown(String),
}

impl OberkategorieKey {
    pub fn from_raw(s: &str) -> Self {
        match normalize_key(s).as_str() {
            "menschliches" => Self::Menschliches,
            "universum" => Self::Universum,
            "religion" => Self::Religion,
            "religionen" => Self::Religionen,
            "galaxie" => Self::Galaxie,
            "planet" | "planet1012" => Self::Planet,
            "bedeutung" => Self::Bedeutung,
            "procontra" => Self::ProContra,
            "grundstrukturen" => Self::Grundstrukturen,
            "multiversum" => Self::Multiversum,
            "wichtigsteszumverstehen" => Self::WichtigstesZumVerstehen,
            "kombinationgalaxie" => Self::KombinationGalaxie,
            "kombinationuniversum" => Self::KombinationUniversum,
            "gebrochenrationalgalaxienm" | "gebrochenrationalgalaxie" => {
                Self::GebrochenRationalGalaxie
            }
            "gebrochenrationaluniversumnm" | "gebrochenrationaluniversum" => {
                Self::GebrochenRationalUniversum
            }
            "gebrochenrationalstrukturgroessenm" | "gebrochenrationalstrukturgroesse" => {
                Self::GebrochenRationalStrukturgroesse
            }
            "gebrochenrationalgefuhlenm"
            | "gebrochenrationalgefuehlenm"
            | "gebrochenrationalgefuhle"
            | "gebrochenrationalgefuehle"
            | "gebrochenrationalemotion"
            | "gebrochenrationalemotionen" => Self::GebrochenRationalGefuehle,
            "eigenschaftenn" => Self::EigenschaftenN,
            "universummetakonkret" => Self::UniversumMetaKonkret,
            "primvielfache" => Self::Primvielfache,
            "multiplikationen" => Self::Multiplikationen,
            _ => Self::Unknown(s.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Menschliches => "Menschliches",
            Self::Universum => "Universum",
            Self::Religion => "Religion",
            Self::Religionen => "Religionen",
            Self::Galaxie => "Galaxie",
            Self::Planet => "Planet",
            Self::Bedeutung => "Bedeutung",
            Self::ProContra => "Pro_Contra",
            Self::Grundstrukturen => "Grundstrukturen",
            Self::Multiversum => "Multiversum",
            Self::WichtigstesZumVerstehen => "Wichtigstes_zum_verstehen",
            Self::KombinationGalaxie => "KombinationGalaxie",
            Self::KombinationUniversum => "KombinationUniversum",
            Self::GebrochenRationalGalaxie => "gebrochen-rational_Galaxie_n/m",
            Self::GebrochenRationalUniversum => "gebrochen-rational_Universum_n/m",
            Self::GebrochenRationalStrukturgroesse => "gebrochen-rational_Strukturgroesse_n/m",
            Self::GebrochenRationalGefuehle => "gebrochen-rational_Gefühle_n/m",
            Self::EigenschaftenN => "Eigenschaften_n",
            Self::UniversumMetaKonkret => "universummetakonkret",
            Self::Primvielfache => "primvielfache",
            Self::Multiplikationen => "multiplikationen",
            Self::Unknown(s) => s.as_str(),
        }
    }

    pub fn normalized(&self) -> String {
        normalize_key(self.as_str())
    }

    pub fn matches_str(&self, other: &str) -> bool {
        self.normalized() == normalize_key(other)
    }
}

impl fmt::Display for OberkategorieKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialOrd for OberkategorieKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OberkategorieKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Unterkategorie {
    pub name: UnterkategorieName,
    pub spaltennummern: Vec<u32>,
}

impl Unterkategorie {
    pub fn new(name: impl Into<String>, spaltennummern: Vec<u32>) -> Self {
        Self {
            name: UnterkategorieName::new(name),
            spaltennummern,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hauptkategorie {
    pub key: OberkategorieKey,
    pub unterkategorien: Vec<Unterkategorie>,
}

impl Hauptkategorie {
    pub fn new(name: impl Into<String>, unterkategorien: Vec<Unterkategorie>) -> Self {
        let raw_name: String = name.into();
        Self {
            key: OberkategorieKey::from_raw(&raw_name),
            unterkategorien,
        }
    }
}

pub type Oberkategorie = Hauptkategorie;

pub struct KategorieMap {
    pub hauptkategorien: Vec<Hauptkategorie>,
}

#[derive(Debug, Clone, Default)]
pub struct GeneratedInference {
    pub generated_befehle: Vec<String>,
    pub required_columns: Vec<u32>,
    pub direct_columns: Vec<u32>,
}

impl KategorieMap {
    pub fn new() -> Self {
        let mut instanz = Self {
            hauptkategorien: Vec::new(),
        };
        instanz.lade_kategorien();
        instanz
    }

    pub fn infer_generated_pair(&self, ober: &str, unter: &str) -> Option<GeneratedInference> {
        let ober_n = normalize_key(ober);
        let unter_n = normalize_key(unter);

        let mut direct_columns = self.finde_spaltennummern_fuer_kategorien(ober, unter);
        direct_columns.sort();
        direct_columns.dedup();

        let has = |n: u32| direct_columns.contains(&n);

        let mut generated_befehle = Vec::<String>::new();
        let mut required_columns = Vec::<u32>::new();

        if matches!(ober_n.as_str(), "procontra" | "bedeutung" | "universum")
            && matches!(unter_n.as_str(), "primzahlkreuz" | "primzahlkreuzprocontra")
        {
            generated_befehle.push("primzahlkreuzprocontra".to_string());
        }

        if has(9) {
            generated_befehle.push("lovepolygon".to_string());
            required_columns.push(9);
        }

        if has(132) {
            generated_befehle.push("gleichheitfreiheit".to_string());
            required_columns.push(132);
        }

        if has(242) {
            generated_befehle.push("geistemotionenergiematerietopologie".to_string());
            required_columns.push(242);
        }

        if has(64) {
            generated_befehle.push("primcreativitytype".to_string());
            generated_befehle.push("mondexponzierenlogarithmustyp".to_string());
            required_columns.push(64);
        }

        if has(19) || has(90) {
            generated_befehle.push("vervielfachezeile".to_string());
            if has(19) {
                required_columns.push(19);
            }
            if has(90) {
                required_columns.push(90);
            }
        }

        generated_befehle.sort();
        generated_befehle.dedup();
        required_columns.sort();
        required_columns.dedup();

        if generated_befehle.is_empty() && direct_columns.is_empty() {
            None
        } else {
            Some(GeneratedInference {
                generated_befehle,
                required_columns,
                direct_columns,
            })
        }
    }

    pub fn finde_spaltennummern_exakt(&self, ober: &str, unter: &str) -> Vec<u32> {
        python_source_of_truth::exact_columns_for_pair(ober, unter)
            .into_iter()
            .map(|n| n + 1)
            .collect()
    }

    pub fn finde_spaltennummern_fuer_kategorien(&self, ober: &str, unter: &str) -> Vec<u32> {
        let exakt = self.finde_spaltennummern_exakt(ober, unter);
        if !exakt.is_empty() {
            return exakt;
        }

        python_source_of_truth::fuzzy_columns_for_pair(ober, unter)
            .into_iter()
            .map(|n| n + 1)
            .collect()
    }

    fn lade_kategorien(&mut self) {
        let mut main_to_sub: HashMap<String, HashMap<String, Vec<u32>>> = HashMap::new();

        for decl in PY_DECLS {
            let korrigierte_ids: Vec<u32> = decl.columns.iter().map(|&id| id + 1).collect();

            for &main_cat in decl.main_aliases {
                for &sub_cat in decl.sub_aliases {
                    Self::insert_entry(
                        &mut main_to_sub,
                        main_cat,
                        sub_cat,
                        korrigierte_ids.clone(),
                    );
                }
            }
        }

        self.hauptkategorien = Self::convert_main_to_hauptkategorien(main_to_sub);
    }

    fn convert_main_to_hauptkategorien(
        main_to_sub: HashMap<String, HashMap<String, Vec<u32>>>,
    ) -> Vec<Hauptkategorie> {
        let mut hauptkategorien: Vec<Hauptkategorie> = main_to_sub
            .into_iter()
            .map(|(haupt_name, unter_map)| {
                let mut unterkategorien: Vec<Unterkategorie> = unter_map
                    .into_iter()
                    .map(|(unter_name, mut spaltennummern)| {
                        spaltennummern.sort();
                        spaltennummern.dedup();
                        Unterkategorie::new(unter_name, spaltennummern)
                    })
                    .collect();

                unterkategorien.sort_by(|a, b| a.name.cmp(&b.name));
                Hauptkategorie::new(haupt_name, unterkategorien)
            })
            .collect();

        hauptkategorien.sort_by(|a, b| a.key.cmp(&b.key));
        hauptkategorien
    }

    pub fn alle_paare(&self) -> Vec<(String, String)> {
        let mut paare = Vec::new();

        for haupt in &self.hauptkategorien {
            for unter in &haupt.unterkategorien {
                paare.push((haupt.key.to_string(), unter.name.to_string()));
            }
        }

        paare.sort();
        paare.dedup();
        paare
    }

    pub fn alle_spaltennummern(&self) -> Vec<u32> {
        let mut nummern = Vec::new();

        for haupt in &self.hauptkategorien {
            for unter in &haupt.unterkategorien {
                nummern.extend(unter.spaltennummern.iter().copied());
            }
        }

        nummern.sort_unstable();
        nummern.dedup();
        nummern
    }

    fn insert_entry(
        main_to_sub: &mut HashMap<String, HashMap<String, Vec<u32>>>,
        main_category: &str,
        sub_category: &str,
        new_ids: Vec<u32>,
    ) {
        let main_entry = main_to_sub
            .entry(main_category.to_string())
            .or_insert_with(HashMap::new);

        let existing_ids = main_entry
            .entry(sub_category.to_string())
            .or_insert_with(Vec::new);

        let mut all_ids: HashSet<u32> = existing_ids.iter().cloned().collect();
        for &id in &new_ids {
            all_ids.insert(id);
        }

        let mut sorted_ids: Vec<u32> = all_ids.into_iter().collect();
        sorted_ids.sort();
        *existing_ids = sorted_ids;
    }

    pub fn filtere_nach_spaltennummern(&self, nummern: &[usize]) -> Vec<(String, String, Vec<u32>)> {
        let nummern_set: HashSet<u32> = nummern.iter().map(|&n| n as u32).collect();
        let mut result = Vec::new();

        for haupt in &self.hauptkategorien {
            for unter in &haupt.unterkategorien {
                let passende_spalten: Vec<u32> = unter
                    .spaltennummern
                    .iter()
                    .copied()
                    .filter(|num| nummern_set.contains(num))
                    .collect();

                if !passende_spalten.is_empty() {
                    result.push((haupt.key.to_string(), unter.name.to_string(), passende_spalten));
                }
            }
        }

        result
    }

    pub fn generiere_sql_selects(
        &self,
        oberkategorie_name: &str,
        unterkategorie_name: &str,
        spalten_filter: Option<&[usize]>,
    ) -> String {
        let mut output = String::new();

        output.push_str("-- SQL SELECTS für Kategorie-Datenbank\n");
        output.push_str(&format!(
            "-- Spaltennamen: {}, {}\n\n",
            oberkategorie_name, unterkategorie_name
        ));

        output.push_str("CREATE TABLE kategorien (\n");
        output.push_str("  id INTEGER PRIMARY KEY AUTOINCREMENT,\n");
        output.push_str(&format!("  {} VARCHAR(255) NOT NULL,\n", oberkategorie_name));
        output.push_str(&format!("  {} VARCHAR(255) NOT NULL,\n", unterkategorie_name));
        output.push_str("  spaltennummer INTEGER NOT NULL\n");
        output.push_str(");\n\n");

        output.push_str("INSERT INTO kategorien (");
        output.push_str(oberkategorie_name);
        output.push_str(", ");
        output.push_str(unterkategorie_name);
        output.push_str(", spaltennummer) VALUES\n");

        let mut first = true;
        for haupt in &self.hauptkategorien {
            for unter in &haupt.unterkategorien {
                let spalten_iter: Vec<u32> = if let Some(filter) = spalten_filter {
                    let filter_set: HashSet<u32> = filter.iter().map(|&n| n as u32).collect();
                    unter
                        .spaltennummern
                        .iter()
                        .copied()
                        .filter(|num| filter_set.contains(num))
                        .collect()
                } else {
                    unter.spaltennummern.clone()
                };

                for spaltennummer in spalten_iter {
                    if !first {
                        output.push_str(",\n");
                    }
                    output.push_str(&format!(
                        "  ('{}', '{}', {})",
                        haupt.key, unter.name, spaltennummer
                    ));
                    first = false;
                }
            }
        }

        if !first {
            output.push_str(";\n\n");
        }

        output.push_str("-- Beispiele für SELECT-Abfragen:\n\n");
        output.push_str(&format!(
            "-- 1. Alle eindeutigen {}s:\n",
            oberkategorie_name
        ));
        output.push_str(&format!(
            "SELECT DISTINCT {} FROM kategorien ORDER BY {};\n\n",
            oberkategorie_name, oberkategorie_name
        ));

        output.push_str(&format!(
            "-- 2. {}s für eine bestimmte {}:\n",
            unterkategorie_name, oberkategorie_name
        ));
        output.push_str(&format!("SELECT DISTINCT {} FROM kategorien ", unterkategorie_name));
        output.push_str(&format!(
            "WHERE {} = 'Menschliches' ORDER BY {};\n\n",
            oberkategorie_name, unterkategorie_name
        ));

        output.push_str("-- 3. Spaltennummern für eine Kategorie-Kombination:\n");
        output.push_str("SELECT spaltennummer FROM kategorien ");
        output.push_str(&format!(
            "WHERE {} = 'Universum' AND {} = 'Transzendentalien';\n",
            oberkategorie_name, unterkategorie_name
        ));

        output
    }
}

pub fn lade_kategorie_map() -> KategorieMap {
    KategorieMap::new()
}
