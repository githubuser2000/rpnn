use crate::domain::python_source_of_truth::PY_DECLS;

fn extract_id_suffix_1_based(raw: &str) -> Option<u32> {
    let id_pos = raw.rfind("(ID_")?;
    let rest = &raw[id_pos + 4..];
    let end = rest.find(')')?;
    rest[..end].parse::<u32>().ok()
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

    // Python-Struktur:
    // - bei 1 Hauptgruppe: p3_0..p3_24  => 25 Slots
    // - bei 2 Hauptgruppen: p3_0..p3_25 => 26 Slots
    // - bei 3 Hauptgruppen: p3_0..p3_26 => 27 Slots
    //
    // Also: 25 + (Anzahl konkreter Hierarchieeinträge - 1)
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

    // Wichtig: ID im Header ist 1-basiert, die PY_DECLS-Spalten sind 0-basiert.
    let col0 = if let Some(id1) = extract_id_suffix_1_based(raw) {
        id1.checked_sub(1)?
    } else {
        // Fallback nur, falls mal kein (ID_xxx) im Header steht.
        // Die ersten zwei Spalten sind Zählung/Nummerierung, daher -2.
        col_idx.checked_sub(2)? as u32
    };

    let p1_groups = p1_groups_for_column(col0);
    let p2_slots = p2_slots_for_column(col0);

    if p1_groups.is_empty() {
        return None;
    }

    let class_str = format!(
        "z_0 r_{} {} {} {}",
        col_idx,
        render_p1(&p1_groups),
        render_p2_p3(&p2_slots),
        render_p4(col0),
    );

    Some(class_str)
}
