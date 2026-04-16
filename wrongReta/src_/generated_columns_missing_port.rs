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
    GebrRat,
}

#[derive(Debug, Clone, Default)]
pub struct Tables {
    pub generated_spalten_parameter_tags: TagsMap,
    pub generated_spalten_parameter: GeneratedParams,
    pub spalten_vanilla_amount: usize,
    pub last_line_number: usize,
    pub data_dict: HashMap<usize, HashMap<usize, String>>,
    pub html_output_yes: bool,
    pub bbcode_output_yes: bool,
    pub hoechste_zeile_1024: usize,
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

fn nonempty_join(parts: impl IntoIterator<Item = String>, sep: &str) -> String {
    parts.into_iter().filter(|s| !s.trim().is_empty()).collect::<Vec<_>>().join(sep)
}

fn unique_preserve_order(items: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in items {
        let norm = item.trim().to_string();
        if !norm.is_empty() && seen.insert(norm.clone()) {
            out.push(norm);
        }
    }
    out
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

fn divisors(n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    if n == 0 {
        return out;
    }
    let mut i = 1usize;
    while i * i <= n {
        if n % i == 0 {
            out.push(i);
            if i * i != n {
                out.push(n / i);
            }
        }
        i += 1;
    }
    out.sort_unstable();
    out
}

fn prime_repeat(factors: &[usize]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < factors.len() {
        let p = factors[i];
        let mut c = 1usize;
        i += 1;
        while i < factors.len() && factors[i] == p {
            c += 1;
            i += 1;
        }
        out.push((p, c));
    }
    out
}

fn could_be_prime_number_primzahlkreuz_fuer_innen(n: usize) -> bool {
    matches!(n % 24, 5 | 11 | 17 | 23)
}

fn could_be_prime_number_primzahlkreuz_fuer_aussen(n: usize) -> bool {
    matches!(n % 24, 1 | 7 | 13 | 19)
}

fn orientation_text(n: usize) -> String {
    let factors = prime_factors(n);
    if factors.is_empty() {
        return String::new();
    }
    let innen = factors.iter().filter(|&&p| could_be_prime_number_primzahlkreuz_fuer_innen(p)).count();
    let aussen = factors.iter().filter(|&&p| could_be_prime_number_primzahlkreuz_fuer_aussen(p)).count();
    let twos = factors.iter().filter(|&&p| p == 2).count();

    let mut parts = Vec::new();
    if innen > aussen && innen > 0 {
        parts.push("für innen".to_string());
    } else if aussen > innen && aussen > 0 {
        parts.push("für außen".to_string());
    } else if innen > 0 || aussen > 0 {
        parts.push("für seitlich".to_string());
    }

    if twos >= 2 {
        parts.push("gegen Schwächlinge innen".to_string());
    } else if twos == 1 {
        parts.push("neutral gegen innen".to_string());
    }

    nonempty_join(parts, "; ")
}

fn rational_text(table: &Table, numerator: usize, denominator: usize, n_col: usize, inv_col: usize) -> String {
    if numerator == 0 || denominator == 0 {
        return String::new();
    }
    if denominator == 1 {
        let txt = get_cell(table, numerator, n_col).trim();
        if txt.is_empty() {
            String::new()
        } else {
            format!("{} ({})", txt, numerator)
        }
    } else if numerator == 1 {
        let txt = get_cell(table, denominator, inv_col).trim();
        if txt.is_empty() {
            String::new()
        } else {
            format!("{} (1/{})", txt, denominator)
        }
    } else {
        let num_txt = get_cell(table, numerator, n_col).trim();
        let den_txt = get_cell(table, denominator, inv_col).trim();
        if num_txt.is_empty() && den_txt.is_empty() {
            String::new()
        } else if den_txt.is_empty() {
            format!("{} ({}/{})", num_txt, numerator, denominator)
        } else if num_txt.is_empty() {
            format!("{} ({}/{})", den_txt, numerator, denominator)
        } else {
            format!("{} zu {} ({}/{})", num_txt, den_txt, numerator, denominator)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PrimUniverseKind {
    PrimMotivStern,
    PrimStrukStern,
    PrimMotivGleichf,
    PrimStrukGleichf,
    PrimMotivSternGebr,
    PrimStrukSternGebr,
    PrimMotivGleichfGebr,
    PrimStrukGleichfGebr,
}

impl PrimUniverseKind {
    fn all() -> [Self; 8] {
        [
            Self::PrimMotivStern,
            Self::PrimStrukStern,
            Self::PrimMotivGleichf,
            Self::PrimStrukGleichf,
            Self::PrimMotivSternGebr,
            Self::PrimStrukSternGebr,
            Self::PrimMotivGleichfGebr,
            Self::PrimStrukGleichfGebr,
        ]
    }

    fn title(self) -> &'static str {
        match self {
            Self::PrimMotivStern => "Prim-Motiv Stern",
            Self::PrimStrukStern => "Prim-Struktur Stern",
            Self::PrimMotivGleichf => "Prim-Motiv gleichförmiges Polygon",
            Self::PrimStrukGleichf => "Prim-Struktur gleichförmiges Polygon",
            Self::PrimMotivSternGebr => "Prim-Motiv Stern gebrochen/rational",
            Self::PrimStrukSternGebr => "Prim-Struktur Stern gebrochen/rational",
            Self::PrimMotivGleichfGebr => "Prim-Motiv gleichförmiges Polygon gebrochen/rational",
            Self::PrimStrukGleichfGebr => "Prim-Struktur gleichförmiges Polygon gebrochen/rational",
        }
    }

    fn tags(self) -> BTreeSet<ST> {
        match self {
            Self::PrimMotivStern => tagset(&[ST::SternPolygon, ST::Galaxie]),
            Self::PrimStrukStern => tagset(&[ST::SternPolygon, ST::Universum]),
            Self::PrimMotivGleichf => tagset(&[ST::GleichfoermigesPolygon, ST::Galaxie]),
            Self::PrimStrukGleichf => tagset(&[ST::GleichfoermigesPolygon, ST::Universum]),
            Self::PrimMotivSternGebr => tagset(&[ST::SternPolygon, ST::Galaxie, ST::GebrRat]),
            Self::PrimStrukSternGebr => tagset(&[ST::SternPolygon, ST::Universum, ST::GebrRat]),
            Self::PrimMotivGleichfGebr => tagset(&[ST::GleichfoermigesPolygon, ST::Galaxie, ST::GebrRat]),
            Self::PrimStrukGleichfGebr => tagset(&[ST::GleichfoermigesPolygon, ST::Universum, ST::GebrRat]),
        }
    }
}

fn source_columns_for_kind(kind: PrimUniverseKind) -> (usize, usize) {
    match kind {
        PrimUniverseKind::PrimMotivStern | PrimUniverseKind::PrimMotivSternGebr => (10, 42),
        PrimUniverseKind::PrimStrukStern | PrimUniverseKind::PrimStrukSternGebr => (5, 131),
        PrimUniverseKind::PrimMotivGleichf | PrimUniverseKind::PrimMotivGleichfGebr => (42, 10),
        PrimUniverseKind::PrimStrukGleichf | PrimUniverseKind::PrimStrukGleichfGebr => (131, 5),
    }
}

fn build_prim_universe_text(table: &Table, n: usize, kind: PrimUniverseKind) -> String {
    if n == 0 {
        return kind.title().to_string();
    }

    let (n_col, inv_col) = source_columns_for_kind(kind);
    let factors = prime_factors(n);
    if factors.is_empty() {
        return String::new();
    }

    let factor_texts = unique_preserve_order(
        factors
            .iter()
            .map(|&p| get_cell(table, p, n_col).trim().to_string())
            .collect::<Vec<_>>()
    );

    let divisor_pairs = divisors(n)
        .into_iter()
        .filter(|&d| d > 1 && d < n)
        .map(|d| rational_text(table, d, n / d, n_col, inv_col))
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>();

    let mut parts = Vec::new();

    if !factor_texts.is_empty() {
        parts.push(format!("Primfaktoren: {}", factor_texts.join(" | ")));
    }

    if matches!(kind, PrimUniverseKind::PrimMotivSternGebr | PrimUniverseKind::PrimStrukSternGebr | PrimUniverseKind::PrimMotivGleichfGebr | PrimUniverseKind::PrimStrukGleichfGebr) {
        if !divisor_pairs.is_empty() {
            parts.push(format!("Gebrochene Kombinationen: {}", divisor_pairs.join(" || ")));
        }
    } else {
        let direct = get_cell(table, n, n_col).trim();
        if !direct.is_empty() {
            parts.push(format!("Direkt: {}", direct));
        }
        let inverse = get_cell(table, n, inv_col).trim();
        if !inverse.is_empty() {
            parts.push(format!("1/n: {}", inverse));
        }
    }

    if parts.is_empty() {
        String::new()
    } else {
        nonempty_join(parts, "; ")
    }
}

pub fn concat1_row_prim_universe2(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
) {
    let end = tables.last_line_number.min(table.len().saturating_sub(1));
    for kind in PrimUniverseKind::all() {
        let values = (0..=end)
            .map(|row| build_prim_universe_text(table, row, kind))
            .collect::<Vec<_>>();
        append_generated_col(table, values);
        register_generated_column(tables, rows_as_numbers, table, kind.tags(), kind.title());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MetaVariable {
    Meta = 2,
    Theorie = 3,
    Management = 4,
    Ganzheitlich = 5,
    Verwertung = 6,
    Richtung = 7,
}

impl MetaVariable {
    fn all() -> [Self; 6] {
        [
            Self::Meta,
            Self::Theorie,
            Self::Management,
            Self::Ganzheitlich,
            Self::Verwertung,
            Self::Richtung,
        ]
    }

    fn upper_title(self) -> &'static str {
        match self {
            Self::Meta => "Meta",
            Self::Theorie => "Theorie",
            Self::Management => "Management",
            Self::Ganzheitlich => "ganzheitlich",
            Self::Verwertung => "Verwertung, Unternehmung, Geschäft",
            Self::Richtung => "regieren, beherrschen",
        }
    }

    fn lower_title(self) -> &'static str {
        match self {
            Self::Meta => "Konkretes",
            Self::Theorie => "Praxis",
            Self::Management => "verändernd",
            Self::Ganzheitlich => "darüber hinaus gehend",
            Self::Verwertung => "wertvoll",
            Self::Richtung => "Richtung",
        }
    }
}

pub fn spalte_meta_konkret_abstrakt_is_ganz_zahlig(zahl: f64, spalten_wahl_invers: bool) -> bool {
    let value = if spalten_wahl_invers { 1.0 / zahl } else { zahl };
    let frac = value.fract().abs();
    frac < 0.00001 || frac > 0.99999
}

fn meta_source_cols(if_invers: bool) -> (usize, usize) {
    if if_invers { (131, 42) } else { (5, 10) }
}

fn build_meta_text(
    table: &Table,
    row_num: usize,
    meta: MetaVariable,
    both_rows: usize,
    if_invers: bool,
) -> String {
    if row_num == 0 {
        let base = if both_rows == 0 { meta.upper_title() } else { meta.lower_title() };
        return if if_invers {
            format!("{} für 1/n statt n", base)
        } else {
            base.to_string()
        };
    }

    let (uni_n, gal_n) = meta_source_cols(if_invers);
    let factors = prime_repeat(&prime_factors(row_num));
    if factors.is_empty() {
        return String::new();
    }

    let mut parts = Vec::new();
    for (p, exp) in factors {
        let src_col = if both_rows == 0 { uni_n } else { gal_n };
        let label = get_cell(table, p, src_col).trim();
        if !label.is_empty() {
            if exp == 1 {
                parts.push(format!("{} durch {}", meta.lower_title(), label));
            } else {
                parts.push(format!("{} durch {}^{}", meta.lower_title(), label, exp));
            }
        }
    }

    if parts.is_empty() {
        let direct = get_cell(table, row_num, if both_rows == 0 { uni_n } else { gal_n }).trim();
        if direct.is_empty() {
            String::new()
        } else if both_rows == 0 {
            format!("{}-Ebene von {}", meta.upper_title(), direct)
        } else {
            format!("{}-Ebene in {}", meta.lower_title(), direct)
        }
    } else {
        nonempty_join(parts, "; ")
    }
}

pub fn spalte_meta_kontret_theorie_abstrakt_etc_1(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
) {
    let end = tables.last_line_number.min(table.len().saturating_sub(1));
    for if_invers in [false, true] {
        for both_rows in [0usize, 1usize] {
            for meta in MetaVariable::all() {
                let values = (0..=end)
                    .map(|row| build_meta_text(table, row, meta, both_rows, if_invers))
                    .collect::<Vec<_>>();
                append_generated_col(table, values);
                let tags = if both_rows == 0 {
                    tagset(&[
                        if if_invers { ST::GleichfoermigesPolygon } else { ST::SternPolygon },
                        ST::Universum,
                    ])
                } else {
                    tagset(&[
                        if if_invers { ST::GleichfoermigesPolygon } else { ST::SternPolygon },
                        ST::Universum,
                        ST::GebrRat,
                    ])
                };
                let title = if both_rows == 0 { meta.upper_title() } else { meta.lower_title() };
                register_generated_column(tables, rows_as_numbers, table, tags, title);
            }
        }
    }
}

pub fn spalte_fuer_gegen_innen_aussen_seitlich_prim(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
) {
    let source_cols: &[(Option<usize>, &str, BTreeSet<ST>)] = &[
        (Some(5), "Transzendentalien, Strukturalien, Universum n", tagset(&[ST::SternPolygon, ST::Universum])),
        (Some(10), "Galaxie n", tagset(&[ST::SternPolygon, ST::Galaxie])),
        (Some(42), "Galaxie 1/n", tagset(&[ST::GleichfoermigesPolygon, ST::Galaxie])),
        (Some(131), "Transzendentalien, Strukturalien, Universum 1/n", tagset(&[ST::GleichfoermigesPolygon, ST::Universum])),
        (Some(138), "Dagegen-Gegen-Transzendentalien, Gegen-Strukturalien, Universum n", tagset(&[ST::SternPolygon, ST::Universum])),
        (Some(202), "neutrale Gegen-Transzendentalien, Gegen-Strukturalien, Universum n", tagset(&[ST::SternPolygon, ST::Universum])),
        (None, "Richtung-Richtung", tagset(&[ST::SternPolygon, ST::Universum])),
    ];

    let end = tables.last_line_number.min(table.len().saturating_sub(1));

    for (maybe_col, heading, tags) in source_cols {
        let values = (0..=end)
            .map(|row| {
                if row == 0 {
                    return (*heading).to_string();
                }
                let orient = orientation_text(row);
                match maybe_col {
                    Some(col) => {
                        let base = get_cell(table, row, *col).trim();
                        if base.is_empty() {
                            orient
                        } else if orient.is_empty() {
                            base.to_string()
                        } else {
                            format!("{} — {}", base, orient)
                        }
                    }
                    None => orient,
                }
            })
            .collect::<Vec<_>>();
        append_generated_col(table, values);
        register_generated_column(tables, rows_as_numbers, table, tags.clone(), *heading);
    }
}
