use crate::domain::decl_model::{
    HtmlDeclMeta, HtmlEigenschaftFamilie, HtmlP1Group, HtmlP2Slot, HtmlP4Tag,
};
use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::python_html_meta::css_class_for_visible_header;
use crate::domain::python_source_of_truth::{exact_decl_meta_for_column, exact_meta_for_column};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeaderSemantic {
    Counter,
    Numbering,
    SourceColumn(u32),
    GeneratedEigenschaft {
        key: EigenschaftKeyId,
        family: HtmlEigenschaftFamilie,
    },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct GeneratorMarker {
    eigkey: Option<EigenschaftKeyId>,
    family: Option<HtmlEigenschaftFamilie>,
}

fn extract_id_suffix_1_based(raw: &str) -> Option<u32> {
    let id_pos = raw.rfind("(ID_")?;
    let rest = &raw[id_pos + 4..];
    let end = rest.find(')')?;
    rest[..end].parse::<u32>().ok()
}

fn split_marker(raw: &str) -> (&str, Option<&str>) {
    if let Some(start) = raw.rfind("[[RPNN:") {
        let marker_start = start + "[[RPNN:".len();

        if let Some(rest) = raw.get(marker_start..) {
            if let Some(end_rel) = rest.find("]]") {
                let head = raw[..start].trim_end();
                let marker = &rest[..end_rel];
                return (head, Some(marker));
            }
        }
    }

    (raw, None)
}

fn strip_visible_text(raw: &str) -> String {
    let (raw_no_marker, _) = split_marker(raw);
    raw_no_marker.trim().trim_matches('"').trim().to_string()
}

fn fold_visible_text(s: &str) -> String {
    s.trim()
        .trim_matches('„')
        .trim_matches('“')
        .trim_matches('"')
        .trim()
        .to_lowercase()
}

fn parse_marker(raw: &str) -> Option<GeneratorMarker> {
    let (_, marker_raw) = split_marker(raw);
    let marker_raw = marker_raw?;
    let mut marker = GeneratorMarker::default();

    for part in marker_raw.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        if let Some(value) = part.strip_prefix("EIGKEY=") {
            marker.eigkey = EigenschaftKeyId::from_alias(value).or_else(|| {
                EigenschaftKeyId::ALL
                    .iter()
                    .copied()
                    .find(|k| fold_visible_text(k.canonical_name()) == fold_visible_text(value))
            });
        } else if let Some(value) = part.strip_prefix("FAMILY=") {
            marker.family = match value {
                "1N" => Some(HtmlEigenschaftFamilie::EinsDurchN),
                "N" => Some(HtmlEigenschaftFamilie::N),
                _ => None,
            };
        }
    }

    Some(marker)
}

fn family_from_visible_text(visible_text: &str) -> Option<HtmlEigenschaftFamilie> {
    let n = fold_visible_text(visible_text);
    if n.contains("gleichförmigenpolygonen")
        || n.contains("gleichfoermigenpolygonen")
        || n.contains("eigenschaften1n")
        || n.contains("1n")
    {
        Some(HtmlEigenschaftFamilie::EinsDurchN)
    } else if n.contains("sternenpolygonen") || n.contains("eigenschaftenn") {
        Some(HtmlEigenschaftFamilie::N)
    } else {
        None
    }
}

fn between_quotes(text: &str) -> Option<String> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let start_byte = chars.iter().find_map(|(i, ch)| if *ch == '„' || *ch == '"' { Some(*i) } else { None })?;
    let after_start = start_byte + text[start_byte..].chars().next()?.len_utf8();
    let tail = text.get(after_start..)?;
    let end_rel = tail.char_indices().find_map(|(i, ch)| if ch == '“' || ch == '"' { Some(i) } else { None })?;
    Some(tail[..end_rel].trim().to_string())
}

fn key_from_text(text: &str) -> Option<EigenschaftKeyId> {
    let seg_n = fold_visible_text(text);
    let mut best: Option<(usize, EigenschaftKeyId)> = None;
    for key in EigenschaftKeyId::ALL.iter().copied() {
        let canon = fold_visible_text(key.canonical_name());
        if !canon.is_empty() && seg_n.contains(&canon) {
            let score = canon.len() + 100;
            if best.map(|(n, _)| score > n).unwrap_or(true) {
                best = Some((score, key));
            }
        }
        for alias in key.aliases() {
            let a = fold_visible_text(alias);
            if !a.is_empty() && seg_n.contains(&a) {
                let score = a.len();
                if best.map(|(n, _)| score > n).unwrap_or(true) {
                    best = Some((score, key));
                }
            }
        }
    }
    best.map(|(_, key)| key)
}

fn key_from_quoted_segment(visible_text: &str) -> Option<EigenschaftKeyId> {
    let seg = between_quotes(visible_text)?;
    key_from_text(&seg)
}

fn semantic_from_header(raw: &str, col_idx: usize) -> HeaderSemantic {
    if col_idx == 0 {
        return HeaderSemantic::Counter;
    }
    if col_idx == 1 {
        return HeaderSemantic::Numbering;
    }

    if let Some(marker) = parse_marker(raw) {
        if let (Some(key), Some(family)) = (marker.eigkey, marker.family) {
            return HeaderSemantic::GeneratedEigenschaft { key, family };
        }
    }

    let visible = strip_visible_text(raw);
    if let (Some(key), Some(family)) = (key_from_quoted_segment(&visible), family_from_visible_text(&visible)) {
        return HeaderSemantic::GeneratedEigenschaft { key, family };
    }

    if let Some(id1) = extract_id_suffix_1_based(&visible) {
        if let Some(col0) = id1.checked_sub(1) {
            return HeaderSemantic::SourceColumn(col0);
        }
    }

    HeaderSemantic::Unknown
}

fn source_candidates_for_key(key: EigenschaftKeyId) -> Vec<u32> {
    let mut out: Vec<u32> = key.direct_columns().iter().map(|&c| c as u32).collect();
    if let Some((a, b)) = key.maybe_pair() {
        out.push(a as u32);
        out.push(b as u32);
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn choose_variant_index(col_idx: usize, len: usize) -> usize {
    if len == 0 { 0 } else { col_idx.saturating_sub(2) % len }
}

fn parsed_meta_for_column(col0: u32) -> Option<HtmlDeclMeta> {
    exact_decl_meta_for_column(col0)
}

fn fallback_meta(key: EigenschaftKeyId, family: HtmlEigenschaftFamilie) -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![family.render_p1()],
        p2_slots: vec![HtmlP2Slot::Eigenschaft(key), HtmlP2Slot::Empty],
        p4_tags: family.default_p4(),
    }
}

fn build_generated_meta(key: EigenschaftKeyId, family: HtmlEigenschaftFamilie, col_idx: usize) -> HtmlDeclMeta {
    let mut candidates: Vec<HtmlDeclMeta> = source_candidates_for_key(key)
        .into_iter()
        .filter_map(parsed_meta_for_column)
        .collect();

    let mut meta = if !candidates.is_empty() {
        candidates.swap_remove(choose_variant_index(col_idx, candidates.len()))
    } else {
        fallback_meta(key, family)
    };

    meta.p1_groups = vec![family.render_p1()];
    if meta.p2_slots.is_empty() { meta.p2_slots.push(HtmlP2Slot::Empty); }
    if meta.p2_slots.len() == 1 { meta.p2_slots.push(HtmlP2Slot::Empty); }
    for slot in &mut meta.p2_slots {
        if slot.is_empty() {
            *slot = HtmlP2Slot::Empty;
        }
    }
    meta.p2_slots[0] = HtmlP2Slot::Eigenschaft(key);
    if meta.p4_tags.is_empty() { meta.p4_tags = family.default_p4(); }
    meta
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlHeaderClass {
    pub class_attr: String,
}

pub fn resolve_html_header_class(raw: &str, col_idx: usize, is_header_row: bool) -> Option<HtmlHeaderClass> {
    if !is_header_row { return None; }
    let class_attr = match semantic_from_header(raw, col_idx) {
        HeaderSemantic::Counter => "z_0 r_0 p1_✗Zählung,, p2_p3_0_, p4_".to_string(),
        HeaderSemantic::Numbering => "z_0 r_1 p1_✗Nummerierung,, p2_p3_0_, p4_".to_string(),
        HeaderSemantic::GeneratedEigenschaft { key, family } => {
            let meta = build_generated_meta(key, family, col_idx);
            format!("z_0 r_{} {}", col_idx, meta.render())
        }
        HeaderSemantic::SourceColumn(col0) => format!("z_0 r_{} {}", col_idx, exact_meta_for_column(col0)?),
        HeaderSemantic::Unknown => {
            let visible = strip_visible_text(raw);
            format!("z_0 r_{} {}", col_idx, css_class_for_visible_header(&visible)?)
        }
    };
    Some(HtmlHeaderClass { class_attr })
}

pub fn build_python_exact_html_class(raw: &str, col_idx: usize, is_header_row: bool) -> Option<String> {
    resolve_html_header_class(raw, col_idx, is_header_row).map(|resolved| resolved.class_attr)
}
