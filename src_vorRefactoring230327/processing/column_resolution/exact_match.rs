use crate::cli::TextBereich;

pub fn finalize_exact_columns(bereich: &mut TextBereich) {
    bereich.spalten_bereiche.sort_unstable();
    bereich.spalten_bereiche.dedup();
    if !bereich.spalten_bereiche.is_empty() {
        bereich.von_spalte = bereich.spalten_bereiche[0].0;
        bereich.bis_spalte = bereich.spalten_bereiche.last().unwrap().1;
    }
    bereich.mark_columns_resolved();
}
