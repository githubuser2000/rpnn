#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OberToken {
    Menschliches,
    Religion,
    Galaxie,
    Universum,
    Grundstrukturen,
    Kontinuum,
    Multiversum,
    Planet10Oder12,
    Eigenschaften,
    EigenschaftenN,
    Eigenschaften1ProN,
    GebrochenRationalGalaxie,
    GebrochenRationalUniversum,
    GebrochenRationalGefuehle,
    GebrochenRationalStrukturgroesse,
    KombinationGalaxie,
    KombinationUniversum,
    KombinationGefuehle,
    KombinationStrukturgroesse,
    Primzahlkreuz,
    Multiplikationen,
    Primvielfache,
    MetaKonkret,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardUnterToken {
    Gewalt,
    Politische,
    Richtungen,
    Formationen,
    Klasse,
    Hoelle,
    Liebe,
    Geist,
    Religion,
    Primzahlkreuz,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KombiUnterToken {
    Tiere,
    Berufe,
    Religion,
    Politik,
}

fn normalize_case_and_spaces(input: &str) -> String {
    input
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn matches_any(input: &str, aliases: &[&str]) -> bool {
    let normalized = normalize_case_and_spaces(input);
    aliases
        .iter()
        .any(|alias| normalize_case_and_spaces(alias) == normalized)
}

impl OberToken {
    pub fn parse(input: &str) -> Option<Self> {
        let table: &[(OberToken, &[&str])] = &[
            (Self::Menschliches, &["Menschliches"]),
            (Self::Religion, &["Religion", "Religionen"]),
            (Self::Galaxie, &["Galaxie", "Galaxien"]),
            (Self::Universum, &["Universum"]),
            (Self::Grundstrukturen, &["Grundstrukturen"]),
            (Self::Kontinuum, &["Kontinuum"]),
            (Self::Multiversum, &["Multiversum"]),
            (Self::Planet10Oder12, &["Planet_(10_und_oder_12)", "Planet"]),
            (Self::Eigenschaften, &["Eigenschaft", "Eigenschaften", "konzept", "konzepte"]),
            (Self::EigenschaftenN, &["Eigenschaften_n", "Eigenschaften n", "konzept1", "konzepte1"]),
            (Self::Eigenschaften1ProN, &[
                "Eigenschaften_1/n",
                "Eigenschaften 1/n",
                "Eigenschaften_1pro_n",
                "Eigenschaften 1pro n",
                "konzept2",
                "konzepte2",
            ]),
            (Self::GebrochenRationalGalaxie, &[
                "gebrochen-rational_Galaxie_n/m",
                "gebrochen-rational Galaxie n/m",
                "gebrochengalaxie",
            ]),
            (Self::GebrochenRationalUniversum, &[
                "gebrochen-rational_Universum_n/m",
                "gebrochen-rational Universum n/m",
                "gebrochenuniversum",
            ]),
            (Self::GebrochenRationalGefuehle, &[
                "gebrochen-rational_Gefuehle_n/m",
                "gebrochen-rational Gefühle n/m",
                "gebrochenemotion",
                "gebrochengemotion",
            ]),
            (Self::GebrochenRationalStrukturgroesse, &[
                "gebrochen-rational_Strukturgroesse_n/m",
                "gebrochen-rational Strukturgroesse n/m",
                "gebrochengroesse",
            ]),
            (Self::KombinationGalaxie, &["KombinationGalaxie", "Kombination Galaxie"]),
            (Self::KombinationUniversum, &["KombinationUniversum", "Kombination Universum"]),
            (Self::KombinationGefuehle, &["KombinationGefuehle", "Kombination Gefühle"]),
            (Self::KombinationStrukturgroesse, &["KombinationStrukturgroesse", "Kombination Strukturgroesse"]),
            (Self::Primzahlkreuz, &["Primzahlkreuz"]),
            (Self::Multiplikationen, &["Multiplikationen"]),
            (Self::Primvielfache, &["Primvielfache", "primvielfache"]),
            (Self::MetaKonkret, &["MetaKonkret", "Universum_Metakonkret", "Universum Metakonkret"]),
        ];

        table
            .iter()
            .find_map(|(token, aliases)| matches_any(input, aliases).then_some(*token))
    }
}

impl StandardUnterToken {
    pub fn parse(input: &str) -> Option<Self> {
        let table: &[(StandardUnterToken, &[&str])] = &[
            (Self::Gewalt, &["Gewalt"]),
            (Self::Politische, &["politische"]),
            (Self::Richtungen, &["Richtungen"]),
            (Self::Formationen, &["Formationen"]),
            (Self::Klasse, &["Klasse"]),
            (Self::Hoelle, &["Hölle", "Hoelle"]),
            (Self::Liebe, &["Liebe"]),
            (Self::Geist, &["Geist"]),
            (Self::Religion, &["Religion", "Symbole Religion", "Symbole_Religion"]),
            (Self::Primzahlkreuz, &["Primzahlkreuz"]),
        ];

        table
            .iter()
            .find_map(|(token, aliases)| matches_any(input, aliases).then_some(*token))
    }
}

impl KombiUnterToken {
    pub fn parse(input: &str) -> Option<Self> {
        let table: &[(KombiUnterToken, &[&str])] = &[
            (Self::Tiere, &["tiere"]),
            (Self::Berufe, &["berufe"]),
            (Self::Religion, &["religion"]),
            (Self::Politik, &["politik"]),
        ];

        table
            .iter()
            .find_map(|(token, aliases)| matches_any(input, aliases).then_some(*token))
    }
}
