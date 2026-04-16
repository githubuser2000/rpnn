use crate::input_help::pattern_definitions::ZEILEN_PATTERN;
use crate::input_help::generator_parser::str_as_generator_to_list_of_num_strs;

// 5. isZeilenAngabe_betweenKommas
pub fn is_zeilen_angabe_between_kommas(g: &str) -> bool {
    ZEILEN_PATTERN.is_match(g) || 
    str_as_generator_to_list_of_num_strs(g).is_some() ||
    (g.len() > 1 && str_as_generator_to_list_of_num_strs(&g[1..]).is_some())
}
