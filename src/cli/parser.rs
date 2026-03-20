use crate::if_is_zeilen_angabe::{is_zeilen_angabe, str_as_generator_to_vec_i64};
use super::bereich::TextBereich;
use std::collections::BTreeSet;

// Neuer Datentyp für Spaltennamen-Konfiguration
#[derive(Debug, Clone)]
pub struct SpaltenNamen {
    pub oberkategorie: String,
    pub unterkategorie: String,
}

#[derive(Debug, Clone, Default)]
pub struct SpaltenNamenListe {
    pub eintraege: Vec<SpaltenNamen>,
}

impl Default for SpaltenNamen {
    fn default() -> Self {
        Self {
            oberkategorie: String::new(),
            unterkategorie: String::new(),
        }
    }
}

fn is_flag(s: &str) -> bool {
    s.starts_with('-')
}

fn parse_vorhervonausschnitt_token(input: &str) -> (String, bool, bool) {
    let trimmed = input.trim();
    let mut vielfache = false;
    let mut primfaktoren = false;

    let mut core = trimmed;

    loop {
        let mut changed = false;

        if let Some(rest) = core.strip_prefix('v') {
            vielfache = true;
            core = rest;
            changed = true;
        }
        if let Some(rest) = core.strip_prefix('w') {
            primfaktoren = true;
            core = rest;
            changed = true;
        }
        if let Some(rest) = core.strip_suffix('v') {
            vielfache = true;
            core = rest;
            changed = true;
        }
        if let Some(rest) = core.strip_suffix('w') {
            primfaktoren = true;
            core = rest;
            changed = true;
        }

        if !changed {
            break;
        }
    }

    (core.trim().to_string(), vielfache, primfaktoren)
}

fn print_all_oberkategorien(
    kategorie_map: Option<&crate::column_categories_complete::KategorieMap>,
) {
    if let Some(kategorie_map) = kategorie_map {
        let mut set = BTreeSet::new();

        for eintrag in &kategorie_map.alle_eintraege {
            let ok = eintrag.oberkategorie.trim();
            if !ok.is_empty() {
                set.insert(ok.to_string());
            }
        }

        println!("Mögliche erste Wörter nach --spaltenname:");
        for item in set {
            println!("{item}");
        }
    } else {
        println!("Keine Kategoriedaten verfügbar.");
    }
}

fn print_passende_unterkategorien(
    kategorie_map: Option<&crate::column_categories_complete::KategorieMap>,
    oberkategorie: &str,
) {
    if let Some(kategorie_map) = kategorie_map {
        let mut set = BTreeSet::new();
        let needle = oberkategorie.to_lowercase();

        for eintrag in &kategorie_map.alle_eintraege {
            if eintrag.oberkategorie.to_lowercase() == needle {
                let uk = eintrag.unterkategorie.trim();
                if !uk.is_empty() {
                    set.insert(uk.to_string());
                }
            }
        }

        if set.is_empty() {
            println!("Keine passenden zweiten Wörter für '{oberkategorie}' gefunden.");
        } else {
            println!("Mögliche zweite Wörter für '{oberkategorie}':");
            for item in set {
                println!("{item}");
            }
        }
    } else {
        println!("Keine Kategoriedaten verfügbar.");
    }
}

// Rückgabetyp erweitert um Spaltennamen
pub fn parse_cli_args(
    args: &[String],
    kategorie_map: Option<&crate::column_categories_complete::KategorieMap>
) -> (Vec<usize>, Vec<String>, TextBereich, SpaltenNamen, SpaltenNamenListe) {
    let mut minuses = Vec::with_capacity(args.len());
    let mut params = Vec::with_capacity(args.len());

    let mut bereich = TextBereich::default();
    let mut spalten_namen = SpaltenNamen::default();

    let automatische_spalten_suche = false;
    let gesuchte_oberkategorie = String::new();
    let gesuchte_unterkategorie = String::new();

    let mut iter = args.iter().enumerate().peekable();
    let mut spalten_namen_liste = SpaltenNamenListe::default();

    while let Some((_i, arg)) = iter.next() {
        let mut dash_count = 0;

        for c in arg.chars() {
            if c == '-' {
                dash_count += 1;
            } else {
                break;
            }
        }

        match arg.as_str() {
            "--vorhervonausschnitt" => {
                if let Some((_, nachfolger)) = iter.next() {
                    println!("📋 Verarbeite --vorhervonausschnitt: {}", nachfolger);

                    let (bereinigte_angabe, vielfache, primfaktoren) =
                        parse_vorhervonausschnitt_token(nachfolger);

                    bereich.vorher_vielfache = vielfache;
                    bereich.vorher_primfaktoren = primfaktoren;

                    if is_zeilen_angabe(&bereinigte_angabe) {
                        if let Some(bereichspaare) = parse_zeilenangabe_zu_bereichen(&bereinigte_angabe) {
                            if !bereichspaare.is_empty() {
                                bereich.zeilen_bereiche = bereichspaare.clone();
                                bereich.von_zeile = bereichspaare[0].0;

                                if let Some(last_bereich) = bereichspaare.last() {
                                    bereich.bis_zeile = last_bereich.1;
                                }
                            }
                        } else if let Ok(zahl) = bereinigte_angabe.parse::<usize>() {
                            bereich.zeilen_bereiche.push((zahl, zahl));
                            bereich.von_zeile = zahl;
                            bereich.bis_zeile = zahl;
                        }
                    }
                }
            }
            "--breite" => {
                if let Some((_, nachfolger)) = iter.next() {
                    let breite = nachfolger.trim().parse::<usize>().unwrap_or_else(|_| {
                        panic!(
                            "Ungültige Breite '{}': keine Zahl",
                            nachfolger
                        )
                    });

                    if breite == 0 {
                        panic!("--breite muss größer als 0 sein");
                    }

                    // Eine einzige Breite = globale Breite für alle Spalten
                    bereich.breiten = vec![breite];
                } else {
                    panic!("--breite erwartet genau eine Zahl");
                }
            }
            "--keineleereninhalte" => {
                bereich.keineleereninhalte = true;
            }

            "--breiten" => {
                if let Some((_, nachfolger)) = iter.next() {
                    let breiten: Vec<usize> = nachfolger
                        .split(',')
                        .map(|s| {
                            let trimmed = s.trim();
                            trimmed.parse::<usize>().unwrap_or_else(|_| {
                                panic!(
                                    "Ungültige Breitenliste '{}': '{}' ist keine Zahl",
                                    nachfolger, trimmed
                                )
                            })
                        })
                        .collect();

                    if breiten.is_empty() {
                        panic!("--breiten darf nicht leer sein");
                    }

                    bereich.breiten = breiten;
                } else {
                    panic!("--breiten erwartet eine kommagetrennte Zahlenliste");
                }
            }

            "--spaltenname" => {
                // Fall 1: gar nichts dahinter oder direkt nächster Flag
                let first = match iter.peek() {
                    None => {
                        print_all_oberkategorien(kategorie_map);
                        std::process::exit(0);
                    }
                    Some((_, next_arg)) if is_flag(next_arg) => {
                        print_all_oberkategorien(kategorie_map);
                        std::process::exit(0);
                    }
                    Some(_) => {
                        let (_, v) = iter.next().unwrap();
                        v.clone()
                    }
                };

                // Fall 2: erstes Wort da, aber zweites fehlt oder nächster Wert ist schon ein Flag
                let second = match iter.peek() {
                    None => {
                        print_passende_unterkategorien(kategorie_map, &first);
                        std::process::exit(0);
                    }
                    Some((_, next_arg)) if is_flag(next_arg) => {
                        print_passende_unterkategorien(kategorie_map, &first);
                        std::process::exit(0);
                    }
                    Some(_) => {
                        let (_, v) = iter.next().unwrap();
                        v.clone()
                    }
                };

                // Fall 3: beide da -> normal übernehmen
                spalten_namen.oberkategorie = first.clone();
                spalten_namen.unterkategorie = second.clone();

                spalten_namen_liste.eintraege.push(SpaltenNamen {
                    oberkategorie: first,
                    unterkategorie: second,
                });

                bereich.spalten_gesucht = true;
                bereich.spalten_gesucht2 = true;
            }

            "--spaltenreihenfolgeundnurdiese" => {
                if let Some((_, nachfolger)) = iter.next() {
                    let spalten: Vec<usize> = nachfolger
                        .split(',')
                        .map(|s| {
                            s.parse::<usize>().unwrap_or_else(|_| {
                                panic!(
                                    "Ungültige Spaltenliste '{}': '{}' ist keine Zahl",
                                    nachfolger, s
                                )
                            })
                        })
                        .collect();

                    if spalten.is_empty() {
                        panic!("--spaltenreihenfolgeundnurdiese darf nicht leer sein");
                    }

                    bereich.spaltenreihenfolgeundnurdiese = spalten;
                } else {
                    panic!("--spaltenreihenfolgeundnurdiese erwartet eine kommagetrennte Zahlenliste");
                }
            }

            "--zeilevon" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.von_zeile = zahl;
                        if bereich.zeilen_bereiche.is_empty() {
                            bereich.zeilen_bereiche.push((zahl, zahl));
                        } else if let Some(last) = bereich.zeilen_bereiche.last_mut() {
                            last.0 = zahl;
                        }
                    }
                }
            }

            "--zeilebis" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.bis_zeile = zahl;
                        if let Some(last) = bereich.zeilen_bereiche.last_mut() {
                            last.1 = zahl;
                        }
                    }
                }
            }

            "--spaltevon" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.von_spalte = zahl;
                        bereich.spalten_bereiche.push((zahl, zahl));
                        bereich.spalten_gesucht = true;
                    }
                }
            }

            "--spaltebis" => {
                if let Some((_, nachfolger)) = iter.next() {
                    if let Ok(zahl) = nachfolger.parse::<usize>() {
                        bereich.bis_spalte = zahl;
                        if let Some(last) = bereich.spalten_bereiche.last_mut() {
                            last.1 = zahl;
                            bereich.spalten_gesucht = true;
                        }
                    }
                }
            }

            "--help" | "-h" => {
                println!("📖 Hilfe:");
                println!("  --spalten OBER UNTER      Suche Spaltennummern für Kategorien");
                println!("  --spaltenname OBER UNTER  Setze Spaltennamen (für SQL)");
                println!("  --vorhervonausschnitt Z   Zeilenbereiche (z.B. 1-10, 1,3,5)");
                println!("  --zeilevon ZAHL           Startzeile");
                println!("  --zeilebis ZAHL           Endzeile");
                println!("  --spaltevon ZAHL          Startspalte");
                println!("  --spaltebis ZAHL          Endspalte");
                println!("  --help, -h                Diese Hilfe");
                println!();
                println!("Beispiele:");
                println!("  mein-rpnn --spalten Menschliches Motive");
                println!("  mein-rpnn --spalten Universum Transzendentalien --zeilevon 1 --zeilebis 10");
            }

            _ => {}
        }

        let param = if dash_count > 0 {
            arg.chars().skip(dash_count).collect()
        } else {
            arg.clone()
        };

        minuses.push(dash_count);
        params.push(param);
    }

    if automatische_spalten_suche {
        if let Some(kategorie_map) = kategorie_map {
            let gefundene_spalten = kategorie_map.finde_spaltennummern_exakt(
                &gesuchte_oberkategorie,
                &gesuchte_unterkategorie
            );

            if !gefundene_spalten.is_empty() {
                println!("✅ {} Spaltennummern gefunden", gefundene_spalten.len());

                let mut sorted: Vec<usize> =
                    gefundene_spalten.iter().map(|&n| n as usize).collect();
                sorted.sort();
                sorted.dedup();

                bereich.spalten_bereiche.clear();
                for &num in &sorted {
                    bereich.spalten_bereiche.push((num, num));
                }

                if !bereich.spalten_bereiche.is_empty() {
                    bereich.von_spalte = bereich.spalten_bereiche[0].0;
                    bereich.bis_spalte = bereich.spalten_bereiche.last().unwrap().1;
                }
            }
        } else {
            println!("⚠️  Kategoriedaten nicht verfügbar");
            bereich.spalten_bereiche.push((1, 1));
            bereich.von_spalte = 1;
            bereich.bis_spalte = 1;
        }
    }

    (minuses, params, bereich, spalten_namen, spalten_namen_liste)
}

// Hilfsfunktion zum Parsen von Zeilenangaben (bereinigt)
pub(crate) fn parse_zeilenangabe_zu_bereichen(text: &str) -> Option<Vec<(usize, usize)>> {
    let mut bereiche = Vec::new();

    if let Some(zahlen) = str_as_generator_to_vec_i64(text) {
        for &zahl in &zahlen {
            if zahl >= 0 {
                bereiche.push((zahl as usize, zahl as usize));
            }
        }

        if !bereiche.is_empty() {
            bereiche.sort_by(|a, b| a.0.cmp(&b.0));
            return Some(bereiche);
        }
    }

    let teile: Vec<&str> = text.split(',').collect();

    if teile.len() == 1 {
        let teil = teile[0].trim();

        if teil.contains('-') {
            let bereichs_teile: Vec<&str> = teil.split('-').collect();
            if bereichs_teile.len() == 2 {
                if let (Ok(von), Ok(bis)) = (
                    bereichs_teile[0].trim().parse::<usize>(),
                    bereichs_teile[1].trim().parse::<usize>()
                ) {
                    bereiche.push((von, bis));
                }
            }
        } else if let Ok(num) = teil.parse::<usize>() {
            bereiche.push((num, num));
        }
    } else {
        for teil in teile {
            let teil_trimmed = teil.trim();

            if teil_trimmed.is_empty() {
                continue;
            }

            if teil_trimmed.contains('-') {
                let bereichs_teile: Vec<&str> = teil_trimmed.split('-').collect();
                if bereichs_teile.len() == 2 {
                    if let (Ok(von), Ok(bis)) = (
                        bereichs_teile[0].trim().parse::<usize>(),
                        bereichs_teile[1].trim().parse::<usize>()
                    ) {
                        bereiche.push((von, bis));
                        continue;
                    }
                }
            }

            if let Ok(num) = teil_trimmed.parse::<usize>() {
                bereiche.push((num, num));
            }
        }
    }

    if !bereiche.is_empty() {
        bereiche.sort_by(|a, b| a.0.cmp(&b.0));
        Some(bereiche)
    } else {
        None
    }
}
