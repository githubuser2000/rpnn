use crate::domain::decl_model::HtmlDeclMeta;
use crate::domain::model::spalten_anfrage::ColumnTarget;
use crate::domain::resolver::request_resolver::resolve_request;
use crate::domain::spalten_anfrage::SpaltenAnfrage;
use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::domain::eigenschaften::{EigenschaftKeyId, EigenschaftStandardFamilie};
use crate::domain::exact_mappings::META_KONKRET_MAPPINGS;
use crate::domain::ids::domain_id::DomainId;
use crate::domain::model::spalten_anfrage::{
    EigenschaftRequest as CanonicalEigenschaftRequest,
    EigenschaftsFamilie as CanonicalEigenschaftsFamilie,
    SpaltenAnfrage as CanonicalSpaltenAnfrage,
    StandardUnterId as CanonicalStandardUnterId,
};
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



fn canonical_target_to_columns(target: &ColumnTarget) -> Vec<u32> {
    match target {
        ColumnTarget::DirectColumn(id) => vec![*id as u32],
        ColumnTarget::DirectColumns(ids) => ids.iter().map(|id| *id as u32).collect(),
        ColumnTarget::Pair(left, right) => vec![*left as u32, *right as u32],
        ColumnTarget::Generator(_) | ColumnTarget::Combination(_) => Vec::new(),
    }
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

    pub fn alle_typed_requests_fuer_cli_alles(&self) -> Vec<CanonicalSpaltenAnfrage> {
        let mut requests = Self::typed_eigenschaften_requests_fuer_alles();
        requests.sort_by_key(|req| req.to_cli_pair());
        requests.dedup();
        requests
    }

    pub fn alle_paare_fuer_cli_alles(&self) -> Vec<(String, String)> {
        use std::collections::BTreeSet;

        let mut paare_set: BTreeSet<(String, String)> = self.alle_paare().into_iter().collect();

        fn push_pair(paare: &mut BTreeSet<(String, String)>, ober: &str, unter: &str) {
            paare.insert((ober.to_string(), unter.to_string()));
        }

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
            push_pair(&mut paare_set, "KombinationGalaxie", unter);
        }

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
            push_pair(&mut paare_set, "KombinationUniversum", unter);
        }

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
            push_pair(&mut paare_set, ober, unter);
        }

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
                push_pair(&mut paare_set, ober, unter);
            }
        }

        for request in self.alle_typed_requests_fuer_cli_alles() {
            if let Some((ober, unter)) = request.to_cli_pair() {
                paare_set.insert((ober, unter));
            }
        }

        paare_set.into_iter().collect()
    }

    pub fn finde_spaltennummern_fuer_canonical_request(&self, request: &CanonicalSpaltenAnfrage) -> Vec<u32> {
        resolve_request(request.clone())
            .map(|spec| canonical_target_to_columns(&spec.target))
            .unwrap_or_default()
    }

    pub fn infer_generated_canonical_request(&self, request: &CanonicalSpaltenAnfrage) -> Option<GeneratedInference> {
        resolve_request(request.clone()).map(|spec| match spec.target {
            ColumnTarget::Generator(generator) => GeneratedInference {
                generated_befehle: vec![generator.art.to_string().to_lowercase()],
                required_columns: Vec::new(),
                direct_columns: Vec::new(),
            },
            ColumnTarget::Combination(_) => GeneratedInference::default(),
            other => {
                let direct_columns = canonical_target_to_columns(&other);
                GeneratedInference {
                    generated_befehle: Vec::new(),
                    required_columns: direct_columns.clone(),
                    direct_columns,
                }
            }
        }).filter(|inf| !inf.generated_befehle.is_empty() || !inf.required_columns.is_empty() || !inf.direct_columns.is_empty())
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
        Self::merge_html_meta_aliases(&mut main_to_sub);

        self.hauptkategorien = Self::convert_main_to_hauptkategorien(main_to_sub);
    }

    fn merge_exact_eigenschaften_aliases(
        main_to_sub: &mut HashMap<String, HashMap<String, Vec<u32>>>,
    ) {
        for key in EigenschaftKeyId::ALL.iter().copied() {
            let ids = key.all_column_ids_1_based();
            if ids.is_empty() {
                continue;
            }

            for main_cat in Self::eigenschaft_main_aliases_for_key(key) {
                Self::insert_entry(main_to_sub, &main_cat, key.canonical_name(), ids.clone());
            }
        }
    }

    fn eigenschaft_main_aliases_for_key(key: EigenschaftKeyId) -> Vec<String> {
        let mut mains = HashSet::<String>::new();
        mains.insert("Eigenschaft".to_string());
        mains.insert("Eigenschaften".to_string());
        mains.insert("konzept".to_string());
        mains.insert("konzepte".to_string());

        match key.standard_familie() {
            EigenschaftStandardFamilie::N => {
                mains.insert("Eigenschaften_n".to_string());
                mains.insert("konzept1".to_string());
                mains.insert("konzepte1".to_string());
            }
            EigenschaftStandardFamilie::EinsDurchN => {
                mains.insert("Eigenschaften_1/n".to_string());
                mains.insert("konzept2".to_string());
                mains.insert("konzepte2".to_string());
            }
        }

        for col in key.all_column_ids_1_based().iter().map(|n| *n - 1) {
            if let Some(meta) = python_source_of_truth::exact_decl_meta_for_column(col) {
                for main in Self::extract_main_categories_from_decl_meta(&meta) {
                    let normalized = normalize_key(&main);
                    match normalized.as_str() {
                        "eigenschaften1n" => {
                            mains.insert("Eigenschaften_1/n".to_string());
                            mains.insert("konzept2".to_string());
                            mains.insert("konzepte2".to_string());
                        }
                        "eigenschaftenn" | "eigenschaft" | "eigenschaften" => {
                            mains.insert("Eigenschaften_n".to_string());
                            mains.insert("konzept1".to_string());
                            mains.insert("konzepte1".to_string());
                        }
                        _ => {}
                    }
                }
            }
        }

        let mut out: Vec<String> = mains.into_iter().collect();
        out.sort();
        out
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

    fn merge_html_meta_aliases(
        main_to_sub: &mut HashMap<String, HashMap<String, Vec<u32>>>,
    ) {
        for (col, meta) in python_source_of_truth::all_exact_decl_meta() {
            let mains = Self::extract_main_categories_from_decl_meta(&meta);
            let subs = Self::extract_sub_categories_from_decl_meta(&meta);
            if mains.is_empty() || subs.is_empty() {
                continue;
            }

            let ids = vec![col + 1];
            for main in &mains {
                for sub in &subs {
                    Self::insert_entry(main_to_sub, main, sub, ids.clone());
                }
            }
        }
    }

    fn extract_main_categories_from_decl_meta(meta: &HtmlDeclMeta) -> Vec<String> {
        let mut out: Vec<String> = meta
            .p1_groups
            .iter()
            .map(|s| s.trim().trim_start_matches('✗').trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        out.sort();
        out.dedup();
        out
    }

    fn extract_sub_categories_from_decl_meta(meta: &HtmlDeclMeta) -> Vec<String> {
        let mut out: Vec<String> = meta
            .p2_slots
            .iter()
            .filter_map(|opt| opt.as_ref())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .filter(|s| !s.chars().all(|c| c.is_ascii_digit()))
            .collect();
        out.sort();
        out.dedup();
        out
    }

    fn extract_main_categories_from_meta(meta: &str) -> Vec<String> {
        let Some(start) = meta.find("p1_") else { return Vec::new(); };
        let Some(end_rel) = meta[start..].find(", p2_p3_0_") else { return Vec::new(); };
        let slice = &meta[start + 3 .. start + end_rel];

        let mut out = Vec::new();
        for raw in slice.split(',') {
            let value = raw.trim().trim_start_matches('✗').trim();
            if value.is_empty() { continue; }
            out.push(value.to_string());
        }
        out.sort();
        out.dedup();
        out
    }

    fn extract_sub_categories_from_meta(meta: &str) -> Vec<String> {
        let Some(start) = meta.find("p2_p3_0_") else { return Vec::new(); };
        let end = meta[start..].find(", p4_").map(|idx| start + idx).unwrap_or(meta.len());
        let slice = &meta[start + 8 .. end];

        let mut out = Vec::new();
        for raw in slice.split(',') {
            let value = raw.trim();
            if value.is_empty() { continue; }

            let candidate = if let Some(pos) = value.find('_') {
                let (prefix, rest) = value.split_at(pos);
                if prefix == "p3" || prefix == "p2" {
                    rest.trim_start_matches('_').trim()
                } else {
                    value
                }
            } else {
                value
            };

            if candidate.is_empty() { continue; }
            if candidate.chars().all(|c| c.is_ascii_digit()) { continue; }
            out.push(candidate.to_string());
        }

        out.sort();
        out.dedup();
        out
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

        paare
    }

    fn typed_eigenschaften_requests_fuer_alles() -> Vec<CanonicalSpaltenAnfrage> {
        let mut requests = Vec::new();

        for key in EigenschaftKeyId::ALL.iter().copied() {
            requests.push(CanonicalSpaltenAnfrage::Standard {
                domain: DomainId::Eigenschaften,
                unter: CanonicalStandardUnterId::Eigenschaft(CanonicalEigenschaftRequest {
                    familie: CanonicalEigenschaftsFamilie::Generisch,
                    key,
                }),
            });

            let familie = match key.standard_familie() {
                EigenschaftStandardFamilie::N => CanonicalEigenschaftsFamilie::N,
                EigenschaftStandardFamilie::EinsDurchN => CanonicalEigenschaftsFamilie::EinsDurchN,
            };

            let domain = match familie {
                CanonicalEigenschaftsFamilie::Generisch => DomainId::Eigenschaften,
                CanonicalEigenschaftsFamilie::N => DomainId::EigenschaftenN,
                CanonicalEigenschaftsFamilie::EinsDurchN => DomainId::Eigenschaften1ProN,
            };

            requests.push(CanonicalSpaltenAnfrage::Standard {
                domain,
                unter: CanonicalStandardUnterId::Eigenschaft(CanonicalEigenschaftRequest { familie, key }),
            });
        }

        requests
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