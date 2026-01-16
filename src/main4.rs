use std::env;
#[derive(Debug)]
enum Element {
    Text(String),
    Liste(Vec<Element>),
}

fn main() {
    // Erstellung der Struktur: [["zeilen", ["nummer"]], ["spalten", ["nummer"]]]
    // Hilfsfunktionen definieren
    fn text(s: &str) -> Element {
        Element::Text(String::from(s))
    }

    fn list(elements: Vec<Element>) -> Element {
        Element::Liste(elements)
    }

    // Jetzt viel lesbarer
    let meine_liste = list(vec![
    list(vec![
        text("zeilen"),
            list(vec![
                text("nummer")
            ]),
        ]),
        list(vec![
           text("spalten"),
          list(vec![
                text("nummer")
            ]),
        ]),
    ]);
    println!("Struktur wurde erstellt. Hier ist die rekursive Ausgabe:");
    print_recursive(&meine_liste, 0);
    let args: Vec<String> = env::args().collect();
    deep_match(&args, &meine_liste, 0);
}

fn print_recursive(el: &Element, tiefe: usize) {
    let einrückung = "  ".repeat(tiefe);
    match el {
        Element::Text(t) => {
            println!("{}- {}", einrückung, t);
        }
        Element::Liste(l) => {
            println!("{}[Liste]:", einrückung);
            for kind in l {
                print_recursive(kind, tiefe + 1);
            }
        }
    }
}

fn process_cli_with_structure(args: &[String], structure: &Element) {
    match (args, structure) {
        ([arg1, arg2], Element::Liste(items)) => {
            for item in items {
                match item {
                    Element::Text(text1) if arg1 == text1 => {
                        println!("Level 1 match: {}", text1);
                        
                        match item {
                            Element::Liste(sub_items) => {
                                for sub_item in sub_items {
                                    match sub_item {
                                        Element::Text(text2) if arg2 == text2 => {
                                            println!("Level 2 direct match: {}", text2);
                                            return;
                                        }
                                        Element::Liste(third_items) => {
                                            for third_item in third_items {
                                                match third_item {
                                                    Element::Text(text3) if arg2 == text3 => {
                                                        println!("Level 3 match: {}", text3);
                                                        return;
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            println!("No match found");
        }
        _ => println!("Invalid arguments or structure"),
    }
}
/*
fn deep_match(args: &[String], element: &Element) -> Option<Vec<String>> {
    match element {
        Element::Text(text) => args.first()
            .filter(|arg| arg == &text)
            .map(|_| vec![text.clone()]),
        
        Element::Liste(items) => items.iter()
            .find_map(|item| match (args.first(), item) {
                (Some(arg), Element::Text(text)) if arg == text => {
                    let mut pfad = vec![text.clone()];
                    if let Some(rest) = deep_match(&args[1..], item) {
                        pfad.extend(rest);
                    }
                    Some(pfad)
                }
                (Some(_), Element::Liste(_)) => deep_match(args, item),
                _ => None,
            }),
    }
}*/


fn deep_match(args: &[String], element: &Element, tiefe: usize) -> Option<Vec<String>> {
    println!("{}Tiefe {}: Prüfe {:?}", "  ".repeat(tiefe), tiefe, element);

    match element {
        Element::Text(t) => {
            match args.first() {
                Some(arg) if arg == t => {
                    println!("{}✓ Tiefe {}: Text '{}' matcht Argument '{}'",
                            "  ".repeat(tiefe), tiefe, t, arg);
                    Some(vec![t.clone()])
                }
                Some(arg) => {
                    println!("{}✗ Tiefe {}: Text '{}' matcht NICHT Argument '{}'",
                            "  ".repeat(tiefe), tiefe, t, arg);
                    None
                }
                None => {
                    println!("{}⚠ Tiefe {}: Kein Argument mehr für Text '{}'",
                            "  ".repeat(tiefe), tiefe, t);
                    None
                }
            }
        }

        Element::Liste(items) => {
            println!("{}Tiefe {}: Durchsuche Liste mit {} Elementen",
                    "  ".repeat(tiefe), tiefe, items.len());

            for (i, item) in items.iter().enumerate() {
                println!("{}  Element {} von {}:", "  ".repeat(tiefe), i + 1, items.len());

                match (args.first(), item) {
                    (Some(arg), Element::Text(text)) if arg == text => {
                        println!("{}  ✓ Text '{}' matcht Argument '{}'",
                                "  ".repeat(tiefe), text, arg);
                        let mut pfad = vec![text.clone()];

                        if let Some(rest) = deep_match(&args[1..], item, tiefe + 1) {
                            pfad.extend(rest);
                            return Some(pfad);
                        }
                    }

                    (Some(_), Element::Liste(_)) => {
                        println!("{}  → Gehe in Liste tiefer", "  ".repeat(tiefe));
                        if let Some(pfad) = deep_match(args, item, tiefe + 1) {
                            return Some(pfad);
                        }
                    }

                    (Some(arg), Element::Text(text)) => {
                        println!("{}  ✗ Text '{}' matcht NICHT Argument '{}'",
                                "  ".repeat(tiefe), text, arg);
                    }

                    (None, _) => {
                        println!("{}  ⚠ Keine Argumente mehr", "  ".repeat(tiefe));
                    }
                }
            }

            println!("{}✗ Tiefe {}: Kein Match in dieser Liste",
                    "  ".repeat(tiefe), tiefe);
            None
        }
    }
}
