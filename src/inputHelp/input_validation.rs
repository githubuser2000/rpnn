use regex::Regex;
use lazy_static::lazy_static;
use crate::inputHelp::pattern_definitions::*;
use crate::inputHelp::generator_parser::str_as_generator_to_list_of_num_strs_alt;
use crate::inputHelp::bruch_validator::is_zeilen_bruch_angabe_between_kommas;
use crate::inputHelp::zeilen_angabe_validator::is_zeilen_angabe_between_kommas;

// 1. isZeilenBruchOrGanzZahlAngabe
pub fn is_zeilen_bruch_or_ganz_zahl_angabe(text: &str) -> bool {
    text.split(',')
        .all(|g| is_zeilen_bruch_angabe_between_kommas(g) || is_zeilen_angabe_between_kommas(g))
}

// 2. isZeilenBruchAngabe
pub fn is_zeilen_bruch_angabe(text: &str) -> bool {
    let stext: Vec<&str> = text.split(',').collect();
    let any_at_all = stext.iter().any(|txt| !txt.is_empty());
    
    stext.iter().all(|&g| {
        is_zeilen_bruch_angabe_between_kommas(g) || (g.is_empty() && any_at_all)
    })
}

// 3. isZeilenAngabe
pub fn is_zeilen_angabe(text: &str) -> bool {
    let stext: Vec<&str> = K_PATTERN.split(text).collect();
    let any_at_all = stext.iter().any(|txt| !txt.is_empty());
    
    stext.iter().all(|&g| {
        is_zeilen_angabe_between_kommas(g) || (g.is_empty() && any_at_all)
    })
}

// Optimierte Version für isZeilenAngabe_betweenKommas
pub fn is_zeilen_angabe_between_kommas_optimized(g: &str) -> bool {
    // Prüfe zuerst das reguläre Muster
    if OPTIMIZED_PATTERN.is_match(g) {
        return true;
    }
    
    // Prüfe Generator-Notation
    if str_as_generator_to_list_of_num_strs_alt(g).is_some() {
        return true;
    }
    
    // Prüfe ohne erstes Zeichen (falls es ein Sonderzeichen ist)
    if g.len() > 1 {
        if let Some(ch) = g.chars().next() {
            if !ch.is_ascii_digit() && ch != '-' && ch != 'v' {
                return str_as_generator_to_list_of_num_strs_alt(&g[1..]).is_some();
            }
        }
    }
    
    false
}
