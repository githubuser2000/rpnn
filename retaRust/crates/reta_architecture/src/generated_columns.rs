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
