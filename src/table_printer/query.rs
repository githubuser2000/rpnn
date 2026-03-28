use std::collections::BTreeSet;
use std::process;

use rusqlite::Connection;
use unicode_width::UnicodeWidthStr;

use crate::cli::TextBereich;
use crate::column_manager::{build_column_query, collect_spalten_nummern, get_column_names};
use crate::data_fetcher::fetch_data_with_stats;
use crate::generated_columns_words_registry::{apply_generated_columns, ParametersMain};
use crate::multiples_teiler::teiler_utils::prime_factors;
use crate::table_printer::printer::print_table_chunked_with_line_numbers;

use crate::domain::categories::KategorieMap;
use crate::domain::reverse_request_report::print_reverse_request_pairs_dual;
use crate::domain::spalten_anfrage::SpaltenAnfrage;
use crate::domain::parser::legacy_cli_typed::{matches_any_alias, GeneratedCommandToken, LegacyOberToken};

fn build_original_line_numbers(bereich: &TextBereich, data_len: usize) -> Vec<usize> {
    if !bereich.zeilen_bereiche.is_empty() {
        let mut nums = Vec::new();

        for &(from, to) in &bereich.zeilen_bereiche {
            if from == 0 || to == 0 || from > to {
                continue;
            }

            for n in from..=to {
                nums.push(n);
            }
        }

        nums.sort_unstable();
        nums.dedup();

        if nums.len() > data_len {
            nums.truncate(data_len);
        }

        return nums;
    }

    if bereich.von_zeile > 0 && bereich.bis_zeile >= bereich.von_zeile {
        let mut nums: Vec<usize> = (bereich.von_zeile..=bereich.bis_zeile).collect();

        if nums.len() > data_len {
            nums.truncate(data_len);
        }

        return nums;
    }

    (1..=data_len).collect()
}

fn expand_bereich_rows(
    conn: &Connection,
    bereich: &mut TextBereich,
) -> Result<(), Box<dyn std::error::Error>> {
    if !bereich.expands_with_multiples() && !bereich.expands_with_prime_factors() {
        return Ok(());
    }

    let mut basis: BTreeSet<usize> = BTreeSet::new();

    if !bereich.zeilen_bereiche.is_empty() {
        for &(from, to) in &bereich.zeilen_bereiche {
            if from == 0 || to == 0 || from > to {
                continue;
            }
            for n in from..=to {
                basis.insert(n);
            }
        }
    } else if bereich.von_zeile > 0 && bereich.bis_zeile >= bereich.von_zeile {
        for n in bereich.von_zeile..=bereich.bis_zeile {
            basis.insert(n);
        }
    }

    if basis.is_empty() {
        return Ok(());
    }

    let max_rows: usize = conn.query_row("SELECT COUNT(*) FROM csv_data", [], |row| row.get(0))?;

    let mut result: BTreeSet<usize> = basis.clone();
    let mut prims: BTreeSet<usize> = BTreeSet::new();

    if bereich.expands_with_prime_factors() {
        for &n in &basis {
            for (p, _) in prime_factors(n as i64) {
                if p > 0 {
                    prims.insert(p as usize);
                }
            }
        }
        result.extend(prims.iter().copied());
    }

    if bereich.expands_with_multiples() {
        let mut multiple_sources: BTreeSet<usize> = basis.clone();
        if bereich.expands_with_prime_factors() {
            multiple_sources.extend(prims.iter().copied());
        }

        for n in multiple_sources {
            if n == 0 {
                continue;
            }
            let mut m = n;
            while m <= max_rows {
                result.insert(m);
                match m.checked_add(n) {
                    Some(next) => m = next,
                    None => break,
                }
            }
        }
    }

    let nums: Vec<usize> = result.into_iter().collect();
    if nums.is_empty() {
        return Ok(());
    }

    bereich.zeilen_bereiche = nums.iter().map(|&n| (n, n)).collect();
    bereich.von_zeile = *nums.first().unwrap();
    bereich.bis_zeile = *nums.last().unwrap();

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum QueryHintToken {
    Liebe,
    GleichheitFreiheitOrdnung,
    Geist,
    Mond64,
    Vervielfache,
    PrimMotOrStruk,
}

fn parse_query_hint(input: &str) -> Option<QueryHintToken> {
    if matches_any_alias(input, &["Liebe", "ethik", "Liebe_(7)"]) {
        Some(QueryHintToken::Liebe)
    } else if matches_any_alias(
        input,
        &[
            "Gleichheit_Freiheit_Ordnung",
            "Gleichheit_Freiheit",
            "ungleichheit",
            "dominieren",
            "gleichheit",
            "freiheit",
            "Ordnung_und_Filterung_12_und_1pro12",
            "ordnen",
            "ordnenundfiltern",
            "filtern",
        ],
    ) {
        Some(QueryHintToken::GleichheitFreiheitOrdnung)
    } else if matches_any_alias(input, &["Geist__(15)", "Geist_(15)", "geist", "bewusstsein"]) {
        Some(QueryHintToken::Geist)
    } else if matches_any_alias(
        input,
        &["Drittwichtigste", "Gestirn", "gestirn", "mond", "sonne", "planet"],
    ) {
        Some(QueryHintToken::Mond64)
    } else if matches_any_alias(
        input,
        &[
            "Zweitwichtigste",
            "Primzahlen",
            "primzahlen",
            "vielfache",
            "vielfacher",
            "Offenbarung_des_Johannes",
            "offenbarung",
            "offenbarungdesjohannes",
            "johannes",
            "bibel",
            "offenbarungjohannes",
        ],
    ) {
        Some(QueryHintToken::Vervielfache)
    } else {
        let lower = input.trim().to_lowercase();
        if lower.starts_with("primmotiv") || lower.starts_with("primstruk") {
            Some(QueryHintToken::PrimMotOrStruk)
        } else {
            None
        }
    }
}

fn should_use_full_table_for_generated(
    generated_befehle: &BTreeSet<String>,
    parameters_main: &ParametersMain,
) -> bool {
    let mut ober_tokens: BTreeSet<LegacyOberToken> = BTreeSet::new();
    let mut generated_tokens: BTreeSet<GeneratedCommandToken> = BTreeSet::new();
    let mut hints: BTreeSet<QueryHintToken> = BTreeSet::new();

    for token in generated_befehle {
        if let Some(g) = GeneratedCommandToken::parse(token) {
            generated_tokens.insert(g);
        }
        if let Some(h) = parse_query_hint(token) {
            hints.insert(h);
        }
    }

    for raw in [
        parameters_main.bedeutung0.as_str(),
        parameters_main.procontra0.as_str(),
        parameters_main.grundstrukturen0.as_str(),
        parameters_main.unter0.as_str(),
    ] {
        if raw.is_empty() {
            continue;
        }
        let ober = LegacyOberToken::parse(raw);
        if !matches!(ober, LegacyOberToken::Unknown(_)) {
            ober_tokens.insert(ober);
        }
        if let Some(g) = GeneratedCommandToken::parse(raw) {
            generated_tokens.insert(g);
        }
        if let Some(h) = parse_query_hint(raw) {
            hints.insert(h);
        }
    }

    generated_tokens.contains(&GeneratedCommandToken::PrimzahlkreuzProContra)
        || hints.contains(&QueryHintToken::PrimMotOrStruk)
        || ((ober_tokens.contains(&LegacyOberToken::ProContra)
            || ober_tokens.contains(&LegacyOberToken::Bedeutung)
            || ober_tokens.contains(&LegacyOberToken::Grundstrukturen))
            && generated_tokens.contains(&GeneratedCommandToken::PrimzahlkreuzProContra))
        || ((ober_tokens.contains(&LegacyOberToken::Menschliches)
            || ober_tokens.contains(&LegacyOberToken::Grundstrukturen))
            && hints.contains(&QueryHintToken::Liebe))
        || ((ober_tokens.contains(&LegacyOberToken::Planet)
            || ober_tokens.contains(&LegacyOberToken::Menschliches)
            || ober_tokens.contains(&LegacyOberToken::Grundstrukturen))
            && hints.contains(&QueryHintToken::GleichheitFreiheitOrdnung))
        || ((ober_tokens.contains(&LegacyOberToken::Universum)
            || ober_tokens.contains(&LegacyOberToken::Multiversum)
            || ober_tokens.contains(&LegacyOberToken::Grundstrukturen))
            && hints.contains(&QueryHintToken::Geist))
        || ((ober_tokens.contains(&LegacyOberToken::WichtigstesZumVerstehen)
            || ober_tokens.contains(&LegacyOberToken::Bedeutung))
            && hints.contains(&QueryHintToken::Mond64))
        || ((ober_tokens.contains(&LegacyOberToken::WichtigstesZumVerstehen)
            || ober_tokens.contains(&LegacyOberToken::Bedeutung)
            || ober_tokens.contains(&LegacyOberToken::Galaxie))
            && hints.contains(&QueryHintToken::Vervielfache))
}

fn should_use_full_table_for_requests(
    kategorie_map: &KategorieMap,
    typed_requests: &[SpaltenAnfrage],
    generated_befehle: &BTreeSet<String>,
    parameters_main: &ParametersMain,
) -> bool {
    if typed_requests.is_empty() {
        return should_use_full_table_for_generated(generated_befehle, parameters_main)
            || !generated_befehle.is_empty();
    }

    for request in typed_requests {
        let generated_for_request = kategorie_map
            .infer_generated_request(request)
            .map(|inf| inf.generated_befehle.into_iter().collect::<BTreeSet<String>>())
            .unwrap_or_default();

        let (ober, unter) = request.ober_unter_cli_pair();
        let typed_parameters = ParametersMain {
            bedeutung0: ober.clone(),
            procontra0: ober.clone(),
            grundstrukturen0: ober,
            unter0: unter,
        };

        if should_use_full_table_for_generated(&generated_for_request, &typed_parameters)
            || !generated_for_request.is_empty()
        {
            return true;
        }
    }

    should_use_full_table_for_generated(generated_befehle, parameters_main)
        || !generated_befehle.is_empty()
}

fn build_full_table_row_query(column_names: &[String], _bereich: &TextBereich) -> String {
    let columns = column_names
        .iter()
        .map(|name| format!("\"{}\"", name.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(", ");

    format!("SELECT {} FROM csv_data", columns)
}


fn attach_column_ids_to_headers(headers: &[String], columns_1_based: &[usize]) -> Vec<String> {
    headers
        .iter()
        .enumerate()
        .map(|(idx, header)| {
            let col_1_based = columns_1_based.get(idx).copied().unwrap_or(idx + 1);
            let py_col_0_based = col_1_based.saturating_sub(1);
            format!("{} [[COL:{}]]", header, py_col_0_based)
        })
        .collect()
}

fn sanitize_headers(headers: &[String]) -> Vec<String> {
    headers
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let trimmed = h.trim();
            if trimmed.is_empty() {
                format!("SQL-Spalte {}", i + 1)
            } else {
                trimmed.to_string()
            }
        })
        .collect()
}

pub fn query_column_by_index(
    conn: &Connection,
    mut bereich: TextBereich,
    generated_befehle: &BTreeSet<String>,
    parameters_main: &ParametersMain,
    kategorie_map: &KategorieMap,
    typed_requests: &[SpaltenAnfrage],
) -> Result<(), Box<dyn std::error::Error>> {
   expand_bereich_rows(conn, &mut bereich)?;
    let column_names = get_column_names(conn)?;
    let wants_generated = should_use_full_table_for_requests(
        kategorie_map,
        typed_requests,
        generated_befehle,
        parameters_main,
    );
    let is_generated_mode = wants_generated;

    let (query, headers): (String, Vec<String>) = if is_generated_mode {
        bereich.mark_columns_resolved();
        (
            build_full_table_row_query(&column_names, &bereich),
            sanitize_headers(&column_names),
        )
    } else {
        let (query, headers) = build_column_query(&column_names, &mut bereich)?;
        (query, sanitize_headers(&headers))
    };

    let mut source_columns_1_based: Vec<usize> = if is_generated_mode {
    (1..=headers.len()).collect()
} else if !bereich.exact_visible_columns.is_empty() {
    bereich.exact_visible_columns.clone()
} else {
    let mut temp_bereich = bereich.clone();
    collect_spalten_nummern(&mut temp_bereich)?
};
   source_columns_1_based.dedup();

    if !bereich.columns_resolved() {
        println!("❌ FEHLER: Spalten wurden nicht gefunden!");
        process::exit(1);
    }

    let header_lengths: Vec<usize> = headers
        .iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();

    let (data, _max_lengths) =
        fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    let (mut final_headers, mut final_data) = (headers.clone(), data.clone());

    if wants_generated {
        apply_generated_columns(
            &mut final_headers,
            &mut final_data,
            &bereich,
            generated_befehle,
            parameters_main,
        )?;
    }

    let should_filter_generated_rows_after_build = is_generated_mode;

    if should_filter_generated_rows_after_build {
        let mut selected_line_numbers = build_original_line_numbers(&bereich, usize::MAX);
        selected_line_numbers.sort_unstable();
        selected_line_numbers.dedup();

        if !selected_line_numbers.is_empty() {
            final_data = selected_line_numbers
                .iter()
                .filter_map(|&line_no| final_data.get(line_no.saturating_sub(1)).cloned())
                .collect();
        }
    }

    if !bereich.spaltenreihenfolgeundnurdiese.is_empty() {
        let null_basierte_indizes: Vec<usize> = bereich
            .spaltenreihenfolgeundnurdiese
            .iter()
            .filter_map(|&i| i.checked_sub(1))
            .collect();

        if let Ok(sorted_headers) = sort_by_indices(&final_headers, &null_basierte_indizes) {
            let sorted_data: Vec<Vec<String>> = final_data
                .iter()
                .map(|row| sort_by_indices(row, &null_basierte_indizes).unwrap_or_else(|_| row.clone()))
                .collect();
            final_headers = sorted_headers;
            final_data = sorted_data;
        }
    }

    if matches!(bereich.output_syntax, crate::reta_ausgabe::OutputSyntax::HTML) {
        if final_headers.len() > source_columns_1_based.len() {
            let start = source_columns_1_based.len() + 1;
            source_columns_1_based.extend(start..=final_headers.len());
        }
        final_headers = attach_column_ids_to_headers(&final_headers, &source_columns_1_based);
    }

    final_headers = sanitize_headers(&final_headers);

    let original_line_numbers = build_original_line_numbers(&bereich, final_data.len());

    print_table_chunked_with_line_numbers(
        &final_headers,
        &final_data,
        &bereich.breiten,
        &original_line_numbers,
        bereich.drops_empty_content(),
        bereich.output_syntax,
        bereich.pretty_output,
    );

    print_reverse_request_pairs_dual(kategorie_map, &bereich, generated_befehle);

    Ok(())
}

fn sort_by_indices<T: Clone>(values: &[T], indices: &[usize]) -> Result<Vec<T>, String> {
    if indices.is_empty() {
        return Ok(Vec::new());
    }

    let max_index = indices.iter().max().copied().unwrap_or(0);
    if max_index >= values.len() {
        return Err(format!(
            "Index {} ist außerhalb der Grenzen (0..{})",
            max_index,
            values.len().saturating_sub(1)
        ));
    }

    Ok(indices.iter().map(|&i| values[i].clone()).collect())
}
