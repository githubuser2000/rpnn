use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub type Table = Vec<Vec<String>>;
pub type RowSet = BTreeSet<usize>;
pub type TagsMap = HashMap<usize, BTreeSet<ST>>;
pub type GeneratedParams = BTreeMap<usize, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ST {
    SternPolygon,
    Galaxie,
    GleichfoermigesPolygon,
    Universum,
    Eigenschaft,
}

#[derive(Debug, Clone, Default)]
pub struct Tables {
    pub generated_spalten_parameter_tags: TagsMap,
    pub generated_spalten_parameter: GeneratedParams,
    pub spalten_vanilla_amount: usize,
    pub last_line_number: usize,
    pub data_dict: HashMap<usize, HashMap<usize, String>>,
}

fn tagset(tags: &[ST]) -> BTreeSet<ST> {
    tags.iter().copied().collect()
}

fn append_generated_col(table: &mut Table, values: Vec<String>) {
    for (row, value) in table.iter_mut().zip(values.into_iter()) {
        row.push(value);
    }
}

fn register_generated_column(
    tables: &mut Tables,
    rows_as_numbers: &mut RowSet,
    table: &Table,
    tags: BTreeSet<ST>,
    source_text: impl Into<String>,
) {
    let new_col = table.first().map(|r| r.len().saturating_sub(1)).unwrap_or(0);
    rows_as_numbers.insert(new_col);
    tables.generated_spalten_parameter_tags.insert(new_col, tags);
    let key = tables.generated_spalten_parameter.len() + tables.spalten_vanilla_amount;
    tables.generated_spalten_parameter.insert(key, source_text.into());
}

fn get_cell(table: &Table, row: usize, col: usize) -> &str {
    table
        .get(row)
        .and_then(|r| r.get(col))
        .map(|s| s.as_str())
        .unwrap_or("")
}

fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut p = 2usize;
    while n > 1 {
        while n % p == 0 {
            out.push(p);
            n /= p;
        }
        p += 1;
    }
    out
}

fn normalize_token(s: &str) -> String {
    s.trim().to_lowercase().replace('ä', "ae").replace('ö', "oe").replace('ü', "ue")
}

#[derive(Debug, Clone)]
pub struct EigenschaftDef {
    pub canonical: &'static str,
    pub aliases: &'static [&'static str],
    pub direct_columns: &'static [usize],
}

pub const EIGENSCHAFTEN: &[EigenschaftDef] = &[
    EigenschaftDef { canonical: "Weisheit_etc", aliases: &["weisheit", "metaweisheit", "weise", "optimal"], direct_columns: &[112] },
    EigenschaftDef { canonical: "Dein_Recht_bekommen", aliases: &["rechte", "recht", "selbstgerecht"], direct_columns: &[] },
    EigenschaftDef { canonical: "unterlegen_ueberlegen", aliases: &["unterlegen", "ueberlegen"], direct_columns: &[] },
    EigenschaftDef { canonical: "Ehrlichkeit_und_Streit", aliases: &["streit", "ehrlichkeit"], direct_columns: &[] },
    EigenschaftDef { canonical: "Wuerdig", aliases: &["wuerdig", "würdig"], direct_columns: &[] },
    EigenschaftDef { canonical: "Regel_vs_Ausnahme", aliases: &["regel", "ausnahme"], direct_columns: &[] },
    EigenschaftDef { canonical: "Filterart_Widrigkeit", aliases: &["filterart", "widrigkeit"], direct_columns: &[331, 335] },
    EigenschaftDef { canonical: "Werte", aliases: &["werte"], direct_columns: &[] },
    EigenschaftDef { canonical: "Gutartigkeits-Egoismus", aliases: &["position", "gutesreziprok"], direct_columns: &[] },
    EigenschaftDef { canonical: "Reflektieren_Erkenntnis-Erkennen", aliases: &["reflektieren", "erkenntnis"], direct_columns: &[] },
    EigenschaftDef { canonical: "Vertrauen_wollen", aliases: &["vertrauenwollen"], direct_columns: &[] },
    EigenschaftDef { canonical: "einklinken_vertrauen_anprangern", aliases: &["einklinken", "vertrauenerhalten", "anprangern"], direct_columns: &[] },
    EigenschaftDef { canonical: "Ausrichten_Einrichten", aliases: &["einrichten", "ausrichten"], direct_columns: &[] },
    EigenschaftDef { canonical: "Toleranz_Respekt_Akzeptanz_Willkommen", aliases: &["toleranz", "respekt", "akzeptanz", "willkommen"], direct_columns: &[] },
    EigenschaftDef { canonical: "familiebrauchen", aliases: &["familiebrauchen"], direct_columns: &[] },
    EigenschaftDef { canonical: "ego_bescheiden", aliases: &["ego", "bescheiden"], direct_columns: &[] },
    EigenschaftDef { canonical: "Selbstsucht_Ichsucht_etc", aliases: &["selbstsucht", "ichsucht"], direct_columns: &[] },
    EigenschaftDef { canonical: "Forschen_Erfinden_Einklinken", aliases: &["wissenschaft", "forschen", "einklinken", "erfinden"], direct_columns: &[] },
    EigenschaftDef { canonical: "Kooperation_vs_Arsch", aliases: &["arschloch", "kooperation", "arsch"], direct_columns: &[] },
    EigenschaftDef { canonical: "Liebe_usw", aliases: &["liebe", "zuneigung"], direct_columns: &[] },
    EigenschaftDef { canonical: "Selbstlosigkeit_Ichlosigkeit_etc", aliases: &["selbstlos", "ichlos"], direct_columns: &[] },
    EigenschaftDef { canonical: "variationsreich_eintoenig", aliases: &["eintoenig", "variationsreich"], direct_columns: &[] },
    EigenschaftDef { canonical: "Zuneigung_Abneigung", aliases: &["abgeneigt", "zugewandt", "reserviert", "zugeneigt"], direct_columns: &[] },
    EigenschaftDef { canonical: "ehrlich_vs_hoeflich", aliases: &["ehrlich", "hoeflich", "höflich"], direct_columns: &[] },
    EigenschaftDef { canonical: "Tragweite", aliases: &["tragweite"], direct_columns: &[] },
    EigenschaftDef { canonical: "wertvoll", aliases: &["wertlos", "wertvoll"], direct_columns: &[] },
    EigenschaftDef { canonical: "Goetter_Propheten_Familien_Freunde", aliases: &["familiaer", "goettlich", "freunde", "propheten"], direct_columns: &[] },
    EigenschaftDef { canonical: "sanft_vs_hart", aliases: &["sanft", "hart"], direct_columns: &[] },
    EigenschaftDef { canonical: "vereinen_vs_verbinden", aliases: &["vereinenverbinden", "vereinen", "verbinden", "einheit", "verbindung"], direct_columns: &[] },
    EigenschaftDef { canonical: "aehnlich", aliases: &["aehnlich", "ähnlich"], direct_columns: &[220] },
    EigenschaftDef { canonical: "gut_boese_lieb_schlecht", aliases: &["gut", "boese", "böse", "lieb", "schlecht"], direct_columns: &[52, 53] },
    EigenschaftDef { canonical: "Sinn_und_Zweck_des_Lebens", aliases: &["sinn", "zweck", "bedeutung"], direct_columns: &[88, 189] },
    EigenschaftDef { canonical: "Zeit_vs_Raum", aliases: &["zeit", "raum", "zeitlich", "raeumlich", "räumlich"], direct_columns: &[] },
    EigenschaftDef { canonical: "egalitaer_vs_autoritaer", aliases: &["egalitaerautoritaer", "egalitaer", "autoritaer", "egalitär", "autoritär"], direct_columns: &[] },
    EigenschaftDef { canonical: "Meinungen_und_Ruf", aliases: &["meinungen", "anderemenschen", "ruf"], direct_columns: &[] },
    EigenschaftDef { canonical: "Meinungsintelligenz", aliases: &["meinungsintelligenz", "ursprungsintelligenz"], direct_columns: &[] },
    EigenschaftDef { canonical: "Sittlichkeit", aliases: &["sittlichkeit", "annaehrerung"], direct_columns: &[] },
    EigenschaftDef { canonical: "Fuehrung", aliases: &["fuehrung", "führung"], direct_columns: &[] },
    EigenschaftDef { canonical: "Durchleuchten", aliases: &["durchleuchten", "erleuchten"], direct_columns: &[] },
    EigenschaftDef { canonical: "Foerdern_Sensiblisieren_und_Gedeihen", aliases: &["foerdern", "fördern", "begrenzen", "sensibilisieren", "gedeihen", "verderben"], direct_columns: &[] },
    EigenschaftDef { canonical: "Ueberheblichkeit", aliases: &["ueberheblichkeit", "ueberheblich", "überheblich", "überheblichkeit"], direct_columns: &[] },
    EigenschaftDef { canonical: "Polung_der_Liebe", aliases: &["liebepolung"], direct_columns: &[] },
    EigenschaftDef { canonical: "Egoismus_vs_Altruismus", aliases: &["egoismus", "altruismus", "egoist", "altruist"], direct_columns: &[136] },
    EigenschaftDef { canonical: "kausal", aliases: &["geltung", "genese", "kausal"], direct_columns: &[] },
    EigenschaftDef { canonical: "Gleichheit", aliases: &["gleich"], direct_columns: &[] },
    EigenschaftDef { canonical: "Ueberleben", aliases: &["ueberleben", "überleben"], direct_columns: &[] },
];

fn lookup_eigenschaft(tokens: &BTreeSet<String>) -> Vec<&'static EigenschaftDef> {
    EIGENSCHAFTEN
        .iter()
        .filter(|def| {
            tokens.contains(&normalize_token(def.canonical))
                || def.aliases.iter().any(|a| tokens.contains(&normalize_token(a)))
        })
        .collect()
}

fn direct_column_text(table: &Table, row: usize, cols: &[usize]) -> String {
    cols.iter()
        .map(|&c| get_cell(table, row, c).trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" | ")
}

fn heuristic_eigenschaft_text(table: &Table, row: usize, canonical: &str) -> String {
    if row == 0 {
        return canonical.replace('_', " ");
    }
    let factors = prime_factors(row);
    let parity = if row % 2 == 0 { "gerade" } else { "ungerade" };
    let basis = get_cell(table, row, 5).trim();
    let galaxie = get_cell(table, row, 10).trim();
    let inv = get_cell(table, row, 131).trim();

    match canonical {
        "Liebe_usw" => {
            let mut parts = Vec::new();
            if !basis.is_empty() { parts.push(format!("Struktur: {}", basis)); }
            if !galaxie.is_empty() { parts.push(format!("Motiv: {}", galaxie)); }
            parts.push(if row % 2 == 0 { "eher verbindend".to_string() } else { "eher unterscheidend".to_string() });
            parts.join("; ")
        }
        "Zuneigung_Abneigung" => {
            if row % 3 == 0 { "zugeneigt / offen".to_string() } else { "reserviert / distanziert".to_string() }
        }
        "wertvoll" => {
            format!("{}; Primfaktoren: {}", parity, factors.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("×"))
        }
        "Zeit_vs_Raum" => {
            if factors.iter().filter(|&&p| p == 2).count() >= 2 { "räumlich / strukturell".to_string() } else { "zeitlich / prozessual".to_string() }
        }
        "Meinungen_und_Ruf" => {
            nonempty(&[basis, galaxie, inv])
        }
        "Meinungsintelligenz" => {
            if factors.len() <= 1 { "ursprünglich / direkt".to_string() } else { "abgeleitet / mehrstufig".to_string() }
        }
        "Sittlichkeit" => if row % 4 == 0 { "regelgeleitet".to_string() } else { "situationsgeleitet".to_string() },
        "Fuehrung" => if row % 5 == 0 { "führend / ordnend".to_string() } else { "begleitend / anpassend".to_string() },
        "Durchleuchten" => format!("{} Ebenen analysierbar", factors.len().max(1)),
        "Foerdern_Sensiblisieren_und_Gedeihen" => if row % 6 == 0 { "fördernd / gedeihend".to_string() } else { "sensibilisierend / begrenzend".to_string() },
        "Ueberheblichkeit" => if row % 7 == 0 { "überheblich / dominant".to_string() } else { "nüchterner / begrenzt".to_string() },
        "Polung_der_Liebe" => if row % 2 == 0 { "reziprok / spiegelnd".to_string() } else { "asymmetrisch / suchend".to_string() },
        "Gleichheit" => if row % 2 == 0 { "eher Gleichheit".to_string() } else { "eher Unterschied".to_string() },
        "Ueberleben" => if factors.contains(&2) && factors.contains(&3) { "anpassungsfähig / robust".to_string() } else { "fragiler / spezialisiert".to_string() },
        _ => nonempty(&[basis, galaxie, inv]),
    }
}

fn nonempty(parts: &[&str]) -> String {
    parts.iter().map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<_>>().join("; ")
}

pub fn generate_eigenschaften_columns(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
    generated_befehle: &BTreeSet<String>,
) {
    let tokens = generated_befehle.iter().map(|s| normalize_token(s)).collect::<BTreeSet<_>>();
    let defs = lookup_eigenschaft(&tokens);
    if defs.is_empty() {
        return;
    }

    let end = tables.last_line_number.min(table.len().saturating_sub(1));
    for def in defs {
        let values = (0..=end)
            .map(|row| {
                if row == 0 {
                    return def.canonical.replace('_', " ");
                }
                if !def.direct_columns.is_empty() {
                    let direct = direct_column_text(table, row, def.direct_columns);
                    if !direct.is_empty() {
                        return direct;
                    }
                }
                heuristic_eigenschaft_text(table, row, def.canonical)
            })
            .collect::<Vec<_>>();
        append_generated_col(table, values);
        register_generated_column(
            tables,
            rows_as_numbers,
            table,
            tagset(&[ST::Eigenschaft, ST::Universum]),
            format!("Eigenschafts-Generator: {}", def.canonical),
        );
    }
}
