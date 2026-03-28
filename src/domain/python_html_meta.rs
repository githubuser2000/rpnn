use crate::domain::eigenschaften::EigenschaftKeyId;
use crate::domain::ids::domain_id::DomainId;
use crate::domain::model::spalten_anfrage::{
    EigenschaftsFamilie, SpaltenAnfrage, StandardUnterId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PythonHtmlMeta {
    pub css_class: &'static str,
}

pub fn meta_for_request(request: &SpaltenAnfrage) -> Option<PythonHtmlMeta> {
    let css_class = match request {
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Klasse } => {
            "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gesellschaftsschicht,p3_1_Klassen_(20), p4_3,5,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Liebe } => {
            "p1_✗Menschliches,✗Grundstrukturen,✗Eigenschaften_n,, p2_p3_0_Liebe,p3_1_Liebe_(7), p4_3,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Gewalt } => {
            "p1_✗Menschliches,, p2_p3_0_Gewalt, p4_3,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Politische } => {
            "p1_✗Menschliches,, p2_p3_0_politische, p4_3,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Richtungen } => {
            "p1_✗Menschliches,, p2_p3_0_Richtungen, p4_3,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Formationen } => {
            "p1_✗Menschliches,, p2_p3_0_Formationen, p4_3,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Planet10Oder12, unter: StandardUnterId::Eigenschaft(req) }
            if req.key == EigenschaftKeyId::Gleichheit => {
            "p1_✗Planet_(10_und_oder_12),✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_Gleichheit_Freiheit,p3_2_Ordnung_und_Filterung_12_und_1pro12, p4_4,5,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Menschliches, unter: StandardUnterId::Eigenschaft(req) }
            if req.key == EigenschaftKeyId::Gleichheit => {
            "p1_✗Planet_(10_und_oder_12),✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_Gleichheit_Freiheit,p3_2_Ordnung_und_Filterung_12_und_1pro12, p4_4,5,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Universum, unter: StandardUnterId::Geist } => {
            "p1_✗Universum,, p2_p3_0_Geist__(15), p4_3,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Universum, unter: StandardUnterId::Primzahlkreuz } => {
            "p1_✗Universum,, p2_p3_0_Primzahlkreuz, p4_3,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Religion, unter: StandardUnterId::SymboleReligion } => {
            "p1_✗Religion,, p2_p3_0_Religion, p4_3,0"
        }
        SpaltenAnfrage::Standard { domain: DomainId::Eigenschaften1ProN, unter: StandardUnterId::Eigenschaft(req) }
            if req.key == EigenschaftKeyId::ToleranzRespektAkzeptanzWillkommen
                && req.familie == EigenschaftsFamilie::EinsDurchN => {
            "p1_✗Eigenschaften_1/n,, p2_p3_0_Toleranz_Respekt_Akzeptanz_Willkommen, p4_3,0"
        }
        _ => return None,
    };

    Some(PythonHtmlMeta { css_class })
}

pub fn css_class_for_request(request: &SpaltenAnfrage) -> Option<&'static str> {
    meta_for_request(request).map(|m| m.css_class)
}

pub fn css_class_for_visible_header(header: &str) -> Option<&'static str> {
    match header.trim() {
        "Gesellschaftsklassen (20), welche aus den Paradigmen (13) hervorgehen" => Some(
            "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gesellschaftsschicht,p3_1_Klassen_(20), p4_3,5,0",
        ),
        "Gleichheit oder Ungleichheit als Ordnung und Freiheit oder nicht als Unordnung (12)" => Some(
            "p1_✗Planet_(10_und_oder_12),✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_Gleichheit_Freiheit,p3_2_Ordnung_und_Filterung_12_und_1pro12, p4_4,5,0",
        ),
        "Geist" | "Geist__(15)" => Some("p1_✗Universum,, p2_p3_0_Geist__(15), p4_3,0"),
        _ => None,
    }
}
