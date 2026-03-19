use terminal_size::{terminal_size, Width as TermWidth};
use unicode_width::UnicodeWidthStr;

use crate::reta_ausgabe::{TableCell, TableRow};
use crate::table_printer::config::{
    COLUMN_OVERHEAD, MAX_COLUMNS_CAP, MAX_COLUMN_WIDTH, MIN_COLUMN_WIDTH,
};

pub type RowRange = (usize, usize);

#[derive(Debug, Clone)]
pub struct ColumnStats {
    pub header_width: usize,
    pub max_width: usize,
    pub total_width: usize,
    pub avg_width: f64,
    pub non_empty_ratio: f64,
    pub text_mass: f64,
}

#[derive(Debug, Clone)]
pub struct TableLayout {
    pub term_width: usize,
    pub max_lengths: Vec<usize>,
    pub column_widths: Vec<usize>,
}

pub fn get_terminal_width() -> usize {
    terminal_size()
        .map(|(TermWidth(w), _)| w as usize)
        .unwrap_or(80)
}

pub fn compute_max_lengths(headers: &[String], data: &[Vec<String>]) -> Vec<usize> {
    let mut max_lengths: Vec<usize> = headers
        .iter()
        .map(|h| UnicodeWidthStr::width(h.as_str()))
        .collect();

    for row in data {
        for (i, cell) in row.iter().enumerate() {
            if let Some(current) = max_lengths.get_mut(i) {
                *current = (*current).max(UnicodeWidthStr::width(cell.as_str()));
            }
        }
    }

    max_lengths
}

/// ------------------------------------------------------------
/// Funktion: compute_column_stats
///
/// Beschreibung:
/// Berechnet statistische Kennzahlen für jede Spalte einer Tabelle.
/// Diese Statistiken werden später verwendet, um Spaltenbreiten
/// intelligent (inhaltlich gewichtet) zu bestimmen.
///
/// Parameter:
/// - headers: &[String]
///     Liste der Spaltenüberschriften
///
/// - data: &[Vec<String>]
///     Tabelleninhalt, jede innere Vec ist eine Zeile
///
/// Rückgabe:
/// - Vec<ColumnStats>
///     Für jede Spalte ein Statistikobjekt mit:
///       - header_width: Breite des Headers
///       - max_width: maximale Zellbreite
///       - total_width: Summe aller Zellbreiten
///       - avg_width: Durchschnittliche Breite
///       - non_empty_ratio: Anteil nicht-leerer Zellen
///       - text_mass: gewichtete Gesamttextmenge
///
/// Zweck:
/// Grundlage für dynamische Spaltenbreiten (wichtiger Kern deiner Engine)
pub fn compute_column_stats(headers: &[String], data: &[Vec<String>]) -> Vec<ColumnStats> {
    let mut stats = Vec::with_capacity(headers.len());

    for (col_idx, header) in headers.iter().enumerate() {
        let header_width = UnicodeWidthStr::width(header.as_str());

        let mut max_width = header_width;
        let mut total_width = 0usize;
        let mut row_count = 0usize;
        let mut non_empty_count = 0usize;

        for row in data {
            let w = row
                .get(col_idx)
                .map(|cell| UnicodeWidthStr::width(cell.as_str()))
                .unwrap_or(0);

            max_width = max_width.max(w);
            total_width += w;
            row_count += 1;

            if w > 0 {
                non_empty_count += 1;
            }
        }

        let avg_width = if row_count > 0 {
            total_width as f64 / row_count as f64
        } else {
            header_width as f64
        };

        let non_empty_ratio = if row_count > 0 {
            non_empty_count as f64 / row_count as f64
        } else {
            0.0
        };

        // echte lineare Textmasse: SUMME, nicht Durchschnitt
        let text_mass = total_width as f64 + header_width as f64 * 0.5;

        stats.push(ColumnStats {
            header_width,
            max_width,
            total_width,
            avg_width,
            non_empty_ratio,
            text_mass,
        });
    }

    stats
}

/// Wandelt absolute Textmassen in lineare Bildschirmbreiten um.
/// NICHT direkt text_mass als Breite nehmen, sonst explodieren große Tabellen.
/// Stattdessen: proportionale Verteilung innerhalb des verfügbaren Budgets.
// In table_utils.rs

// In table_utils.rs -> compute_column_widths_from_global_mass

/// Wandelt absolute Textmassen in lineare Bildschirmbreiten um.
/// NICHT direkt text_mass als Breite nehmen, sonst explodieren große Tabellen.
/// Stattdessen: proportionale Verteilung innerhalb des verfügbaren Budgets.
pub fn compute_column_widths_from_global_mass(
    headers: &[String],
    data: &[Vec<String>],
    available_budget: usize,
) -> Vec<usize> {
    if headers.is_empty() { return Vec::new(); }

    let stats = compute_column_stats(headers, data);
    
    // 1. AGGRESSIVE GEWICHTUNG:
    // Wir quadrieren die Textmasse. 
    // Spalten mit wenig Inhalt fallen dadurch fast auf 0 zurück (werden durch MIN_COLUMN_WIDTH gefangen).
    // Spalten mit viel Inhalt ziehen das gesamte Budget an sich.
    let weighted_masses: Vec<f64> = stats.iter()
        .map(|s| {
            let m = s.text_mass.max(0.1);
            m * m // Quadratischer Effekt für extreme Unterschiede
        })
        .collect();

    let mass_sum: f64 = weighted_masses.iter().sum();

    let mut widths: Vec<usize> = weighted_masses
        .iter()
        .enumerate()
        .map(|(i, &mass)| {
            let ratio = if mass_sum > 0.0 { mass / mass_sum } else { 1.0 / headers.len() as f64 };
            
            // VERDOPPELTE BREITE: Das Budget wird nun extrem ungleich verteilt
            let proportional = (ratio * available_budget as f64).round() as usize;
            
            // Verdopple die berechnete Breite
            let doubled = proportional * 3;

            let header_w = stats[i].header_width;
            
            doubled
                .max(MIN_COLUMN_WIDTH) // Kleine Spalten bleiben klein
                .max(header_w.min(20)) // Header nicht zu extrem priorisieren
                .min(MAX_COLUMN_WIDTH) // Große Spalten dürfen jetzt bis zu 100 breit sein
        })
        .collect();

    shrink_widths_to_fit_budget(&mut widths, available_budget);
    widths
}

/// Fallback: natürliche lokale Breiten
pub fn compute_column_widths_linear_natural(
    headers: &[String],
    data: &[Vec<String>],
) -> Vec<usize> {
    if headers.is_empty() {
        return Vec::new();
    }

    let stats = compute_column_stats(headers, data);

    stats.iter()
        .map(|s| s.avg_width.ceil() as usize)
        .map(|w| w.max(MIN_COLUMN_WIDTH).min(MAX_COLUMN_WIDTH))
        .collect()
}

/// Kürzt Spaltenbreiten lokal so weit ein,
/// bis sie in das verfügbare Budget des aktuellen Chunks passen.
///
/// Was die Funktion macht:
/// - Summiert alle Breiten des Chunks
/// - Wenn die Summe zu groß ist, werden Spalten schrittweise verkleinert
/// - Keine Spalte wird unter `MIN_COLUMN_WIDTH` reduziert
///
/// Warum das nötig ist:
/// - Explizite Breiten oder aggressive Statistik können ein Chunk-Budget sprengen
/// - Dann muss lokal nachkorrigiert werden
///
/// Parameter:
/// - `widths`:
///   Spaltenbreiten des aktuellen Chunks
/// - `budget`:
///   Maximale Gesamtbreite für den aktuellen Chunk
///
/// Rückgabe:
/// - Keine; `widths` wird direkt verändert
pub fn shrink_widths_to_fit_budget(widths: &mut [usize], budget: usize) {
    let mut current: usize = widths.iter().sum();

    while current > budget {
        let mut changed = false;

        for w in widths.iter_mut() {
            if *w > MIN_COLUMN_WIDTH {
                *w -= 1;
                current -= 1;
                changed = true;

                if current <= budget {
                    break;
                }
            }
        }

        if !changed {
            break;
        }
    }
}

pub fn compute_column_widths_optimized(
    headers: &[String],
    data: &[Vec<String>],
    term_width: usize,
) -> Vec<usize> {
    let available_budget = term_width
        .saturating_sub(1)
        .saturating_sub(headers.len() * COLUMN_OVERHEAD);

    compute_column_widths_from_global_mass(headers, data, available_budget)
}

pub fn compute_column_widths(
    headers: &[String],
    max_lengths: &[usize],
    _term_width: usize,
) -> Vec<usize> {
    headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let hw = UnicodeWidthStr::width(header.as_str());
            let mw = max_lengths.get(i).copied().unwrap_or(hw);
            hw.max(mw).max(MIN_COLUMN_WIDTH).min(MAX_COLUMN_WIDTH)
        })
        .collect()
}

pub fn compute_columns_per_table_from_widths(
    term_width: usize,
    widths: &[usize],
) -> usize {
    if widths.is_empty() {
        return 1;
    }

    let available_total = term_width.saturating_sub(1);

    let mut used = 0usize;
    let mut cols = 0usize;

    for &w in widths {
        let total_col_width = w + COLUMN_OVERHEAD;

        if used + total_col_width > available_total {
            break;
        }

        used += total_col_width;
        cols += 1;

        if cols >= MAX_COLUMNS_CAP {
            break;
        }
    }

    cols.max(1)
}

pub fn compute_columns_per_table(
    term_width: usize,
    _headers: &[String],
    max_lengths: &[usize],
) -> usize {
    if max_lengths.is_empty() {
        return 1;
    }

    let widths: Vec<usize> = max_lengths
        .iter()
        .map(|&m| m.max(MIN_COLUMN_WIDTH).min(MAX_COLUMN_WIDTH))
        .collect();

    compute_columns_per_table_from_widths(term_width, &widths)
}

pub fn build_table_layout(headers: &[String], data: &[Vec<String>]) -> TableLayout {
    let term_width = get_terminal_width();
    let max_lengths = compute_max_lengths(headers, data);

    let available_budget = term_width
        .saturating_sub(1)
        .saturating_sub(headers.len() * COLUMN_OVERHEAD);

    let column_widths = compute_column_widths_from_global_mass(headers, data, available_budget);

    TableLayout {
        term_width,
        max_lengths,
        column_widths,
    }
}

pub fn normalize_row(row: &[String], expected_len: usize) -> Vec<String> {
    let mut out = row.iter().take(expected_len).cloned().collect::<Vec<_>>();
    out.resize(expected_len, String::new());
    out
}

pub fn row_numbers_for_data_len(data_len: usize, row_ranges: &[RowRange]) -> Vec<usize> {
    if row_ranges.is_empty() {
        return (1..=data_len).collect();
    }

    row_ranges
        .iter()
        .flat_map(|&(from, to)| from..=to)
        .take(data_len)
        .collect()
}

pub fn build_header_row(headers: &[String], column_widths: &[usize]) -> TableRow {
    let cells = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let width = column_widths.get(i).copied().unwrap_or(MAX_COLUMN_WIDTH);
            TableCell::new(header.clone(), width)
        })
        .collect();

    TableRow::new(cells, 0, 0)
}

pub fn build_data_row(
    row_data: &[String],
    headers_len: usize,
    column_widths: &[usize],
    line_num: usize,
) -> TableRow {
    let normalized = normalize_row(row_data, headers_len);

    let cells = normalized
        .into_iter()
        .enumerate()
        .map(|(i, content)| {
            let width = column_widths.get(i).copied().unwrap_or(MAX_COLUMN_WIDTH);
            TableCell::new(content, width)
        })
        .collect();

    TableRow::new(cells, line_num as i32, line_num as i32)
}

pub fn convert_to_table_rows(
    headers: &[String],
    data: &[Vec<String>],
    column_widths: &[usize],
    row_ranges: &[RowRange],
) -> Vec<TableRow> {
    let mut rows = Vec::new();
    rows.push(build_header_row(headers, column_widths));

    if row_ranges.is_empty() {
        for (idx, row_data) in data.iter().enumerate() {
            let line_num = idx + 1;
            rows.push(build_data_row(
                row_data,
                headers.len(),
                column_widths,
                line_num,
            ));
        }
        return rows;
    }

    for &(from, to) in row_ranges {
        if from == 0 || to == 0 || from > to {
            continue;
        }

        for line_num in from..=to {
            let data_index = line_num - 1;

            if let Some(row_data) = data.get(data_index) {
                rows.push(build_data_row(
                    row_data,
                    headers.len(),
                    column_widths,
                    line_num,
                ));
            }
        }
    }

    rows
}

pub fn chunk_bounds(headers_len: usize, chunk_size: usize) -> Vec<(usize, usize)> {
    if headers_len == 0 || chunk_size == 0 {
        return Vec::new();
    }

    let mut bounds = Vec::new();
    let mut start = 0;

    while start < headers_len {
        let end = (start + chunk_size).min(headers_len);
        bounds.push((start, end));
        start = end;
    }

    bounds
}
