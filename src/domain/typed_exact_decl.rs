use crate::domain::decl_model::{
    HtmlDeclMeta, HtmlEigenschaftFamilie, HtmlP1Group, HtmlP2Slot, HtmlP4Tag, HtmlSlotLabel,
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
        0 => Some(decl(
            vec![
                HtmlP1Group::Label("Wichtigstes_zum_gedanklich_einordnen"),
                HtmlP1Group::Domain(DomainId::Religion),
                HtmlP1Group::Domain(DomainId::Religion),
                HtmlP1Group::Domain(DomainId::Galaxie),
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Wichtigste),
                HtmlP2Slot::Label(HtmlSlotLabel::Sternpolygon),
                HtmlP2Slot::Label(HtmlSlotLabel::DerTierkreiszeichen),
                HtmlP2Slot::Label(HtmlSlotLabel::Thomasevangelium),
                HtmlP2Slot::Empty,
            ],
            &[3, 0],
        )),
        1 | 2 => Some(decl(
            vec![
                HtmlP1Group::Label("Wichtigstes_zum_gedanklich_einordnen"),
                HtmlP1Group::Domain(DomainId::Galaxie),
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Wichtigste),
                HtmlP2Slot::Label(HtmlSlotLabel::BabylonischeTierkreiszeichen),
                HtmlP2Slot::Empty,
            ],
            &[3, 0],
        )),
        3 => Some(decl(
            vec![HtmlP1Group::Domain(DomainId::Galaxie)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Thomasevangelium), HtmlP2Slot::Empty],
            &[3, 0],
        )),
        10 => Some(decl(
            vec![
                HtmlP1Group::Label("Wichtigstes_zum_verstehen"),
                HtmlP1Group::Grundstrukturen,
                HtmlP1Group::Domain(DomainId::Menschliches),
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Wichtigste),
                HtmlP2Slot::Label(HtmlSlotLabel::Motive),
                HtmlP2Slot::Empty,
            ],
            &[3, 0],
        )),
        19 => Some(decl(
            vec![
                HtmlP1Group::Label("Wichtigstes_zum_verstehen"),
                HtmlP1Group::Label("Bedeutung"),
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Zweitwichtigste),
                HtmlP2Slot::Label(HtmlSlotLabel::Primzahlen),
                HtmlP2Slot::Empty,
            ],
            &[3, 0],
        )),
        21 => Some(decl(
            vec![HtmlP1Group::Grundstrukturen, HtmlP1Group::Label("Größenordnung")],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Strukturgroesse),
                HtmlP2Slot::Label(HtmlSlotLabel::Strukturgroesse),
                HtmlP2Slot::Empty,
            ],
            &[3, 4, 0, 5],
        )),
        22 => Some(decl(
            vec![HtmlP1Group::Label("Bedeutung")],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::AnwendungDerSonnenUndMonde), HtmlP2Slot::Empty],
            &[3, 0],
        )),
        25 => Some(decl(
            vec![
                HtmlP1Group::Domain(DomainId::Universum),
                HtmlP1Group::Label("Bedeutung"),
                HtmlP1Group::Domain(DomainId::Kontinuum),
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Netzwerk),
                HtmlP2Slot::Label(HtmlSlotLabel::Zaehlungen),
                HtmlP2Slot::Label(HtmlSlotLabel::X),
                HtmlP2Slot::Empty,
            ],
            &[4, 0],
        )),
        29 => Some(decl(
            vec![HtmlP1Group::Grundstrukturen, HtmlP1Group::Domain(DomainId::Menschliches)],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Gefuehle7),
                HtmlP2Slot::Label(HtmlSlotLabel::Anfuehrer),
                HtmlP2Slot::Empty,
            ],
            &[3, 0],
        )),
        30 => Some(decl(
            vec![
                HtmlP1Group::Label("Wichtigstes_zum_gedanklich_einordnen"),
                HtmlP1Group::Label("Größenordnung"),
                HtmlP1Group::Domain(DomainId::Menschliches),
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Zweitwichtigste),
                HtmlP2Slot::Label(HtmlSlotLabel::Organisationen),
                HtmlP2Slot::Label(HtmlSlotLabel::Berufe),
                HtmlP2Slot::Empty,
            ],
            &[3, 0],
        )),
        31 => Some(decl(
            vec![HtmlP1Group::Domain(DomainId::Menschliches)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Loesungen), HtmlP2Slot::Empty],
            &[3, 0],
        )),
        34 => Some(decl(
            vec![HtmlP1Group::Domain(DomainId::Universum), HtmlP1Group::Label("Bedeutung")],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::UniversellesRecht),
                HtmlP2Slot::Label(HtmlSlotLabel::Jura),
                HtmlP2Slot::Empty,
            ],
            &[3, 0],
        )),
        35 => Some(decl(
            vec![HtmlP1Group::Label("Bedeutung")],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::VollkommenheitDesGeistes), HtmlP2Slot::Empty],
            &[3, 0],
        )),
        36 => Some(decl(
            vec![
                HtmlP1Group::Label("Wichtigstes_zum_gedanklich_einordnen"),
                HtmlP1Group::Domain(DomainId::Religion),
                HtmlP1Group::Domain(DomainId::Religion),
                HtmlP1Group::Label("Symbole"),
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Wichtigste),
                HtmlP2Slot::Label(HtmlSlotLabel::Sternpolygon),
                HtmlP2Slot::Label(HtmlSlotLabel::DerTierkreiszeichen),
                HtmlP2Slot::Label(HtmlSlotLabel::Religionen),
                HtmlP2Slot::Empty,
            ],
            &[3, 4, 0, 5],
        )),
        37 => Some(decl(
            vec![
                HtmlP1Group::Label("Wichtigstes_zum_gedanklich_einordnen"),
                HtmlP1Group::Domain(DomainId::Religion),
                HtmlP1Group::Label("Symbole"),
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Wichtigste),
                HtmlP2Slot::Label(HtmlSlotLabel::GleichfoermigesPolygon),
                HtmlP2Slot::Label(HtmlSlotLabel::Religionen),
                HtmlP2Slot::Empty,
            ],
            &[3, 5, 1, 4],
        )),
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
        52 | 53 => Some(decl(
            vec![HtmlP1Group::EigenschaftFamilie(HtmlEigenschaftFamilie::N)],
            vec![
                HtmlP2Slot::Eigenschaft(EigenschaftKeyId::GutBoeseLiebSchlecht),
                HtmlP2Slot::Empty,
            ],
            &[3, 1, 0],
        )),
        112 => Some(decl(
            vec![HtmlP1Group::EigenschaftFamilie(HtmlEigenschaftFamilie::N)],
            vec![
                HtmlP2Slot::Eigenschaft(EigenschaftKeyId::WeisheitEtc),
                HtmlP2Slot::Empty,
            ],
            &[3, 0],
        )),
        132 => Some(decl(
            vec![
                HtmlP1Group::Domain(DomainId::Planet10Oder12),
                HtmlP1Group::Domain(DomainId::Menschliches),
                HtmlP1Group::Grundstrukturen,
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheitOrdnung),
                HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheit),
                HtmlP2Slot::Label(HtmlSlotLabel::OrdnungUndFilterung12Und1pro12),
                HtmlP2Slot::Empty,
            ],
            &[4, 0, 5],
        )),
        220 => Some(decl(
            vec![HtmlP1Group::EigenschaftFamilie(HtmlEigenschaftFamilie::N)],
            vec![
                HtmlP2Slot::Eigenschaft(EigenschaftKeyId::Aehnlich),
                HtmlP2Slot::Empty,
            ],
            &[3, 4, 0],
        )),
        221 => Some(decl(
            vec![HtmlP1Group::Grundstrukturen],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Liebe7), HtmlP2Slot::Empty],
            &[],
        )),
        241 => Some(decl(
            vec![
                HtmlP1Group::Domain(DomainId::Menschliches),
                HtmlP1Group::Grundstrukturen,
            ],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::Gesellschaftsschicht),
                HtmlP2Slot::Label(HtmlSlotLabel::Klassen20),
                HtmlP2Slot::Empty,
            ],
            &[3, 0, 5],
        )),
        289 => Some(decl(
            vec![HtmlP1Group::Grundstrukturen],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Klassen20), HtmlP2Slot::Empty],
            &[0, 5],
        )),
        324 => Some(decl(
            vec![HtmlP1Group::Domain(DomainId::Planet10Oder12)],
            vec![
                HtmlP2Slot::Label(HtmlSlotLabel::GleichheitFreiheitOrdnung),
                HtmlP2Slot::Empty,
            ],
            &[0, 5],
        )),
        394 | 395 => Some(decl(
            vec![HtmlP1Group::Grundstrukturen],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Klassen20), HtmlP2Slot::Empty],
            &[3, 0, 5],
        )),
        461 => Some(decl(
            vec![HtmlP1Group::Domain(DomainId::Menschliches)],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Formationen), HtmlP2Slot::Empty],
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
        479 => Some(decl(
            vec![HtmlP1Group::Grundstrukturen],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Gewalt), HtmlP2Slot::Empty],
            &[4, 0],
        )),
        485 | 516 => Some(decl(
            vec![HtmlP1Group::Grundstrukturen],
            vec![HtmlP2Slot::Label(HtmlSlotLabel::Klassen20), HtmlP2Slot::Empty],
            &[0, 5],
        )),
        _ => None,
    }
}

pub fn all_typed_exact_decls() -> Vec<(u32, HtmlDeclMeta)> {
    [0u32, 1, 2, 3, 6, 7, 8, 9, 10, 19, 21, 22, 25, 29, 30, 31, 34, 35, 36, 37, 52, 53, 112, 132, 220, 221, 241, 289, 324, 394, 395, 461, 466, 479, 485, 516]
        .into_iter()
        .filter_map(|col| typed_exact_decl_for_column(col).map(|meta| (col, meta)))
        .collect()
}
