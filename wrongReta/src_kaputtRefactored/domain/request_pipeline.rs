use std::collections::BTreeSet;

use crate::cli::TextBereich;
use crate::domain::categories::KategorieMap;
use crate::domain::errors::RequestPipelineError;
use crate::domain::spalten_anfrage::SpaltenAnfrage;

#[derive(Debug, Clone)]
pub struct RawSelectionRequest {
    pub ober: String,
    pub unter: String,
}

#[derive(Debug, Clone)]
pub struct ParsedSelectionRequest {
    pub ober: String,
    pub unter: String,
    pub request: SpaltenAnfrage,
}

#[derive(Debug, Clone)]
pub struct ExpandedSelectionRequest {
    pub ober: String,
    pub unter: String,
    pub request: SpaltenAnfrage,
    pub generated_befehle: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedSelectionRequest {
    pub ober: String,
    pub unter: String,
    pub request: SpaltenAnfrage,
    pub generated_befehle: BTreeSet<String>,
    pub direct_columns: Vec<u32>,
    pub required_columns: Vec<u32>,
}

pub fn parse_many<I>(pairs: I) -> Result<Vec<ParsedSelectionRequest>, RequestPipelineError>
where
    I: IntoIterator<Item = RawSelectionRequest>,
{
    pairs.into_iter().map(RawSelectionRequest::parse).collect()
}

impl RawSelectionRequest {
    pub fn new(ober: impl Into<String>, unter: impl Into<String>) -> Self {
        Self { ober: ober.into(), unter: unter.into() }
    }

    pub fn parse(self) -> Result<ParsedSelectionRequest, RequestPipelineError> {
        let request = SpaltenAnfrage::parse(&self.ober, &self.unter)
            .map_err(RequestPipelineError::ParseSpaltenAnfrage)?;
        Ok(ParsedSelectionRequest { ober: self.ober, unter: self.unter, request })
    }
}

impl ParsedSelectionRequest {
    pub fn expand(self, kategorie_map: &KategorieMap) -> ExpandedSelectionRequest {
        let mut generated_befehle = BTreeSet::new();
        if let Some(inference) = kategorie_map.infer_generated_request(&self.request) {
            generated_befehle.extend(inference.generated_befehle);
        }
        ExpandedSelectionRequest { ober: self.ober, unter: self.unter, request: self.request, generated_befehle }
    }
}

impl ExpandedSelectionRequest {
    pub fn resolve(self, kategorie_map: &KategorieMap) -> Result<ResolvedSelectionRequest, RequestPipelineError> {
        let direct_columns = kategorie_map.finde_spaltennummern_fuer_request(&self.request);
        let required_columns = kategorie_map
            .infer_generated_request(&self.request)
            .map(|g| g.required_columns)
            .unwrap_or_default();

        if direct_columns.is_empty() && required_columns.is_empty() && self.generated_befehle.is_empty() {
            return Err(RequestPipelineError::NoColumnsForRequest {
                ober: self.ober,
                unter: self.unter,
            });
        }

        Ok(ResolvedSelectionRequest {
            ober: self.ober,
            unter: self.unter,
            request: self.request,
            generated_befehle: self.generated_befehle,
            direct_columns,
            required_columns,
        })
    }
}

impl ResolvedSelectionRequest {
    pub fn apply_to_bereich(&self, bereich: &mut TextBereich) {
        bereich.exact_generated_befehle.extend(self.generated_befehle.iter().cloned());
        bereich.exact_visible_columns.extend(
    self.required_columns
        .iter()
        .map(|&c| usize::try_from(c).expect("u32 column index does not fit into usize")),
);
        bereich.exact_visible_columns.extend(self.direct_columns.iter().copied().map(|n| n as usize));
    }
}
