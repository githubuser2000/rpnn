use crate::cli::TextBereich;

pub fn begin_resolution(bereich: &mut TextBereich) {
    bereich.mark_columns_requested();
}

pub fn clear_resolution(bereich: &mut TextBereich) {
    bereich.reset_column_request();
}
