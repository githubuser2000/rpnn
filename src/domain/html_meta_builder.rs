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

fn replace_p4_fragment(meta: &str, p4: &str) -> String {
    if let Some(pos) = meta.rfind(" p4_") {
        let prefix = &meta[..pos];
        format!("{} p4_{}", prefix, p4)
    } else {
        format!("{} p4_{}", meta.trim_end_matches(','), p4)
    }
}

fn apply_lib4tables_enum_p4(meta: String, raw: &str) -> String {
    let Some(id1) = extract_id_suffix_1_based(raw) else {
        return meta;
    };

    let Some(col0) = id1.checked_sub(1) else {
        return meta;
    };

    let p4 = crate::domain::lib4tables_enum::p4_fragment_for_column(col0);
    replace_p4_fragment(&meta, &p4)
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

    let visible = strip_transport_and_id(raw);

    // 1) Bevorzugt: reichere Meta über sichtbaren Headertext
    if let Some(meta) = crate::domain::python_html_meta::lookup_header_meta(&visible) {
        let enriched = apply_lib4tables_enum_p4(meta.to_string(), raw);
        return Some(format!("z_0 r_{} {}", col_idx, enriched));
    }

    // 2) Fallback: komprimierte Spaltenmeta über ID
    if let Some(id1) = extract_id_suffix_1_based(raw) {
        if let Some(col0) = id1.checked_sub(1) {
            if let Some(meta) = crate::domain::python_source_of_truth::exact_meta_for_column(col0) {
                let enriched = apply_lib4tables_enum_p4(meta, raw);
                return Some(format!("z_0 r_{} {}", col_idx, enriched));
            }
        }
    }

    None
}
