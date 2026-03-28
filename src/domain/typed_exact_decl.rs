use crate::domain::decl_model::{
    HtmlDeclMeta, HtmlP1Group, HtmlP2Slot, HtmlP4Tag, HtmlSlotLabel,
};
use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::DomainId;

fn decl(
    p1_groups: Vec<HtmlP1Group>,
    p2_slots: Vec<HtmlP2Slot>,
    p4_tags: &[u8],
) -> HtmlDeclMeta {
    HtmlDeclMeta::new(
        p1_groups,
        p2_slots,
        p4_tags.iter().copied().map(HtmlP4Tag::new).collect(),
    )
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    match col {
        6 => Some(decl(
            vec![HtmlP1Group::Domain(DomainId::Religion)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Sternpolygon), HtmlP2Slot::Empty],
            &[3, 0],
        )),
        7 => Some(decl(
            vec![HtmlP1Group::Domain(DomainId::Religion)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Messias), HtmlP2Slot::Empty],
            &[3, 0],
        )),
        8 => Some(decl(
            vec![
                HtmlP1Group::Label("Wichtigstes_zum_verstehen"),
                HtmlP1Group::Domain(DomainId::Menschliches),
                HtmlP1Group::Grundstrukturen,
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Wichtigste),
                HtmlP2Slot::Eigenschaft(EigenschaftKeyId::LiebeUsw),
                HtmlP2Slot::Label(HtmlSlotLabel::Liebe7),
                HtmlP2Slot::Empty,
            ],
            &[0, 5],
        )),
        9 => Some(decl(
            vec![
                HtmlP1Group::Domain(DomainId::Menschliches),
                HtmlP1Group::Grundstrukturen,
            ],
            vec![
                HtmlP2Slot::Eigenschaft(EigenschaftKeyId::LiebeUsw),
                HtmlP2Slot::Label(HtmlSlotLabel::Liebe7),
                HtmlP2Slot::Empty,
            ],
            &[0, 5],
        )),
        466 => Some(decl(
            vec![
                HtmlP1Group::Domain(DomainId::Menschliches),
                HtmlP1Group::Grundstrukturen,
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Gewalt),
                HtmlP2Slot::Label(HtmlSlotLabel::Gewalt),
                HtmlP2Slot::Empty,
            ],
            &[3, 4, 0],
        )),
        _ => None,
    }
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    [6u32, 7, 8, 9, 466]
        .into_iter()
        .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
