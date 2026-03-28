
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlDeclMeta {
    pub p1_groups: Vec<String>,
    pub p2_slots: Vec<Option<String>>,
    pub p4_tags: Vec<u8>,
}

impl HtmlDeclMeta {
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw.is_empty() {
            return None;
        }

        let p1_start = raw.find("p1_")?;
        let p2_start = raw[p1_start..].find("p2_")? + p1_start;
        let p4_start = raw[p2_start..].find("p4_")? + p2_start;

        let p1_raw = raw[p1_start + 3..p2_start].trim().trim_end_matches(',').trim();
        let p2_raw = raw[p2_start + 3..p4_start].trim().trim_end_matches(',').trim();
        let p4_raw = raw[p4_start + 3..].trim();

        let p1_groups = parse_p1_groups(p1_raw);
        let p2_slots = parse_p2_slots(p2_raw);
        let p4_tags = parse_p4_tags(p4_raw);

        Some(Self {
            p1_groups,
            p2_slots,
            p4_tags,
        })
    }

    pub fn render(&self) -> String {
        format!(
            "{} {} {}",
            render_p1_groups(&self.p1_groups),
            render_p2_slots(&self.p2_slots),
            render_p4_tags(&self.p4_tags)
        )
    }
}


impl HtmlDeclMeta {
    pub fn new(p1_groups: Vec<String>, p2_slots: Vec<Option<String>>, p4_tags: Vec<u8>) -> Self {
        Self { p1_groups, p2_slots, p4_tags }
    }

    pub fn from_slices(p1_groups: &[&str], p2_slots: &[Option<&str>], p4_tags: &[u8]) -> Self {
        Self {
            p1_groups: p1_groups.iter().map(|s| (*s).to_string()).collect(),
            p2_slots: p2_slots.iter().map(|s| s.map(|v| v.to_string())).collect(),
            p4_tags: p4_tags.to_vec(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlEigenschaftFamilie {
    N,
    EinsDurchN,
}

impl HtmlEigenschaftFamilie {
    pub fn render_p1(self) -> &'static str {
        match self {
            Self::N => "Eigenschaften_n",
            Self::EinsDurchN => "Eigenschaften_1/n",
        }
    }

    pub fn default_p4(self) -> Vec<u8> {
        match self {
            Self::N => vec![0, 5],
            Self::EinsDurchN => vec![3, 5, 1, 4],
        }
    }
}

fn parse_p1_groups(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start_matches('✗').to_string())
        .collect()
}

fn parse_p2_slots(raw: &str) -> Vec<Option<String>> {
    if raw.is_empty() {
        return vec![None];
    }

    let mut entries: Vec<(usize, Option<String>)> = Vec::new();
    let mut max_idx = 0usize;

    for token in raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if !token.starts_with("p3_") {
            continue;
        }
        let after = &token[3..];
        let Some((idx_part, value_part)) = after.split_once('_') else {
            continue;
        };
        let Ok(idx) = idx_part.parse::<usize>() else {
            continue;
        };
        max_idx = max_idx.max(idx);
        let value = if value_part.is_empty() {
            None
        } else {
            Some(value_part.to_string())
        };
        entries.push((idx, value));
    }

    let mut out = vec![None; max_idx + 1];
    for (idx, value) in entries {
        out[idx] = value;
    }
    out
}

fn parse_p4_tags(raw: &str) -> Vec<u8> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<u8>().ok())
        .collect()
}

fn render_p1_groups(groups: &[String]) -> String {
    let mut out = String::from("p1_");
    for g in groups {
        out.push('✗');
        out.push_str(g);
        out.push(',');
    }
    out.push(',');
    out
}

fn render_p2_slots(slots: &[Option<String>]) -> String {
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

fn render_p4_tags(tags: &[u8]) -> String {
    let mut out = String::from("p4_");
    for (i, tag) in tags.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&tag.to_string());
    }
    out
}
