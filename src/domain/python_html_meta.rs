use crate::domain::decl_model::{
    HtmlDeclMeta, HtmlEigenschaftFamilie, HtmlP1Group, HtmlP2Slot, HtmlP4Tag, HtmlSlotLabel,
};
use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::DomainId;
use crate::domain::model::spalten_anfrage::{
    EigenschaftsFamilie, SpaltenAnfrage, StandardUnterId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonHtmlMeta {
    pub decl: HtmlDeclMeta,
}

fn p4(values: &[u8]) -> Vec<HtmlP4Tag> {
    values.iter().copied().map(HtmlP4Tag::new).collect()
}

fn decl(
    p1_groups: Vec<HtmlP1Group>,
    p2_slots: Vec<HtmlP2Slot>,
    p4_tags: &[u8],
) -> PythonHtmlMeta {
    PythonHtmlMeta {
        decl: HtmlDeclMeta {
            p1_groups,
            p2_slots,
            p4_tags: p4(p4_tags),
        },
    }
}

pub fn meta_for_request(request: &SpaltenAnfrage) -> Option<PythonHtmlMeta> {
    let meta = match request {
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Klasse } => {
            decl(
                vec![HtmlP1Group::Domain(DomainId::Menschliches), HtmlP1Group::Grundstrukturen],
                vec![
                    HtmlP2Slot::Label(HtmlSlotLabel::Gesellschaftsschicht),
                    HtmlP2Slot::Label(HtmlSlotLabel::Klassen20),
                ],
                &[3, 5, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Liebe } => {
            decl(
                vec![
                    HtmlP1Group::Domain(DomainId::Menschliches),
                    HtmlP1Group::Grundstrukturen,
                    HtmlP1Group::EigenschaftFamilie(HtmlEigenschaftFamilie::N),
                ],
                vec![
                    HtmlP2Slot::Label(HtmlSlotLabel::Liebe),
                    HtmlP2Slot::Label(HtmlSlotLabel::Liebe7),
                ],
                &[3, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Gewalt } => {
            decl(
                vec![HtmlP1Group::Domain(DomainId::Menschliches)],
                vec![HtmlP2Slot::Label(HtmlSlotLabel::Gewalt)],
                &[3, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Politische } => {
            decl(
                vec![HtmlP1Group::Domain(DomainId::Menschliches)],
                vec![HtmlP2Slot::Label(HtmlSlotLabel::Politische)],
                &[3, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Richtungen } => {
            decl(
                vec![HtmlP1Group::Domain(DomainId::Menschliches)],
                vec![HtmlP2Slot::Label(HtmlSlotLabel::Richtungen)],
                &[3, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Formationen } => {
            decl(
                vec![HtmlP1Group::Domain(DomainId::Menschliches)],
                vec![HtmlP2Slot::Label(HtmlSlotLabel::Formationen)],
                &[3, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Planet10Oder12, unter: StandardUnterId::Eigenschaft(req) }
            if req.key == EigenschaftKeyId::Gleichheit => {
            decl(
                vec![
                    HtmlP1Group::Domain(DomainId::Planet10Oder12),
                    HtmlP1Group::Domain(DomainId::Menschliches),
                    HtmlP1Group::Grundstrukturen,
                ],
                vec![
                    HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheitOrdnung),
                    HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheit),
                    HtmlP2Slot::Label(HtmlSlotLabel::OrdnungUndFilterung12Und1pro12),
                ],
                &[4, 5, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Eigenschaft(req) }
            if req.key == EigenschaftKeyId::Gleichheit => {
            decl(
                vec![
                    HtmlP1Group::Domain(DomainId::Planet10Oder12),
                    HtmlP1Group::Domain(DomainId::Menschliches),
                    HtmlP1Group::Grundstrukturen,
                ],
                vec![
                    HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheitOrdnung),
                    HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheit),
                    HtmlP2Slot::Label(HtmlSlotLabel::OrdnungUndFilterung12Und1pro12),
                ],
                &[4, 5, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Universum, unter: StandardUnterId::Geist } => {
            decl(
                vec![HtmlP1Group::Domain(DomainId::Universum)],
                vec![HtmlP2Slot::Label(HtmlSlotLabel::Geist15)],
                &[3, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Universum, unter: StandardUnterId::Primzahlkreuz } => {
            decl(
                vec![HtmlP1Group::Domain(DomainId::Universum)],
                vec![HtmlP2Slot::Label(HtmlSlotLabel::Primzahlkreuz)],
                &[3, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Religion, unter: StandardUnterId::SymboleReligion } => {
            decl(
                vec![HtmlP1Group::Domain(DomainId::Religion)],
                vec![HtmlP2Slot::Label(HtmlSlotLabel::Religion)],
                &[3, 0],
            )
        }
        SpaltenAnfrage::Standard { domain: DomainId::Eigenschaften1ProN, unter: StandardUnterId::Eigenschaft(req) }
            if req.key == EigenschaftKeyId::ToleranzRespektAkzeptanzWillkommen
                && req.familie == EigenschaftsFamilie::EinsDurchN => {
            decl(
                vec![HtmlP1Group::EigenschaftFamilie(HtmlEigenschaftFamilie::EinsDurchN)],
                vec![HtmlP2Slot::Eigenschaft(EigenschaftKeyId::ToleranzRespektAkzeptanzWillkommen)],
                &[3, 0],
            )
        }
        _ => return None,
    };

    Some(meta)
}

pub fn decl_for_request(request: &SpaltenAnfrage) -> Option<HtmlDeclMeta> {
    meta_for_request(request).map(|m| m.decl)
}

pub fn css_class_for_request(request: &SpaltenAnfrage) -> Option<String> {
    decl_for_request(request).map(|decl| decl.render())
}

pub fn css_class_for_visible_header(header: &str) -> Option<String> {
    match header.trim() {
        "Gesellschaftsklassen (20), welche aus den Paradigmen (13) hervorgehen" => Some(
            decl(
                vec![HtmlP1Group::Domain(DomainId::Menschliches), HtmlP1Group::Grundstrukturen],
                vec![
                    HtmlP2Slot::Label(HtmlSlotLabel::Gesellschaftsschicht),
                    HtmlP2Slot::Label(HtmlSlotLabel::Klassen20),
                ],
                &[3, 5, 0],
            )
            .decl
            .render(),
        ),
        "Gleichheit oder Ungleichheit als Ordnung und Freiheit oder nicht als Unordnung (12)" => Some(
            decl(
                vec![
                    HtmlP1Group::Domain(DomainId::Planet10Oder12),
                    HtmlP1Group::Domain(DomainId::Menschliches),
                    HtmlP1Group::Grundstrukturen,
                ],
                vec![
                    HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheitOrdnung),
                    HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheit),
                    HtmlP2Slot::Label(HtmlSlotLabel::OrdnungUndFilterung12Und1pro12),
                ],
                &[4, 5, 0],
            )
            .decl
            .render(),
        ),
        "Geist" | "Geist__(15)" => Some(
            decl(
                vec![HtmlP1Group::Domain(DomainId::Universum)],
                vec![HtmlP2Slot::Label(HtmlSlotLabel::Geist15)],
                &[3, 0],
            )
            .decl
            .render(),
        ),
        _ => None,
    }
}
