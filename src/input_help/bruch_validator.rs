use crate::input_help::pattern_definitions::ZEILEN_BRUCH_PATTERN;

// 1. isZeilenBruchAngabe_betweenKommas
pub fn is_zeilen_bruch_angabe_between_kommas(g: &str) -> bool {
    ZEILEN_BRUCH_PATTERN.is_match(g)
}
