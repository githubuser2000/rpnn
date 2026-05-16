//! CSV-backed table materialization layer.
//!
//! Stage 20 turns the Stage-17/18 parameter matrix and the Stage-19 CSV
//! catalog into concrete Rust table sections.  The legacy renderer is still the
//! visible behaviour owner, but Rust can now answer the next architectural
//! question directly: which local CSV cells are selected by a CLI context such
//! as `-spalten --kontinuum=m`?

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::column_selection::ColumnBucketKey;
use crate::csv_catalog::{
    csv_asset_by_name, csv_asset_for_language_with_required_columns, csv_rows_by_name, CsvAssetKind,
    CsvLanguage,
};
use crate::html_class_catalog::html_class_record;
use crate::parameter_runtime::{bootstrap_parameter_runtime, ParameterCommandSets};
use crate::table_generation::TableGenerationPlan;
use crate::tag_schema::ordinary_tags_for_column;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableMaterializationConfig {
    /// Preferred language for local CSV sections.  `Base` keeps historical
    /// German/base assets such as `religion.csv`.
    pub language: CsvLanguage,
    /// Always include the header/source row 0 when a row projection is built.
    pub include_header: bool,
    /// Limit preview/materialized rows.  `None` means all selected rows.
    pub max_rows: Option<usize>,
    /// Limit selected columns in preview/materialized sections.  `None` means
    /// all selected columns.
    pub max_columns: Option<usize>,
    /// Carry `--spaltenreihenfolgeundnurdiese` through to the Rust materialized
    /// view.  This is enabled by default but only has an effect when the CLI
    /// actually provided an override.
    pub honor_column_order_override: bool,
    /// Carry explicit row-selector order from `--vorhervonausschnitt`/`--zaehlung`
    /// through materialization.  Membership still lives in a set, but rendering
    /// should use this order when present.
    pub honor_row_order_override: bool,
    /// Prefer a localized CSV section only if it can satisfy the direct
    /// zero-based columns requested by the parameter matrix.  This guards the
    /// Stage-55 religion.csv update: base religion.csv now owns column 744,
    /// while language variants can still lag at 744 columns and should not
    /// turn `--kontinuum=m` back into a virtual-column case.
    pub fallback_to_base_for_missing_language_columns: bool,
}

impl Default for TableMaterializationConfig {
    fn default() -> Self {
        Self {
            language: CsvLanguage::Base,
            include_header: true,
            max_rows: None,
            max_columns: None,
            honor_column_order_override: true,
            honor_row_order_override: true,
            fallback_to_base_for_missing_language_columns: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CsvProjectionRequest {
    pub asset_name: String,
    pub rows_zero_based: BTreeSet<usize>,
    /// Optional ordered row projection.  Header row insertion is still handled
    /// by `include_header`, but data rows follow this vector when present.
    pub row_order_zero_based: Vec<usize>,
    pub columns_legacy: BTreeSet<usize>,
    /// Optional ordered projection.  When present, this preserves the explicit
    /// legacy output order from `--spaltenreihenfolgeundnurdiese` instead of the
    /// sorted `BTreeSet` column order.
    pub column_order_legacy: Vec<usize>,
    pub include_header: bool,
    pub max_rows: Option<usize>,
    pub max_columns: Option<usize>,
}

impl CsvProjectionRequest {
    pub fn for_asset(asset_name: impl Into<String>) -> Self {
        Self {
            asset_name: asset_name.into(),
            rows_zero_based: BTreeSet::new(),
            row_order_zero_based: Vec::new(),
            columns_legacy: BTreeSet::new(),
            column_order_legacy: Vec::new(),
            include_header: true,
            max_rows: None,
            max_columns: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaterializedCsvCell {
    pub source_row_zero_based: usize,
    pub source_column_index: usize,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaterializedCsvRow {
    pub source_row_zero_based: usize,
    pub cells: Vec<MaterializedCsvCell>,
}

impl MaterializedCsvRow {
    pub fn values(&self) -> Vec<String> {
        self.cells.iter().map(|cell| cell.value.clone()).collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaterializedCsvSection {
    pub asset_name: String,
    pub asset_kind: String,
    pub language: String,
    pub source_row_count: usize,
    pub source_max_columns: usize,
    pub selected_rows_zero_based: Vec<usize>,
    pub selected_columns_legacy: Vec<usize>,
    pub missing_rows_zero_based: Vec<usize>,
    pub missing_columns_legacy: Vec<usize>,
    pub rows: Vec<MaterializedCsvRow>,
    pub universal_property: String,
}

impl MaterializedCsvSection {
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn cell_count(&self) -> usize {
        self.rows.iter().map(|row| row.cells.len()).sum()
    }

    pub fn rendered_rows(&self) -> Vec<Vec<String>> {
        self.rows.iter().map(MaterializedCsvRow::values).collect()
    }

    pub fn column_headers(&self) -> Vec<String> {
        self.rows
            .iter()
            .find(|row| row.source_row_zero_based == 0)
            .map(MaterializedCsvRow::values)
            .unwrap_or_default()
    }

    pub fn cell_by_source_coordinates(
        &self,
        source_row_zero_based: usize,
        source_column_index: usize,
    ) -> Option<&str> {
        self.rows
            .iter()
            .find(|row| row.source_row_zero_based == source_row_zero_based)?
            .cells
            .iter()
            .find(|cell| cell.source_column_index == source_column_index)
            .map(|cell| cell.value.as_str())
    }

    pub fn contains_text(&self, needle: &str) -> bool {
        self.rows
            .iter()
            .any(|row| row.cells.iter().any(|cell| cell.value.contains(needle)))
    }

    pub fn signature(&self) -> String {
        format!(
            "{}:{}x{}:rows={:?}:cols={:?}",
            self.asset_name,
            self.row_count(),
            self.cell_count(),
            self.selected_rows_zero_based,
            self.selected_columns_legacy
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SymbolicBucketMaterialization {
    pub bucket: ColumnBucketKey,
    pub symbols: Vec<String>,
    pub asset_names: Vec<String>,
    pub numeric_selectors: Vec<usize>,
    pub sections: Vec<MaterializedCsvSection>,
    pub unresolved_symbols: Vec<String>,
}

impl SymbolicBucketMaterialization {
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct VirtualColumnMaterialization {
    pub column_legacy: usize,
    pub asset_name: String,
    pub source_max_columns: usize,
    pub reason: String,
    pub tag_names: Vec<String>,
    pub html_class_text: Option<String>,
    pub html_class_row_number: Option<i64>,
    pub predecessor_source_column_index: Option<usize>,
    pub predecessor_header: Option<String>,
    pub universal_property: String,
}

impl VirtualColumnMaterialization {
    pub fn is_column(&self, column: usize) -> bool {
        self.column_legacy == column
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableMaterializationReport {
    pub class: String,
    pub selected_column_count: usize,
    pub selected_row_count: usize,
    pub requested_row_order_zero_based: Vec<usize>,
    pub materialized_row_order_zero_based: Vec<usize>,
    pub row_order_override_applied: bool,
    pub requested_column_order_legacy: Vec<usize>,
    pub materialized_column_order_legacy: Vec<usize>,
    pub column_order_override_applied: bool,
    pub required_csv_assets: Vec<String>,
    pub ordinary_sections: Vec<MaterializedCsvSection>,
    pub symbolic_sections: Vec<SymbolicBucketMaterialization>,
    pub virtual_columns: Vec<VirtualColumnMaterialization>,
    pub missing_assets: Vec<String>,
    pub materialized_row_count: usize,
    pub materialized_cell_count: usize,
    pub virtual_column_count: usize,
    pub continuum_m_columns_present: bool,
    pub continuum_m_virtual_column_present: bool,
    pub continuum_m_missing_columns: Vec<usize>,
    pub continuum_m_header_preview: Vec<String>,
    pub continuum_m_first_data_preview: Vec<String>,
    pub universal_property: String,
}

impl TableMaterializationReport {
    pub fn section_count(&self) -> usize {
        self.ordinary_sections.len()
            + self
                .symbolic_sections
                .iter()
                .map(SymbolicBucketMaterialization::section_count)
                .sum::<usize>()
    }

    pub fn is_empty(&self) -> bool {
        self.section_count() == 0
    }

    pub fn signatures(&self) -> Vec<String> {
        let mut out = Vec::new();
        for section in &self.ordinary_sections {
            out.push(section.signature());
        }
        for symbolic in &self.symbolic_sections {
            for section in &symbolic.sections {
                out.push(section.signature());
            }
        }
        out
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableMaterializationSnapshot {
    pub class: String,
    pub morphisms: Vec<String>,
    pub csv_asset_count: usize,
    pub ordinary_asset: String,
    pub universal_property: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TableMaterializationBundle {
    pub morphisms: Vec<String>,
    pub universal_property: String,
}

impl TableMaterializationBundle {
    pub fn materialize_csv_projection(
        &self,
        request: CsvProjectionRequest,
    ) -> Option<MaterializedCsvSection> {
        materialize_csv_projection(request)
    }

    pub fn materialize_plan(
        &self,
        plan: &TableGenerationPlan,
        config: &TableMaterializationConfig,
    ) -> TableMaterializationReport {
        materialize_generation_plan(plan, config)
    }

    pub fn materialize_command_sets(
        &self,
        sets: &ParameterCommandSets,
        config: &TableMaterializationConfig,
    ) -> TableMaterializationReport {
        let plan = TableGenerationPlan::from_parameter_command_sets(sets);
        self.materialize_plan(&plan, config)
    }

    pub fn materialize_cli_args<S: AsRef<str>>(
        &self,
        args: &[S],
        config: &TableMaterializationConfig,
    ) -> TableMaterializationReport {
        let parsed = bootstrap_parameter_runtime().parse_cli_args(args);
        self.materialize_command_sets(&parsed.command_sets, config)
    }

    pub fn snapshot(&self) -> TableMaterializationSnapshot {
        TableMaterializationSnapshot {
            class: "TableMaterializationBundle".to_string(),
            morphisms: self.morphisms.clone(),
            csv_asset_count: crate::csv_catalog::csv_asset_count(),
            ordinary_asset: "religion.csv".to_string(),
            universal_property: self.universal_property.clone(),
        }
    }
}

pub fn bootstrap_table_materialization() -> TableMaterializationBundle {
    TableMaterializationBundle {
        morphisms: vec![
            "materialize_csv_projection".to_string(),
            "materialize_generation_plan".to_string(),
            "materialize_ordinary_religion_section".to_string(),
            "materialize_symbolic_bucket_sections".to_string(),
            "materialize_virtual_columns".to_string(),
            "ordered_columns_for_projection".to_string(),
            "ordered_rows_for_projection".to_string(),
            "column_order_override".to_string(),
            "row_order_override".to_string(),
            "cell_by_source_coordinates".to_string(),
            "column_headers".to_string(),
        ],
        universal_property:
            "selected parameter/bucket sections glue to deterministic CSV-backed table sections"
                .to_string(),
    }
}

pub fn materialize_generation_plan(
    plan: &TableGenerationPlan,
    config: &TableMaterializationConfig,
) -> TableMaterializationReport {
    let mut missing_assets = Vec::new();
    let mut ordinary_sections = Vec::new();

    if !plan.selected_columns.is_empty() {
        let requested_columns = plan
            .selected_columns
            .iter()
            .filter_map(|column| usize::try_from(*column).ok())
            .filter(|column| *column > 0)
            .collect::<BTreeSet<_>>();
        let ordered_requested_columns = if config.honor_column_order_override {
            plan.ordered_selected_columns()
                .into_iter()
                .filter_map(|column| usize::try_from(column).ok())
                .filter(|column| *column > 0)
                .collect::<Vec<_>>()
        } else {
            requested_columns.iter().copied().collect::<Vec<_>>()
        };
        let ordinary_asset_name = asset_name_for_language_with_columns(
            "religion.csv",
            config.language,
            &ordered_requested_columns,
            config.fallback_to_base_for_missing_language_columns,
        );
        let mut request = CsvProjectionRequest::for_asset(ordinary_asset_name.clone());
        request.include_header = config.include_header;
        request.max_rows = config.max_rows;
        request.max_columns = config.max_columns;
        request.rows_zero_based = plan_rows_to_source_indices(&plan.selected_rows);
        if config.honor_row_order_override {
            request.row_order_zero_based = plan_rows_to_source_order(&plan.ordered_selected_rows());
        }
        request.columns_legacy = requested_columns;
        if config.honor_column_order_override {
            request.column_order_legacy = ordered_requested_columns;
        }
        match materialize_csv_projection(request) {
            Some(section) => ordinary_sections.push(section),
            None => missing_assets.push(ordinary_asset_name),
        }
    }

    let symbolic_sections = materialize_symbolic_bucket_sections(
        &plan.symbolic_column_buckets,
        &plan.selected_rows,
        config,
        &mut missing_assets,
    );
    let virtual_columns = materialize_virtual_columns(&ordinary_sections);

    let materialized_row_count = ordinary_sections
        .iter()
        .map(MaterializedCsvSection::row_count)
        .sum::<usize>()
        + symbolic_sections
            .iter()
            .flat_map(|section| section.sections.iter())
            .map(MaterializedCsvSection::row_count)
            .sum::<usize>();
    let materialized_cell_count = ordinary_sections
        .iter()
        .map(MaterializedCsvSection::cell_count)
        .sum::<usize>()
        + symbolic_sections
            .iter()
            .flat_map(|section| section.sections.iter())
            .map(MaterializedCsvSection::cell_count)
            .sum::<usize>();

    let continuum_m_columns_present = ordinary_sections.iter().any(|section| {
        section.selected_columns_legacy.contains(&493)
            && section.selected_columns_legacy.contains(&744)
    });
    let continuum_m_missing_columns = ordinary_sections
        .iter()
        .find(|section| section.selected_columns_legacy.contains(&744))
        .map(|section| section.missing_columns_legacy.clone())
        .unwrap_or_default();
    let continuum_m_header_preview = ordinary_sections
        .iter()
        .find(|section| section.selected_columns_legacy.contains(&744))
        .map(MaterializedCsvSection::column_headers)
        .unwrap_or_default();
    let continuum_m_first_data_preview = ordinary_sections
        .iter()
        .find(|section| section.selected_columns_legacy.contains(&744))
        .and_then(|section| {
            section
                .rows
                .iter()
                .find(|row| row.source_row_zero_based != 0)
                .map(MaterializedCsvRow::values)
        })
        .unwrap_or_default();
    let continuum_m_virtual_column_present =
        virtual_columns.iter().any(|column| column.is_column(744));
    let requested_row_order_zero_based = if config.honor_row_order_override {
        plan_rows_to_source_order(&plan.ordered_selected_rows())
    } else {
        plan_rows_to_source_indices(&plan.selected_rows)
            .into_iter()
            .collect::<Vec<_>>()
    };
    let materialized_row_order_zero_based = ordinary_sections
        .first()
        .map(|section| section.selected_rows_zero_based.clone())
        .unwrap_or_default();

    let requested_column_order_legacy = if config.honor_column_order_override {
        plan.ordered_selected_columns()
            .into_iter()
            .filter_map(|column| usize::try_from(column).ok())
            .filter(|column| *column > 0)
            .collect::<Vec<_>>()
    } else {
        plan.selected_columns
            .iter()
            .filter_map(|column| usize::try_from(*column).ok())
            .filter(|column| *column > 0)
            .collect::<Vec<_>>()
    };
    let materialized_column_order_legacy = ordinary_sections
        .first()
        .map(|section| section.selected_columns_legacy.clone())
        .unwrap_or_default();

    TableMaterializationReport {
        class: "TableMaterializationReport".to_string(),
        selected_column_count: plan.selected_columns.len(),
        selected_row_count: plan.selected_rows.len(),
        requested_row_order_zero_based,
        materialized_row_order_zero_based,
        row_order_override_applied: config.honor_row_order_override
            && plan.row_order_override_applies(),
        requested_column_order_legacy,
        materialized_column_order_legacy,
        column_order_override_applied: config.honor_column_order_override
            && plan.column_order_override_applies(),
        required_csv_assets: plan.csv_asset_names.clone(),
        ordinary_sections,
        symbolic_sections,
        virtual_column_count: virtual_columns.len(),
        virtual_columns,
        missing_assets,
        materialized_row_count,
        materialized_cell_count,
        continuum_m_columns_present,
        continuum_m_virtual_column_present,
        continuum_m_missing_columns,
        continuum_m_header_preview,
        continuum_m_first_data_preview,
        universal_property:
            "same CLI parameter projections select the same local CSV cells before rendering"
                .to_string(),
    }
}

pub fn materialize_virtual_columns(
    ordinary_sections: &[MaterializedCsvSection],
) -> Vec<VirtualColumnMaterialization> {
    let mut out = Vec::new();
    for section in ordinary_sections {
        let source_rows = csv_rows_by_name(&section.asset_name).unwrap_or_default();
        let header = source_rows.get(0);
        for column in &section.missing_columns_legacy {
            let tag_names = ordinary_tags_for_column(*column as i64)
                .unwrap_or_default()
                .into_iter()
                .map(|tag| tag.py_name().to_string())
                .collect::<Vec<_>>();
            let html_record = html_class_record(*column as i64, None)
                .or_else(|| html_class_record(*column as i64, Some(0)));
            let predecessor_source_column_index = if section.source_max_columns == 0 {
                None
            } else {
                Some(
                    (*column)
                        .saturating_sub(1)
                        .min(section.source_max_columns - 1),
                )
            };
            let predecessor_header = predecessor_source_column_index
                .and_then(|index| header.and_then(|row| row.get(index)).cloned());
            let reason = if *column >= section.source_max_columns {
                "selected_column_is_not_a_direct_csv_source_column".to_string()
            } else {
                "selected_column_was_missing_in_at_least_one_projected_row".to_string()
            };
            out.push(VirtualColumnMaterialization {
                column_legacy: *column,
                asset_name: section.asset_name.clone(),
                source_max_columns: section.source_max_columns,
                reason,
                tag_names,
                html_class_text: html_record.map(|record| record.text.to_string()),
                html_class_row_number: html_record.and_then(|record| record.row_number),
                predecessor_source_column_index,
                predecessor_header,
                universal_property: "non-csv local column keeps tag/html witnesses until a concrete renderer morphism owns it".to_string(),
            });
        }
    }
    out
}

pub fn materialize_csv_projection(request: CsvProjectionRequest) -> Option<MaterializedCsvSection> {
    let asset = csv_asset_by_name(&request.asset_name)?;
    let source_rows = csv_rows_by_name(&request.asset_name)?;
    let selected_rows = ordered_rows_for_projection(&request, !source_rows.is_empty());

    let selected_columns = ordered_columns_for_projection(&request);

    let mut missing_rows = Vec::new();
    let mut missing_columns = BTreeSet::new();
    let mut rows = Vec::new();

    for row_index in &selected_rows {
        let Some(source_row) = source_rows.get(*row_index) else {
            missing_rows.push(*row_index);
            continue;
        };
        let mut cells = Vec::new();
        for column_legacy in &selected_columns {
            match source_row.get(*column_legacy) {
                Some(value) => cells.push(MaterializedCsvCell {
                    source_row_zero_based: *row_index,
                    source_column_index: *column_legacy,
                    value: value.clone(),
                }),
                None => {
                    missing_columns.insert(*column_legacy);
                }
            }
        }
        rows.push(MaterializedCsvRow {
            source_row_zero_based: *row_index,
            cells,
        });
    }

    Some(MaterializedCsvSection {
        asset_name: asset.name.to_string(),
        asset_kind: asset.kind.canonical().to_string(),
        language: asset.language.canonical().to_string(),
        source_row_count: asset.row_count,
        source_max_columns: asset.max_columns,
        selected_rows_zero_based: selected_rows,
        selected_columns_legacy: selected_columns,
        missing_rows_zero_based: missing_rows,
        missing_columns_legacy: missing_columns.into_iter().collect(),
        rows,
        universal_property: "projection(row,column) is independent of traversal order".to_string(),
    })
}

pub fn materialize_symbolic_bucket_sections(
    symbolic_column_buckets: &BTreeMap<ColumnBucketKey, BTreeSet<String>>,
    selected_rows: &BTreeSet<i64>,
    config: &TableMaterializationConfig,
    missing_assets: &mut Vec<String>,
) -> Vec<SymbolicBucketMaterialization> {
    let mut out = Vec::new();
    for (bucket, symbols) in symbolic_column_buckets {
        if symbols.is_empty() {
            continue;
        }
        let asset_names = asset_names_for_symbolic_bucket(*bucket, config.language);
        let numeric_selectors = numeric_selectors_from_symbols(symbols);
        let mut sections = Vec::new();
        for asset_name in &asset_names {
            let mut request = CsvProjectionRequest::for_asset(asset_name.clone());
            request.include_header = config.include_header;
            request.max_rows = config.max_rows;
            request.max_columns = config.max_columns;
            request.rows_zero_based = plan_rows_to_source_indices(selected_rows);
            if config.honor_row_order_override {
                request.row_order_zero_based = plan_rows_to_source_order_from_set(selected_rows);
            }
            request.columns_legacy = numeric_selectors.iter().copied().collect();
            request.column_order_legacy = numeric_selectors.clone();
            match materialize_csv_projection(request) {
                Some(section) => sections.push(section),
                None => missing_assets.push(asset_name.clone()),
            }
        }
        let unresolved_symbols = symbols
            .iter()
            .filter(|symbol| symbol.parse::<usize>().is_err())
            .cloned()
            .collect();
        out.push(SymbolicBucketMaterialization {
            bucket: *bucket,
            symbols: symbols.iter().cloned().collect(),
            asset_names,
            numeric_selectors,
            sections,
            unresolved_symbols,
        });
    }
    out
}

pub fn asset_names_for_symbolic_bucket(
    bucket: ColumnBucketKey,
    language: CsvLanguage,
) -> Vec<String> {
    let base = match bucket.bucket {
        2 | 7 => vec!["primenumbers.csv"],
        3 => vec!["kombi.csv"],
        5 => vec!["gebrochen-rational-universum.csv"],
        6 => vec!["gebrochen-rational-galaxie.csv"],
        8 => vec!["kombi-meta.csv"],
        9 => vec!["gebrochen-rational-emotionen.csv"],
        10 => vec!["gebrochen-rational-strukturgroesse.csv"],
        _ => Vec::new(),
    };
    base.into_iter()
        .map(|name| asset_name_for_language(name, language))
        .filter(|name| csv_asset_by_name(name).is_some())
        .collect()
}

pub fn asset_name_for_language(base_name: &str, language: CsvLanguage) -> String {
    if language == CsvLanguage::Base {
        return base_name.to_string();
    }
    let prefix = language.canonical();
    let candidate = format!("{prefix}-{base_name}");
    if csv_asset_by_name(&candidate).is_some() {
        candidate
    } else {
        base_name.to_string()
    }
}

pub fn asset_name_for_language_with_columns(
    base_name: &str,
    language: CsvLanguage,
    columns_zero_based: &[usize],
    fallback_to_base_for_missing_columns: bool,
) -> String {
    if fallback_to_base_for_missing_columns {
        if let Some(asset) = csv_asset_for_language_with_required_columns(
            base_name,
            language,
            columns_zero_based,
        ) {
            return asset.name.to_string();
        }
    }
    asset_name_for_language(base_name, language)
}

pub fn ordered_rows_for_projection(
    request: &CsvProjectionRequest,
    source_has_rows: bool,
) -> Vec<usize> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    if request.include_header && source_has_rows && seen.insert(0) {
        out.push(0);
    }
    let source = if request.row_order_zero_based.is_empty() {
        request.rows_zero_based.iter().copied().collect::<Vec<_>>()
    } else {
        request.row_order_zero_based.clone()
    };
    for row in source {
        if seen.insert(row) {
            out.push(row);
        }
    }
    if out.is_empty() && source_has_rows {
        out.push(0);
    }
    if let Some(limit) = request.max_rows {
        out.truncate(limit);
    }
    out
}

pub fn ordered_columns_for_projection(request: &CsvProjectionRequest) -> Vec<usize> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    let source = if request.column_order_legacy.is_empty() {
        request.columns_legacy.iter().copied().collect::<Vec<_>>()
    } else {
        request.column_order_legacy.clone()
    };
    for column in source {
        if seen.insert(column) {
            out.push(column);
        }
    }
    if out.is_empty() {
        // A symbolic selector without a concrete column still gets a stable
        // preview of the local section.  Keeping the first source column avoids
        // materializing huge tables while preserving a data witness.
        out.push(0);
    }
    if let Some(limit) = request.max_columns {
        out.truncate(limit);
    }
    out
}

pub fn plan_rows_to_source_indices(rows: &BTreeSet<i64>) -> BTreeSet<usize> {
    rows.iter()
        .filter_map(|row| usize::try_from(*row).ok())
        .collect()
}

pub fn plan_rows_to_source_order(rows: &[i64]) -> Vec<usize> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for row in rows {
        if let Ok(value) = usize::try_from(*row) {
            if seen.insert(value) {
                out.push(value);
            }
        }
    }
    out
}

pub fn plan_rows_to_source_order_from_set(rows: &BTreeSet<i64>) -> Vec<usize> {
    rows.iter()
        .filter_map(|row| usize::try_from(*row).ok())
        .collect()
}

pub fn numeric_selectors_from_symbols(symbols: &BTreeSet<String>) -> Vec<usize> {
    symbols
        .iter()
        .filter_map(|symbol| symbol.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .collect()
}

pub fn materialize_cli_args<S: AsRef<str>>(
    args: &[S],
    config: &TableMaterializationConfig,
) -> TableMaterializationReport {
    bootstrap_table_materialization().materialize_cli_args(args, config)
}

pub fn materialize_kontinuum_m_smoke() -> TableMaterializationReport {
    let args = [
        "reta",
        "-zeilen",
        "--vorhervonausschnitt=1-1",
        "-spalten",
        "--kontinuum=m",
        "--breite=0",
    ];
    materialize_cli_args(&args, &TableMaterializationConfig::default())
}

pub fn csv_kind_for_asset(name: &str) -> Option<CsvAssetKind> {
    csv_asset_by_name(name).map(|asset| asset.kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continuum_m_materializes_493_and_direct_744_after_religion_csv_update() {
        let report = materialize_kontinuum_m_smoke();
        assert!(report.continuum_m_columns_present);
        assert!(!report.continuum_m_missing_columns.contains(&744));
        assert!(!report.continuum_m_virtual_column_present);
        assert!(report.virtual_columns.iter().all(|column| column.column_legacy != 744));
        assert!(report
            .continuum_m_header_preview
            .iter()
            .any(|cell| cell.contains("M Kontinuum")));
        assert!(report
            .continuum_m_header_preview
            .iter()
            .any(|cell| cell.contains("Neues M")));
        assert!(report
            .continuum_m_first_data_preview
            .iter()
            .any(|cell| cell.contains("Wege-Gabelung")));
        assert!(report
            .continuum_m_first_data_preview
            .iter()
            .any(|cell| cell.contains("Identität")));
        assert_eq!(
            report.ordinary_sections[0].selected_columns_legacy,
            vec![493, 744]
        );
    }

    #[test]
    fn vorhervonausschnitt_preserves_requested_row_order() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=3,1-2",
            "-spalten",
            "--religion=493",
        ];
        let report = materialize_cli_args(&args, &TableMaterializationConfig::default());
        assert!(report.row_order_override_applied);
        assert_eq!(report.requested_row_order_zero_based, vec![3, 1, 2]);
        assert_eq!(report.materialized_row_order_zero_based, vec![0, 3, 1, 2]);
    }

    #[test]
    fn numeric_spalten_range_materializes_only_header_when_no_zeilen_selected() {
        let args = ["reta", "-spalten", "--religion=493"];
        let report = materialize_cli_args(&args, &TableMaterializationConfig::default());
        assert_eq!(report.requested_row_order_zero_based, Vec::<usize>::new());
        assert_eq!(report.materialized_row_order_zero_based, vec![0]);
    }

    #[test]
    fn spaltenreihenfolgeundnurdiese_preserves_requested_materialization_order() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
            "-ausgabe",
            "--spaltenreihenfolgeundnurdiese=744,493",
        ];
        let report = materialize_cli_args(&args, &TableMaterializationConfig::default());
        assert!(report.column_order_override_applied);
        assert_eq!(report.requested_column_order_legacy, vec![744, 493]);
        assert_eq!(report.materialized_column_order_legacy, vec![744, 493]);
        assert!(!report.continuum_m_missing_columns.contains(&744));
    }

    #[test]
    fn symbolic_bucket_materializes_fraction_csv() {
        let args = ["reta", "-spalten", "--gebrochenuniversum=2"];
        let report = materialize_cli_args(&args, &TableMaterializationConfig::default());
        assert!(report.symbolic_sections.iter().any(|section| {
            section
                .asset_names
                .iter()
                .any(|name| name == "gebrochen-rational-universum.csv")
        }));
        assert!(report.materialized_cell_count > 0);
    }

    #[test]
    fn language_asset_resolution_falls_back_to_base() {
        assert_eq!(
            asset_name_for_language("religion.csv", CsvLanguage::English),
            "en-religion.csv"
        );
        assert_eq!(
            asset_name_for_language(
                "2024-07-06-symbols-alt-ak-circle-sphere-etc.csv",
                CsvLanguage::English
            ),
            "2024-07-06-symbols-alt-ak-circle-sphere-etc.csv"
        );
    }


    #[test]
    fn language_materialization_falls_back_to_base_for_direct_744() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--kontinuum=m",
        ];
        let report = materialize_cli_args(
            &args,
            &TableMaterializationConfig {
                language: CsvLanguage::English,
                ..TableMaterializationConfig::default()
            },
        );
        let section = report.ordinary_sections.first().unwrap();
        assert_eq!(section.asset_name, "religion.csv");
        assert_eq!(section.language, "base");
        assert!(section.selected_columns_legacy.contains(&744));
        assert!(!section.missing_columns_legacy.contains(&744));
        assert!(section
            .column_headers()
            .iter()
            .any(|cell| cell.contains("Neues M")));
    }

    #[test]
    fn language_materialization_keeps_variant_when_direct_columns_exist() {
        let args = [
            "reta",
            "-zeilen",
            "--vorhervonausschnitt=1-1",
            "-spalten",
            "--religion=493",
        ];
        let report = materialize_cli_args(
            &args,
            &TableMaterializationConfig {
                language: CsvLanguage::English,
                ..TableMaterializationConfig::default()
            },
        );
        let section = report.ordinary_sections.first().unwrap();
        assert_eq!(section.asset_name, "en-religion.csv");
        assert_eq!(section.language, "en");
        assert!(section.selected_columns_legacy.contains(&493));
    }
}
