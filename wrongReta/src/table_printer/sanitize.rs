pub fn filter_small_lines_in_cell(cell: &str) -> String {
    cell.lines()
        .map(str::trim)
        .filter(|line| line.chars().count() > 2)
        .map(ToOwned::to_owned)
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn sanitize_chunk_data(chunk_data: &[Vec<String>], keineleereninhalte: bool) -> Vec<Vec<String>> {
    if !keineleereninhalte {
        return chunk_data.to_vec();
    }

    chunk_data
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| filter_small_lines_in_cell(cell))
                .collect()
        })
        .collect()
}

pub fn row_has_visible_content(row: &[String]) -> bool {
    row.iter().any(|cell| {
        cell.lines()
            .map(str::trim)
            .any(|line| line.chars().count() > 2)
    })
}

pub fn sanitize_chunk_data_with_rows(
    chunk_data: &[Vec<String>],
    row_numbers: &[usize],
    keineleereninhalte: bool,
) -> (Vec<Vec<String>>, Vec<usize>) {
    let mut new_data = Vec::with_capacity(row_numbers.len().min(chunk_data.len()));
    let mut new_rows = Vec::with_capacity(row_numbers.len().min(chunk_data.len()));

    for (row, &num) in chunk_data.iter().zip(row_numbers.iter()) {
        if !keineleereninhalte {
            new_data.push(row.clone());
            new_rows.push(num);
            continue;
        }

        let cleaned_row: Vec<String> = row
            .iter()
            .map(|cell| filter_small_lines_in_cell(cell))
            .collect();

        if row_has_visible_content(&cleaned_row) {
            new_data.push(cleaned_row);
            new_rows.push(num);
        }
    }

    (new_data, new_rows)
}

pub fn sanitize_header_preserve_id(header: &str, global_index: usize) -> String {
    let trimmed = header.trim();

    if trimmed.is_empty() {
        return format!("SQL-Spalte {}", global_index + 1);
    }

    trimmed.to_string()
}


pub fn sanitize_header_for_output(header: &str, global_index: usize, structured: bool) -> String {
    let trimmed = header.trim();

    if trimmed.is_empty() {
        return format!("SQL-Spalte {}", global_index + 1);
    }

    if !structured {
        return trimmed.to_string();
    }

    let mut out = trimmed.to_string();
    loop {
        if let Some(pos) = out.rfind(" (ID_") {
            if out.ends_with(')') {
                out = out[..pos].trim_end().to_string();
                continue;
            }
        }
        break;
    }
    out
}
