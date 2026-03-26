use crate::cli::TextBereich;

pub fn merge_exact_columns_into_bereich(bereich: &mut TextBereich, mut gefundene_spalten: Vec<u32>) {
    if gefundene_spalten.is_empty() {
        return;
    }

    bereich.mark_columns_resolved();
    let mut combined = bereich.spaltenreihenfolgeundnurdiese.clone();
    combined.extend(gefundene_spalten.drain(..).map(|x| x as usize));
    combined.sort_unstable();
    combined.dedup();
    bereich.spaltenreihenfolgeundnurdiese = combined.clone();

    bereich.exact_visible_columns.extend(combined.iter().copied());
    bereich.exact_visible_columns.sort_unstable();
    bereich.exact_visible_columns.dedup();

    bereich.spalten_bereiche = combined.iter().map(|&x| (x, x)).collect();

    if let Some(&(first, _)) = bereich.spalten_bereiche.first() {
        bereich.von_spalte = first;
    }
    if let Some(&(_, last)) = bereich.spalten_bereiche.last() {
        bereich.bis_spalte = last;
    }
}
