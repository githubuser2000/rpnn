use terminal_size::{terminal_size, Width as TermWidth};
use unicode_width::UnicodeWidthStr;

use crate::reta_ausgabe::{TableCell, TableRow};
use crate::table_printer::config::{
    ColumnKind, COLUMN_OVERHEAD, MAX_COLUMNS_CAP, MAX_COLUMN_WIDTH, MIN_COLUMN_WIDTH,
};

pub type RowRange = (usize, usize);

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

pub fn compute_columns_per_table(
    term_width: usize,
    headers: &[String],
    max_lengths: &[usize],
) -> usize {
    if headers.is_empty() {
        return 1;
    }

    let mut used_width = 0;
    let mut cols = 0;

    for (header, &max_len) in headers.iter().zip(max_lengths.iter()) {
        let kind = ColumnKind::infer_from_header(header);
        let min_width = kind.min_width().max(MIN_COLUMN_WIDTH);
        let col_width = max_len.max(min_width).min(MAX_COLUMN_WIDTH) + COLUMN_OVERHEAD;

        if cols >= 2 && used_width + col_width > term_width {
            break;
        }

        used_width += col_width;
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

pub fn compute_column_widths(
    headers: &[String],
    max_lengths: &[usize],
    term_width: usize,
) -> Vec<usize> {
    if headers.is_empty() {
        return Vec::new();
    }

    let mut widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let header_width = UnicodeWidthStr::width(header.as_str());
            let data_width = max_lengths.get(i).copied().unwrap_or(0);
            let kind = ColumnKind::infer_from_header(header);
            let min_width = kind.min_width().max(MIN_COLUMN_WIDTH);

            header_width
                .max(data_width)
                .max(min_width)
                .min(MAX_COLUMN_WIDTH)
        })
        .collect();

    shrink_widths_to_fit(term_width, &mut widths);
    widths
}

fn shrink_widths_to_fit(term_width: usize, widths: &mut [usize]) {
    if widths.is_empty() {
        return;
    }

    let available_for_content = term_width.saturating_sub(widths.len() * COLUMN_OVERHEAD);

    let current_total: usize = widths.iter().sum();
    if current_total <= available_for_content {
        return;
    }

    let min_widths = vec![MIN_COLUMN_WIDTH; widths.len()];
    let mut overflow = current_total - available_for_content;

    while overflow > 0 {
        let mut changed = false;

        for (w, min_w) in widths.iter_mut().zip(min_widths.iter()) {
            if *w > *min_w {
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

pub fn build_table_layout(headers: &[String], data: &[Vec<String>]) -> TableLayout {
    let term_width = get_terminal_width();
    let max_lengths = compute_max_lengths(headers, data);
    let column_widths = compute_column_widths(headers, &max_lengths, term_width);

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
        let line_num = row_numbers
            .get(idx)
            .copied()
            .unwrap_or_else(|| idx + 1);

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
        assert_eq!(max_lengths[1], 5); // "Alter" ist breiter als "25"/"30"
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
}
