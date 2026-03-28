use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::DomainId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlDeclMeta {
    pub p1_groups: Vec<HtmlP1Group>,
    pub p2_slots: Vec<HtmlP2Slot>,
    pub p4_tags: Vec<HtmlP4Tag>,
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

        let p1_groups = parse_p1_groups(p1_raw)?;
        let p2_slots = parse_p2_slots(p2_raw)?;
        let p4_tags = parse_p4_tags(p4_raw)?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlEigenschaftFamilie {
    N,
    EinsDurchN,
}

impl HtmlEigenschaftFamilie {
    pub fn render_p1(self) -> HtmlP1Group {
        HtmlP1Group::EigenschaftFamilie(self)
    }

    pub fn default_p4(self) -> Vec<HtmlP4Tag> {
        match self {
            Self::N => vec![HtmlP4Tag::new(0), HtmlP4Tag::new(5)],
            Self::EinsDurchN => vec![HtmlP4Tag::new(3), HtmlP4Tag::new(5), HtmlP4Tag::new(1), HtmlP4Tag::new(4)],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlP1Group {
    Zaehlung,
    Nummerierung,
    Domain(DomainId),
    Grundstrukturen,
    EigenschaftFamilie(HtmlEigenschaftFamilie),
}

impl HtmlP1Group {
    fn parse(raw: &str) -> Option<Self> {
        match raw.trim() {
            "Zählung" => Some(Self::Zaehlung),
            "Nummerierung" => Some(Self::Nummerierung),
            "Menschliches" => Some(Self::Domain(DomainId::Menschliches)),
            "Religion" | "Religionen" => Some(Self::Domain(DomainId::Religion)),
            "Universum" => Some(Self::Domain(DomainId::Universum)),
            "Planet_(10_und_oder_12)" => Some(Self::Domain(DomainId::Planet10Oder12)),
            "Grundstrukturen" => Some(Self::Grundstrukturen),
            "Eigenschaften_n" => Some(Self::EigenschaftFamilie(HtmlEigenschaftFamilie::N)),
            "Eigenschaften_1/n" => Some(Self::EigenschaftFamilie(HtmlEigenschaftFamilie::EinsDurchN)),
            _ => None,
        }
    }

    fn render(&self) -> &'static str {
        match self {
            Self::Zaehlung => "Zählung",
            Self::Nummerierung => "Nummerierung",
            Self::Domain(DomainId::Menschliches) => "Menschliches",
            Self::Domain(DomainId::Religion) => "Religion",
            Self::Domain(DomainId::Universum) => "Universum",
            Self::Domain(DomainId::Planet10Oder12) => "Planet_(10_und_oder_12)",
            Self::Grundstrukturen => "Grundstrukturen",
            Self::EigenschaftFamilie(HtmlEigenschaftFamilie::N) => "Eigenschaften_n",
            Self::EigenschaftFamilie(HtmlEigenschaftFamilie::EinsDurchN) => "Eigenschaften_1/n",
            Self::Domain(_) => "Menschliches",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlP2Slot {
    Empty,
    Eigenschaft(EigenschaftKeyId),
    Label(HtmlSlotLabel),
}

impl HtmlP2Slot {
    fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw.is_empty() || raw == "E" || raw == "e" {
            return Some(Self::Empty);
        }
        if let Some(key) = EigenschaftKeyId::from_alias(raw) {
            return Some(Self::Eigenschaft(key));
        }
        Some(Self::Label(HtmlSlotLabel::parse(raw)?))
    }

    fn render(&self) -> String {
        match self {
            Self::Empty => String::new(),
            Self::Eigenschaft(key) => key.canonical_name().to_string(),
            Self::Label(label) => label.render().to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlSlotLabel {
    Gesellschaftsschicht,
    Klassen20,
    Gewalt,
    Politische,
    Richtungen,
    Formationen,
    GleichheitFreiheitOrdnung,
    GleichheitFreiheit,
    OrdnungUndFilterung12Und1pro12,
    Geist15,
    Religion,
    Primzahlkreuz,
    Liebe,
    Liebe7,
}

impl HtmlSlotLabel {
    fn parse(raw: &str) -> Option<Self> {
        match raw.trim() {
            "Gesellschaftsschicht" => Some(Self::Gesellschaftsschicht),
            "Klassen_(20)" => Some(Self::Klassen20),
            "Gewalt" => Some(Self::Gewalt),
            "politische" => Some(Self::Politische),
            "Richtungen" => Some(Self::Richtungen),
            "Formationen" => Some(Self::Formationen),
            "Gleichheit_Freiheit_Ordnung" => Some(Self::GleichheitFreiheitOrdnung),
            "Gleichheit_Freiheit" => Some(Self::GleichheitFreiheit),
            "Ordnung_und_Filterung_12_und_1pro12" => Some(Self::OrdnungUndFilterung12Und1pro12),
            "Geist__(15)" => Some(Self::Geist15),
            "Religion" => Some(Self::Religion),
            "Primzahlkreuz" => Some(Self::Primzahlkreuz),
            "Liebe" => Some(Self::Liebe),
            "Liebe_(7)" => Some(Self::Liebe7),
            _ => None,
        }
    }

    fn render(&self) -> &'static str {
        match self {
            Self::Gesellschaftsschicht => "Gesellschaftsschicht",
            Self::Klassen20 => "Klassen_(20)",
            Self::Gewalt => "Gewalt",
            Self::Politische => "politische",
            Self::Richtungen => "Richtungen",
            Self::Formationen => "Formationen",
            Self::GleichheitFreiheitOrdnung => "Gleichheit_Freiheit_Ordnung",
            Self::GleichheitFreiheit => "Gleichheit_Freiheit",
            Self::OrdnungUndFilterung12Und1pro12 => "Ordnung_und_Filterung_12_und_1pro12",
            Self::Geist15 => "Geist__(15)",
            Self::Religion => "Religion",
            Self::Primzahlkreuz => "Primzahlkreuz",
            Self::Liebe => "Liebe",
            Self::Liebe7 => "Liebe_(7)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HtmlP4Tag(u8);

impl HtmlP4Tag {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    fn parse(raw: &str) -> Option<Self> {
        Some(Self(raw.trim().parse::<u8>().ok()?))
    }

    fn render(self) -> u8 {
        self.0
    }
}

fn parse_p1_groups(raw: &str) -> Option<Vec<HtmlP1Group>> {
    let groups: Vec<HtmlP1Group> = raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start_matches('✗'))
        .map(HtmlP1Group::parse)
        .collect::<Option<Vec<_>>>()?;
    Some(groups)
}

fn parse_p2_slots(raw: &str) -> Option<Vec<HtmlP2Slot>> {
    if raw.is_empty() {
        return Some(vec![HtmlP2Slot::Empty]);
    }

    let mut entries: Vec<(usize, HtmlP2Slot)> = Vec::new();
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
        let value = HtmlP2Slot::parse(value_part)?;
        entries.push((idx, value));
    }

    let mut out = vec![HtmlP2Slot::Empty; max_idx + 1];
    for (idx, value) in entries {
        out[idx] = value;
    }
    Some(out)
}

fn parse_p4_tags(raw: &str) -> Option<Vec<HtmlP4Tag>> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(HtmlP4Tag::parse)
        .collect::<Option<Vec<_>>>()
}

fn render_p1_groups(groups: &[HtmlP1Group]) -> String {
    let mut out = String::from("p1_");
    for g in groups {
        out.push('✗');
        out.push_str(g.render());
        out.push(',');
    }
    out.push(',');
    out
}

fn render_p2_slots(slots: &[HtmlP2Slot]) -> String {
    let mut out = String::from("p2_p3_");
    for (i, slot) in slots.iter().enumerate() {
        if i == 0 {
            out.push('0');
            out.push('_');
            out.push_str(&slot.render());
        } else {
            out.push(',');
            out.push_str("p3_");
            out.push_str(&i.to_string());
            out.push('_');
            out.push_str(&slot.render());
        }
    }
    out
}

fn render_p4_tags(tags: &[HtmlP4Tag]) -> String {
    let mut out = String::from("p4_");
    for (i, tag) in tags.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&tag.render().to_string());
    }
    out
}
