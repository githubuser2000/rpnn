use crate::domain::decl_model::HtmlDeclMeta;
use crate::domain::spalten_anfrage::{
    MenschlichesUnter, ReligionUnter, SpaltenAnfrage, StandardAnfrage, StandardOberkategorie,
    UniversumUnter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PythonHtmlMeta {
    pub css_class: &'static str,
}

pub fn meta_for_request(request: &SpaltenAnfrage) -> Option<PythonHtmlMeta> {
    let css_class = match request {
        SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::Klasse)) => {
            "p1_✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gesellschaftsschicht,p3_1_Klassen_(20), p4_3,5,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::Liebe)) => {
            "p1_✗Menschliches,✗Grundstrukturen,✗Eigenschaften_n,, p2_p3_0_Liebe,p3_1_Liebe_(7), p4_3,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::Gleichheit)) => {
            "p1_✗Planet_(10_und_oder_12),✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_Gleichheit_Freiheit,p3_2_Ordnung_und_Filterung_12_und_1pro12, p4_4,5,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::Gewalt)) => {
            "p1_✗Menschliches,, p2_p3_0_Gewalt, p4_3,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::Politische)) => {
            "p1_✗Menschliches,, p2_p3_0_politische, p4_3,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::Richtungen)) => {
            "p1_✗Menschliches,, p2_p3_0_Richtungen, p4_3,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::Formationen)) => {
            "p1_✗Menschliches,, p2_p3_0_Formationen, p4_3,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Menschliches(MenschlichesUnter::Motive)) => {
            "p1_✗Menschliches,, p2_p3_0_Motive, p4_3,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Sonstige {
            ober: StandardOberkategorie::Planet,
            unter,
        }) if unter == "Gleichheit" => {
            "p1_✗Planet_(10_und_oder_12),✗Menschliches,✗Grundstrukturen,, p2_p3_0_Gleichheit_Freiheit_Ordnung,p3_1_Gleichheit_Freiheit,p3_2_Ordnung_und_Filterung_12_und_1pro12, p4_4,5,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Universum(UniversumUnter::Geist)) => {
            "p1_✗Universum,, p2_p3_0_Geist__(15), p4_3,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Universum(UniversumUnter::Primzahlkreuz)) => {
            "p1_✗Universum,, p2_p3_0_Primzahlkreuz, p4_3,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Religion(ReligionUnter::Religion)) => {
            "p1_✗Religion,, p2_p3_0_Religion, p4_3,0"
        }
        SpaltenAnfrage::Standard(StandardAnfrage::Sonstige {
            ober: StandardOberkategorie::Eigenschaften1ProN,
            unter,
        }) if unter == "Toleranz_Respekt_Akzeptanz_Willkommen" => {
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


pub fn decl_for_request(request: &SpaltenAnfrage) -> Option<HtmlDeclMeta> {
    let css = css_class_for_request(request)?;
    HtmlDeclMeta::parse(css)
}

pub fn decl_for_visible_header(header: &str) -> Option<HtmlDeclMeta> {
    let css = css_class_for_visible_header(header)?;
    HtmlDeclMeta::parse(css)
}
