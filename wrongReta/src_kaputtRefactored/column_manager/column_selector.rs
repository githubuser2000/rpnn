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
        bereich.mark_columns_resolved();

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
        if bereich.columns_pending() {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Col1(usize);

impl Col1 {
    fn try_new(value: usize) -> Result<Self, Box<dyn std::error::Error>> {
        if value == 0 {
            return Err("Spalte 0 existiert nicht".into());
        }
        Ok(Self(value))
    }

    fn to_zero_based(self) -> usize {
        self.0 - 1
    }

    fn get(self) -> usize {
        self.0
    }
}

pub fn resolve_spaltennamen(
    column_names: &[String],
    spalten_nummern: &[usize],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut names = Vec::new();

    for &nr in spalten_nummern {
        let col1 = Col1::try_new(nr)?;
        let idx0 = col1.to_zero_based();
        if idx0 >= column_names.len() {
            return Err(format!("Spalte {} existiert nicht", col1.get()).into());
        }

        let name = &column_names[idx0];
        names.push(format!("\"{}\"", name.replace("\"", "\"\"")));
    }

    Ok(names)
}
