
use crate::domain::eigenschaften::{EigenschaftKeyId, EigenschaftStandardFamilie};
use crate::domain::python_html_meta::css_class_for_visible_header;
use crate::domain::python_source_of_truth::{exact_meta_for_column, PY_DECLS};

#[derive(Debug, Default, Clone)]
struct HeaderMeta {
    col0: Option<u32>,
    tags: Option<String>,
    source_text: Option<String>,
}

fn decode_hex_utf8(hex: &str) -> Option<String> {
    if hex.len() % 2 != 0 {
        return None;
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut i = 0usize;
    while i < hex.len() {
        let part = &hex[i..i + 2];
        let value = u8::from_str_radix(part, 16).ok()?;
        bytes.push(value);
        i += 2;
    }
    String::from_utf8(bytes).ok()
}

fn parse_square_marker(content: &str) -> HeaderMeta {
    let mut meta = HeaderMeta::default();
    let mut rest = content;

    while let Some(start) = rest.find("[[") {
        let tail = &rest[start + 2..];
        let Some(end_rel) = tail.find("]]") else { break; };
        let inner = &tail[..end_rel].trim();

        if let Some(payload) = inner.strip_prefix("COL:") {
            meta.col0 = payload.trim().parse::<u32>().ok();
        } else if let Some(payload) = inner.strip_prefix("RPNN:") {
            for piece in payload.split(';') {
                let piece = piece.trim();
                if let Some(v) = piece.strip_prefix("TAGS=") {
                    meta.tags = Some(v.trim().to_string());
                } else if let Some(v) = piece.strip_prefix("SRCHEX=") {
                    meta.source_text = decode_hex_utf8(v.trim());
                }
            }
        }

        rest = &tail[end_rel + 2..];
    }

    meta
}

fn extract_id_suffix_1_based(raw: &str) -> Option<u32> {
    let id_pos = raw.rfind("(ID_")?;
    let rest = &raw[id_pos + 4..];
    let end = rest.find(')')?;
    rest[..end].parse::<u32>().ok()
}

fn strip_visible_text(raw: &str) -> String {
    let mut s = raw.to_string();

    loop {
        let Some(start) = s.find("[[") else { break; };
        let Some(rel_end) = s[start..].find("]]") else { break; };
        let end = start + rel_end + 2;
        s.replace_range(start..end, "");
    }

    if let Some(pos) = s.find('\u{1f}') {
        s.truncate(pos);
    }

    let s = s.trim().trim_matches('"').trim().to_string();

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

fn p2_slots_for_column(col0: u32) -> Vec<String> {
    let mut concrete_slots: Vec<String> = Vec::new();

    for decl in PY_DECLS.iter() {
        if decl.columns.contains(&col0) {
            if let Some(sub) = decl.sub_aliases.first() {
                concrete_slots.push((*sub).to_string());
            }
        }
    }

    if concrete_slots.is_empty() {
        return vec![String::new()];
    }

    concrete_slots
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

fn render_p2(slots: &[String]) -> String {
    let mut out = String::from("p2_");
    if slots.is_empty() {
        return out;
    }
    for (i, value) in slots.iter().enumerate() {
        out.push_str("p3_");
        out.push_str(&i.to_string());
        out.push('_');
        out.push_str(value);
        out.push(',');
    }
    out
}

fn render_p4_from_tag_string(tags: &str) -> String {
    if tags.trim().is_empty() {
        "p4_".to_string()
    } else {
        format!("p4_{}", tags.trim())
    }
}

fn render_p4(col0: u32) -> String {
    let tags = crate::lib4tables_enum::p4_fragment_for_column(col0);
    render_p4_from_tag_string(&tags)
}

fn replace_or_append_p4(class_attr: &str, p4: &str) -> String {
    if let Some(pos) = class_attr.find(" p4_") {
        let head = &class_attr[..pos];
        format!("{head} {p4}")
    } else {
        format!("{class_attr} {p4}")
    }
}

fn render_generated_eigenschaft_class(
    col_idx: usize,
    key: EigenschaftKeyId,
    family: EigenschaftStandardFamilie,
    tags: Option<&str>,
) -> String {
    let p1 = match family {
        EigenschaftStandardFamilie::N => "p1_✗Eigenschaften_n,,",
        EigenschaftStandardFamilie::EinsDurchN => "p1_✗Eigenschaften_1/n,,",
    };

    let p4 = render_p4_from_tag_string(tags.unwrap_or("3,0"));

    format!(
        "z_0 r_{} {} p2_p3_0_{}, {}",
        col_idx,
        p1,
        key.canonical_name(),
        p4
    )
}

fn class_from_generated_visible_text(visible_text: &str, col_idx: usize, tags: Option<&str>) -> Option<String> {
    let stripped = strip_generated_prefix(visible_text);
    let visible_n = normalize_key(stripped);

    if visible_n.is_empty() {
        return None;
    }

    for key in EigenschaftKeyId::ALL.iter().copied() {
        let matched = key.aliases().iter().any(|alias| {
            let alias_n = normalize_key(alias);
            !alias_n.is_empty()
                && (alias_n == visible_n
                    || visible_n.starts_with(&alias_n)
                    || alias_n.starts_with(&visible_n)
                    || visible_n.contains(&alias_n))
        });

        if matched {
            return Some(render_generated_eigenschaft_class(
                col_idx,
                key,
                key.standard_familie(),
                tags,
            ));
        }
    }

    None
}

fn class_from_decl_visible_text(visible_text: &str, col_idx: usize, forced_p4: Option<&str>) -> Option<String> {
    let visible_n = normalize_key(visible_text);

    if visible_n.is_empty() {
        return None;
    }

    for decl in PY_DECLS.iter() {
        let matched = decl.sub_aliases.iter().any(|alias| {
            let alias_n = normalize_key(alias);
            !alias_n.is_empty()
                && (alias_n == visible_n
                    || visible_n.starts_with(&alias_n)
                    || alias_n.starts_with(&visible_n)
                    || visible_n.contains(&alias_n)
                    || alias_n.contains(&visible_n))
        });

        if matched {
            if let Some(&col0) = decl.columns.first() {
                if let Some(meta) = exact_meta_for_column(col0) {
                    let class = format!("z_0 r_{} {}", col_idx, meta);
                    return Some(match forced_p4 {
                        Some(p4raw) => replace_or_append_p4(&class, &render_p4_from_tag_string(p4raw)),
                        None => class,
                    });
                }

                let p1_groups = p1_groups_for_column(col0);
                let p2_slots = p2_slots_for_column(col0);

                if !p1_groups.is_empty() {
                    let class = format!(
                        "z_0 r_{} {} {} {}",
                        col_idx,
                        render_p1(&p1_groups),
                        render_p2(&p2_slots),
                        forced_p4
                            .map(render_p4_from_tag_string)
                            .unwrap_or_else(|| render_p4(col0)),
                    );
                    return Some(class);
                }
            }
        }
    }

    None
}

fn class_from_marker_meta(raw: &str, visible_text: &str, col_idx: usize) -> Option<String> {
    let meta = parse_square_marker(raw);

    if let Some(col0) = meta.col0 {
        if let Some(meta_str) = exact_meta_for_column(col0) {
            let class = format!("z_0 r_{} {}", col_idx, meta_str);
            if let Some(tag_str) = meta.tags.as_deref() {
                return Some(replace_or_append_p4(&class, &render_p4_from_tag_string(tag_str)));
            }
            return Some(class);
        }

        let p1_groups = p1_groups_for_column(col0);
        let p2_slots = p2_slots_for_column(col0);
        if !p1_groups.is_empty() {
            return Some(format!(
                "z_0 r_{} {} {} {}",
                col_idx,
                render_p1(&p1_groups),
                render_p2(&p2_slots),
                meta.tags
                    .as_deref()
                    .map(render_p4_from_tag_string)
                    .unwrap_or_else(|| render_p4(col0)),
            ));
        }
    }

    if let Some(source_text) = meta.source_text.as_deref() {
        if let Some(class) = class_from_decl_visible_text(source_text, col_idx, meta.tags.as_deref()) {
            return Some(class);
        }
        if let Some(class) = class_from_generated_visible_text(source_text, col_idx, meta.tags.as_deref()) {
            return Some(class);
        }
    }

    if let Some(class) = class_from_generated_visible_text(visible_text, col_idx, meta.tags.as_deref()) {
        return Some(class);
    }

    if let Some(class) = class_from_decl_visible_text(visible_text, col_idx, meta.tags.as_deref()) {
        return Some(class);
    }

    None
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

    let visible_text = strip_visible_text(raw);

    if let Some(class_attr) = class_from_marker_meta(raw, &visible_text, col_idx) {
        return Some(class_attr);
    }

    if let Some(id1) = extract_id_suffix_1_based(raw) {
        if let Some(col0) = id1.checked_sub(1) {
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
                    render_p2(&p2_slots),
                    render_p4(col0),
                ));
            }
        }
    }

    if let Some(class_attr) = class_from_generated_visible_text(&visible_text, col_idx, None) {
        return Some(class_attr);
    }

    if let Some(class_attr) = class_from_decl_visible_text(&visible_text, col_idx, None) {
        return Some(class_attr);
    }

    if let Some(meta) = css_class_for_visible_header(&visible_text) {
        return Some(format!("z_0 r_{} {}", col_idx, meta));
    }

    None
}
