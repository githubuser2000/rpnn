use regex::Regex;
use lazy_static::lazy_static;

// Für die i18n Simulation (ersetzt durch Konstanten)
mod i18n {
    pub mod befehle2 {
        pub const V: &str = "v";
    }
}

lazy_static! {
    pub static ref ZEILEN_BRUCH_PATTERN: Regex = Regex::new(r"^(-?\d+/\d+)(-\d+/\d+)?((\+)(\d+/\d+))*$").unwrap();
    pub static ref ZEILEN_PATTERN: Regex = Regex::new(&format!("^({}?-?\\d+)(-\\d+)?((\\+)(\\d+))*$", i18n::befehle2::V)).unwrap();
    pub static ref K_PATTERN: Regex = Regex::new(r",(?![^\[\]\{\}\(\)]*[\]\}\)])").unwrap();
    pub static ref OPTIMIZED_PATTERN: Regex = Regex::new(r"^(v?-?\d+)(-\d+)?((\+)(\d+))*$").unwrap();
}
