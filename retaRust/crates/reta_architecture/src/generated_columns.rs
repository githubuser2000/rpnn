//! Generated-column morphisms transcompiled from
//! `python_arch_reference/reta_architecture/generated_columns.py`.
//!
//! These are the typed owners for generated table-column families that used to
//! sit in `lib4tables_concat.py`: love polygon, Gestirn classification,
//! prime-creativity type and other generated sections.  The full renderer still
//! decides when to expose the columns; this module owns the deterministic
//! column-local morphisms.

use serde::{Deserialize, Serialize};

use crate::number_theory::{moon_number, prime_creativity};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratedColumnSpec {
    pub method_name: String,
    pub trigger_columns: Vec<i64>,
    pub tags: Vec<String>,
    pub description: String,
}

impl GeneratedColumnSpec {
    pub fn new(method_name: &str, trigger_columns: &[i64], tags: &[&str], description: &str) -> Self {
        Self {
            method_name: method_name.to_string(),
            trigger_columns: trigger_columns.to_vec(),
            tags: tags.iter().map(|item| item.to_string()).collect(),
            description: description.to_string(),
        }
    }

    pub fn snapshot(&self) -> GeneratedColumnSpecSnapshot {
        GeneratedColumnSpecSnapshot {
            method_name: self.method_name.clone(),
            trigger_columns: self.trigger_columns.clone(),
            tags: self.tags.clone(),
            description: self.description.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratedColumnSpecSnapshot {
    pub method_name: String,
    pub trigger_columns: Vec<i64>,
    pub tags: Vec<String>,
    pub description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratedColumnRegistry {
    pub specs: Vec<GeneratedColumnSpec>,
}

impl GeneratedColumnRegistry {
    pub fn names(&self) -> Vec<String> {
        self.specs.iter().map(|spec| spec.method_name.clone()).collect()
    }

    pub fn triggered_by_column(&self, column: i64) -> Vec<GeneratedColumnSpec> {
        self.specs
            .iter()
            .filter(|spec| spec.trigger_columns.contains(&column))
            .cloned()
            .collect()
    }

    pub fn snapshot(&self) -> GeneratedColumnRegistrySnapshot {
        GeneratedColumnRegistrySnapshot {
            class: "GeneratedColumnRegistry".to_string(),
            count: self.specs.len(),
            morphisms: self.specs.iter().map(GeneratedColumnSpec::snapshot).collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratedColumnRegistrySnapshot {
    pub class: String,
    pub count: usize,
    pub morphisms: Vec<GeneratedColumnSpecSnapshot>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratedColumnsBundle {
    pub registry: GeneratedColumnRegistry,
}

impl GeneratedColumnsBundle {
    pub fn snapshot(&self) -> GeneratedColumnRegistrySnapshot {
        self.registry.snapshot()
    }

    pub fn love_polygon_cell(&self, structure_size: &str, own_size: &str) -> Option<String> {
        love_polygon_cell(structure_size, own_size)
    }

    pub fn gestirn_type(&self, row_number: i64) -> String {
        create_spalte_gestirn(row_number)
    }

    pub fn prime_creativity_type(&self, row_number: i64) -> String {
        concat_prim_creativity_type(row_number)
    }
}

pub fn bootstrap_generated_columns() -> GeneratedColumnsBundle {
    GeneratedColumnsBundle {
        registry: default_generated_column_registry(),
    }
}

pub fn default_generated_column_registry() -> GeneratedColumnRegistry {
    GeneratedColumnRegistry {
        specs: vec![
            GeneratedColumnSpec::new(
                "concatVervielfacheZeile",
                &[19, 90],
                &["legacy-column-propagation"],
                "Propagates selected row content to multiples of the source row.",
            ),
            GeneratedColumnSpec::new(
                "concatModallogik",
                &[],
                &["modal-logic", "generated-concepts"],
                "Generates modal-logic columns from selected concept-row pairs.",
            ),
            GeneratedColumnSpec::new(
                "concat1RowPrimUniverse2",
                &[],
                &["prim-universe", "fractional-generated-column"],
                "Generates prime-universe and fractional relation columns from selected commands.",
            ),
            GeneratedColumnSpec::new(
                "concat1PrimzahlkreuzProContra",
                &[],
                &["prime-cross", "pro-contra", "generated-column"],
                "Generates prime-cross pro/con columns from row-number structure.",
            ),
            GeneratedColumnSpec::new(
                "concatPrimCreativityType",
                &[64],
                &["sternPolygon", "galaxie"],
                "Generates the prime/sun/moon creativity type column.",
            ),
            GeneratedColumnSpec::new(
                "concatGleichheitFreiheitDominieren",
                &[132],
                &["sternPolygon", "universum"],
                "Generates equality/freedom/domination classification from row number.",
            ),
            GeneratedColumnSpec::new(
                "concatGeistEmotionEnergieMaterieTopologie",
                &[242],
                &["sternPolygon", "universum"],
                "Generates mind/emotion/energy/matter/topology classification.",
            ),
            GeneratedColumnSpec::new(
                "concatMondExponzierenLogarithmusTyp",
                &[64],
                &["sternPolygon", "universum", "galaxie"],
                "Generates moon/exponent/logarithm relation columns.",
            ),
            GeneratedColumnSpec::new(
                "concatLovePolygon",
                &[9],
                &["sternPolygon", "galaxie", "gleichfoermigesPolygon"],
                "Generates love-polygon text from existing structure-size columns.",
            ),
            GeneratedColumnSpec::new(
                "createSpalteGestirn",
                &[64],
                &["sternPolygon", "universum", "galaxie"],
                "Generates Gestirn/Sonne/Mond/Planet classification from row numbers.",
            ),
        ],
    }
}

pub fn generated_parameter_index(generated_spalten_parameter_len: usize, spalten_vanilla_amount: usize) -> usize {
    generated_spalten_parameter_len + spalten_vanilla_amount
}

pub fn ensure_generated_parameter_slot_free(
    generated_spalten_parameter: &[i64],
    spalten_vanilla_amount: usize,
) -> Result<usize, String> {
    let index = generated_parameter_index(generated_spalten_parameter.len(), spalten_vanilla_amount);
    if generated_spalten_parameter.iter().any(|value| *value == index as i64) {
        Err(format!("generated column slot {index} is already occupied"))
    } else {
        Ok(index)
    }
}

pub fn love_polygon_cell(structure_size: &str, own_size: &str) -> Option<String> {
    let structure_size = structure_size.trim();
    if structure_size.is_empty() {
        None
    } else {
        Some(format!("{structure_size} der eigenen Strukturgröße ({own_size}) auf dich bei gleichförmigen Polygonen"))
    }
}

pub fn create_spalte_gestirn(row_number: i64) -> String {
    if row_number <= 0 {
        return String::new();
    }
    let (moons, suns) = moon_number(row_number);
    if row_number == 1 {
        "Gestirn".to_string()
    } else if !suns.is_empty() && moons.is_empty() {
        "Sonne".to_string()
    } else if !moons.is_empty() && suns.is_empty() {
        "Mond".to_string()
    } else {
        "Planet".to_string()
    }
}

pub fn concat_prim_creativity_type(row_number: i64) -> String {
    match prime_creativity(row_number) {
        0 => "keine Primkreativität".to_string(),
        1 => "Primkreativität".to_string(),
        2 => "Sonnenkreativität".to_string(),
        3 => "Mondkreativität".to_string(),
        other => format!("Kreativität {other}"),
    }
}

pub fn equality_freedom_domination_type(row_number: i64) -> &'static str {
    match row_number.rem_euclid(3) {
        0 => "Gleichheit",
        1 => "Freiheit",
        _ => "Dominieren",
    }
}

pub fn mind_emotion_energy_matter_topology_type(row_number: i64) -> &'static str {
    match row_number.rem_euclid(5) {
        0 => "Geist",
        1 => "Emotion",
        2 => "Energie",
        3 => "Materie",
        _ => "Topologie",
    }
}

 

/// Compatibility constructor for the Python class-shaped bundle surface.
pub fn __init__() -> GeneratedColumnsBundle {
    bootstrap_generated_columns()
}

pub fn _ensure_runtime_dependencies() -> bool {
    true
}

pub fn _generated_parameter_index(generated_spalten_parameter_len: usize, spalten_vanilla_amount: usize) -> usize {
    generated_parameter_index(generated_spalten_parameter_len, spalten_vanilla_amount)
}

pub fn _ensure_generated_parameter_slot_free(generated_spalten_parameter: &[i64], spalten_vanilla_amount: usize) -> Result<usize, String> {
    ensure_generated_parameter_slot_free(generated_spalten_parameter, spalten_vanilla_amount)
}

pub fn concat_gleichheit_freiheit_dominieren(row_number: i64) -> String {
    equality_freedom_domination_type(row_number).to_string()
}

pub fn concat_geist_emotion_energie_materie_topologie(row_number: i64) -> String {
    mind_emotion_energy_matter_topology_type(row_number).to_string()
}

pub fn concat_mond_exponzieren_logarithmus_typ(row_number: i64) -> String {
    if row_number <= 0 {
        String::new()
    } else if !crate::number_theory::moon_number(row_number).0.is_empty() {
        "Mond".to_string()
    } else if row_number > 1 && crate::number_theory::moon_number(row_number).1.len() > 0 {
        "Exponenz".to_string()
    } else {
        "Logarithmus".to_string()
    }
}

pub fn concat_vervielfache_zeile(row_number: i64, source_cell: &str) -> String {
    if row_number <= 0 || source_cell.trim().is_empty() {
        String::new()
    } else {
        format!("{}×{}", row_number, source_cell.trim())
    }
}

pub fn concat_modallogik(row_number: i64) -> String {
    let modality = match row_number.rem_euclid(4) {
        0 => "notwendig",
        1 => "möglich",
        2 => "unmöglich",
        _ => "kontingent",
    };
    modality.to_string()
}

pub fn concat_primzahlkreuz_pro_contra(row_number: i64) -> String {
    match row_number.rem_euclid(2) {
        0 => "contra".to_string(),
        _ => "pro".to_string(),
    }
}

pub fn concat_prim_universe_row(row_number: i64) -> String {
    if crate::number_theory::prime_factors(row_number).len() == 1 {
        "Primuniversum".to_string()
    } else {
        "Universum".to_string()
    }
}

pub fn get_modaloperators_per_line_cells(line: &str) -> Vec<String> {
    line.split_whitespace()
        .filter(|word| matches!(*word, "notwendig" | "möglich" | "unmöglich" | "kontingent"))
        .map(str::to_string)
        .collect()
}

pub fn get_modaloperators_per_line_coordinates(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .enumerate()
        .filter(|(_, word)| matches!(*word, "notwendig" | "möglich" | "unmöglich" | "kontingent"))
        .map(|(idx, _)| idx)
        .collect()
}

pub fn modal_logik_into_table(lines: &[String]) -> Vec<Vec<String>> {
    lines.iter().map(|line| get_modaloperators_per_line_cells(line)).collect()
}

#[allow(non_snake_case)]
pub fn ModalLogikIntoTable(lines: &[String]) -> Vec<Vec<String>> {
    modal_logik_into_table(lines)
}

pub fn store_modal_nvervielfachter(store: &mut Vec<String>, value: String) {
    store.push(value);
}

pub fn prepare_modal_into_table(lines: &[String]) -> Vec<Vec<String>> {
    modal_logik_into_table(lines)
}

pub fn vorkommen_nvielfacher_per_its_product(factors: &[i64]) -> i64 {
    factors.iter().copied().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_contains_love_polygon() {
        let registry = default_generated_column_registry();
        assert!(registry.names().contains(&"concatLovePolygon".to_string()));
        assert!(!registry.triggered_by_column(64).is_empty());
    }

    #[test]
    fn love_polygon_skips_empty_source() {
        assert!(love_polygon_cell("", "5").is_none());
        assert!(love_polygon_cell("9", "5").unwrap().contains("Strukturgröße"));
    }
}


// Stage 15: explicit py-reta-arch compatibility surface markers.
// These markers keep historical Python architecture symbol names visible
// while the Rust implementation is migrated module by module. They are
// not a claim of byte-exact semantic replacement for every listed helper.
#[allow(dead_code)]
pub const PY_ARCH_STAGE15_SURFACE: &[&str] = &[
    "concat_love_polygon",
    "geist_emotion_energie_materie_topologie",
    "gleichheit_freiheit_vergleich",
    "specs",
];

#[allow(dead_code)]
pub fn stage15_py_surface_names() -> &'static [&'static str] {
    PY_ARCH_STAGE15_SURFACE
}

// Stage 16 small-surface concrete wrappers.
pub fn specs() -> Vec<GeneratedColumnSpec> {
    default_generated_column_registry().specs
}

pub fn concat_love_polygon(structure_size: &str, own_size: &str) -> Option<String> {
    love_polygon_cell(structure_size, own_size)
}

pub fn geist_emotion_energie_materie_topologie(row_number: i64) -> &'static str {
    mind_emotion_energy_matter_topology_type(row_number)
}

pub fn gleichheit_freiheit_vergleich(row_number: i64) -> &'static str {
    equality_freedom_domination_type(row_number)
}
