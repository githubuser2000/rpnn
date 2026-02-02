#[derive(Debug)]
pub enum Element {
    Text(String),
    Liste(Vec<Element>),
}

pub fn text(s: &str) -> Element {
    Element::Text(String::from(s))
}

pub fn list(elements: Vec<Element>) -> Element {
    Element::Liste(elements)
}

pub fn create_example_structure() -> Element {
    let _hauptParameter = vec!["zeilen1", "zeilen2", "spalten","ausgabe"];
    let _zeilenParameter  = vec!["von", "bis", "ohnevon", "ohnebis"];
    let _spaltenParameter = vec!["spaltennummer", "universum", "cluster", "galaxie", "kontinuum"];
    let _ausgabeParameter = vec!["nichtsleeres", "unfarbig"];

    list(vec![
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
    ])
}
