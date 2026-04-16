#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_zeilen_bruch_angabe_between_kommas() {
        // Gültige Bruchangaben
        assert!(is_zeilen_bruch_angabe_between_kommas("1/2"));
        assert!(is_zeilen_bruch_angabe_between_kommas("-3/4"));
        assert!(is_zeilen_bruch_angabe_between_kommas("1/2-3/4"));
        assert!(is_zeilen_bruch_angabe_between_kommas("1/2+3/4"));
        assert!(is_zeilen_bruch_angabe_between_kommas("1/2-3/4+5/6"));
        
        // Ungültige Bruchangaben
        assert!(!is_zeilen_bruch_angabe_between_kommas("abc"));
        assert!(!is_zeilen_bruch_angabe_between_kommas("1/2/3"));
        assert!(!is_zeilen_bruch_angabe_between_kommas("1.5/2"));
    }
    
    #[test]
    fn test_is_zeilen_angabe_between_kommas() {
        // Gültige Zeilenangaben
        assert!(is_zeilen_angabe_between_kommas("123"));
        assert!(is_zeilen_angabe_between_kommas("-456"));
        assert!(is_zeilen_angabe_between_kommas("v123"));
        assert!(is_zeilen_angabe_between_kommas("1-10"));
        assert!(is_zeilen_angabe_between_kommas("1+2+3"));
        assert!(is_zeilen_angabe_between_kommas("1-10+5"));
        
        // Generator-Notationen
        assert!(is_zeilen_angabe_between_kommas("(1,2,3)"));
        assert!(is_zeilen_angabe_between_kommas("[4,5,6]"));
        assert!(is_zeilen_angabe_between_kommas("{7,8,9}"));
        
        // Ungültige Angaben
        assert!(!is_zeilen_angabe_between_kommas("abc"));
        assert!(!is_zeilen_angabe_between_kommas("1-"));
        assert!(!is_zeilen_angabe_between_kommas("+5"));
    }
    
    // ... (weitere Tests)
}
