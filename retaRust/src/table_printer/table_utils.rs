pub fn natural_column_widths(table: &[Vec<String>]) -> Vec<usize> {
    let max_cols = table.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut widths = vec![0usize; max_cols];
    for row in table {
        for (idx, cell) in row.iter().enumerate() {
            widths[idx] = widths[idx].max(cell.chars().count());
        }
    }
    widths
}

pub fn shrink_widths_to_budget(widths: &[usize], budget: usize) -> Vec<usize> {
    if widths.is_empty() {
        return Vec::new();
    }
    let mut out = widths.to_vec();
    while out.iter().sum::<usize>() > budget && out.iter().any(|value| *value > 1) {
        if let Some((idx, _)) = out.iter().enumerate().max_by_key(|(_, value)| **value) {
            out[idx] -= 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widths_follow_longest_cell() {
        let table = vec![vec!["abc".to_string(), "x".to_string()], vec!["abcdef".to_string(), "xyz".to_string()]];
        assert_eq!(natural_column_widths(&table), vec![6, 3]);
    }
}
