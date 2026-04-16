use crate::cli::TextBereich;

pub fn apply_resolved_columns(bereich: &mut TextBereich, columns: &[usize]) {
    bereich.spalten_bereiche = columns.iter().map(|&num| (num, num)).collect();
    if let Some(first) = columns.first().copied() {
        bereich.von_spalte = first;
        bereich.bis_spalte = *columns.last().unwrap_or(&first);
        bereich.mark_columns_resolved();
    } else {
        bereich.reset_column_request();
    }
}
