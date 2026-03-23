use crate::cli::TextBereich;
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

#[derive(Debug, Clone, Default)]
pub struct ParametersMain {
    pub bedeutung0: String,
    pub procontra0: String,
    pub grundstrukturen0: String,
    pub unter0: String,
}

fn normalize_token(s: &str) -> String {
    s.trim().to_lowercase()
}

fn contains_any_alias(tokens: &BTreeSet<String>, aliases: &[&str]) -> bool {
    aliases
        .iter()
        .any(|alias| tokens.contains(&normalize_token(alias)))
}

fn selected_by_pair(
    tokens: &BTreeSet<String>,
    first_aliases: &[&str],
    second_aliases: &[&str],
) -> bool {
    contains_any_alias(tokens, first_aliases) && contains_any_alias(tokens, second_aliases)
}

pub fn apply_generated_columns(
    headers: &mut Vec<String>,
    data: &mut Vec<Vec<String>>,
    bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
    parameters_main: &ParametersMain,
) -> Result<(), Box<dyn std::error::Error>> {

    let original_headers = headers.clone();
    let original_data = data.clone();

    let mut table: Table = Vec::with_capacity(original_data.len() + 1);
    table.push(original_headers.clone());
    table.extend(original_data.clone());

    let original_header_len = original_headers.len();

    let mut tables = Tables::default();
    let mut rows_as_numbers: BTreeSet<usize> = (0..original_header_len).collect();
    tables.spalten_vanilla_amount = original_header_len;
    tables.last_line_number = table.len().saturating_sub(1);
    tables.hoechste_zeile_1024 = table.len().saturating_sub(1);
    let mut tokens: BTreeSet<String> = generated_befehle
        .iter()
        .map(|s| normalize_token(s))
        .collect();

    if !parameters_main.bedeutung0.is_empty() {
        tokens.insert(normalize_token(&parameters_main.bedeutung0));
    }
    if !parameters_main.procontra0.is_empty() {
        tokens.insert(normalize_token(&parameters_main.procontra0));
    }
    if !parameters_main.grundstrukturen0.is_empty() {
        tokens.insert(normalize_token(&parameters_main.grundstrukturen0));
    }
    if !parameters_main.unter0.is_empty() {
        tokens.insert(normalize_token(&parameters_main.unter0));
    }

    const MENSCHLICHES: &[&str] = &["menschliches"];
    const PLANET: &[&str] = &["planet"];
    const UNIVERSUM: &[&str] = &["universum", "multiversum", "grundstrukturen"];
    const BEDEUTUNG: &[&str] = &["bedeutung", "wichtigste"];
    const GALAXIE: &[&str] = &["galaxie"];

    const LOVE_ALIASES: &[&str] = &["liebe", "ethik"];
    const GLEICHHEIT_ALIASES: &[&str] = &[
        "gleichheit",
        "freiheit",
        "dominieren",
        "ordnung",
        "ordnen",
        "filterung",
        "ungleichheit",
    ];
    const GEIST_ALIASES: &[&str] = &[
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
    ];
    const MOND64_ALIASES: &[&str] = &[
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
    ];
    const VERVIELFACHE_ALIASES: &[&str] = &[
        "primzahlen",
        "vielfache",
        "vielfacher",
        "multis",
        "multiplikationen",
        "offenbarung",
        "offenbarungjohannes",
    ];
    const MODAL_ALIASES: &[&str] = &[
        "modallogik",
        "modal",
        "modus",
        "modi",
        "sein",
        "zustaende",
        "zustände",
    ];

    let want_primzahlkreuz = tokens.contains("primzahlkreuzprocontra")
        || (contains_any_alias(&tokens, BEDEUTUNG)
            && contains_any_alias(&tokens, &["primzahlkreuz"]))
        || (contains_any_alias(&tokens, &["procontra"])
            && contains_any_alias(&tokens, &["primzahlkreuz"]));

    let want_love = selected_by_pair(&tokens, MENSCHLICHES, LOVE_ALIASES);
    let want_gleichheit = selected_by_pair(&tokens, PLANET, GLEICHHEIT_ALIASES)
        || selected_by_pair(
            &tokens,
            &["menschliches", "grundstrukturen"],
            GLEICHHEIT_ALIASES,
        );
    let want_geist = selected_by_pair(&tokens, UNIVERSUM, GEIST_ALIASES);
    let want_mond64 = selected_by_pair(&tokens, BEDEUTUNG, MOND64_ALIASES);
    let want_vervielfache = selected_by_pair(&tokens, BEDEUTUNG, VERVIELFACHE_ALIASES)
        || selected_by_pair(&tokens, GALAXIE, VERVIELFACHE_ALIASES);
    let want_modal = contains_any_alias(&tokens, MODAL_ALIASES);

    if want_primzahlkreuz {
        concat1_primzahlkreuz_pro_contra(
            &mut table,
            &mut rows_as_numbers,
            &mut tables,
            generated_befehle,
            parameters_main,
        );
    }

    if want_love {
        concat_love_polygon(&mut table, &mut rows_as_numbers, &mut tables);
    }

    if want_gleichheit {
        concat_gleichheit_freiheit_dominieren(&mut table, &mut rows_as_numbers, &mut tables);
    }

    if want_geist {
        concat_geist_emotion_energie_materie_topologie(
            &mut table,
            &mut rows_as_numbers,
            &mut tables,
        );
    }

    if want_mond64 {
        concat_prim_creativity_type(&mut table, &mut rows_as_numbers, &mut tables);
        concat_mond_exponzieren_logarithmus_typ(&mut table, &mut rows_as_numbers, &mut tables);
    }

    if want_vervielfache {
        concat_vervielfache_zeile(&mut table, &mut rows_as_numbers, &mut tables);
    }

    if want_modal {
        let mut modal_concepts: BTreeSet<(usize, usize)> = BTreeSet::new();

        for pair in &bereich.exact_modal_pairs {
            modal_concepts.insert(*pair);
        }

        if modal_concepts.is_empty() {
            let selected_zero_based: Vec<usize> = if !bereich.spaltenreihenfolgeundnurdiese.is_empty() {
                bereich
                    .spaltenreihenfolgeundnurdiese
                    .iter()
                    .filter_map(|&i| i.checked_sub(1))
                    .collect()
            } else if !bereich.spalten_bereiche.is_empty() {
                let mut cols = Vec::new();
                for &(from, to) in &bereich.spalten_bereiche {
                    if from == 0 || to == 0 || from > to {
                        continue;
                    }
                    for c in from..=to {
                        if let Some(zero) = c.checked_sub(1) {
                            cols.push(zero);
                        }
                    }
                }
                cols.sort_unstable();
                cols.dedup();
                cols
            } else {
                Vec::new()
            };

            if selected_zero_based.len() >= 2 {
                for pair in selected_zero_based.chunks(2) {
                    if pair.len() == 2 {
                        modal_concepts.insert((pair[0], pair[1]));
                    }
                }
            }
        }

        if modal_concepts.is_empty() {
            modal_concepts.insert((121usize, 122usize));
        }

        concat_modallogik(
            &mut table,
            &modal_concepts,
            &mut rows_as_numbers,
            &mut tables,
        );
    }

    if !bereich.exact_meta_konkret_specs.is_empty() {
        concat_universum_meta_konkret(
            &mut table,
            &bereich.exact_meta_konkret_specs,
            &mut rows_as_numbers,
            &mut tables,
        );
    }

    let mut keep_indices: Vec<usize> = if !bereich.exact_visible_columns.is_empty() {
        bereich
            .exact_visible_columns
            .iter()
            .filter_map(|&i| i.checked_sub(1))
            .collect()
    } else if !bereich.spaltenreihenfolgeundnurdiese.is_empty() {
        bereich
            .spaltenreihenfolgeundnurdiese
            .iter()
            .filter_map(|&i| i.checked_sub(1))
            .collect()
    } else if !bereich.spalten_bereiche.is_empty() {
        let mut cols = Vec::new();
        for &(from, to) in &bereich.spalten_bereiche {
            if from == 0 || to == 0 || from > to {
                continue;
            }
            for c in from..=to {
                if let Some(zero) = c.checked_sub(1) {
                    cols.push(zero);
                }
            }
        }
        cols.sort_unstable();
        cols.dedup();
        cols
    } else {
        Vec::new()
    };

    for i in original_header_len..table[0].len() {
        keep_indices.push(i);
    }

    if want_vervielfache {
        if original_header_len > 19 {
            keep_indices.push(19);
        }
        if original_header_len > 90 {
            keep_indices.push(90);
        }
    }

    let mut seen = BTreeSet::new();
    keep_indices.retain(|i| seen.insert(*i));

    if keep_indices.is_empty() {
        return Ok(());
    }

    let header_row: Vec<String> = table
        .first()
        .cloned()
        .unwrap_or_else(|| original_headers.clone());

    *headers = keep_indices
        .iter()
        .map(|&i| {
            let raw: String = header_row
                .get(i)
                .cloned()
                .or_else(|| original_headers.get(i).cloned())
                .unwrap_or_default();

            if raw.trim().is_empty() {
                format!("SQL-Spalte {}", i + 1)
            } else {
                raw
            }
        })
        .collect();

    *data = table
        .into_iter()
        .skip(1)
        .map(|row| {
            keep_indices
                .iter()
                .map(|&i| row.get(i).cloned().unwrap_or_default())
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(())
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SimpleFraction {
    num: usize,
    den: usize,
}

impl SimpleFraction {
    fn new(num: usize, den: usize) -> Option<Self> {
        if num == 0 || den == 0 { return None; }
        let g = gcd(num, den);
        Some(Self { num: num / g, den: den / g })
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.max(1)
}

fn meta_pair_labels(metavariable: usize, side: usize) -> (&'static str, &'static str, &'static str) {
    match (metavariable, side) {
        (2, 0) => ("Meta-Thema: ", "Meta-", "Meta für n"),
        (2, 1) => ("Konkretes: ", "Konkret-", "Konkretes für n"),
        (3, 0) => ("Theorie-Thema: ", "Theorie-", "Theorie für n"),
        (3, 1) => ("Praxis: ", "Praxis-", "Praxis für n"),
        (4, 0) => ("Planungs-Thema: ", "Planung-", "Management für n"),
        (4, 1) => ("Umsetzungs-Thema: ", "Umsetzung-", "verändernd für n"),
        (5, 0) => ("Anlass-Thema: ", "Anlass-", "ganzheitlich für n"),
        (5, 1) => ("Wirkungs-Thema: ", "wirkung-", "darüber hinaus gehend für n"),
        (6, 0) => ("Kraft-Gebung: ", "Kraft-geben-", "Verwertung, Unternehmung, Geschäft für n"),
        (6, 1) => ("Verstärkungs-Thema: ", "Verstärkung-", "wertvoll für n"),
        (7, 0) => ("Beherrschung: ", "beherrschend-", "regieren, beherrschen für n"),
        (7, 1) => ("Richtung-Thema: ", "Richtung-", "Richtung für n"),
        _ => ("", "", ""),
    }
}

fn make_prefix(repetitions: usize, base: &str) -> String {
    if repetitions <= 1 { base.to_string() } else { base.repeat(repetitions) }
}

fn lookup_universe_fraction(table: &Table, frac: SimpleFraction, struct_cols: (usize, usize)) -> String {
    if frac.den == 1 {
        let text = get_cell(table, frac.num, struct_cols.0);
        if text.trim().len() > 3 {
            return format!("{} ({})", text, frac.num);
        }
        return String::new();
    }
    if frac.num == 1 {
        let text = get_cell(table, frac.den, struct_cols.1);
        if text.trim().len() > 3 {
            return format!("{} (1/{})", text, frac.den);
        }
        return String::new();
    }
    String::new()
}

pub fn concat_universum_meta_konkret(
    table: &mut Table,
    specs: &[(usize, usize)],
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
) {
    let end = tables.last_line_number.min(table.len().saturating_sub(1));
    let struct_cols = (5usize, 131usize);

    for &(metavariable, side01) in specs {
        let side = if side01 == 0 { 0usize } else { 1usize };
        let (_, repeated_label, heading) = meta_pair_labels(metavariable, side);
        let mut values = vec![String::new(); table.len()];
        if !values.is_empty() { values[0] = heading.to_string(); }
        if values.len() > 1 { values[1] = String::new(); }

        for i in 2..=end {
            let mut pieces: Vec<String> = Vec::new();
            let mut more = i;
            let mut less_num = i;
            let mut less_den = 1usize;
            let mut current_col = struct_cols.1;
            let mut level = 0usize;
            let mut seen_fracs: BTreeSet<SimpleFraction> = BTreeSet::new();

            loop {
                current_col = if current_col == struct_cols.1 { struct_cols.0 } else { struct_cols.1 };
                level += 1;

                let next_more = more.checked_mul(metavariable).filter(|&v| v < table.len());
                let next_less = SimpleFraction::new(less_num, less_den.checked_mul(metavariable).unwrap_or(usize::MAX));

                if next_more.is_none() && next_less.is_none() {
                    break;
                }

                let pref = make_prefix(level, repeated_label);
                if side == 0 {
                    if let Some(r) = next_more {
                        let txt = get_cell(table, r, current_col);
                        if txt.trim().len() > 3 {
                            let inv = if current_col != struct_cols.0 && r != 1 { "1/" } else { "" };
                            pieces.push(format!("{}{} ({}{})", pref, txt, inv, r));
                        }
                        more = r;
                    } else {
                        more = table.len();
                    }
                } else {
                    if let Some(fr) = next_less {
                        if !seen_fracs.insert(fr) { break; }
                        let txt = if fr.den == 1 || fr.num == 1 {
                            lookup_universe_fraction(table, fr, struct_cols)
                        } else {
                            String::new()
                        };
                        if txt.trim().len() > 3 {
                            pieces.push(format!("{}{}", pref, txt));
                        }
                        less_num = fr.num;
                        less_den = fr.den;
                    } else {
                        break;
                    }
                }

                if level > 32 { break; }
            }

            values[i] = unique_preserve_order(pieces).join(" | ");
        }

        append_generated_col(table, values);
        register_generated_column(
            tables,
            rows_as_numbers,
            table,
            tagset(&[ST::SternPolygon, ST::Universum]),
            heading.to_string(),
        );
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConcatState {
    pub ones: BTreeSet<usize>,
    pub csvs_already_read: BTreeMap<usize, String>,
    pub csvs_same: BTreeMap<usize, Vec<usize>>,
}

/* -----------------------------
   Kleine Hilfsfunktionen
------------------------------ */

fn append_generated_col(table: &mut Table, values: Vec<String>) {
    for (row, value) in table.iter_mut().zip(values.into_iter()) {
        row.push(value);
    }
}

fn current_new_col_index(table: &Table) -> usize {
    table
        .first()
        .map(|r| r.len().saturating_sub(1))
        .unwrap_or(0)
}

fn register_generated_column(
    tables: &mut Tables,
    rows_as_numbers: &mut RowSet,
    table: &Table,
    tags: BTreeSet<ST>,
    source_text: String,
) {
    let new_col = table.first().map(|r| r.len() - 1).unwrap_or(0);
    rows_as_numbers.insert(new_col);
    tables
        .generated_spalten_parameter_tags
        .insert(new_col, tags);

    let key = tables.generated_spalten_parameter.len() + tables.spalten_vanilla_amount;
    if tables.generated_spalten_parameter.contains_key(&key) {
        panic!("generated_spalten_parameter key collision");
    }
    tables.generated_spalten_parameter.insert(key, source_text);
}

fn get_cell(table: &Table, row: usize, col: usize) -> &str {
    table
        .get(row)
        .and_then(|r| r.get(col))
        .map(|s| s.as_str())
        .unwrap_or("")
}

fn join_nonempty(parts: impl IntoIterator<Item = String>, sep: &str) -> String {
    parts
        .into_iter()
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>()
        .join(sep)
}

fn unique_preserve_order(items: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for item in items {
        if seen.insert(item.clone()) {
            out.push(item);
        }
    }
    out
}

/* -----------------------------
   Platzhalter für deine Domain-Funktionen
------------------------------ */

fn primfaktoren(mut n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut p = 2;
    while n > 1 {
        while n % p == 0 {
            out.push(p);
            n /= p;
        }
        p += 1;
    }
    out
}
fn could_be_prime_number_primzahlkreuz(n: usize) -> bool {
    matches!(n % 24, 1 | 5 | 7 | 11 | 13 | 17 | 19 | 23)
}

fn could_be_prime_number_primzahlkreuz_fuer_innen(n: usize) -> bool {
    matches!(n % 24, 5 | 11 | 17 | 23)
}

fn could_be_prime_number_primzahlkreuz_fuer_aussen(n: usize) -> bool {
    matches!(n % 24, 1 | 7 | 13 | 19)
}

fn prim_repeat(mut factors: Vec<usize>) -> Vec<(usize, usize)> {
    if factors.is_empty() {
        return Vec::new();
    }

    factors.reverse();

    let mut c = 1usize;
    let mut b: Option<usize> = None;
    let mut d: Vec<(usize, usize)> = Vec::new();

    for a in factors {
        if b == Some(a) {
            c += 1;
        } else {
            c = 1;
        }
        d.push((a, c));
        b = Some(a);
    }

    d.reverse();

    let mut out: Vec<(usize, usize)> = Vec::new();
    let mut last_prime: Option<usize> = None;

    for (e, g) in d {
        if last_prime != Some(e) {
            out.push((e, g));
        }
        last_prime = Some(e);
    }

    out
}

fn divisors(n: usize) -> BTreeSet<usize> {
    let mut out = BTreeSet::new();
    if n == 0 {
        return out;
    }

    let mut i = 1usize;
    while i * i <= n {
        if n % i == 0 {
            out.insert(i);
            out.insert(n / i);
        }
        i += 1;
    }
    out
}

fn prim_creativity(num: usize) -> usize {
    if num == 0 {
        return 0;
    }

    let fak = prim_repeat(primfaktoren(num));

    if fak.len() == 1 && fak[0].1 == 1 {
        return 1;
    }
    if fak.len() == 1 {
        return 3;
    }
    if fak.is_empty() {
        return 0;
    }

    let mut schnittmenge: Option<BTreeSet<usize>> = None;

    for (_, prim_amount) in &fak {
        let mut ds = divisors(*prim_amount);
        ds.remove(&1);

        if ds.is_empty() {
            schnittmenge = None;
            break;
        }

        schnittmenge = match schnittmenge {
            None => Some(ds),
            Some(prev) => Some(prev.intersection(&ds).copied().collect()),
        };
    }

    match schnittmenge {
        Some(s) if !s.is_empty() => 3,
        _ => 2,
    }
}

fn moon_number(num: usize) -> (Vec<usize>, Vec<usize>) {
    if num < 2 {
        return (Vec::new(), Vec::new());
    }

    let mut results = Vec::new();
    let mut exponents_minus_2 = Vec::new();

    for i in 2..num {
        let one_result = (num as f64).powf(1.0 / i as f64);
        let rounded = one_result.round();

        if (rounded * 100000.0).round() == (one_result * 100000.0).round() {
            let basis = rounded as usize;
            if basis >= 2 && basis.pow(i as u32) == num {
                results.push(basis);
                exponents_minus_2.push(i - 2);
            }
        }
    }

    (results, exponents_minus_2)
}

fn prim_multiple(n: usize) -> Vec<(usize, usize)> {
    if n == 0 {
        return Vec::new();
    }

    let mut multiples = vec![(1, n)];
    for (prim, _) in prim_repeat(primfaktoren(n)) {
        multiples.push((prim, n / prim));
    }
    multiples
}

fn tagset(tags: &[ST]) -> BTreeSet<ST> {
    tags.iter().copied().collect()
}

/* -----------------------------
   1) gleichheitFreiheitVergleich
------------------------------ */

pub fn gleichheit_freiheit_vergleich(zahl: usize) -> String {
    let mut out = Vec::new();

    if zahl % 4 == 0 {
        out.push("Dominieren, Unterordnen".to_string());
    }
    if zahl % 4 == 1 {
        out.push("Freiheit".to_string());
    }
    if zahl % 4 == 3 {
        out.push("Einschränkung der Freiheit".to_string());
    }
    if zahl % 4 == 2 {
        if zahl >= 2 && (zahl - 2) % 8 == 0 {
            out.push("Gleichheit".to_string());
        }
        if zahl >= 6 && (zahl - 6) % 16 == 0 {
            out.push("den anderen überbieten wollen".to_string());
        }
        if zahl >= 14 && (zahl - 14) % 16 == 0 {
            out.push("den anderen unterbieten wollen".to_string());
        }
    }

    out.join("; ")
}

/* -----------------------------
   2) geistEmotionEnergieMaterieTopologie
------------------------------ */

pub fn geist_emotion_energie_materie_topologie(zahl: usize) -> String {
    let pr_fa = primfaktoren(zahl);
    let auss: Vec<bool> = pr_fa
        .iter()
        .map(|&a| could_be_prime_number_primzahlkreuz_fuer_aussen(a))
        .collect();
    let innen: Vec<bool> = pr_fa
        .iter()
        .map(|&a| could_be_prime_number_primzahlkreuz_fuer_innen(a))
        .collect();

    let zwei = pr_fa.iter().filter(|&&a| a == 2).count();
    let gefuehl = auss.iter().any(|&b| b);
    let denken = innen.iter().any(|&b| b);

    let total_topologie = zwei > 1 && gefuehl;
    let etwas_topologie = (zwei > 1 || (zwei > 0 && gefuehl)) && !total_topologie;

    let total_materie = zwei > 4;
    let etwas_materie = zwei == 4;
    let wenig_materie = zwei == 3;
    let kaum_materie = zwei == 2;

    let x = denken;
    let y = pr_fa.contains(&2);
    let z = pr_fa.contains(&3);

    let total_energie = x && y && z;
    let einigermassen_energie = ((x && y) || (y && z) || (x && z)) && !total_energie;
    let kaum_energie = !einigermassen_energie && !total_energie && (x || y || z);

    let mut out = Vec::new();

    if denken {
        out.push("eine Denkart".to_string());
    }
    if gefuehl {
        out.push("eine Gefühlsart".to_string());
    }
    if total_materie {
        out.push("total eine Art, etwas geistig zu erzeugen".to_string());
    }
    if total_topologie {
        out.push("total eine Art zu erleben".to_string());
    }
    if total_energie {
        out.push("total eine Energie-Art".to_string());
    }
    if etwas_topologie {
        out.push("etwas eine Art zu erleben".to_string());
    }
    if etwas_materie {
        out.push("etwas eine Art, etwas geistig zu erzeugen".to_string());
    }
    if wenig_materie {
        out.push("wenig eine Art, etwas geistig zu erzeugen".to_string());
    }
    if einigermassen_energie {
        out.push("einigermaßen eine Energie-Art".to_string());
    }
    if kaum_energie {
        out.push("kaum eine Energie-Art".to_string());
    }
    if kaum_materie {
        out.push("kaum eine Art, etwas geistig zu erzeugen".to_string());
    }

    out.join("; ")
}

/* -----------------------------
   3) concatLovePolygon
------------------------------ */

pub fn concat_love_polygon(table: &mut Table, rows_as_numbers: &mut RowSet, tables: &mut Tables) {

    let values: Vec<String> = (0..table.len())
        .map(|i| {
            let c8 = get_cell(table, i, 8).trim();
            let c4 = get_cell(table, i, 4);
            if c8.is_empty() {
                String::new()
            } else {
                format!(
                    "{c8} der eigenen Strukturgröße ({c4}) auf dich bei gleichförmigen Polygonen"
                )
            }
        })
        .collect();

    append_generated_col(table, values);

    register_generated_column(
        tables,
        rows_as_numbers,
        table,
        tagset(&[ST::SternPolygon, ST::Galaxie, ST::GleichfoermigesPolygon]),
        tables
            .data_dict
            .get(&0)
            .and_then(|m| m.get(&9))
            .cloned()
            .unwrap_or_default(),
    );
}

/* -----------------------------
   4) concatGleichheitFreiheitDominieren
------------------------------ */

pub fn concat_gleichheit_freiheit_dominieren(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
) {

    let values: Vec<String> = (0..=tables.last_line_number.min(table.len().saturating_sub(1)))
        .map(|i| {
            if i == 0 {
                "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert".to_string()
            } else {
                gleichheit_freiheit_vergleich(i)
            }
        })
        .collect();

    append_generated_col(table, values);

    register_generated_column(
        tables,
        rows_as_numbers,
        table,
        tagset(&[ST::SternPolygon, ST::Universum]),
        tables
            .data_dict
            .get(&0)
            .and_then(|m| m.get(&132))
            .cloned()
            .unwrap_or_default(),
    );
}

/* -----------------------------
   5) concatGeistEmotionEnergieMaterieTopologie
------------------------------ */

pub fn concat_geist_emotion_energie_materie_topologie(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
) {

    let values: Vec<String> = (0..=tables.last_line_number.min(table.len().saturating_sub(1)))
        .map(|i| {
            if i == 0 {
                "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art"
                    .to_string()
            } else {
                geist_emotion_energie_materie_topologie(i)
            }
        })
        .collect();

    append_generated_col(table, values);

    register_generated_column(
        tables,
        rows_as_numbers,
        table,
        tagset(&[ST::SternPolygon, ST::Universum]),
        tables
            .data_dict
            .get(&0)
            .and_then(|m| m.get(&242))
            .cloned()
            .unwrap_or_default(),
    );
}

/* -----------------------------
   6) concatPrimCreativityType
------------------------------ */

pub fn concat_prim_creativity_type(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
) {

    let end = tables.last_line_number.min(table.len().saturating_sub(1));
    let values: Vec<String> = (0..=end)
        .map(|i| {
            if i == 0 {
                "Evolutions-Züchtungs-Kreativität".to_string()
            } else {
                match prim_creativity(i) {
                    0 => "0. Primzahl 1".to_string(),
                    1 => "1. Primzahl und Sonnenzahl".to_string(),
                    2 => "2. Sonnenzahl, aber keine Primzahl".to_string(),
                    _ => "3. Mondzahl".to_string(),
                }
            }
        })
        .collect();

    append_generated_col(table, values);

    register_generated_column(
        tables,
        rows_as_numbers,
        table,
        tagset(&[ST::SternPolygon, ST::Galaxie]),
        tables
            .data_dict
            .get(&0)
            .and_then(|m| m.get(&64))
            .cloned()
            .unwrap_or_default(),
    );
}

/* -----------------------------
   7) concatMondExponzierenLogarithmusTyp
------------------------------ */

pub fn concat_mond_exponzieren_logarithmus_typ(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
) {

    let pairs = [
        (44usize, "Mond-Typ eines Sternpolygons"),
        (56usize, "Mond-Typ eines gleichförmigen Polygons"),
    ];

    for (rownum, rowheading) in pairs {
        let end = tables.last_line_number.min(table.len().saturating_sub(1));
        let values: Vec<String> = (0..=end)
            .map(|i| {
                if i == 0 {
                    return rowheading.to_string();
                }

                let (bases, exponents_minus_2) = moon_number(i);
                if bases.is_empty() {
                    return "kein Mond".to_string();
                }

                let mut parts = Vec::new();

                for (k, (basis, exponent_minus_2)) in bases
                    .into_iter()
                    .zip(exponents_minus_2.into_iter())
                    .enumerate()
                {
                    let mut insert = get_cell(table, basis, rownum).trim_end().to_string();
                    insert = insert.replace("<SG>", get_cell(table, i, 4).trim());
                    insert = insert.replace("&lt;SG&gt;", get_cell(table, i, 4).trim());

                    let mut s = String::new();
                    if k > 0 {
                        s.push_str(" | ");
                    }
                    s.push_str(&insert);
                    s.push_str(" - ");
                    s.push_str(get_cell(table, exponent_minus_2 + 2, 10));
                    s.push_str(" | ");
                    s.push_str(get_cell(table, i, 10));
                    s.push_str(" + ");
                    s.push_str(get_cell(table, i, 11));
                    s.push_str(", ");
                    s.push_str(get_cell(table, exponent_minus_2 + 2, 85));
                    parts.push(s);
                }

                parts.join("")
            })
            .collect();

        append_generated_col(table, values);

        let tags = if rownum == 44 {
            tagset(&[ST::SternPolygon, ST::Universum, ST::Galaxie])
        } else {
            tagset(&[ST::GleichfoermigesPolygon, ST::Universum, ST::Galaxie])
        };

        register_generated_column(
            tables,
            rows_as_numbers,
            table,
            tags,
            tables
                .data_dict
                .get(&0)
                .and_then(|m| m.get(&64))
                .cloned()
                .unwrap_or_default(),
        );
    }
}

/* -----------------------------
   8) concatVervielfacheZeile
------------------------------ */

pub fn concat_vervielfache_zeile(table: &mut Table, rows_as_numbers: &RowSet, tables: &Tables) {
    let spalten_to_vervielfache: Vec<usize> = rows_as_numbers
        .iter()
        .copied()
        .filter(|s| *s == 90 || *s == 19)
        .collect();

    for s in spalten_to_vervielfache {
        let mut store: HashMap<(usize, usize), String> = HashMap::new();

        for z in 2..=tables.last_line_number.min(table.len().saturating_sub(1)) {
            let content = get_cell(table, z, s).to_string();
            if !content.trim().is_empty() {
                store.insert((z, s), content);
            }
        }

        let mut multis: HashMap<usize, Vec<usize>> = HashMap::new();
        for ((z, _), _) in &store {
            let mut vielfacher = 1usize;
            let mut ergebnis = vielfacher * *z;
            multis.entry(ergebnis).or_default().push(*z);

            while ergebnis < table.len() {
                vielfacher += 1;
                ergebnis = vielfacher * *z;
                if ergebnis < table.len() {
                    multis.entry(ergebnis).or_default().push(*z);
                }
            }
        }

        for z in 2..=tables.last_line_number.min(table.len().saturating_sub(1)) {
            let original = get_cell(table, z, s).to_string();
            let mut items = Vec::new();

            if !original.trim().is_empty() {
                items.push(original.clone());
            }

            if let Some(orig_rows) = multis.get(&z) {
                for ur_zeile in orig_rows {
                    if *ur_zeile == z {
                        continue;
                    }
                    if let Some(extra) = store.get(&(*ur_zeile, s)) {
                        if !extra.is_empty() && !items.contains(extra) {
                            items.push(extra.clone());
                        }
                    }
                }
            }

            let joined = if tables.html_output_yes {
                format!(
                    "<ul>{}</ul>",
                    items
                        .into_iter()
                        .map(|x| format!("<li>{x}</li>"))
                        .collect::<String>()
                )
            } else if tables.bbcode_output_yes {
                format!(
                    "[list]{}[/list]",
                    items
                        .into_iter()
                        .map(|x| format!("[*]{x}"))
                        .collect::<String>()
                )
            } else {
                items.join(" | ")
            };

            if let Some(row) = table.get_mut(z) {
                if let Some(cell) = row.get_mut(s) {
                    *cell = joined;
                }
            }
        }
    }
}

/* -----------------------------
   9) concatModallogik + prepareModalIntoTable
------------------------------ */

#[derive(Debug, Clone, Default)]
pub struct ModalEntry {
    pub i_orig_s: Vec<isize>,
    pub modal_s: Vec<Vec<String>>,
    pub vervielfachter: Vec<usize>,
}

pub type VorkommenVielfacher = BTreeMap<isize, Vec<(usize, usize)>>;
pub type VorkommenVielfacherB = BTreeMap<usize, BTreeMap<isize, ModalEntry>>;

pub fn get_modaloperators_per_line_cells(table: &Table, line: usize) -> Vec<String> {
    let mut modaloperators = Vec::new();

    if !get_cell(table, line, 97).is_empty() {
        modaloperators.push(get_cell(table, line, 97).to_string());
    }
    if !get_cell(table, line, 98).is_empty() {
        modaloperators.push(get_cell(table, line, 98).to_string());
    }

    let begin = line + 1;
    let end = line + (line.saturating_sub(1)) + 1;
    for coord in begin..end {
        let c = get_cell(table, coord, 42);
        if !c.is_empty() {
            modaloperators.push(c.to_string());
        }
    }

    modaloperators
}

pub fn prepare_modal_into_table(
    distance_from_line: isize,
    i: usize,
    vorkommen_vielfacher: &VorkommenVielfacher,
    vorkommen_vielfacher_b: &mut VorkommenVielfacherB,
    table: &Table,
) {
    let i_with_distance = i as isize + distance_from_line;

    let Some(couples) = vorkommen_vielfacher.get(&i_with_distance) else {
        return;
    };

    let mut modal_operator_en_en = Vec::new();
    let mut original_i_mehrere = Vec::new();
    let mut verviel_fachter = Vec::new();

    for (vorkommen, vielfacher) in couples {
        modal_operator_en_en.push(get_modaloperators_per_line_cells(table, *vielfacher));
        verviel_fachter.push(*vorkommen);
        original_i_mehrere.push(i_with_distance);
    }

    let inner = vorkommen_vielfacher_b.entry(i).or_default();
    let entry = inner.entry(distance_from_line).or_default();

    let mut new_i_orig = original_i_mehrere;
    new_i_orig.extend(entry.i_orig_s.clone());
    entry.i_orig_s = new_i_orig;

    let mut new_modal = modal_operator_en_en;
    new_modal.extend(entry.modal_s.clone());
    entry.modal_s = new_modal;

    let mut new_verv = verviel_fachter;
    new_verv.extend(entry.vervielfachter.clone());
    entry.vervielfachter = new_verv;
}

pub fn concat_modallogik(
    table: &mut Table,
    concepts_rows_set_of_tuple: &BTreeSet<(usize, usize)>,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
) {
    let distances = [-4, -3, -2, -1, 0, 1, 2, 3, 4];
    let end = tables.last_line_number.min(table.len().saturating_sub(1));

    let mut concepts: Vec<(usize, usize)> = concepts_rows_set_of_tuple.iter().copied().collect();
    concepts.sort_unstable();

    let table_copy = table.clone();

    for concept in concepts {
        let mut into: BTreeMap<usize, Vec<String>> = BTreeMap::new();
        let mut ein_mal_vorkommen = BTreeSet::new();

        for i in 0..=end {
            into.insert(i, vec![String::new()]);
            if i == 0 {
                into.insert(
                    i,
                    vec![
                        "Generiert: ".to_string(),
                        get_cell(&table_copy, i, concept.0).to_string(),
                    ],
                );
            } else if !get_cell(&table_copy, i, concept.0).trim().is_empty() {
                ein_mal_vorkommen.insert(i);
            }
        }

        let mut vorkommen_vielfacher: VorkommenVielfacher = BTreeMap::new();
        for ein_vorkommen in ein_mal_vorkommen {
            let mut vielfacher = 1usize;
            let mut ergebnis = vielfacher * ein_vorkommen;
            vorkommen_vielfacher
                .entry(ergebnis as isize)
                .or_default()
                .push((ein_vorkommen, vielfacher));

            while ergebnis < table_copy.len() {
                vielfacher += 1;
                ergebnis = vielfacher * ein_vorkommen;
                if ergebnis < table_copy.len() {
                    vorkommen_vielfacher
                        .entry(ergebnis as isize)
                        .or_default()
                        .push((ein_vorkommen, vielfacher));
                }
            }
        }

        let mut vorkommen_vielfacher_b: VorkommenVielfacherB = BTreeMap::new();
        for i in 1..=end {
            for distance in distances {
                prepare_modal_into_table(
                    distance,
                    i,
                    &vorkommen_vielfacher,
                    &mut vorkommen_vielfacher_b,
                    &table_copy,
                );
            }
        }

        for i in 1..=end {
            for distance in distances {
                let Some(distance_map) = vorkommen_vielfacher_b.get(&i) else {
                    continue;
                };
                let Some(entry) = distance_map.get(&distance) else {
                    continue;
                };

                for (modal_operatoren, vervielfachter) in
                    entry.modal_s.iter().zip(entry.vervielfachter.iter())
                {
                    let into_its_content = if distance.abs() % 2 == 0 {
                        get_cell(&table_copy, *vervielfachter, concept.0)
                    } else {
                        get_cell(&table_copy, *vervielfachter, concept.1)
                    };

                    let prefix = match distance.abs() {
                        2 => "mittelstark überdurchschnittlich: ",
                        1 => "überdurchschnittlich: ",
                        3 => "mittelleicht überdurchschnittlich: ",
                        0 => "sehr: ",
                        _ => "sehr leicht überdurchschnittlich: ",
                    };

                    let mut piece = String::new();
                    piece.push_str(prefix);

                    if let Some(op0) = modal_operatoren.get(0) {
                        piece.push_str(op0);
                        piece.push(' ');
                    }

                    let normalized = if modal_operatoren.get(0).map(|s| s.as_str())
                        == Some(get_cell(&table_copy, 1, 97))
                    {
                        into_its_content.to_string()
                    } else {
                        into_its_content
                            .replace("intrinsisch", "zuerst")
                            .replace("extrinsisch", "als zweites")
                    };
                    piece.push_str(&normalized);

                    if let Some(op1) = modal_operatoren.get(1) {
                        piece.push(' ');
                        piece.push_str(op1);
                    }

                    if distance.abs() % 2 == 1 && modal_operatoren.len() > 2 {
                        piece.push_str(", nicht: ");
                        piece.push_str(&modal_operatoren[2..].join(", "));
                        piece.push_str(" (das alles nicht): ");
                        piece.push_str(
                            &get_cell(&table_copy, *vervielfachter, concept.0)
                                .replace("extrinsisch", "als zweites")
                                .replace("intrinsisch", "zuerst"),
                        );
                    }

                    into.entry(i).or_default().push(piece);
                }
            }

            let condition_n_vs_1_per_n = concept.0 == 62
                || concept.0 == 63
                || (358..=367).contains(&concept.0)
                || (371..=374).contains(&concept.0);

            let fill_ = if condition_n_vs_1_per_n {
                get_cell(&table_copy, i, 197)
            } else {
                get_cell(&table_copy, i, 4)
            };

            if let Some(parts) = into.get_mut(&i) {
                if !(parts.len() == 1 && parts[0].is_empty()) {
                    parts.push(format!("Alles nur bezogen auf Satz {}", fill_));
                }
            }
        }

        let values: Vec<String> = (0..=end)
            .map(|w| into.remove(&w).unwrap_or_default().join(" | "))
            .collect();

        append_generated_col(table, values);

        let condition_n_vs_1_per_n = concept.0 == 62
            || concept.0 == 63
            || (358..=367).contains(&concept.0)
            || (371..=374).contains(&concept.0);

        register_generated_column(
            tables,
            rows_as_numbers,
            table,
            if condition_n_vs_1_per_n {
                tagset(&[ST::GleichfoermigesPolygon, ST::Galaxie])
            } else {
                tagset(&[ST::SternPolygon, ST::Galaxie])
            },
            tables
                .data_dict
                .get(&1)
                .and_then(|m| m.get(&concept.0))
                .cloned()
                .unwrap_or_default(),
        );
    }
}

/* -----------------------------
   10) concat1PrimzahlkreuzProContra
------------------------------ */

pub fn concat1_primzahlkreuz_pro_contra(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
    generated_befehle: &BTreeSet<String>,
    parameters_main: &ParametersMain,
) {

    let dreli = table.clone();
    let max_num = tables
        .hoechste_zeile_1024
        .max(dreli.len().saturating_sub(1));

    let headline = "Primzahlkreuz pro contra".to_string();

    let mut keine_primzahl_1 = true;
    let mut keine_primzahl_2 = true;
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();
    let (mut weiter1a, mut weiter1b, mut weiter2a, mut weiter2b) = (0usize, 0usize, 0usize, 0usize);

    let mut pro_pro: BTreeMap<usize, usize> = BTreeMap::new();
    let mut contra_contra: BTreeMap<usize, usize> = BTreeMap::new();

    let mut pro_pro2: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut contra_contra2: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();

    let mut first_generated_column = vec![String::new(); table.len()];

    for num in 0..=max_num {
        pro_pro2.entry(num).or_default();
        contra_contra2.entry(num).or_default();

        let mut into: Vec<String> = if num == 0 {
            vec![headline.clone()]
        } else {
            Vec::new()
        };
        let mut into1: Vec<String> = Vec::new();
        let mut into2: Vec<String> = Vec::new();

        if prim_creativity(num) == 1 || num == 1 {
            if could_be_prime_number_primzahlkreuz_fuer_innen(num) {
                list1.push(num);
                if num > 16 {
                    let gegen = if keine_primzahl_1 {
                        if let Some(&g) = list2.get(weiter1b + 1) {
                            weiter1b += 1;
                            g
                        } else {
                            continue;
                        }
                    } else if let Some(&g) = list1.get(weiter1a) {
                        weiter1a += 1;
                        g
                    } else {
                        continue;
                    };
                    contra_contra.insert(num, gegen);
                    contra_contra2.entry(num).or_default().insert(gegen);
                    into1.push(format!("gegen {}", gegen));
                } else if num == 11 || num == 5 {
                    let gegen = 2;
                    contra_contra.insert(num, gegen);
                    contra_contra2.entry(num).or_default().insert(gegen);
                    into1.push(format!("gegen {}", gegen));
                }
                keine_primzahl_1 = false;
            }

            if num == 2 {
                contra_contra.insert(num, 1);
                contra_contra2.entry(num).or_default().insert(1);
                into1.push("gegen 1".to_string());
            } else if num == 3 {
                pro_pro.insert(num, 1);
                pro_pro2.entry(num).or_default().insert(1);
                into2.push("pro 1".to_string());
            }

            if could_be_prime_number_primzahlkreuz_fuer_aussen(num) {
                list2.push(num);
                if num > 16 {
                    let pro = if keine_primzahl_2 {
                        let p = list1[weiter2b + 1];
                        weiter2b += 1;
                        p
                    } else {
                        let p = list2[weiter2a];
                        weiter2a += 1;
                        p
                    };
                    pro_pro.insert(num, pro);
                    pro_pro2.entry(num).or_default().insert(pro);
                    into2.push(format!("pro {}", pro));
                } else if num == 7 || num == 13 {
                    pro_pro.insert(num, 3);
                    pro_pro2.entry(num).or_default().insert(3);
                    into2.push("pro 3".to_string());
                }
                keine_primzahl_2 = false;
            }
        } else {
            if could_be_prime_number_primzahlkreuz_fuer_innen(num) {
                keine_primzahl_1 = true;
            } else if could_be_prime_number_primzahlkreuz_fuer_aussen(num) {
                keine_primzahl_2 = true;
            }

            let mut menge: BTreeSet<(usize, usize)> = BTreeSet::new();
            for (a, b) in prim_multiple(num) {
                let pair = if a <= b { (a, b) } else { (b, a) };
                menge.insert(pair);
            }

            let paare: Vec<(usize, usize)> = menge.into_iter().collect();
            for couple_a in paare {
                if couple_a.0 != 1 && couple_a.1 != 1 {
                    for couple in [couple_a, (couple_a.1, couple_a.0)] {
                        let firsts: Vec<usize> = if couple.0 != couple.1 {
                            vec![1, 0]
                        } else {
                            vec![1]
                        };

                        for first_or_second in firsts {
                            let idx = first_or_second;
                            let other_idx = if first_or_second == 1 { 0 } else { 1 };
                            let selected = if idx == 0 { couple.0 } else { couple.1 };
                            let other = if other_idx == 0 { couple.0 } else { couple.1 };

                            if could_be_prime_number_primzahlkreuz_fuer_innen(selected)
                                || couple.0 % 2 == 0
                                || couple.1 % 2 == 0
                            {
                                if let Some(v) = contra_contra.get(&selected) {
                                    let gegen3 = other * *v;
                                    contra_contra.insert(num, gegen3);
                                    contra_contra2.entry(num).or_default().insert(gegen3);
                                    into1.push(format!("gegen {}", gegen3));
                                }
                            }

                            if could_be_prime_number_primzahlkreuz_fuer_aussen(couple.1)
                                || couple.1 % 3 == 0
                                || couple.0 % 3 == 0
                            {
                                if let Some(v) = pro_pro.get(&selected) {
                                    let pro3 = other * *v;
                                    pro_pro.insert(num, pro3);
                                    pro_pro2.entry(num).or_default().insert(pro3);
                                    into2.push(format!("pro {}", pro3));
                                }
                            }
                        }
                    }
                }
            }
        }

        let text = dreli
            .get(num)
            .and_then(|r| r.get(206))
            .and_then(|s| s.split('|').nth(1))
            .unwrap_or("")
            .to_string();

        if !text.is_empty() {
            into.push(text);
        }

        into1 = unique_preserve_order(into1);
        into2 = unique_preserve_order(into2);

        let mut into_b = Vec::new();
        if num == 0 {
            into_b = into;
        } else {
            if !into1.is_empty() {
                into_b.push(into1.join(", "));
                into_b.push(format!(
                    " Darin kann sich die {} am Besten hineinversetzen.",
                    num
                ));
            }
            if !into2.is_empty() {
                into_b.push(into2.join(", "));
                into_b.push(format!(
                    " Darin kann sich die {} am Besten hineinversetzen.",
                    num
                ));
            }
            if !into.is_empty() {
                into_b.push(into.join(", "));
            }
        }

        if num < first_generated_column.len() {
            first_generated_column[num] = into_b.join(" | ");
        }
    }

    append_generated_col(table, first_generated_column);

    let mut reverse_contra: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for (key, values) in &contra_contra2 {
        for value in values {
            reverse_contra.entry(*value).or_default().insert(*key);
        }
    }

    let mut reverse_pro: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for (key, values) in &pro_pro2 {
        for value in values {
            reverse_pro.entry(*value).or_default().insert(*key);
        }
    }

    let mut second_generated_column = vec![String::new(); table.len()];

    for num in 0..table.len() {
        let pro2: Vec<usize> = reverse_pro
            .get(&num)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        let contra2: Vec<usize> = reverse_contra
            .get(&num)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();

        second_generated_column[num] = if num == 0 {
            headline.clone()
        } else if !pro2.is_empty() || !contra2.is_empty() {
            let mut parts = Vec::new();
            if !pro2.is_empty() {
                parts.push(format!("pro dieser Zahl sind: {:?}", pro2));
            }
            if !contra2.is_empty() {
                parts.push(format!("contra dieser Zahl sind: {:?}", contra2));
            }
            parts.push("Darin kann man sich hineinversetzen.".to_string());
            parts.join(" | ")
        } else {
            "-".to_string()
        };
    }

    append_generated_col(table, second_generated_column);

    let new_col_1 = table[0].len() - 2;
    let new_col_2 = table[0].len() - 1;
    rows_as_numbers.insert(new_col_1);
    rows_as_numbers.insert(new_col_2);

    tables
        .generated_spalten_parameter_tags
        .insert(new_col_1, tagset(&[ST::SternPolygon, ST::Universum]));
    tables
        .generated_spalten_parameter_tags
        .insert(new_col_2, tagset(&[ST::SternPolygon, ST::Universum]));

    let key1 = tables.generated_spalten_parameter.len() + tables.spalten_vanilla_amount;
    tables.generated_spalten_parameter.insert(
        key1,
        format!(
            "{} | {} | {}",
            parameters_main.bedeutung0,
            parameters_main.procontra0,
            parameters_main.grundstrukturen0
        ),
    );
    let key2 = tables.generated_spalten_parameter.len() + tables.spalten_vanilla_amount;
    tables.generated_spalten_parameter.insert(
        key2,
        format!(
            "{} | {} | {}",
            parameters_main.bedeutung0,
            parameters_main.procontra0,
            parameters_main.grundstrukturen0
        ),
    );
}
