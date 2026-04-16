use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt;

use crate::domain::model::spalten_anfrage::{ColumnTarget, SpaltenAnfrage as CanonicalSpaltenAnfrage};
use crate::domain::python_source_of_truth::{
    self, combination_seed_pairs, generated_seed_pairs, is_strict_generated_pair,
    multiplication_seed_pairs, source_generated_inference_for_pair, PY_DECLS,
};
use crate::domain::request_bridge::bridge_cli_selection;
use crate::domain::resolver::request_resolver::resolve_request;
use crate::domain::spalten_anfrage::SpaltenAnfrage;

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
        ColumnTarget::DirectColumn(id) => vec![u32::from(*id)],
        ColumnTarget::DirectColumns(ids) => ids.iter().map(|id| u32::from(*id)).collect(),
        ColumnTarget::Pair(left, right) => vec![u32::from(*left), u32::from(*right)],
        ColumnTarget::Generator(_) | ColumnTarget::Combination(_) => Vec::new(),
    }
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
        let mut out = Vec::new();
        for (ober, unter) in self.alle_paare_fuer_cli_alles() {
            if let Some(request) = bridge_cli_selection(&ober, &unter) {
                out.push(request);
            }
        }
        out.sort_by_key(|req| req.to_cli_pair());
        out.dedup();
        out
    }

    pub fn alle_paare_fuer_cli_alles(&self) -> Vec<(String, String)> {
        let mut paare: BTreeSet<(String, String)> = self.alle_paare().into_iter().collect();

        for (ober, unter) in generated_seed_pairs() {
            paare.insert((ober, unter));
        }
        for (ober, unter) in combination_seed_pairs() {
            paare.insert((ober, unter));
        }
        for (ober, unter) in multiplication_seed_pairs() {
            paare.insert((ober, unter));
        }

        paare.into_iter().collect()
    }

    pub fn finde_spaltennummern_fuer_canonical_request(
        &self,
        request: &CanonicalSpaltenAnfrage,
    ) -> Vec<u32> {
        resolve_request(request.clone())
            .map(|spec| canonical_target_to_columns(&spec.target))
            .unwrap_or_default()
    }

    pub fn infer_generated_canonical_request(
        &self,
        request: &CanonicalSpaltenAnfrage,
    ) -> Option<GeneratedInference> {
        resolve_request(request.clone())
            .map(|spec| match spec.target {
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
            })
            .filter(|inf| {
                !inf.generated_befehle.is_empty()
                    || !inf.required_columns.is_empty()
                    || !inf.direct_columns.is_empty()
            })
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
        let mut direct_columns = self.finde_spaltennummern_fuer_kategorien(ober, unter);
        direct_columns.sort_unstable();
        direct_columns.dedup();

        let mut source = source_generated_inference_for_pair(ober, unter).unwrap_or_default();
        if !is_strict_generated_pair(ober, unter) {
            source.direct_columns.extend(direct_columns.iter().copied());
            source.required_columns.extend(direct_columns.iter().copied());
            source.direct_columns.sort_unstable();
            source.direct_columns.dedup();
            source.required_columns.sort_unstable();
            source.required_columns.dedup();
        }

        if source.generated_befehle.is_empty() && source.direct_columns.is_empty() {
            None
        } else {
            Some(source)
        }
    }

    pub fn finde_spaltennummern_exakt(&self, ober: &str, unter: &str) -> Vec<u32> {
        python_source_of_truth::exact_all_direct_columns_for_pair(ober, unter)
            .into_iter()
            .map(|n| n + 1)
            .collect()
    }

    pub fn finde_spaltennummern_fuer_kategorien(&self, ober: &str, unter: &str) -> Vec<u32> {
        if is_strict_generated_pair(ober, unter) {
            return Vec::new();
        }
        self.finde_spaltennummern_exakt(ober, unter)
    }

    fn lade_kategorien(&mut self) {
        let mut main_to_sub: HashMap<String, HashMap<String, Vec<u32>>> = HashMap::new();

        for decl in PY_DECLS {
            let ids: Vec<u32> = decl.columns.iter().map(|&id| id + 1).collect();
            let mut inserted_any = false;

            for &main_cat in decl.main_aliases {
                for &sub_cat in decl.sub_aliases {
                    if let Ok(request) = SpaltenAnfrage::parse(main_cat, sub_cat) {
                        let (ober, unter) = request.ober_unter_cli_pair();
                        Self::insert_entry(&mut main_to_sub, &ober, &unter, ids.clone());
                        inserted_any = true;
                    }
                }
            }

            if !inserted_any {
                for &main_cat in decl.main_aliases {
                    for &sub_cat in decl.sub_aliases {
                        Self::insert_entry(&mut main_to_sub, main_cat, sub_cat, ids.clone());
                    }
                }
            }
        }

        self.hauptkategorien = Self::convert_main_to_hauptkategorien(main_to_sub);
    }

    fn convert_main_to_hauptkategorien(
        main_to_sub: HashMap<String, HashMap<String, Vec<u32>>>,
    ) -> Vec<Oberkategorie> {
        let mut oberkategorien = Vec::new();
        let mut sorted_main: Vec<(String, HashMap<String, Vec<u32>>)> = main_to_sub.into_iter().collect();
        sorted_main.sort_by(|a, b| a.0.cmp(&b.0));

        for (main_cat, sub_map) in sorted_main {
            let mut sub_entries: Vec<(String, Vec<u32>)> = sub_map.into_iter().collect();
            sub_entries.sort_by(|a, b| a.0.cmp(&b.0));

            let unterkategorien = sub_entries
                .into_iter()
                .map(|(sub_cat, ids)| Unterkategorie::new(sub_cat, ids))
                .collect();

            oberkategorien.push(Oberkategorie::new(main_cat, unterkategorien));
        }

        oberkategorien
    }

    fn insert_entry(
        main_to_sub: &mut HashMap<String, HashMap<String, Vec<u32>>>,
        main_cat: &str,
        sub_cat: &str,
        ids: Vec<u32>,
    ) {
        let sub_map = main_to_sub.entry(main_cat.to_string()).or_default();
        let entry = sub_map.entry(sub_cat.to_string()).or_default();
        entry.extend(ids);
        entry.sort_unstable();
        entry.dedup();
    }

    pub fn alle_paare(&self) -> Vec<(String, String)> {
        let mut paare: BTreeSet<(String, String)> = BTreeSet::new();
        for haupt in &self.hauptkategorien {
            for unter in &haupt.unterkategorien {
                paare.insert((haupt.name.to_string(), unter.name.to_string()));
            }
        }
        paare.into_iter().collect()
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

    pub fn alle_hauptkategorien(&self) -> Vec<String> {
        self.hauptkategorien.iter().map(|k| k.name.to_string()).collect()
    }

    pub fn alle_unterkategorien_fuer_hauptkategorie(&self, hauptkategorie: &str) -> Vec<String> {
        self.hauptkategorien
            .iter()
            .find(|haupt| haupt.name.as_str() == hauptkategorie)
            .map(|haupt| haupt.unterkategorien.iter().map(|u| u.name.to_string()).collect())
            .unwrap_or_default()
    }

    pub fn kategorien_count(&self) -> usize {
        self.hauptkategorien.len()
    }

    pub fn unterkategorien_count(&self) -> usize {
        let mut set = HashSet::<(String, String)>::new();
        for haupt in &self.hauptkategorien {
            for unter in &haupt.unterkategorien {
                set.insert((haupt.name.to_string(), unter.name.to_string()));
            }
        }
        set.len()
    }
}


pub fn lade_kategorie_map() -> KategorieMap {
    KategorieMap::new()
}
