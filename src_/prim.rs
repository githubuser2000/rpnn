use std::collections::HashMap;

// 24: primfaktoren - Zerlegt eine Zahl in ihre Primfaktoren
pub fn primfaktoren(n: i64, modulo: bool) -> Vec<i64> {
    let mut faktoren = Vec::new();
    let mut z = n;
    
    while z > 1 {
        // Bestimme den kleinsten Primfaktor p von z
        let mut i = 2;
        let mut gefunden = false;
        let mut p = z;
        
        while i * i <= n && !gefunden {
            if z % i == 0 {
                gefunden = true;
                p = i;
            } else {
                i += 1;
            }
        }
        
        // Füge p in die Liste der Faktoren ein
        if modulo {
            faktoren.push(p % 24);
        } else {
            faktoren.push(p);
        }
        z = z / p;
    }
    
    faktoren
}

// 25: primRepeat - Gruppiert wiederkehrende Primfaktoren mit Exponenten als String
pub fn prim_repeat(n: &[i64]) -> Vec<String> {
    let mut result = Vec::new();
    
    if n.is_empty() {
        return result;
    }
    
    let mut reversed = n.to_vec();
    reversed.reverse();
    
    let mut current_factor = reversed[0];
    let mut count = 1;
    let mut grouped: Vec<(i64, i64)> = Vec::new();
    
    // Gruppiere aufsteigend (vom Ende der umgekehrten Liste)
    for &factor in &reversed[1..] {
        if factor == current_factor {
            count += 1;
        } else {
            grouped.push((current_factor, count));
            current_factor = factor;
            count = 1;
        }
    }
    grouped.push((current_factor, count));
    
    // Umkehren, um ursprüngliche Reihenfolge wiederherzustellen
    grouped.reverse();
    
    // Konvertiere in String-Format
    let mut prev_factor = None;
    for (factor, exp) in grouped {
        if Some(factor) != prev_factor {
            if exp == 1 {
                result.push(factor.to_string());
            } else {
                result.push(format!("{}^{}", factor, exp));
            }
        }
        prev_factor = Some(factor);
    }
    
    result
}

// 26: primRepeat2 - Gruppiert wiederkehrende Primfaktoren als Tupel (Faktor, Exponent)
pub fn prim_repeat2(n: &[i64]) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    
    if n.is_empty() {
        return result;
    }
    
    let mut reversed = n.to_vec();
    reversed.reverse();
    
    let mut current_factor = reversed[0];
    let mut count = 1;
    let mut grouped: Vec<(i64, i64)> = Vec::new();
    
    // Gruppiere aufsteigend (vom Ende der umgekehrten Liste)
    for &factor in &reversed[1..] {
        if factor == current_factor {
            count += 1;
        } else {
            grouped.push((current_factor, count));
            current_factor = factor;
            count = 1;
        }
    }
    grouped.push((current_factor, count));
    
    // Umkehren, um ursprüngliche Reihenfolge wiederherzustellen
    grouped.reverse();
    
    // Entferne Duplikate und behalte nur das erste Vorkommen jeder Primzahl
    let mut prev_factor = None;
    for (factor, exp) in grouped {
        if Some(factor) != prev_factor {
            result.push((factor, exp));
        }
        prev_factor = Some(factor);
    }
    
    result
}

// Alternative Implementierung von primRepeat2 mit effizienterem Algorithmus
pub fn prim_repeat2_alternative(n: &[i64]) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    
    if n.is_empty() {
        return result;
    }
    
    let mut iter = n.iter();
    let mut current_factor = *iter.next().unwrap();
    let mut count = 1;
    
    for &factor in iter {
        if factor == current_factor {
            count += 1;
        } else {
            result.push((current_factor, count));
            current_factor = factor;
            count = 1;
        }
    }
    
    // Letzte Gruppe hinzufügen
    result.push((current_factor, count));
    
    result
}

// Testfunktionen
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_primfaktoren() {
        assert_eq!(primfaktoren(24, false), vec![2, 2, 2, 3]);
        assert_eq!(primfaktoren(17, false), vec![17]);
        assert_eq!(primfaktoren(1, false), vec![]);
        assert_eq!(primfaktoren(36, false), vec![2, 2, 3, 3]);
    }
    
    #[test]
    fn test_primfaktoren_modulo() {
        let result = primfaktoren(24, true);
        // 2%24=2, 2%24=2, 2%24=2, 3%24=3
        assert_eq!(result, vec![2, 2, 2, 3]);
        
        let result = primfaktoren(25, true);
        // 5%24=5, 5%24=5
        assert_eq!(result, vec![5, 5]);
    }
    
    #[test]
    fn test_prim_repeat() {
        assert_eq!(
            prim_repeat(&[2, 2, 2, 3]),
            vec!["2^3".to_string(), "3".to_string()]
        );
        
        assert_eq!(
            prim_repeat(&[2, 2, 3, 3, 5]),
            vec!["2^2".to_string(), "3^2".to_string(), "5".to_string()]
        );
        
        assert_eq!(
            prim_repeat(&[2, 3, 5]),
            vec!["2".to_string(), "3".to_string(), "5".to_string()]
        );
        
        assert_eq!(prim_repeat(&[]), Vec::<String>::new());
    }
    
    #[test]
    fn test_prim_repeat2() {
        assert_eq!(
            prim_repeat2(&[2, 2, 2, 3]),
            vec![(2, 3), (3, 1)]
        );
        
        assert_eq!(
            prim_repeat2(&[2, 2, 3, 3, 5]),
            vec![(2, 2), (3, 2), (5, 1)]
        );
        
        assert_eq!(
            prim_repeat2(&[2, 3, 5]),
            vec![(2, 1), (3, 1), (5, 1)]
        );
    }
    
    #[test]
    fn test_prim_repeat2_alternative() {
        assert_eq!(
            prim_repeat2_alternative(&[2, 2, 2, 3]),
            vec![(2, 3), (3, 1)]
        );
        
        assert_eq!(
            prim_repeat2_alternative(&[2, 2, 3, 3, 5]),
            vec![(2, 2), (3, 2), (5, 1)]
        );
    }
}

// Beispiel: Verwendung der Funktionen
fn main() {
    // Test mit 24
    let n = 24;
    let faktoren = primfaktoren(n, false);
    println!("Primfaktoren von {}: {:?}", n, faktoren);
    
    let gruppiert_str = prim_repeat(&faktoren);
    println!("Gruppiert als String: {:?}", gruppiert_str);
    
    let gruppiert_tupel = prim_repeat2(&faktoren);
    println!("Gruppiert als Tupel: {:?}", gruppiert_tupel);
    
    // Test mit 36
    let n2 = 36;
    let faktoren2 = primfaktoren(n2, false);
    println!("\nPrimfaktoren von {}: {:?}", n2, faktoren2);
    
    println!("Gruppiert: {:?}", prim_repeat(&faktoren2));
    
    // Test mit modulo = true
    let faktoren_mod = primfaktoren(24, true);
    println!("\nPrimfaktoren von 24 mit modulo 24: {:?}", faktoren_mod);
}
