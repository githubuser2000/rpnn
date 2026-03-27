use crate::cli::TextBereich;
use crate::domain::resolve_cli_legacy_adapter::resolve_cli_selection;

pub fn merge_exact(bereich: &mut TextBereich, ober: &str, unter: &str) -> bool {
    let dummy_map = crate::domain::categories::lade_kategorie_map();
    let Ok(exact) = resolve_cli_selection(&dummy_map, ober, unter) else {
        return false;
    };

    for cmd in exact.generated_befehle {
        bereich.exact_generated_befehle.insert(cmd);
    }
    for pair in exact.exact_modal_pairs {
        if !bereich.exact_modal_pairs.contains(&pair) {
            bereich.exact_modal_pairs.push(pair);
        }
    }
    for spec in exact.exact_meta_konkret_specs {
        if !bereich.exact_meta_konkret_specs.contains(&spec) {
            bereich.exact_meta_konkret_specs.push(spec);
        }
    }
    for col in exact.direct_columns {
        let col = col as usize;
        if !bereich.exact_visible_columns.contains(&col) {
            bereich.exact_visible_columns.push(col);
        }
        bereich.spalten_bereiche.push((col, col));
    }
    for col in exact.required_columns {
        let col = col as usize;
        if !bereich.exact_visible_columns.contains(&col) {
            bereich.exact_visible_columns.push(col);
        }
        bereich.spalten_bereiche.push((col, col));
    }
    for col in exact.exact_direct_columns {
        if !bereich.exact_visible_columns.contains(&col) {
            bereich.exact_visible_columns.push(col);
        }
        bereich.spalten_bereiche.push((col, col));
    }

    bereich.exact_visible_columns.sort_unstable();
    bereich.exact_visible_columns.dedup();
    bereich.spalten_bereiche.sort_unstable();
    bereich.spalten_bereiche.dedup();
    bereich.mark_columns_resolved();
    true
}
