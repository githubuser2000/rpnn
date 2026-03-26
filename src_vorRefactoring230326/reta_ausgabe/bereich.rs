// src/cli/bereich.rs
use std::collections::{BTreeMap, BTreeSet};

use crate::domain::selection_state::{
    ColumnRequestState, EmptyContentMode, FractionInputVisibility, RowExpansionMode,
};
use crate::reta_ausgabe::OutputSyntax;

#[derive(Debug, Clone, Default)]
pub struct PypyCompatConfig {
    pub gebrochengalaxie: BTreeSet<usize>,
    pub gebrochenuniversum: BTreeSet<usize>,
    pub gebrochenemotion: BTreeSet<usize>,
    pub gebrochengroesse: BTreeSet<usize>,
    pub kombi_galaxie: BTreeSet<usize>,
    pub kombi_universum: BTreeSet<usize>,
    pub added_headers: BTreeMap<String, Vec<usize>>,
    pub fraction_input_visibility: FractionInputVisibility,
}

#[derive(Debug, Clone)]
pub struct TextBereich {
    pub von_zeile: usize,
    pub bis_zeile: usize,
    pub von_spalte: usize,
    pub bis_spalte: usize,
    pub empty_content_mode: EmptyContentMode,
    pub row_expansion_mode: RowExpansionMode,
    pub zeilen_bereiche: Vec<(usize, usize)>,
    pub spalten_bereiche: Vec<(usize, usize)>,
    pub spaltenreihenfolgeundnurdiese: Vec<usize>,
    pub breiten: Vec<usize>,
    pub output_syntax: OutputSyntax,
    pub column_request_state: ColumnRequestState,
    pub exact_generated_befehle: BTreeSet<String>,
    pub exact_modal_pairs: Vec<(usize, usize)>,
    pub exact_meta_konkret_specs: Vec<(usize, usize)>,
    pub exact_visible_columns: Vec<usize>,
    pub pypy_compat: PypyCompatConfig,
}

impl TextBereich {
    pub fn enable_empty_content_filter(&mut self) {
        self.empty_content_mode = EmptyContentMode::DropEmpty;
    }

    pub fn drops_empty_content(&self) -> bool {
        matches!(self.empty_content_mode, EmptyContentMode::DropEmpty)
    }

    pub fn enable_row_expansion_multiples(&mut self) {
        self.row_expansion_mode = self.row_expansion_mode.with_multiples();
    }

    pub fn enable_row_expansion_prime_factors(&mut self) {
        self.row_expansion_mode = self.row_expansion_mode.with_prime_factors();
    }

    pub fn expands_with_multiples(&self) -> bool {
        self.row_expansion_mode.uses_multiples()
    }

    pub fn expands_with_prime_factors(&self) -> bool {
        self.row_expansion_mode.uses_prime_factors()
    }

    pub fn mark_columns_requested(&mut self) {
        if !self.column_request_state.is_resolved() {
            self.column_request_state = ColumnRequestState::RequestedPendingResolution;
        }
    }

    pub fn mark_columns_resolved(&mut self) {
        self.column_request_state = ColumnRequestState::Resolved;
    }

    pub fn reset_column_request(&mut self) {
        self.column_request_state = ColumnRequestState::NotRequested;
    }

    pub fn columns_requested(&self) -> bool {
        self.column_request_state.is_requested()
    }

    pub fn columns_pending(&self) -> bool {
        self.column_request_state.is_pending()
    }

    pub fn columns_resolved(&self) -> bool {
        self.column_request_state.is_resolved()
    }

    pub fn hide_fraction_inputs(&mut self) {
        self.pypy_compat.fraction_input_visibility = FractionInputVisibility::HideInputs;
    }

    pub fn fraction_inputs_visible(&self) -> bool {
        self.pypy_compat.fraction_input_visibility.inputs_visible()
    }
}

impl Default for TextBereich {
    fn default() -> Self {
        Self {
            von_zeile: 0,
            bis_zeile: 0,
            von_spalte: usize::MAX,
            bis_spalte: usize::MAX,
            empty_content_mode: EmptyContentMode::default(),
            row_expansion_mode: RowExpansionMode::default(),
            zeilen_bereiche: Vec::new(),
            spalten_bereiche: Vec::new(),
            spaltenreihenfolgeundnurdiese: Vec::new(),
            breiten: Vec::new(),
            output_syntax: OutputSyntax::default(),
            column_request_state: ColumnRequestState::default(),
            exact_generated_befehle: BTreeSet::new(),
            exact_modal_pairs: Vec::new(),
            exact_meta_konkret_specs: Vec::new(),
            exact_visible_columns: Vec::new(),
            pypy_compat: PypyCompatConfig::default(),
        }
    }
}
