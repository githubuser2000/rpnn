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
}
