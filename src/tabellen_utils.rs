use crate::retaAusgabe::{Tables, CliOutput, OutputSyntax};
use crate::table_printer::print_table;

// 1. Funktion: Tabellentest
pub fn test_simple_table() {
    let tables = Tables::new(Some(100));
    let mut output = CliOutput::new(&tables, OutputSyntax::Plain);
    output.cliout2("TEST: Dies sollte ausgegeben werden\n");

    println!("\n=== EINFACHER TABELLEN-TEST ===");

    let headers = vec![
        "Name".to_string(),
        "Alter".to_string(),
        "Stadt".to_string(),
    ];
    
    let data = vec![
        vec!["Hans".to_string(), "25".to_string(), "Berlin".to_string()],
        vec!["Anna".to_string(), "30".to_string(), "München".to_string()],
        vec!["Peter".to_string(), "22".to_string(), "Hamburg".to_string()],
    ];

    let mut max_lengths = vec![0, 0, 0];
    for (i, header) in headers.iter().enumerate() {
        max_lengths[i] = max_lengths[i].max(header.len());
    }
    for row in &data {
        for (i, cell) in row.iter().enumerate() {
            if i < max_lengths.len() {
                max_lengths[i] = max_lengths[i].max(cell.len());
            }
        }
    }

    let zeilen_bereiche: Vec<(usize, usize)> = Vec::new();
    print_table(&headers, data, &max_lengths, &zeilen_bereiche);
}

// 2. Funktion: Anzeige der Nutzungshinweise
pub fn show_usage() {
    println!("Benutzung: mein-rpnn --spalten OBERKATEGORIE UNTERKATEGORIE");
    println!("Beispiel:  mein-rpnn --spalten Menschliches Motive");
    println!("Beispiel:  mein-rpnn --spalten Universum Transzendentalien");
    println!("\nAlternative mit manuellen Bereichen:");
    println!("mein-rpnn --vorhervonausschnitt 7-12 --spaltenname Menschliches Motive");
    println!("mein-rpnn --vorhervonausschnitt 7,9 --spaltenname Menschliches Motive");
}
