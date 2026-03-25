use crate::cli::TextBereich;

pub fn fallback_zu_standards(bereich: &mut TextBereich) {
    bereich.spalten_bereiche.clear();
    bereich.spaltenreihenfolgeundnurdiese.clear();
    bereich.exact_visible_columns.clear();
    bereich.von_spalte = usize::MAX;
    bereich.bis_spalte = usize::MAX;
    bereich.reset_column_request();
}
