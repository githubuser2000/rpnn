#[derive(Debug)]
enum Element {
    Text(String),
    Liste(Vec<Element>),
}

fn main() {
    let meine_liste = Element::Liste(vec![
        Element::Text(String::from("Hallo Welt")),
        Element::Liste(vec![
            Element::Text(String::from("Verschachtelt")),
            Element::Liste(vec![Element::Text(String::from("Noch tiefer"))]),
        ]),
    ]);

    println!("Programm erfolgreich gestartet:");
    print_recursive(&meine_liste, 0);
}

fn print_recursive(el: &Element, tiefe: usize) {
    let einrückung = "  ".repeat(tiefe);
    match el {
        Element::Text(t) => println!("{}- {}", einrückung, t),
        Element::Liste(l) => {
            println!("{}[Liste]:", einrückung);
            for kind in l {
                print_recursive(kind, tiefe + 1);
            }
        }
    }
}

