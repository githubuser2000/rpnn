use csv::ReaderBuilder;
use rusqlite::Connection;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::path::{Path, PathBuf};

use crate::cli::TextBereich;

#[derive(Debug)]
pub struct PypyCompatDbs {
    pub kombi: Connection,
    pub kombi_meta: Connection,
    pub gebr_galaxie: Connection,
    pub gebr_universum: Connection,
    pub gebr_strukturgroesse: Connection,
    pub gebr_emotionen: Connection,
}

fn read_csv_matrix(path: &Path) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .quoting(true)
        .trim(csv::Trim::None)
        .has_headers(false)
        .from_path(path)?;

    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result?;
        rows.push(record.iter().map(|s| s.to_string()).collect());
    }
    Ok(rows)
}

fn create_single_csv_db(path: &Path) -> Result<Connection, Box<dyn Error>> {
    let rows = read_csv_matrix(path)?;
    let max_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut conn = Connection::open_in_memory()?;
    let tx = conn.transaction()?;
    let cols = (0..max_cols)
        .map(|i| format!("\"col_{}\" TEXT", i + 1))
        .collect::<Vec<_>>()
        .join(", ");
    tx.execute(&format!("CREATE TABLE csv_data ({})", cols), [])?;
    if max_cols > 0 {
        let placeholders = vec!["?"; max_cols].join(", ");
        let mut stmt = tx.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;
        for row in rows {
            let mut padded = row;
            padded.resize(max_cols, String::new());
            stmt.execute(rusqlite::params_from_iter(padded.iter()))?;
        }
    }
    tx.commit()?;
    Ok(conn)
}

fn load_table(conn: &Connection) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>> {
    let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
    let headers: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<Result<Vec<_>, _>>()?;

    let select = format!(
        "SELECT {} FROM csv_data",
        headers
            .iter()
            .map(|h| format!("\"{}\"", h.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let mut stmt = conn.prepare(&select)?;
    let mut rows = stmt.query([])?;
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let mut values = Vec::with_capacity(headers.len());
        for idx in 0..headers.len() {
            values.push(row.get::<_, String>(idx).unwrap_or_default());
        }
        data.push(values);
    }
    Ok((headers, data))
}

fn rebuild_table(conn: &Connection, headers: &[String], rows: &[Vec<String>]) -> Result<(), Box<dyn Error>> {
    conn.execute("DROP TABLE IF EXISTS csv_data", [])?;
    let create_columns = headers
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let safe_name = name.replace('"', "\"\"");
            format!("\"{}\" TEXT", safe_name)
        })
        .collect::<Vec<_>>()
        .join(", ");
    conn.execute(&format!("CREATE TABLE csv_data ({})", create_columns), [])?;
    if headers.is_empty() {
        return Ok(());
    }
    let placeholders = vec!["?"; headers.len()].join(", ");
    let mut stmt = conn.prepare(&format!("INSERT INTO csv_data VALUES ({})", placeholders))?;
    for row in rows {
        let mut padded = row.clone();
        padded.resize(headers.len(), String::new());
        stmt.execute(rusqlite::params_from_iter(padded.iter()))?;
    }
    Ok(())
}

fn csv_path(base: &Path, name: &str) -> PathBuf {
    base.join("csv").join(name)
}

fn parse_kombi_numbers(raw: &str, out: &mut BTreeSet<usize>) {
    let value = raw.trim();
    if value.is_empty() {
        return;
    }
    if value.starts_with('(') && value.ends_with(')') && value.len() > 2 {
        parse_kombi_numbers(&value[1..value.len() - 1], out);
        return;
    }
    if let Ok(v) = value.parse::<isize>() {
        let n = v.unsigned_abs();
        if n > 0 {
            out.insert(n);
        }
        return;
    }
    if let Some((a, b)) = value.split_once('/') {
        parse_kombi_numbers(a, out);
        parse_kombi_numbers(b, out);
    }
}

fn main_lookup(main_rows: &[Vec<String>], row_number: usize, col_index: usize) -> String {
    if row_number == 0 {
        return String::new();
    }
    main_rows
        .get(row_number - 1)
        .and_then(|r| r.get(col_index))
        .cloned()
        .unwrap_or_default()
}

fn format_fraction_cell(
    numer: usize,
    denom: usize,
    frac_table: &[Vec<String>],
    main_rows: &[Vec<String>],
    pair: (usize, usize),
    is_universe: bool,
) -> String {
    if numer == 0 || denom == 0 {
        return String::new();
    }
    if numer > 100 || denom > 100 {
        return String::new();
    }
    if numer == 1 {
        let base = main_lookup(main_rows, denom, pair.1);
        if base.trim().len() <= 3 {
            return String::new();
        }
        if is_universe {
            let extra = main_lookup(main_rows, denom, 201);
            if extra.trim().len() > 2 {
                return format!("{} (1/{}) ; {}", base, denom, extra);
            }
        }
        return base;
    }
    if denom == 1 {
        let base = main_lookup(main_rows, numer, pair.0);
        if base.trim().len() <= 3 {
            return String::new();
        }
        if is_universe {
            let extra = main_lookup(main_rows, numer, 198);
            if extra.trim().len() > 2 {
                return format!("{} ({}) ; {}", base, numer, extra);
            }
        }
        return base;
    }
    frac_table
        .get(numer - 1)
        .and_then(|r| r.get(denom - 1))
        .cloned()
        .unwrap_or_default()
}

fn append_fraction_columns(
    headers: &mut Vec<String>,
    rows: &mut [Vec<String>],
    added_headers: &mut BTreeMap<String, Vec<usize>>,
    frac_table: &[Vec<String>],
    selected: &BTreeSet<usize>,
    base_label: &str,
    pair: (usize, usize),
    is_universe: bool,
) {
    if selected.is_empty() {
        return;
    }

    let snapshot = rows.to_vec();
    for &sel in selected {
        let denom = sel;
        if denom <= 1 {
            continue;
        }

        let header_normal = format!("n/{} {}", denom, base_label);
        headers.push(header_normal.clone());
        let idx_normal = headers.len();
        for (row_idx, row) in rows.iter_mut().enumerate() {
            let main_row = row_idx + 1;
            let value = format_fraction_cell(main_row, denom, frac_table, &snapshot, pair, is_universe);
            row.push(value);
        }
        added_headers.entry(base_label.to_string()).or_default().push(idx_normal);

        let header_inverse = format!("{}/n {}", denom, base_label);
        headers.push(header_inverse.clone());
        let idx_inverse = headers.len();
        for (row_idx, row) in rows.iter_mut().enumerate() {
            let main_row = row_idx + 1;
            let value = format_fraction_cell(denom, main_row, frac_table, &snapshot, pair, is_universe);
            row.push(value);
        }
        added_headers.entry(base_label.to_string()).or_default().push(idx_inverse);
    }
}

fn append_kombi_columns(
    headers: &mut Vec<String>,
    rows: &mut [Vec<String>],
    added_headers: &mut BTreeMap<String, Vec<usize>>,
    kombi_table: &[Vec<String>],
    selected: &BTreeSet<usize>,
    prefix: &str,
) {
    if selected.is_empty() || kombi_table.is_empty() {
        return;
    }

    let header_row = &kombi_table[0];
    for &sel in selected {
        if sel == 0 {
            continue;
        }
        let csv_col = sel; // first usable payload column is 1 in Python rowsOfcombi
        let source_header = header_row.get(csv_col).cloned().unwrap_or_else(|| format!("{} {}", prefix, sel));
        let header = format!("{} {}", prefix, source_header);
        headers.push(header);
        let idx = headers.len();
        for (row_idx, row) in rows.iter_mut().enumerate() {
            let religion_number = row_idx + 1;
            let mut parts = Vec::new();
            for combo_row in kombi_table.iter().skip(1) {
                if combo_row.is_empty() { continue; }
                let mut nums = BTreeSet::new();
                for part in combo_row[0].split('|') {
                    parse_kombi_numbers(part, &mut nums);
                }
                if !nums.contains(&religion_number) { continue; }
                let cell = combo_row.get(csv_col).cloned().unwrap_or_default();
                if cell.trim().is_empty() { continue; }
                let wrapped = format!("({}) {} ({})", combo_row[0], cell, combo_row[0]);
                if !parts.contains(&wrapped) {
                    parts.push(wrapped);
                }
            }
            row.push(parts.join(" | "));
        }
        added_headers.entry(prefix.to_string()).or_default().push(idx);
    }
}

pub fn build_extra_csv_dbs(base: &Path) -> Result<PypyCompatDbs, Box<dyn Error>> {
    Ok(PypyCompatDbs {
        kombi: create_single_csv_db(&csv_path(base, "kombi.csv"))?,
        kombi_meta: create_single_csv_db(&csv_path(base, "kombi-meta.csv"))?,
        gebr_galaxie: create_single_csv_db(&csv_path(base, "gebrochen-rational-galaxie.csv"))?,
        gebr_universum: create_single_csv_db(&csv_path(base, "gebrochen-rational-universum.csv"))?,
        gebr_strukturgroesse: create_single_csv_db(&csv_path(base, "gebrochen-rational-strukturgroesse.csv"))?,
        gebr_emotionen: create_single_csv_db(&csv_path(base, "gebrochen-rational-emotionen.csv"))?,
    })
}

fn collect_existing_selected_columns(bereich: &TextBereich) -> Vec<usize> {
    let mut cols = Vec::new();

    if !bereich.spalten_bereiche.is_empty() {
        for &(from, to) in &bereich.spalten_bereiche {
            if from == 0 || to == 0 || from > to {
                continue;
            }
            for col in from..=to {
                cols.push(col);
            }
        }
    } else if bereich.von_spalte > 0
        && bereich.bis_spalte > 0
        && bereich.von_spalte != usize::MAX
        && bereich.bis_spalte != usize::MAX
        && bereich.von_spalte <= bereich.bis_spalte
    {
        for col in bereich.von_spalte..=bereich.bis_spalte {
            cols.push(col);
        }
    }

    cols.sort_unstable();
    cols.dedup();
    cols
}

pub fn apply_pypy_compat(
    conn: &Connection,
    bereich: &mut TextBereich,
    base: &Path,
) -> Result<(), Box<dyn Error>> {
    let wants_any = !(bereich.pypy_compat.gebrochengalaxie.is_empty()
        && bereich.pypy_compat.gebrochenuniversum.is_empty()
        && bereich.pypy_compat.gebrochenemotion.is_empty()
        && bereich.pypy_compat.gebrochengroesse.is_empty()
        && bereich.pypy_compat.kombi_galaxie.is_empty()
        && bereich.pypy_compat.kombi_universum.is_empty());
    if !wants_any {
        return Ok(());
    }

    let existing_selected = collect_existing_selected_columns(bereich);
    let had_explicit_order = !bereich.spaltenreihenfolgeundnurdiese.is_empty();
    let explicit_order_before = bereich.spaltenreihenfolgeundnurdiese.clone();

    let _dbs = build_extra_csv_dbs(base)?;
    let (mut headers, mut rows) = load_table(conn)?;

    let gal = read_csv_matrix(&csv_path(base, "gebrochen-rational-galaxie.csv"))?;
    let uni = read_csv_matrix(&csv_path(base, "gebrochen-rational-universum.csv"))?;
    let emo = read_csv_matrix(&csv_path(base, "gebrochen-rational-emotionen.csv"))?;
    let groe = read_csv_matrix(&csv_path(base, "gebrochen-rational-strukturgroesse.csv"))?;
    let kombi = read_csv_matrix(&csv_path(base, "kombi.csv"))?;
    let kombi_meta = read_csv_matrix(&csv_path(base, "kombi-meta.csv"))?;

    append_fraction_columns(
        &mut headers,
        &mut rows,
        &mut bereich.pypy_compat.added_headers,
        &gal,
        &bereich.pypy_compat.gebrochengalaxie,
        "Galaxie",
        (10, 42),
        false,
    );
    append_fraction_columns(
        &mut headers,
        &mut rows,
        &mut bereich.pypy_compat.added_headers,
        &uni,
        &bereich.pypy_compat.gebrochenuniversum,
        "Universum",
        (5, 131),
        true,
    );
    append_fraction_columns(
        &mut headers,
        &mut rows,
        &mut bereich.pypy_compat.added_headers,
        &emo,
        &bereich.pypy_compat.gebrochenemotion,
        "Emotion",
        (243, 284),
        false,
    );
    append_fraction_columns(
        &mut headers,
        &mut rows,
        &mut bereich.pypy_compat.added_headers,
        &groe,
        &bereich.pypy_compat.gebrochengroesse,
        "Strukturgroesse",
        (4, 197),
        false,
    );
    append_kombi_columns(
        &mut headers,
        &mut rows,
        &mut bereich.pypy_compat.added_headers,
        &kombi,
        &bereich.pypy_compat.kombi_galaxie,
        "Kombi-Galaxie",
    );
    append_kombi_columns(
        &mut headers,
        &mut rows,
        &mut bereich.pypy_compat.added_headers,
        &kombi_meta,
        &bereich.pypy_compat.kombi_universum,
        "Kombi-Universum",
    );

    rebuild_table(conn, &headers, &rows)?;

    let mut appended = Vec::new();
    appended.extend(bereich.pypy_compat.added_headers.values().flat_map(|v| v.iter().copied()));
    appended.sort_unstable();
    appended.dedup();
    if !appended.is_empty() {
        let mut combined = existing_selected;
        if !bereich.pypy_compat.hidden_fraction_inputs {
            combined.extend(appended.iter().copied());
            combined.sort_unstable();
            combined.dedup();
        }

        if !combined.is_empty() {
            bereich.spalten_bereiche = combined.iter().map(|&col| (col, col)).collect();
        }

        if !bereich.pypy_compat.hidden_fraction_inputs {
            let mut visible = bereich.exact_visible_columns.clone();
            visible.extend(appended.iter().copied());
            visible.sort_unstable();
            visible.dedup();
            bereich.exact_visible_columns = visible;
        }

        if had_explicit_order {
            bereich.spaltenreihenfolgeundnurdiese = explicit_order_before;
        } else if !combined.is_empty() {
            bereich.spaltenreihenfolgeundnurdiese = combined;
        }

        if let Some(&(first_from, _)) = bereich.spalten_bereiche.first() {
            bereich.von_spalte = first_from;
        }
        if let Some(&(_, last_to)) = bereich.spalten_bereiche.last() {
            bereich.bis_spalte = last_to;
        }
        bereich.spalten_gefunden = true;
        bereich.spalten_gesucht = true;
        bereich.spalten_gesucht2 = false;
    }

    Ok(())
}
