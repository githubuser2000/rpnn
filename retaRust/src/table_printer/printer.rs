use crate::table_printer::table_utils::{compute_column_widths_linear_natural, shrink_widths_to_fit_budget};

fn truncate_cell(cell: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    cell.chars().take(width).collect()
}

pub fn render_shell_table(table: &[Vec<String>], budget: usize, min_width: usize) -> Vec<String> {
    let natural = compute_column_widths_linear_natural(table);
    let widths = shrink_widths_to_fit_budget(&natural, budget, min_width);
    table.iter().map(|row| {
        row.iter().enumerate().map(|(idx, cell)| {
            let width = widths.get(idx).copied().unwrap_or(min_width.max(1));
            format!("{:<width$}", truncate_cell(cell, width), width = width)
        }).collect::<Vec<_>>().join(" ")
    }).collect()
}
