use crate::domain::decl_model::{HtmlDeclMeta, HtmlP1Group, HtmlP2Slot, HtmlP4Tag, HtmlSlotLabel};
use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::DomainId;

fn mk(
    p1_groups: Vec<HtmlP1Group>,
    p2_slots: Vec<HtmlP2Slot>,
    p4_tags: Vec<HtmlP4Tag>,
) -> HtmlDeclMeta {
    HtmlDeclMeta { p1_groups, p2_slots, p4_tags }
}

pub fn typed_exact_decl_for_column(col: u32) -> Option<HtmlDeclMeta> {
    match col {
        8 | 9 | 28 => Some(mk(
            vec![
                HtmlP1Group::Domain(DomainId::Menschliches),
                HtmlP1Group::Grundstrukturen,
            ],
            vec![
                HtmlP2Slot::Eigenschaft(EigenschaftKeyId::LiebeUsw),
                HtmlP2Slot::Label(HtmlSlotLabel::Liebe7),
                HtmlP2Slot::Empty,
            ],
            vec![HtmlP4Tag::new(0), HtmlP4Tag::new(5)],
        )),
        466 => Some(mk(
            vec![HtmlP1Group::Domain(DomainId::Menschliches)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Gewalt)],
            vec![HtmlP4Tag::new(3), HtmlP4Tag::new(0)],
        )),
        _ => None,
    }
}

pub fn all_typed_exact_decl() -> Vec<(u32, HtmlDeclMeta)> {
    let keys = [8u32, 9u32, 28u32, 466u32];
    keys.into_iter()
        .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
