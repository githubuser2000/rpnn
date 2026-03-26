use crate::domain::python_source_of_truth::PY_DECLS;
use crate::lib4tables_enum::{table_tags2, ST};
use std::collections::BTreeSet;

fn extract_id_suffix_1_based(raw: &str) -> Option<u32> {
    let id_pos = raw.rfind("(ID_")?;
    let rest = &raw[id_pos + 4..];
    let end = rest.find(')')?;
    rest[..end].parse::<u32>().ok()
}

fn strip_transport_and_id(raw: &str) -> String {
    let mut s = raw.to_string();

    if let Some(pos) = s.find('\u{1f}') {
        s.truncate(pos);
    }

    let s = s.trim().trim_matches('"').to_string();

    if let Some(pos) = s.rfind("(ID_") {
        s[..pos].trim().to_string()
    } else {
        s
    }
}

fn dedupe_preserve_order(items: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for item in items {
        if item.is_empty() {
            continue;
        }
        if seen.insert(item.clone()) {
            out.push(item);
        }
    }
    out
}

fn col0_from_raw_or_visible_index(raw: &str, col_idx: usize) -> Option<u32> {
    if let Some(id1) = extract_id_suffix_1_based(raw) {
        return id1.checked_sub(1);
    }

    if col_idx >= 2 {
        return Some((col_idx - 2) as u32);
    }

    None
}

fn canonical_decl_meta_for_column(col0: u32) -> Option<(Vec<String>, Vec<String>)> {
    let mut p1_groups = Vec::new();
    let mut p2_slots = Vec::new();

    for decl in PY_DECLS {
        if decl.columns.iter().any(|&c| c == col0) {
            if let Some(main) = decl.main_aliases.first() {
                p1_groups.push((*main).to_string());
            }
            if let Some(sub) = decl.sub_aliases.first() {
                p2_slots.push((*sub).to_string());
            }
        }
    }

    let p1_groups = dedupe_preserve_order(p1_groups);
    let p2_slots = dedupe_preserve_order(p2_slots);

    if p1_groups.is_empty() && p2_slots.is_empty() {
        None
    } else {
        Some((p1_groups, p2_slots))
    }
}

fn render_p1(groups: &[String]) -> String {
    let mut out = String::from("p1_");
    for group in groups {
        out.push('✗');
        out.push_str(group);
        out.push(',');
    }
    out.push(',');
    out
}

fn render_p2(slots: &[String]) -> String {
    let mut out = String::from("p2_");
    for (idx, slot) in slots.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push_str("p3_");
        out.push_str(&idx.to_string());
        out.push('_');
        out.push_str(slot);
    }
    out
}

fn st_value(tag: &ST) -> u8 {
    match tag {
        ST::SternPolygon => 0,
        ST::GleichfoermigesPolygon => 1,
        ST::KeinPolygon => 2,
        ST::Galaxie => 3,
        ST::Universum => 4,
        ST::KeinParaOdMetaP => 5,
        ST::GebrRat => 6,
    }
}

fn render_p4(col0: u32) -> String {
    let tags = table_tags2();
    let mut out = String::from("p4_");

    if let Some(tagset) = tags.get(&(col0 as usize)) {
        let mut first = true;
        for tag in tagset {
            if !first {
                out.push(',');
            }
            first = false;
            out.push_str(&st_value(tag).to_string());
        }
    }

    out
}

fn render_meta_for_column(col0: u32) -> Option<String> {
    let (p1_groups, p2_slots) = canonical_decl_meta_for_column(col0)?;
    Some(format!("{} {} {}", render_p1(&p1_groups), render_p2(&p2_slots), render_p4(col0)))
}

pub fn build_python_exact_html_class(
    raw: &str,
    col_idx: usize,
    is_header_row: bool,
) -> Option<String> {
    if !is_header_row {
        return None;
    }

    if col_idx == 0 {
        return Some("z_0 r_0 p1_✗Zählung,, p2_p3_0_, p4_".to_string());
    }

    if col_idx == 1 {
        return Some("z_0 r_1 p1_✗Nummerierung,, p2_p3_0_, p4_".to_string());
    }

    let _visible = strip_transport_and_id(raw);
    let col0 = col0_from_raw_or_visible_index(raw, col_idx)?;
    let meta = render_meta_for_column(col0)?;
    Some(format!("z_0 r_{} {}", col_idx, meta))
}
