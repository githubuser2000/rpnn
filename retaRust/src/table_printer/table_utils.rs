pub fn compute_column_widths_linear_natural(table: &[Vec<String>]) -> Vec<usize> {
    let col_count = table.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut widths = vec![0usize; col_count];
    for row in table {
        for (idx, cell) in row.iter().enumerate() {
            widths[idx] = widths[idx].max(cell.chars().count());
        }
    }
    widths
}

pub fn shrink_widths_to_fit_budget(widths: &[usize], budget: usize, min_width: usize) -> Vec<usize> {
    if widths.is_empty() {
        return Vec::new();
    }
    let min_width = min_width.max(1);
    let mut result = widths.to_vec();
    let mut current: usize = result.iter().sum();
    if current <= budget {
        return result;
    }
    while current > budget {
        let mut changed = false;
        for width in &mut result {
            if current <= budget {
                break;
            }
            if *width > min_width {
                *width -= 1;
                current -= 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    result
}
