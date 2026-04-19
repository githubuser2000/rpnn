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
    keinPolygon,
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
            Self::keinPolygon => "keinPolygon",
        }
    }

    pub const fn py_value(self) -> i64 {
        match self {
            Self::sternPolygon => 0,
            Self::gleichfoermigesPolygon => 1,
            Self::keinPolygon => 2,
            Self::galaxie => 3,
            Self::universum => 4,
            Self::keinParaOdMetaP => 5,
            Self::gebrRat => 6,
        }
    }

    pub fn from_py_value(value: i64) -> Option<Self> {
        match value {
            0 => Some(Self::sternPolygon),
            1 => Some(Self::gleichfoermigesPolygon),
            2 => Some(Self::keinPolygon),
            3 => Some(Self::galaxie),
            4 => Some(Self::universum),
            5 => Some(Self::keinParaOdMetaP),
            6 => Some(Self::gebrRat),
            _ => None,
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
            "keinPolygon" => Some(Self::keinPolygon),
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

#[cfg(test)]
mod tests {
    use super::ST;

    #[test]
    fn st_python_enum_values_match_lib4tables_enum_py() {
        assert_eq!(ST::sternPolygon.py_value(), 0);
        assert_eq!(ST::gleichfoermigesPolygon.py_value(), 1);
        assert_eq!(ST::keinPolygon.py_value(), 2);
        assert_eq!(ST::galaxie.py_value(), 3);
        assert_eq!(ST::universum.py_value(), 4);
        assert_eq!(ST::keinParaOdMetaP.py_value(), 5);
        assert_eq!(ST::gebrRat.py_value(), 6);
        assert_eq!(ST::from_py_value(2), Some(ST::keinPolygon));
        assert_eq!(ST::from_py_name("keinPolygon"), Some(ST::keinPolygon));
    }
}
