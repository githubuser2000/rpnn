#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ST {
    sternPolygon,
    universum,
    galaxie,
    gleichfoermigesPolygon,
    gebrRat,
    keinParaOdMetaP,
}

impl ST {
    pub const fn py_name(self) -> &'static str {
        match self {
            Self::sternPolygon => "sternPolygon",
            Self::universum => "universum",
            Self::galaxie => "galaxie",
            Self::gleichfoermigesPolygon => "gleichfoermigesPolygon",
            Self::gebrRat => "gebrRat",
            Self::keinParaOdMetaP => "keinParaOdMetaP",
        }
    }

    pub fn from_py_name(value: &str) -> Option<Self> {
        match value.trim() {
            "sternPolygon" => Some(Self::sternPolygon),
            "universum" => Some(Self::universum),
            "galaxie" => Some(Self::galaxie),
            "gleichfoermigesPolygon" => Some(Self::gleichfoermigesPolygon),
            "gebrRat" => Some(Self::gebrRat),
            "keinParaOdMetaP" => Some(Self::keinParaOdMetaP),
            _ => None,
        }
    }

    pub fn html_class(self) -> String {
        format!("p4_{}", self.py_name())
    }
}

impl fmt::Display for ST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.py_name())
    }
}
