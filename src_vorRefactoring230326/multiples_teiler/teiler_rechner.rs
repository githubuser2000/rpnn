use std::collections::HashSet;
use crate::multiples_teiler::faktor_finder::{multiples, find_all_divisors};
use crate::multiples_teiler::bereichs_verarbeitung::simulate_bereich_to_numbers2;

// 21: teiler - Findet alle Teiler einer oder mehrerer Zahlen
pub fn teiler(zahlen_bereichs_angabe: &str) -> (Vec<String>, HashSet<i64>) {
    let zahlen_bereich_menge = simulate_bereich_to_numbers2(zahlen_bereichs_angabe);
    
    let mut zahlen_wbereich_menge = HashSet::new();
    
    for &each1 in &zahlen_bereich_menge {
        if each1 > 0 {
            for (faktor1, faktor2) in multiples(each1, true) {
                zahlen_wbereich_menge.insert(faktor1);
                zahlen_wbereich_menge.insert(faktor2);
            }
        }
    }
    
    // 1 entfernen, wenn nicht die einzige Zahl
    if zahlen_wbereich_menge.len() > 1 {
        zahlen_wbereich_menge.remove(&1);
    }
    
    // Konvertiere zu String-Liste
    let mut zahlen_wbereich_string_liste: Vec<String> = 
        zahlen_wbereich_menge.iter()
            .map(|n| n.to_string())
            .collect();
    
    // Sortiere die String-Liste
    zahlen_wbereich_string_liste.sort_by(|a, b| {
        a.parse::<i64>().unwrap().cmp(&b.parse::<i64>().unwrap())
    });
    
    (zahlen_wbereich_string_liste, zahlen_wbereich_menge)
}

// Verbesserte teiler-Funktion mit Bereichs-Unterstützung
pub fn teiler_enhanced(zahlen_bereichs_angabe: &str) -> (Vec<String>, HashSet<i64>) {
    let numbers = simulate_bereich_to_numbers2(zahlen_bereichs_angabe);
    let mut all_divisors = HashSet::new();
    
    for &n in &numbers {
        let divisors = find_all_divisors(n);
        all_divisors.extend(divisors);
    }
    
    // 1 entfernen, wenn nicht die einzige Zahl
    if all_divisors.len() > 1 {
        all_divisors.remove(&1);
    }
    
    // Konvertiere und sortiere
    let mut string_list: Vec<String> = 
        all_divisors.iter()
            .map(|n| n.to_string())
            .collect();
    
    string_list.sort_by(|a, b| {
        a.parse::<i64>().unwrap().cmp(&b.parse::<i64>().unwrap())
    });
    
    (string_list, all_divisors)
}

// Einfache Teiler-Funktion für eine einzelne Zahl
pub fn teiler_einzeln(n: i64) -> Vec<i64> {
    let mut divisors = find_all_divisors(n);
    
    // 1 entfernen, wenn es andere Teiler gibt
    if divisors.len() > 1 {
        divisors.retain(|&x| x != 1);
    }
    
    divisors
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    
    #[test]
    fn test_teiler() {
        let (strings, set) = teiler("12,8");
        
        // Erwartete Teiler von 12: 2,3,4,6,12
        // Erwartete Teiler von 8: 2,4,8
        // Kombiniert: 2,3,4,6,8,12 (ohne 1)
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&6));
        assert!(set.contains(&8));
        assert!(set.contains(&12));
        assert!(!set.contains(&1));
        
        // Strings sollten sortiert sein
        assert_eq!(strings, vec!["2", "3", "4", "6", "8", "12"]);
    }
    
    #[test]
    fn test_teiler_single_number() {
        let (strings, set) = teiler("17");
        assert_eq!(set, HashSet::from([17]));
        assert_eq!(strings, vec!["17"]);
    }
    
    #[test]
    fn test_teiler_enhanced() {
        let (strings, set) = teiler_enhanced("12,8");
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&6));
        assert!(set.contains(&8));
        assert!(set.contains(&12));
        assert!(!set.contains(&1));
        assert_eq!(strings, vec!["2", "3", "4", "6", "8", "12"]);
    }
    
    #[test]
    fn test_teiler_einzeln() {
        assert_eq!(teiler_einzeln(12), vec![2, 3, 4, 6, 12]);
        assert_eq!(teiler_einzeln(17), vec![17]);
        assert_eq!(teiler_einzeln(1), vec![1]);
    }
}
