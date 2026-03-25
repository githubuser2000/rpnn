use rusqlite::Connection;
use crate::cli::TextBereich;

pub fn get_column_names(conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
    let names = stmt
        .query_map([], |row| row.get(1))?
        .collect::<Result<Vec<String>, _>>()?;
    Ok(names)
}

pub fn collect_spalten_nummern(
    bereich: &mut TextBereich,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mut nums = Vec::new();

    if !bereich.spalten_bereiche.is_empty() {
        bereich.spalten_gefunden = true;

        for &(von, bis) in &bereich.spalten_bereiche {
            for i in von..=bis {
                nums.push(i);
            }
        }
    } else if bereich.von_spalte > 0 && bereich.bis_spalte > 0 {
        if bereich.von_spalte > bereich.bis_spalte {
            return Err("Startspalte > Endspalte".into());
        }
        for i in bereich.von_spalte..=bereich.bis_spalte {
            nums.push(i);
        }
    } else {
        if bereich.spalten_gesucht2 {
            return Err("Spalten wurden gesucht, aber keine verarbeitet".into());
        }
        nums.push(1);
    }

    nums.sort();
    nums.dedup();

    if nums.is_empty() {
        return Err("Keine Spaltennummern ausgewählt".into());
    }

    Ok(nums)
}

pub fn resolve_spaltennamen(
    column_names: &[String],
    spalten_nummern: &[usize],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut names = Vec::new();

    for &nr in spalten_nummern {
        if nr == 0 || nr > column_names.len() {
            return Err(format!("Spalte {} existiert nicht", nr).into());
        }

        let name = &column_names[nr - 1];
        names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
    }

    Ok(names)
}
