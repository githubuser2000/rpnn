use crate::domain::ids::domain_id::{GebrochenRationalArt, GeneratorArt, KombinationsArt};
use crate::domain::model::spalten_anfrage::{
    CanonicalColumnSpec, ColumnTarget, CombinationSpec, GeneratorParameter, GeneratorSpec,
    KombiUnterId, SpaltenAnfrage, StandardUnterId,
};

pub fn resolve_request(req: SpaltenAnfrage) -> Option<CanonicalColumnSpec> {
    match &req {
        SpaltenAnfrage::Standard { unter, .. } => resolve_standard(req.clone(), *unter),
        SpaltenAnfrage::GebrochenRational { art, index } => {
            resolve_gebrochen_rational(req.clone(), *art, *index)
        }
        SpaltenAnfrage::Kombination { art, unter } => {
            resolve_kombination(req.clone(), *art, *unter)
        }
        SpaltenAnfrage::Generator { art, parameter } => {
            resolve_generator(req.clone(), *art, parameter.clone())
        }
        SpaltenAnfrage::DirektSpalten { ids } => {
            let ids_cloned = ids.clone();
            Some(CanonicalColumnSpec {
                request: req.clone(),
                target: ColumnTarget::DirectColumns(ids_cloned.clone()),
                header_display: format!("DirektSpalten {:?}", ids_cloned),
                aliases_for_report: vec![],
            })
        },
    }
}

fn resolve_standard(req: SpaltenAnfrage, unter: StandardUnterId) -> Option<CanonicalColumnSpec> {
    let (target, header_display) = match unter {
        StandardUnterId::Wuerdig => (ColumnTarget::Pair(358, 359), "Würdig".to_string()),
        StandardUnterId::RegelVsAusnahme => {
            (ColumnTarget::Pair(356, 357), "Regel_vs_Ausnahme".to_string())
        }
        StandardUnterId::FilterartWidrigkeit => {
            (ColumnTarget::Pair(354, 355), "Filterart_Widrigkeit".to_string())
        }
        StandardUnterId::Werte => (ColumnTarget::Pair(352, 353), "Werte".to_string()),
        StandardUnterId::GutartigkeitsEgoismus => {
            (ColumnTarget::Pair(350, 351), "Gutartigkeits-Egoismus".to_string())
        }
        StandardUnterId::ReflektierenErkenntnisErkennen => (
            ColumnTarget::Pair(348, 349),
            "Reflektieren_Erkenntnis-Erkennen".to_string(),
        ),
        StandardUnterId::VertrauenWollen => {
            (ColumnTarget::Pair(346, 347), "Vertrauen_wollen".to_string())
        }
        StandardUnterId::AusrichtenEinrichten => {
            (ColumnTarget::Pair(344, 345), "Ausrichten_Einrichten".to_string())
        }
        StandardUnterId::ToleranzRespektAkzeptanzWillkommen => (
            ColumnTarget::Pair(62, 63),
            "Toleranz_Respekt_Akzeptanz_Willkommen".to_string(),
        ),

        // Platzhalter/erste Brücke – diese IDs später gegen Python-Wahrheit austauschen
        StandardUnterId::Gewalt => (ColumnTarget::DirectColumn(496), "Gewalt".to_string()),
        StandardUnterId::Politische => (ColumnTarget::DirectColumn(497), "politische".to_string()),
        StandardUnterId::Richtungen => (ColumnTarget::DirectColumn(498), "Richtungen".to_string()),
        StandardUnterId::Formationen => {
            (ColumnTarget::DirectColumn(499), "Formationen".to_string())
        }
        StandardUnterId::Klasse => (ColumnTarget::DirectColumn(242), "Klasse".to_string()),
        StandardUnterId::Hoelle => (ColumnTarget::DirectColumn(496), "Hölle".to_string()),
        StandardUnterId::Liebe => (ColumnTarget::DirectColumn(14), "Liebe".to_string()),
        StandardUnterId::Geist => (ColumnTarget::DirectColumn(15), "Geist".to_string()),
        StandardUnterId::SymboleReligion => {
            (ColumnTarget::DirectColumn(700), "Symbole Religion".to_string())
        }
        StandardUnterId::Primzahlkreuz => (
            ColumnTarget::Generator(GeneratorSpec {
                art: GeneratorArt::Primzahlkreuz,
                parameter: GeneratorParameter::Keine,
            }),
            "Primzahlkreuz".to_string(),
        ),
    };

    Some(CanonicalColumnSpec {
        request: req,
        target,
        header_display,
        aliases_for_report: vec![],
    })
}

fn resolve_gebrochen_rational(
    req: SpaltenAnfrage,
    art: GebrochenRationalArt,
    index: u16,
) -> Option<CanonicalColumnSpec> {
    let header_display = match art {
        GebrochenRationalArt::Galaxie => format!("gebrochen-rational_Galaxie_n/m {index}"),
        GebrochenRationalArt::Universum => format!("gebrochen-rational_Universum_n/m {index}"),
        GebrochenRationalArt::Gefuehle => format!("gebrochen-rational_Gefuehle_n/m {index}"),
        GebrochenRationalArt::Strukturgroesse => {
            format!("gebrochen-rational_Strukturgroesse_n/m {index}")
        }
    };

    Some(CanonicalColumnSpec {
        request: req,
        target: ColumnTarget::DirectColumn(index),
        header_display,
        aliases_for_report: vec![],
    })
}

fn resolve_kombination(
    req: SpaltenAnfrage,
    art: KombinationsArt,
    unter: KombiUnterId,
) -> Option<CanonicalColumnSpec> {
    Some(CanonicalColumnSpec {
        request: req,
        target: ColumnTarget::Combination(CombinationSpec { art, unter }),
        header_display: format!("{art:?} {unter:?}"),
        aliases_for_report: vec![],
    })
}

fn resolve_generator(
    req: SpaltenAnfrage,
    art: GeneratorArt,
    parameter: GeneratorParameter,
) -> Option<CanonicalColumnSpec> {
    let header_display = match art {
        GeneratorArt::Primzahlkreuz => "Primzahlkreuz",
        GeneratorArt::Multiplikationen => "Multiplikationen",
        GeneratorArt::Primvielfache => "Primvielfache",
        GeneratorArt::MetaKonkret => "MetaKonkret",
    }
    .to_string();

    Some(CanonicalColumnSpec {
        request: req,
        target: ColumnTarget::Generator(GeneratorSpec { art, parameter }),
        header_display,
        aliases_for_report: vec![],
    })
}
