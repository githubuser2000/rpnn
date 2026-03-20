use std::collections::BTreeSet;
use std::process;

use rusqlite::Connection;
use unicode_width::UnicodeWidthStr;

use crate::cli::TextBereich;
use crate::column_manager::{build_column_query, get_column_names};
use crate::data_fetcher::fetch_data_with_stats;
use crate::generated_columns_words_registry::{apply_generated_columns, ParametersMain};
use crate::table_printer::printer::print_table_chunked_with_line_numbers;

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

fn should_use_full_table_for_generated(
    generated_befehle: &BTreeSet<String>,
    parameters_main: &ParametersMain,
) -> bool {
    let mut tokens: BTreeSet<String> =
        generated_befehle.iter().map(|s| normalize_token(s)).collect();

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

    const BEDEUTUNG: &[&str] = &["Bedeutung", "bedeutung"];
    const PROCONTRA: &[&str] = &["Pro_Contra", "procontra", "dagegendafuer"];
    const GRUNDSTRUKTUREN: &[&str] = &["Grundstrukturen", "grundstrukturen"];
    const MENSCHLICHES: &[&str] = &["Menschliches", "menschliches"];
    const UNIVERSUM: &[&str] = &[
        "Universum",
        "universum",
        "transzendentalien",
        "strukturalien",
        "kugel",
        "kugeln",
        "ball",
        "baelle",
        "bälle",
    ];
    const MULTIVERSUM: &[&str] = &["Multiversum", "multiversum"];
    const PLANET: &[&str] = &["Planet_(10_und_oder_12)", "planet"];
    const WICHTIGSTE: &[&str] = &["Wichtigstes_zum_verstehen", "wichtigsteverstehen"];
    const GALAXIE: &[&str] = &["Galaxie", "galaxie", "alteschriften", "kreis", "galaxien", "kreise"];

    const PK_PROCONTRA_ALIASES: &[&str] = &[
        "Primzahlkreuz pro contra",
        "nachvollziehen emotional oder geistig durch Primzahl-Kreuz-Algorithmus",
        "primzahlkreuz",
        "nachvollziehen",
        "primzahlkreuzprocontra",
    ];
    const LOVE_ALIASES: &[&str] = &["Liebe", "liebe", "ethik", "Liebe_(7)"];
    const GLEICHHEIT_ALIASES: &[&str] = &[
        "Gleichheit_Freiheit_Ordnung",
        "Gleichheit_Freiheit",
        "gleichheitfreiheit",
        "ungleichheit",
        "dominieren",
        "gleichheit",
        "freiheit",
        "Ordnung_und_Filterung_12_und_1pro12",
        "ordnen",
        "ordnenundfiltern",
        "filtern",
    ];
    const GEIST_ALIASES: &[&str] = &["Geist__(15)", "Geist_(15)", "geist", "bewusstsein"];
    const MOND64_ALIASES: &[&str] = &[
        "Drittwichtigste",
        "drittwichtigste",
        "Gestirn",
        "gestirn",
        "mond",
        "sonne",
        "planet",
    ];
    const VERVIELFACHE_ALIASES: &[&str] = &[
        "Zweitwichtigste",
        "zweitwichtigste",
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
    ];

    tokens.contains("primzahlkreuzprocontra")
        || selected_by_pair(&tokens, PROCONTRA, PK_PROCONTRA_ALIASES)
        || selected_by_pair(&tokens, BEDEUTUNG, PK_PROCONTRA_ALIASES)
        || selected_by_pair(&tokens, GRUNDSTRUKTUREN, PK_PROCONTRA_ALIASES)
        || selected_by_pair(&tokens, MENSCHLICHES, LOVE_ALIASES)
        || selected_by_pair(&tokens, GRUNDSTRUKTUREN, LOVE_ALIASES)
        || selected_by_pair(&tokens, PLANET, GLEICHHEIT_ALIASES)
        || selected_by_pair(&tokens, MENSCHLICHES, GLEICHHEIT_ALIASES)
        || selected_by_pair(&tokens, GRUNDSTRUKTUREN, GLEICHHEIT_ALIASES)
        || selected_by_pair(&tokens, UNIVERSUM, GEIST_ALIASES)
        || selected_by_pair(&tokens, MULTIVERSUM, GEIST_ALIASES)
        || selected_by_pair(&tokens, GRUNDSTRUKTUREN, GEIST_ALIASES)
        || selected_by_pair(&tokens, WICHTIGSTE, MOND64_ALIASES)
        || selected_by_pair(&tokens, BEDEUTUNG, MOND64_ALIASES)
        || selected_by_pair(&tokens, WICHTIGSTE, VERVIELFACHE_ALIASES)
        || selected_by_pair(&tokens, BEDEUTUNG, VERVIELFACHE_ALIASES)
        || selected_by_pair(&tokens, GALAXIE, VERVIELFACHE_ALIASES)
        || contains_any_alias(&tokens, &["vielfache", "vielfacher", "primzahlen"])
}

fn build_full_table_row_query(column_names: &[String], bereich: &TextBereich) -> String {
    let columns = column_names
        .iter()
        .map(|name| format!("\"{}\"", name.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(", ");

    if !bereich.zeilen_bereiche.is_empty() {
        let mut all_row_numbers = Vec::new();

        for &(start, end) in &bereich.zeilen_bereiche {
            if start == 0 || end == 0 || start > end {
                continue;
            }

            for row in start..=end {
                all_row_numbers.push(row);
            }
        }

        all_row_numbers.sort_unstable();
        all_row_numbers.dedup();

        if !all_row_numbers.is_empty() {
            let row_numbers_str = all_row_numbers
                .iter()
                .map(|n| (n - 1).to_string())
                .collect::<Vec<_>>()
                .join(", ");

            return format!(
                "SELECT {} FROM (
                    SELECT *, ROW_NUMBER() OVER (ORDER BY rowid) - 1 as row_num
                    FROM csv_data
                ) numbered_data
                WHERE row_num IN ({})
                ORDER BY row_num",
                columns, row_numbers_str
            );
        }
    }

    if bereich.von_zeile > 0 && bereich.bis_zeile >= bereich.von_zeile {
        let anzahl = bereich.bis_zeile - bereich.von_zeile + 1;
        let offset = bereich.von_zeile.saturating_sub(1);

        return format!(
            "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
            columns, anzahl, offset
        );
    }

    format!("SELECT {} FROM csv_data", columns)
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
) -> Result<TextBereich, Box<dyn std::error::Error>> {
    let column_names = get_column_names(conn)?;
    let has_generated_request = !generated_befehle.is_empty();
    let is_generated_mode =
        has_generated_request && should_use_full_table_for_generated(generated_befehle, parameters_main);

    let (query, headers): (String, Vec<String>) = if is_generated_mode {
        bereich.spalten_gefunden = true;
        (
            build_full_table_row_query(&column_names, &bereich),
            sanitize_headers(&column_names),
        )
    } else {
        let (query, headers) = build_column_query(&column_names, &mut bereich)?;
        (query, sanitize_headers(&headers))
    };

    if !bereich.spalten_gefunden {
        println!("❌ FEHLER: Spalten wurden nicht gefunden!");
        process::exit(1);
    }

    let header_lengths: Vec<usize> = headers
        .iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();

    let (data, _max_lengths) =
        fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    let (mut final_headers, mut final_data) = if is_generated_mode {
        (headers.clone(), data.clone())
    } else if !bereich.spaltenreihenfolgeundnurdiese.is_empty() {
        let null_basierte_indizes: Vec<usize> = bereich
            .spaltenreihenfolgeundnurdiese
            .iter()
            .map(|&i| i.saturating_sub(1))
            .collect();

        let sorted_headers = sort_by_indices(&headers, &null_basierte_indizes)
            .unwrap_or_else(|_| headers.clone());

        let sorted_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| {
                sort_by_indices(row, &null_basierte_indizes).unwrap_or_else(|_| row.clone())
            })
            .collect();

        (sorted_headers, sorted_data)
    } else {
        (headers.clone(), data.clone())
    };

    if has_generated_request {
        apply_generated_columns(
            &mut final_headers,
            &mut final_data,
            &bereich,
            generated_befehle,
            parameters_main,
        )?;
    }

    final_headers = sanitize_headers(&final_headers);

    let original_line_numbers = build_original_line_numbers(&bereich, final_data.len());

    print_table_chunked_with_line_numbers(
        &final_headers,
        &final_data,
        &bereich.breiten,
        &original_line_numbers,
        false,
    );

    Ok(bereich)
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
