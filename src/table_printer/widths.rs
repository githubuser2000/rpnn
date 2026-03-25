use crate::table_printer::config::{COLUMN_OVERHEAD, MAX_COLUMN_WIDTH, MIN_COLUMN_WIDTH};
use crate::table_printer::table_utils::{
    compute_column_stats,
    compute_column_widths_from_global_mass,
};

const CHUNK_MIN_COLUMN_WIDTH: usize = 21;

pub fn get_explicit_width(explizite_breiten: &[usize], index: usize) -> Option<usize> {
    match explizite_breiten.len() {
        0 => None,
        1 => Some(explizite_breiten[0]),
        _ => explizite_breiten.get(index).copied(),
    }
}

pub fn effective_min_column_width() -> usize {
    CHUNK_MIN_COLUMN_WIDTH.max(MIN_COLUMN_WIDTH)
}

pub fn clamp_chunk_width(width: usize) -> usize {
    width.clamp(effective_min_column_width(), MAX_COLUMN_WIDTH)
}

pub fn clamp_explicit_width(width: usize) -> usize {
    width.clamp(1, MAX_COLUMN_WIDTH)
}

pub fn estimate_natural_width_for_chunking(
    header: &String,
    data: &[Vec<String>],
    col_idx: usize,
) -> usize {
    let single_header = vec![header.clone()];

    let single_col_data: Vec<Vec<String>> = data
        .iter()
        .map(|row| vec![row.get(col_idx).cloned().unwrap_or_default()])
        .collect();

    let stats = compute_column_stats(&single_header, &single_col_data);
    let guessed = stats
        .first()
        .map(|s| s.avg_width.ceil() as usize)
        .unwrap_or(effective_min_column_width());

    clamp_chunk_width(guessed)
}

pub fn explicit_mask_for_range(
    explizite_breiten: &[usize],
    start: usize,
    end: usize,
) -> Vec<bool> {
    (start..end)
        .map(|global_i| get_explicit_width(explizite_breiten, global_i).is_some())
        .collect()
}

pub fn shrink_widths_to_budget_preserving_explicit(
    widths: &mut [usize],
    chunk_budget: usize,
    min_width: usize,
    explicit_mask: &[bool],
) {
    let mut current_total: usize = widths.iter().sum();

    while current_total > chunk_budget {
        let mut changed = false;

        for (i, w) in widths.iter_mut().enumerate() {
            let is_explicit = explicit_mask.get(i).copied().unwrap_or(false);
            if !is_explicit && *w > min_width && current_total > chunk_budget {
                *w -= 1;
                current_total -= 1;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }
}

pub fn stretch_last_non_explicit_or_last_column(
    widths: &mut [usize],
    chunk_budget: usize,
    explicit_mask: &[bool],
) {
    if widths.is_empty() {
        return;
    }

    let current_total: usize = widths.iter().sum();
    if current_total >= chunk_budget {
        return;
    }

    let extra = chunk_budget - current_total;

    if let Some(idx) = (0..widths.len())
        .rev()
        .find(|&i| !explicit_mask.get(i).copied().unwrap_or(false))
    {
        widths[idx] += extra;
    } else {
        let last_idx = widths.len() - 1;
        widths[last_idx] += extra;
    }
}

pub fn determine_chunk_end(
    headers: &[String],
    data: &[Vec<String>],
    explizite_breiten: &[usize],
    start: usize,
    available_total: usize,
) -> usize {
    let min_width = effective_min_column_width();
    let squeeze_threshold = available_total.saturating_mul(2) / 5;

    let mut end = start;
    let mut used = 0usize;

    while end < headers.len() {
        let guessed_width = if let Some(breite) = get_explicit_width(explizite_breiten, end) {
            clamp_explicit_width(breite)
        } else {
            estimate_natural_width_for_chunking(&headers[end], data, end)
        };

        let needed = guessed_width + COLUMN_OVERHEAD + 1;

        if used + needed > available_total {
            let remaining_total = available_total.saturating_sub(used);
            let remaining_content = remaining_total.saturating_sub(COLUMN_OVERHEAD + 1);

            if end > start && remaining_total >= squeeze_threshold && remaining_content >= min_width {
                end += 1;
            } else if end == start {
                end += 1;
            }

            break;
        }

        used += needed;
        end += 1;
    }

    if end <= start {
        (start + 1).min(headers.len())
    } else {
        end
    }
}

pub fn build_chunk_widths(
    chunk_headers: &[String],
    chunk_data: &[Vec<String>],
    explizite_breiten: &[usize],
    start: usize,
    end: usize,
    available_total: usize,
) -> Vec<usize> {
    let min_width = effective_min_column_width();
    let chunk_overhead = chunk_headers.len() * (COLUMN_OVERHEAD + 1);
    let chunk_budget = available_total.saturating_sub(chunk_overhead);

    let mut chunk_widths =
        compute_column_widths_from_global_mass(chunk_headers, chunk_data, chunk_budget);

    if chunk_widths.len() != chunk_headers.len() {
        chunk_widths.resize(chunk_headers.len(), min_width);
    }

    for width in chunk_widths.iter_mut() {
        *width = clamp_chunk_width(*width);
    }

    for (local_i, global_i) in (start..end).enumerate() {
        if let Some(breite) = get_explicit_width(explizite_breiten, global_i) {
            chunk_widths[local_i] = clamp_explicit_width(breite);
        }
    }

    let explicit_mask = explicit_mask_for_range(explizite_breiten, start, end);

    shrink_widths_to_budget_preserving_explicit(
        &mut chunk_widths,
        chunk_budget,
        min_width,
        &explicit_mask,
    );

    stretch_last_non_explicit_or_last_column(&mut chunk_widths, chunk_budget, &explicit_mask);

    chunk_widths
}

pub fn stretch_last_column_to_fill_budget(widths: &mut [usize], chunk_budget: usize) {
    if widths.is_empty() {
        return;
    }

    let current_total: usize = widths.iter().sum();
    if current_total < chunk_budget {
        let extra = chunk_budget - current_total;
        let last_idx = widths.len() - 1;
        widths[last_idx] += extra;
    }
}
