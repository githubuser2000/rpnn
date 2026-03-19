// table_printer/query.rs
use std::collections::BTreeSet;
use std::process;
use rusqlite::Connection;
use crate::cli::TextBereich;
use crate::column_manager::{get_column_names, build_column_query};
use crate::data_fetcher::fetch_data_with_stats;
use unicode_width::UnicodeWidthStr;
use crate::generated_columns_words_registry::{apply_generated_columns, ParametersMain};
use crate::table_printer::printer::print_table_chunked_with_line_numbers;

fn normalize_token(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .replace('ä', "ae")
        .replace('ö', "oe")
        .replace('ü', "ue")
        .replace('ß', "ss")
        .replace('-', "")
        .replace('_', "")
        .replace(' ', "")
}

fn token_is(token: &str, aliases: &[&str]) -> bool {
    let token = normalize_token(token);
    aliases.iter().any(|alias| token == normalize_token(alias))
}

fn generated_alias_present(generated_befehle: &BTreeSet<String>, aliases: &[&str]) -> bool {
    generated_befehle
        .iter()
        .any(|token| token_is(token, aliases))
}

/// Best-effort Auflösung für generierte Spaltenpaare aus words.py.
/// Das erste Wort ist die Hauptkategorie (`ParametersMain.*`), das zweite Wort
/// ein Alias des konkreten Eintrags in `paraNdataMatrix`.
///
/// Rückgabe:
/// - `generated_befehle` wird mit dem passenden Generator-Tag ergänzt
/// - `required_columns` bekommt die Basisspalten, die der Generator erwartet
pub fn try_resolve_generated_pair(
    ober: &str,
    unter: &str,
    generated_befehle: &mut BTreeSet<String>,
    required_columns: &mut BTreeSet<usize>,
) -> bool {
    let ober = normalize_token(ober);
    let unter = normalize_token(unter);

    let is_ober = |aliases: &[&str]| aliases.iter().any(|a| ober == normalize_token(a));
    let is_unter = |aliases: &[&str]| aliases.iter().any(|a| unter == normalize_token(a));

    // words.py:
    // - ParametersMain.procontra + ("Primzahlkreuz pro contra", "primzahlkreuz")
    // - ParametersMain.bedeutung + ("Primzahlkreuz pro contra", primzahlkreuzWort)
    // -> {"primzahlkreuzprocontra"}
    if is_ober(&["procontra", "bedeutung", "grundstrukturen"])
        && is_unter(&[
            "primzahlkreuz",
            "nachvollziehen",
            "nachvollziehen emotional oder geistig durch primzahl kreuz algorithmus",
        ])
    {
        generated_befehle.insert("primzahlkreuzprocontra".to_string());
        return true;
    }

    // words.py:
    // ParametersMain.menschliches + ("Liebe", "liebe", "ethik") -> {8, 9, 28, ...}
    // lib4tables_concat.py / generated_columns.rs: concatLovePolygon braucht Basis-Spalte 9
    if is_ober(&["menschliches"]) && is_unter(&["liebe", "ethik"]) {
        generated_befehle.insert("lovepolygon".to_string());
        required_columns.insert(9);
        return true;
    }

    // words.py:
    // ParametersMain.menschliches + ("Gleichheit_Freiheit", "gleichheitfreiheit", "ungleichheit",
    // "dominieren", "gleichheit", "freiheit") -> {132, ...}
    // lib4tables_concat.py / generated_columns.rs: concatGleichheitFreiheitDominieren braucht 132
    if is_ober(&["planet", "menschliches", "grundstrukturen"])
        && is_unter(&[
            "ordnen",
            "ordnung",
            "filterung",
            "gleichheitfreiheit",
            "ungleichheit",
            "dominieren",
            "gleichheit",
            "freiheit",
        ])
    {
        generated_befehle.insert("gleichheitfreiheit".to_string());
        required_columns.insert(132);
        return true;
    }

    // Best-effort aus words.py + concatGeistEmotionEnergieMaterieTopologie:
    // die Generatorspalte hängt an Basis-Spalte 242
    if is_ober(&["universum", "multiversum", "grundstrukturen", "menschliches"])
        && is_unter(&[
            "geist",
            "bewusstsein",
            "emotionen",
            "gefuehle",
            "gefuehl",
            "gefühl",
            "gefühle",
        ])
    {
        generated_befehle.insert("geistemotionenergiematerietopologie".to_string());
        required_columns.insert(242);
        return true;
    }

    // 64-getriebene Generatoren aus generated_columns.rs:
    // concatPrimCreativityType und concatMondExponzierenLogarithmusTyp
    if is_ober(&["wichtigste", "bedeutung"])
        && is_unter(&[
            "gestirn",
            "mond",
            "sonne",
            "planet",
            "evolution",
            "erwerben",
            "persoenlichkeit",
            "persönlichkeit",
            "kreativitaet",
            "kreativität",
            "intelligenz",
        ])
    {
        generated_befehle.insert("primcreativitytype".to_string());
        generated_befehle.insert("mondexponzierenlogarithmustyp".to_string());
        required_columns.insert(64);
        return true;
    }

    // concatVervielfacheZeile arbeitet mit 19 / 90.
    if is_ober(&["bedeutung", "wichtigste", "galaxie"])
        && is_unter(&[
            "primzahlen",
            "vielfache",
            "vielfacher",
            "multis",
            "multiplikationen",
        ])
    {
        generated_befehle.insert("vervielfachezeile".to_string());
        required_columns.insert(19);
        required_columns.insert(90);
        return true;
    }

    false
}

fn requires_full_table_for_generated(generated_befehle: &BTreeSet<String>) -> bool {
    generated_alias_present(generated_befehle, &[
        "primzahlkreuzprocontra",
        "lovepolygon",
        "gleichheitfreiheit",
        "geistemotionenergiematerietopologie",
        "primcreativitytype",
        "mondexponzierenlogarithmustyp",
        "vervielfachezeile",
        "modallogik",
    ])
}

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

        nums.sort();
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

// --- Query-Funktion ---
fn build_full_table_row_query(column_names: &[String]) -> String {
    let columns = column_names
        .iter()
        .map(|name| format!("\"{}\"", name.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(", ");

    format!("SELECT {} FROM csv_data", columns)
}

pub fn query_column_by_index(
    conn: &Connection,
    mut bereich: TextBereich,
    generated_befehle: &BTreeSet<String>,
    parameters_main: &ParametersMain,
) -> Result<TextBereich, Box<dyn std::error::Error>> {
    let column_names = get_column_names(conn)?;

    let (query, headers): (String, Vec<String>) =
        if requires_full_table_for_generated(generated_befehle) {
            println!("ℹ️ Generierte-Spalten-Sonderpfad: lade Volltabelle für Generator");
            bereich.spalten_gefunden = true;
            (build_full_table_row_query(&column_names), column_names.clone())
        } else {
            build_column_query(&column_names, &mut bereich)?
        };

    println!("Headerslänge vor Sortierung: {}", headers.len());
    if !bereich.spalten_gefunden {
        println!("❌ FEHLER: Spalten wurden nicht gefunden!");
        process::exit(1);
    }

    // Berechne Header-Längen mit Unicode-Unterstützung
    let header_lengths: Vec<usize> = headers.iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();

    let (data, _max_lengths) = fetch_data_with_stats(conn, &query, headers.len(), &header_lengths)?;

    // SORTIERUNG DER SPALTEN: NUR wenn spaltenreihenfolgeundnurdiese befüllt ist
    let (mut final_headers, mut final_data) =
    if !bereich.spaltenreihenfolgeundnurdiese.is_empty() {
        let null_basierte_indizes: Vec<usize> = bereich
            .spaltenreihenfolgeundnurdiese
            .iter()
            .map(|&i| if i == 0 { 0 } else { i - 1 })
            .collect();

        let sorted_headers = sort_by_indices(&headers, &null_basierte_indizes)
            .unwrap_or_else(|_| headers.clone());

        let sorted_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| {
                sort_by_indices(row, &null_basierte_indizes)
                    .unwrap_or_else(|_| row.clone())
            })
            .collect();

        (sorted_headers, sorted_data)
    } else {
        (headers.clone(), data.clone())
    };

    apply_generated_columns(
        &mut final_headers,
        &mut final_data,
        &bereich,
        generated_befehle,
        parameters_main,
    )?;

    // Rückwärtskompatibilität: alter Primzahlkreuz-Pfad hing nur die letzten 2 Spalten an.
    if generated_alias_present(generated_befehle, &["primzahlkreuzprocontra"]) {
        if final_headers.len() >= 2 {
            let keep_from = final_headers.len() - 2;
            final_headers = final_headers[keep_from..].to_vec();

            final_data = final_data
                .into_iter()
                .map(|row| {
                    if row.len() >= 2 {
                        row[row.len() - 2..].to_vec()
                    } else {
                        row
                    }
                })
                .collect();
        }
    }

    let original_line_numbers = build_original_line_numbers(&bereich, final_data.len());

    print_table_chunked_with_line_numbers(
        &final_headers,
        &final_data,
        &bereich.breiten,
        &original_line_numbers,
        bereich.keineleereninhalte,
    );
    Ok(bereich)
}

fn sort_by_indices<T: Clone>(values: &Vec<T>, indices: &[usize]) -> Result<Vec<T>, String> {
    // Wenn der Index-Vektor leer ist, gibt einen leeren Vektor zurück
    if indices.is_empty() {
        return Ok(Vec::new());
    }

    // Finde den maximalen Index
    let max_index = indices.iter().max().copied().unwrap_or(0);

    // Überprüfe, ob alle Indizes gültig sind
    if max_index >= values.len() {
        return Err(format!(
            "Index {} ist außerhalb der Grenzen (0..{})",
            max_index,
            values.len() - 1
        ));
    }

    // Erstelle den sortierten Vektor basierend auf den Indizes
    let result = indices
        .iter()
        .map(|&i| {
            if i >= values.len() {
                panic!("Unerwarteter Fehler: Index {} außerhalb der Grenzen", i);
            }
            values[i].clone()
        })
        .collect();

    Ok(result)
}
