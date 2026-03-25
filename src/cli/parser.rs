use crate::if_is_zeilen_angabe::is_zeilen_angabe;

use super::bereich::TextBereich;
use super::parser_ranges::parse_zeilenangabe_zu_bereichen;
use super::parser_support::{
    apply_pypy_compat_arg, is_flag, parse_usize_csv_list, print_all_oberkategorien,
    print_passende_unterkategorien,
};
pub use super::parser_types::{SpaltenNamen, SpaltenNamenListe};

pub fn parse_cli_args(
    args: &[String],
    kategorie_map: Option<&crate::column_categories_complete::KategorieMap>,
) -> (Vec<usize>, Vec<String>, TextBereich, SpaltenNamen, SpaltenNamenListe) {
    let mut minuses = Vec::with_capacity(args.len());
    let mut params = Vec::with_capacity(args.len());

    let mut bereich = TextBereich::default();
    let mut spalten_namen = SpaltenNamen::default();
    let mut spalten_namen_liste = SpaltenNamenListe::default();

    let automatische_spalten_suche = false;
    let gesuchte_oberkategorie = String::new();
    let gesuchte_unterkategorie = String::new();

    let mut iter = args.iter().enumerate().peekable();

    while let Some((_i, arg)) = iter.next() {
        let mut dash_count = 0;
        for c in arg.chars() {
            if c == '-' {
                dash_count += 1;
            } else {
                break;
            }
        }

        if let Some(value) = arg.strip_prefix("--breite=") {
            let breite = value
                .trim()
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("Ungültige Breite '{}': keine Zahl", value));

            if breite == 0 {
                panic!("--breite muss größer als 0 sein");
            }

            bereich.breiten = vec![breite];
        } else if let Some(value) = arg.strip_prefix("--breiten=") {
            bereich.breiten = parse_usize_csv_list(value, "--breiten");
        } else if let Some(value) = arg.strip_prefix("--spaltenreihenfolgeundnurdiese=") {
            bereich.spaltenreihenfolgeundnurdiese =
                parse_usize_csv_list(value, "--spaltenreihenfolgeundnurdiese");
        } else {
            match arg.as_str() {
                "--vorhervonausschnitt" => {
                    if let Some((_, nachfolger)) = iter.next() {
                        let mut vielfache = false;
                        let mut primfaktoren = false;
                        let mut zeilentext = nachfolger.clone();

                        while zeilentext.starts_with('v') || zeilentext.starts_with('w') {
                            if zeilentext.starts_with('v') {
                                vielfache = true;
                            } else {
                                primfaktoren = true;
                            }
                            zeilentext.remove(0);
                        }

                        while zeilentext.ends_with('v') || zeilentext.ends_with('w') {
                            if zeilentext.ends_with('v') {
                                vielfache = true;
                            } else {
                                primfaktoren = true;
                            }
                            zeilentext.pop();
                        }

                        bereich.vorher_vielfache |= vielfache;
                        bereich.vorher_primfaktoren |= primfaktoren;

                        if is_zeilen_angabe(&zeilentext) {
                            if let Some(bereichspaare) =
                                parse_zeilenangabe_zu_bereichen(&zeilentext)
                            {
                                if !bereichspaare.is_empty() {
                                    bereich.zeilen_bereiche = bereichspaare.clone();
                                    bereich.von_zeile = bereichspaare[0].0;

                                    if let Some(last_bereich) = bereichspaare.last() {
                                        bereich.bis_zeile = last_bereich.1;
                                    }
                                }
                            } else if let Ok(zahl) = zeilentext.parse::<usize>() {
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
                            panic!("Ungültige Breite '{}': keine Zahl", nachfolger)
                        });

                        if breite == 0 {
                            panic!("--breite muss größer als 0 sein");
                        }

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
                        bereich.breiten = parse_usize_csv_list(nachfolger, "--breiten");
                    } else {
                        panic!("--breiten erwartet eine kommagetrennte Zahlenliste");
                    }
                }
                "-spalten" | "-kombination" => {
                    if let Some((_, nachfolger)) = iter.peek() {
                        let candidate = (*nachfolger).as_str();
                        if apply_pypy_compat_arg(&mut bereich, candidate) {
                            iter.next();
                        }
                    }
                }
                "--spaltenname" => {
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
                        bereich.spaltenreihenfolgeundnurdiese =
                            parse_usize_csv_list(nachfolger, "--spaltenreihenfolgeundnurdiese");
                    } else {
                        panic!(
                            "--spaltenreihenfolgeundnurdiese erwartet eine kommagetrennte Zahlenliste"
                        );
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
                    println!(
                        "  mein-rpnn --spalten Universum Transzendentalien --zeilevon 1 --zeilebis 10"
                    );
                }
                _ => {
                    let _ = apply_pypy_compat_arg(&mut bereich, arg);
                }
            }
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
                &gesuchte_unterkategorie,
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
            bereich.spalten_bereiche.clear();
            bereich.spaltenreihenfolgeundnurdiese.clear();
            bereich.exact_visible_columns.clear();
            bereich.von_spalte = usize::MAX;
            bereich.bis_spalte = usize::MAX;
            bereich.spalten_gefunden = false;
            bereich.spalten_gesucht = false;
            bereich.spalten_gesucht2 = false;
        }
    }

    (minuses, params, bereich, spalten_namen, spalten_namen_liste)
}
