use crate::table_printer::table_utils::natural_column_widths;

pub fn render_table_shell(table: &[Vec<String>]) -> String {
    let widths = natural_column_widths(table);
    let mut lines = Vec::new();
    for row in table {
        let parts = row
            .iter()
            .enumerate()
            .map(|(idx, cell)| format!("{:<width$}", cell, width = widths.get(idx).copied().unwrap_or(0)))
            .collect::<Vec<_>>();
        lines.push(parts.join(" | "));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printer_keeps_all_cells() {
        let table = vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string(), "d".to_string()]];
        let rendered = render_table_shell(&table);
        assert!(rendered.contains('a'));
        assert!(rendered.contains('d'));
    }
}
