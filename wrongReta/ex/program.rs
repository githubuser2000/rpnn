/// Das Enum definiert unsere rekursive Datenstruktur.
/// Ein Element ist entweder ein String (Text) oder eine Liste von weiteren Elementen.
#[derive(Debug)]
enum Element {
    Text(String),
    Liste(Vec<Element>),
}

fn main() {
    // Erstellung einer tief verschachtelten Struktur:
    // ["Ebene 1", ["Ebene 2", ["Ebene 3"]], "Ende"]
    let meine_liste = Element::Liste(vec![
        Element::Text(String::from("Hallo Welt")),
        Element::Liste(vec![
            Element::Text(String::from("Ich bin verschachtelt")),
            Element::Liste(vec![
                Element::Text(String::from("Ich bin noch tiefer")),
            ]),
        ]),
        Element::Text(String::from("Rust macht Spaß!")),
    ]);

    println!("Ausgabe der Struktur:");
    print_recursive(&meine_liste, 0);
}

/// Eine einfache Funktion, um die verschachtelte Liste formatiert auszugeben.
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

