use crate::cli::TextBereich;
use crate::domain::exact_generator_bridge::resolve_exact_generator;

pub fn merge_exact(bereich: &mut TextBereich, ober: &str, unter: &str) -> bool {
    let Some(exact) = resolve_exact_generator(ober, unter) else {
        return false;
    };

    let exact_is_meta = exact.generated_befehle.contains("universummetakonkret");
    for cmd in exact.generated_befehle {
        bereich.exact_generated_befehle.insert(cmd);
    }
    for pair in exact.modal_pairs {
        if !bereich.exact_modal_pairs.contains(&pair) {
            bereich.exact_modal_pairs.push(pair);
        }
    }
    for spec in exact.meta_konkret_specs {
        if !bereich.exact_meta_konkret_specs.contains(&spec) {
            bereich.exact_meta_konkret_specs.push(spec);
        }
    }
    for col in exact.direct_columns {
        if !exact_is_meta && !bereich.exact_visible_columns.contains(&col) {
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
