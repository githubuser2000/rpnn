use std::collections::BTreeSet;

use crate::python_exact_mappings::{EIGENSCHAFT_MAPPINGS, META_KONKRET_MAPPINGS};

#[derive(Debug, Clone, Default)]
pub struct ExactResolved {
    pub direct_columns: Vec<u32>,
    pub pair_columns: Vec<u32>,
    pub generated_befehle: BTreeSet<String>,
}

fn normalize_key(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss")
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
}

fn stable_dedup(values: &[u32]) -> Vec<u32> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for &v in values {
        if v > 0 && seen.insert(v) {
            out.push(v);
        }
    }
    out
}

fn mode_from_oberkategorie(ober: &str) -> Option<&'static str> {
    let n = normalize_key(ober);
    match n.as_str() {
        "universummetakonkret" | "metakonkret" | "meta" | "konkret" => Some("universummetakonkret"),
        "eigenschaft" | "eigenschaften" | "eigenschaftenn" | "eigenschaften1n" => Some("eigenschaften"),
        "konzept" | "konzepte" => Some("konzept"),
        "konzept2" | "konzepte2" => Some("konzept2"),
        _ => None,
    }
}

fn resolve_meta_konkret(value: &str) -> ExactResolved {
    let needle = normalize_key(value);
    let mut out = ExactResolved::default();

    for (aliases, (group, side)) in META_KONKRET_MAPPINGS {
        if aliases.iter().any(|a| normalize_key(a) == needle) {
            let base = (*group as u32).saturating_sub(2) * 2 + 1;
            let left = base;
            let right = base + 1;
            let chosen = if *side == 0 { left } else { right };
            out.direct_columns.push(chosen);
            out.pair_columns.push(left);
            out.pair_columns.push(right);
            out.generated_befehle.insert("modallogik".to_string());
        }
    }

    out.direct_columns = stable_dedup(&out.direct_columns);
    out.pair_columns = stable_dedup(&out.pair_columns);
    out
}

fn resolve_eigenschaft_like(value: &str) -> ExactResolved {
    let needle = normalize_key(value);
    let mut out = ExactResolved::default();

    for (aliases, directs, pair) in EIGENSCHAFT_MAPPINGS {
        if aliases.iter().any(|a| normalize_key(a) == needle) {
            out.direct_columns.extend(directs.iter().map(|&v| v as u32));
            if let Some((a, b)) = pair {
                out.pair_columns.push(*a as u32);
                out.pair_columns.push(*b as u32);
                out.generated_befehle.insert("modallogik".to_string());
            }
        }
    }

    out.direct_columns = stable_dedup(&out.direct_columns);
    out.pair_columns = stable_dedup(&out.pair_columns);
    out
}

pub fn resolve_exact_generator(ober: &str, unter: &str) -> Option<ExactResolved> {
    let mode = mode_from_oberkategorie(ober)?;
    let resolved = match mode {
        "universummetakonkret" => resolve_meta_konkret(unter),
        "eigenschaften" | "konzept" | "konzept2" => resolve_eigenschaft_like(unter),
        _ => ExactResolved::default(),
    };

    if resolved.direct_columns.is_empty()
        && resolved.pair_columns.is_empty()
        && resolved.generated_befehle.is_empty()
    {
        None
    } else {
        Some(resolved)
    }
}
