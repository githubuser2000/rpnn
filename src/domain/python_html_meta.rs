use crate::domain::decl_model::{
    HtmlDeclMeta, HtmlEigenschaftFamilie, HtmlP1Group, HtmlP2Slot, HtmlP4Tag, HtmlSlotLabel,
};
use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::DomainId;
use crate::domain::model::spalten_anfrage::{
    EigenschaftsFamilie, SpaltenAnfrage, StandardUnterId,
};

fn counter_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![HtmlP1Group::Zaehlung],
        p2_slots: vec![HtmlP2Slot::Empty],
        p4_tags: vec![],
    }
}

fn numbering_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![HtmlP1Group::Nummerierung],
        p2_slots: vec![HtmlP2Slot::Empty],
        p4_tags: vec![],
    }
}

fn klasse_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![
            HtmlP1Group::Domain(DomainId::Menschliches),
            HtmlP1Group::Grundstrukturen,
        ],
        p2_slots: vec![
            HtmlP2Slot::Label(HtmlSlotLabel::Gesellschaftsschicht),
            HtmlP2Slot::Label(HtmlSlotLabel::Klassen20),
        ],
        p4_tags: vec![HtmlP4Tag::new(3), HtmlP4Tag::new(5), HtmlP4Tag::new(0)],
    }
}

fn liebe_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![
            HtmlP1Group::Domain(DomainId::Menschliches),
            HtmlP1Group::Grundstrukturen,
            HtmlP1Group::EigenschaftFamilie(HtmlEigenschaftFamilie::N),
        ],
        p2_slots: vec![HtmlP2Slot::Eigenschaft(EigenschaftKeyId::LiebeUsw), HtmlP2Slot::Empty],
        p4_tags: vec![HtmlP4Tag::new(3), HtmlP4Tag::new(0)],
    }
}

fn gewalt_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![HtmlP1Group::Domain(DomainId::Menschliches)],
        p2_slots: vec![HtmlP2Slot::Label(HtmlSlotLabel::Gewalt)],
        p4_tags: vec![HtmlP4Tag::new(3), HtmlP4Tag::new(0)],
    }
}

fn richtungen_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![HtmlP1Group::Domain(DomainId::Menschliches)],
        p2_slots: vec![
            HtmlP2Slot::Label(HtmlSlotLabel::Politische),
            HtmlP2Slot::Label(HtmlSlotLabel::Richtungen),
        ],
        p4_tags: vec![HtmlP4Tag::new(3), HtmlP4Tag::new(0)],
    }
}

fn formationen_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![HtmlP1Group::Domain(DomainId::Menschliches)],
        p2_slots: vec![HtmlP2Slot::Label(HtmlSlotLabel::Formationen), HtmlP2Slot::Empty],
        p4_tags: vec![HtmlP4Tag::new(0), HtmlP4Tag::new(5)],
    }
}

fn geist_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![HtmlP1Group::Domain(DomainId::Universum)],
        p2_slots: vec![HtmlP2Slot::Label(HtmlSlotLabel::Geist15)],
        p4_tags: vec![HtmlP4Tag::new(3), HtmlP4Tag::new(0)],
    }
}

fn symbole_religion_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![HtmlP1Group::Domain(DomainId::Religion)],
        p2_slots: vec![HtmlP2Slot::Label(HtmlSlotLabel::Religion)],
        p4_tags: vec![HtmlP4Tag::new(3), HtmlP4Tag::new(0)],
    }
}

fn toleranz_1n_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![HtmlP1Group::EigenschaftFamilie(HtmlEigenschaftFamilie::EinsDurchN)],
        p2_slots: vec![
            HtmlP2Slot::Eigenschaft(EigenschaftKeyId::ToleranzRespektAkzeptanzWillkommen),
            HtmlP2Slot::Empty,
        ],
        p4_tags: vec![HtmlP4Tag::new(3), HtmlP4Tag::new(0)],
    }
}

fn planet_gleichheit_decl() -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: vec![
            HtmlP1Group::Domain(DomainId::Planet10Oder12),
            HtmlP1Group::Domain(DomainId::Menschliches),
            HtmlP1Group::Grundstrukturen,
        ],
        p2_slots: vec![
            HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheitOrdnung),
            HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheit),
            HtmlP2Slot::Label(HtmlSlotLabel::OrdnungUndFilterung12Und1pro12),
        ],
        p4_tags: vec![HtmlP4Tag::new(4), HtmlP4Tag::new(5), HtmlP4Tag::new(0)],
    }
}

pub fn decl_for_request(request: &SpaltenAnfrage) -> Option<HtmlDeclMeta> {
    match request {
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Klasse } => Some(klasse_decl()),
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Liebe } => Some(liebe_decl()),
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Gewalt } => Some(gewalt_decl()),
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Richtungen } => Some(richtungen_decl()),
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Formationen } => Some(formationen_decl()),
        SpaltenAnfrage::Standard { domain: DomainId::Universum, unter: StandardUnterId::Geist } => Some(geist_decl()),
        SpaltenAnfrage::Standard { domain: DomainId::Religion, unter: StandardUnterId::SymboleReligion } => Some(symbole_religion_decl()),
        SpaltenAnfrage::Standard { domain: DomainId::Eigenschaften1ProN, unter: StandardUnterId::Eigenschaft(req) }
            if req.key == EigenschaftKeyId::ToleranzRespektAkzeptanzWillkommen
                && req.familie == EigenschaftsFamilie::EinsDurchN => Some(toleranz_1n_decl()),
        _ => None,
    }
}

pub fn css_class_for_request(request: &SpaltenAnfrage) -> Option<String> {
    decl_for_request(request).map(|m| m.render())
}

pub fn decl_for_visible_header(header: &str) -> Option<HtmlDeclMeta> {
    match header.trim() {
        "" => Some(counter_decl()),
        "P" | "Z" => Some(numbering_decl()),
        "Gesellschaftsklassen (20), welche aus den Paradigmen (13) hervorgehen" => Some(klasse_decl()),
        "Gleichheit oder Ungleichheit als Ordnung und Freiheit oder nicht als Unordnung (12)" => Some(planet_gleichheit_decl()),
        "Geist" | "Geist__(15)" => Some(geist_decl()),
        _ => None,
    }
}

pub fn css_class_for_visible_header(header: &str) -> Option<String> {
    decl_for_visible_header(header).map(|m| m.render())
}
