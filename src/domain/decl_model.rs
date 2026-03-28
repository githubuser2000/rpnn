use crate::domain::eigenschaften::{EigenschaftKeyId, EigenschaftStandardFamilie};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlDeclMeta {
    pub class: HtmlClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlClass {
    pub z: usize,
    pub r: usize,
    pub p1_groups: Vec<P1Group>,
    pub p_slots: Vec<PSlot>,
    pub p4_tags: Vec<P4Tag>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum P1Group {
    Zaehlung,
    Nummerierung,
    Eigenschaften(EigenschaftStandardFamilie),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PSlot {
    pub index: usize,
    pub value: PSlotValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PSlotValue {
    Empty,
    Eigenschaft(EigenschaftKeyId),
    Raw(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct P4Tag(pub u8);

impl HtmlDeclMeta {
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw.is_empty() {
            return None;
        }

        let z = parse_prefixed_usize(raw, "z_").unwrap_or(0);
        let r = parse_prefixed_usize(raw, "r_").unwrap_or(0);

        let p1_start = raw.find("p1_")?;
        let p2_start = raw[p1_start..]
            .find("p2_p3_")
            .map(|n| n + p1_start)
            .or_else(|| raw[p1_start..].find("p2_").map(|n| n + p1_start))?;
        let p4_start = raw[p2_start..].find("p4_")? + p2_start;

        let p1_raw = raw[p1_start + 3..p2_start].trim().trim_end_matches(',').trim();
        let p2_raw = if raw[p2_start..].starts_with("p2_p3_") {
            raw[p2_start + 6..p4_start].trim().trim_end_matches(',').trim()
        } else {
            raw[p2_start + 3..p4_start].trim().trim_end_matches(',').trim()
        };
        let p4_raw = raw[p4_start + 3..].trim();

        Some(Self {
            class: HtmlClass {
                z,
                r,
                p1_groups: parse_p1_groups(p1_raw),
                p_slots: parse_p_slots(p2_raw),
                p4_tags: parse_p4_tags(p4_raw),
            },
        })
    }

    pub fn render(&self) -> String {
        self.class.render()
    }
}

impl HtmlClass {
    pub fn render(&self) -> String {
        format!(
            "z_{} r_{} {} {} {}",
            self.z,
            self.r,
            render_p1_groups(&self.p1_groups),
            render_p_slots(&self.p_slots),
            render_p4_tags(&self.p4_tags)
        )
    }

    pub fn set_row(&mut self, row: usize) {
        self.r = row;
    }

    pub fn force_eigenschaft_family(&mut self, family: EigenschaftStandardFamilie) {
        self.p1_groups = vec![P1Group::Eigenschaften(family)];
    }

    pub fn replace_or_insert_eigenschaft(&mut self, key: EigenschaftKeyId) {
        let mut last_match: Option<usize> = None;
        let mut generic_e_match: Option<usize> = None;

        for (idx, slot) in self.p_slots.iter().enumerate() {
            match &slot.value {
                PSlotValue::Eigenschaft(_) => last_match = Some(idx),
                PSlotValue::Raw(raw) if raw.trim() == "E" => generic_e_match = Some(idx),
                _ => {}
            }
        }

        if let Some(idx) = last_match.or(generic_e_match) {
            self.p_slots[idx].value = PSlotValue::Eigenschaft(key);
            return;
        }

        let next_index = self.p_slots.iter().map(|s| s.index).max().unwrap_or(0) + 1;
        self.p_slots.push(PSlot {
            index: next_index,
            value: PSlotValue::Eigenschaft(key),
        });
    }

    pub fn force_p4_for_family(&mut self, family: EigenschaftStandardFamilie) {
        self.p4_tags = match family {
            EigenschaftStandardFamilie::N => vec![P4Tag(3), P4Tag(0)],
            EigenschaftStandardFamilie::EinsDurchN => vec![P4Tag(3), P4Tag(5), P4Tag(1), P4Tag(4)],
        };
    }

    pub fn ensure_default_p4_for_family(&mut self, family: EigenschaftStandardFamilie) {
        if !self.p4_tags.is_empty() {
            return;
        }
        self.force_p4_for_family(family);
    }
}

fn parse_prefixed_usize(raw: &str, prefix: &str) -> Option<usize> {
    let idx = raw.find(prefix)?;
    let rest = &raw[idx + prefix.len()..];
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse::<usize>().ok()
}

fn parse_p1_groups(raw: &str) -> Vec<P1Group> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| parse_p1_group(s.trim_start_matches('✗')))
        .collect()
}

fn parse_p1_group(raw: &str) -> P1Group {
    match raw {
        "Zählung" => P1Group::Zaehlung,
        "Nummerierung" => P1Group::Nummerierung,
        "Eigenschaften_n" => P1Group::Eigenschaften(EigenschaftStandardFamilie::N),
        "Eigenschaften_1/n" => P1Group::Eigenschaften(EigenschaftStandardFamilie::EinsDurchN),
        other => P1Group::Raw(other.to_string()),
    }
}

fn parse_p_slots(raw: &str) -> Vec<PSlot> {
    if raw.is_empty() {
        return vec![PSlot {
            index: 0,
            value: PSlotValue::Empty,
        }];
    }

    let mut entries: Vec<PSlot> = Vec::new();
    for token in raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let after = if let Some(rest) = token.strip_prefix("p3_") {
            rest
        } else if let Some(rest) = token.strip_prefix('0') {
            let value = rest.strip_prefix('_').unwrap_or(rest).trim();
            entries.push(PSlot {
                index: 0,
                value: parse_p_slot_value(value),
            });
            continue;
        } else {
            token
        };

        let Some((idx_part, value_part)) = after.split_once('_') else {
            continue;
        };
        let Ok(idx) = idx_part.parse::<usize>() else {
            continue;
        };
        entries.push(PSlot {
            index: idx,
            value: parse_p_slot_value(value_part.trim()),
        });
    }

    if entries.is_empty() {
        return vec![PSlot {
            index: 0,
            value: PSlotValue::Empty,
        }];
    }

    entries.sort_by_key(|s| s.index);
    entries
}

fn parse_p_slot_value(raw: &str) -> PSlotValue {
    if raw.is_empty() {
        return PSlotValue::Empty;
    }
    if let Some(key) = EigenschaftKeyId::from_alias(raw) {
        return PSlotValue::Eigenschaft(key);
    }
    PSlotValue::Raw(raw.to_string())
}

fn parse_p4_tags(raw: &str) -> Vec<P4Tag> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<u8>().ok())
        .map(P4Tag)
        .collect()
}

fn render_p1_groups(groups: &[P1Group]) -> String {
    let mut out = String::from("p1_");
    for g in groups {
        out.push('✗');
        out.push_str(&render_p1_group(g));
        out.push(',');
    }
    out.push(',');
    out
}

fn render_p1_group(group: &P1Group) -> String {
    match group {
        P1Group::Zaehlung => "Zählung".to_string(),
        P1Group::Nummerierung => "Nummerierung".to_string(),
        P1Group::Eigenschaften(EigenschaftStandardFamilie::N) => "Eigenschaften_n".to_string(),
        P1Group::Eigenschaften(EigenschaftStandardFamilie::EinsDurchN) => "Eigenschaften_1/n".to_string(),
        P1Group::Raw(s) => s.clone(),
    }
}

fn render_p_slots(slots: &[PSlot]) -> String {
    let mut out = String::from("p2_p3_");
    if slots.is_empty() {
        out.push_str("0_");
        return out;
    }
    for (i, slot) in slots.iter().enumerate() {
        if i == 0 {
            out.push_str(&slot.index.to_string());
            out.push('_');
            out.push_str(&render_p_slot_value(&slot.value));
        } else {
            out.push(',');
            out.push_str("p3_");
            out.push_str(&slot.index.to_string());
            out.push('_');
            out.push_str(&render_p_slot_value(&slot.value));
        }
    }
    out
}

fn render_p_slot_value(value: &PSlotValue) -> String {
    match value {
        PSlotValue::Empty => String::new(),
        PSlotValue::Eigenschaft(key) => key.canonical_name().to_string(),
        PSlotValue::Raw(s) => s.clone(),
    }
}

fn render_p4_tags(tags: &[P4Tag]) -> String {
    let mut out = String::from("p4_");
    for (i, tag) in tags.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&tag.0.to_string());
    }
    out
}
