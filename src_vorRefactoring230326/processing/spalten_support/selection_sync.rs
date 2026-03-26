use crate::cli::TextBereich;

pub fn finalize_found_columns(bereich: &mut TextBereich) {
    bereich.spalten_bereiche.sort_unstable();
    bereich.spalten_bereiche.dedup();
    if !bereich.spalten_bereiche.is_empty() {
        bereich.von_spalte = bereich.spalten_bereiche[0].0;
        bereich.bis_spalte = bereich.spalten_bereiche.last().unwrap().1;
    }
}

pub fn setze_gefundene_spalten(bereich: &mut TextBereich, gefundene_spalten: Vec<u32>) {
    let mut sorted: Vec<usize> = gefundene_spalten.iter().map(|&n| n as usize).collect();
    sorted.sort_unstable();
    sorted.dedup();

    bereich.spalten_bereiche = sorted.iter().map(|&num| (num, num)).collect();

    for &num in &sorted {
        if !bereich.exact_visible_columns.contains(&num) {
            bereich.exact_visible_columns.push(num);
        }
        if !bereich.spaltenreihenfolgeundnurdiese.contains(&num) {
            bereich.spaltenreihenfolgeundnurdiese.push(num);
        }
    }

    bereich.exact_visible_columns.sort_unstable();
    bereich.exact_visible_columns.dedup();
    bereich.spaltenreihenfolgeundnurdiese.sort_unstable();
    bereich.spaltenreihenfolgeundnurdiese.dedup();

    if !bereich.spalten_bereiche.is_empty() {
        bereich.von_spalte = bereich.spalten_bereiche[0].0;
        bereich.bis_spalte = bereich.spalten_bereiche.last().unwrap().1;
    } else {
        bereich.von_spalte = usize::MAX;
        bereich.bis_spalte = usize::MAX;
    }

    if sorted.is_empty() {
        bereich.reset_column_request();
    } else {
        bereich.mark_columns_resolved();
    }
}
