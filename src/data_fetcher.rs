use rusqlite::Connection;

pub fn fetch_data_with_stats(
    conn: &Connection,
    query: &str,
    column_count: usize,
    header_lengths: &[usize],
) -> Result<(Vec<Vec<String>>, Vec<usize>), Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query([])?;

    let mut all_data = Vec::new();
    let mut max_lengths = header_lengths.to_vec();

    while let Some(row) = rows.next()? {
        let mut values = Vec::new();
        for i in 0..column_count {
            let val: String = row.get(i).unwrap_or_default();
            let len = val.chars().count();
            if len > max_lengths[i] {
                max_lengths[i] = len;
            }
            values.push(val);
        }
        all_data.push(values);
    }

    Ok((all_data, max_lengths))
}
