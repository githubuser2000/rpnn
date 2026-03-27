use crate::domain::ids::domain_id::{DomainId, GeneratorArt};
use crate::domain::model::spalten_anfrage::{
    CanonicalColumnSpec, ColumnTarget, GeneratorParameter, GeneratorSpec, SpaltenAnfrage,
    StandardUnterId,
};

/// Typisierte Brücke für exakte Generator-/Spezialfälle.
///
/// Wichtige Regel:
/// - keine CLI-String-Auflösung mehr hier
/// - keine Normalisierung mehr hier
/// - keine Aliaslogik mehr hier
/// - nur noch Match auf kanonische Typen
///
/// Der Parser muss bereits vorher aus Text -> SpaltenAnfrage gemacht haben.
pub fn resolve_exact_generator(req: &SpaltenAnfrage) -> Option<CanonicalColumnSpec> {
    match req {
        //
        // Eigenschaften_1/n / konzept2
        //
        SpaltenAnfrage::Standard {
            domain: DomainId::Eigenschaften1ProN,
            unter: StandardUnterId::Wuerdig,
        } => Some(spec_pair(
            req,
            358,
            359,
            "Würdig",
            &["Eigenschaften_1/n", "konzept2", "Würdig"],
        )),

        SpaltenAnfrage::Standard {
            domain: DomainId::Eigenschaften1ProN,
            unter: StandardUnterId::RegelVsAusnahme,
        } => Some(spec_pair(
            req,
            356,
            357,
            "Regel_vs_Ausnahme",
            &["Eigenschaften_1/n", "konzept2", "Regel_vs_Ausnahme"],
        )),

        SpaltenAnfrage::Standard {
            domain: DomainId::Eigenschaften1ProN,
            unter: StandardUnterId::FilterartWidrigkeit,
        } => Some(spec_pair(
            req,
            354,
            355,
            "Filterart_Widrigkeit",
            &["Eigenschaften_1/n", "konzept2", "Filterart_Widrigkeit"],
        )),

        SpaltenAnfrage::Standard {
            domain: DomainId::Eigenschaften1ProN,
            unter: StandardUnterId::Werte,
        } => Some(spec_pair(
            req,
            352,
            353,
            "Werte",
            &["Eigenschaften_1/n", "konzept2", "Werte"],
        )),

        SpaltenAnfrage::Standard {
            domain: DomainId::Eigenschaften1ProN,
            unter: StandardUnterId::GutartigkeitsEgoismus,
        } => Some(spec_pair(
            req,
            350,
            351,
            "Gutartigkeits-Egoismus",
            &["Eigenschaften_1/n", "konzept2", "Gutartigkeits-Egoismus"],
        )),

        SpaltenAnfrage::Standard {
            domain: DomainId::Eigenschaften1ProN,
            unter: StandardUnterId::ReflektierenErkenntnisErkennen,
        } => Some(spec_pair(
            req,
            348,
            349,
            "Reflektieren_Erkenntnis-Erkennen",
            &[
                "Eigenschaften_1/n",
                "konzept2",
                "Reflektieren_Erkenntnis-Erkennen",
            ],
        )),

        SpaltenAnfrage::Standard {
            domain: DomainId::Eigenschaften1ProN,
            unter: StandardUnterId::VertrauenWollen,
        } => Some(spec_pair(
            req,
            346,
            347,
            "Vertrauen_wollen",
            &["Eigenschaften_1/n", "konzept2", "Vertrauen_wollen"],
        )),

        SpaltenAnfrage::Standard {
            domain: DomainId::Eigenschaften1ProN,
            unter: StandardUnterId::AusrichtenEinrichten,
        } => Some(spec_pair(
            req,
            344,
            345,
            "Ausrichten_Einrichten",
            &["Eigenschaften_1/n", "konzept2", "Ausrichten_Einrichten"],
        )),

        SpaltenAnfrage::Standard {
            domain: DomainId::Eigenschaften1ProN,
            unter: StandardUnterId::ToleranzRespektAkzeptanzWillkommen,
        } => Some(spec_pair(
            req,
            62,
            63,
            "Toleranz_Respekt_Akzeptanz_Willkommen",
            &[
                "Eigenschaften_1/n",
                "konzept2",
                "Toleranz_Respekt_Akzeptanz_Willkommen",
            ],
        )),

        //
        // Standard-Unterkategorie, die in Wahrheit auf Generatoren abbildet
        //
        SpaltenAnfrage::Standard {
            unter: StandardUnterId::Primzahlkreuz,
            ..
        } => Some(spec_generator(
            req,
            GeneratorArt::Primzahlkreuz,
            GeneratorParameter::Keine,
            "Primzahlkreuz",
            &["Primzahlkreuz"],
        )),

        //
        // Direkte Generator-Anfragen
        //
        SpaltenAnfrage::Generator {
            art: GeneratorArt::Primzahlkreuz,
            parameter,
        } => Some(spec_generator(
            req,
            GeneratorArt::Primzahlkreuz,
            parameter.clone(),
            "Primzahlkreuz",
            &["Primzahlkreuz"],
        )),

        SpaltenAnfrage::Generator {
            art: GeneratorArt::Multiplikationen,
            parameter,
        } => Some(spec_generator(
            req,
            GeneratorArt::Multiplikationen,
            parameter.clone(),
            "Multiplikationen",
            &["Multiplikationen"],
        )),

        SpaltenAnfrage::Generator {
            art: GeneratorArt::Primvielfache,
            parameter,
        } => Some(spec_generator(
            req,
            GeneratorArt::Primvielfache,
            parameter.clone(),
            "Primvielfache",
            &["Primvielfache"],
        )),

        SpaltenAnfrage::Generator {
            art: GeneratorArt::MetaKonkret,
            parameter,
        } => Some(spec_generator(
            req,
            GeneratorArt::MetaKonkret,
            parameter.clone(),
            "MetaKonkret",
            &["MetaKonkret", "Universum_Metakonkret"],
        )),

        _ => None,
    }
}

fn spec_pair(
    req: &SpaltenAnfrage,
    left: u16,
    right: u16,
    header_display: &str,
    aliases: &[&str],
) -> CanonicalColumnSpec {
    CanonicalColumnSpec {
        request: req.clone(),
        target: ColumnTarget::Pair(left, right),
        header_display: header_display.to_string(),
        aliases_for_report: aliases.iter().map(|s| s.to_string()).collect(),
    }
}

fn spec_generator(
    req: &SpaltenAnfrage,
    art: GeneratorArt,
    parameter: GeneratorParameter,
    header_display: &str,
    aliases: &[&str],
) -> CanonicalColumnSpec {
    CanonicalColumnSpec {
        request: req.clone(),
        target: ColumnTarget::Generator(GeneratorSpec { art, parameter }),
        header_display: header_display.to_string(),
        aliases_for_report: aliases.iter().map(|s| s.to_string()).collect(),
    }
}
