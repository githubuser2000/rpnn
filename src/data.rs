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
