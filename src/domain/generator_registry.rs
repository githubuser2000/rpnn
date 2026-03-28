use crate::cli::TextBereich;
use crate::lib4tables_enum::ST;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use crate::domain::generator_logic::common::{contains_any_alias, normalize_token, selected_by_pair};
use crate::domain::generator_logic::number_theory::gcd;
use crate::domain::eigenschaften::{EigenschaftKeyId, EigenschaftStandardFamilie};

pub type Table = Vec<Vec<String>>;
pub type RowSet = BTreeSet<usize>;
pub type TagsMap = HashMap<usize, BTreeSet<ST>>;
pub type GeneratedParams = BTreeMap<usize, String>;


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

#[derive(Debug, Clone)]
struct GeneratorRuleContext {
    tokens: BTreeSet<String>,
}

impl GeneratorRuleContext {
    fn from_inputs(generated_befehle: &BTreeSet<String>, parameters_main: &ParametersMain) -> Self {
        let mut tokens: BTreeSet<String> = generated_befehle.iter().map(|s| normalize_token(s)).collect();

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

        Self { tokens }
    }
}

struct GeneratorExecutionContext<'a> {
    table: &'a mut Table,
    rows_as_numbers: &'a mut RowSet,
    tables: &'a mut Tables,
    bereich: &'a TextBereich,
    generated_befehle: &'a BTreeSet<String>,
    parameters_main: &'a ParametersMain,
}

mod sealed {
    pub trait Sealed {}
}

trait GeneratedColumnRule: sealed::Sealed {
    fn name(&self) -> &'static str;
    fn should_apply(&self, ctx: &GeneratorRuleContext) -> bool;
    fn apply(&self, ctx: &mut GeneratorExecutionContext<'_>) -> Result<(), crate::domain::errors::GeneratorError>;
}

struct PrimzahlkreuzRule;
impl sealed::Sealed for PrimzahlkreuzRule {}
impl GeneratedColumnRule for PrimzahlkreuzRule {
    fn name(&self) -> &'static str { "primzahlkreuzprocontra" }
    fn should_apply(&self, ctx: &GeneratorRuleContext) -> bool {
        ctx.tokens.contains("primzahlkreuzprocontra")
            || (contains_any_alias(&ctx.tokens, BEDEUTUNG) && contains_any_alias(&ctx.tokens, &["primzahlkreuz"]))
            || (contains_any_alias(&ctx.tokens, &["procontra"]) && contains_any_alias(&ctx.tokens, &["primzahlkreuz"]))
    }
    fn apply(&self, ctx: &mut GeneratorExecutionContext<'_>) -> Result<(), crate::domain::errors::GeneratorError> {
        concat1_primzahlkreuz_pro_contra(ctx.table, ctx.rows_as_numbers, ctx.tables, ctx.generated_befehle, ctx.parameters_main);
        Ok(())
    }
}

struct PairAliasRule {
    name: &'static str,
    first_aliases: &'static [&'static str],
    second_aliases: &'static [&'static str],
    apply_fn: fn(&mut Table, &mut RowSet, &mut Tables),
}
impl sealed::Sealed for PairAliasRule {}
impl GeneratedColumnRule for PairAliasRule {
    fn name(&self) -> &'static str { self.name }
    fn should_apply(&self, ctx: &GeneratorRuleContext) -> bool { selected_by_pair(&ctx.tokens, self.first_aliases, self.second_aliases) }
    fn apply(&self, ctx: &mut GeneratorExecutionContext<'_>) -> Result<(), crate::domain::errors::GeneratorError> { (self.apply_fn)(ctx.table, ctx.rows_as_numbers, ctx.tables); Ok(()) }
}

struct MultiPairAliasRule {
    name: &'static str,
    pairs: &'static [(&'static [&'static str], &'static [&'static str])],
    apply_fn: fn(&mut Table, &mut RowSet, &mut Tables),
}
impl sealed::Sealed for MultiPairAliasRule {}
impl GeneratedColumnRule for MultiPairAliasRule {
    fn name(&self) -> &'static str { self.name }
    fn should_apply(&self, ctx: &GeneratorRuleContext) -> bool {
        self.pairs.iter().any(|(a,b)| selected_by_pair(&ctx.tokens, a, b))
    }
    fn apply(&self, ctx: &mut GeneratorExecutionContext<'_>) -> Result<(), crate::domain::errors::GeneratorError> { (self.apply_fn)(ctx.table, ctx.rows_as_numbers, ctx.tables); Ok(()) }
}

struct TokenRule {
    name: &'static str,
    aliases: &'static [&'static str],
    apply_fn: fn(&mut Table, &mut RowSet, &mut Tables),
}
impl sealed::Sealed for TokenRule {}
impl GeneratedColumnRule for TokenRule {
    fn name(&self) -> &'static str { self.name }
    fn should_apply(&self, ctx: &GeneratorRuleContext) -> bool { contains_any_alias(&ctx.tokens, self.aliases) }
    fn apply(&self, ctx: &mut GeneratorExecutionContext<'_>) -> Result<(), crate::domain::errors::GeneratorError> { (self.apply_fn)(ctx.table, ctx.rows_as_numbers, ctx.tables); Ok(()) }
}

struct MondRule;
impl sealed::Sealed for MondRule {}
impl GeneratedColumnRule for MondRule {
    fn name(&self) -> &'static str { "mond64" }
    fn should_apply(&self, ctx: &GeneratorRuleContext) -> bool { selected_by_pair(&ctx.tokens, BEDEUTUNG, MOND64_ALIASES) }
    fn apply(&self, ctx: &mut GeneratorExecutionContext<'_>) -> Result<(), crate::domain::errors::GeneratorError> {
        concat_prim_creativity_type(ctx.table, ctx.rows_as_numbers, ctx.tables);
        concat_mond_exponzieren_logarithmus_typ(ctx.table, ctx.rows_as_numbers, ctx.tables);
        Ok(())
    }
}

struct PrimUniverseRule {
    name: &'static str,
    token: &'static str,
    gebrochen: bool,
    gleichfoermig: usize,
    struktural: usize,
}
impl sealed::Sealed for PrimUniverseRule {}
impl GeneratedColumnRule for PrimUniverseRule {
    fn name(&self) -> &'static str { self.name }
    fn should_apply(&self, ctx: &GeneratorRuleContext) -> bool { ctx.tokens.contains(self.token) }
    fn apply(&self, ctx: &mut GeneratorExecutionContext<'_>) -> Result<(), crate::domain::errors::GeneratorError> {
        concat_prim_universe_generated(ctx.table, ctx.rows_as_numbers, ctx.tables, self.gebrochen, self.gleichfoermig, self.struktural);
        Ok(())
    }
}

struct ModalRule;
impl sealed::Sealed for ModalRule {}
impl GeneratedColumnRule for ModalRule {
    fn name(&self) -> &'static str { "modallogik" }
    fn should_apply(&self, ctx: &GeneratorRuleContext) -> bool { contains_any_alias(&ctx.tokens, MODAL_ALIASES) }
    fn apply(&self, ctx: &mut GeneratorExecutionContext<'_>) -> Result<(), crate::domain::errors::GeneratorError> {
        let mut modal_concepts: BTreeSet<(usize, usize)> = BTreeSet::new();
        for pair in &ctx.bereich.exact_modal_pairs {
            modal_concepts.insert(*pair);
        }

        if modal_concepts.is_empty() {
            let selected_zero_based: Vec<usize> = if !ctx.bereich.spaltenreihenfolgeundnurdiese.is_empty() {
                ctx.bereich.spaltenreihenfolgeundnurdiese.iter().filter_map(|&i| i.checked_sub(1)).collect()
            } else if !ctx.bereich.spalten_bereiche.is_empty() {
                let mut cols = Vec::new();
                for &(from, to) in &ctx.bereich.spalten_bereiche {
                    if from == 0 || to == 0 || from > to { continue; }
                    for c in from..=to {
                        if let Some(zero) = c.checked_sub(1) { cols.push(zero); }
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
                    if pair.len() == 2 { modal_concepts.insert((pair[0], pair[1])); }
                }
            }
        }

        if modal_concepts.is_empty() {
            modal_concepts.insert((121usize, 122usize));
        }

        concat_modallogik(ctx.table, &modal_concepts, ctx.rows_as_numbers, ctx.tables);
        Ok(())
    }
}

fn build_generator_rules() -> Vec<Box<dyn GeneratedColumnRule>> {
    vec![
        Box::new(PrimzahlkreuzRule),
        Box::new(PairAliasRule { name: "love_polygon", first_aliases: MENSCHLICHES, second_aliases: LOVE_ALIASES, apply_fn: concat_love_polygon }),
        Box::new(MultiPairAliasRule { name: "gleichheit_freiheit", pairs: &[(PLANET, GLEICHHEIT_ALIASES), (&["menschliches", "grundstrukturen"], GLEICHHEIT_ALIASES)], apply_fn: concat_gleichheit_freiheit_dominieren }),
        Box::new(PairAliasRule { name: "geist_emotion", first_aliases: UNIVERSUM, second_aliases: GEIST_ALIASES, apply_fn: concat_geist_emotion_energie_materie_topologie }),
        Box::new(MondRule),
        Box::new(MultiPairAliasRule { name: "vervielfache", pairs: &[(BEDEUTUNG, VERVIELFACHE_ALIASES), (GALAXIE, VERVIELFACHE_ALIASES)], apply_fn: |t, r, tb| concat_vervielfache_zeile(t, r, tb) }),
        Box::new(PrimUniverseRule { name: "primmotivstern", token: "primmotivstern", gebrochen: false, gleichfoermig: 0, struktural: 0 }),
        Box::new(PrimUniverseRule { name: "primstrukstern", token: "primstrukstern", gebrochen: false, gleichfoermig: 0, struktural: 1 }),
        Box::new(PrimUniverseRule { name: "primmotivgleichf", token: "primmotivgleichf", gebrochen: false, gleichfoermig: 1, struktural: 0 }),
        Box::new(PrimUniverseRule { name: "primstrukgleichf", token: "primstrukgleichf", gebrochen: false, gleichfoermig: 1, struktural: 1 }),
        Box::new(PrimUniverseRule { name: "primmotivsterngebr", token: "primmotivsterngebr", gebrochen: true, gleichfoermig: 0, struktural: 0 }),
        Box::new(PrimUniverseRule { name: "primstruksterngebr", token: "primstruksterngebr", gebrochen: true, gleichfoermig: 0, struktural: 1 }),
        Box::new(PrimUniverseRule { name: "primmotivgleichfgebr", token: "primmotivgleichfgebr", gebrochen: true, gleichfoermig: 1, struktural: 0 }),
        Box::new(PrimUniverseRule { name: "primstrukgleichfgebr", token: "primstrukgleichfgebr", gebrochen: true, gleichfoermig: 1, struktural: 1 }),
        Box::new(ModalRule),
    ]
}


fn encode_hex_utf8(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for b in s.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{:02x}", b);
    }
    out
}

fn st_to_tag_num(st: ST) -> u8 {
    match st {
        ST::SternPolygon => 0,
        ST::GleichfoermigesPolygon => 1,
        ST::KeinPolygon => 2,
        ST::Galaxie => 3,
        ST::Universum => 4,
        ST::KeinParaOdMetaP => 5,
        ST::GebrRat => 6,
    }
}

fn append_safe_html_meta_marker(
    header: &str,
    tags: Option<&BTreeSet<ST>>,
    source_text: Option<&str>,
) -> String {
    let mut extras: Vec<String> = Vec::new();

    if let Some(tags) = tags {
        let mut nums: Vec<u8> = tags.iter().copied().map(st_to_tag_num).collect();
        nums.sort_unstable();
        nums.dedup();
        extras.push(format!(
            "TAGS={}",
            nums.into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
    }

    if let Some(src) = source_text {
        if !src.is_empty() {
            extras.push(format!("SRCHEX={}", encode_hex_utf8(src)));
            let src_l = src.to_lowercase();
            let family = if src_l.contains("gleichförmigen polygonen") || src_l.contains("gleichfoermigen polygonen") || src_l.contains("(1/n)") {
                Some("1N")
            } else if src_l.contains("sternenpolygonen") || src_l.contains("(n)") {
                Some("N")
            } else {
                None
            };
            if let Some(fam) = family {
                extras.push(format!("FAMILY={}", fam));
            }
            if let Some(start) = src.find('„').or_else(|| src.find('"')) {
                let tail = &src[start + 1..];
                if let Some(end) = tail.find('“').or_else(|| tail.find('"')) {
                    let seg = &tail[..end];
                    if let Some(key) = crate::domain::eigenschaften::EigenschaftKeyId::from_alias(seg) {
                        extras.push(format!("EIGKEY={}", key.canonical_name()));
                    }
                }
            }
        }
    }

    if extras.is_empty() {
        header.to_string()
    } else {
        format!("{header} [[RPNN:{}]]", extras.join(";"))
    }
}

pub fn apply_generated_columns(
    headers: &mut Vec<String>,
    data: &mut Vec<Vec<String>>,
    bereich: &TextBereich,
    generated_befehle: &BTreeSet<String>,
    parameters_main: &ParametersMain,
) -> Result<(), crate::domain::errors::GeneratorError> {
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

    let rule_ctx = GeneratorRuleContext::from_inputs(generated_befehle, parameters_main);
    let rules = build_generator_rules();
    let mut exec_ctx = GeneratorExecutionContext {
        table: &mut table,
        rows_as_numbers: &mut rows_as_numbers,
        tables: &mut tables,
        bereich,
        generated_befehle,
        parameters_main,
    };

    let mut want_vervielfache = false;
    for rule in rules.iter() {
        if rule.should_apply(&rule_ctx) {
            if rule.name() == "vervielfache" {
                want_vervielfache = true;
            }
            rule.apply(&mut exec_ctx).map_err(|err| crate::domain::errors::GeneratorError::RuleApplicationFailed { rule: rule.name(), detail: err.to_string() })?;
        }
    }

    if !bereich.exact_meta_konkret_specs.is_empty() {
        concat_universum_meta_konkret(&mut table, &bereich.exact_meta_konkret_specs, &mut rows_as_numbers, &mut tables);
    }

    let mut keep_indices: Vec<usize> = if !bereich.exact_visible_columns.is_empty() {
        bereich.exact_visible_columns.iter().filter_map(|&i| i.checked_sub(1)).collect()
    } else if !bereich.spalten_bereiche.is_empty() {
        let mut cols = Vec::new();
        for &(from, to) in &bereich.spalten_bereiche {
            if from == 0 || to == 0 || from > to { continue; }
            for c in from..=to {
                if let Some(zero) = c.checked_sub(1) { cols.push(zero); }
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
        if original_header_len > 19 { keep_indices.push(19); }
        if original_header_len > 90 { keep_indices.push(90); }
    }

    let mut seen = BTreeSet::new();
    keep_indices.retain(|i| seen.insert(*i));

    if keep_indices.is_empty() {
        return Ok(());
    }

    let header_row: Vec<String> = table.first().cloned().unwrap_or_else(|| original_headers.clone());

    *headers = keep_indices
        .iter()
        .map(|&i| {
            let raw: String = header_row
                .get(i)
                .cloned()
                .or_else(|| original_headers.get(i).cloned())
                .unwrap_or_default();
            let visible = if raw.trim().is_empty() {
                format!("SQL-Spalte {}", i + 1)
            } else {
                raw
            };

            let tags = tables.generated_spalten_parameter_tags.get(&i);
            let source_text = tables.generated_spalten_parameter.get(&i).map(|s| s.as_str());

            append_safe_html_meta_marker(&visible, tags, source_text)
        })
        .collect();

    *data = table.into_iter().skip(1).map(|row| {
        keep_indices.iter().map(|&i| row.get(i).cloned().unwrap_or_default()).collect::<Vec<_>>()
    }).collect();

    Ok(())
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct SimpleFraction {
    pub(crate) num: usize,
    pub(crate) den: usize,
}

impl SimpleFraction {
    pub(crate) fn new(num: usize, den: usize) -> Option<Self> {
        if num == 0 || den == 0 { return None; }
        let g = gcd(num, den);
        Some(Self { num: num / g, den: den / g })
    }

    pub(crate) fn mul(self, other: Self) -> Option<Self> {
        Self::new(self.num.saturating_mul(other.num), self.den.saturating_mul(other.den))
    }

    pub(crate) fn div(self, other: Self) -> Option<Self> {
        if other.num == 0 { return None; }
        Self::new(self.num.saturating_mul(other.den), self.den.saturating_mul(other.num))
    }

    pub(crate) fn inv(self) -> Option<Self> {
        Self::new(self.den, self.num)
    }

    pub(crate) fn is_integer(self) -> bool {
        self.den == 1
    }
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

fn find_header_index_casefold(headers: &[String], wanted: &str) -> Option<usize> {
    let w = wanted.trim().to_lowercase();
    headers.iter().position(|h| h.trim().to_lowercase() == w)
}

fn cell_by_header(table: &Table, row: usize, wanted: &str) -> String {
    let idx = match table.first().and_then(|h| find_header_index_casefold(h, wanted)) {
        Some(i) => i,
        None => return String::new(),
    };
    table.get(row).and_then(|r| r.get(idx)).cloned().unwrap_or_default()
}

fn csv_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("csv").join(name)
}

fn read_semicolon_csv(name: &str) -> Vec<Vec<String>> {
    let path = csv_path(name);
    let Ok(content) = fs::read_to_string(path) else { return Vec::new(); };
    content
        .lines()
        .map(|line| line.split(';').map(|s| s.to_string()).collect::<Vec<_>>())
        .collect()
}

fn transpose_csv(table: &[Vec<String>]) -> Vec<Vec<String>> {
    let rows = table.len();
    let cols = table.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut out = vec![vec![String::new(); rows]; cols];
    for (r, row) in table.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            out[c][r] = cell.clone();
        }
    }
    out
}

fn get_all_brueche(table: &[Vec<String>]) -> Vec<SimpleFraction> {
    let mut set = BTreeSet::new();
    for (i, row) in table.iter().enumerate().skip(1) {
        for (k, cell) in row.iter().enumerate().skip(1) {
            if cell.trim().chars().count() > 3 {
                if let Some(fr) = SimpleFraction::new(i + 1, k + 1) {
                    if fr.den != 1 && fr.num != 1 {
                        set.insert(fr);
                    }
                }
            }
        }
    }
    set.into_iter().collect()
}

fn fraction_source_text(
    table: &Table,
    frac: SimpleFraction,
    n_and_invers_cols: (usize, usize),
    gebr_table: &[Vec<String>],
    is_not_universe: bool,
) -> Option<String> {
    let is_universe = !is_not_universe;
    if frac.den == 0 || frac.num == 0 {
        return Some(String::new());
    }
    if frac.den > 100 || frac.num > 100 {
        return None;
    }
    if frac.num == 1 {
        let main = get_cell(table, frac.den, n_and_invers_cols.1).trim().to_string();
        if main.chars().count() <= 3 {
            return Some(String::new());
        }
        if is_universe {
            let extra = get_cell(table, frac.den, 201).trim();
            if extra.chars().count() > 2 {
                return Some(format!("{} (1/{}) ; {}", main, frac.den, extra));
            }
        }
        return Some(main);
    }
    if frac.den == 1 {
        let main = get_cell(table, frac.num, n_and_invers_cols.0).trim().to_string();
        if main.chars().count() <= 3 {
            return Some(String::new());
        }
        if is_universe {
            let extra = get_cell(table, frac.num, 198).trim();
            if extra.chars().count() > 2 {
                return Some(format!("{} ({}) ; {}", main, frac.num, extra));
            }
        }
        return Some(main);
    }
    let r = frac.num - 1;
    let c = frac.den - 1;
    Some(gebr_table.get(r).and_then(|row| row.get(c)).cloned().unwrap_or_default())
}

fn add_pair_unique(map: &mut BTreeMap<usize, Vec<(SimpleFraction, SimpleFraction)>>, key: usize, pair: (SimpleFraction, SimpleFraction)) {
    let vec = map.entry(key).or_default();
    let canon = if pair.0 <= pair.1 { pair } else { (pair.1, pair.0) };
    if !vec.iter().any(|&(a,b)| { let c = if a <= b {(a,b)} else {(b,a)}; c == canon }) {
        vec.push(pair);
    }
}

fn build_fraction_pairs_for_row(max_row: usize, poly: usize, combo: usize) -> BTreeMap<usize, Vec<(SimpleFraction, SimpleFraction)>> {
    let gal_csv = read_semicolon_csv("gebrochen-rational-galaxie.csv");
    let uni_csv = read_semicolon_csv("gebrochen-rational-universum.csv");
    let gal_fracs = get_all_brueche(&gal_csv);
    let uni_fracs = get_all_brueche(&uni_csv);
    let (fracs1, fracs2) = match combo {
        0 => (&gal_fracs, &gal_fracs),
        1 => (&gal_fracs, &uni_fracs),
        2 => (&uni_fracs, &gal_fracs),
        3 => (&uni_fracs, &uni_fracs),
        _ => (&gal_fracs, &gal_fracs),
    };
    let gleichf = poly == 1;
    let mut out: BTreeMap<usize, Vec<(SimpleFraction, SimpleFraction)>> = BTreeMap::new();

    for &a in fracs1 {
        for &b in fracs2 {
            if a == b { continue; }
            let value = if gleichf { a.mul(b).and_then(|x| x.inv()) } else { a.mul(b) };
            if let Some(v) = value {
                if v.is_integer() && v.num <= max_row {
                    add_pair_unique(&mut out, v.num, (a, b));
                }
            }
        }
    }

    if !gleichf {
        for &frac in fracs1 {
            for zusatz in 1..=max_row {
                let Some(f2) = SimpleFraction::new(frac.den.saturating_mul(zusatz), 1) else { continue; };
                let Some(v) = frac.mul(f2) else { continue; };
                if !v.is_integer() { continue; }
                if v.num > max_row { break; }
                add_pair_unique(&mut out, v.num, (frac, f2));
            }
        }
        let fracs2set: BTreeSet<SimpleFraction> = fracs2.iter().copied().collect();
        for &frac in fracs1 {
            for zusatz in (1..=max_row).rev() {
                let Some(faktor) = SimpleFraction::new(frac.den, zusatz) else { continue; };
                if fracs2set.contains(&faktor) || faktor.num == 1 {
                    let Some(v) = frac.mul(faktor) else { continue; };
                    if v.is_integer() && v.num <= max_row {
                        add_pair_unique(&mut out, v.num, (frac, faktor));
                    }
                }
            }
        }
    } else {
        for &frac in fracs1 {
            for zusatz in 1..=max_row {
                let Some(f2) = SimpleFraction::new(1, frac.num.saturating_mul(zusatz)) else { continue; };
                let Some(prod) = frac.mul(f2) else { continue; };
                let Some(v) = prod.inv() else { continue; };
                if !v.is_integer() { continue; }
                if v.num > max_row { break; }
                add_pair_unique(&mut out, v.num, (frac, f2));
            }
        }
        let fracs2set: BTreeSet<SimpleFraction> = fracs2.iter().copied().collect();
        for &frac in fracs1 {
            let Some(inv_frac) = frac.inv() else { continue; };
            for zusatz in 1..=max_row {
                let Some(faktor) = SimpleFraction::new(inv_frac.num, inv_frac.den.saturating_mul(zusatz)) else { continue; };
                if fracs2set.contains(&faktor) || faktor.num == 1 {
                    let Some(prod) = frac.mul(faktor) else { continue; };
                    let Some(v) = prod.inv() else { continue; };
                    if v.is_integer() && v.num <= max_row {
                        add_pair_unique(&mut out, v.num, (frac, faktor));
                    }
                }
            }
        }
    }
    out
}

fn exact_gebr_prim_source(table: &Table, poly: usize, combo: usize, pair: (SimpleFraction, SimpleFraction)) -> Option<(String, String)> {
    let gal_csv = read_semicolon_csv("gebrochen-rational-galaxie.csv");
    let gal_t = transpose_csv(&gal_csv);
    let gal_cols = (10usize, 42usize);
    let uni_cols = (5usize, 131usize);
    let gal_or_uni = match combo {
        0 => (gal_cols, gal_cols),
        1 => (gal_cols, uni_cols),
        2 => (uni_cols, gal_cols),
        3 => (uni_cols, uni_cols),
        _ => (gal_cols, gal_cols),
    };
    let n_and_invers = if poly == 0 { gal_or_uni.0 } else { gal_or_uni.1 };
    let (left_table, left_not_universe) = if combo >= 2 { (&gal_csv, false) } else { (&gal_t, true) };
    let (right_table, right_not_universe) = if combo == 1 || combo == 3 { (&gal_csv, false) } else { (&gal_t, true) };
    let left = fraction_source_text(table, pair.0, n_and_invers, left_table, left_not_universe)?;
    let right = fraction_source_text(table, pair.1, n_and_invers, right_table, right_not_universe)?;
    Some((left, right))
}


fn normalize_generated_operand(text: &str) -> String {
    let mut s = text.trim().replace(r#"\""#, r#"""#).trim().to_string();

    loop {
        let before = s.clone();
        let trimmed = s.trim();

        let mut start = 0usize;
        let mut end = trimmed.len();

        while start < end {
            let rest = &trimmed[start..end];
            if rest.starts_with('"') || rest.starts_with('„') || rest.starts_with('“') {
                start += rest.chars().next().map(|c| c.len_utf8()).unwrap_or(0);
            } else {
                break;
            }
        }

        while end > start {
            let rest = &trimmed[start..end];
            if rest.ends_with('"') || rest.ends_with('“') || rest.ends_with('”') {
                let ch_len = rest.chars().next_back().map(|c| c.len_utf8()).unwrap_or(0);
                end -= ch_len;
            } else {
                break;
            }
        }

        s = trimmed[start..end].trim().replace("\"\"", "\"");

        if s == before {
            break;
        }
    }

    s.trim().replace("\"\"", "\"")
}


fn plain_prim_source(table: &Table, row: usize, poly: usize, role: usize, combo: usize) -> String {
    let stern_motiv = 10usize;
    let stern_struk = 5usize;
    let gleichf_motiv = 42usize;
    let gleichf_struk = 131usize;
    let (motiv_col, struk_col) = if poly == 0 { (stern_motiv, stern_struk) } else { (gleichf_motiv, gleichf_struk) };
    let left_col = if combo <= 1 { motiv_col } else { struk_col };
    let right_col = if combo == 0 || combo == 2 { motiv_col } else { struk_col };
    let col = if role == 0 { left_col } else { right_col };
    get_cell(table, row, col).trim().to_string()
}

pub fn concat_prim_universe_generated(
    table: &mut Table,
    rows_as_numbers: &mut RowSet,
    tables: &mut Tables,
    gebr: bool,
    poly: usize,
    kind: usize,
) {
    let combos: [usize; 3] = if kind == 0 { [0, 1, 2] } else { [1, 2, 3] };
    let poly_name = if poly == 0 { "Sternpolygone" } else { "gleichförmige Polygone" };
    let kind_name = if kind == 0 { "Motiv" } else { "Struktur" };
    let combo_names = ["Motiv -> Motiv", "Motiv -> Struktur", "Struktur -> Motiv", "Struktur -> Struktur"];
    let suffix = if gebr { ", mit Faktoren aus gebrochen-rationalen Zahlen" } else { "" };

    let exact_fraction_pairs: BTreeMap<usize, BTreeMap<usize, Vec<(SimpleFraction, SimpleFraction)>>> = if gebr {
        combos.iter().copied().map(|combo| (combo, build_fraction_pairs_for_row(table.len().saturating_sub(1), poly, combo))).collect()
    } else {
        BTreeMap::new()
    };

    for combo in combos {
        let mut values = vec![String::new(); table.len()];
        if !values.is_empty() {
            values[0] = format!("generierte Multiplikationen {} {} {}{}", poly_name, kind_name, combo_names[combo], suffix);
        }

        for i in 1..table.len() {
            let mut parts: Vec<String> = Vec::new();

            if gebr {
                if let Some(pairs_for_rows) = exact_fraction_pairs.get(&combo) {
                    if let Some(pairs) = pairs_for_rows.get(&i) {
                        for &pair in pairs {
                            if let Some((left, right)) = exact_gebr_prim_source(table, poly, combo, pair) {
                                let left_clean = normalize_generated_operand(&left);
                                let right_clean = normalize_generated_operand(&right);
                                if left_clean.chars().count() > 3 && right_clean.chars().count() > 3 {
                                    let frac_left = format!("{}/{}", pair.0.num, pair.0.den);
                                    let frac_right = format!("{}/{}", pair.1.num, pair.1.den);
                                    parts.push(format!(r#"("{}") ({})*({}) ("{}")"#, left_clean, frac_left, frac_right, right_clean));
                                }
                            }
                        }
                    }
                }
            } else {
                for (a, b) in prim_multiple(i) {
                    if a == 0 || b == 0 {
                        continue;
                    }
                    let left = plain_prim_source(table, a, poly, 0, combo);
                    let right = plain_prim_source(table, b, poly, 1, combo);
                    let left_clean = normalize_generated_operand(&left);
                    let right_clean = normalize_generated_operand(&right);
                    if left_clean.chars().count() > 2 && right_clean.chars().count() > 2 {
                        parts.push(format!(r#"("{}") * ("{}")"#, left_clean, right_clean));
                    }
                }
            }

            values[i] = unique_preserve_order(parts).join(" | außerdem: ");
        }

        append_generated_col(table, values);
        let tag_poly = if poly == 0 { ST::SternPolygon } else { ST::GleichfoermigesPolygon };
        let mut tags = vec![tag_poly];
        if gebr {
            tags.push(ST::Galaxie);
            tags.push(ST::Universum);
        } else {
            tags.push(ST::Galaxie);
        }
        register_generated_column(
            tables,
            rows_as_numbers,
            table,
            tagset(&tags),
            format!("{} {} {}{}", poly_name, kind_name, combo_names[combo], suffix),
        );
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConcatState {
    pub ones: BTreeSet<usize>,
    pub csvs_already_read: BTreeMap<usize, String>,
    pub csvs_same: BTreeMap<usize, Vec<usize>>,
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
    source_text: String,
) {
    let new_col = table.first().map(|r| r.len() - 1).unwrap_or(0);
    rows_as_numbers.insert(new_col);
    tables
        .generated_spalten_parameter_tags
        .insert(new_col, tags);

    if tables.generated_spalten_parameter.contains_key(&new_col) {
        panic!("generated_spalten_parameter key collision");
    }
    tables.generated_spalten_parameter.insert(new_col, source_text);
}

fn get_cell(table: &Table, row: usize, col: usize) -> &str {
    table
        .get(row)
        .and_then(|r| r.get(col))
        .map(|s| s.as_str())
        .unwrap_or("")
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
fn could_be_prime_number_primzahlkreuz_fuer_innen(n: usize) -> bool {
    matches!(n % 24, 5 | 11 | 17 | 23)
}
fn could_be_prime_number_primzahlkreuz_fuer_aussen(n: usize) -> bool {
    matches!(n % 24, 1 | 7 | 13 | 19)
}
fn prim_repeat(mut factors: Vec<usize>) -> Vec<(usize, usize)> {
    if factors.is_empty() { return Vec::new(); }
    factors.reverse();
    let mut c = 1usize;
    let mut b: Option<usize> = None;
    let mut d: Vec<(usize, usize)> = Vec::new();
    for a in factors {
        if b == Some(a) { c += 1; } else { c = 1; }
        d.push((a, c));
        b = Some(a);
    }
    d.reverse();
    let mut out: Vec<(usize, usize)> = Vec::new();
    let mut last_prime: Option<usize> = None;
    for (e, g) in d {
        if last_prime != Some(e) { out.push((e, g)); }
        last_prime = Some(e);
    }
    out
}
fn divisors(n: usize) -> BTreeSet<usize> {
    let mut out = BTreeSet::new();
    if n == 0 { return out; }
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
    if num == 0 { return 0; }
    let fak = prim_repeat(primfaktoren(num));
    if fak.len() == 1 && fak[0].1 == 1 { return 1; }
    if fak.len() == 1 { return 3; }
    if fak.is_empty() { return 0; }
    let mut schnittmenge: Option<BTreeSet<usize>> = None;
    for (_, prim_amount) in &fak {
        let mut ds = divisors(*prim_amount);
        ds.remove(&1);
        if ds.is_empty() { schnittmenge = None; break; }
        schnittmenge = match schnittmenge {
            None => Some(ds),
            Some(prev) => Some(prev.intersection(&ds).copied().collect()),
        };
    }
    match schnittmenge { Some(s) if !s.is_empty() => 3, _ => 2 }
}
fn moon_number(num: usize) -> (Vec<usize>, Vec<usize>) {
    if num < 2 { return (Vec::new(), Vec::new()); }
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
    if n == 0 { return Vec::new(); }
    let mut multiples = vec![(1, n)];
    for (prim, _) in prim_repeat(primfaktoren(n)) { multiples.push((prim, n / prim)); }
    multiples
}
fn tagset(tags: &[ST]) -> BTreeSet<ST> { tags.iter().copied().collect() }

pub fn gleichheit_freiheit_vergleich(zahl: usize) -> String {
    let mut out = Vec::new();
    if zahl % 4 == 0 { out.push("Dominieren, Unterordnen".to_string()); }
    if zahl % 4 == 1 { out.push("Freiheit".to_string()); }
    if zahl % 4 == 3 { out.push("Einschränkung der Freiheit".to_string()); }
    if zahl % 4 == 2 {
        if zahl >= 2 && (zahl - 2) % 8 == 0 { out.push("Gleichheit".to_string()); }
        if zahl >= 6 && (zahl - 6) % 16 == 0 { out.push("den anderen überbieten wollen".to_string()); }
        if zahl >= 14 && (zahl - 14) % 16 == 0 { out.push("den anderen unterbieten wollen".to_string()); }
    }
    out.join("; ")
}

pub fn geist_emotion_energie_materie_topologie(zahl: usize) -> String {
    let pr_fa = primfaktoren(zahl);
    let auss: Vec<bool> = pr_fa.iter().map(|&a| could_be_prime_number_primzahlkreuz_fuer_aussen(a)).collect();
    let innen: Vec<bool> = pr_fa.iter().map(|&a| could_be_prime_number_primzahlkreuz_fuer_innen(a)).collect();
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
    if denken { out.push("eine Denkart".to_string()); }
    if gefuehl { out.push("eine Gefühlsart".to_string()); }
    if total_materie { out.push("total eine Art, etwas geistig zu erzeugen".to_string()); }
    if total_topologie { out.push("total eine Art zu erleben".to_string()); }
    if total_energie { out.push("total eine Energie-Art".to_string()); }
    if etwas_topologie { out.push("etwas eine Art zu erleben".to_string()); }
    if etwas_materie { out.push("etwas eine Art, etwas geistig zu erzeugen".to_string()); }
    if wenig_materie { out.push("wenig eine Art, etwas geistig zu erzeugen".to_string()); }
    if einigermassen_energie { out.push("einigermaßen eine Energie-Art".to_string()); }
    if kaum_energie { out.push("kaum eine Energie-Art".to_string()); }
    if kaum_materie { out.push("kaum eine Art, etwas geistig zu erzeugen".to_string()); }
    out.join("; ")
}

pub fn concat_love_polygon(table: &mut Table, rows_as_numbers: &mut RowSet, tables: &mut Tables) {
    let values: Vec<String> = (0..table.len())
        .map(|i| {
            let c8 = get_cell(table, i, 8).trim();
            let c4 = get_cell(table, i, 4);
            if c8.is_empty() { String::new() } else { format!("{c8} der eigenen Strukturgröße ({c4}) auf dich bei gleichförmigen Polygonen") }
        })
        .collect();
    append_generated_col(table, values);
    register_generated_column(
        tables,
        rows_as_numbers,
        table,
        tagset(&[ST::SternPolygon, ST::Galaxie, ST::GleichfoermigesPolygon]),
        tables.data_dict.get(&0).and_then(|m| m.get(&9)).cloned().unwrap_or_default(),
    );
}

pub fn concat_gleichheit_freiheit_dominieren(table: &mut Table, rows_as_numbers: &mut RowSet, tables: &mut Tables) {
    let values: Vec<String> = (0..=tables.last_line_number.min(table.len().saturating_sub(1)))
        .map(|i| if i == 0 { "Gleichheit, Freiheit, Dominieren (Ordnungen [12]) Generiert".to_string() } else { gleichheit_freiheit_vergleich(i) })
        .collect();
    append_generated_col(table, values);
    register_generated_column(
        tables, rows_as_numbers, table,
        tagset(&[ST::SternPolygon, ST::Universum]),
        tables.data_dict.get(&0).and_then(|m| m.get(&132)).cloned().unwrap_or_default(),
    );
}

pub fn concat_geist_emotion_energie_materie_topologie(table: &mut Table, rows_as_numbers: &mut RowSet, tables: &mut Tables) {
    let values: Vec<String> = (0..=tables.last_line_number.min(table.len().saturating_sub(1)))
        .map(|i| if i == 0 { "Energie oder Denkart oder Gefühlsart oder Materie-Art oder Topologie-Art".to_string() } else { geist_emotion_energie_materie_topologie(i) })
        .collect();
    append_generated_col(table, values);
    register_generated_column(
        tables, rows_as_numbers, table,
        tagset(&[ST::SternPolygon, ST::Universum]),
        tables.data_dict.get(&0).and_then(|m| m.get(&242)).cloned().unwrap_or_default(),
    );
}

pub fn concat_prim_creativity_type(table: &mut Table, rows_as_numbers: &mut RowSet, tables: &mut Tables) {
    let end = tables.last_line_number.min(table.len().saturating_sub(1));
    let values: Vec<String> = (0..=end)
        .map(|i| if i == 0 { "Evolutions-Züchtungs-Kreativität".to_string() } else { match prim_creativity(i) { 0 => "0. Primzahl 1".to_string(), 1 => "1. Primzahl und Sonnenzahl".to_string(), 2 => "2. Sonnenzahl, aber keine Primzahl".to_string(), _ => "3. Mondzahl".to_string() } })
        .collect();
    append_generated_col(table, values);
    register_generated_column(
        tables, rows_as_numbers, table,
        tagset(&[ST::SternPolygon, ST::Galaxie]),
        tables.data_dict.get(&0).and_then(|m| m.get(&64)).cloned().unwrap_or_default(),
    );
}

pub fn concat_mond_exponzieren_logarithmus_typ(table: &mut Table, rows_as_numbers: &mut RowSet, tables: &mut Tables) {
    let pairs = [(44usize, "Mond-Typ eines Sternpolygons"), (56usize, "Mond-Typ eines gleichförmigen Polygons")];
    for (rownum, rowheading) in pairs {
        let end = tables.last_line_number.min(table.len().saturating_sub(1));
        let values: Vec<String> = (0..=end)
            .map(|i| {
                if i == 0 { return rowheading.to_string(); }
                let (bases, exponents_minus_2) = moon_number(i);
                if bases.is_empty() { return "kein Mond".to_string(); }
                let mut parts = Vec::new();
                for (k, (basis, exponent_minus_2)) in bases.into_iter().zip(exponents_minus_2.into_iter()).enumerate() {
                    let mut insert = get_cell(table, basis, rownum).trim_end().to_string();
                    insert = insert.replace("<SG>", get_cell(table, i, 4).trim());
                    insert = insert.replace("&lt;SG&gt;", get_cell(table, i, 4).trim());
                    let mut s = String::new();
                    if k > 0 { s.push_str(" | "); }
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
        let tags = if rownum == 44 { tagset(&[ST::SternPolygon, ST::Universum, ST::Galaxie]) } else { tagset(&[ST::GleichfoermigesPolygon, ST::Universum, ST::Galaxie]) };
        register_generated_column(
            tables, rows_as_numbers, table, tags,
            tables.data_dict.get(&0).and_then(|m| m.get(&64)).cloned().unwrap_or_default(),
        );
    }
}

pub fn concat_vervielfache_zeile(table: &mut Table, rows_as_numbers: &RowSet, tables: &Tables) {
    let spalten_to_vervielfache: Vec<usize> = rows_as_numbers.iter().copied().filter(|s| *s == 90 || *s == 19).collect();
    for s in spalten_to_vervielfache {
        let mut store: HashMap<(usize, usize), String> = HashMap::new();
        for z in 2..=tables.last_line_number.min(table.len().saturating_sub(1)) {
            let content = get_cell(table, z, s).to_string();
            if !content.trim().is_empty() { store.insert((z, s), content); }
        }
        let mut multis: HashMap<usize, Vec<usize>> = HashMap::new();
        for ((z, _), _) in &store {
            let mut vielfacher = 1usize;
            let mut ergebnis = vielfacher * *z;
            multis.entry(ergebnis).or_default().push(*z);
            while ergebnis < table.len() {
                vielfacher += 1;
                ergebnis = vielfacher * *z;
                if ergebnis < table.len() { multis.entry(ergebnis).or_default().push(*z); }
            }
        }
        for z in 2..=tables.last_line_number.min(table.len().saturating_sub(1)) {
            let original = get_cell(table, z, s).to_string();
            let mut items = Vec::new();
            if !original.trim().is_empty() { items.push(original.clone()); }
            if let Some(orig_rows) = multis.get(&z) {
                for ur_zeile in orig_rows {
                    if *ur_zeile == z { continue; }
                    if let Some(extra) = store.get(&(*ur_zeile, s)) {
                        if !extra.is_empty() && !items.contains(extra) { items.push(extra.clone()); }
                    }
                }
            }
            let joined = if tables.html_output_yes {
                format!("<ul>{}</ul>", items.into_iter().map(|x| format!("<li>{x}</li>")).collect::<String>())
            } else if tables.bbcode_output_yes {
                format!("[list]{}[/list]", items.into_iter().map(|x| format!("[*]{x}")).collect::<String>())
            } else {
                items.join(" | ")
            };
            if let Some(row) = table.get_mut(z) {
                if let Some(cell) = row.get_mut(s) { *cell = joined; }
            }
        }
    }
}

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
    if !get_cell(table, line, 97).is_empty() { modaloperators.push(get_cell(table, line, 97).to_string()); }
    if !get_cell(table, line, 98).is_empty() { modaloperators.push(get_cell(table, line, 98).to_string()); }
    let begin = line + 1;
    let end = line + (line.saturating_sub(1)) + 1;
    for coord in begin..end {
        let c = get_cell(table, coord, 42);
        if !c.is_empty() { modaloperators.push(c.to_string()); }
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
    let Some(couples) = vorkommen_vielfacher.get(&i_with_distance) else { return; };
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


fn eigenschaft_standard_familie_for_modal_concept(
    left: usize,
    right: usize,
) -> Option<EigenschaftStandardFamilie> {
    if let Some(key) = EigenschaftKeyId::from_modal_pair(left, right) {
        return Some(key.standard_familie());
    }

    let left_family = EigenschaftKeyId::from_any_column(left).map(|k| k.standard_familie());
    let right_family = EigenschaftKeyId::from_any_column(right).map(|k| k.standard_familie());

    match (left_family, right_family) {
        (Some(EigenschaftStandardFamilie::EinsDurchN), _) => Some(EigenschaftStandardFamilie::EinsDurchN),
        (_, Some(EigenschaftStandardFamilie::EinsDurchN)) => Some(EigenschaftStandardFamilie::EinsDurchN),
        (Some(EigenschaftStandardFamilie::N), _) => Some(EigenschaftStandardFamilie::N),
        (_, Some(EigenschaftStandardFamilie::N)) => Some(EigenschaftStandardFamilie::N),
        _ => None,
    }
}

pub fn concat_modallogik(table: &mut Table, concepts_rows_set_of_tuple: &BTreeSet<(usize, usize)>, rows_as_numbers: &mut RowSet, tables: &mut Tables) {
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
            if i == 0 { into.insert(i, vec!["Generiert: ".to_string(), get_cell(&table_copy, i, concept.0).to_string()]); }
            else if !get_cell(&table_copy, i, concept.0).trim().is_empty() { ein_mal_vorkommen.insert(i); }
        }
        let mut vorkommen_vielfacher: VorkommenVielfacher = BTreeMap::new();
        for ein_vorkommen in ein_mal_vorkommen {
            let mut vielfacher = 1usize;
            let mut ergebnis = vielfacher * ein_vorkommen;
            vorkommen_vielfacher.entry(ergebnis as isize).or_default().push((ein_vorkommen, vielfacher));
            while ergebnis < table_copy.len() {
                vielfacher += 1;
                ergebnis = vielfacher * ein_vorkommen;
                if ergebnis < table_copy.len() { vorkommen_vielfacher.entry(ergebnis as isize).or_default().push((ein_vorkommen, vielfacher)); }
            }
        }
        let mut vorkommen_vielfacher_b: VorkommenVielfacherB = BTreeMap::new();
        for i in 1..=end { for distance in distances { prepare_modal_into_table(distance, i, &vorkommen_vielfacher, &mut vorkommen_vielfacher_b, &table_copy); } }
        for i in 1..=end {
            for distance in distances {
                let Some(distance_map) = vorkommen_vielfacher_b.get(&i) else { continue; };
                let Some(entry) = distance_map.get(&distance) else { continue; };
                for (modal_operatoren, vervielfachter) in entry.modal_s.iter().zip(entry.vervielfachter.iter()) {
                    let into_its_content = if distance.abs() % 2 == 0 { get_cell(&table_copy, *vervielfachter, concept.0) } else { get_cell(&table_copy, *vervielfachter, concept.1) };
                    let prefix = match distance.abs() { 2 => "mittelstark überdurchschnittlich: ", 1 => "überdurchschnittlich: ", 3 => "mittelleicht überdurchschnittlich: ", 0 => "sehr: ", _ => "sehr leicht überdurchschnittlich: ", };
                    let mut piece = String::new();
                    piece.push_str(prefix);
                    if let Some(op0) = modal_operatoren.get(0) { piece.push_str(op0); piece.push(' '); }
                    let normalized = if modal_operatoren.get(0).map(|s| s.as_str()) == Some(get_cell(&table_copy, 1, 97)) { into_its_content.to_string() } else { into_its_content.replace("intrinsisch", "zuerst").replace("extrinsisch", "als zweites") };
                    piece.push_str(&normalized);
                    if let Some(op1) = modal_operatoren.get(1) { piece.push(' '); piece.push_str(op1); }
                    if distance.abs() % 2 == 1 && modal_operatoren.len() > 2 {
                        piece.push_str(", nicht: ");
                        piece.push_str(&modal_operatoren[2..].join(", "));
                        piece.push_str(" (das alles nicht): ");
                        piece.push_str(&get_cell(&table_copy, *vervielfachter, concept.0).replace("extrinsisch", "als zweites").replace("intrinsisch", "zuerst"));
                    }
                    into.entry(i).or_default().push(piece);
                }
            }
            let condition_n_vs_1_per_n = matches!(
            eigenschaft_standard_familie_for_modal_concept(concept.0, concept.1),
            Some(EigenschaftStandardFamilie::EinsDurchN)
        );
            let fill_ = if condition_n_vs_1_per_n { get_cell(&table_copy, i, 197) } else { get_cell(&table_copy, i, 4) };
            if let Some(parts) = into.get_mut(&i) {
                if !(parts.len() == 1 && parts[0].is_empty()) { parts.push(format!("Alles nur bezogen auf Satz {}", fill_)); }
            }
        }
        let values: Vec<String> = (0..=end).map(|w| into.remove(&w).unwrap_or_default().join(" | ")).collect();
        append_generated_col(table, values);
        let condition_n_vs_1_per_n = matches!(
            eigenschaft_standard_familie_for_modal_concept(concept.0, concept.1),
            Some(EigenschaftStandardFamilie::EinsDurchN)
        );
        register_generated_column(
            tables,
            rows_as_numbers,
            table,
            if condition_n_vs_1_per_n { tagset(&[ST::GleichfoermigesPolygon, ST::Galaxie]) } else { tagset(&[ST::SternPolygon, ST::Galaxie]) },
            tables.data_dict.get(&1).and_then(|m| m.get(&concept.0)).cloned().unwrap_or_default(),
        );
    }
}

pub fn concat1_primzahlkreuz_pro_contra(table: &mut Table, rows_as_numbers: &mut RowSet, tables: &mut Tables, _generated_befehle: &BTreeSet<String>, parameters_main: &ParametersMain) {
    let dreli = table.clone();
    let max_num = tables.hoechste_zeile_1024.max(dreli.len().saturating_sub(1));
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
        let mut into: Vec<String> = if num == 0 { vec![headline.clone()] } else { Vec::new() };
        let mut into1: Vec<String> = Vec::new();
        let mut into2: Vec<String> = Vec::new();
        if prim_creativity(num) == 1 || num == 1 {
            if could_be_prime_number_primzahlkreuz_fuer_innen(num) {
                list1.push(num);
                if num > 16 {
                    let gegen = if keine_primzahl_1 {
                        if let Some(&g) = list2.get(weiter1b + 1) { weiter1b += 1; g } else { continue; }
                    } else if let Some(&g) = list1.get(weiter1a) { weiter1a += 1; g } else { continue; };
                    contra_contra.insert(num, gegen);
                    contra_contra2.entry(num).or_default().insert(gegen);
                    into1.push(format!("gegen {}", gegen));
                } else if num == 11 || num == 5 {
                    let gegen = 2;
                    contra_contra.insert(num, gegen);
                    contra_contra2.entry(num).or_default().insert(gegen);
                    into1.push("gegen 2".to_string());
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
                        let p = list1[weiter2b + 1]; weiter2b += 1; p
                    } else {
                        let p = list2[weiter2a]; weiter2a += 1; p
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
            if could_be_prime_number_primzahlkreuz_fuer_innen(num) { keine_primzahl_1 = true; }
            else if could_be_prime_number_primzahlkreuz_fuer_aussen(num) { keine_primzahl_2 = true; }
            let mut menge: BTreeSet<(usize, usize)> = BTreeSet::new();
            for (a, b) in prim_multiple(num) { let pair = if a <= b { (a, b) } else { (b, a) }; menge.insert(pair); }
            let paare: Vec<(usize, usize)> = menge.into_iter().collect();
            for couple_a in paare {
                if couple_a.0 != 1 && couple_a.1 != 1 {
                    for couple in [couple_a, (couple_a.1, couple_a.0)] {
                        let firsts: Vec<usize> = if couple.0 != couple.1 { vec![1, 0] } else { vec![1] };
                        for first_or_second in firsts {
                            let idx = first_or_second;
                            let other_idx = if first_or_second == 1 { 0 } else { 1 };
                            let selected = if idx == 0 { couple.0 } else { couple.1 };
                            let other = if other_idx == 0 { couple.0 } else { couple.1 };
                            if could_be_prime_number_primzahlkreuz_fuer_innen(selected) || couple.0 % 2 == 0 || couple.1 % 2 == 0 {
                                if let Some(v) = contra_contra.get(&selected) {
                                    let gegen3 = other * *v;
                                    contra_contra.insert(num, gegen3);
                                    contra_contra2.entry(num).or_default().insert(gegen3);
                                    into1.push(format!("gegen {}", gegen3));
                                }
                            }
                            if could_be_prime_number_primzahlkreuz_fuer_aussen(couple.1) || couple.1 % 3 == 0 || couple.0 % 3 == 0 {
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
        let text = dreli.get(num).and_then(|r| r.get(206)).and_then(|s| s.split('|').nth(1)).unwrap_or("").to_string();
        if !text.is_empty() { into.push(text); }
        into1 = unique_preserve_order(into1);
        into2 = unique_preserve_order(into2);
        let mut into_b = Vec::new();
        if num == 0 { into_b = into; } else {
            if !into1.is_empty() { into_b.push(into1.join(", ")); into_b.push(format!(" Darin kann sich die {} am Besten hineinversetzen.", num)); }
            if !into2.is_empty() { into_b.push(into2.join(", ")); into_b.push(format!(" Darin kann sich die {} am Besten hineinversetzen.", num)); }
            if !into.is_empty() { into_b.push(into.join(", ")); }
        }
        if num < first_generated_column.len() { first_generated_column[num] = into_b.join(" | "); }
    }
    append_generated_col(table, first_generated_column);
    let mut reverse_contra: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for (key, values) in &contra_contra2 { for value in values { reverse_contra.entry(*value).or_default().insert(*key); } }
    let mut reverse_pro: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for (key, values) in &pro_pro2 { for value in values { reverse_pro.entry(*value).or_default().insert(*key); } }
    let mut second_generated_column = vec![String::new(); table.len()];
    for num in 0..table.len() {
        let pro2: Vec<usize> = reverse_pro.get(&num).map(|s| s.iter().copied().collect()).unwrap_or_default();
        let contra2: Vec<usize> = reverse_contra.get(&num).map(|s| s.iter().copied().collect()).unwrap_or_default();
        second_generated_column[num] = if num == 0 { headline.clone() } else if !pro2.is_empty() || !contra2.is_empty() {
            let mut parts = Vec::new();
            if !pro2.is_empty() { parts.push(format!("pro dieser Zahl sind: {:?}", pro2)); }
            if !contra2.is_empty() { parts.push(format!("contra dieser Zahl sind: {:?}", contra2)); }
            parts.push("Darin kann man sich hineinversetzen.".to_string());
            parts.join(" | ")
        } else { "-".to_string() };
    }
    append_generated_col(table, second_generated_column);
    let new_col_1 = table[0].len() - 2;
    let new_col_2 = table[0].len() - 1;
    rows_as_numbers.insert(new_col_1);
    rows_as_numbers.insert(new_col_2);
    tables.generated_spalten_parameter_tags.insert(new_col_1, tagset(&[ST::SternPolygon, ST::Universum]));
    tables.generated_spalten_parameter_tags.insert(new_col_2, tagset(&[ST::SternPolygon, ST::Universum]));
    tables.generated_spalten_parameter.insert(new_col_1, format!("{} | {} | {}", parameters_main.bedeutung0, parameters_main.procontra0, parameters_main.grundstrukturen0));
    tables.generated_spalten_parameter.insert(new_col_2, format!("{} | {} | {}", parameters_main.bedeutung0, parameters_main.procontra0, parameters_main.grundstrukturen0));
}


pub fn generated_befehle_for_request(
    request: &crate::domain::spalten_anfrage::SpaltenAnfrage,
) -> BTreeSet<String> {
    request.generated_befehle_hint().into_iter().collect()
}

pub fn parameters_main_for_request(
    request: &crate::domain::spalten_anfrage::SpaltenAnfrage,
) -> ParametersMain {
    let (bedeutung0, procontra0, grundstrukturen0, unter0) = request.parameters_main_hint();
    ParametersMain {
        bedeutung0: bedeutung0.unwrap_or_default(),
        procontra0: procontra0.unwrap_or_default(),
        grundstrukturen0: grundstrukturen0.unwrap_or_default(),
        unter0: unter0.unwrap_or_default(),
    }
}
