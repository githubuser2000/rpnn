use comfy_table::{Table, ContentArrangement, ColumnConstraint, Width, presets};
use terminal_size::{terminal_size, Width as TermWidth};

pub fn calculate_max_lengths(headers: &[String], data: &[Vec<String>]) -> Vec<usize> {
    let mut max_lengths: Vec<usize> = headers.iter().map(|n| n.len()).collect();
    for row in data {
        for (i, val) in row.iter().enumerate() {
            max_lengths[i] = max_lengths[i].max(val.chars().count());
        }
    }
    max_lengths
}

pub fn print_formatted_table(headers: Vec<String>, data: Vec<Vec<String>>, max_lengths: Vec<usize>) {
    let mut table = Table::new();
    let term_width = terminal_size().map(|(TermWidth(w), _)| w).unwrap_or(100);

    table.set_content_arrangement(ContentArrangement::DynamicFullWidth)
         .set_width(term_width)
         .load_preset(presets::UTF8_FULL)
         .set_header(&headers);

    let gesamt_zeichen: usize = max_lengths.iter().sum();

    for i in 0..headers.len() {
        let anteil = max_lengths[i] as f32 / gesamt_zeichen.max(1) as f32;
        let prozent = (anteil * 100.0) as u16;
        let column = table.column_mut(i).unwrap();
        column.set_constraint(ColumnConstraint::UpperBoundary(Width::Percentage(prozent.max(5))));
    }

    for row in data {
        table.add_row(row);
    }

    if !headers.is_empty() {
        println!("{table}");
    }
}

