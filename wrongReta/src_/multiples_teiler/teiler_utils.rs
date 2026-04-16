use std::collections::HashSet;

// Hilfsfunktion zum Konvertieren von HashSet zu sortierter String-Liste
pub fn hashset_to_sorted_strings(set: &HashSet<i64>) -> Vec<String> {
    let mut strings: Vec<String> = set.iter()
        .map(|n| n.to_string())
        .collect();
    
    strings.sort_by(|a, b| {
        a.parse::<i64>().unwrap().cmp(&b.parse::<i64>().unwrap())
    });
    
    strings
}

// Prüft ob eine Zahl eine Primzahl ist
pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    
    if n == 2 || n == 3 {
        return true;
    }
    
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let limit = (n as f64).sqrt() as i64;
    
    for i in (5..=limit).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }
    
    true
}

// Findet den größten gemeinsamen Teiler (GCD)
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

// Findet das kleinste gemeinsame Vielfache (LCM)
pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a.abs() / gcd(a, b)) * b.abs()
    }
}

// Berechnet Primfaktorzerlegung
pub fn prime_factors(mut n: i64) -> Vec<(i64, usize)> {
    let mut factors = Vec::new();
    
    if n <= 1 {
        return factors;
    }
    
    // Zähle Faktor 2
    let mut count = 0;
    while n % 2 == 0 {
        n /= 2;
        count += 1;
    }
    
    if count > 0 {
        factors.push((2, count));
    }
    
    // Prüfe ungerade Faktoren
    let mut i = 3;
    while i * i <= n {
        let mut count = 0;
        while n % i == 0 {
            n /= i;
            count += 1;
        }
        
        if count > 0 {
            factors.push((i, count));
        }
        
        i += 2;
    }
    
    // Falls n noch größer als 1 ist, ist es ein Primfaktor
    if n > 1 {
        factors.push((n, 1));
    }
    
    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(is_prime(17));
        assert!(!is_prime(18));
    }
    
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(-12, 8), 4);
    }
    
    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 8), 24);
        assert_eq!(lcm(17, 13), 221);
        assert_eq!(lcm(0, 5), 0);
        assert_eq!(lcm(5, 0), 0);
        assert_eq!(lcm(-12, 8), 24);
    }
    
    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(12), vec![(2, 2), (3, 1)]);
        assert_eq!(prime_factors(17), vec![(17, 1)]);
        assert_eq!(prime_factors(1), vec![]);
        assert_eq!(prime_factors(0), vec![]);
        assert_eq!(prime_factors(36), vec![(2, 2), (3, 2)]);
    }
}
