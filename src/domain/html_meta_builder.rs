use crate::domain::decl_model::HtmlDeclMeta;
use crate::domain::eigenschaften::{EigenschaftKeyId, EigenschaftStandardFamilie};
use crate::domain::python_html_meta::css_class_for_visible_header;
use crate::domain::python_source_of_truth::{exact_meta_for_column, PY_DECLS};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeaderSemantic {
    Counter,
    Numbering,
    SourceColumn(u32),
    GeneratedEigenschaft {
        key: EigenschaftKeyId,
        family: EigenschaftStandardFamilie,
    },
    Unknown,
}

fn extract_id_suffix_1_based(raw: &str) -> Option<u32> {
    let id_pos = raw.rfind("(ID_")?;
    let rest = &raw[id_pos + 4..];
    let end = rest.find(')')?;
    rest[..end].parse::<u32>().ok()
}

fn strip_visible_text(raw: &str) -> String {
    let s = raw.trim().trim_matches('"').trim().to_string();
    if let Some(pos) = s.rfind("(ID_") {
        s[..pos].trim().to_string()
    } else {
        s
    }
}

fn strip_generated_prefix(raw: &str) -> &str {
    raw.trim()
        .strip_prefix("Generiert:")
        .map(str::trim)
        .unwrap_or_else(|| raw.trim())
}

fn normalize_key(s: &str) -> String {
    s.to_lowercase()
        .replace('_', "")
        .replace('-', "")
        .replace(' ', "")
        .replace('/', "")
        .replace('(', "")
        .replace(')', "")
        .replace(',', "")
        .replace(':', "")
        .replace('„', "")
        .replace('“', "")
        .replace('"', "")
}

fn semantic_from_header(raw: &str, col_idx: usize) -> HeaderSemantic {
    if col_idx == 0 {
        return HeaderSemantic::Counter;
    }
    if col_idx == 1 {
        return HeaderSemantic::Numbering;
    }

    let visible = strip_visible_text(raw);

    // WICHTIG: Generator-Eigenschaften haben oft trotzdem einen ID-Suffix aus einer Quellspalte.
    // Dieser darf die semantische Generator-Klassifikation NICHT überstimmen.
    if let Some((key, family)) = generated_eigenschaft_from_visible_text(&visible) {
        return HeaderSemantic::GeneratedEigenschaft { key, family };
    }

    if let Some(id1) = extract_id_suffix_1_based(raw) {
        if let Some(col0) = id1.checked_sub(1) {
            return HeaderSemantic::SourceColumn(col0);
        }
    }

    HeaderSemantic::Unknown
}

fn generated_eigenschaft_from_visible_text(visible_text: &str) -> Option<(EigenschaftKeyId, EigenschaftStandardFamilie)> {
    let stripped = strip_generated_prefix(visible_text);
    let normalized = normalize_key(stripped);
    if normalized.is_empty() {
        return None;
    }

    let key = EigenschaftKeyId::ALL.iter().copied().find(|key| {
        key.aliases().iter().any(|alias| {
            let alias_n = normalize_key(alias);
            !alias_n.is_empty()
                && (normalized.contains(&alias_n)
                    || alias_n.contains(&normalized))
        })
    })?;

    let family = if normalized.contains("eigenschaften1n")
        || normalized.contains("gleichförmigenpolygonen")
        || normalized.contains("gleichfoermigenpolygonen")
        || normalized.contains("1n")
    {
        EigenschaftStandardFamilie::EinsDurchN
    } else if normalized.contains("eigenschaftenn") || normalized.contains("sternenpolygonen") {
        EigenschaftStandardFamilie::N
    } else {
        key.standard_familie()
    };

    Some((key, family))
}

fn source_columns_for_eigenschaft(key: EigenschaftKeyId) -> Vec<u32> {
    let mut out: Vec<u32> = key.direct_columns().iter().map(|&n| n as u32).collect();
    if let Some((a, b)) = key.maybe_pair() {
        out.push(a as u32);
        out.push(b as u32);
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn template_columns_for_family(family: EigenschaftStandardFamilie) -> &'static [u32] {
    match family {
        EigenschaftStandardFamilie::N => &[112u32],
        EigenschaftStandardFamilie::EinsDurchN => &[331u32, 335u32],
    }
}

fn choose_variant_index(col_idx: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        col_idx.saturating_sub(2) % len
    }
}

fn parsed_meta_for_column(col0: u32) -> Option<crate::domain::decl_model::HtmlClass> {
    let meta = exact_meta_for_column(col0)?;
    let parsed = HtmlDeclMeta::parse(&meta)?;
    Some(parsed.class)
}

fn best_source_class_for_generated_eigenschaft(
    key: EigenschaftKeyId,
    family: EigenschaftStandardFamilie,
    col_idx: usize,
) -> Option<crate::domain::decl_model::HtmlClass> {
    let exact_candidates: Vec<_> = source_columns_for_eigenschaft(key)
        .into_iter()
        .filter_map(parsed_meta_for_column)
        .collect();

    let template_candidates: Vec<_> = template_columns_for_family(family)
        .iter()
        .copied()
        .filter_map(parsed_meta_for_column)
        .collect();

    match family {
        EigenschaftStandardFamilie::EinsDurchN => {
            if !template_candidates.is_empty() {
                return template_candidates
                    .get(choose_variant_index(col_idx, template_candidates.len()))
                    .cloned();
            }
            if !exact_candidates.is_empty() {
                return exact_candidates
                    .get(choose_variant_index(col_idx, exact_candidates.len()))
                    .cloned();
            }
        }
        EigenschaftStandardFamilie::N => {
            if !exact_candidates.is_empty() {
                return exact_candidates
                    .get(choose_variant_index(col_idx, exact_candidates.len()))
                    .cloned();
            }
            if !template_candidates.is_empty() {
                return template_candidates
                    .get(choose_variant_index(col_idx, template_candidates.len()))
                    .cloned();
            }
        }
    }

    None
}

fn build_typed_generated_eigenschaft_class(
    key: EigenschaftKeyId,
    family: EigenschaftStandardFamilie,
    col_idx: usize,
) -> Option<String> {
    let mut class = best_source_class_for_generated_eigenschaft(key, family, col_idx).unwrap_or_else(|| {
        let raw = match family {
            EigenschaftStandardFamilie::N => format!(
                "z_0 r_{} p1_✗Eigenschaften_n,, p2_p3_0_{},p3_1_, p4_3,0",
                col_idx,
                key.canonical_name()
            ),
            EigenschaftStandardFamilie::EinsDurchN => format!(
                "z_0 r_{} p1_✗Eigenschaften_1/n,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_Gleichheit_Freiheit,p3_2_Reziprokes,p3_3_Ordnung_und_Filterung_12_und_1pro12,p3_4_{},p3_5_, p4_3,5,1,4",
                col_idx,
                key.canonical_name()
            ),
        };
        HtmlDeclMeta::parse(&raw).expect("fallback html meta must parse").class
    });

    class.set_row(col_idx);
    class.force_eigenschaft_family(family);
    class.replace_or_insert_eigenschaft(key);
    class.force_p4_for_family(family);
    Some(class.render())
}

fn push_unique(out: &mut Vec<String>, value: &str) {
    if !out.iter().any(|v| v == value) {
        out.push(value.to_string());
    }
}

fn p1_groups_for_column(col0: u32) -> Vec<String> {
    let mut out = Vec::new();
    for decl in PY_DECLS.iter() {
        if decl.columns.contains(&col0) {
            if let Some(main) = decl.main_aliases.first() {
                push_unique(&mut out, main);
            }
        }
    }
    out
}

fn p2_slots_for_column(col0: u32) -> Vec<Option<String>> {
    let mut concrete_slots: Vec<String> = Vec::new();
    for decl in PY_DECLS.iter() {
        if decl.columns.contains(&col0) {
            if let Some(sub) = decl.sub_aliases.first() {
                concrete_slots.push((*sub).to_string());
            }
        }
    }
    if concrete_slots.is_empty() {
        return vec![None];
    }
    let total_slots = 15 + concrete_slots.len().saturating_sub(1);
    let mut out: Vec<Option<String>> = concrete_slots.into_iter().map(Some).collect();
    while out.len() < total_slots {
        out.push(None);
    }
    out
}

fn render_p1(groups: &[String]) -> String {
    let mut out = String::from("p1_");
    for g in groups {
        out.push('✗');
        out.push_str(g);
        out.push(',');
    }
    out.push(',');
    out
}

fn render_p2_p3(slots: &[Option<String>]) -> String {
    let mut out = String::from("p2_p3_");
    for (i, slot) in slots.iter().enumerate() {
        if i == 0 {
            out.push('0');
            out.push('_');
            if let Some(value) = slot {
                out.push_str(value);
            }
        } else {
            out.push(',');
            out.push_str("p3_");
            out.push_str(&i.to_string());
            out.push('_');
            if let Some(value) = slot {
                out.push_str(value);
            }
        }
    }
    out
}

fn render_p4(col0: u32) -> String {
    let tags = crate::lib4tables_enum::p4_fragment_for_column(col0);
    format!("p4_{}", tags)
}

fn class_from_decl_visible_text(visible_text: &str, col_idx: usize) -> Option<String> {
    let visible_n = normalize_key(visible_text);
    if visible_n.is_empty() {
        return None;
    }

    for decl in PY_DECLS.iter() {
        let matched = decl.sub_aliases.iter().any(|alias| {
            let alias_n = normalize_key(alias);
            !alias_n.is_empty() && (alias_n == visible_n || visible_n.starts_with(&alias_n) || alias_n.starts_with(&visible_n))
        });

        if matched {
            if let Some(&col0) = decl.columns.first() {
                if let Some(meta) = exact_meta_for_column(col0) {
                    return Some(format!("z_0 r_{} {}", col_idx, meta));
                }

                let p1_groups = p1_groups_for_column(col0);
                let p2_slots = p2_slots_for_column(col0);
                if !p1_groups.is_empty() {
                    return Some(format!(
                        "z_0 r_{} {} {} {}",
                        col_idx,
                        render_p1(&p1_groups),
                        render_p2_p3(&p2_slots),
                        render_p4(col0),
                    ));
                }
            }
        }
    }
    None
}

pub fn build_python_exact_html_class(raw: &str, col_idx: usize, is_header_row: bool) -> Option<String> {
    if !is_header_row {
        return None;
    }

    match semantic_from_header(raw, col_idx) {
        HeaderSemantic::Counter => {
            return Some("z_0 r_0 p1_✗Zählung,, p2_p3_0_, p4_".to_string());
        }
        HeaderSemantic::Numbering => {
            return Some("z_0 r_1 p1_✗Nummerierung,, p2_p3_0_, p4_".to_string());
        }
        HeaderSemantic::SourceColumn(col0) => {
            if let Some(meta) = exact_meta_for_column(col0) {
                return Some(format!("z_0 r_{} {}", col_idx, meta));
            }
            let p1_groups = p1_groups_for_column(col0);
            let p2_slots = p2_slots_for_column(col0);
            if !p1_groups.is_empty() {
                return Some(format!(
                    "z_0 r_{} {} {} {}",
                    col_idx,
                    render_p1(&p1_groups),
                    render_p2_p3(&p2_slots),
                    render_p4(col0),
                ));
            }
        }
        HeaderSemantic::GeneratedEigenschaft { key, family } => {
            if let Some(class_attr) = build_typed_generated_eigenschaft_class(key, family, col_idx) {
                return Some(class_attr);
            }
        }
        HeaderSemantic::Unknown => {}
    }

    let visible_text = strip_visible_text(raw);
    if let Some(class_attr) = class_from_decl_visible_text(&visible_text, col_idx) {
        return Some(class_attr);
    }
    if let Some(meta) = css_class_for_visible_header(&visible_text) {
        return Some(format!("z_0 r_{} {}", col_idx, meta));
    }

    None
}
