use terminal_size::{terminal_size, Width as TermWidth};
use unicode_width::UnicodeWidthStr;

use crate::reta_ausgabe::{TableCell, TableRow};
use crate::table_printer::config::{
    ColumnKind, COLUMN_OVERHEAD, MAX_COLUMNS_CAP, MAX_COLUMN_WIDTH, MIN_COLUMN_WIDTH,
};

pub type RowRange = (usize, usize);

const DEFAULT_COMPACT_WIDTH: usize = 12;
const SOFT_TEXT_TARGET_WIDTH: usize = 21;
const SCREEN_SAFETY_MARGIN: usize = 1;

#[derive(Debug, Clone)]
pub struct ColumnStats {
    pub header_width: usize,
    pub max_width: usize,
    pub avg_width: usize,
    pub short_ratio: f64,
    pub long_ratio: f64,
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

pub fn compute_column_stats(headers: &[String], data: &[Vec<String>]) -> Vec<ColumnStats> {
    let mut stats = Vec::with_capacity(headers.len());

    for (col_idx, header) in headers.iter().enumerate() {
        let header_width = UnicodeWidthStr::width(header.as_str());

        let mut max_width = header_width;
        let mut total_width = 0usize;
        let mut count = 0usize;

        let mut short = 0usize;
        let mut long = 0usize;

        for row in data {
            if let Some(cell) = row.get(col_idx) {
                let w = UnicodeWidthStr::width(cell.as_str());

                max_width = max_width.max(w);
                total_width += w;
                count += 1;

                if w <= 3 {
                    short += 1;
                } else if w > 10 {
                    long += 1;
                }
            }
        }

        let avg_width = if count > 0 {
            total_width.div_ceil(count)
        } else {
            header_width
        };

        let short_ratio = short as f64 / count.max(1) as f64;
        let long_ratio = long as f64 / count.max(1) as f64;

        stats.push(ColumnStats {
            header_width,
            max_width,
            avg_width,
            short_ratio,
            long_ratio,
        });
    }

    stats
}

fn available_content_width(column_count: usize, term_width: usize) -> usize {
    let available_total = term_width.saturating_sub(SCREEN_SAFETY_MARGIN);
    let total_overhead = column_count * COLUMN_OVERHEAD;
    available_total.saturating_sub(total_overhead)
}

fn natural_width_for_column(header: &str, s: &ColumnStats) -> usize {
    let min_width = 3;

    // EXTREM wichtig:
    // Verteilung entscheidet, nicht nur Durchschnitt

    if s.short_ratio > 0.8 {
        // fast alles kurz → sehr schmal
        return min_width;
    }

    if s.short_ratio > 0.5 {
        return min_width + 1;
    }

    if s.long_ratio > 0.7 {
        // viel Text → breit
        return s.avg_width.max(12);
    }

    if s.long_ratio > 0.3 {
        return s.avg_width.max(8);
    }

    // Standard
    s.avg_width.max(min_width)
}

fn distribute_extra_space(
    widths: &mut [usize],
    stats: &[ColumnStats],
    headers: &[String],
    mut extra: usize,
) {
    if extra == 0 || widths.is_empty() {
        return;
    }

    // Harte Obergrenzen pro Spalte:
    // fast leere Spalten dürfen praktisch nicht wachsen
    let growth_caps: Vec<usize> = headers
        .iter()
        .zip(stats.iter())
        .map(|(header, s)| {
            let min_width = ColumnKind::infer_from_header(header)
                .min_width()
                .max(MIN_COLUMN_WIDTH);

            let avg = s.avg_width;
            let max = s.max_width;
            let header_w = s.header_width;

            if max <= 1 {
                min_width
            } else if max <= 2 {
                min_width
            } else if avg <= 2 {
                (min_width + 1).min(MAX_COLUMN_WIDTH)
            } else if avg <= 4 {
                (min_width + 2).min(MAX_COLUMN_WIDTH)
            } else if avg <= 6 {
                header_w.max(min_width + 3).min(MAX_COLUMN_WIDTH)
            } else if avg <= 10 {
                header_w.max(avg + 1).min(max).min(MAX_COLUMN_WIDTH)
            } else if avg <= 16 {
                header_w.max(avg + 3).min(max).min(MAX_COLUMN_WIDTH)
            } else {
                header_w.max(avg + 6).min(max).min(MAX_COLUMN_WIDTH)
            }
        })
        .collect();

    // Gewichte:
    // textarme Spalten bekommen fast nichts
    // textreiche fast alles
    let weights: Vec<usize> = stats
        .iter()
        .map(|s| {
            let avg = s.avg_width;
            let max = s.max_width;

            if max <= 1 {
                0
            } else if avg <= 2 {
                0
            } else if avg <= 4 {
                1
            } else if avg <= 6 {
                2
            } else if avg <= 10 {
                4
            } else if avg <= 16 {
                8
            } else {
                16
            }
        })
        .collect();

    while extra > 0 {
        let mut changed = false;

        for i in 0..widths.len() {
            if weights[i] == 0 {
                continue;
            }

            for _ in 0..weights[i] {
                if widths[i] < growth_caps[i] && extra > 0 {
                    widths[i] += 1;
                    extra -= 1;
                    changed = true;
                } else {
                    break;
                }
            }

            if extra == 0 {
                break;
            }
        }

        if !changed {
            break;
        }
    }
}

fn shrink_widths_to_fit(widths: &mut [usize], headers: &[String], mut overflow: usize) {
    if overflow == 0 || widths.is_empty() {
        return;
    }

    while overflow > 0 {
        let mut changed = false;

        for (i, w) in widths.iter_mut().enumerate() {
            let min_width = ColumnKind::infer_from_header(&headers[i])
                .min_width()
                .max(MIN_COLUMN_WIDTH);

            if *w > min_width {
                *w -= 1;
                overflow -= 1;
                changed = true;

                if overflow == 0 {
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
    if headers.is_empty() {
        return Vec::new();
    }

    let stats = compute_column_stats(headers, data);
    let available_content = available_content_width(headers.len(), term_width);

    if available_content == 0 {
        return vec![MIN_COLUMN_WIDTH; headers.len()];
    }

    let mut widths: Vec<usize> = headers
        .iter()
        .zip(stats.iter())
        .map(|(header, s)| natural_width_for_column(header, s))
        .collect();

    let current_sum: usize = widths.iter().sum();

    if current_sum < available_content {
        let extra = available_content - current_sum;
        distribute_extra_space(&mut widths, &stats, headers, extra);
    } else if current_sum > available_content {
        let overflow = current_sum - available_content;
        shrink_widths_to_fit(&mut widths, headers, overflow);
    }

    widths
}

pub fn compute_column_widths(
    headers: &[String],
    max_lengths: &[usize],
    term_width: usize,
) -> Vec<usize> {
    if headers.is_empty() {
        return Vec::new();
    }

    let available_content = available_content_width(headers.len(), term_width);

    if available_content == 0 {
        return vec![MIN_COLUMN_WIDTH; headers.len()];
    }

    let mut widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let header_width = UnicodeWidthStr::width(header.as_str());
            let max_data_width = max_lengths.get(i).copied().unwrap_or(header_width);

            let kind = ColumnKind::infer_from_header(header);
            let min_width = kind.min_width().max(MIN_COLUMN_WIDTH);

             const TEXT_AMOUNT_FACTOR: f64 = 3.5;

let width_above_min = max_data_width.saturating_sub(min_width) as f64;
let amplified = min_width + (width_above_min * TEXT_AMOUNT_FACTOR).round() as usize;

let natural = if max_data_width <= DEFAULT_COMPACT_WIDTH {
    max_data_width.max(min_width)
} else {
    header_width
        .max(amplified)
        .max(DEFAULT_COMPACT_WIDTH)
        .min(MAX_COLUMN_WIDTH)
};
            natural.clamp(min_width, MAX_COLUMN_WIDTH)
        })
        .collect();

    let current_sum: usize = widths.iter().sum();

    if current_sum < available_content {
        let mut extra = available_content - current_sum;

        while extra > 0 {
            let mut changed = false;

            for (i, w) in widths.iter_mut().enumerate() {
                let header_width = UnicodeWidthStr::width(headers[i].as_str());
                let max_data_width = max_lengths.get(i).copied().unwrap_or(header_width);

                let soft_cap = max_data_width
                    .max(header_width)
                    .min(SOFT_TEXT_TARGET_WIDTH)
                    .max(MIN_COLUMN_WIDTH);

                if *w < soft_cap {
                    *w += 1;
                    extra -= 1;
                    changed = true;

                    if extra == 0 {
                        break;
                    }
                }
            }

            if !changed {
                break;
            }
        }
    } else if current_sum > available_content {
        let overflow = current_sum - available_content;
        shrink_widths_to_fit(&mut widths, headers, overflow);
    }

    widths
}

pub fn compute_columns_per_table(
    term_width: usize,
    headers: &[String],
    max_lengths: &[usize],
) -> usize {
    if headers.is_empty() {
        return 1;
    }

    let available_total = term_width.saturating_sub(SCREEN_SAFETY_MARGIN);

    let mut used_width = 0usize;
    let mut cols = 0usize;

    for (header, &max_len) in headers.iter().zip(max_lengths.iter()) {
        let kind = ColumnKind::infer_from_header(header);
        let min_width = kind.min_width().max(MIN_COLUMN_WIDTH);

        let desired_width = if max_len <= DEFAULT_COMPACT_WIDTH {
            max_len.max(min_width)
        } else if max_len <= SOFT_TEXT_TARGET_WIDTH {
            header.len()
                .max((header.len() + max_len).div_ceil(2))
                .max(DEFAULT_COMPACT_WIDTH)
                .max(min_width)
        } else {
            DEFAULT_COMPACT_WIDTH.max(min_width)
        }
        .min(MAX_COLUMN_WIDTH);

        let total_col_width = desired_width + COLUMN_OVERHEAD;

        if cols >= 2 && used_width + total_col_width > available_total {
            break;
        }

        used_width += total_col_width;
        cols += 1;

        if cols >= MAX_COLUMNS_CAP {
            break;
        }
    }

    if headers.len() < 2 {
        headers.len()
    } else {
        cols.clamp(2, MAX_COLUMNS_CAP)
    }
}

pub fn compute_columns_per_table_optimized(
    term_width: usize,
    headers: &[String],
    data: &[Vec<String>],
) -> usize {
    if headers.is_empty() {
        return 1;
    }

    let min_columns = if headers.len() >= 2 { 2 } else { 1 };
    let max_try = headers.len();
    let available_total = term_width.saturating_sub(SCREEN_SAFETY_MARGIN);

    for cols in (min_columns..=max_try).rev() {
        let trial_headers = &headers[..cols];

        let trial_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| row.iter().take(cols).cloned().collect())
            .collect();

        let widths = compute_column_widths_optimized(trial_headers, &trial_data, term_width);
        let total = widths.iter().sum::<usize>() + cols * COLUMN_OVERHEAD;

        if total <= available_total {
            return cols.max(min_columns).min(headers.len());
        }
    }

    min_columns.min(headers.len()).max(1)
}

pub fn build_table_layout(headers: &[String], data: &[Vec<String>]) -> TableLayout {
    let term_width = get_terminal_width();
    let max_lengths = compute_max_lengths(headers, data);
    let column_widths = compute_column_widths_optimized(headers, data, term_width);

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
    let mut rows = Vec::with_capacity(data.len() + 1);
    rows.push(build_header_row(headers, column_widths));

    let row_numbers = row_numbers_for_data_len(data.len(), row_ranges);

    for (idx, row_data) in data.iter().enumerate() {
        let line_num = row_numbers.get(idx).copied().unwrap_or(idx + 1);

        rows.push(build_data_row(
            row_data,
            headers.len(),
            column_widths,
            line_num,
        ));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_max_lengths() {
        let headers = vec!["Name".to_string(), "Alter".to_string()];
        let data = vec![
            vec!["Hans".to_string(), "25".to_string()],
            vec!["Anna".to_string(), "30".to_string()],
        ];

        let max_lengths = compute_max_lengths(&headers, &data);
        assert_eq!(max_lengths[0], 4);
        assert_eq!(max_lengths[1], 5);
    }

    #[test]
    fn test_row_numbers_without_ranges() {
        let nums = row_numbers_for_data_len(3, &[]);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_row_numbers_with_ranges() {
        let nums = row_numbers_for_data_len(3, &[(5, 6), (10, 10)]);
        assert_eq!(nums, vec![5, 6, 10]);
    }

    #[test]
    fn test_normalize_row() {
        let row = vec!["a".to_string()];
        let out = normalize_row(&row, 3);
        assert_eq!(out, vec!["a".to_string(), "".to_string(), "".to_string()]);
    }

    #[test]
    fn test_chunk_bounds() {
        let bounds = chunk_bounds(7, 3);
        assert_eq!(bounds, vec![(0, 3), (3, 6), (6, 7)]);
    }

    #[test]
    fn test_compute_column_widths_optimized_nonempty() {
        let headers = vec!["ID".to_string(), "Beschreibung".to_string()];
        let data = vec![
            vec!["1".to_string(), "kurz".to_string()],
            vec!["2".to_string(), "etwas längerer Inhalt".to_string()],
        ];

        let widths = compute_column_widths_optimized(&headers, &data, 80);
        assert_eq!(widths.len(), 2);
        assert!(widths[1] >= widths[0]);
    }

    #[test]
    fn test_compute_columns_per_table_optimized_at_least_two() {
        let headers = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
        ];
        let data = vec![vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ]];

        let cols = compute_columns_per_table_optimized(80, &headers, &data);
        assert!(cols >= 2);
    }
}
