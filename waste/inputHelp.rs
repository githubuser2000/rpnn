use regex::Regex;
use lazy_static::lazy_static;

// Für die i18n Simulation (ersetzt durch Konstanten)
mod i18n {
    pub mod befehle2 {
        pub const V: &str = "v";
    }
}

lazy_static! {
    static ref ZEILEN_BRUCH_PATTERN: Regex = Regex::new(r"^(-?\d+/\d+)(-\d+/\d+)?((\+)(\d+/\d+))*$").unwrap();
    static ref ZEILEN_PATTERN: Regex = Regex::new(&format!("^({}?-?\\d+)(-\\d+)?((\\+)(\\d+))*$", i18n::befehle2::V)).unwrap();
    static ref K_PATTERN: Regex = Regex::new(r",(?![^\[\]\{\}\(\)]*[\]\}\)])").unwrap();
    static ref OPTIMIZED_PATTERN: Regex = Regex::new(r"^(v?-?\d+)(-\d+)?((\+)(\d+))*$").unwrap();
}

// 1. isZeilenBruchAngabe_betweenKommas
pub fn is_zeilen_bruch_angabe_between_kommas(g: &str) -> bool {
    ZEILEN_BRUCH_PATTERN.is_match(g)
}

// 2. isZeilenBruchOrGanzZahlAngabe
pub fn is_zeilen_bruch_or_ganz_zahl_angabe(text: &str) -> bool {
    text.split(',')
        .all(|g| is_zeilen_bruch_angabe_between_kommas(g) || is_zeilen_angabe_between_kommas(g))
}

// 3. isZeilenBruchAngabe
pub fn is_zeilen_bruch_angabe(text: &str) -> bool {
    let stext: Vec<&str> = text.split(',').collect();
    let any_at_all = stext.iter().any(|txt| !txt.is_empty());
    
    stext.iter().all(|&g| {
        is_zeilen_bruch_angabe_between_kommas(g) || (g.is_empty() && any_at_all)
    })
}

// 4. isZeilenAngabe
pub fn is_zeilen_angabe(text: &str) -> bool {
    let stext: Vec<&str> = K_PATTERN.split(text).collect();
    let any_at_all = stext.iter().any(|txt| !txt.is_empty());
    
    stext.iter().all(|&g| {
        is_zeilen_angabe_between_kommas(g) || (g.is_empty() && any_at_all)
    })
}

// 5. isZeilenAngabe_betweenKommas
pub fn is_zeilen_angabe_between_kommas(g: &str) -> bool {
    ZEILEN_PATTERN.is_match(g) || 
    str_as_generator_to_list_of_num_strs(g).is_some() ||
    (g.len() > 1 && str_as_generator_to_list_of_num_strs(&g[1..]).is_some())
}

// Hilfsfunktion: strAsGeneratorToListOfNumStrs (vereinfacht für Rust)
pub fn str_as_generator_to_list_of_num_strs(text: &str) -> Option<Vec<String>> {
    if text.is_empty() {
        return None;
    }
    
    let trimmed = text.trim();
    
    // Prüfe auf (a,b,c) oder [a,b,c] oder {a,b,c} Format
    if (trimmed.starts_with('(') && trimmed.ends_with(')')) ||
       (trimmed.starts_with('[') && trimmed.ends_with(']')) ||
       (trimmed.starts_with('{') && trimmed.ends_with('}')) {
        
        let inner = &trimmed[1..trimmed.len()-1];
        let numbers: Result<Vec<i32>, _> = inner.split(',')
            .map(|s| s.trim().parse::<i32>())
            .collect();
            
        match numbers {
            Ok(nums) => {
                let strings: Vec<String> = nums.iter()
                    .map(|n| n.to_string())
                    .collect();
                Some(strings)
            }
            Err(_) => None,
        }
    } else {
        None
    }
}

// Alternative Implementierung mit besserer Fehlerbehandlung
pub fn str_as_generator_to_list_of_num_strs_alt(text: &str) -> Option<Vec<i32>> {
    let trimmed = text.trim();
    
    // Konvertiere (a,b,c) zu [a,b,c]
    let processed = if trimmed.starts_with('(') && trimmed.ends_with(')') {
        format!("[{}]", &trimmed[1..trimmed.len()-1])
    } else {
        trimmed.to_string()
    };
    
    // Prüfe auf Array/Set-Format
    if (processed.starts_with('[') && processed.ends_with(']')) ||
       (processed.starts_with('{') && processed.ends_with('}')) {
        
        let inner = &processed[1..processed.len()-1];
        if inner.trim().is_empty() {
            return Some(Vec::new()); // Leere Menge/Liste
        }
        
        let numbers: Result<Vec<i32>, _> = inner.split(',')
            .map(|s| s.trim().parse::<i32>())
            .collect();
            
        numbers.ok()
    } else {
        None
    }
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
    
    #[test]
    fn test_is_zeilen_bruch_or_ganz_zahl_angabe() {
        assert!(is_zeilen_bruch_or_ganz_zahl_angabe("1/2,3/4"));
        assert!(is_zeilen_bruch_or_ganz_zahl_angabe("1-10,20-30"));
        assert!(is_zeilen_bruch_or_ganz_zahl_angabe("1/2,10-20"));
        assert!(is_zeilen_bruch_or_ganz_zahl_angabe("(1,2,3),[4,5]"));
        
        assert!(!is_zeilen_bruch_or_ganz_zahl_angabe("abc,def"));
        assert!(!is_zeilen_bruch_or_ganz_zahl_angabe("1/2/3,4-5"));
    }
    
    #[test]
    fn test_is_zeilen_bruch_angabe() {
        assert!(is_zeilen_bruch_angabe("1/2,3/4"));
        assert!(is_zeilen_bruch_angabe("1/2-3/4,5/6+7/8"));
        assert!(is_zeilen_bruch_angabe("1/2,")); // Leere mit any_at_all
        assert!(is_zeilen_bruch_angabe(",1/2")); // Leere mit any_at_all
        
        assert!(!is_zeilen_bruch_angabe("1/2,abc"));
        assert!(!is_zeilen_bruch_angabe("")); // Keine Angabe
    }
    
    #[test]
    fn test_is_zeilen_angabe() {
        assert!(is_zeilen_angabe("1,2,3"));
        assert!(is_zeilen_angabe("1-10,20-30"));
        assert!(is_zeilen_angabe("(1,2,3),[4,5]"));
        assert!(is_zeilen_angabe("v1-10+v5"));
        assert!(is_zeilen_angabe("1,2,")); // Leere mit any_at_all
        assert!(is_zeilen_angabe(",1,2")); // Leere mit any_at_all
        
        assert!(!is_zeilen_angabe("abc,def"));
        assert!(!is_zeilen_angabe("")); // Keine Angabe
    }
    
    #[test]
    fn test_str_as_generator_to_list_of_num_strs() {
        assert_eq!(
            str_as_generator_to_list_of_num_strs("(1,2,3)"),
            Some(vec!["1".to_string(), "2".to_string(), "3".to_string()])
        );
        
        assert_eq!(
            str_as_generator_to_list_of_num_strs("[4,5,6]"),
            Some(vec!["4".to_string(), "5".to_string(), "6".to_string()])
        );
        
        assert_eq!(
            str_as_generator_to_list_of_num_strs("{7,8,9}"),
            Some(vec!["7".to_string(), "8".to_string(), "9".to_string()])
        );
        
        assert_eq!(
            str_as_generator_to_list_of_num_strs("(1, 2, 3)"), // Mit Leerzeichen
            Some(vec!["1".to_string(), "2".to_string(), "3".to_string()])
        );
        
        assert_eq!(str_as_generator_to_list_of_num_strs(""), None);
        assert_eq!(str_as_generator_to_list_of_num_strs("abc"), None);
        assert_eq!(str_as_generator_to_list_of_num_strs("(1,abc)"), None);
    }
    
    #[test]
    fn test_str_as_generator_to_list_of_num_strs_alt() {
        assert_eq!(
            str_as_generator_to_list_of_num_strs_alt("(1,2,3)"),
            Some(vec![1, 2, 3])
        );
        
        assert_eq!(
            str_as_generator_to_list_of_num_strs_alt("[4,5,6]"),
            Some(vec![4, 5, 6])
        );
        
        assert_eq!(
            str_as_generator_to_list_of_num_strs_alt("{7,8,9}"),
            Some(vec![7, 8, 9])
        );
        
        assert_eq!(
            str_as_generator_to_list_of_num_strs_alt("[]"),
            Some(vec![]) // Leere Liste
        );
        
        assert_eq!(
            str_as_generator_to_list_of_num_strs_alt("{}"),
            Some(vec![]) // Leere Menge
        );
        
        assert_eq!(str_as_generator_to_list_of_num_strs_alt(""), None);
        assert_eq!(str_as_generator_to_list_of_num_strs_alt("(1,abc)"), None);
    }
}

// Beispiel: Hauptprogramm mit CLI
fn main() {
    use std::env;
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        let input = &args[1];
        
        println!("Testing input: '{}'", input);
        println!("=================================");
        
        println!("1. is_zeilen_bruch_angabe_between_kommas: {}",
            is_zeilen_bruch_angabe_between_kommas(input));
        
        println!("2. is_zeilen_bruch_or_ganz_zahl_angabe: {}",
            is_zeilen_bruch_or_ganz_zahl_angabe(input));
        
        println!("3. is_zeilen_bruch_angabe: {}",
            is_zeilen_bruch_angabe(input));
        
        println!("4. is_zeilen_angabe: {}",
            is_zeilen_angabe(input));
        
        println!("5. is_zeilen_angabe_between_kommas: {}",
            is_zeilen_angabe_between_kommas(input));
        
        println!("5a. is_zeilen_angabe_between_kommas_optimized: {}",
            is_zeilen_angabe_between_kommas_optimized(input));
        
        if let Some(numbers) = str_as_generator_to_list_of_num_strs_alt(input) {
            println!("Generator notation detected: {:?}", numbers);
        }
    } else {
        // Testfälle ausführen
        println!("Running test cases...");
        println!("=================================");
        
        let test_cases = vec![
            "1/2",
            "1-10",
            "(1,2,3)",
            "v5-10+v3",
            "1/2,3/4",
            "1-10,20-30",
            "abc",
            "",
        ];
        
        for test in test_cases {
            println!("\nTest: '{}'", test);
            println!("  is_zeilen_angabe: {}", is_zeilen_angabe(test));
            println!("  is_zeilen_bruch_angabe: {}", is_zeilen_bruch_angabe(test));
        }
        
        println!("\n=================================");
        println!("Beispiel für Generator-Notationen:");
        
        let generator_cases = vec![
            "(1,2,3,4,5)",
            "[10,20,30]",
            "{5,15,25}",
            "(1, 2, 3)", // mit Leerzeichen
        ];
        
        for case in generator_cases {
            if let Some(nums) = str_as_generator_to_list_of_num_strs_alt(case) {
                println!("  {} -> {:?}", case, nums);
            }
        }
    }
}
