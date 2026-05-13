use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MorphismKind {
    Parse,
    Resolve,
    Select,
    Derive,
    Generate,
    Format,
    Annotate,
    Enqueue,
    Dequeue,
    Dispatch,
    Glue,
    Render,
}

impl MorphismKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parse => "parse",
            Self::Resolve => "resolve",
            Self::Select => "select",
            Self::Derive => "derive",
            Self::Generate => "generate",
            Self::Format => "format",
            Self::Annotate => "annotate",
            Self::Enqueue => "enqueue",
            Self::Dequeue => "dequeue",
            Self::Dispatch => "dispatch",
            Self::Glue => "glue",
            Self::Render => "render",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MorphismEdge {
    pub name: String,
    pub source: String,
    pub target: String,
    pub kind: MorphismKind,
    pub owner: String,
}

impl MorphismEdge {
    pub fn new(
        name: impl Into<String>,
        source: impl Into<String>,
        target: impl Into<String>,
        kind: MorphismKind,
        owner: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
            target: target.into(),
            kind,
            owner: owner.into(),
        }
    }

    pub fn then(&self, next: &Self, composed_name: impl Into<String>) -> Option<Self> {
        if self.target != next.source {
            return None;
        }
        Some(Self {
            name: composed_name.into(),
            source: self.source.clone(),
            target: next.target.clone(),
            kind: next.kind,
            owner: format!("{} ∘ {}", next.owner, self.owner),
        })
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct MorphismGraph {
    pub edges: Vec<MorphismEdge>,
}

impl MorphismGraph {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn add(&mut self, edge: MorphismEdge) {
        self.edges.push(edge);
    }

    pub fn outgoing<'a>(&'a self, source: &'a str) -> impl Iterator<Item = &'a MorphismEdge> + 'a {
        self.edges.iter().filter(move |edge| edge.source == source)
    }

    pub fn compose_named(&self, first: &str, second: &str, name: &str) -> Option<MorphismEdge> {
        let first = self.edges.iter().find(|edge| edge.name == first)?;
        let second = self.edges.iter().find(|edge| edge.name == second)?;
        first.then(second, name)
    }
}
