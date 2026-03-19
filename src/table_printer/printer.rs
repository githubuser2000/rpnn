use std::collections::BTreeSet;

use crate::reta_ausgabe::{CliOutput, OutputSyntax, TableRow, Tables};
use crate::table_printer::config::{COLUMN_OVERHEAD, MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH};
use crate::table_printer::table_utils::{
    build_table_layout,
    compute_column_stats,
    compute_column_widths_from_global_mass,
    convert_to_table_rows,
    convert_to_table_rows_with_offset,
    get_terminal_width,
    RowRange,
};


/// Erstellt das Ausgabeobjekt für das eigentliche Tabellen-Rendering.
///
/// Was die Funktion macht:
/// - Baut ein `CliOutput`-Objekt auf
/// - Schaltet Farbausgabe, Tabellenbreite, Spaltenbreiten und Zeilennummern ein
/// - Konfiguriert die Ausgabe so, dass immer genau eine Tabelle gerendert wird
///
/// Parameter:
/// - `tables`:
///   Tabellen-Kontext aus deiner Ausgabelogik
/// - `term_width`:
///   Aktuelle Terminalbreite in Zeichen
/// - `column_widths`:
///   Finale Breiten der Spalten, die gerendert werden sollen
///
/// Rückgabe:
/// - Ein fertig konfiguriertes `CliOutput`
fn build_output<'a>(
    tables: &'a Tables,
    term_width: usize,
    column_widths: Vec<usize>,
) -> CliOutput<'a> {
    let mut output = CliOutput::new(tables, OutputSyntax::Plain);
    output.color_enabled = true;
    output.table_width = term_width;
    output.column_widths = column_widths;
    output.line_numbering = true;
    output.one_table = true;
    output
}

/// Rendert bereits vorbereitete `TableRow`-Strukturen auf das Terminal.
///
/// Was die Funktion macht:
/// - Erzeugt intern die Ausgabe-Struktur
/// - Bestimmt, wie viele Zeilen in den Zellen maximal nötig sind
/// - Rendert dann alle sichtbaren Tabellenzeilen
///
/// Parameter:
/// - `term_width`:
///   Terminalbreite in Zeichen
/// - `column_widths`:
///   Spaltenbreiten für den aktuell zu rendernden Chunk
/// - `table_rows`:
///   Bereits umgewandelte Tabellenzeilen
///
/// Rückgabe:
/// - Keine; die Funktion druckt direkt auf stdout
fn render_rows(term_width: usize, column_widths: Vec<usize>, table_rows: &[TableRow]) {
    let tables = Tables::new(Some(100));
    let mut output = build_output(&tables, term_width, column_widths);

    let display_lines: BTreeSet<usize> = (0..table_rows.len()).collect();

    let max_lines_in_cells = table_rows
        .iter()
        .map(TableRow::max_line_count)
        .max()
        .unwrap_or(1);

    let rows_range = 0..max_lines_in_cells;
    output.cli_out(&display_lines, table_rows, rows_range);
}

pub fn print_table(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
) {
    print_table_with_offset(headers, data, row_ranges, 1);
}

pub fn print_table_with_offset(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
    original_start_line: usize,
) {
    let term_width = get_terminal_width();
    let available_budget = term_width
        .saturating_sub(1)
        .saturating_sub(headers.len() * COLUMN_OVERHEAD);

    let column_widths =
        compute_column_widths_from_global_mass(headers, data, available_budget);

    let table_rows = convert_to_table_rows_with_offset(
        headers,
        data,
        &column_widths,
        row_ranges,
        original_start_line,
    );

    render_rows(term_width, column_widths, &table_rows);
}


/// Ermittelt eine grobe natürliche Breite einer einzelnen Spalte,
/// ohne schon das gesamte Chunk-Budget hart zu verteilen.
/// Diese Heuristik dient NUR dazu, zu entscheiden,
/// wie viele Spalten ungefähr in einen Chunk passen.
/// Schätzt eine grobe natürliche Breite einer einzelnen Spalte für die Chunk-Bildung.
///
/// Was die Funktion macht:
/// - Betrachtet genau eine Spalte isoliert
/// - Berechnet dafür lokale Spaltenstatistiken
/// - Verwendet die durchschnittliche Zellbreite als Heuristik
///
/// Wofür diese Funktion da ist:
/// - Nicht für das finale Rendern
/// - Sondern nur dafür, grob abzuschätzen,
///   wie viele Spalten in den nächsten Chunk passen
///
/// Warum das wichtig ist:
/// - Die alte Logik hat globale Breiten genommen
/// - Dadurch wurden spätere Chunks systematisch verzerrt
/// - Diese Funktion trennt Chunk-Bildung von finaler Breitenberechnung
///
/// Parameter:
/// - `header`:
///   Header der betrachteten Spalte
/// - `data`:
///   Gesamte Tabellendaten
/// - `col_idx`:
///   Globaler Index der Spalte in der Gesamttabelle
///
/// Rückgabe:
/// - Eine grob geschätzte Breite dieser Spalte
fn estimate_natural_width_for_chunking(
    header: &String,
    data: &[Vec<String>],
    col_idx: usize,
) -> usize {
    // Baue künstlich eine 1-Spalten-Tabelle,
    // damit die bestehende Statistikfunktion wiederverwendet werden kann.
    let single_header = vec![header.clone()];

    let single_col_data: Vec<Vec<String>> = data
        .iter()
        .map(|row| vec![row.get(col_idx).cloned().unwrap_or_default()])
        .collect();

    let stats = compute_column_stats(&single_header, &single_col_data);
    // Nutze den Durchschnitt als grobe natürliche Breite.
    let guessed = stats
        .first()
        .map(|s| s.avg_width.ceil() as usize)
        .unwrap_or(MIN_COLUMN_WIDTH);

    guessed.clamp(MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH)
}

/// Gibt die Tabelle chunkweise aus.
/// Jeder Chunk bekommt seine eigene Statistik und eigene Breitenberechnung.
/// Dadurch werden spätere Chunks nicht mehr durch frühe Spalten "leerbudgetiert".
/// Gibt die Tabelle chunkweise aus.
///
/// Was die Funktion macht:
/// - Zerlegt eine breite Tabelle in mehrere untereinander ausgegebene Chunks
/// - Berechnet für jeden Chunk eigene Spaltenbreiten
/// - Beachtet optionale explizite Breiten pro globaler Spalte
///
/// Wichtige Designidee:
/// - Chunk-Grenzen werden nur grob heuristisch bestimmt
/// - Die echten Spaltenbreiten werden danach pro Chunk lokal berechnet
///
/// Dadurch werden zwei alte Fehler behoben:
/// 1. Die Statistik wird nicht mehr nur einmal global am Anfang berechnet
/// 2. Spätere Chunks werden nicht mehr durch frühe Spalten "dünn gerechnet"
///
/// Parameter:
/// - `headers`:
///   Alle Header der Gesamttabelle
/// - `data`:
///   Gesamte Tabellendaten
/// - `row_ranges`:
///   Angabe, welche Zeilenbereiche pro Zelle sichtbar sein sollen
/// - `explizite_breiten`:
///   Optional vom Benutzer vorgegebene Breiten pro globaler Spalte
///
/// Rückgabe:
/// - Keine; die Funktion druckt direkt die Tabelle
pub fn print_table_chunked(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
    explizite_breiten: &[usize],
) {
    print_table_chunked_with_offset(
        headers,
        data,
        row_ranges,
        explizite_breiten,
        1,
    );
}

pub fn print_table_chunked_with_offset(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
    explizite_breiten: &[usize],
    original_start_line: usize,
) {
    let term_width = get_terminal_width();
    let available_total = term_width.saturating_sub(3);
    let mut start = 0usize;

    while start < headers.len() {
        let mut end = start;
        let mut used = 0usize;

        while end < headers.len() {
            let guessed_width = if let Some(&breite) = explizite_breiten.get(end) {
                breite.clamp(MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH)
            } else {
                estimate_natural_width_for_chunking(&headers[end], data, end)
            };

            let needed = guessed_width + COLUMN_OVERHEAD + 1;

            if used + needed > available_total {
                if end == start {
                    end += 1;
                }
                break;
            }

            used += needed;
            end += 1;
        }

        if end <= start {
            end = (start + 1).min(headers.len());
        }

        let chunk_headers: Vec<String> = headers[start..end].to_vec();

        let chunk_data: Vec<Vec<String>> = data
            .iter()
            .map(|row| {
                (start..end)
                    .map(|i| row.get(i).cloned().unwrap_or_default())
                    .collect()
            })
            .collect();

        let chunk_overhead = chunk_headers.len() * (COLUMN_OVERHEAD + 1);
        let chunk_budget = available_total.saturating_sub(chunk_overhead);

        let mut chunk_widths =
            compute_column_widths_from_global_mass(&chunk_headers, &chunk_data, chunk_budget);

        for (local_i, global_i) in (start..end).enumerate() {
            if let Some(&breite) = explizite_breiten.get(global_i) {
                chunk_widths[local_i] = breite.clamp(MIN_COLUMN_WIDTH, MAX_COLUMN_WIDTH);
            }
        }

        let current_sum: usize = chunk_widths.iter().sum();
        if current_sum > chunk_budget {
            let mut shrinkable = chunk_widths.clone();
            let mut current_total: usize = shrinkable.iter().sum();

            while current_total > chunk_budget {
                let mut changed = false;

                for w in shrinkable.iter_mut() {
                    if *w > MIN_COLUMN_WIDTH && current_total > chunk_budget {
                        *w -= 1;
                        current_total -= 1;
                        changed = true;
                    }
                }

                if !changed {
                    break;
                }
            }

            chunk_widths = shrinkable;
        }

        let table_rows = convert_to_table_rows_with_offset(
            &chunk_headers,
            &chunk_data,
            &chunk_widths,
            row_ranges,
            original_start_line,
        );

        render_rows(term_width, chunk_widths, &table_rows);
        start = end;
    }
}

pub fn print_table_auto(
    headers: &[String],
    data: &[Vec<String>],
    row_ranges: &[RowRange],
) {
    let layout = build_table_layout(headers, data);
    let table_rows = convert_to_table_rows(headers, data, &layout.column_widths, row_ranges);

    render_rows(layout.term_width, layout.column_widths, &table_rows);
}
