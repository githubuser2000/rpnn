use std::collections::BTreeSet;

use super::normalize::contains_any_alias;
use crate::domain::parser::legacy_cli_typed::LegacyOberToken;

pub fn infer_generator_only_request(ober: &str, unter: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();

    let ober_token = LegacyOberToken::parse(ober);

    let is_bedeutung = matches!(
        ober_token,
        LegacyOberToken::Bedeutung | LegacyOberToken::WichtigstesZumVerstehen
    );
    let is_procontra = matches!(ober_token, LegacyOberToken::ProContra);
    let is_universum = matches!(
        ober_token,
        LegacyOberToken::Universum | LegacyOberToken::Multiversum | LegacyOberToken::Grundstrukturen
    );
    let is_planet = matches!(ober_token, LegacyOberToken::Planet);
    let is_menschliches = matches!(ober_token, LegacyOberToken::Menschliches);
    let is_galaxie = matches!(ober_token, LegacyOberToken::Galaxie);
    let is_primvielfache = matches!(ober_token, LegacyOberToken::Primvielfache);
    let is_multiplikationen = matches!(ober_token, LegacyOberToken::Multiplikationen);
    let is_prim_generated_group = is_primvielfache || is_multiplikationen;

    if (is_bedeutung || is_procontra || is_universum)
        && contains_any_alias(unter, &["primzahlkreuzprocontra", "primzahlkreuz"])
    {
        out.insert("primzahlkreuzprocontra".to_string());
    }
    if (is_menschliches || matches!(ober_token, LegacyOberToken::Grundstrukturen))
        && contains_any_alias(unter, &["liebe", "ethik"])
    {
        out.insert("lovepolygon".to_string());
    }
    if (is_planet || is_menschliches || matches!(ober_token, LegacyOberToken::Grundstrukturen))
        && contains_any_alias(
            unter,
            &[
                "gleichheit",
                "freiheit",
                "dominieren",
                "ordnung",
                "ordnen",
                "ordnen und filtern",
                "filterung",
                "ungleichheit",
            ],
        )
    {
        out.insert("gleichheitfreiheit".to_string());
    }
    if is_universum
        && contains_any_alias(
            unter,
            &[
                "geist",
                "bewusstsein",
                "emotion",
                "emotionen",
                "gefuehl",
                "gefuehle",
                "gefühl",
                "gefühle",
                "energie",
                "materie",
                "topologie",
            ],
        )
    {
        out.insert("geistemotionenergiematerietopologie".to_string());
    }
    if is_bedeutung
        && contains_any_alias(
            unter,
            &[
                "gestirn",
                "mond",
                "sonne",
                "planet",
                "evolution",
                "intelligenz",
                "kreativ",
                "kreativitaet",
                "kreativität",
                "lernen",
                "erwerben",
            ],
        )
    {
        out.insert("primcreativitytype".to_string());
        out.insert("mondexponzierenlogarithmustyp".to_string());
    }
    if (is_bedeutung || is_galaxie)
        && contains_any_alias(
            unter,
            &[
                "primzahlen",
                "vielfache",
                "vielfacher",
                "multis",
                "multiplikationen",
                "offenbarung",
                "offenbarungjohannes",
            ],
        )
    {
        out.insert("vervielfachezeile".to_string());
    }
    if contains_any_alias(
        unter,
        &["modallogik", "modal", "modus", "modi", "sein", "zustaende", "zustände"],
    ) || contains_any_alias(
        ober,
        &["modallogik", "modal", "modus", "modi", "sein", "zustaende", "zustände"],
    ) {
        out.insert("modallogik".to_string());
    }
    if is_prim_generated_group {
        if contains_any_alias(
            unter,
            &[
                "motivgleichfoermig",
                "motivgleichförmig",
                "motivegleichfoermigepolygone",
                "motivegleichförmige polygone",
            ],
        ) {
            out.insert("primmotivgleichf".to_string());
        }
        if contains_any_alias(
            unter,
            &[
                "strukturgleichfoermig",
                "strukturgleichförmig",
                "strukturgleichfoermigepolygone",
                "strukturgleichförmige polygone",
            ],
        ) {
            out.insert("primstrukgleichf".to_string());
        }
        if contains_any_alias(unter, &["motivstern", "motivesternpolygone", "motivesternpolygon"])
        {
            out.insert("primmotivstern".to_string());
        }
        if contains_any_alias(
            unter,
            &["strukturstern", "struktursternpolygone", "struktursternpolygon"],
        ) {
            out.insert("primstrukstern".to_string());
        }
        if contains_any_alias(
            unter,
            &[
                "motivgebrstern",
                "motivsternpolygongebrochenrational",
                "motivsternpolygongebrochen-rational",
            ],
        ) {
            out.insert("primmotivsterngebr".to_string());
        }
        if contains_any_alias(
            unter,
            &[
                "strukgebrstern",
                "struktursternpolyongebrochenrational",
                "struktursternpolygongebrochen-rational",
            ],
        ) {
            out.insert("primstruksterngebr".to_string());
        }
        if contains_any_alias(
            unter,
            &[
                "motivgebrgleichf",
                "motivgleichfoermigepolygonegebrochenrational",
                "motivgleichförmigepolygonegebrochen-rational",
            ],
        ) {
            out.insert("primmotivgleichfgebr".to_string());
        }
        if contains_any_alias(
            unter,
            &[
                "strukgebrgleichf",
                "strukturgleichfoermigepolygonegebrochenrational",
                "strukturgleichförmigepolygonegebrochen-rational",
            ],
        ) {
            out.insert("primstrukgleichfgebr".to_string());
        }
    }

    out
}
