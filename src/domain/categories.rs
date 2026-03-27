use crate::domain::spalten_anfrage::SpaltenAnfrage;
use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::domain::exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};
use crate::domain::python_source_of_truth::{self, PY_DECLS};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OberkategorieName(String);

impl OberkategorieName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for OberkategorieName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for OberkategorieName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for OberkategorieName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnterkategorieName(String);

impl UnterkategorieName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UnterkategorieName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for UnterkategorieName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for UnterkategorieName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpaltenNummern(Box<[u32]>);

impl SpaltenNummern {
    pub fn new(mut values: Vec<u32>) -> Self {
        values.sort_unstable();
        values.dedup();
        Self(values.into_boxed_slice())
    }

    pub fn as_slice(&self) -> &[u32] {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains(&self, needle: u32) -> bool {
        self.0.contains(&needle)
    }

    pub fn to_vec(&self) -> Vec<u32> {
        self.0.to_vec()
    }
}

impl Default for SpaltenNummern {
    fn default() -> Self {
        Self(Vec::new().into_boxed_slice())
    }
}

impl AsRef<[u32]> for SpaltenNummern {
    fn as_ref(&self) -> &[u32] {
        self.as_slice()
    }
}


pub trait UnterkategorieEntry {
    fn unter_name(&self) -> &str;
    fn column_numbers(&self) -> &[u32];
}

pub trait OberkategorieEntry {
    type Unter: UnterkategorieEntry;

    fn ober_name(&self) -> &str;
    fn unterkategorien(&self) -> &[Self::Unter];
}

pub trait KategorieProvider {
    type Ober: OberkategorieEntry;

    fn hauptkategorien(&self) -> &[Self::Ober];
}

#[derive(Debug, Clone)]
pub struct Unterkategorie {
    pub name: UnterkategorieName,
    pub spaltennummern: SpaltenNummern,
}

impl Unterkategorie {
    pub fn new(name: impl Into<UnterkategorieName>, spaltennummern: Vec<u32>) -> Self {
        Self {
            name: name.into(),
            spaltennummern: SpaltenNummern::new(spaltennummern),
        }
    }
}

impl UnterkategorieEntry for Unterkategorie {
    fn unter_name(&self) -> &str {
        self.name.as_str()
    }

    fn column_numbers(&self) -> &[u32] {
        self.spaltennummern.as_slice()
    }
}

#[derive(Debug, Clone)]
pub struct Oberkategorie {
    pub name: OberkategorieName,
    pub unterkategorien: Vec<Unterkategorie>,
}

impl Oberkategorie {
    pub fn new(name: impl Into<OberkategorieName>, unterkategorien: Vec<Unterkategorie>) -> Self {
        Self {
            name: name.into(),
            unterkategorien,
        }
    }
}

impl OberkategorieEntry for Oberkategorie {
    type Unter = Unterkategorie;

    fn ober_name(&self) -> &str {
        self.name.as_str()
    }

    fn unterkategorien(&self) -> &[Self::Unter] {
        &self.unterkategorien
    }
}

pub struct KategorieMap {
    pub hauptkategorien: Vec<Oberkategorie>,
}

impl KategorieProvider for KategorieMap {
    type Ober = Oberkategorie;

    fn hauptkategorien(&self) -> &[Self::Ober] {
        &self.hauptkategorien
    }
}

#[derive(Debug, Clone, Default)]
pub struct GeneratedInference {
    pub generated_befehle: Vec<String>,
    pub required_columns: Vec<u32>,
    pub direct_columns: Vec<u32>,
}

fn normalize_key(s: &str) -> String {
    s.to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
}

impl KategorieMap {
    pub fn new() -> Self {
        let mut instanz = Self {
            hauptkategorien: Vec::new(),
        };
        instanz.lade_kategorien();
        instanz
    }

    pub fn alle_paare_fuer_cli_alles(&self) -> Vec<(String, String)> {
        let mut paare = self.alle_paare();

        fn push_pair(paare: &mut Vec<(String, String)>, ober: &str, unter: &str) {
            paare.push((ober.to_string(), unter.to_string()));
        }

        // Kombi-Galaxie
        for unter in [
            "tiere",
            "berufe",
            "kreativität",
            "liebe",
            "männer",
            "persönlichkeit",
            "religion",
            "motive",
            "emotionen",
            "personen",
            "wirtschaftssysteme",
            "eigentum",
        ] {
            push_pair(&mut paare, "KombinationGalaxie", unter);
        }

        // Kombi-Universum
        for unter in [
            "tiere",
            "berufe",
            "transzendentalien",
            "primzahlkreuz",
            "persönlichkeit",
            "religion",
            "motive",
            "ontologie",
            "personen",
            "mechanismen",
            "gegentranszendentalien",
            "maschinen",
            "geist",
            "bewusstsein",
        ] {
            push_pair(&mut paare, "KombinationUniversum", unter);
        }

        // Rein generatorische Requests
        for (ober, unter) in [
            ("Universum", "Primzahlkreuz"),
            ("Bedeutung", "Primzahlkreuz"),
            ("Pro_Contra", "Primzahlkreuz"),

            ("Menschliches", "Liebe"),
            ("Grundstrukturen", "Liebe"),

            ("Planet", "Gleichheit"),
            ("Menschliches", "Gleichheit"),
            ("Grundstrukturen", "Gleichheit"),

            ("Universum", "Geist"),
            ("Multiversum", "Geist"),
            ("Grundstrukturen", "Geist"),

            ("Wichtigstes_zum_verstehen", "Gestirn"),
            ("Bedeutung", "Gestirn"),

            ("Wichtigstes_zum_verstehen", "Primzahlen"),
            ("Bedeutung", "Primzahlen"),
            ("Galaxie", "Primzahlen"),

            ("Modallogik", "Modallogik"),
        ] {
            push_pair(&mut paare, ober, unter);
        }

        // Prim-/Multiplikations-Generatoren
        for ober in ["primvielfache", "multiplikationen"] {
            for unter in [
                "motivgleichfoermig",
                "strukturgleichfoermig",
                "motivstern",
                "strukturstern",
                "motivgebrstern",
                "strukgebrstern",
                "motivgebrgleichf",
                "strukgebrgleichf",
            ] {
                push_pair(&mut paare, ober, unter);
            }
        }

        paare.sort();
        paare.dedup();
        paare
    }


    pub fn infer_generated_request(&self, request: &SpaltenAnfrage) -> Option<GeneratedInference> {
        let (ober, unter) = request.ober_unter_cli_pair();
        self.infer_generated_pair(&ober, &unter)
    }

    pub fn finde_spaltennummern_fuer_request(&self, request: &SpaltenAnfrage) -> Vec<u32> {
        let (ober, unter) = request.ober_unter_cli_pair();
        self.finde_spaltennummern_fuer_kategorien(&ober, &unter)
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

        Self::merge_exact_eigenschaften_aliases(&mut main_to_sub);
        Self::merge_meta_konkret_aliases(&mut main_to_sub);
        Self::merge_fraction_number_aliases(&mut main_to_sub);

        self.hauptkategorien = Self::convert_main_to_hauptkategorien(main_to_sub);
    }

    fn merge_exact_eigenschaften_aliases(
        main_to_sub: &mut HashMap<String, HashMap<String, Vec<u32>>>,
    ) {
        let main_aliases = [
            "Eigenschaften_1/n",
            "konzept2",
            "konzepte2",
            "Eigenschaft",
            "Eigenschaften",
            "konzept",
            "konzepte",
        ];

        for (aliases, direct_columns, maybe_pair) in EIGENSCHAFT_MAPPINGS {
            let mut ids: Vec<u32> = direct_columns
                .iter()
                .copied()
                .map(|n| (n as u32) + 1)
                .collect();

            if let Some((left, right)) = maybe_pair {
                ids.push((*left as u32) + 1);
                ids.push((*right as u32) + 1);
            }

            ids.sort_unstable();
            ids.dedup();

            for &main_cat in &main_aliases {
                for &sub_cat in *aliases {
                    Self::insert_entry(main_to_sub, main_cat, sub_cat, ids.clone());
                }
            }
        }
    }

    fn merge_meta_konkret_aliases(
        main_to_sub: &mut HashMap<String, HashMap<String, Vec<u32>>>,
    ) {
        let main_aliases = ["Universum_Metakonkret", "MetaKonkret", "metakonkret"];

        for (aliases, (left, right)) in META_KONKRET_MAPPINGS {
            let ids = vec![(*left as u32) + 1, (*right as u32) + 1];
            for &main_cat in &main_aliases {
                for &sub_cat in *aliases {
                    Self::insert_entry(main_to_sub, main_cat, sub_cat, ids.clone());
                }
            }
        }
    }

    fn merge_fraction_number_aliases(
        main_to_sub: &mut HashMap<String, HashMap<String, Vec<u32>>>,
    ) {
        let families: &[(&[&str], std::ops::RangeInclusive<u32>)] = &[
            (
                &["gebrochen-rational_Galaxie_n/m", "gebrochengalaxie"],
                2..=24,
            ),
            (
                &["gebrochen-rational_Universum_n/m", "gebrochenuniversum"],
                2..=24,
            ),
            (
                &[
                    "gebrochen-rational_Gefuehle_n/m",
                    "gebrochenemotion",
                    "gebrochengemotion",
                ],
                2..=24,
            ),
            (
                &[
                    "gebrochen-rational_Strukturgroesse_n/m",
                    "gebrochengroesse",
                ],
                2..=24,
            ),
        ];

        for (main_aliases, range) in families {
            for n in range.clone() {
                let sub = n.to_string();
                let ids = vec![n];
                for &main_cat in *main_aliases {
                    Self::insert_entry(main_to_sub, main_cat, &sub, ids.clone());
                }
            }
        }
    }

    fn convert_main_to_hauptkategorien(
        main_to_sub: HashMap<String, HashMap<String, Vec<u32>>>,
    ) -> Vec<Oberkategorie> {
        let mut hauptkategorien: Vec<Oberkategorie> = main_to_sub
            .into_iter()
            .map(|(haupt_name, unter_map)| {
                let mut unterkategorien: Vec<Unterkategorie> = unter_map
                    .into_iter()
                    .map(|(unter_name, spaltennummern)| Unterkategorie::new(unter_name, spaltennummern))
                    .collect();

                unterkategorien.sort_by(|a, b| a.name.as_str().cmp(b.name.as_str()));
                Oberkategorie::new(haupt_name, unterkategorien)
            })
            .collect();

        hauptkategorien.sort_by(|a, b| a.name.as_str().cmp(b.name.as_str()));
        hauptkategorien
    }

    pub fn alle_paare(&self) -> Vec<(String, String)> {
        let mut paare = Vec::new();

        for haupt in &self.hauptkategorien {
            for unter in &haupt.unterkategorien {
                paare.push((haupt.name.as_str().to_string(), unter.name.as_str().to_string()));
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
                nummern.extend(unter.spaltennummern.as_slice().iter().copied());
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
}
pub fn lade_kategorie_map() -> KategorieMap {
        KategorieMap::new()
    }
