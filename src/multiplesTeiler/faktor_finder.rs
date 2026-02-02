use std::collections::HashSet;

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

// Verbesserte Multiples-Funktion mit besseren Namen
pub fn find_factor_pairs(n: i64, include_one: bool) -> Vec<(i64, i64)> {
    multiples(n, include_one)
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
    fn test_find_factor_pairs() {
        let pairs = find_factor_pairs(12, true);
        assert!(pairs.contains(&(6, 2)));
        assert!(pairs.contains(&(4, 3)));
        assert!(pairs.contains(&(12, 1)));
    }
}
