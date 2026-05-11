#![allow(non_snake_case)]

use crate::shared::reta_program_types::Program;

fn resulting_parallel_ranges_py(
    total_rows: usize,
    min_rows_per_worker: usize,
) -> Vec<(usize, usize)> {
    if total_rows == 0 {
        return Vec::new();
    }
    if total_rows <= 1 || total_rows < min_rows_per_worker.saturating_mul(2) {
        return vec![(0, total_rows)];
    }
    let max_workers_by_grain = (total_rows + min_rows_per_worker - 1) / min_rows_per_worker;
    let worker_count = std::thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(1)
        .min(max_workers_by_grain)
        .min(total_rows)
        .max(1);
    if worker_count <= 1 {
        return vec![(0, total_rows)];
    }

    let chunk_size = (total_rows + worker_count - 1) / worker_count;
    let mut ranges = Vec::new();
    let mut start = 0usize;
    while start < total_rows {
        let end = start.saturating_add(chunk_size).min(total_rows);
        ranges.push((start, end));
        start = end;
    }
    ranges
}

fn resulting_parallel_map_indexed_py<T, F>(
    total_rows: usize,
    min_rows_per_worker: usize,
    map_row: F,
) -> Vec<T>
where
    T: Send,
    F: Fn(usize) -> T + Sync,
{
    if total_rows == 0 {
        return Vec::new();
    }
    let ranges = resulting_parallel_ranges_py(total_rows, min_rows_per_worker);
    if ranges.len() <= 1 {
        return (0..total_rows).map(map_row).collect();
    }

    std::thread::scope(|scope| {
        let map_row = &map_row;
        let mut handles = Vec::new();
        for (start, end) in ranges {
            handles.push(scope.spawn(move || {
                let mut chunk = Vec::with_capacity(end - start);
                for row_idx in start..end {
                    chunk.push(map_row(row_idx));
                }
                chunk
            }));
        }

        let mut rows = Vec::with_capacity(total_rows);
        for handle in handles {
            match handle.join() {
                Ok(mut chunk) => rows.append(&mut chunk),
                Err(payload) => std::panic::resume_unwind(payload),
            }
        }
        rows
    })
}

impl Program {
    pub fn resultingTable(&mut self) -> Vec<Vec<String>> {
        self.__resultingTable.clone()
    }

    pub fn onlyThatColumns_py(
        &self,
        table: Vec<Vec<String>>,
        onlyThatColumns: Vec<i64>,
    ) -> Vec<Vec<String>> {
        if onlyThatColumns.is_empty() {
            return table;
        }

        let column_indices: Vec<usize> = onlyThatColumns
            .iter()
            .filter_map(|i| {
                if *i <= 0 {
                    None
                } else {
                    Some((*i - 1) as usize)
                }
            })
            .collect();

        let newTable: Vec<Vec<String>> =
            resulting_parallel_map_indexed_py(table.len(), 32, |row_idx| {
                let row = &table[row_idx];
                let mut newCol: Vec<String> = vec![];
                for idx in column_indices.iter().copied() {
                    if idx < row.len() {
                        newCol.push(row[idx].clone());
                    }
                }

                // Python hängt auch leere Zeilen an
                newCol
            });

        if !newTable.is_empty() {
            newTable
        } else {
            table
        }
    }

    pub fn onlyThatColumns_i64_py(&self, values: Vec<i64>, onlyThatColumns: Vec<i64>) -> Vec<i64> {
        if onlyThatColumns.is_empty() {
            return values;
        }

        let mut out: Vec<i64> = vec![];
        for i in onlyThatColumns.iter() {
            if *i <= 0 {
                continue;
            }
            let idx = (*i - 1) as usize;
            if idx < values.len() {
                out.push(values[idx]);
            }
        }

        out
    }
}
