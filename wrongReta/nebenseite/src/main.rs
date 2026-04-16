// src/main.rs
use regex::Regex;
use lazy_static::lazy_static;

// Für die i18n Simulation
mod i18n {
    pub mod befehle2 {
        pub const V: &str = "v";
    }
}

// Globale Regex-Patterns (einmalig kompiliert)
lazy_static! {
    static ref ZEILEN_BRUCH_PATTERN: Regex = Regex::new(r"^(-?\d+/\d+)(-\d+/\d+)?((\+)(\d+/\d+))*$").unwrap();
    static ref ZEILEN_PATTERN: Regex = Regex::new(&format!("^({}?-?\\d+)(-\\d+)?((\\+)(\\d+))*$", i18n::befehle2::V)).unwrap();
    static ref OPTIMIZED_PATTERN: Regex = Regex::new(r"^(v?-?\d+)(-\d+)?((\+)(\d+))*$").unwrap();
}

// Implementierung von Lookahead: r",(?![^\[\]\{\}\(\)]*[\]\}\)])"
// Diese Regex sucht Kommas, die NICHT gefolgt werden von einem schließenden Bracket/Klammer ohne vorher ein öffnendes gesehen zu haben
fn split_with_lookahead(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    
    let mut i = 0;
    while i < len {
        if chars[i] == ',' {
            // Prüfe Lookahead: nach diesem Komma darf nicht direkt ein schließendes Bracket/Klammer kommen,
            // ohne vorher ein entsprechendes öffnendes zu haben
            if !has_unmatched_closing_bracket_ahead(&chars[i+1..]) {
                result.push(&text[start..i]);
                start = i + 1;
            }
        }
        i += 1;
    }
    
    // Letzten Teil hinzufügen
    result.push(&text[start..]);
    result
}

// Prüft ob nach aktueller Position ein schließendes Bracket/Klammer kommt,
// ohne dass vorher ein entsprechendes öffnendes im aktuellen Kontext war
fn has_unmatched_closing_bracket_ahead(chars: &[char]) -> bool {
    if chars.is_empty() {
        return false;
    }
    
    // Zähle die Balance für jede Bracket-Art
    let mut bracket_balance = 0;
    let mut brace_balance = 0;
    let mut paren_balance = 0;
    
    // Gehe durch alle verbleibenden Zeichen
    for &c in chars {
        match c {
            '[' => bracket_balance += 1,
            ']' => {
                if bracket_balance > 0 {
                    bracket_balance -= 1;
                } else {
                    // Unmatched closing bracket gefunden
                    return true;
                }
            }
            '{' => brace_balance += 1,
            '}' => {
                if brace_balance > 0 {
                    brace_balance -= 1;
                } else {
                    // Unmatched closing brace gefunden
                    return true;
                }
            }
            '(' => paren_balance += 1,
            ')' => {
                if paren_balance > 0 {
                    paren_balance -= 1;
                } else {
                    // Unmatched closing paren gefunden
                    return true;
                }
            }
            ',' => {
                // Wenn wir ein Komma erreichen und alle Balances sind 0,
                // dann war das vorherige Komma gültig
                if bracket_balance == 0 && brace_balance == 0 && paren_balance == 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    
    // Am Ende prüfen ob unmatchede schließende Brackets existieren
    false
}

// Alternative: Split mit vollständiger Bracket-Balance Berechnung
fn split_with_bracket_balance(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    
    let mut bracket_balance = 0;
    let mut brace_balance = 0;
    let mut paren_balance = 0;
    
    let chars: Vec<char> = text.chars().collect();
    
    for (i, &c) in chars.iter().enumerate() {
        match c {
            '[' => bracket_balance += 1,
            ']' => bracket_balance -= 1,
            '{' => brace_balance += 1,
            '}' => brace_balance -= 1,
            '(' => paren_balance += 1,
            ')' => paren_balance -= 1,
            ',' => {
                // Komma ist nur ein Trenner wenn alle Balances 0 sind
                if bracket_balance == 0 && brace_balance == 0 && paren_balance == 0 {
                    result.push(&text[start..i]);
                    start = i + 1;
                }
            }
            _ => {}
        }
    }
    
    // Letzten Teil hinzufügen
    result.push(&text[start..]);
    result
}

// Optimierte Version mit Lookahead-Simulation
fn split_with_lookahead_optimized(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    
    let mut bracket_depth = 0;
    let mut brace_depth = 0;
    let mut paren_depth = 0;
    
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    
    let mut i = 0;
    while i < len {
        match chars[i] {
            '[' => bracket_depth += 1,
            ']' => {
                if bracket_depth > 0 {
                    bracket_depth -= 1;
                }
            }
            '{' => brace_depth += 1,
            '}' => {
                if brace_depth > 0 {
                    brace_depth -= 1;
                }
            }
            '(' => paren_depth += 1,
            ')' => {
                if paren_depth > 0 {
                    paren_depth -= 1;
                }
            }
            ',' => {
                // Prüfe Lookahead: nach diesem Komma darf kein schließendes Bracket kommen,
                // ohne dass wir in einem Bracket-Kontext sind
                
                // Wenn wir nicht in einem Bracket-Kontext sind, ist das Komma gültig
                if bracket_depth == 0 && brace_depth == 0 && paren_depth == 0 {
                    result.push(&text[start..i]);
                    start = i + 1;
                } else {
                    // Wir sind in einem Bracket-Kontext, prüfe ob danach ein schließendes Bracket kommt
                    let mut j = i + 1;
                    let mut found_closing = false;
                    
                    while j < len {
                        match chars[j] {
                            ']' | '}' | ')' => {
                                found_closing = true;
                                break;
                            }
                            '[' | '{' | '(' | ',' => {
                                // Neues Bracket oder Komma bricht die Suche
                                break;
                            }
                            _ => {}
                        }
                        j += 1;
                    }
                    
                    // Wenn kein schließendes Bracket gefunden wurde, ist das Komma gültig
                    if !found_closing {
                        result.push(&text[start..i]);
                        start = i + 1;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    // Letzten Teil hinzufügen
    result.push(&text[start..]);
    result
}

// 1. isZeilenBruchAngabe_betweenKommas
pub fn is_zeilen_bruch_angabe_between_kommas(g: &str) -> bool {
    ZEILEN_BRUCH_PATTERN.is_match(g)
}

// 2. isZeilenBruchOrGanzZahlAngabe
pub fn is_zeilen_bruch_or_ganz_zahl_angabe(text: &str) -> bool {
    split_with_bracket_balance(text)
        .iter()
        .all(|g| is_zeilen_bruch_angabe_between_kommas(g) || is_zeilen_angabe_between_kommas(g))
}

// 3. isZeilenBruchAngabe
pub fn is_zeilen_bruch_angabe(text: &str) -> bool {
    let stext: Vec<&str> = split_with_bracket_balance(text);
    let any_at_all = stext.iter().any(|txt: &&str| !txt.is_empty());
    
    stext.iter().all(|&g| {
        is_zeilen_bruch_angabe_between_kommas(g) || (g.is_empty() && any_at_all)
    })
}

// 4. isZeilenAngabe
pub fn is_zeilen_angabe(text: &str) -> bool {
    let stext: Vec<&str> = split_with_bracket_balance(text);
    let any_at_all = stext.iter().any(|txt: &&str| !txt.is_empty());
    
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

// Hilfsfunktion: strAsGeneratorToListOfNumStrs
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

// Optimierte Version für isZeilenAngabe_betweenKommas
pub fn is_zeilen_angabe_between_kommas_optimized(g: &str) -> bool {
    // Prüfe zuerst das reguläre Muster
    if OPTIMIZED_PATTERN.is_match(g) {
        return true;
    }
    
    // Prüfe Generator-Notation
    if str_as_generator_to_list_of_num_strs(g).is_some() {
        return true;
    }
    
    // Prüfe ohne erstes Zeichen (falls es ein Sonderzeichen ist)
    if g.len() > 1 {
        if let Some(ch) = g.chars().next() {
            if !ch.is_ascii_digit() && ch != '-' && ch != 'v' {
                return str_as_generator_to_list_of_num_strs(&g[1..]).is_some();
            }
        }
    }
    
    false
}

// Test der Lookahead-Implementierung
fn test_lookahead_implementation() {
    println!("Testing Lookahead Implementation");
    println!("=================================");
    
    let test_cases = vec![
        ("1,2,3", vec!["1", "2", "3"]),
        ("(1,2),3", vec!["(1,2)", "3"]),
        ("[1,2],3", vec!["[1,2]", "3"]),
        ("{1,2},3", vec!["{1,2}", "3"]),
        ("(1,2),[3,4],5", vec!["(1,2)", "[3,4]", "5"]),
        ("a,b,c", vec!["a", "b", "c"]),
        ("", vec![""]),
        ("(a,b),c,(d,e)", vec!["(a,b)", "c", "(d,e)"]),
        ("[1,2,(3,4)],5", vec!["[1,2,(3,4)]", "5"]),
        ("1,(2,3),4", vec!["1", "(2,3)", "4"]),
    ];
    
    for (input, expected) in test_cases {
        let result1 = split_with_lookahead(input);
        let result2 = split_with_bracket_balance(input);
        let result3 = split_with_lookahead_optimized(input);
        
        println!("\nInput: '{}'", input);
        println!("Expected: {:?}", expected);
        println!("split_with_lookahead: {:?}", result1);
        println!("split_with_bracket_balance: {:?}", result2);
        println!("split_with_lookahead_optimized: {:?}", result3);
        
        // Alle sollten gleich sein
        assert_eq!(result1, expected, "split_with_lookahead failed");
        assert_eq!(result2, expected, "split_with_bracket_balance failed");
        assert_eq!(result3, expected, "split_with_lookahead_optimized failed");
    }
    
    println!("\n✅ All lookahead tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_split_functions() {
        assert_eq!(split_with_bracket_balance("1,2,3"), vec!["1", "2", "3"]);
        assert_eq!(split_with_bracket_balance("(1,2),3"), vec!["(1,2)", "3"]);
        assert_eq!(split_with_bracket_balance("[1,2],3"), vec!["[1,2]", "3"]);
        assert_eq!(split_with_bracket_balance("{1,2},3"), vec!["{1,2}", "3"]);
        assert_eq!(split_with_bracket_balance(""), vec![""]);
    }
    
    #[test]
    fn test_is_zeilen_bruch_angabe_between_kommas() {
        assert!(is_zeilen_bruch_angabe_between_kommas("1/2"));
        assert!(is_zeilen_bruch_angabe_between_kommas("-3/4"));
        assert!(is_zeilen_bruch_angabe_between_kommas("1/2-3/4"));
        assert!(!is_zeilen_bruch_angabe_between_kommas("abc"));
    }
    
    #[test]
    fn test_is_zeilen_angabe() {
        assert!(is_zeilen_angabe("1,2,3"));
        assert!(is_zeilen_angabe("1-10,20-30"));
        assert!(is_zeilen_angabe("(1,2,3),[4,5]"));
        assert!(!is_zeilen_angabe("abc,def"));
    }
}

// Hauptprogramm
fn main() {
    use std::env;
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--test-lookahead" {
        test_lookahead_implementation();
        return;
    }
    
    if args.len() > 1 {
        let input = &args[1];
        
        println!("Testing input: '{}'", input);
        println!("=================================");
        
        // Zeige alle Split-Varianten
        println!("split_with_lookahead: {:?}", split_with_lookahead(input));
        println!("split_with_bracket_balance: {:?}", split_with_bracket_balance(input));
        println!("split_with_lookahead_optimized: {:?}", split_with_lookahead_optimized(input));
        
        println!("\nValidation Results:");
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
            "(1,2),3",
            "[4,5],6",
            "{7,8},9",
            "abc",
            "",
        ];
        
        for test in test_cases {
            println!("\nTest: '{}'", test);
            println!("  split_with_bracket_balance: {:?}", split_with_bracket_balance(test));
            println!("  is_zeilen_angabe: {}", is_zeilen_angabe(test));
            println!("  is_zeilen_bruch_angabe: {}", is_zeilen_bruch_angabe(test));
        }
        
        // Lookahead-Tests
        println!("\n=================================");
        println!("Lookahead Algorithm Tests:");
        println!("'1,2,3' -> {:?}", split_with_lookahead("1,2,3"));
        println!("'(1,2),3' -> {:?}", split_with_lookahead("(1,2),3"));
        println!("'[1,2],3' -> {:?}", split_with_lookahead("[1,2],3"));
        // Hier müssen die geschweiften Klammern escaped werden:
        println!("'{{1,2}},3' -> {:?}", split_with_lookahead("{1,2},3"));
    }
}
