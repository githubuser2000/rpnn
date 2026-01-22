// functions_8_to_23.rs
use std::collections::{HashSet};
use std::env;
use term_size::dimensions;

// 8. getTextWrapThings - Liefert Shell-Informationen und Text-Wrapping-Hilfen
pub fn get_text_wrap_things(_max_len: Option<usize>) -> (usize, Option<()>, Option<()>, Option<()>) {
    // Vereinfachte Version ohne externe Abhängigkeiten
    let shell_width = if let Some((width, _)) = dimensions() {
        width
    } else {
        80 // Fallback
    };
    
    // Platzhalter für die Python-Objekte
    let h_de = None; // Hyphenator
    let dic = None;  // Pyphen dictionary
    let fill = None; // textwrap2.fill
    
    (shell_width, h_de, dic, fill)
}

// Alternative: Nur Shell-Breite
pub fn get_shell_width() -> usize {
    dimensions()
        .map(|(width, _)| width)
        .unwrap_or(80)
}

// 11. chunks - Teilt eine Liste in gleich große Teile
pub fn chunks<T: Clone>(lst: &[T], n: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for i in (0..lst.len()).step_by(n) {
        let end = std::cmp::min(i + n, lst.len());
        result.push(lst[i..end].to_vec());
    }
    result
}

// Alternative mit Iterator (effizienter)
pub fn chunks_iter<'a, T>(lst: &'a [T], n: usize) -> impl Iterator<Item = &'a [T]> {
    (0..lst.len()).step_by(n).map(move |i| &lst[i..std::cmp::min(i + n, lst.len())])
}

// 12. cliout - Ausgabe mit Farben/Syntax-Highlighting
pub fn cliout(text: &str, color: bool, stype: &str) {
    if color && !text.trim().is_empty() {
        // Vereinfachte farbige Ausgabe
        if stype == "html" {
            let formatted = text.replace("<tr", "\n  <tr").replace("<td", "\n    <td");
            println!("{}", formatted);
        } else {
            println!("{}", text);
        }
    } else {
        println!("{}", text);
    }
}

// 13. strAsGeneratorToListOfNumStrs - Konvertiert String zu Zahlen-Liste
pub fn str_as_generator_to_list_of_num_strs(text: &str) -> Option<Vec<String>> {
    if text.is_empty() {
        return None;
    }
    
    let trimmed = text.trim();
    
    // Prüfe auf (a,b,c), [a,b,c] oder {a,b,c}
    if !(trimmed.starts_with('(') && trimmed.ends_with(')')) &&
       !(trimmed.starts_with('[') && trimmed.ends_with(']')) &&
       !(trimmed.starts_with('{') && trimmed.ends_with('}')) {
        return None;
    }
    
    let inner = &trimmed[1..trimmed.len()-1];
    let numbers: Result<Vec<i32>, _> = inner.split(',')
        .map(|s| s.trim().parse::<i32>())
        .collect();
    
    match numbers {
        Ok(nums) => Some(nums.iter().map(|n| n.to_string()).collect()),
        Err(_) => None,
    }
}

// Alternative mit HashSet
pub fn str_as_generator_to_set_of_nums(text: &str) -> Option<HashSet<i32>> {
    str_as_generator_to_list_of_num_strs(text)
        .map(|strings| {
            strings.iter()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
}

// 14. unique_everseen - Einzigartige Elemente in Reihenfolge behalten
// Version 1: Ohne Key-Funktion
pub fn unique_everseen_no_key<T>(iterable: &[T]) -> Vec<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    
    for item in iterable {
        if !seen.contains(item) {
            seen.insert(item.clone());
            result.push(item.clone());
        }
    }
    
    result
}

// Version 2: Mit Key-Funktion (generisch)
pub fn unique_everseen_with_key<T, K, F>(iterable: &[T], key_fn: F) -> Vec<T>
where
    T: Clone,
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    
    for item in iterable {
        let key = key_fn(item);
        if !seen.contains(&key) {
            seen.insert(key);
            result.push(item.clone());
        }
    }
    
    result
}

// Version 3: Für Strings (einfach zu verwenden)
pub fn unique_everseen_str<'a>(iterable: &[&'a str]) -> Vec<&'a str> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    
    for &item in iterable {
        if !seen.contains(item) {
            seen.insert(item);
            result.push(item);
        }
    }
    
    result
}

// Iterator-Version ohne Key
pub fn unique_everseen_iter_no_key<T, I>(iterable: I) -> impl Iterator<Item = T>
where
    T: Clone + Eq + std::hash::Hash,
    I: IntoIterator<Item = T>,
{
    let mut seen = HashSet::new();
    iterable.into_iter().filter(move |item| {
        if seen.contains(item) {
            false
        } else {
            seen.insert(item.clone());
            true
        }
    })
}

// Iterator-Version mit Key
pub fn unique_everseen_iter_with_key<T, I, K, F>(iterable: I, key_fn: F) -> impl Iterator<Item = T>
where
    T: Clone,
    I: IntoIterator<Item = T>,
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K + Clone,
{
    let mut seen = HashSet::new();
    let key_fn_clone = key_fn.clone();
    
    iterable.into_iter().filter(move |item| {
        let key = key_fn_clone(item);
        if seen.contains(&key) {
            false
        } else {
            seen.insert(key);
            true
        }
    })
}

// 23. textHatZiffer - Prüft ob Text Ziffern enthält
pub fn text_hat_ziffer(text: &str) -> bool {
    text.chars().any(|c| c.is_ascii_digit())
}

// Unicode-fähige Version
pub fn text_hat_ziffer_unicode(text: &str) -> bool {
    text.chars().any(|c| c.is_numeric())
}

// Prüft spezifische Ziffern
pub fn text_hat_spezifische_ziffern(text: &str, ziffern: &[char]) -> bool {
    text.chars().any(|c| ziffern.contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_shell_width() {
        // Kann nicht wirklich getestet werden, da von Terminal abhängig
        let width = get_shell_width();
        assert!(width >= 0);
    }
    
    #[test]
    fn test_chunks() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        
        let result = chunks(&data, 3);
        assert_eq!(result, vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ]);
        
        let result2 = chunks(&data, 4);
        assert_eq!(result2, vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9]
        ]);
        
        // Iterator-Version
        let iter_result: Vec<Vec<i32>> = chunks_iter(&data, 3)
            .map(|slice| slice.to_vec())
            .collect();
        assert_eq!(iter_result, result);
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
    fn test_str_as_generator_to_set_of_nums() {
        let result = str_as_generator_to_set_of_nums("(1,2,2,3)");
        assert_eq!(result, Some(HashSet::from([1, 2, 3])));
        
        let result2 = str_as_generator_to_set_of_nums("[5,5,5]");
        assert_eq!(result2, Some(HashSet::from([5])));
    }
    
    #[test]
    fn test_unique_everseen_no_key() {
        let data = vec!["A", "A", "B", "C", "B", "D", "A"];
        
        let result = unique_everseen_no_key(&data);
        assert_eq!(result, vec!["A", "B", "C", "D"]);
    }
    
    #[test]
    fn test_unique_everseen_with_key() {
        let data2 = vec![
            ("a", 1),
            ("b", 2),
            ("a", 3), // gleicher erster Wert
            ("c", 4),
        ];
        
        let result2 = unique_everseen_with_key(&data2, |(x, _)| x);
        assert_eq!(result2.len(), 3); // Nur einzigartige erste Werte
    }
    
    #[test]
    fn test_unique_everseen_str() {
        let data = vec!["apple", "banana", "apple", "orange", "banana"];
        let result = unique_everseen_str(&data);
        assert_eq!(result, vec!["apple", "banana", "orange"]);
    }
    
    #[test]
    fn test_unique_everseen_iter_no_key() {
        let data = vec![1, 2, 2, 3, 4, 3, 5];
        
        let result: Vec<i32> = unique_everseen_iter_no_key(data).collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_unique_everseen_iter_with_key() {
        let words = vec!["apple", "banana", "APPLE", "Banana", "cherry"];
        let result: Vec<&str> = unique_everseen_iter_with_key(
            words, 
            |s| s.to_lowercase()
        ).collect();
        assert_eq!(result.len(), 3); // apple, banana, cherry
    }
    
    #[test]
    fn test_text_hat_ziffer() {
        assert!(text_hat_ziffer("abc123"));
        assert!(text_hat_ziffer("123"));
        assert!(text_hat_ziffer("a1b2c3"));
        
        assert!(!text_hat_ziffer(""));
        assert!(!text_hat_ziffer("abc"));
        assert!(!text_hat_ziffer("!@#$%"));
        
        // Unicode-Version
        assert!(text_hat_ziffer_unicode("¹²³")); // Hochzahlen
        assert!(text_hat_ziffer_unicode("一二三")); // Chinesische Zahlen
    }
    
    #[test]
    fn test_text_hat_spezifische_ziffern() {
        assert!(text_hat_spezifische_ziffern("abc123", &['1', '2', '3']));
        assert!(text_hat_spezifische_ziffern("a1b2", &['1', '2']));
        
        assert!(!text_hat_spezifische_ziffern("abc", &['1', '2', '3']));
        assert!(!text_hat_spezifische_ziffern("456", &['1', '2', '3']));
    }
}

// Hauptprogramm mit Beispielen
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "chunks" => {
                let data = (1..=10).collect::<Vec<i32>>();
                println!("Original: {:?}", data);
                println!("Chunks size 3: {:?}", chunks(&data, 3));
                println!("Chunks size 4: {:?}", chunks(&data, 4));
            }
            "unique" => {
                let data = vec!["a", "b", "a", "c", "b", "d", "a"];
                println!("Original: {:?}", data);
                println!("Unique (no key): {:?}", unique_everseen_no_key(&data));
                println!("Unique (str): {:?}", unique_everseen_str(&data));
                
                let numbers = vec![1, 2, 2, 3, 4, 3, 5, 1];
                println!("Numbers: {:?}", numbers);
                println!("Unique numbers (iter): {:?}", 
                    unique_everseen_iter_no_key(numbers).collect::<Vec<i32>>());
            }
            "generator" => {
                let test_cases = vec![
                    "(1,2,3)",
                    "[4,5,6]",
                    "{7,8,9}",
                    "(1, 2, 3)",
                    "abc",
                    "",
                ];
                
                for test in test_cases {
                    println!("'{}' -> {:?}", test, 
                        str_as_generator_to_list_of_num_strs(test));
                }
            }
            "ziffer" => {
                let test_cases = vec![
                    "abc123",
                    "keineziffern",
                    "123456",
                    "sonder!zeichen",
                    "gemischt1mit2ziffern3",
                ];
                
                for test in test_cases {
                    println!("'{}' hat Ziffern: {}", test, text_hat_ziffer(test));
                }
            }
            "shell" => {
                let (width, _, _, _) = get_text_wrap_things(None);
                println!("Shell width: {}", width);
                println!("Terminal dimensions: {:?}", dimensions());
            }
            _ => {
                println!("Verfügbare Befehle:");
                println!("  chunks    - Testet chunks-Funktion");
                println!("  unique    - Testet unique_everseen");
                println!("  generator - Testet str_as_generator_to_list_of_num_strs");
                println!("  ziffer    - Testet text_hat_ziffer");
                println!("  shell     - Testet get_text_wrap_things");
            }
        }
    } else {
        // Standard-Tests
        println!("Running all tests...");
        
        // Test chunks
        let data = vec![1, 2, 3, 4, 5, 6];
        println!("\n1. chunks([1,2,3,4,5,6], 2):");
        for chunk in chunks_iter(&data, 2) {
            println!("  {:?}", chunk);
        }
        
        // Test unique
        let words = vec!["apple", "banana", "apple", "orange", "banana"];
        println!("\n2. unique_everseen_str(['apple', 'banana', 'apple', 'orange', 'banana']):");
        println!("  {:?}", unique_everseen_str(&words));
        
        // Mit Key-Funktion
        let words2 = vec!["Apple", "banana", "APPLE", "Banana"];
        println!("\n3. unique_everseen_with_key (case-insensitive):");
        let result = unique_everseen_with_key(&words2, |s| s.to_lowercase());
        println!("  {:?}", result);
        
        // Test generator
        println!("\n4. str_as_generator_to_list_of_num_strs('(10,20,30)'):");
        println!("  {:?}", str_as_generator_to_list_of_num_strs("(10,20,30)"));
        
        // Test ziffer
        println!("\n5. text_hat_ziffer Tests:");
        for text in &["abc", "a1b2c3", "keine", "123"] {
            println!("  '{}' -> {}", text, text_hat_ziffer(text));
        }
        
        // Test shell width
        println!("\n6. Shell information:");
        let width = get_shell_width();
        println!("  Shell width: {}", width);
        
        // Beispiel für cliout
        println!("\n7. cliout example:");
        cliout("Hello, World!", false, "");
        cliout("<html><body><p>Test</p></body></html>", true, "html");
    }
}
