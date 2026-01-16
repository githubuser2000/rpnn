#[derive(Debug)]
enum Element {
    Text(String),
    Liste(Vec<Element>),
}

fn main() {
    // Erstellung der Struktur: [["zeilen", ["nummer"]], ["spalten", ["nummer"]]]
    let meine_liste = Element::Liste(vec![
        // Erste Haupt-Gruppe: ["zeilen", ["nummer"]]
        Element::Liste(vec![
            Element::Text(String::from("zeilen")),
            Element::Liste(vec![
                Element::Text(String::from("nummer"))
            ]),
        ]),
        // Zweite Haupt-Gruppe: ["spalten", ["nummer"]]
        Element::Liste(vec![
            Element::Text(String::from("spalten")),
            Element::Liste(vec![
                Element::Text(String::from("nummer"))
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

