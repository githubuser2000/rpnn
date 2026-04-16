use std::collections::{HashMap, HashSet};

// 20: multiples - Findet alle Faktorpaare für eine Zahl
pub fn multiples(a: i64, mul1: bool) -> Vec<(i64, i64)> {
    if a <= 0 {
        return Vec::new();
    }
    
    let mut menge = HashSet::new();
    let limit = (a as f64).sqrt().floor() as i64 + 1;
    
    for b in 2..limit {
        if a % b == 0 {
            let c = a / b;
            menge.insert((c, b));
        }
    }
    
    let mut result: Vec<(i64, i64)> = menge.into_iter().collect();
    
    if mul1 {
        result.push((a, 1));
    }
    
    // Sortieren für konsistente Ausgabe
    result.sort();
    result
}

// 21: teiler - Findet alle Teiler einer oder mehrerer Zahlen
pub fn teiler(zahlen_bereichs_angabe: &str) -> (Vec<String>, HashSet<i64>) {
    // Simulieren wir zunächst BereichToNumbers2 für den Test
    // In der Realität würdest du die echte Rust-Version von BereichToNumbers2 verwenden
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

// Hilfsfunktion: Simuliert BereichToNumbers2 für den Test
// In der echten Implementierung würdest du die richtige Rust-Version verwenden
fn simulate_bereich_to_numbers2(input: &str) -> HashSet<i64> {
    let mut result = HashSet::new();
    
    // Einfache Simulation: nimmt durch Komma getrennte Zahlen
    for part in input.split(',') {
        if let Ok(num) = part.trim().parse::<i64>() {
            if num > 0 {
                result.insert(num);
            }
        }
    }
    
    result
}

// Alternative: Optimierte Teiler-Funktion
pub fn find_all_divisors(n: i64) -> Vec<i64> {
    let mut divisors = Vec::new();
    
    if n <= 0 {
        return divisors;
    }
    
    let limit = (n as f64).sqrt() as i64;
    
    for i in 1..=limit {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    
    divisors.sort();
    divisors
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_multiples() {
        // Test mit 12
        let result = multiples(12, true);
        assert!(result.contains(&(6, 2)));
        assert!(result.contains(&(4, 3)));
        assert!(result.contains(&(12, 1)));
        assert_eq!(result.len(), 4); // (6,2), (4,3), (3,4), (12,1) + ggf. (2,6)
        
        // Test ohne mul1
        let result_no1 = multiples(12, false);
        assert!(result_no1.contains(&(6, 2)));
        assert!(result_no1.contains(&(4, 3)));
        assert!(!result_no1.contains(&(12, 1)));
        
        // Test mit Primzahl
        let prime_result = multiples(17, true);
        assert_eq!(prime_result, vec![(17, 1)]);
        
        let prime_result_no1 = multiples(17, false);
        assert!(prime_result_no1.is_empty());
        
        // Test mit 0 oder negativen Zahlen
        assert!(multiples(0, true).is_empty());
        assert!(multiples(-5, true).is_empty());
    }
    
    #[test]
    fn test_find_all_divisors() {
        assert_eq!(find_all_divisors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(find_all_divisors(17), vec![1, 17]);
        assert_eq!(find_all_divisors(1), vec![1]);
        assert_eq!(find_all_divisors(0), vec![]);
        assert_eq!(find_all_divisors(-5), vec![]);
        
        // Test mit perfektem Quadrat
        assert_eq!(find_all_divisors(16), vec![1, 2, 4, 8, 16]);
    }
    
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
        
        // 17 ist Primzahl: Teiler sind 1 und 17
        // Aber nach Logik: wenn mehr als 1 Teiler, wird 1 entfernt
        assert_eq!(set, HashSet::from([17]));
        assert_eq!(strings, vec!["17"]);
    }
    
    #[test]
    fn test_teiler_enhanced() {
        let (strings, set) = teiler_enhanced("12,8");
        
        // Vollständige Teiler von 12: 1,2,3,4,6,12
        // Vollständige Teiler von 8: 1,2,4,8
        // Kombiniert: 1,2,3,4,6,8,12
        // Nach Entfernen der 1: 2,3,4,6,8,12
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&4));
        assert!(set.contains(&6));
        assert!(set.contains(&8));
        assert!(set.contains(&12));
        assert!(!set.contains(&1));
        
        assert_eq!(strings, vec!["2", "3", "4", "6", "8", "12"]);
    }
}

// Beispiel: Hauptprogramm
fn main() {
    println!("=== Test der multiples-Funktion ===");
    
    let test_numbers = vec![12, 17, 24, 36];
    
    for n in test_numbers {
        println!("\nFaktorpaare für {}:", n);
        
        let with_one = multiples(n, true);
        println!("  Mit 1: {:?}", with_one);
        
        let without_one = multiples(n, false);
        println!("  Ohne 1: {:?}", without_one);
    }
    
    println!("\n=== Test der teiler-Funktion ===");
    
    let test_inputs = vec!["12", "12,8", "17,19", "24,36"];
    
    for input in test_inputs {
        println!("\nEingabe: '{}'", input);
        
        let (strings, set) = teiler(input);
        println!("  Teiler-Menge: {:?}", set);
        println!("  Teiler als Strings: {:?}", strings);
    }
    
    println!("\n=== Test der find_all_divisors-Funktion ===");
    
    for n in vec![12, 16, 17, 24] {
        println!("Teiler von {}: {:?}", n, find_all_divisors(n));
    }
    
    // Interaktiver Modus
    if let Some(arg) = std::env::args().nth(1) {
        println!("\n=== Interaktive Berechnung ===");
        
        if let Ok(num) = arg.parse::<i64>() {
            println!("Faktorpaare für {}:", num);
            println!("  {:?}", multiples(num, true));
            
            let divisors = find_all_divisors(num);
            println!("Alle Teiler von {}: {:?}", num, divisors);
        } else {
            // Als Bereichsangabe behandeln
            let (strings, set) = teiler(&arg);
            println!("Teiler für Bereich '{}':", arg);
            println!("  Menge: {:?}", set);
            println!("  Strings: {:?}", strings);
        }
    }
}
