use crate::domain::decl_model::{
    HtmlDeclMeta, HtmlEigenschaftFamilie, HtmlP1Group, HtmlP2Slot, HtmlP4Tag, HtmlSlotLabel,
};
use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::DomainId;

fn mk(groups: Vec<HtmlP1Group>, slots: Vec<HtmlP2Slot>, tags: &[u8]) -> HtmlDeclMeta {
    HtmlDeclMeta {
        p1_groups: groups,
        p2_slots: slots,
        p4_tags: tags.iter().copied().map(HtmlP4Tag::new).collect(),
    }
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    match col {
        8 => Some(mk(
            vec![
                HtmlP1Group::Domain(DomainId::Menschliches),
                HtmlP1Group::Grundstrukturen,
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Gesellschaftsschicht),
                HtmlP2Slot::Eigenschaft(EigenschaftKeyId::LiebeUsw),
                HtmlP2Slot::Label(HtmlSlotLabel::Religion),
                HtmlP2Slot::Empty,
            ],
            &[0, 5],
        )),
        9 | 28 => Some(mk(
            vec![
                HtmlP1Group::Domain(DomainId::Menschliches),
                HtmlP1Group::Grundstrukturen,
            ],
            vec![
                HtmlP2Slot::Eigenschaft(EigenschaftKeyId::LiebeUsw),
                HtmlP2Slot::Label(HtmlSlotLabel::Religion),
                HtmlP2Slot::Empty,
            ],
            &[0, 5],
        )),
        19 => Some(mk(
            vec![HtmlP1Group::Domain(DomainId::Menschliches)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Primzahlkreuz), HtmlP2Slot::Empty],
            &[3, 0],
        )),
        214 => Some(mk(
            vec![HtmlP1Group::Domain(DomainId::Planet10Oder12)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheitOrdnung)],
            &[3, 0],
        )),
        235 => Some(mk(
            vec![HtmlP1Group::Domain(DomainId::Menschliches)],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Politische),
                HtmlP2Slot::Label(HtmlSlotLabel::Richtungen),
            ],
            &[3, 0],
        )),
        461 => Some(mk(
            vec![HtmlP1Group::Domain(DomainId::Menschliches)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Formationen), HtmlP2Slot::Empty],
            &[0, 5],
        )),
        462 => Some(mk(
            vec![HtmlP1Group::Domain(DomainId::Universum)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Geist15)],
            &[3, 0],
        )),
        497 | 498 | 499 => Some(mk(
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
        )),
        524 => Some(mk(
            vec![HtmlP1Group::EigenschaftFamilie(HtmlEigenschaftFamilie::EinsDurchN)],
            vec![
                HtmlP2Slot::Eigenschaft(EigenschaftKeyId::ToleranzRespektAkzeptanzWillkommen),
                HtmlP2Slot::Empty,
            ],
            &[3, 0],
        )),
        _ => None,
    }
}

pub fn all_typed_exact_decl_meta() -> Vec<(u32, HtmlDeclMeta)> {
    [8u32, 9, 19, 28, 214, 235, 461, 462, 497, 498, 499, 524]
        .into_iter()
        .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
