#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EigenschaftStandardFamilie {
    N,
    EinsDurchN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EigenschaftKeyId {
    WeisheitEtc,
    DeinRechtBekommen,
    UnterlegenUeberlegen,
    EhrlichkeitUndStreit,
    EinklinkenVertrauenAnprangern,
    FamilieBrauchen,
    EgoBescheiden,
    SelbstsuchtIchsuchtEtc,
    ForschenErfindenEinklinken,
    KooperationVsArsch,
    LiebeUsw,
    SelbstlosigkeitIchlosigkeitEtc,
    VariationsreichEintoenig,
    ZuneigungAbneigung,
    Delegieren,
    EhrlichVsHoeflich,
    Tragweite,
    Wertvoll,
    GoetterProphetenFamilienFreunde,
    SanftVsHart,
    VereinenVsVerbinden,
    Aehnlich,
    GutBoeseLiebSchlecht,
    SinnUndZweckDesLebens,
    ZeitVsRaum,
    EgalitaerVsAutoritaer,
    MeinungenUndRuf,
    Meinungsintelligenz,
    Sittlichkeit,
    Fuehrung,
    Durchleuchten,
    FoerdernSensiblisierenUndGedeihen,
    Ueberheblichkeit,
    PolungDerLiebe,
    EgoismusVsAltruismus,
    Kausal,
    Gleichheit,
    Ueberleben,
    Wuerdig,
    RegelVsAusnahme,
    FilterartWidrigkeit,
    Werte,
    GutartigkeitsEgoismus,
    ReflektierenErkenntnisErkennen,
    VertrauenWollen,
    AusrichtenEinrichten,
    ToleranzRespektAkzeptanzWillkommen,
}

impl EigenschaftKeyId {
    pub const ALL: &'static [EigenschaftKeyId] = &[
        Self::WeisheitEtc,
        Self::DeinRechtBekommen,
        Self::UnterlegenUeberlegen,
        Self::EhrlichkeitUndStreit,
        Self::EinklinkenVertrauenAnprangern,
        Self::FamilieBrauchen,
        Self::EgoBescheiden,
        Self::SelbstsuchtIchsuchtEtc,
        Self::ForschenErfindenEinklinken,
        Self::KooperationVsArsch,
        Self::LiebeUsw,
        Self::SelbstlosigkeitIchlosigkeitEtc,
        Self::VariationsreichEintoenig,
        Self::ZuneigungAbneigung,
        Self::Delegieren,
        Self::EhrlichVsHoeflich,
        Self::Tragweite,
        Self::Wertvoll,
        Self::GoetterProphetenFamilienFreunde,
        Self::SanftVsHart,
        Self::VereinenVsVerbinden,
        Self::Aehnlich,
        Self::GutBoeseLiebSchlecht,
        Self::SinnUndZweckDesLebens,
        Self::ZeitVsRaum,
        Self::EgalitaerVsAutoritaer,
        Self::MeinungenUndRuf,
        Self::Meinungsintelligenz,
        Self::Sittlichkeit,
        Self::Fuehrung,
        Self::Durchleuchten,
        Self::FoerdernSensiblisierenUndGedeihen,
        Self::Ueberheblichkeit,
        Self::PolungDerLiebe,
        Self::EgoismusVsAltruismus,
        Self::Kausal,
        Self::Gleichheit,
        Self::Ueberleben,
        Self::Wuerdig,
        Self::RegelVsAusnahme,
        Self::FilterartWidrigkeit,
        Self::Werte,
        Self::GutartigkeitsEgoismus,
        Self::ReflektierenErkenntnisErkennen,
        Self::VertrauenWollen,
        Self::AusrichtenEinrichten,
        Self::ToleranzRespektAkzeptanzWillkommen,
    ];

    pub fn from_alias(input: &str) -> Option<Self> {
        let wanted = normalize_cli_token(input);
        Self::ALL
            .iter()
            .copied()
            .find(|key| key.aliases().iter().any(|alias| normalize_cli_token(alias) == wanted))
    }

    pub fn canonical_name(self) -> &'static str {
        match self {
            Self::WeisheitEtc => "Weisheit_etc",
            Self::DeinRechtBekommen => "Dein_Recht_bekommen",
            Self::UnterlegenUeberlegen => "unterlegen_überlegen",
            Self::EhrlichkeitUndStreit => "Ehrlichkeit_und_Streit",
            Self::EinklinkenVertrauenAnprangern => "einklinken_vertrauen_anprangern",
            Self::FamilieBrauchen => "familiebrauchen",
            Self::EgoBescheiden => "ego",
            Self::SelbstsuchtIchsuchtEtc => "Selbstsucht_Ichsucht_etc",
            Self::ForschenErfindenEinklinken => "Forschen_Erfinden_Einklinken",
            Self::KooperationVsArsch => "Kooperation_vs_Arsch",
            Self::LiebeUsw => "Liebe_usw",
            Self::SelbstlosigkeitIchlosigkeitEtc => "Selbstlosigkeit_Ichlosigkeit_etc",
            Self::VariationsreichEintoenig => "variationsreich_eintönig",
            Self::ZuneigungAbneigung => "Zuneigung_Abneigung",
            Self::Delegieren => "delegieren",
            Self::EhrlichVsHoeflich => "ehrlich_vs_höflich",
            Self::Tragweite => "Tragweite",
            Self::Wertvoll => "wertvoll",
            Self::GoetterProphetenFamilienFreunde => "Götter_Propheten_Familien_Freunde",
            Self::SanftVsHart => "sanft_vs_hart",
            Self::VereinenVsVerbinden => "vereinen_vs_verbinden",
            Self::Aehnlich => "ähnlich",
            Self::GutBoeseLiebSchlecht => "gut_böse_lieb_schlecht",
            Self::SinnUndZweckDesLebens => "Sinn_und_Zweck_des_Lebens",
            Self::ZeitVsRaum => "Zeit_vs_Raum",
            Self::EgalitaerVsAutoritaer => "egalitär_vs_autoritär",
            Self::MeinungenUndRuf => "Meinungen_und_Ruf",
            Self::Meinungsintelligenz => "Meinungsintelligenz",
            Self::Sittlichkeit => "Sittlichkeit",
            Self::Fuehrung => "Führung",
            Self::Durchleuchten => "Durchleuchten",
            Self::FoerdernSensiblisierenUndGedeihen => "Fördern_Sensiblisieren_und_Gedeihen",
            Self::Ueberheblichkeit => "Überheblichkeit",
            Self::PolungDerLiebe => "Polung_der_Liebe",
            Self::EgoismusVsAltruismus => "Egoismus_vs_Altruismus",
            Self::Kausal => "kausal",
            Self::Gleichheit => "Gleichheit",
            Self::Ueberleben => "Überleben",
            Self::Wuerdig => "Würdig",
            Self::RegelVsAusnahme => "Regel_vs_Ausnahme",
            Self::FilterartWidrigkeit => "Filterart_Widrigkeit",
            Self::Werte => "Werte",
            Self::GutartigkeitsEgoismus => "Gutartigkeits-Egoismus",
            Self::ReflektierenErkenntnisErkennen => "Reflektieren_Erkenntnis-Erkennen",
            Self::VertrauenWollen => "Vertrauen_wollen",
            Self::AusrichtenEinrichten => "Ausrichten_Einrichten",
            Self::ToleranzRespektAkzeptanzWillkommen => "Toleranz_Respekt_Akzeptanz_Willkommen",
        }
    }

    pub fn aliases(self) -> &'static [&'static str] {
        match self {
            Self::WeisheitEtc => &["Weisheit_etc", "weisheit", "metaweisheit", "meta-weisheit", "idiot", "weise", "optimal", "optimum"],
            Self::DeinRechtBekommen => &["Dein_Recht_bekommen", "rechte", "recht", "selbstgerecht"],
            Self::UnterlegenUeberlegen => &["unterlegen_überlegen", "unterlegen", "ueberlegen"],
            Self::EhrlichkeitUndStreit => &["Ehrlichkeit_und_Streit", "streit", "ehrlichkeit"],
            Self::EinklinkenVertrauenAnprangern => &["einklinken_vertrauen_anprangern", "einklinken", "vertrauenerhalten", "anprangern"],
            Self::FamilieBrauchen => &["familiebrauchen"],
            Self::EgoBescheiden => &["ego", "bescheiden"],
            Self::SelbstsuchtIchsuchtEtc => &["Selbstsucht_Ichsucht_etc", "selbstsucht", "ichsucht"],
            Self::ForschenErfindenEinklinken => &["Forschen_Erfinden_Einklinken", "wissenschaft", "forschen", "einklinken", "erfinden"],
            Self::KooperationVsArsch => &["Kooperation_vs_Arsch", "arschloch", "kooperation", "arsch"],
            Self::LiebeUsw => &["Liebe_usw", "liebe", "zuneigung"],
            Self::SelbstlosigkeitIchlosigkeitEtc => &["Selbstlosigkeit_Ichlosigkeit_etc", "selbstlos", "ichlos"],
            Self::VariationsreichEintoenig => &["variationsreich_eintönig", "eintönig", "eintoenig", "variationsreich"],
            Self::ZuneigungAbneigung => &["Zuneigung_Abneigung", "abgeneigt", "zugewandt", "reserviert", "zugeneigt"],
            Self::Delegieren => &["delegieren", "ansammlung"],
            Self::EhrlichVsHoeflich => &["ehrlich_vs_höflich", "ehrlich", "höflich", "hoeflich"],
            Self::Tragweite => &["Tragweite", "tragweite"],
            Self::Wertvoll => &["wertvoll", "wertlos"],
            Self::GoetterProphetenFamilienFreunde => &["Götter_Propheten_Familien_Freunde", "familiaer", "goettlich", "freunde", "propheten"],
            Self::SanftVsHart => &["sanft_vs_hart", "sanft", "hart"],
            Self::VereinenVsVerbinden => &["vereinen_vs_verbinden", "vereinenverbinden", "vereinen", "verbinden", "einheit", "verbindung"],
            Self::Aehnlich => &["ähnlich", "aehnlich"],
            Self::GutBoeseLiebSchlecht => &["gut_böse_lieb_schlecht", "gut", "böse", "boese", "lieb", "schlecht"],
            Self::SinnUndZweckDesLebens => &["Sinn_und_Zweck_des_Lebens", "sinn", "zweck", "bedeutung"],
            Self::ZeitVsRaum => &["Zeit_vs_Raum", "zeit", "raum", "zeitlich", "räumlich"],
            Self::EgalitaerVsAutoritaer => &["egalitär_vs_autoritär", "egalitaerautoritaer", "egalitaer", "autoritaer", "egalitär", "autoritär"],
            Self::MeinungenUndRuf => &["Meinungen_und_Ruf", "meinungen", "anderemenschen", "ruf"],
            Self::Meinungsintelligenz => &["Meinungsintelligenz", "meinungsintelligenz", "ursprungsintelligenz"],
            Self::Sittlichkeit => &["Sittlichkeit", "sittlichkeit", "annaehrerung"],
            Self::Fuehrung => &["Führung", "führung", "fuehrung"],
            Self::Durchleuchten => &["Durchleuchten", "durchleuchten", "erleuchten"],
            Self::FoerdernSensiblisierenUndGedeihen => &["Fördern_Sensiblisieren_und_Gedeihen", "foerdern", "fördern", "begrenzen", "sensibilisieren", "gedeihen", "verderben"],
            Self::Ueberheblichkeit => &["Überheblichkeit", "überheblich", "ueberheblichkeit", "ueberheblich", "überheblichkeit"],
            Self::PolungDerLiebe => &["Polung_der_Liebe", "liebepolung"],
            Self::EgoismusVsAltruismus => &["Egoismus_vs_Altruismus", "egoismus", "altruismus", "egoist", "altruist"],
            Self::Kausal => &["kausal", "geltung", "genese"],
            Self::Gleichheit => &["Gleichheit", "gleich"],
            Self::Ueberleben => &["Überleben", "ueberleben"],
            Self::Wuerdig => &["Würdig", "wuerdig", "würdig"],
            Self::RegelVsAusnahme => &["Regel_vs_Ausnahme", "regel", "ausnahme"],
            Self::FilterartWidrigkeit => &["Filterart_Widrigkeit", "filterart", "widrigkeit"],
            Self::Werte => &["Werte", "werte"],
            Self::GutartigkeitsEgoismus => &["Gutartigkeits-Egoismus", "position", "gutesreziprok"],
            Self::ReflektierenErkenntnisErkennen => &["Reflektieren_Erkenntnis-Erkennen", "reflektieren", "erkenntnis"],
            Self::VertrauenWollen => &["Vertrauen_wollen", "vertrauenwollen"],
            Self::AusrichtenEinrichten => &["Ausrichten_Einrichten", "einrichten", "ausrichten"],
            Self::ToleranzRespektAkzeptanzWillkommen => &["Toleranz_Respekt_Akzeptanz_Willkommen", "toleranz", "respekt", "akzeptanz", "willkommen"],
        }
    }

    pub fn direct_columns(self) -> &'static [usize] {
        match self {
            Self::WeisheitEtc => &[112],
            Self::Aehnlich => &[220],
            Self::GutBoeseLiebSchlecht => &[52, 53],
            Self::SinnUndZweckDesLebens => &[88, 189],
            Self::EgoismusVsAltruismus => &[136],
            Self::FilterartWidrigkeit => &[331, 335],
            _ => &[],
        }
    }

    pub fn maybe_pair(self) -> Option<(usize, usize)> {
        match self {
            Self::WeisheitEtc => Some((40, 41)),
            Self::DeinRechtBekommen => Some((291, 292)),
            Self::UnterlegenUeberlegen => Some((380, 381)),
            Self::EhrlichkeitUndStreit => Some((375, 376)),
            Self::EinklinkenVertrauenAnprangern => Some((368, 369)),
            Self::FamilieBrauchen => Some((279, 280)),
            Self::EgoBescheiden => Some((277, 278)),
            Self::SelbstsuchtIchsuchtEtc => Some((274, 275)),
            Self::ForschenErfindenEinklinken => Some((258, 259)),
            Self::KooperationVsArsch => Some((245, 246)),
            Self::LiebeUsw => Some((247, 248)),
            Self::SelbstlosigkeitIchlosigkeitEtc => Some((238, 239)),
            Self::VariationsreichEintoenig => Some((236, 237)),
            Self::ZuneigungAbneigung => Some((199, 200)),
            Self::Delegieren => Some((227, 228)),
            Self::EhrlichVsHoeflich => Some((224, 225)),
            Self::Tragweite => Some((211, 212)),
            Self::Wertvoll => Some((186, 187)),
            Self::GoetterProphetenFamilienFreunde => Some((184, 185)),
            Self::SanftVsHart => None,
            Self::VereinenVsVerbinden => Some((133, 134)),
            Self::Aehnlich => None,
            Self::GutBoeseLiebSchlecht => Some((38, 39)),
            Self::SinnUndZweckDesLebens => Some((181, 182)),
            Self::ZeitVsRaum => Some((49, 50)),
            Self::EgalitaerVsAutoritaer => Some((163, 164)),
            Self::MeinungenUndRuf => Some((60, 61)),
            Self::Meinungsintelligenz => Some((151, 152)),
            Self::Sittlichkeit => Some((179, 180)),
            Self::Fuehrung => Some((173, 174)),
            Self::Durchleuchten => Some((177, 178)),
            Self::FoerdernSensiblisierenUndGedeihen => Some((175, 176)),
            Self::Ueberheblichkeit => Some((171, 172)),
            Self::PolungDerLiebe => Some((121, 122)),
            Self::EgoismusVsAltruismus => Some((66, 67)),
            Self::Kausal => Some((110, 111)),
            Self::Gleichheit => Some((192, 193)),
            Self::Ueberleben => Some((194, 195)),
            Self::Wuerdig => Some((373, 374)),
            Self::RegelVsAusnahme => Some((371, 372)),
            Self::FilterartWidrigkeit => None,
            Self::Werte => Some((360, 361)),
            Self::GutartigkeitsEgoismus => Some((362, 363)),
            Self::ReflektierenErkenntnisErkennen => Some((364, 365)),
            Self::VertrauenWollen => Some((366, 367)),
            Self::AusrichtenEinrichten => Some((358, 359)),
            Self::ToleranzRespektAkzeptanzWillkommen => Some((62, 63)),
        }
    }

    pub fn standard_familie(self) -> EigenschaftStandardFamilie {
        match self {
            Self::Wuerdig
            | Self::RegelVsAusnahme
            | Self::FilterartWidrigkeit
            | Self::Werte
            | Self::GutartigkeitsEgoismus
            | Self::ReflektierenErkenntnisErkennen
            | Self::VertrauenWollen
            | Self::AusrichtenEinrichten
            | Self::ToleranzRespektAkzeptanzWillkommen => EigenschaftStandardFamilie::EinsDurchN,
            _ => EigenschaftStandardFamilie::N,
        }
    }

    pub fn all_column_ids_1_based(self) -> Vec<u32> {
        let mut out: Vec<u32> = self.direct_columns().iter().map(|n| (*n as u32) + 1).collect();
        if let Some((left, right)) = self.maybe_pair() {
            out.push((left as u32) + 1);
            out.push((right as u32) + 1);
        }
        out.sort_unstable();
        out.dedup();
        out
    }
}

fn normalize_cli_token(s: &str) -> String {
    s.trim().to_lowercase()
}
