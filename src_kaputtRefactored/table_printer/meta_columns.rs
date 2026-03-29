use crate::table_printer::config::COLUMN_OVERHEAD;

const POTENZ_HEADER: &str = "P";
const ZEILE_HEADER: &str = "Z";

fn is_special_power(n: usize) -> bool {
    if n < 4 || n == 8 {
        return false;
    }

    let mut base = 2usize;
    while base.saturating_mul(base) <= n {
        let mut value = base.saturating_mul(base);

        while value < n {
            match value.checked_mul(base) {
                Some(next) => value = next,
                None => break,
            }
        }

        if value == n {
            return true;
        }

        base += 1;
    }

    false
}

fn next_special_power(after: usize) -> usize {
    let mut candidate = after.saturating_add(1);

    loop {
        if is_special_power(candidate) {
            return candidate;
        }
        candidate = candidate.saturating_add(1);
    }
}

fn power_bucket_for_line(line_number: usize) -> usize {
    if line_number == 0 {
        return 0;
    }

    let mut bucket = 1usize;
    let mut boundary = 4usize;

    while line_number > boundary {
        bucket += 1;
        boundary = next_special_power(boundary);
    }

    bucket
}

pub fn build_power_bucket_strings(line_numbers: &[usize]) -> Vec<String> {
    line_numbers
        .iter()
        .map(|&n| power_bucket_for_line(n).to_string())
        .collect()
}

pub fn build_meta_widths(
    power_buckets: &[String],
    line_numbers: &[usize],
    available_total: usize,
) -> (Vec<usize>, usize) {
    let power_width = power_buckets
        .iter()
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(1)
        .max(POTENZ_HEADER.chars().count());

    let line_width = line_numbers
        .iter()
        .map(|n| n.to_string().chars().count())
        .max()
        .unwrap_or(1)
        .max(ZEILE_HEADER.chars().count());

    let widths = vec![power_width, line_width];
    let overhead = widths.len() * (COLUMN_OVERHEAD + 1);
    let content = widths.iter().sum::<usize>();
    let reserved_total = (content + overhead).min(available_total);

    (widths, reserved_total)
}

pub fn prepend_meta_columns(
    chunk_headers: &[String],
    chunk_data: &[Vec<String>],
    chunk_line_numbers: &[usize],
) -> (Vec<String>, Vec<Vec<String>>, Vec<usize>) {
    let effective_line_numbers: Vec<usize> = if chunk_line_numbers.len() == chunk_data.len() {
        chunk_line_numbers.to_vec()
    } else {
        chunk_data
            .iter()
            .enumerate()
            .map(|(idx, _)| chunk_line_numbers.get(idx).copied().unwrap_or(idx + 1))
            .collect()
    };

    let power_buckets = build_power_bucket_strings(&effective_line_numbers);

    let mut headers = vec![POTENZ_HEADER.to_string(), ZEILE_HEADER.to_string()];
    headers.extend(chunk_headers.iter().cloned());

    let mut data = Vec::with_capacity(chunk_data.len());
    for (idx, row) in chunk_data.iter().enumerate() {
        let mut new_row = Vec::with_capacity(row.len() + 2);
        new_row.push(power_buckets.get(idx).cloned().unwrap_or_else(|| "".to_string()));
        new_row.push(
            effective_line_numbers
                .get(idx)
                .copied()
                .unwrap_or(idx + 1)
                .to_string(),
        );
        new_row.extend(row.iter().cloned());
        data.push(new_row);
    }

    let (meta_widths, _) = build_meta_widths(&power_buckets, &effective_line_numbers, usize::MAX);

    (headers, data, meta_widths)
}
