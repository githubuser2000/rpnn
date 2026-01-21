use rusqlite::{Connection, Result};
K Kontinuum;Begründung;Verbundgrad;Nr.;\n Radialfäden (Speichenfäden);Sie verbinden die Nabe mit dem Rahmen, kreuzen viele andere Fäden (Hilfsspirale, Fangspirale) und bilden das tragende Skelett des Netzes.;hoch;1;\n Rahmenfäden (Rahmenstruktur);Umfassen das gesamte Netz und verbinden alle Radialfäden an der Peripherie – wesentlich für Gesamtstabilität und Spannung.;hoch;2;\n Nabe (Zentrum);\"Zentraler Verbindungspunkt aller Radialfäden; struktureller und funktionaler Mittelpunkt des Netzes.\";hoch;3;\n Fangspirale (Klebespirale);\"Berührt alle Radialfäden; verknüpft Netzbereiche funktional (Beutefang), stabilisiert aber weniger stark als Rahmen und Speichen.\";mittel–hoch;4;\n Hilfsspirale (Bauhilfsspirale);Temporäres Verbindungselement, das während des Baus Orientierung und Spannung gibt, danach aber teilweise entfernt wird.;mittel;5;\n Brückenfäden;\"Dienen als erste Verbindungsbasis und tragen nach außen zur Befestigung bei; verbinden äußere Punkte, aber nicht viele interne Elemente.\";mittel;6;\n Stützfäden (Sekundärfäden);Verstärken lokale Spannungen und verbinden Teilbereiche, sind aber keine Hauptachsen.;mittel;7;\n Rahmenkreuzungen;\"Lokale Verbindungsknoten zwischen Rahmen- und Radialfäden; wichtig für Festigkeit, aber nur punktuell wirksam.\";mittel–niedrig;8;\n Anheftungspunkte (Verankerungsfäden);\"Verbinden das Netz mit der Umgebung; wichtig für Halt, aber nicht für interne Strukturvernetzung.\";niedrig–mittel;9;\n Signal- oder Meldefaden;\"Verbunden meist nur mit Nabe und Rückzugsort der Spinne; funktional wichtig, strukturell aber kaum verbindend.\";niedrig;10;\n Anheftungsschleifen (Verklebungspunkte);\"Sehr kleine lokale Verbindungen zur Fixierung; kaum Einfluss auf das Gesamtgefüge.\";niedrig;11;\n 
use crate::cli::TextBereich; // Angenommen, TextBereich ist in main oder lib definiert

pub fn get_column_names(conn: &Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("PRAGMA table_info(csv_data)")?;
    let names = stmt.query_map([], |row| row.get(1))?
        .collect::<Result<Vec<String>, _>>()?;
    Ok(names)
}

pub fn fetch_data(conn: &Connection, names: &[String], bereich: &TextBereich) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let targets_sql = names.join(", ");
    let anzahl = if bereich.bis_zeile >= bereich.von_zeile {
        bereich.bis_zeile - bereich.von_zeile
    } else { 0 };

    println!( "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        targets_sql, anzahl, bereich.von_zeile);
    let query = format!(
        "SELECT {} FROM csv_data LIMIT {} OFFSET {}",
        targets_sql, anzahl, bereich.von_zeile
    );

    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;
    let mut data = Vec::new();

    while let Some(row) = rows.next()? {
        let mut row_values = Vec::new();
        for i in 0..names.len() {
            row_values.push(row.get(i).unwrap_or_default());
        }
        data.push(row_values);
    }
    Ok(data)
}

